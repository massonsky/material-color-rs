# Migration From material-color-utilities C++

This project replaces runtime use of the original C++ implementation with a
pure Rust core and thin bindings.

Original upstream project:

- https://github.com/material-foundation/material-color-utilities

## C++ Projects

### Before

Projects using `material-color-utilities/cpp` directly typically linked C++
algorithm sources into the application.

### After

Depend on the Rust-backed C++ wrapper:

```cpp
#include "material_color/material_color.hpp"

uint32_t primary = material_color::DynamicSchemeColor(
    0xff4285f4, MATERIAL_COLOR_VARIANT_TONAL_SPOT, false, 0.0,
    MATERIAL_COLOR_ROLE_PRIMARY);
```

The wrapper calls the C ABI and owns returned arrays with RAII types. It does not
contain algorithm implementations.

## C Projects

Use `bindings/c/include/material_color/material_color.h` and link the generated
Rust C ABI library.

```c
uint32_t primary = 0;
MaterialColorStatus status = material_color_dynamic_scheme_color(
    0xff4285f4, MATERIAL_COLOR_VARIANT_TONAL_SPOT, false, 0.0,
    MATERIAL_COLOR_ROLE_PRIMARY, &primary);
```

Arrays returned through the C ABI must be released with the matching
`material_color_*_free` function.

## Rust Projects

Use the `material_color` crate directly:

```rust
use material_color::argb::Argb;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};

let scheme = DynamicScheme::from_argb(Argb::new(0xff42_85f4), Variant::TonalSpot, false, 0.0);
let primary = scheme.color(DynamicColorRole::Primary);
```

## Python Projects

Use the PyO3 extension:

```python
import material_color_py as mc

scheme = mc.DynamicScheme(0xFF4285F4, mc.Variant.TONAL_SPOT, False, 0.0)
primary = scheme.color(mc.DynamicColorRole.PRIMARY)
```

## Compatibility Notes

- ARGB values are represented as opaque `0xAARRGGBB` integers.
- C and Python role/variant constants use the same integer values as the Rust
  enums.
- Floating-point values are expected to match the C++ reference within the
  tolerances documented in tests.
- The Rust core has no runtime dependency on the original C++ library.
