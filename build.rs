fn main() {
    cc::Build::new()
        .cpp(false)
        .include("clibs")
        .file("clibs/decode.c")
        .compile("FastArmDecoder");
}