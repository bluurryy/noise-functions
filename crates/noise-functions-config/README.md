# noise-functions-config

[![Crates.io](https://img.shields.io/crates/v/noise-functions-config.svg)](https://crates.io/crates/noise-functions-config)
[![Documentation](https://img.shields.io/docsrs/noise-functions-config)](https://docs.rs/noise-functions-config)
[![License](https://img.shields.io/crates/l/noise_functions_config)](#license)

Configurable noise generator struct for the [`noise-functions`](https://docs.rs/noise-functions) crate.

<!-- crate documentation feature start -->
## Feature flags
<!-- feature documentation start -->
- **`std`** *(enabled by default)* — Uses floating point functions from the standard library.
- **`libm`** — Uses `libm` for floating point functions. Required for `no_std`.
- **`nightly-simd`** — Adds support for sampling with simd types.
  Some of the noise algorithms have optimized implementations for simd that can be faster than the scalar versions.
  Currently those are the 2d and 3d implementations of `Perlin`, `Cell*` and `Value*` noises.
<!-- feature documentation end -->

<!-- crate documentation feature end -->