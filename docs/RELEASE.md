# Release 1.0.0

## Release Artifacts

The GitHub Actions release workflow is tag-driven:

```sh
git tag -a v1.0.0 -m "material color 1.0.0"
git push origin v1.0.0
```

It builds:

- Cargo `.crate` packages.
- PyO3 wheels for Linux, macOS and Windows.
- Native C ABI/C++ wrapper archives for Linux, macOS and Windows.
- `SHA256SUMS.txt`.

Build commands:

```sh
cargo package -p material_color --list --offline
cargo build -p material_color_py --release --offline
bazel build //examples:cpp_dynamic_scheme
```

`material_color_capi` and `material_color_py` depend on the published
`material_color` crate for registry verification. The GitHub release workflow
ships their production artifacts as native library archives and Python wheels;
their `.crate` packages can be published after `material_color` is available in
the registry.

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
