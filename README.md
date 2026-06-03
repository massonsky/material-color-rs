# material-color-rs

Pure Rust port of the Material Design color utilities algorithms, with shared
bindings for Rust, C, C++ and Python.

The Rust crate is the single source of color logic. C ABI, C++ and Python
bindings call into the Rust implementation and do not duplicate algorithms.

## Status

Current milestone: `1.0.0`.

## Attribution

This project is an independent Rust port of Google's
[Material Color Utilities](https://github.com/material-foundation/material-color-utilities),
the color algorithm library behind dynamic color in
[Material Design](https://m3.material.io/).

Official upstream references:

- [Material Color Utilities](https://github.com/material-foundation/material-color-utilities)
- [Material Design color styles](https://m3.material.io/styles/color)
- [Material Theme Builder](https://material-foundation.github.io/material-theme-builder/)

Material Design, Material You and related names are trademarks of Google LLC.
This project is not affiliated with, endorsed by, sponsored by or maintained by
Google LLC. See [NOTICE](NOTICE) for full upstream attribution and trademark
notes.

Implemented modules:

- ARGB utilities, L*, XYZ/Lab helpers and tone conversion.
- CAM16, HCT and HCT solver.
- Tonal palettes, core palettes, contrast, blend and dislike correction.
- Material dynamic schemes and all 54 dynamic color roles.
- Quantization, scoring and temperature utilities.
- C ABI, C++ wrapper and PyO3 extension module.

## Rust

```rust
use material_color::argb::Argb;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};

let source = Argb::new(0xff42_85f4);
let scheme = DynamicScheme::from_argb(source, Variant::TonalSpot, false, 0.0);
let primary = scheme.color(DynamicColorRole::Primary);

assert_eq!(primary.to_u32(), 0xff44_5e91);
assert_eq!(material_color::VERSION, "1.0.0");
```

Useful commands:

```sh
cargo run -p material_color --example dynamic_scheme --offline
cargo test --workspace --offline
cargo clippy --workspace --all-targets --offline
cargo run -p material_color --example benchmark --release --offline
```

## C and C++

The C ABI is defined in
`bindings/c/include/material_color/material_color.h`. C++ users can include the
RAII wrapper:

```cpp
#include "material_color/material_color.hpp"

auto primary = material_color::DynamicSchemeColor(
    0xff4285f4, MATERIAL_COLOR_VARIANT_TONAL_SPOT, false, 0.0,
    MATERIAL_COLOR_ROLE_PRIMARY);
```

Build the C++ example:

```sh
bazel build //examples:cpp_dynamic_scheme
bazel run //examples:cpp_extract_colors
```

## Python

The Python module is implemented with PyO3.

```python
import material_color_py as mc

scheme = mc.DynamicScheme(0xFF4285F4, mc.Variant.TONAL_SPOT, False, 0.0)
assert scheme.color(mc.DynamicColorRole.PRIMARY) == 0xFF445E91
```

Local extension smoke path:

```sh
cargo build -p material_color_py --release --offline
mkdir -p /tmp/material_color_py
cp target/release/libmaterial_color_py.so /tmp/material_color_py/material_color_py.so
PYTHONPATH=/tmp/material_color_py python3 tests/python/material_color_py_smoke.py
```

Wheel builds use `crates/material_color_py/pyproject.toml` with maturin.

More examples are available in [examples/](examples/).

## Bazel

Run the full Bazel smoke suite:

```sh
bazel test //...
```

See [docs/BAZEL.md](docs/BAZEL.md) for integration notes.

## Compatibility

Representative C++ reference snapshots are covered by Rust unit tests,
integration snapshots, C++ smoke tests and Python import workflow tests. See
[docs/COMPATIBILITY.md](docs/COMPATIBILITY.md).

## API Policy

`1.x` follows the compatibility policy in
[docs/API_STABILITY.md](docs/API_STABILITY.md). Migration notes from the C++
implementation are in [docs/MIGRATION.md](docs/MIGRATION.md).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) and
[NOTICE](NOTICE).
