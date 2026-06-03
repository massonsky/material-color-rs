# API Stability

`1.x` is the stable API line.

SemVer applies to Rust crates and Python package metadata. The C ABI and C++
wrapper follow the same compatibility intent: compatible additions can happen in
minor releases, while breaking ABI or API changes require a new major version.

## Rust Core

- `material_color` is the source of algorithmic truth.
- Safe Rust remains required in the core crate.
- Public value types should stay small, copyable where practical and explicit.
- New public APIs should prefer typed enums and structs over stringly typed
  parameters.
- Existing public constructors and role/variant names are stable for `1.x`.
- `material_color::VERSION` exposes the crate version.

## C ABI

- C ABI functions use primitive values, `repr(C)` structs and explicit status
  codes.
- Rust panics are caught at FFI boundaries and returned as
  `MATERIAL_COLOR_STATUS_PANIC`.
- Caller-provided output pointers are validated.
- Rust-owned arrays returned through C ABI must be released with the matching
  `material_color_*_free` function.
- `material_color_version_*` is derived from Cargo package metadata.
- `MATERIAL_COLOR_VERSION*` header macros expose compile-time header versioning.

## C++

- The C++ wrapper is a thin RAII layer over the C ABI.
- It must not implement color algorithms.
- Owned arrays are move-only and release through the C ABI.
- Wrapper helpers may throw `std::runtime_error` for non-OK C ABI status values.
- `kHeaderVersion` and `RuntimeVersion()` expose compile-time and linked runtime
  versions.

## Python

- Python bindings are PyO3 wrappers over Rust core.
- Python constants mirror Rust `Variant` and `DynamicColorRole` values.
- Python APIs return plain Python integers for ARGB values and dictionaries for
  population maps.
- The extension should keep accepting wrapper objects and integer enum values
  for roles and variants through `1.x`.
- `__version__`, `VERSION`, `VERSION_*` and `version()` expose package versioning.

## Release Checklist

- No known algorithmic gaps against the ported C++ modules.
- Public role and variant inventories are complete.
- C ABI version, status and ownership rules are documented.
- Cargo, Bazel, C++ and Python smoke tests pass in CI.
- Known limitations are documented before release.
