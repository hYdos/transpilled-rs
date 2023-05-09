use std::mem::MaybeUninit;
use cty::uint32_t;
use crate::fast_arm_decoder_bindings::{fad_decode, Inst};

mod fast_arm_decoder_bindings;

fn main() {
    unsafe {
        let mut asm: uint32_t = 0xd10083ff; // sub sp, sp, #32
        let mut out = MaybeUninit::uninit();
        let n = fad_decode(&mut asm, 1, out.as_mut_ptr());
        let inst = out.assume_init();
        println!("Output: {}", n);
        println!("Hello, world!");
    }

}
