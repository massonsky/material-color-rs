//! Pure Rust Material Design color utilities.
//!
//! This crate contains the shared implementation used by the Rust API, C ABI,
//! C++ wrapper and Python extension. It ports the Material color utilities
//! algorithms without a runtime dependency on the original C++ library.

pub mod argb;
pub mod blend;
pub mod cam;
pub mod contrast;
pub mod dislike;
pub mod dynamic;
pub mod palettes;
pub mod quantize;
pub mod scheme;
pub mod score;
pub mod temperature;
pub mod utils;

pub use argb::Argb;
pub use dynamic::{DynamicColorRole, DynamicScheme, Variant};
pub use temperature::TemperatureCache;
