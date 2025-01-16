#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{
    math::lerp,
    modifiers::{Frequency, Seeded},
    Sample, SampleWithSeed,
};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

use super::{fast_abs, fractal_bounding, Weighted};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ridged<Noise> {
    pub noise: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
}

impl<Noise> Ridged<Noise> {
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

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Ridged {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            ..
        } = self;

        ridged(noise, octaves, gain, lacunarity, fractal_bounding, 0, point)
    }
}

impl<Noise, const DIM: usize> SampleWithSeed<DIM, [f32; DIM]> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        let &Ridged {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            ..
        } = self;

        ridged(noise, octaves, gain, lacunarity, fractal_bounding, seed, point)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Weighted<Ridged<Noise>>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        let &Weighted {
            fractal: Ridged {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_ridged(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, 0, point)
    }
}

impl<Noise, const DIM: usize> SampleWithSeed<DIM, [f32; DIM]> for Weighted<Ridged<Noise>>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        let &Weighted {
            fractal: Ridged {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_ridged(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        self.sample_with_seed(point, 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        let &Ridged {
            ref noise,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            ..
        } = self;

        ridged_a(noise, octaves, gain, lacunarity, fractal_bounding, seed, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Weighted<Ridged<Noise>>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        let &Weighted {
            fractal: Ridged {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_ridged_a(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, 0, point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for Weighted<Ridged<Noise>>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        let &Weighted {
            fractal: Ridged {
                ref noise,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_ridged_a(noise, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, point)
    }
}

#[inline(always)]
fn ridged<Noise, const DIM: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut point: [f32; DIM]) -> f32
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = fast_abs(noise.sample_with_seed(point, seed));
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;

        for x in &mut point {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn ridged_a<Noise, const DIM: usize, const LANES: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut point: Simd<f32, LANES>) -> f32
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = noise.sample_with_seed(point, seed);
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}

#[inline(always)]
fn weighted_ridged<Noise, const DIM: usize>(noise: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, weighted_strength: f32, mut seed: i32, mut point: [f32; DIM]) -> f32
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = fast_abs(noise.sample_with_seed(point, seed));
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;
        amp *= lerp(1.0, 1.0 - noise, weighted_strength);

        for x in &mut point {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn weighted_ridged_a<Noise, const DIM: usize, const LANES: usize>(
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
        let noise = fast_abs(noise.sample_with_seed(point, seed));
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;
        amp *= lerp(1.0, 1.0 - noise, weighted_strength);

        point *= splat(lacunarity);
        amp *= gain;
    }

    sum
}
