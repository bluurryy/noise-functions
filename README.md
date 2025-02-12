# Noise Functions

[![Crates.io](https://img.shields.io/crates/v/noise-functions.svg)](https://crates.io/crates/noise-functions)
[![Documentation](https://img.shields.io/docsrs/noise-functions)](https://docs.rs/noise-functions)
[![Rust](https://img.shields.io/crates/msrv/noise-functions)](#)
[![License](https://img.shields.io/crates/l/noise_functions)](#license)

A collection of fast and lightweight noise functions.

Check out the [live demo][demo] and [node editor][playground] (experimental)!

# Example Images
Click on the images to view the code that created them.

### Basic
[![](/example-images/cell_distance_sq.jpg "Cell Distance Squared")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L36)
[![](/example-images/cell_distance.jpg "Cell Distance")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L37)
[![](/example-images/cell_value.jpg "Cell Value")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L39)
[![](/example-images/perlin.jpg "Perlin")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L40)
[![](/example-images/open_simplex_2.jpg "OpenSimplex2")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L42)
[![](/example-images/open_simplex_2s.jpg "OpenSimplex2s")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L43)
[![](/example-images/value.jpg "Value")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L44)
[![](/example-images/value_cubic.jpg "Value Cubic")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L45)

### Fractal
[![](/example-images/fbm.jpg "Fbm (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L47)
[![](/example-images/ridged.jpg "Ridged (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L48)
[![](/example-images/ping_pong.jpg "Ping Pong (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L49)

### Domain warped
[![](/example-images/warped.jpg "Domain Warped (OpenSimplex2s)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L51)
[![](/example-images/warped_fbm.jpg "Domain Warped Fbm (OpenSimplex2s)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L61)

### Tileable
[![](/example-images/tileable_perlin.jpg "Tileable (Perlin)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L73)
[![](/example-images/tileable_value.jpg "Tileable (Value)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L75)
[![](/example-images/tileable_cell_value.jpg "Tileable (CellValue)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L77)
[![](/example-images/tileable_cell_distance_sq.jpg "Tileable (CellDistanceSq)")](https://github.com/bluurryy/noise-functions/blob/014d4159f3ddfa5d9d6f03bffd17eecf40de8388/generate-example-images/src/main.rs#L79)

# Motivation
Noise libraries like [`noise`](https://docs.rs/noise) or [`libnoise`](https://docs.rs/libnoise) create a permutation table at runtime for each instance of `Perlin` and the like. This library uses static permutation tables / hashing instead. As such, there is no need to store and reuse noise structs for the sake of efficiency. There is no downside to writing code like this:
```rust
fn my_noise(point: Vec2) -> f32 {
    Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0).sample2(point)
}
```

> [!NOTE]
> This library uses `f32` instead of `f64`.

## Why not [`fastnoise-lite`](https://docs.rs/fastnoise-lite)?
`fastnoise-lite` provides its noise generation via a big struct that you are to mutate to get the noise you want. If you already know what noise you want or you want to compose multiple noises in a custom way then this design is less efficient and less convenient. There is the [`noise-functions-config`][config] crate that provides a similar configurable struct (the [demo] is powered by it). It opts to return a trait object like `Box<dyn Sample<2>>` instead of branching on each sample call.

[config]: https://docs.rs/noise-functions-config
[demo]: https://bluurryy.github.io/noise-functions-demo/
[playground]: https://bluurryy.github.io/noise-functions-playground/

## License

Licensed under either of:

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Your contributions

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, 
shall be dual licensed as above,
without any additional terms or conditions.
