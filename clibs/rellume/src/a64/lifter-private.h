/**
 * This file is part of Rellume.
 *
 * (c) 2016-2019, Alexis Engelke <alexis.engelke@googlemail.com>
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

#ifndef RELLUME_A64_LIFTER_H
#define RELLUME_A64_LIFTER_H

#include <farmdec.h>
#include "../basicblock.h"
#include "../config.h"
#include "../function-info.h"
#include "../instr.h"
#include "../regfile.h"
#include "../lifter-base.h"

namespace rellume::aarch64 {

class Lifter : public LifterBase {
public:
    Lifter(FunctionInfo& fi, const LLConfig& cfg, ArchBasicBlock& ab) :
            LifterBase(fi, cfg, ab) {}

    bool Lift(const Instr&);

private:
    bool LiftSIMD(farmdec::Inst a64);
    llvm::Value* GetVec(farmdec::Reg r, farmdec::VectorArrangement va, bool fp = false);
    llvm::Value* GetElem(farmdec::Reg r, farmdec::VectorArrangement va, unsigned i, bool fp = false);
    void SetVec(farmdec::Reg r, llvm::Value* val);
    void InsertElem(farmdec::Reg r, unsigned i, llvm::Value* elem);
    void Dup(farmdec::Reg r, farmdec::VectorArrangement va, llvm::Value* elem);
    void InsertInHalf(farmdec::Reg r, farmdec::VectorArrangement va, llvm::Value* narrow);
    unsigned NumElem(farmdec::VectorArrangement va);
    Facet FacetOf(farmdec::VectorArrangement va, bool fp = false);
    farmdec::VectorArrangement DoubleWidth(farmdec::VectorArrangement va);
    llvm::Value* MinMax(llvm::Value* lhs, llvm::Value* rhs, bool sgn, bool min);
    llvm::Value* Abs(llvm::Value* v);
    llvm::Value* Narrow(llvm::Value* v);
    llvm::Value* Halve(llvm::Value* v, farmdec::VectorArrangement va);
    llvm::Value* Concat(farmdec::VectorArrangement va, llvm::Value* a, llvm::Value* b);
    llvm::Value* Concat(farmdec::VectorArrangement va, llvm::Value* a, llvm::Value* b, llvm::Value* c, llvm::Value* d);

    llvm::Value* GetGp(farmdec::Reg, bool w32, bool ptr = false);
    void SetGp(farmdec::Reg, bool w32, llvm::Value* val);
    llvm::Value* GetScalar(farmdec::Reg r, farmdec::FPSize fsz, bool fp = true);
    void SetScalar(farmdec::Reg r, llvm::Value* val);

    void FlagCalcAdd(llvm::Value* res, llvm::Value* lhs, llvm::Value* rhs);
    void FlagCalcSub(llvm::Value* res, llvm::Value* lhs, llvm::Value* rhs);
    void FlagCalcLogic(llvm::Value* res);

    llvm::Value* Shift(llvm::Value* v, farmdec::Shift sh, uint32_t amount);
    llvm::Value* Extend(llvm::Value* v, bool w32, farmdec::ExtendType ext, uint32_t lsl);
    llvm::Value* Round(llvm::Value* v, farmdec::FPRounding mode, bool exact = false);

    llvm::IntegerType* TypeOf(farmdec::Size sz);
    llvm::Type* TypeOf(farmdec::FPSize fsz);
    llvm::Type* TypeOf(farmdec::VectorArrangement va, bool fp = false);
    llvm::Type* ElemTypeOf(farmdec::VectorArrangement va, bool fp = false);
    llvm::AtomicOrdering Ordering(farmdec::MemOrdering);
    llvm::Value* IsTrue(farmdec::Cond);

    llvm::Value* Extract(llvm::Value* v, bool w32, unsigned lsb, unsigned width);
    llvm::Value* MoveField(llvm::Value* v, bool w32, unsigned lsb, unsigned width);

    llvm::Value* MulAddSub(llvm::Value* base, llvm::Instruction::BinaryOps addsub, llvm::Value* lhs, llvm::Value* rhs, llvm::Instruction::CastOps extend);

    llvm::Value* PCRel(uint64_t off);

    // Dispatches to the correct addressing mode variant.
    llvm::Value* Addr(llvm::Type* elemty, farmdec::Inst inst);

    // The various addressing modes. See farmdec::AddrMode.
    llvm::Value* Addr(llvm::Type* elemty, farmdec::Reg base);
    llvm::Value* Addr(llvm::Type* elemty, farmdec::Reg base, uint64_t off);
    llvm::Value* Addr(llvm::Type* elemty, farmdec::Reg base, farmdec::Reg off, uint32_t lsl);
    llvm::Value* Addr(llvm::Type* elemty, farmdec::Reg base, farmdec::Reg off, farmdec::ExtendType ext, uint32_t lsl);

    void Load(farmdec::Reg rt, bool w32, llvm::Type* srcty, llvm::Value* ptr, farmdec::ExtendType ext, farmdec::MemOrdering mo = farmdec::MO_NONE);
    void Load(farmdec::Reg rt, llvm::Type* srcty, llvm::Value* ptr, farmdec::MemOrdering mo = farmdec::MO_NONE);
    void Store(llvm::Value* ptr, llvm::Value* val, farmdec::MemOrdering mo = farmdec::MO_NONE);

    enum class BinOpKind : unsigned {
        SHIFT, EXT, IMM
    };

    void LiftBinOp(farmdec::Inst a64, bool w32, llvm::Instruction::BinaryOps op, BinOpKind kind, bool set_flags = false, bool invert_rhs = false);
    void LiftCCmp(llvm::Value* lhs, llvm::Value* rhs, farmdec::Cond cond, uint8_t nzcv, bool ccmn, bool fp = false);
    void LiftLoadStore(farmdec::Inst a64, bool w32, bool fp = false);

    void FlagCalcFP(llvm::Value* lhs, llvm::Value* rhs);
    void LiftBinOpFP(llvm::Instruction::BinaryOps op, farmdec::FPSize prec, farmdec::Reg rd, farmdec::Reg rn, farmdec::Reg rm);
    void LiftFMA(farmdec::FPSize prec, farmdec::Reg rd, farmdec::Reg rn, farmdec::Reg rm, farmdec::Reg ra, bool neg_mult, bool sub);
    void LiftIntrinsicFP(llvm::Intrinsic::ID op, farmdec::FPSize prec, farmdec::Reg rd, farmdec::Reg rn);
    void LiftIntrinsicFP(llvm::Intrinsic::ID op, farmdec::FPSize prec, farmdec::Reg rd, farmdec::Reg rn, farmdec::Reg rm);

    void LiftIntrinsicFPVec(llvm::Intrinsic::ID op, farmdec::VectorArrangement va, farmdec::Reg rd, farmdec::Reg rn, farmdec::Reg rm);

    void LiftThreeSame(llvm::Instruction::BinaryOps op, farmdec::Reg rd, farmdec::VectorArrangement va, farmdec::Reg rn, farmdec::Reg rm, bool scalar, bool invert_rhs = false, bool fp = false);
    void LiftCmXX(llvm::CmpInst::Predicate cmp, farmdec::Reg rd, farmdec::VectorArrangement va, farmdec::Reg rn, farmdec::Reg rm, bool zero, bool fp = false);
    void LiftScalarCmXX(llvm::CmpInst::Predicate cmp, farmdec::Reg rd, farmdec::Reg rn, farmdec::Reg rm, bool zero, bool fp = false);
    void LiftMulAcc(farmdec::Inst a64, llvm::Instruction::BinaryOps addsub, llvm::Value* acc, bool extend_long, bool fp = false);
    void LiftMulAccElem(farmdec::Inst a64, llvm::Instruction::BinaryOps addsub, llvm::Value* acc, bool extend_long, bool fp = false);
    void TransformSIMDPairwise(farmdec::VectorArrangement va, farmdec::Reg rn, farmdec::Reg rm, llvm::Value** lhs, llvm::Value** rhs, bool fp = false);

    void StoreMulti(llvm::Value* addr, llvm::Value* v0);
    void StoreMulti(llvm::Value* addr, llvm::Value* v0, llvm::Value* v1);
    void StoreMulti(llvm::Value* addr, llvm::Value* v0, llvm::Value* v1, llvm::Value* v2);
    void StoreMulti(llvm::Value* addr, llvm::Value* v0, llvm::Value* v1, llvm::Value* v2, llvm::Value* v3);
    llvm::Value* SIMDLoadStoreAddr(farmdec::Inst a64, llvm::Type* ty);
};

} // namespace

#endif
