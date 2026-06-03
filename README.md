# material-color-rs

Pure Rust port of the Material Design color utilities algorithms, with shared
bindings for Rust, C, C++ and Python.

The Rust crate is the single source of color logic. C ABI, C++ and Python
bindings call into the Rust implementation and do not duplicate algorithms.

## Status

Current milestone: `0.9.0`.

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
```

Useful commands:

```sh
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

`0.9.x` is the stabilization window before `1.0.0`. Public APIs should only
change when the change improves compatibility, safety or naming consistency
before the stable release. See [docs/API_STABILITY.md](docs/API_STABILITY.md).
