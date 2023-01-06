fn main() {
    // Save target for update command
    println!(
        "cargo:rustc-env=FUCKBF_TARGET_OS={}",
        &std::env::var("TARGET").unwrap()
    );
}
