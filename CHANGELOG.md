# Changelog

## Unreleased
- **breaking:** removed `*Weighted` structs, `Weighted` becomes a modifier struct
- **breaking:** rename `base` to `noise`
- **breaking:** `Fbm`, `Ridged`, `PingPong`, `Weighted` and `fractal_bounding` is no longer in the crate root but in the `fractal` module

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
