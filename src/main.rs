#[cfg(target_arch = "wasm32")]
fn main() {
    println!("Hello from your Xila WebAssembly app!");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Build this app for wasm32-wasip1 to run it in Xila.");
}
