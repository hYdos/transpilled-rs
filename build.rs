use std::env;
use std::path::PathBuf;

fn main() {
    let llvm_install = "C:/LLVM-16".to_string();
    println!("cargo:include={}", "clibs/farmdec/src");
    println!("cargo:include={}", llvm_install.clone() + "/include");
    println!("cargo:rerun-if-changed=clibs/rellume/src/rellume.h");

    cc::Build::new()
        .cpp(false)
        .include("clibs/farmdec/src")
        .file("clibs/farmdec/src/decode.c")
        .compile("FastArmDecoder");

    cc::Build::new()
        .cpp(false)
        .flag("-std:c++17")
        .include("clibs/farmdec/src")
        .include("clibs/rellume/src")
        .include(llvm_install + "/include")
        .file("clibs/rellume/src/rellume.cc")
        .compile("Rellume");
}