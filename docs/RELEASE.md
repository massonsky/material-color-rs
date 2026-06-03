# Release 1.0.0

## Release Artifacts

Build commands:

```sh
cargo package -p material_color --list --offline
cargo package -p material_color_capi --list --offline
cargo package -p material_color_py --list --offline
cargo build -p material_color_py --release --offline
bazel build //examples:cpp_dynamic_scheme
```

Wheel builds are configured through `crates/material_color_py/pyproject.toml`
and maturin:

```sh
cd crates/material_color_py
maturin build --release
```

`maturin` is not vendored by this repository.

## Required Checks

```sh
cargo fmt --check
cargo test --workspace --offline
cargo clippy --workspace --all-targets --offline
cargo doc --workspace --offline --no-deps
cargo run -p material_color --example benchmark --release --offline
bazel test //...
bazel build //examples:cpp_dynamic_scheme
```

## Version Surface

- Rust: `material_color::VERSION`.
- C: `MATERIAL_COLOR_VERSION*` macros and `material_color_version_*()`.
- C++: `material_color::kHeaderVersion` and `material_color::RuntimeVersion()`.
- Python: `material_color_py.__version__`, `VERSION`, `VERSION_*` and
  `version()`.

## Known Follow-Up Work

- `no_std` subset for embedded use.
- WASM bindings.
- SIMD or parallel quantization optimization.
- Optional serde support for schemes and palettes.
- Fuzz testing for color conversion and FFI inputs.
- Additional language bindings generated from the C ABI.
