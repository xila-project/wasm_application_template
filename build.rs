//! Linker settings for the WebAssembly guest.
//!
//! A build script applies them even when the `RUSTFLAGS` environment variable
//! is set, which would replace `rustflags` from `.cargo/config.toml`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        // 1-byte linear memory pages (custom page sizes proposal): the initial
        // memory is exactly what the module needs instead of a multiple of
        // 64 KiB. Use 65536 for a runtime without the proposal.
        println!("cargo:rustc-link-arg-bins=--page-size=1");
        // The stack is placed first in memory, so an overflow traps instead of
        // corrupting data. Without this, Rust reserves 1 MiB. Raise it if your
        // application has deep recursion or large local buffers.
        println!("cargo:rustc-link-arg-bins=-zstack-size=32768");
    }
}
