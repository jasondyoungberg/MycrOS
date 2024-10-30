fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        // Tell cargo to pass the linker script to the linker..
        println!("cargo::rustc-link-arg-bins=-Tlinker.ld");
        // ..and to re-run if it changes.
        println!("cargo::rerun-if-changed=linker.ld");
    }
}
