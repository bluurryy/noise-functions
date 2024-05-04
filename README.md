# Noise Functions

[![Crates.io](https://img.shields.io/crates/v/noise-functions.svg)](https://crates.io/crates/noise-functions)
[![Documentation](https://img.shields.io/docsrs/noise-functions)](https://docs.rs/noise-functions)
[![Rust](https://img.shields.io/crates/msrv/noise-functions)](#)
[![License](https://img.shields.io/crates/l/noise_functions)](#license)

Fast and lightweight noise algorithm implementations.

Check out the [live demo](https://bluurryy.github.io/noise-functions-demo/)!

The implementation of these noise functions are from FastNoiseLite ([github](https://github.com/Auburn/FastNoiseLite)/[crate](https://docs.rs/fastnoise-lite/latest/fastnoise_lite/)).

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

## Alternatives

### Why not [`noise`](https://lib.rs/crates/noise)?
With `noise`, constructing a noise struct like `Perlin` creates a permutation table at runtime. So to use the noise efficiently, you need to keep that instance of `Perlin` around.

With `noise-functions`, `Perlin` does not carry any state. So there is no overhead to calling a function like this in a tight loop:
```rust
fn my_noise(point: Vec2) -> f32 {
    Perlin.fbm(3, 0.5, 2.0).seed(42).frequency(3.0).sample2(point)
}
```

**Pros:** `noise` has more noise functions, more dimensions for noise functions and more noise combinators.

**Difference:** `noise` uses `f64` instead of `f32`.

### Why not [`fastnoise-lite`](https://lib.rs/crates/fastnoise-lite)?
`fastnoise-lite` provides its noise generation via a big struct that you are to mutate to get the noise you want. If you already know what noise you want this api is inconvenient and inefficient. There is the [`noise-functions-config`](https://lib.rs/crates/noise-functions-config) crate that provides a similar api if you need it.

**Pros:** `fastnoise-lite` provides more cellular noise variations and has domain warping built into the config struct.