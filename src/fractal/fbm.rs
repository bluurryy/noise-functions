#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{
    math::{fast_min, lerp},
    Frequency, Sample, Seeded,
};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

use super::{fractal_bounding, Weighted};

/// Fractal Brownian motion (fBm) noise.
///
/// `fBm` noise is created by calling the base noise `octaves` amount of times with increasing frequency and decreasing amplitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fbm<Noise> {
    pub noise: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
}

impl<Noise> Fbm<Noise> {
    #[inline(always)]
    pub const fn new(noise: Noise, octaves: u32, gain: f32, lacunarity: f32) -> Self {
        Self {
            noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding: fractal_bounding(octaves, gain),
        }
    }

    #[inline(always)]
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { noise: self, seed }
    }

    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }

    #[inline(always)]
    pub const fn weighted(self, strength: f32) -> Weighted<Self> {
        Weighted { fractal: self, strength }
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Fbm<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Fbm {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
        } = self;
        fbm(noise, octaves, gain, lacunarity, fractal_bounding, 0, point)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<Fbm<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Seeded {
            noise: Fbm {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            seed,
        } = self;
        fbm(noise, octaves, gain, lacunarity, fractal_bounding, seed, point)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Weighted<Fbm<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Weighted {
            fractal: Fbm {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_fbm(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, 0, point)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<Weighted<Fbm<Noise>>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Seeded {
            noise:
                Weighted {
                    fractal:
                        Fbm {
                            ref noise,
                            octaves,
                            gain,
                            lacunarity,
                            fractal_bounding,
                        },
                    strength: weighted_strength,
                },
            seed,
        } = self;
        weighted_fbm(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Fbm<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &Fbm {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
        } = self;
        fbm_a(noise, octaves, gain, lacunarity, fractal_bounding, 0, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<Fbm<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            noise: Fbm {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            seed,
        } = self;
        fbm_a(noise, octaves, gain, lacunarity, fractal_bounding, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Weighted<Fbm<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &Weighted {
            fractal: Fbm {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_fbm_a(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, 0, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<Weighted<Fbm<Noise>>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            noise:
                Weighted {
                    fractal:
                        Fbm {
                            ref noise,
                            octaves,
                            gain,
                            lacunarity,
                            fractal_bounding,
                        },
                    strength: weighted_strength,
                },
            seed,
        } = self;

        weighted_fbm_a(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, point)
    }
}

#[inline(always)]
fn fbm<Noise, const DIM: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut point: [f32; DIM]) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = Seeded { noise, seed }.sample(point);
        seed = seed.wrapping_add(1);
        sum += noise * amp;

        for x in &mut point {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn fbm_a<Noise, const DIM: usize, const LANES: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut point: Simd<f32, LANES>) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = Seeded { noise, seed }.sample(point);
        seed = seed.wrapping_add(1);
        sum += noise * amp;

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}

#[inline(always)]
fn weighted_fbm<Noise, const DIM: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, weighted_strength: f32, mut seed: i32, mut point: [f32; DIM]) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = Seeded { noise, seed }.sample(point);
        seed = seed.wrapping_add(1);
        sum += noise * amp;
        amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

        for x in &mut point {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn weighted_fbm_a<Noise, const DIM: usize, const LANES: usize>(
    noise: &Noise,
    octaves: u32,
    gain: f32,
    lacunarity: f32,
    fractal_bounding: f32,
    weighted_strength: f32,
    mut seed: i32,
    mut point: Simd<f32, LANES>,
) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = Seeded { noise, seed }.sample(point);
        seed = seed.wrapping_add(1);
        sum += noise * amp;
        amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}
