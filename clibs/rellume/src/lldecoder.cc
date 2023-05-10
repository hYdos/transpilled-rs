/**
 * This file is part of Rellume.
 *
 * (c) 2019, Alexis Engelke <alexis.engelke@googlemail.com>
 *
 * Rellume is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License (LGPL)
 * as published by the Free Software Foundation, either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Rellume is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Rellume.  If not, see <http://www.gnu.org/licenses/>.
 */

#include "function.h"

#include "arch.h"
#include "basicblock.h"
#include "config.h"
#include "instr.h"

#include <cstdint>
#include <deque>
#include <unordered_map>
#include <vector>

namespace rellume {

int Function::Decode(uintptr_t addr, DecodeStop stop, MemReader memacc) {
    Instr inst;
    uint8_t inst_buf[15];

    std::deque<uintptr_t> addr_queue;
    addr_queue.push_back(addr);

    std::vector<Instr> insts;
    // List of (start_idx,end_idx) (non-inclusive end)
    std::vector<std::pair<size_t, size_t>> blocks;

    // Mapping from address to (block_idx, instr_idx)
    std::unordered_map<uintptr_t, std::pair<size_t, size_t>> addr_map;

    while (!addr_queue.empty()) {
        uintptr_t cur_addr = addr_queue.front();
        addr_queue.pop_front();

        size_t cur_block_start = insts.size();

        auto cur_addr_entry = addr_map.find(cur_addr);
        while (cur_addr_entry == addr_map.end()) {
            size_t inst_buf_sz = memacc(cur_addr, inst_buf, sizeof(inst_buf));
            // Sanity check.
            if (inst_buf_sz == 0 || inst_buf_sz > sizeof(inst_buf))
                break;

            int ret = inst.DecodeFrom(cfg->arch, inst_buf, inst_buf_sz, cur_addr);
            if (ret < 0) // invalid or unknown instruction
                break;

            addr_map[cur_addr] = std::make_pair(blocks.size(), insts.size());
            insts.push_back(inst);

            if (stop == DecodeStop::INSTR)
                break;

            switch (inst.Kind()) {
            case Instr::Kind::COND_BRANCH:
                addr_queue.push_back(cur_addr + inst.len());
                /* FALLTHROUGH */
            case Instr::Kind::BRANCH:
                if (auto jmp_target = inst.JumpTarget())
                    addr_queue.push_back(jmp_target.value());
                /* FALLTHROUGH */
            case Instr::Kind::UNKNOWN:
                goto end_block;
            case Instr::Kind::CALL:
                if (cfg->call_function)
                    addr_queue.push_back(cur_addr + inst.len());
                // A call still ends a block, because we don't *know* that the
                // execution continues after the call.
                goto end_block;
            default:
                break;
            }
            cur_addr += inst.len();
            cur_addr_entry = addr_map.find(cur_addr);
        }
    end_block:

        if (insts.size() != cur_block_start)
            blocks.push_back(std::make_pair(cur_block_start, insts.size()));

        if (cur_addr_entry != addr_map.end()) {
            auto& other_blk = blocks[cur_addr_entry->second.first];
            size_t split_idx = cur_addr_entry->second.second;
            if (other_blk.first == split_idx)
                continue;
            size_t end = other_blk.second;
            blocks.push_back(std::make_pair(split_idx, end));
            blocks[cur_addr_entry->second.first].second = split_idx;
            size_t new_block_idx = blocks.size() - 1;
            for (size_t j = split_idx; j < end; j++)
                addr_map[insts[j].start()] = std::make_pair(new_block_idx, j);
        }

        if (stop == DecodeStop::BASICBLOCK)
            addr_queue.clear();
    }

    bool first_inst = true;
    for (auto it = blocks.begin(); it != blocks.end(); it++) {
        uint64_t block_addr = insts[it->first].start();
        for (size_t j = it->first; j < it->second; j++) {
            if (!AddInst(block_addr, insts[j])) {
                // If we fail on the first instruction, propagate error.
                if (first_inst)
                    return 1;
                // Otherwise continue with other basic blocks.
                break;
            }
            first_inst = false;
        }
    }

    // If we didn't lift a single instruction, return error code.
    if (first_inst)
        return 1;

    return 0;
}

} // namespace rellume
