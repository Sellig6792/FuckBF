fn main() {
    // Save target for update command
    println!(
        "cargo:rustc-env=FUCKBRAINFUCK_TARGET_OS={}",
        &std::env::var("TARGET").unwrap()
    );
}
