const LLVM_SYS_PREFIX: &str = "LLVM_SYS_160_PREFIX";

fn main() {
    println!("cargo:rerun-if-env-changed={}", LLVM_SYS_PREFIX);

    if let Ok(path) = std::env::var(LLVM_SYS_PREFIX) {
        println!("cargo:rerun-if-changed={}", path);
    }

    println!("cargo:rerun-if-changed=clibs");

    cc::Build::new()
        .cpp(true)
        .include("clibs/farmdec/src")
        .file("clibs/farmdec/src/decode.c")
        .compile("FastArmDecoder");

    cc::Build::new()
        .cpp(true)
        .flag("-std:c++17")
        .include("clibs/farmdec/src")
        .include(format!("{}/include", std::env::var(LLVM_SYS_PREFIX).unwrap()))
        .define("LL_LLVM_MAJOR", "16")
        .define("RELLUME_WITH_AARCH64", "")
        .files(&vec![
            "clibs/rellume/src/a64/main.cc",
            "clibs/rellume/src/a64/simd.cc",
            "clibs/rellume/src/basicblock.cc",
            "clibs/rellume/src/callconv.cc",
            "clibs/rellume/src/facet.cc",
            "clibs/rellume/src/function.cc",
            "clibs/rellume/src/lldecoder.cc",
            "clibs/rellume/src/lifter-base.cc",
            "clibs/rellume/src/regfile.cc",
            "clibs/rellume/src/rellume.cc",
        ])
        .compile("rellume");
}
