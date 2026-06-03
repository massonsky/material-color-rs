# Bazel Integration

The Bazel workspace builds the same Rust implementation used by Cargo.

## Module

`MODULE.bazel` exposes the module as:

```starlark
module(
    name = "material_color_rs",
    version = "1.0.0",
)
```

The C++ wrapper uses `rules_cc`.

## Targets

- `//crates/material_color:srcs` - Rust core source filegroup.
- `//crates/material_color_capi:material_color_capi_static` - Rust C ABI static
  library imported as a C/C++ dependency.
- `//bindings/cpp:material_color_cpp` - C++ wrapper library.
- `//crates/material_color_py:material_color_py_extension` - PyO3 extension
  module built through Cargo.
- `//examples:cpp_dynamic_scheme` - C++ example binary.
- `//tests:cpp_integration_smoke_test` - C++ link and workflow smoke test.
- `//tests:python_import_smoke_test` - Python extension import and workflow
  smoke test.

## Commands

```sh
bazel test //...
bazel build //examples:cpp_dynamic_scheme
```

The Rust genrules run Cargo in offline locked mode and expect the crate index and
dependencies to be available in `CARGO_HOME`. `.bazelrc` passes `CARGO_HOME`
into actions when it is set; local development falls back to `/home/dev/.cargo`.
CI should populate and export `CARGO_HOME` before Bazel tests.
