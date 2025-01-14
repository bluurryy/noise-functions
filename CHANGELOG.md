# Changelog

## Unreleased
- **breaking:** `CellValue`, `CellDistance` and `CellDistanceSq` now have the `Fast*` prefix. They also have a `jitter` field. There is no more `Jitter` wrapper struct.
- **breaking:** sampling with a seed is now expressed by the trait `SampleWithSeed` instead of `Seeded<T>`/`Seeded<&T>`; this brings with it better error messages, less boilerplate and extensibility
- **added:** `Sample4` and `Sample4a` helper trait
- **added:** `Perlin`, `Value`, `CellValue`, `CellDistance`, `CellDistanceSq` now support 4D sampling
- **added:** `tileable` method and `Tileable` to create a 2D tileable noise from a 4D noise

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
