use cty::{c_int, int16_t, int32_t, uint8_t};

extern "C" {
    pub fn fad_decode(
        _in: *mut cty::uint32_t,
        n: cty::c_uint,
        _out: *mut Inst
    ) -> c_int;
}

#[repr(C)]
enum Op {
    A64_UNKNOWN, // unknown instruction (or Op field not set, by accident), Inst.imm contains raw binary instruction
    A64_ERROR,   // invalid instruction, Inst.error contains error string
    A64_UDF,     // throws undefined exception

    /*** Data Processing -- Immediate ***/

    // PC-rel. addressing
    A64_ADR,     // ADR Xd, label  -- Xd ← PC + label
    A64_ADRP,    // ADRP Xd, label -- Xd ← PC + (label * 4K)

    // Add/subtract (immediate, with tags) -- OMITTED

    // Add/subtract (immediate)
    A64_ADD_IMM,
    A64_CMN_IMM,
    A64_MOV_SP, // MOV from/to SP -- ADD (imm) alias (predicate: shift == 0 && imm12 == 0 && (Rd == SP || Rn == SP))
    A64_SUB_IMM,
    A64_CMP_IMM,

    // Logical (immediate)
    A64_AND_IMM,
    A64_ORR_IMM,
    A64_EOR_IMM,
    A64_TST_IMM, // TST Rn -- ANDS alias (Rd := RZR, predicate: Rd == ZR && set_flags)

    // Move wide (immediate)
    A64_MOVK, // keep other bits

    // Synthetic instruction comprising MOV (bitmask immediate), MOV (inverted wide immediate)
    // and MOV (wide immediate), MOVN and MOVZ; essentially all MOVs where the result of the
    // operation can be precalculated. For lifting, we do not care how the immediate was encoded,
    // only that it is an immediate move.
    A64_MOV_IMM,

    // Bitfield
    A64_SBFM,    // always decoded to an alias
    A64_ASR_IMM,
    A64_SBFIZ,
    A64_SBFX,
    A64_BFM,     // always decoded to an alias
    A64_BFC,
    A64_BFI,
    A64_BFXIL,
    A64_UBFM,    // always decoded to an alias
    A64_LSL_IMM,
    A64_LSR_IMM,
    A64_UBFIZ,
    A64_UBFX,

    // Synthetic instruction comprising the SXTB, SXTH, SXTW, UXTB and UXTH aliases of SBFM and UBFM.
    // The kind of extension is stored in Inst.extend.type.
    A64_EXTEND,

    // Extract
    A64_EXTR,
    A64_ROR_IMM, // ROR Rd, Rs, #shift -- EXTR alias (Rm := Rs, Rn := Rs, predicate: Rm == Rn)

    /*** Branches, Exception Generating and System Instructions ***/

    A64_BCOND,

    // Exception generation
    //
    // With the exception of SVC, they are not interesting for lifting
    // userspace programs, but were included since they are trivial.
    A64_SVC, // system call
    A64_HVC,
    A64_SMC,
    A64_BRK,
    A64_HLT,
    A64_DCPS1,
    A64_DCPS2,
    A64_DCPS3,

    // Hints -- we treat all allocated hints as NOP and don't decode to the "aliases"
    // NOP, YIELD, ...
    A64_HINT,

    // Barriers
    A64_CLREX,
    A64_DMB,
    A64_ISB,
    A64_SB,
    A64_DSB,
    A64_SSBB,
    A64_PSSBB,

    // PSTATE
    A64_MSR_IMM, // MSR <pstatefield>, #imm -- Inst.msr_imm
    A64_CFINV,
    A64_XAFlag,  // irrelevant
    A64_AXFlag,  // ------

    // System instructions -- Inst.ldst.rt := Xt
    A64_SYS,  // SYS #op1, Cn, Cm, #op2(, Xt)
    A64_SYSL, // SYSL Xt, #op1, Cn, Cm, #op2

    // System register move -- Inst.ldst.rt := Xt; Inst.imm := sysreg
    A64_MSR_REG, // MSR <sysreg>, Xt
    A64_MRS,     // MRS Xt, <sysreg>

    // Unconditional branch (register)
    A64_BR,
    A64_BLR,
    A64_RET,

    // Unconditional branch (immediate)
    A64_B,
    A64_BL,

    // Compare and branch (immediate)
    A64_CBZ,
    A64_CBNZ,

    // Test and branch (immediate) -- Inst.tbz
    A64_TBZ,
    A64_TBNZ,

    /*** Data Processing -- Register ***/

    // Data-processing (2 source)
    A64_UDIV,
    A64_SDIV,
    A64_LSLV,
    A64_LSRV,
    A64_ASRV,
    A64_RORV,
    A64_CRC32B,
    A64_CRC32H,
    A64_CRC32W,
    A64_CRC32X,
    A64_CRC32CB,
    A64_CRC32CH,
    A64_CRC32CW,
    A64_CRC32CX,
    A64_SUBP,

    // Data-processing (1 source)
    A64_RBIT,
    A64_REV16,
    A64_REV,
    A64_REV32,
    A64_CLZ,
    A64_CLS,

    // Logical (shifted register)
    A64_AND_SHIFTED,
    A64_TST_SHIFTED, // ANDS alias (Rd := ZR, predicate: Rd == ZR)
    A64_BIC,
    A64_ORR_SHIFTED,
    A64_MOV_REG,     // ORR alias (predicate: shift == 0 && imm6 == 0 && Rn == ZR)
    A64_ORN,
    A64_MVN,         // ORN alias (Rn := ZR, predicate: Rn == ZR)
    A64_EOR_SHIFTED,
    A64_EON,

    // Add/subtract (shifted register)
    A64_ADD_SHIFTED,
    A64_CMN_SHIFTED, // ADDS alias (Rd := ZR, predicate: Rd == ZR && set_flags)
    A64_SUB_SHIFTED,
    A64_NEG,         // SUB alias (Rn := ZR, predicate: Rn == ZR)
    A64_CMP_SHIFTED, // SUBS alias (Rd := ZR, predicate: Rd == ZR && set_flags)

    // Add/subtract (extended register)
    // Register 31 is interpreted as the stack pointer (SP/WSP).
    A64_ADD_EXT,
    A64_CMN_EXT, // ADDS alias (Rd := ZR, predicate: Rd == ZR && set_flags)
    A64_SUB_EXT,
    A64_CMP_EXT, // SUBS alias (Rd := ZR, predicate: Rd == ZR && set_flags)

    // Add/subtract (with carry)
    A64_ADC,
    A64_SBC,
    A64_NGC, // SBC alias (Rd := ZR, predicate: Rd == RR)

    // Rotate right into flags
    A64_RMIF,

    // Evaluate into flags
    A64_SETF8,
    A64_SETF16,

    // Conditional compare (register)
    A64_CCMN_REG,
    A64_CCMP_REG,

    // Conditional compare (immediate)
    A64_CCMN_IMM,
    A64_CCMP_IMM,

    // Conditional select
    A64_CSEL,
    A64_CSINC,
    A64_CINC,  // CSINC alias (cond := invert(cond), predicate: Rm == Rn != ZR)
    A64_CSET,  // CSINC alias (cond := invert(cond), predicate: Rm == Rn == ZR)
    A64_CSINV,
    A64_CINV,  // CSINV alias (cond := invert(cond), predicate: Rm == Rn != ZR)
    A64_CSETM, // CSINV alias (cond := invert(cond), predicate: Rm == Rn == ZR)
    A64_CSNEG,
    A64_CNEG,  // CSNEG alias (cond := invert(cond), predicate: Rm == Rn)

    // Data-processing (3 source)
    A64_MADD,
    A64_MUL,    // MADD alias (Ra omitted, predicate: Ra == ZR)
    A64_MSUB,
    A64_MNEG,   // MSUB alias (^---- see above)
    A64_SMADDL,
    A64_SMULL,  // SMADDL alias  (^---- see above)
    A64_SMSUBL,
    A64_SMNEGL, // SMSUBL alias (^---- see above)
    A64_SMULH,
    A64_UMADDL,
    A64_UMULL,  // UMADDL alias (^---- see above)
    A64_UMSUBL,
    A64_UMNEGL, // UMSUBL alias (^---- see above)
    A64_UMULH,

    /*** Loads and Stores ***/

    // There are not that many opcodes because access size, sign-extension
    // and addressing mode (post-indexed, register offset, immediate) are
    // encoded in the Inst, to leverage the regular structure and cut down
    // on opcodes (and by extension, duplicative switch-cases for the user
    // of this decoder).

    // Advanced SIMD load/store multiple structures
    // Advanced SIMD load/store multiple structures (post-indexed)
    A64_LD1_MULT,
    A64_ST1_MULT,
    A64_LD2_MULT,
    A64_ST2_MULT,
    A64_LD3_MULT,
    A64_ST3_MULT,
    A64_LD4_MULT,
    A64_ST4_MULT,

    // Advanced SIMD load/store single structure
    // Advanced SIMD load/store single structure (post-indexed)
    A64_LD1_SINGLE,
    A64_ST1_SINGLE,
    A64_LD2_SINGLE,
    A64_ST2_SINGLE,
    A64_LD3_SINGLE,
    A64_ST3_SINGLE,
    A64_LD4_SINGLE,
    A64_ST4_SINGLE,
    A64_LD1R,
    A64_LD2R,
    A64_LD3R,
    A64_LD4R,

    // Load/store exclusive
    A64_LDXR,  // includes Load-acquire variants
    A64_STXR,  // includes Store-acquire variants (STLXR)
    A64_LDXP,  // ------
    A64_STXP,  // ------
    A64_LDAPR, // Load-AcquirePC Register (actually in Atomic group)

    // Load/store no-allocate pair (offset)
    A64_LDNP,
    A64_STNP,
    A64_LDNP_FP,
    A64_STNP_FP,

    // Load-acquire/store-release register     -- AM_SIMPLE
    // Load/store register pair (post-indexed) -- AM_POST
    // Load/store register pair (offset)       -- AM_OFF_IMM
    // Load/store register pair (pre-indexed)  -- AM_PRE
    A64_LDP, // LDP, LDXP
    A64_STP, // STP, STXP
    A64_LDP_FP,
    A64_STP_FP,

    // Load/store register (unprivileged): unsupported system instructions

    // Load register (literal)                      -- AM_LITERAL
    // Load-acquire/store-release register          -- AM_SIMPLE
    // Load-LOAcquire/Store-LORelease register      -- AM_SIMPLE
    // Load/store register (immediate post-indexed) -- AM_POST
    // Load/store register (immediate pre-indexed)  -- AM_PRE
    // Load/store register (register offset)        -- AM_OFF_REG, AM_OFF_EXT
    // Load/store register (unsigned immediate)     -- AM_OFF_IMM
    // Load/store register (unscaled immediate)     -- AM_OFF_IMM
    A64_LDR, // LDR, LDAR, LDLAR, LDUR
    A64_STR, // STR, STLR, STLLR, STUR
    A64_LDR_FP,
    A64_STR_FP,

    // Prefetch memory
    //
    // The exact prefetch operation is stored in Inst.rt := Rt.
    // We cannot use a "struct prfm" because the addressing mode-specific
    // data (offset, .extend) already occupies the space.
    //
    // PRFM (literal)          -- AM_LITERAL
    // PRFM (register)         -- AM_OFF_EXT
    // PRFM (immediate)        -- AM_OFF_IMM
    // PRFUM (unscaled offset) -- AM_OFF_IMM
    A64_PRFM,

    // Atomic memory operations
    //
    // Whether the instruction has load-acquire (e.g. LDADDA*), load-acquire/
    // store-release (e.g. LDADDAL*) or store-release (e.g. STADDL) semantics
    // is stored in ldst_order.load and .store.
    //
    // There are no ST* aliases; the only difference to the LD* instructions
    // is that the original value of the memory cell is discarded by writing
    // to the zero register.
    A64_LDADD,
    A64_LDCLR,
    A64_LDEOR,
    A64_LDSET,
    A64_LDSMAX,
    A64_LDSMIN,
    A64_LDUMAX,
    A64_LDUMIN,
    A64_SWP,
    A64_CAS,   // Compare and Swap (actually from Exclusive group)
    A64_CASP,  // Compare and Swap Pair of (double)words (actually from Exclusive group)

    /*** Data Processing -- Scalar Floating-Point and Advanced SIMD ***/

    // The instructions are ordered by functionality here, because the order of the
    // top-level encodings, as used in the other categories, splits variants of the
    // same instruction. We want as few opcodes as possible.

    // Conversion between Floating Point and Integer/Fixed-Point
    //
    // Sca: SIMD&FP register interpreted as a scalar (Hn, Sn, Dn).
    // Vec: SIMD&FP register interpreted as a vector (Vn.<T>).
    // GPR: General Purpose Register (Wn, Xn).
    //
    // Inst.flags.W32  := GPR bits == 32
    // Inst.flags.prec := Sca(fp) precision (FPSize)
    // Inst.flags.ext  := Vec(fp) vector arrangement
    // Inst.fcvt.mode  := rounding mode
    // Inst.fcvt.fbits := #fbits for fixed-point
    // Inst.fcvt.typ   := signed OR unsigned OR fixed-point
    A64_FCVT_GPR, // Sca(fp)        → GPR(int|fixed)
    A64_FCVT_VEC, // Vec(fp)        → Vec(int|fixed)
    A64_CVTF,     // GPR(int|fixed) → Sca(fp)
    A64_CVTF_VEC, // Vec(int|fixed) → Vec(fp)
    A64_FJCVTZS,  // Sca(f32)       → GPR(i32); special Javascript instruction

    // Rounding and Precision Conversion
    //
    // Inst.flags.prec := Sca(fp) precision
    // Inst.frint.mode := rounding mode
    // Inst.frint.bits := 0 if any size, 32, 64
    A64_FRINT,   // Round to integral (any size, 32-bit, or 64-bit)
    A64_FRINT_VEC,
    A64_FRINTX,  // ---- Exact (throws Inexact exception on failure)
    A64_FRINTX_VEC,
    A64_FCVT_H,  // Convert from any precision to Half
    A64_FCVT_S,  // -------------------------- to Single
    A64_FCVT_D,  // -------------------------- to Double
    A64_FCVTL,   // Extend to higher precision (vector)
    A64_FCVTN,   // Narrow to lower precision  (vector)
    A64_FCVTXN,  // Narrow to lower precision, round to odd (vector)

    // Floating-Point Computation (scalar)
    A64_FABS,
    A64_FNEG,
    A64_FSQRT,
    A64_FMUL,
    A64_FMULX,
    A64_FDIV,
    A64_FADD,
    A64_FSUB,
    A64_FMAX,   // max(n, NaN) → exception or FPSR flag set
    A64_FMAXNM, // max(n, NaN) → n
    A64_FMIN,   // min(n, NaN) → exception or FPSR flag set
    A64_FMINNM, // min(n, NaN) → n

    // Floating-Point Stepwise (scalar)
    A64_FRECPE,
    A64_FRECPS,
    A64_FRECPX,
    A64_FRSQRTE,
    A64_FRSQRTS,

    // Floating-Point Fused Multiply (scalar)
    A64_FNMUL,
    A64_FMADD,
    A64_FMSUB,
    A64_FNMADD,
    A64_FNMSUB,

    // Floating-Point Compare, Select, Move (scalar)
    A64_FCMP_REG,   // compare Rn, Rm
    A64_FCMP_ZERO,  // compare Rn and 0.0
    A64_FCMPE_REG,
    A64_FCMPE_ZERO,
    A64_FCCMP,
    A64_FCCMPE,
    A64_FCSEL,
    A64_FMOV_VEC2GPR, // GPR ← SIMD&FP reg, without conversion
    A64_FMOV_GPR2VEC, // GPR → SIMD&FP reg, ----
    A64_FMOV_TOP2GPR, // GPR ← SIMD&FP top half (of full 128 bits), ----
    A64_FMOV_GPR2TOP, // GPR → SIMD&FP top half (of full 128 bits), ----
    A64_FMOV_REG, // SIMD&FP ←→ SIMD&FP
    A64_FMOV_IMM, // SIMD&FP ← 8-bit float immediate (see VFPExpandImm)
    A64_FMOV_VEC, // vector ← 8-bit imm ----; replicate imm to all lanes

    // SIMD Floating-Point Compare
    A64_FCMEQ_REG,
    A64_FCMEQ_ZERO,
    A64_FCMGE_REG,
    A64_FCMGE_ZERO,
    A64_FCMGT_REG,
    A64_FCMGT_ZERO,
    A64_FCMLE_ZERO,
    A64_FCMLT_ZERO,
    A64_FACGE,
    A64_FACGT,

    // SIMD Simple Floating-Point Computation (vector <op> vector, vector <op> vector[i])
    A64_FABS_VEC,
    A64_FABD_VEC,
    A64_FNEG_VEC,
    A64_FSQRT_VEC,
    A64_FMUL_ELEM,
    A64_FMUL_VEC,
    A64_FMULX_ELEM,
    A64_FMULX_VEC,
    A64_FDIV_VEC,
    A64_FADD_VEC,
    A64_FCADD, // complex addition; Inst.imm := rotation in degrees (90, 270)
    A64_FSUB_VEC,
    A64_FMAX_VEC,
    A64_FMAXNM_VEC,
    A64_FMIN_VEC,
    A64_FMINNM_VEC,

    // SIMD Floating-Point Stepwise
    A64_FRECPE_VEC,
    A64_FRECPS_VEC,
    A64_FRSQRTE_VEC,
    A64_FRSQRTS_VEC,

    // SIMD Floating-Point Fused Multiply
    A64_FMLA_ELEM,
    A64_FMLA_VEC,
    A64_FMLAL_ELEM,
    A64_FMLAL_VEC,
    A64_FMLAL2_ELEM,
    A64_FMLAL2_VEC,
    A64_FCMLA_ELEM, // Inst.imm := rotation in degrees (0, 90, 180, 270)
    A64_FCMLA_VEC,  // ---
    A64_FMLS_ELEM,
    A64_FMLS_VEC,
    A64_FMLSL_ELEM,
    A64_FMLSL_VEC,
    A64_FMLSL2_ELEM,
    A64_FMLSL2_VEC,

    // SIMD Floating-Point Computation (reduce)
    A64_FADDP,
    A64_FADDP_VEC,
    A64_FMAXP,
    A64_FMAXP_VEC,
    A64_FMAXV,
    A64_FMAXNMP,
    A64_FMAXNMP_VEC,
    A64_FMAXNMV,
    A64_FMINP,
    A64_FMINP_VEC,
    A64_FMINV,
    A64_FMINNMP,
    A64_FMINNMP_VEC,
    A64_FMINNMV,

    // SIMD Bitwise: Logical, Pop Count, Bit Reversal, Byte Swap, Shifts
    A64_AND_VEC,
    A64_BCAX, // ARMv8.2-SHA
    A64_BIC_VEC_IMM,
    A64_BIC_VEC_REG,
    A64_BIF,
    A64_BIT,
    A64_BSL,
    A64_CLS_VEC,
    A64_CLZ_VEC,
    A64_CNT,
    A64_EOR_VEC,
    A64_EOR3,    // ARMv8.2-SHA
    A64_NOT_VEC, // also called MVN
    A64_ORN_VEC,
    A64_ORR_VEC_IMM,
    A64_ORR_VEC_REG,
    A64_MOV_VEC, // alias of ORR_VEC_REG
    A64_RAX1, // ARMv8.2-SHA
    A64_RBIT_VEC,
    A64_REV16_VEC,
    A64_REV32_VEC,
    A64_REV64_VEC,
    A64_SHL_IMM,
    A64_SHL_REG, // SSHL, USHL, SRSHL, URSHL
    A64_SHLL,    // SSHLL, USSHL
    A64_SHR,     // SSHR, USHR, SRSHR, URSHR
    A64_SHRN,    // SHRN, RSHRN
    A64_SRA,     // SSRA, USRA, SRSRA, URSRA
    A64_SLI,
    A64_SRI,
    A64_XAR, // ARMv8.2-SHA

    // SIMD Copy, Table Lookup, Transpose, Extract, Insert, Zip, Unzip
    //
    // Inst.imm := index i
    A64_DUP_ELEM, // ∀k < lanes: Dst[k] ← Src[i] (or if Dst is scalar: Dst ← Src[i])
    A64_DUP_GPR,  // ∀k < lanes: Dst[k] ← Xn
    A64_EXT,
    A64_INS_ELEM, // Dst[j] ← Src[i], (i, j stored in Inst.ins_elem)
    A64_INS_GPR,  // Dst[i] ← Xn
    A64_MOVI,     // includes MVNI
    A64_SMOV,     // Xd ← sext(Src[i])
    A64_UMOV,     // Xd ← Src[i]
    A64_TBL,      // Inst.imm := #regs of table ∈ {1,2,3,4}
    A64_TBX,      // ---
    A64_TRN1,
    A64_TRN2,
    A64_UZP1,
    A64_UZP2,
    A64_XTN,
    A64_ZIP1,
    A64_ZIP2,

    // SIMD Integer/Bitwise Compare
    A64_CMEQ_REG,
    A64_CMEQ_ZERO,
    A64_CMGE_REG,
    A64_CMGE_ZERO,
    A64_CMGT_REG,
    A64_CMGT_ZERO,
    A64_CMHI_REG,  // no ZERO variant
    A64_CMHS_REG,  // no ZERO variant
    A64_CMLE_ZERO, // no REG variant
    A64_CMLT_ZERO, // no REG variant
    A64_CMTST,

    // SIMD Integer Computation (vector <op> vector, vector <op> vector[i])
    //
    // Signedness (e.g. SABD vs UABD) is encoded via the SIMD_SIGNED flag,
    // rounding vs truncating behaviour (e.g. SRSHL vs SSHL) in SIMD_ROUND.
    A64_ABS_VEC,

    A64_ABD,
    A64_ABDL,
    A64_ABA,
    A64_ABAL,

    A64_NEG_VEC,

    A64_MUL_ELEM,
    A64_MUL_VEC,
    A64_MULL_ELEM,
    A64_MULL_VEC,

    A64_ADD_VEC,
    A64_ADDHN,
    A64_ADDL,
    A64_ADDW,
    A64_HADD,

    A64_SUB_VEC,
    A64_SUBHN,
    A64_SUBL,
    A64_SUBW,
    A64_HSUB,

    A64_MAX_VEC,
    A64_MIN_VEC,

    A64_DOT_ELEM,
    A64_DOT_VEC, // Inst.flags.vec = arrangement of destination (2s, 4s); sources are (8b, 16b)

    // SIMD Integer Stepwise (both are unsigned exclusive)
    A64_URECPE,
    A64_URSQRTE,

    // SIMD Integer Fused Multiply
    A64_MLA_ELEM,
    A64_MLA_VEC,
    A64_MLS_ELEM,
    A64_MLS_VEC,
    A64_MLAL_ELEM, // SMLAL, UMLAL
    A64_MLAL_VEC,  // SMLAL, UMLAL
    A64_MLSL_ELEM, // SMLSL, UMLSL
    A64_MLSL_VEC,  // SMLSL, UMLSL

    // SIMD Integer Computation (reduce)
    A64_ADDP,     // Scalar; Dd ← Vn.d[1] + Vn.d[0]
    A64_ADDP_VEC, // Concatenate Vn:Vm, then add pairwise and store result in Vd
    A64_ADDV,
    A64_ADALP,
    A64_ADDLP,
    A64_ADDLV,
    A64_MAXP,
    A64_MAXV,
    A64_MINP,
    A64_MINV,

    // SIMD Saturating Integer Arithmetic (unsigned, signed)
    A64_QADD,
    A64_QABS,
    A64_SUQADD,
    A64_USQADD,
    A64_QSHL_IMM,
    A64_QSHL_REG,
    A64_QSHRN,
    A64_QSUB,
    A64_QXTN,

    // SIMD Saturating Integer Arithmetic (signed exclusive)
    A64_SQABS,
    A64_SQADD,

    A64_SQDMLAL_ELEM,
    A64_SQDMLAL_VEC,
    A64_SQDMLSL_ELEM,
    A64_SQDMLSL_VEC,

    A64_SQDMULH_ELEM, // SQDMULH, SQRDMULH
    A64_SQDMULH_VEC,  // SQDMULH, SQRDMULH
    A64_SQDMULL_ELEM, // SQDMULL, SQRDMULL
    A64_SQDMULL_VEC,  // SQDMULL, SQRDMULL

    A64_SQNEG,

    // Only these rounded variations exist
    A64_SQRDMLAH_ELEM,
    A64_SQRDMLAH_VEC,
    A64_SQRDMLSH_ELEM,
    A64_SQRDMLSH_VEC,

    A64_SQSHLU,
    A64_SQSHRUN, // SQSHRUN, SQRSHRUN
    A64_SQXTUN,

    // SIMD Polynomial Multiply
    A64_PMUL,
    A64_PMULL,
}

#[repr(C)]
pub struct Inst {
    op: Op,
    flags: u8,
    rd: uint8_t,
    rn: uint8_t,
    union_rt_rm_rt2_rs: uint8_t,
    union_imm_fimm_offset_ra_error: u64,
    movk: Movk,
    bfm: Bfm,
    ccmp: Ccmp,
    sys: Sys,
    msr_imm: MsrImm,
    tbz: Tbz,
    shift: Shift,
    rmif: Rmif,
    extend: Extend,
    ldst_order: LdstOrder,
    simd_ldst: SimdLdst,
    fcvt: Fcvt,
    frint: Frint,
    ins_elem: InsElem,
    fcmla_elem: FcmlaElem,
}

#[repr(C)]
struct Movk {
    imm16: u32,
    lsl: u32,
}

#[repr(C)]
struct Bfm {
    lsb: u32,
    width: u32,
}

#[repr(C)]
struct Ccmp {
    nzcv: u32,
    imm5: u32,
}

#[repr(C)]
struct Sys {
    op1: u16,
    op2: u16,
    crn: u16,
    crm: u16,
}

#[repr(C)]
struct MsrImm {
    psfld: u32,
    imm: u32,
}

#[repr(C)]
struct Tbz {
    offset: int32_t,
    bit: u32,
}

#[repr(C)]
struct Shift {
    type_: u32,
    amount: u32,
}

#[repr(C)]
struct Rmif {
    mask: u32,
    ror: u32,
}

#[repr(C)]
struct Extend {
    type_: u32,
    lsl: u32,
}

#[repr(C)]
struct LdstOrder {
    load: u16,
    store: u16,
    rs: uint8_t,
}

#[repr(C)]
struct SimdLdst {
    nreg: u32,
    index: u16,
    offset: int16_t,
}

#[repr(C)]
struct Fcvt {
    mode: u32,
    fbits: u16,
    sgn: u16,
}

#[repr(C)]
struct Frint {
    mode: u32,
    bits: u32,
}

#[repr(C)]
struct InsElem {
    dst: u32,
    src: u32,
}

#[repr(C)]
struct FcmlaElem {
    idx: u32,
    rot: u32,
}