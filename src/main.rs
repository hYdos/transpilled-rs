extern crate core;

use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

use cty::uint32_t;
use llvm_sys::core::{LLVMDumpValue, LLVMModuleCreateWithName};

use crate::fast_arm_decoder_bindings::{fad_decode, Inst};

mod fast_arm_decoder_bindings;
mod rellume_bindings;

fn main() {
    #![feature(extern_types)]

    // Test FastArmDecoder
    unsafe {
        let mut asm: uint32_t = 0xd10083ff; // sub sp, sp, #32
        let mut out = MaybeUninit::uninit();
        let n = fad_decode(&mut asm, 1, out.as_mut_ptr());
        let inst = out.assume_init();
        println!("FastArmDecoder Return Code: {}", n);
        // println!("FastArmDecoder Inst Opcode: {}", inst.op);
    }

    // Test Rellume-Rs
    unsafe {
        // Create LLVM module
        let module_name = CString::new("lifter").unwrap();
        let module = LLVMModuleCreateWithName(module_name.as_ptr());

        // https://armconverter.com/
        //int main() {
        //     int i = 0;
        //     i += 1;
        //     int j = 1 * 2;
        //     return j;
        // }
        let code = [ // main: @main
            0xFF4300D1,      //   sub sp, sp, #16
            0xFF0F00B9,      //   str wzr, [sp, #12]
            0xFF0B00B9,      //   str wzr, [sp, #8]
            0xE80B40B9,      //   ldr w8, [sp, #8]
            0x08050011,      //   add w8, w8, #1
            0xE80B00B9,      //   str w8, [sp, #8]
            0x48008052,      //   mov w8, #2
            0xE80700B9,      //   str w8, [sp, #4]
            0xE00740B9,      //   ldr w0, [sp, #4]
            0xFF430091,      //   add sp, sp, #16
            0xC0035FD6,      //   ret
        ];

        // Create function for lifting
        let cfg = ll_config_new();
        ll_config_set_architecture(cfg, "aarch64");
        ll_config_set_call_ret_clobber_flags(cfg, true);
        let func = ll_func_new(module, cfg);
        // Lift the whole function by following all direct jumps
        ll_func_decode_cfg(func, code.as_ptr(), NULL, NULL);
        let llvm_fn = ll_func_lift(func);
        LLVMDumpValue(llvm_fn);
    }
}
