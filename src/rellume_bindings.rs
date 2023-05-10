use std::os::raw::c_uchar;

use libc::{c_char, c_int, c_size_t, c_uint, c_ulonglong, c_void};
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};

#[repr(C)]
pub struct LLConfig;

#[repr(C)]
pub struct LLFunc;


#[link(name = "llvm")]
extern "C" {
    pub fn ll_config_new() -> *mut LLConfig;
    pub fn ll_config_free(config: *mut LLConfig);
    pub fn ll_config_set_hhvm(config: *mut LLConfig, hhvm: c_bool);
    pub fn ll_config_set_sptr_addrspace(config: *mut LLConfig, addrspace: c_uint);
    pub fn ll_config_enable_overflow_intrinsics(config: *mut LLConfig, enable: c_bool);
    pub fn ll_config_enable_fast_math(config: *mut LLConfig, enable: c_bool);
    pub fn ll_config_enable_verify_ir(config: *mut LLConfig, enable: c_bool);
    pub fn ll_config_set_position_independent_code(config: *mut LLConfig, pic: c_bool);
    pub fn ll_config_set_pc_base(config: *mut LLConfig, base: c_ulonglong, value: LLVMValueRef);
    pub fn ll_config_set_global_base(config: *mut LLConfig, base: c_ulonglong, value: LLVMValueRef);
    pub fn ll_config_set_instr_impl(config: *mut LLConfig, index: c_uint, value: LLVMValueRef);
    pub fn ll_config_set_tail_func(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_call_func(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_syscall_impl(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_cpuinfo_func(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_instr_marker(config: *mut LLConfig, value: LLVMValueRef);
    pub fn ll_config_set_call_ret_clobber_flags(config: *mut LLConfig, value: c_bool);
    pub fn ll_config_set_use_native_segment_base(config: *mut LLConfig, value: c_bool);
    pub fn ll_config_enable_full_facets(config: *mut LLConfig, enable: c_bool);
    pub fn ll_config_set_architecture(config: *mut LLConfig, architecture: *const c_char) -> c_bool;

    pub fn ll_func_new(mod_: LLVMModuleRef, config: *mut LLConfig) -> *mut LLFunc;
    pub fn ll_func_lift(func: *mut LLFunc) -> LLVMValueRef;
    pub fn ll_func_dispose(func: *mut LLFunc);
    pub fn ll_func_add_instr(
        func: *mut LLFunc,
        block_addr: c_ulonglong,
        addr: c_ulonglong,
        bufsz: c_size_t,
        buf: *const c_uchar,
    ) -> c_int;

    pub type RellumeMemAccessCb;

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