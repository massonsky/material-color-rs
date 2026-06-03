# Examples

Runnable examples for the Rust core, C++ wrapper and Python extension.

## Rust

```sh
cargo run -p material_color --example dynamic_scheme --offline
cargo run -p material_color --example benchmark --release --offline
```

The source mirrored for the release tree is in `examples/rust/dynamic_scheme.rs`.

## C++

```sh
bazel run //examples:cpp_dynamic_scheme
bazel run //examples:cpp_extract_colors
```

## Python

Build the extension and run the example directly:

```sh
cargo build -p material_color_py --release --offline
mkdir -p /tmp/material_color_py
cp target/release/libmaterial_color_py.so /tmp/material_color_py/material_color_py.so
PYTHONPATH=/tmp/material_color_py python3 examples/python/dynamic_scheme.py
```

Or validate it through Bazel:

```sh
bazel test //examples:python_dynamic_scheme_example_test
```
