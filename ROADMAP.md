# Roadmap: material-color-rs 0.1.0 -> 1.0.0

Цель проекта - полностью перенести функциональность `material-color-utilities/cpp` на чистый Rust, без runtime-зависимости от C++, и предоставить стабильную интеграцию для Rust, C++, C и Python проектов через Bazel и Cargo.

## Основные принципы

- Rust core должен быть единственным источником алгоритмов.
- C/C++/Python bindings не должны дублировать цветовую логику.
- Публичные API сохраняются простыми, типобезопасными и FFI-friendly.
- Поведение должно совпадать с исходной C++ реализацией в пределах явно заданных floating-point допусков.
- Каждый перенесенный модуль получает regression/golden tests до перехода к следующему крупному этапу.
- Bazel и Cargo должны собирать одну и ту же Rust-реализацию.

## 0.1.0 - Project Scaffold

Статус: завершено.

Scope:
- Создать структуру workspace.
- Разделить Rust core, C ABI, Python extension и C++ wrapper.
- Подготовить пустые Bazel/Cargo/build/test/example директории.

Exit criteria:
- Есть проектная структура.
- Нет алгоритмического кода.
- Roadmap зафиксирован в репозитории.

## 0.2.0 - Core Color Primitives

Статус: завершено.

Scope:
- `Argb` и RGB channel helpers.
- Базовые math utilities.
- Linearized/delinearized RGB.
- XYZ/Lab helper conversions.
- `L*`, `Y`, grayscale conversion.
- Hex formatting/parsing, если нужен публичный API.

Source modules:
- `cpp/utils`

Tests:
- Port `utils_test.cc`.
- Добавить edge cases: alpha, channel bounds, hue wrapping, tone bounds.

Exit criteria:
- `cargo test -p material_color` проходит для primitives.
- `bazel test //crates/material_color:all` или эквивалентный target проходит.
- Нет `unsafe` в core.

## 0.3.0 - CAM16 and HCT

Статус: текущий этап.

Scope:
- `ViewingConditions`.
- CAM16 conversion.
- HCT model.
- HCT solver.
- HCT mutation/builders в Rust-идиоматичном виде.

Source modules:
- `cpp/cam/viewing_conditions`
- `cpp/cam/cam`
- `cpp/cam/hct`
- `cpp/cam/hct_solver`

Tests:
- Port `cam_test.cc`, `hct_test.cc`, `hct_solver_test.cc`.
- Golden compatibility vectors against C++ output.
- Stress tests для hue/chroma/tone boundary values.

Exit criteria:
- HCT/CAM16 output matches C++ within documented tolerances.
- Public Rust API exposes stable value types, not C++-style mutable classes.

## 0.4.0 - Palettes, Contrast, Blend, Dislike

Scope:
- Tonal palettes.
- Key color calculation.
- Core palette model.
- Contrast utilities.
- Blend/harmonize functions.
- Dislike detection and correction.

Source modules:
- `cpp/palettes`
- `cpp/contrast`
- `cpp/blend`
- `cpp/dislike`

Tests:
- Port `tones_test.cc`, `contrast_test.cc`, `blend_test.cc`, `dislike_test.cc`.
- Add public API examples for palette and contrast workflows.

Exit criteria:
- Palette generation is deterministic.
- Contrast behavior matches C++.
- No dependency on dynamic scheme code yet.

## 0.5.0 - Dynamic Material Schemes

Scope:
- `Variant`.
- `DynamicScheme`.
- Material color roles.
- Role evaluation without exposing dynamic closures in public API.
- Scheme constructors:
  - Monochrome
  - Neutral
  - Tonal Spot
  - Vibrant
  - Expressive
  - Fidelity
  - Content
  - Rainbow
  - Fruit Salad

Source modules:
- `cpp/dynamiccolor`
- `cpp/scheme`

Rust API direction:
- Prefer `DynamicColorRole` enum plus `DynamicScheme::color(role)`.
- Keep scheme data immutable after construction.
- Avoid `Box<dyn Fn>` in public FFI-facing API.

Tests:
- Port all available scheme/dynamiccolor tests.
- Add full role snapshot tests for light/dark and contrast levels.
- Verify every Material 3 role is present.

Exit criteria:
- All Material dynamic colors can be generated from source color, variant, dark mode and contrast level.
- Role outputs match C++ golden data.

## 0.6.0 - Quantization, Scoring, Temperature

Scope:
- Lab quantization helper.
- Wu quantizer.
- WSMeans quantizer.
- Celebi quantizer.
- Ranked color suggestions.
- Temperature cache, analogous colors and complements.

Source modules:
- `cpp/quantize`
- `cpp/score`
- `cpp/temperature`

Tests:
- Port `wu_test.cc`, `wsmeans_test.cc`, `celebi_test.cc`, `score_test.cc`, `temperature_cache_test.cc`.
- Add image-like fixture tests with repeated ARGB populations.
- Validate deterministic behavior for quantization.

Exit criteria:
- Full algorithmic parity with C++ modules.
- Quantize/score APIs support both raw pixels and population maps.
- Performance is acceptable for typical wallpaper/image extraction workloads.

## 0.7.0 - C ABI and C++ Integration

Scope:
- Define stable C ABI over Rust core.
- Generate or maintain `material_color.h`.
- Add C++ RAII wrapper in `material_color.hpp`.
- Bazel targets for static and shared libraries.
- C++ example project.

API constraints:
- C ABI uses only primitive types, explicit structs and explicit memory ownership.
- No Rust panics cross FFI.
- All FFI functions validate pointers and lengths.
- Allocation returned across FFI has matching free functions.

Tests:
- C ABI smoke tests.
- C++ wrapper tests.
- Bazel integration test linking C++ binary against Rust library.

Exit criteria:
- Existing C++ projects can consume the Rust implementation through Bazel.
- No C++ algorithm implementation remains required.

## 0.8.0 - Python Integration

Scope:
- PyO3 extension module.
- Python-facing wrappers for:
  - `Argb`
  - `Hct`
  - `TonalPalette`
  - `DynamicScheme`
  - `Variant`
  - quantize/score functions
- Python package metadata.
- Bazel and wheel build path.

Tests:
- Python import smoke test.
- Python unit tests for representative workflows.
- Compare Python outputs to Rust golden snapshots.

Exit criteria:
- Python package can be built and imported locally.
- Python API does not duplicate Rust algorithms.

## 0.9.0 - Compatibility, Documentation, Hardening

Scope:
- Full compatibility test suite against C++ reference data.
- Documentation for Rust, C ABI, C++ and Python usage.
- Bazel module documentation.
- Cargo package documentation.
- Error handling cleanup.
- Benchmark suite for expensive algorithms.
- API review before stabilization.

Tests:
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
- `bazel test //...`
- C++ example build
- Python wheel/import smoke test

Exit criteria:
- No known algorithmic gaps vs `material-color-utilities/cpp`.
- Public API reviewed for 1.0 stability.
- CI covers Cargo, Bazel, C++, Python.

## 1.0.0 - Stable Release

Scope:
- Stabilize Rust public API.
- Stabilize C ABI versioning.
- Stabilize C++ wrapper API.
- Stabilize Python package API.
- Publish release artifacts as applicable.
- Document migration path from C++ implementation to Rust implementation.

Release criteria:
- All C++ functionality is ported to pure Rust.
- No runtime dependency on original C++ library.
- All tests pass under Cargo and Bazel.
- Bindings are generated from or backed by Rust core only.
- Versioned API compatibility policy is documented.
- Known limitations are documented.

## Post-1.0

Potential follow-up work:
- `no_std` subset for embedded use, if practical.
- WASM bindings.
- SIMD or parallel quantization optimization.
- Optional serde support for schemes and palettes.
- Fuzz testing for color conversion and FFI inputs.
- Additional language bindings generated from the C ABI.
