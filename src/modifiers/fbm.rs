#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{
    math::{fast_min, lerp},
    Noise, Sample, SampleWithSeed,
};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

use super::{fractal_bounding, Weighted};

/// Fractal Brownian motion (fBm) noise.
///
/// `fBm` noise is created by calling the base noise `octaves` amount of times with increasing frequency and decreasing amplitude.
///
/// **Note:** This modifier assumes the base noise returns values in the [-1, 1] range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fbm<Noise> {
    pub noise: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
}

impl<N> Noise for Fbm<N> {}

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
    pub const fn weighted(self, strength: f32) -> Weighted<Self> {
        Weighted { fractal: self, strength }
    }
}

impl<Noise, const DIM: usize> Sample<DIM> for Fbm<Noise>
where
    Noise: SampleWithSeed<DIM>,
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

impl<Noise, const DIM: usize> SampleWithSeed<DIM> for Fbm<Noise>
where
    Noise: SampleWithSeed<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        let &Fbm {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
        } = self;
        fbm(noise, octaves, gain, lacunarity, fractal_bounding, seed, point)
    }
}

impl<Noise, const DIM: usize> Sample<DIM> for Weighted<Fbm<Noise>>
where
    Noise: SampleWithSeed<DIM>,
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

impl<Noise, const DIM: usize> SampleWithSeed<DIM> for Weighted<Fbm<Noise>>
where
    Noise: SampleWithSeed<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
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
        weighted_fbm(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Fbm<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
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
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for Fbm<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        let &Fbm {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
        } = self;
        fbm_a(noise, octaves, gain, lacunarity, fractal_bounding, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Weighted<Fbm<Noise>>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
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
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for Weighted<Fbm<Noise>>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
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

        weighted_fbm_a(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, point)
    }
}

#[inline(always)]
fn fbm<Noise, const DIM: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut point: [f32; DIM]) -> f32
where
    Noise: SampleWithSeed<DIM>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = noise.sample_with_seed(point, seed);
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
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = noise.sample_with_seed(point, seed);
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
    Noise: SampleWithSeed<DIM>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = noise.sample_with_seed(point, seed);
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
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = noise.sample_with_seed(point, seed);
        seed = seed.wrapping_add(1);
        sum += noise * amp;
        amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}
