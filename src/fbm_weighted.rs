use super::*;
use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct FbmWeighted<Noise> {
    pub base: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
    pub weighted_strength: f32,
}

impl<Noise> FbmWeighted<Noise> {
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { base: self, seed }
    }

    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for FbmWeighted<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let &FbmWeighted {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            weighted_strength,
            ..
        } = self;

        let mut seed = 0;
        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = Seeded { base, seed }.sample(pos);
            seed = seed.wrapping_add(1);
            sum += noise * amp;
            amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

            for x in &mut pos {
                *x *= lacunarity;
            }

            amp *= gain;
        }

        sum
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<FbmWeighted<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let &Seeded {
            base:
                FbmWeighted {
                    ref base,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    weighted_strength,
                    ..
                },
            mut seed,
        } = self;

        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = Seeded { base, seed }.sample(pos);
            seed = seed.wrapping_add(1);
            sum += noise * amp;
            amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

            for x in &mut pos {
                *x *= lacunarity;
            }

            amp *= gain;
        }

        sum
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for FbmWeighted<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        let &FbmWeighted {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            weighted_strength,
            ..
        } = self;

        let mut seed = 0;
        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = Seeded { base, seed }.sample(pos);
            seed = seed.wrapping_add(1);
            sum += noise * amp;
            amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

            pos *= splat(lacunarity);
            amp *= gain;
        }

        sum
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<FbmWeighted<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            base:
                FbmWeighted {
                    ref base,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    weighted_strength,
                    ..
                },
            mut seed,
        } = self;

        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = Seeded { base, seed }.sample(pos);
            seed = seed.wrapping_add(1);
            sum += noise * amp;
            amp *= lerp(1.0, fast_min(noise + 1.0, 2.0) * 0.5, weighted_strength);

            pos *= splat(lacunarity);
            amp *= gain;
        }

        sum
    }
}
