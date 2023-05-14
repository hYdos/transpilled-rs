use std::os::raw::c_uchar;

use libc::{c_char, c_int, c_uint, c_ulonglong, c_void};
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};

#[repr(C)]
pub struct LLConfig;/* {
    enableOverflowIntrinsics: bool,
    enableFastMath: bool,
    call_ret_clobber_flags: bool,
    use_native_segment_base: bool,
    full_facets: bool,
    verify_ir: bool,
    position_independent_code: bool,
    arch: Arch,
    callconv: CallConv,
    sptr_addrspace: unsigned,
    global_base_addr: u32,
    global_base_value: llvm::Value*,
    pc_base_addr: u32,
    pc_base_value: llvm::Value*,
    instr_overrides: std::unordered_map<u32, llvm::Function*>,
    tail_function: llvm::Function*,
    call_function: llvm::Function*,
    syscall_implementation: llvm::Function*,
    cpuinfo_function: llvm::Function*,
    instr_marker: llvm::Function*,
};*/

#[repr(C)]
pub struct LLFunc {}

pub type RellumeMemAccessCb = Option<unsafe extern "C" fn(usize) -> usize>;

#[link(name = "rellume")]
extern "C" {
    pub fn ll_config_new() -> *mut LLConfig;
    pub fn ll_config_free(config: *mut LLConfig);
    pub fn ll_config_set_hhvm(config: *mut LLConfig, hhvm: bool);
    pub fn ll_config_set_sptr_addrspace(config: *mut LLConfig, addrspace: c_uint);
    pub fn ll_config_enable_overflow_intrinsics(config: *mut LLConfig, enable: bool);
    pub fn ll_config_enable_fast_math(config: *mut LLConfig, enable: bool);
    pub fn ll_config_enable_verify_ir(config: *mut LLConfig, enable: bool);
    pub fn ll_config_set_position_independent_code(config: *mut LLConfig, pic: bool);
    pub fn ll_config_set_pc_base(config: *mut LLConfig, base: c_ulonglong, value: LLVMValueRef);
    pub fn ll_config_set_global_base(config: *mut LLConfig, base: c_ulonglong, value: LLVMValueRef);
    pub fn ll_config_set_instr_impl(config: *mut LLConfig, index: c_uint, value: LLVMValueRef);
    pub fn ll_config_set_tail_func(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_call_func(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_syscall_impl(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_cpuinfo_func(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_instr_marker(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_call_ret_clobber_flags(config: *mut LLConfig, value: bool);
    pub fn ll_config_set_use_native_segment_base(config: *mut LLConfig, value: bool);
    pub fn ll_config_enable_full_facets(config: *mut LLConfig, enable: bool);
    pub fn ll_config_set_architecture(config: *mut LLConfig, architecture: *const c_char) -> bool;

    pub fn ll_func_new(mod_: LLVMModuleRef, config: *mut LLConfig) -> *mut LLFunc;
    pub fn ll_func_lift(func: *mut LLFunc) -> LLVMValueRef;
    pub fn ll_func_dispose(func: *mut LLFunc);
    pub fn ll_func_add_instr(
        func: *mut LLFunc,
        block_addr: c_ulonglong,
        addr: c_ulonglong,
        bufsz: usize,
        buf: *const c_uchar,
    ) -> c_int;

    pub fn RellumeMemAccessCb(
        size: usize,
        buf: *mut c_uchar,
        size2: usize,
        user_arg: *mut c_void,
    ) -> usize;

    pub fn ll_func_decode_instr(
        func: *mut LLFunc,
        addr: c_ulonglong,
        cb: RellumeMemAccessCb,
        user_arg: *mut c_void,
    ) -> c_int;

    pub fn ll_func_decode_block(
        func: *mut LLFunc,
        addr: c_ulonglong,
        cb: RellumeMemAccessCb,
        user_arg: *mut c_void,
    ) -> c_int;

    pub fn ll_func_decode_cfg(
        func: *mut LLFunc,
        addr: c_ulonglong,
        cb: RellumeMemAccessCb,
        user_arg: *mut c_void,
    ) -> c_int;
}