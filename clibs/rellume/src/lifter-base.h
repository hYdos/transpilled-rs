/**
 * This file is part of Rellume.
 *
 * (c) 2016-2020, Alexis Engelke <alexis.engelke@googlemail.com>
 * (c) 2020, Dominik Okwieka <dominik.okwieka@t-online.de>
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

#ifndef RELLUME_LIFTER_BASE_H
#define RELLUME_LIFTER_BASE_H

#include "basicblock.h"
#include "config.h"
#include "facet.h"
#include "function-info.h"
#include "instr.h"
#include "regfile.h"
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/Instruction.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Operator.h>
#include <vector>

namespace rellume {

/**
 * \brief Architecture-independent part of lifter.
 **/
class LifterBase {
protected:
    ArchBasicBlock& ablock;
    RegFile* regfile;
    llvm::IRBuilder<> irb;
    FunctionInfo& fi;
    const LLConfig& cfg;

    LifterBase(FunctionInfo& fi, const LLConfig& cfg, ArchBasicBlock& ab)
            : ablock(ab), regfile(ab.GetInsertBlock()->GetRegFile()),
              irb(regfile->GetInsertBlock()), fi(fi), cfg(cfg) {
        // Set fast-math flags. Newer LLVM supports FastMathFlags::getFast().
        if (cfg.enableFastMath) {
            llvm::FastMathFlags fmf;
            fmf.setFast();
            irb.setFastMathFlags(fmf);
        }
    }

    LifterBase(LifterBase&& rhs);
    LifterBase& operator=(LifterBase&& rhs);

    LifterBase(const LifterBase&) = delete;
    LifterBase& operator=(const LifterBase&) = delete;

    llvm::Module* GetModule() {
        return irb.GetInsertBlock()->getModule();
    }

    llvm::Value* GetReg(ArchReg reg, Facet facet) {
        return regfile->GetReg(reg, facet);
    }
    void SetReg(ArchReg reg, Facet facet, llvm::Value* value) {
        regfile->SetReg(reg, facet, value, true); // clear all other facets
    }
    void SetRegFacet(ArchReg reg, Facet facet, llvm::Value* value) {
        regfile->SetReg(reg, facet, value, false);
    }
    void SetRegPtr(ArchReg reg, llvm::Value* value) {
        SetReg(reg, Facet::I64, irb.CreatePtrToInt(value, irb.getInt64Ty()));
        SetRegFacet(reg, Facet::PTR, value);
    }
    llvm::Value* GetFlag(Facet facet) {
        if (facet == Facet::PF) {
            llvm::Value* res = GetReg(ArchReg::EFLAGS, facet);
            res = irb.CreateUnaryIntrinsic(llvm::Intrinsic::ctpop, res);
            return irb.CreateNot(irb.CreateTrunc(res, irb.getInt1Ty()));
        }
        return GetReg(ArchReg::EFLAGS, facet);
    }
    void SetFlag(Facet facet, llvm::Value* value) {
        SetRegFacet(ArchReg::EFLAGS, facet, value);
    }
    void SetFlagUndef(std::initializer_list<Facet> facets) {
        llvm::Value* undef = llvm::UndefValue::get(irb.getInt1Ty());
        for (const auto facet : facets) {
            // TODO: actually use freeze.
            if (facet == Facet::PF)
                SetFlag(facet, llvm::UndefValue::get(irb.getInt8Ty()));
            else
                SetFlag(facet, undef);
        }
    }
    void SetIP(uint64_t inst_addr, bool nofold = false);

    void SetInsertBlock(BasicBlock* block) {
        ablock.SetInsertBlock(block);
        regfile = block->GetRegFile();
        irb.SetInsertPoint(regfile->GetInsertBlock());
    }

    llvm::Value* AddrConst(uint64_t addr, llvm::PointerType* ptr_ty);

    // Helper function for older LLVM versions
    llvm::Value* CreateUnaryIntrinsic(llvm::Intrinsic::ID id, llvm::Value* v) {
        // TODO: remove this helper function
        return irb.CreateUnaryIntrinsic(id, v);
    }

    void CallExternalFunction(llvm::Function* fn);

    void ForceReturn() {
        cfg.callconv.Return(ablock.GetInsertBlock(), fi);
    }
};

} // namespace rellume

#endif
