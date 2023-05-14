/**
 * This file is part of Rellume.
 *
 * (c) 2020, Alexis Engelke <alexis.engelke@googlemail.com>
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

/**
 * \file
 **/

#ifndef RELLUME_INSTR_H
#define RELLUME_INSTR_H

#include <farmdec.h>
#include <cstdbool>
#include <cstdint>
#include <optional>

#include "arch.h"

namespace rellume {

class Instr {
    Arch arch;
    unsigned char instlen;
    uint64_t addr;
    union {
        farmdec::Inst _a64;
    };

public:
    size_t len() const { return instlen; }
    uintptr_t start() const { return addr; }
    uintptr_t end() const { return start() + len(); }
    operator const farmdec::Inst*() const {
        assert(arch == Arch::AArch64);
        return &_a64;
    }

    enum class Kind {
        BRANCH,
        COND_BRANCH,
        CALL,
        UNKNOWN,
        OTHER
    };
    Kind Kind() {
        switch (arch) {
        case Arch::AArch64:
            switch (_a64.op) {
            default:                 return Kind::OTHER;
            case farmdec::A64_BCOND: return Kind::COND_BRANCH;
            case farmdec::A64_CBZ:   return Kind::COND_BRANCH;
            case farmdec::A64_CBNZ:  return Kind::COND_BRANCH;
            case farmdec::A64_TBZ:   return Kind::COND_BRANCH;
            case farmdec::A64_TBNZ:  return Kind::COND_BRANCH;
            case farmdec::A64_B:     return Kind::BRANCH;
            case farmdec::A64_BL:    return Kind::CALL;
            case farmdec::A64_BR:    return Kind::BRANCH;
            case farmdec::A64_BLR:   return Kind::CALL;
            case farmdec::A64_RET:   return Kind::UNKNOWN;
            case farmdec::A64_SVC:   return Kind::UNKNOWN;
            case farmdec::A64_HVC:   return Kind::UNKNOWN;
            case farmdec::A64_SMC:   return Kind::UNKNOWN;
            case farmdec::A64_BRK:   return Kind::UNKNOWN;
            case farmdec::A64_HLT:   return Kind::UNKNOWN;
            case farmdec::A64_DCPS1: return Kind::UNKNOWN;
            case farmdec::A64_DCPS2: return Kind::UNKNOWN;
            case farmdec::A64_DCPS3: return Kind::UNKNOWN;
            }
        default:
            return Kind::UNKNOWN;
        }
    }
    std::optional<uintptr_t> JumpTarget() {
        switch (arch) {
        case Arch::AArch64:
            switch (Kind()) {
            case Kind::COND_BRANCH:
                if (_a64.op == farmdec::A64_TBZ || _a64.op == farmdec::A64_TBNZ) {
                    return start() + _a64.tbz.offset;
                }
                /* FALLTHROUGH */
            case Kind::BRANCH:
            case Kind::CALL:
                // ARM PC points at current instruction → start()
                if (_a64.op != farmdec::A64_BR && _a64.op != farmdec::A64_BLR)
                    return start() + _a64.offset;
                break;
            default:
                break;
            }
            break;
        default:
            break;
        }
        return std::nullopt;
    }

    /// Fill Instr with the instruction at buf and return number of consumed
    /// bytes (or negative on error). addr is the virtual address of the
    /// instruction.
    int DecodeFrom(Arch arch, const uint8_t* buf, size_t len, uintptr_t addr) {
        this->arch = arch;
        this->addr = addr;
        int res = -1;
        switch (arch) {
        case Arch::AArch64: {
            if (len < 4) {
                return -1;
            }

            uint32_t binst = buf[0] | (buf[1] << 8) | (buf[2] << 16) | (buf[3] << 24);
            fad_decode(&binst, 1, &_a64);
            if (_a64.op == farmdec::A64_ERROR || _a64.op == farmdec::A64_UNKNOWN) {
                return -1;
            }
            res = 4; // all instructions are 32 bits long
            break;
        }
        default:
            break;
        }

        if (res >= 0)
            instlen = res;
        return res;
    }
};

} // namespace
#endif
