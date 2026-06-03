# API Stability

`0.9.x` is the API review window before the `1.0.0` stable release.

## Rust Core

- `material_color` is the source of algorithmic truth.
- Safe Rust remains required in the core crate.
- Public value types should stay small, copyable where practical and explicit.
- New public APIs should prefer typed enums and structs over stringly typed
  parameters.
- Existing public constructors and role/variant names should be preserved unless
  a rename is required before `1.0.0`.

## C ABI

- C ABI functions use primitive values, `repr(C)` structs and explicit status
  codes.
- Rust panics are caught at FFI boundaries and returned as
  `MATERIAL_COLOR_STATUS_PANIC`.
- Caller-provided output pointers are validated.
- Rust-owned arrays returned through C ABI must be released with the matching
  `material_color_*_free` function.
- `material_color_version_*` is derived from Cargo package metadata.

## C++

- The C++ wrapper is a thin RAII layer over the C ABI.
- It must not implement color algorithms.
- Owned arrays are move-only and release through the C ABI.
- Wrapper helpers may throw `std::runtime_error` for non-OK C ABI status values.

## Python

- Python bindings are PyO3 wrappers over Rust core.
- Python constants mirror Rust `Variant` and `DynamicColorRole` values.
- Python APIs return plain Python integers for ARGB values and dictionaries for
  population maps.
- The extension should keep accepting wrapper objects and integer enum values
  for roles and variants through `1.0.0`.

## Pre-1.0 Review Checklist

- No known algorithmic gaps against the ported C++ modules.
- Public role and variant inventories are complete.
- C ABI version, status and ownership rules are documented.
- Cargo, Bazel, C++ and Python smoke tests pass in CI.
- Known limitations are documented before release.
