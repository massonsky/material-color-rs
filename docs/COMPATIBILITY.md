# Compatibility

The project targets behavioral parity with `material-color-utilities/cpp` while
using pure Rust as the only algorithm implementation.

## Covered Surface

- `cpp/utils` -> `material_color::utils`, `Argb`.
- `cpp/cam` -> `material_color::cam`.
- `cpp/palettes`, `cpp/contrast`, `cpp/blend`, `cpp/dislike`.
- `cpp/dynamiccolor`, `cpp/scheme`.
- `cpp/quantize`, `cpp/score`, `cpp/temperature`.
- C ABI, C++ wrapper and Python extension call into the Rust core.

## Test Layers

- Rust unit tests port C++ representative values and boundary behavior.
- `crates/material_color/tests/compatibility_snapshots.rs` covers public
  end-to-end snapshots across HCT, palettes, dynamic schemes, quantization,
  scoring, blending and temperature.
- C ABI tests validate status handling, pointer validation and allocation
  ownership.
- `//tests:cpp_integration_smoke_test` validates C++ linkage through Bazel.
- `//tests:python_import_smoke_test` validates PyO3 extension import and
  representative Python workflows.

## Known Limitations Before 1.0

- No `no_std` support.
- No WASM binding.
- No SIMD or parallel quantization path.
- Python wheel publication is configured through maturin but not published by
  this repository yet.
- Fuzz testing is planned after the stable API surface is finalized.

## Required Release Checks

```sh
cargo fmt --check
cargo test --workspace --offline
cargo clippy --workspace --all-targets --offline
cargo build -p material_color_py --release --offline
bazel test //...
bazel build //examples:cpp_dynamic_scheme
```
