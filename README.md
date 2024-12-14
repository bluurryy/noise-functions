# Noise Functions

[![Crates.io](https://img.shields.io/crates/v/noise-functions.svg)](https://crates.io/crates/noise-functions)
[![Documentation](https://img.shields.io/docsrs/noise-functions)](https://docs.rs/noise-functions)
[![Rust](https://img.shields.io/crates/msrv/noise-functions)](#)
[![License](https://img.shields.io/crates/l/noise_functions)](#license)

Fast and lightweight noise functions.

Check out the [live demo][demo]!

### Base noises
![](/example-images/cell_distance_sq.jpg "Cell Distance Squared")
![](/example-images/cell_distance.jpg "Cell Distance")
![](/example-images/cell_value.jpg "Cell Value")
![](/example-images/perlin.jpg "Perlin")
![](/example-images/open_simplex_2.jpg "OpenSimplex2")
![](/example-images/open_simplex_2s.jpg "OpenSimplex2s")
![](/example-images/value.jpg "Value")
![](/example-images/value_cubic.jpg "Value Cubic")

### Fractal noises
![](/example-images/fbm.jpg "Fbm (OpenSimplex2)")
![](/example-images/ridged.jpg "Ridged (OpenSimplex2)")
![](/example-images/ping_pong.jpg "Ping Pong (OpenSimplex2)")

### Domain warped noises
![](/example-images/warped.jpg "Domain Warped (OpenSimplex2s)")
![](/example-images/warped_fbm.jpg "Domain Warped Fbm (OpenSimplex2s)")

# Motivation
Noise libraries like [`noise`](https://docs.rs/noise) or [`libnoise`](https://docs.rs/libnoise) create a permutation table at runtime for each instance of `Perlin` and the like. This library uses static permutation tables instead. That means you can simply create a function like this:
```rust
fn my_noise(point: Vec2) -> f32 {
    Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0).sample2(point)
}
```
The whole `Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0)` expression will be evaluated at compile time so there is no point in carrying around that noise struct or putting it into a `static`.

[!NOTE]
This library uses `f32` instead of `f64`.

## Why not [`fastnoise-lite`](https://docs.rs/fastnoise-lite)?
`fastnoise-lite` provides its noise generation via a big struct that you are to mutate to get the noise you want. If you already know what noise you want or you want to compose multiple noises in a custom way then this design is less efficient and less convenient. There is the [`noise-functions-config`](https://docs.rs/noise-functions-config) crate that provides a similar configurable struct (the [demo] is powered by it). It opts to return a trait object like `Box<dyn Sample<2>>` instead of branching on each sample call.

[!NOTE]
The implementation of all current noise functions are from [FastNoiseLite](https://github.com/Auburn/FastNoiseLite).

[demo]: https://bluurryy.github.io/noise-functions-demo/