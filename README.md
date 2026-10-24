# Xila WebAssembly Application Template

Build your first **Rust WebAssembly (WASM) application for [Xila](https://github.com/Xila-Project/Core)**. This lightweight starter follows the same `wasm32-wasip1` target and guest-crate setup used by Xila's built-in applications, without relying on an unpublished SDK package.

Use this repository as a GitHub template, clone it, customize the Rust app, and build a `.wasm` guest for the Xila runtime.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo
- The `wasm32-wasip1` Rust target
- Git access to the [Xila Core repository](https://github.com/Xila-Project/Core)

Install the target with:

```sh
rustup target add wasm32-wasip1
```

## Create your application

1. Select **Use this template** on GitHub, or clone this repository directly.
2. Rename `xila-wasm-app` in `Cargo.toml` to your application’s crate name.
3. Update the package description and the greeting in `src/main.rs`.

The Xila guest dependency uses the Git repository directly because Xila is not yet published on crates.io. Cargo records the resolved Git revision in `Cargo.lock`; run `cargo update -p wasm` when you want to refresh it.

## Build

```sh
cargo build --release --target wasm32-wasip1
```

The WebAssembly binary is created at:

```text
target/wasm32-wasip1/release/xila-wasm-app.wasm
```

Copy the `.wasm` file into a running Xila system at `/applications/executables/xila-wasm-app`, then use Xila's shell to install and run it:

```sh
wasm --install /applications/executables/xila-wasm-app
wasm /applications/executables/xila-wasm-app
```

To check the WebAssembly app without producing an optimized binary, run:

```sh
cargo check
```

## GitHub template repository

Repository maintainers can enable the template button in GitHub: open **Settings → General**, check **Template repository**, and save. Once enabled, visitors can select **Use this template** to create a new repository with their own history.

## Learn more

- [Xila Core](https://github.com/Xila-Project/Core)
- [Xila built-in WASM applications](https://github.com/Xila-Project/Core/tree/main/executables)
- [Rust WebAssembly documentation](https://rustwasm.github.io/docs/book/)
