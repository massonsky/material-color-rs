# Changelog

## 1.0.0

Stable release of the pure Rust Material Design color utilities port.

Highlights:

- Rust core is the single algorithm implementation.
- C ABI, C++ wrapper and Python extension are backed by Rust core.
- ARGB utilities, CAM16, HCT, palettes, contrast, blend, dynamic schemes,
  quantization, scoring and temperature utilities are implemented.
- All 54 Material dynamic color roles are exposed.
- Stable version surface is available in Rust, C, C++ and Python.
- Cargo, Bazel, C++ and Python smoke checks are covered by CI.

## 0.9.0

- Added compatibility snapshots, documentation, CI workflow and benchmark
  example.
- Hardened C ABI version reporting against Cargo metadata.

## 0.8.0

- Added PyO3 Python extension and Python import workflow test.

## 0.7.0

- Added C ABI, C++ wrapper and Bazel C++ integration.

## 0.6.0

- Added quantization, scoring and temperature utilities.

## 0.5.0

- Added Material dynamic schemes and dynamic color roles.

## 0.4.0

- Added palettes, contrast, blend and dislike correction.

## 0.3.0

- Added CAM16 and HCT.

## 0.2.0

- Added core color primitives and utility conversions.

## 0.1.0

- Created workspace and Bazel/Cargo scaffolding.
