#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{
    math::{lerp, trunc},
    modifiers::{Frequency, Seeded, Weighted},
    Sample, SampleWithSeed,
};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

use super::fractal_bounding;

/// Generates fractal noise and applies a triangle wave to the output of a base noise function.
///
/// **Note:** This modifier assumes the base noise to return values in the [-1, 1] range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PingPong<Noise> {
    pub noise: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
    pub strength: f32,
}

impl<Noise> PingPong<Noise> {
    #[inline(always)]
    pub const fn new(noise: Noise, octaves: u32, gain: f32, lacunarity: f32, strength: f32) -> Self {
        Self {
            noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding: fractal_bounding(octaves, gain),
            strength,
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

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for PingPong<Noise>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &PingPong {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            ..
        } = self;

        ping_pong(noise, octaves, gain, lacunarity, fractal_bounding, strength, 0, point)
    }
}

impl<Noise, const DIM: usize> SampleWithSeed<DIM, [f32; DIM]> for PingPong<Noise>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        let &PingPong {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            ..
        } = self;

        ping_pong(noise, octaves, gain, lacunarity, fractal_bounding, strength, seed, point)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Weighted<PingPong<Noise>>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Weighted {
            fractal:
                PingPong {
                    ref noise,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                },
            strength: weighted_strength,
        } = self;

        weighted_ping_pong(noise, octaves, gain, lacunarity, fractal_bounding, strength, weighted_strength, 0, point)
    }
}

impl<Noise, const DIM: usize> SampleWithSeed<DIM, [f32; DIM]> for Weighted<PingPong<Noise>>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        let &Weighted {
            fractal:
                PingPong {
                    ref noise,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                },
            strength: weighted_strength,
        } = self;

        weighted_ping_pong(noise, octaves, gain, lacunarity, fractal_bounding, strength, weighted_strength, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for PingPong<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &PingPong {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            ..
        } = self;

        ping_pong_a(noise, octaves, gain, lacunarity, fractal_bounding, strength, 0, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for PingPong<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        let &PingPong {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
        } = self;

        ping_pong_a(noise, octaves, gain, lacunarity, fractal_bounding, strength, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Weighted<PingPong<Noise>>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &Weighted {
            fractal:
                PingPong {
                    ref noise,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                },
            strength: weighted_strength,
        } = self;

        weighted_ping_pong_a(noise, octaves, gain, lacunarity, fractal_bounding, strength, weighted_strength, 0, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for Weighted<PingPong<Noise>>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        let &Weighted {
            fractal:
                PingPong {
                    ref noise,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                },
            strength: weighted_strength,
        } = self;

        weighted_ping_pong_a(noise, octaves, gain, lacunarity, fractal_bounding, strength, weighted_strength, seed, point)
    }
}

#[inline(always)]
fn do_ping_pong(t: f32) -> f32 {
    let t = t - trunc(t * 0.5) * 2.0;
    if t < 1.0 {
        t
    } else {
        2.0 - t
    }
}

#[inline(always)]
fn ping_pong<Noise, const DIM: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, strength: f32, mut seed: i32, mut point: [f32; DIM]) -> f32
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = do_ping_pong((noise.sample_with_seed(point, seed) + 1.0) * strength);
        seed = seed.wrapping_add(1);
        sum += (noise - 0.5) * 2.0 * amp;
        amp *= noise;

        for x in &mut point {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn ping_pong_a<Noise, const DIM: usize, const LANES: usize>(
    noise: &Noise,
    octaves: u32,
    gain: f32,
    lacunarity: f32,
    fractal_bounding: f32,
    strength: f32,
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
        let noise = do_ping_pong((noise.sample_with_seed(point, seed) + 1.0) * strength);
        seed = seed.wrapping_add(1);
        sum += (noise - 0.5) * 2.0 * amp;
        amp *= noise;

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}

#[inline(always)]
fn weighted_ping_pong<Noise, const DIM: usize>(
    noise: &Noise,
    octaves: u32,
    gain: f32,
    lacunarity: f32,
    fractal_bounding: f32,
    strength: f32,
    weighted_strength: f32,
    mut seed: i32,
    mut point: [f32; DIM],
) -> f32
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = do_ping_pong((noise.sample_with_seed(point, seed) + 1.0) * strength);
        seed = seed.wrapping_add(1);
        sum += (noise - 0.5) * 2.0 * amp;
        amp *= lerp(1.0, noise, weighted_strength);

        for x in &mut point {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn weighted_ping_pong_a<Noise, const DIM: usize, const LANES: usize>(
    noise: &Noise,
    octaves: u32,
    gain: f32,
    lacunarity: f32,
    fractal_bounding: f32,
    strength: f32,
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
        let noise = do_ping_pong((noise.sample_with_seed(point, seed) + 1.0) * strength);
        seed = seed.wrapping_add(1);
        sum += (noise - 0.5) * 2.0 * amp;
        amp *= lerp(1.0, noise, weighted_strength);

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}