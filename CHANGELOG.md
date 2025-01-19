# Changelog

## 0.7.0 (2025-01-19)
- **breaking:** sealed the `OpenSimplexNoise` trait
- **breaking:** changed `OpenSimplex2` 2D and 3D implementation and output
- **breaking:** renamed `improve_xy` and `improve_xz` to `improve3_xy` and `improve3_xz`
- **added:** implemented `Default` for `OpenSimplex2(s)`, `Perlin`, `Simplex`, `ValueCubic` and `Value`
- **added:** 2D and 4D OpenSimplex2 `improve_*` methods

## 0.6.0 (2025-01-18)
- **breaking:** fixed open simplex `Improve*` wrappers to apply the improvements on the base unrotated noise instead of the noise with the already improved rotation; this introduces new `#[doc(hidden)]` members on the `OpenSimplexNoise` trait and changes to the bounds required for `Improve*` to implement `Sample`
- **added:** 4D OpenSimplex2s and OpenSimplex2s noise

## 0.5.0 (2025-01-16)
- **breaking:** `Sample*` helper traits now require `Sample` which in turn now requires `Noise`; this is useful for generic code so you don't need to specify those additional bounds if you need them; every type that implements `Sample*` should implement `Sample` and `Noise` anyway
- **added:** missing `Noise` implementations for `Weighted` and `Frequency` modifiers
- **added:** blanket `Noise` impl for every `&N` where `N: Noise`
- **added:** removed `Sized` bound from `Noise`

## 0.4.0 (2025-01-16)
- **breaking:** `CellValue`, `CellDistance` and `CellDistanceSq` now have a `jitter` field. There is no more `Jitter` wrapper struct.
- **breaking:** sampling with a seed is now expressed by the trait `SampleWithSeed` instead of `Seeded<T>`/`Seeded<&T>`; this brings with it better error messages, less boilerplate and extensibility
- **breaking:** moved modifier methods like `seed`, `frequency`, `fbm` and the like to the `Noise` trait
- **breaking:** `ridged` modifier is no longer a fractal; to migrate, replace `.ridged(...)` with `.ridged().fbm(...)`
- **breaking:** `ping_pong` modifier is gone; to migrate, replace `.ping_pong(...)` with `.triangle_wave().fbm(...).weighted(1.0)`
- **added:** `Sample4` and `Sample4a` helper trait
- **added:** `Perlin`, `Value` and `Cell*` noises now support 4D sampling
- **added:** a new 2, 3 and 4 dimensional `Simplex` noise
- **added:** `tileable` modifier to create a 2D tileable noise from a 4D noise
- **added:** `mul_seed` modifier to avoid sampling with the same seed multiple times when composing noises

## 0.3.0 (2024-11-29)
- **breaking:** removed `"nightly-const-fn-float"` feature; those functions are now always const
- **breaking:** increased msrv to `1.82.0`

## 0.2.1 (2024-05-04)
- **fixed:** `no_std`

## 0.2.0 (2024-05-04)
- **breaking:** removed `*Weighted` structs, `Weighted` becomes a modifier struct
- **breaking:** rename `base` to `noise`
- **breaking:** `Fbm`, `Ridged`, `PingPong`, `Weighted` and `fractal_bounding` is no longer in the crate root but in the `fractal` module
- **breaking:** `OpenSimplex2(s)` apply the fallback orientation improvement by default, you can change the orientation improvement with `improve_xy`, `improve_xz` and `ImproveXy`, `ImproveXz`
- **breaking:** renamed module `open_simplex` to `open_simplex_2`
- **breaking:** removed some `Hash` derives from noises
- **added:** modifier methods for `Jitter`

## 0.1.3 (2024-05-03)
- **added:** `Seeded::frequency`
- **added:** `ping_pong(_weighted)` with `PingPong(Weighted)`

## 0.1.2 (2024-04-26)
- **added:** configurable jitter for cellular noises
- **added:** removed unnecessary generic copy bound for generics

## 0.1.1 (2024-04-26)
- **added:** implemented `Debug`, `Clone` and `Copy` for `Frequency`

## 0.1.0 (2024-04-26)
- **breaking:** removed `alloc` feature
- **breaking:** renamed `SampleFn` to `NoiseFn`
- **added:** fractal support for `NoiseFn`

## 0.0.0 (2024-04-25)
