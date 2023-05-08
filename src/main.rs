use cty::uint32_t;
use crate::fast_arm_decoder_bindings::{fad_decode, Inst};

mod fast_arm_decoder_bindings;

fn main() {
    let asm: uint32_t = 0xd10083ff; // sub sp, sp, #32
    let mut out: Inst;
    let n = unsafe { fad_decode(asm, 1, &mut out) };
    println!("Output: {}", n);

    println!("Hello, world!");
}
