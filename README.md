# Noise Functions

[![Crates.io](https://img.shields.io/crates/v/noise-functions.svg)](https://crates.io/crates/noise-functions)
[![Documentation](https://img.shields.io/docsrs/noise-functions)](https://docs.rs/noise-functions)
[![Rust](https://img.shields.io/crates/msrv/noise-functions)](#)
[![License](https://img.shields.io/crates/l/noise_functions)](#license)

Fast and lightweight noise functions.

Check out the [live demo][demo]!

### Base noises
[![](/example-images/cell_distance_sq.jpg "Cell Distance Squared")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L26)
[![](/example-images/cell_distance.jpg "Cell Distance")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L27)
[![](/example-images/cell_value.jpg "Cell Value")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L28)
[![](/example-images/perlin.jpg "Perlin")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L29)
[![](/example-images/open_simplex_2.jpg "OpenSimplex2")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L30)
[![](/example-images/open_simplex_2s.jpg "OpenSimplex2s")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L31)
[![](/example-images/value.jpg "Value")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L32)
[![](/example-images/value_cubic.jpg "Value Cubic")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L33)

### Fractal noises
[![](/example-images/fbm.jpg "Fbm (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L35)
[![](/example-images/ridged.jpg "Ridged (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L36)
[![](/example-images/ping_pong.jpg "Ping Pong (OpenSimplex2)")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L37)

### Domain warped noises
[![](/example-images/warped.jpg "Domain Warped (OpenSimplex2s)")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L39)
[![](/example-images/warped_fbm.jpg "Domain Warped Fbm (OpenSimplex2s)")](https://github.com/bluurryy/noise-functions/blob/b550116dbdc717f1f43f5af16471effb527cf66b/generate-example-images/src/main.rs#L49)

# Motivation
Noise libraries like [`noise`](https://docs.rs/noise) or [`libnoise`](https://docs.rs/libnoise) create a permutation table at runtime for each instance of `Perlin` and the like. This library uses static permutation tables instead. That means you can simply create a function like this:
```rust
fn my_noise(point: Vec2) -> f32 {
    Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0).sample2(point)
}
```
The whole `Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0)` expression will be evaluated at compile time so there is no point in carrying around that noise struct or putting it into a `static`.

> [!NOTE]
> This library uses `f32` instead of `f64`.

## Why not [`fastnoise-lite`](https://docs.rs/fastnoise-lite)?
`fastnoise-lite` provides its noise generation via a big struct that you are to mutate to get the noise you want. If you already know what noise you want or you want to compose multiple noises in a custom way then this design is less efficient and less convenient. There is the [`noise-functions-config`](https://docs.rs/noise-functions-config) crate that provides a similar configurable struct (the [demo] is powered by it). It opts to return a trait object like `Box<dyn Sample<2>>` instead of branching on each sample call.

> [!NOTE]
> The implementation of the current noise functions are from [FastNoiseLite](https://github.com/Auburn/FastNoiseLite). The simd versions were created by me.

[demo]: https://bluurryy.github.io/noise-functions-demo/