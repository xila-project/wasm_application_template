<h1>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset=".github/brand/header-dark.svg">
  <img src=".github/brand/header-light.svg" alt="Xila WebAssembly app template — A Rust starter for your first Xila application." width="1280">
</picture>
</h1>

[Documentation](https://www.xila.dev/docs/) ·
[Try Xila](https://platforms.xila.dev/) ·
[Community](https://matrix.to/#/#xila:anneraud.fr)

[![vulnerabilities](https://vulnerabilities.xila.dev/badges/fd85d38863c4fb58de5fb2bbec143ef7.svg)](https://vulnerabilities.xila.dev/public/0778c05f0afea4ec05f2b0be46f704e4)

Build your first **Rust WebAssembly (WASM) application for [Xila](https://github.com/Xila-Project/Core)**. This lightweight starter follows the same `wasm32-wasip1` target and guest-crate setup used by Xila's built-in applications, without relying on an unpublished SDK package.

Use this repository as a GitHub template, clone it, customize the Rust app, and build a `.wasm` guest for the Xila runtime.

## Requirements

- [Rustup](https://www.rust-lang.org/tools/install) with Cargo
- Git access to the [Xila Core repository](https://github.com/Xila-Project/Core)

Rustup reads `rust-toolchain.toml` to select Rust 1.97.0 and install the
`wasm32-wasip1` target.

## Create your application

1. Select **Use this template** on GitHub, or clone this repository directly.
2. Rename `xila-wasm-app` in `Cargo.toml` to your application’s crate name.
3. Update the package description and the greeting in `src/main.rs`.

The Xila guest dependency uses the Git repository directly because Xila is not yet published on crates.io. Cargo records the resolved Git revision in `Cargo.lock`; run `cargo update -p wasm` when you want to refresh it.

## Build

```sh
cargo build --release --target wasm32-wasip1
```

The build script, `build.rs`, links the guest with 1-byte linear memory pages
and a 32 KiB stack, the settings of Xila's built-in applications. The initial
linear memory is then the stack plus the application's data, instead of a
multiple of 64 KiB plus Rust's default 1 MiB stack. The stack comes first in
memory, so an overflow stops the application with an out-of-bounds memory
access trap; raise `-zstack-size` in `build.rs` if your application recurses
deeply or keeps large buffers on the stack.

1-byte pages use the WebAssembly custom page sizes proposal. If you target a
Xila runtime that does not support it, change `--page-size=1` to
`--page-size=65536` in `build.rs`. If you run Binaryen's `wasm-opt` on the
guest, pass `--enable-custom-page-sizes`. Any guest code that calls
`memory.size` or `memory.grow` uses the linked page size as its unit.

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

## Development

**Rust version.** Use Rust 1.97.0 for development and CI builds and declare 1.97
as the MSRV (`rust-version`), following the esp-rs Xtensa toolchain base. Use
only language features and `std` or `core` APIs available in Rust 1.97. Raise
the version only when esp-rs publishes a new base, and update every Xila
repository together. Rust native executables that use `build-std` use a
separate nightly toolchain ([xila-project/core#687](https://github.com/xila-project/core/issues/687)).

## GitHub template repository

Repository maintainers can enable the template button in GitHub: open **Settings → General**, check **Template repository**, and save. Once enabled, visitors can select **Use this template** to create a new repository with their own history.

## Learn more

- [Xila Core](https://github.com/Xila-Project/Core)
- [Xila built-in WASM applications](https://github.com/Xila-Project/Core/tree/main/executables)
- [Rust WebAssembly documentation](https://rustwasm.github.io/docs/book/)

## License

This template is released under the [MIT License](LICENSE), except the `.github/brand/` header assets, which are covered by their own notice. Applications created from it may use any license.

<!-- xila:footer:start -->

---

Part of **[Xila](https://www.xila.dev/)** ·
[Repositories](https://github.com/xila-project) ·
[Community](https://matrix.to/#/#xila:anneraud.fr)

Special thanks to [Martin Legand](https://www.behance.net/martinlegand) for generously designing the Xila logo.

<!-- xila:footer:end -->
