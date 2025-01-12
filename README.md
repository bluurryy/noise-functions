# Noise Functions

[![Crates.io](https://img.shields.io/crates/v/noise-functions.svg)](https://crates.io/crates/noise-functions)
[![Documentation](https://img.shields.io/docsrs/noise-functions)](https://docs.rs/noise-functions)
[![Rust](https://img.shields.io/crates/msrv/noise-functions)](#)
[![License](https://img.shields.io/crates/l/noise_functions)](#license)

Fast and lightweight noise functions.

Check out the [live demo][demo]!

# Example Images
Click on the images to view the code that created them.

### Basic
[![](/example-images/cell_distance_sq.jpg "Cell Distance Squared")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L49)
[![](/example-images/cell_distance.jpg "Cell Distance")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L50)
[![](/example-images/cell_value.jpg "Cell Value")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L51)
[![](/example-images/perlin.jpg "Perlin")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L52)
[![](/example-images/open_simplex_2.jpg "OpenSimplex2")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L53)
[![](/example-images/open_simplex_2s.jpg "OpenSimplex2s")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L54)
[![](/example-images/value.jpg "Value")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L55)
[![](/example-images/value_cubic.jpg "Value Cubic")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L56)

### Fractal
[![](/example-images/fbm.jpg "Fbm (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L58)
[![](/example-images/ridged.jpg "Ridged (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L59)
[![](/example-images/ping_pong.jpg "Ping Pong (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L60)

### Domain warped
[![](/example-images/warped.jpg "Domain Warped (OpenSimplex2s)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L62)
[![](/example-images/warped_fbm.jpg "Domain Warped Fbm (OpenSimplex2s)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L72)

### Tileable
[![](/example-images/tileable_perlin.jpg "Tileable (Perlin)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L83)
[![](/example-images/tileable_value.jpg "Tileable (Value)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L85)
[![](/example-images/tileable_cell_value.jpg "Tileable (CellValue)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L87)
[![](/example-images/tileable_cell_distance_sq.jpg "Tileable (CellDistanceSq)")](https://github.com/bluurryy/noise-functions/blob/aee438330a530cfab84af088945b7c41c7df1c88/generate-example-images/src/main.rs#L89)

# Motivation
Noise libraries like [`noise`](https://docs.rs/noise) or [`libnoise`](https://docs.rs/libnoise) create a permutation table at runtime for each instance of `Perlin` and the like. This library uses static permutation tables / hashing instead. As such, there is no need to store and reuse noise structs for the sake of efficiency. For example:
```rust
fn my_noise(point: Vec2) -> f32 {
    Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0).sample2(point)
}
```
In this example, the whole `Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0)` expression is evaluated at compile time.

> [!NOTE]
> This library uses `f32` instead of `f64`.

## Why not [`fastnoise-lite`](https://docs.rs/fastnoise-lite)?
`fastnoise-lite` provides its noise generation via a big struct that you are to mutate to get the noise you want. If you already know what noise you want or you want to compose multiple noises in a custom way then this design is less efficient and less convenient. There is the [`noise-functions-config`](https://docs.rs/noise-functions-config) crate that provides a similar configurable struct (the [demo] is powered by it). It opts to return a trait object like `Box<dyn Sample<2>>` instead of branching on each sample call.

> [!NOTE]
> The implementation of the current noise functions are from [FastNoiseLite](https://github.com/Auburn/FastNoiseLite). The simd versions were created by me.

[demo]: https://bluurryy.github.io/noise-functions-demo/