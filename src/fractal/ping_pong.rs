use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct PingPong<Noise> {
    pub base: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
    pub strength: f32,
}

impl<Noise> PingPong<Noise> {
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { base: self, seed }
    }

    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for PingPong<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let &PingPong {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            ..
        } = self;

        let mut seed = 0;
        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = (Seeded { base, seed }.sample(pos) + 1.0) * strength;
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= noise;

            for x in &mut pos {
                *x *= lacunarity;
            }

            amp *= gain;
        }

        sum
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<PingPong<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let &Seeded {
            base:
                PingPong {
                    ref base,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                    ..
                },
            mut seed,
        } = self;

        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = (Seeded { base, seed }.sample(pos) + 1.0) * strength;
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= noise;

            for x in &mut pos {
                *x *= lacunarity;
            }

            amp *= gain;
        }

        sum
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for PingPong<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        let &PingPong {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            ..
        } = self;

        let mut seed = 0;
        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = (Seeded { base, seed }.sample(pos) + 1.0) * strength;
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= noise;

            pos *= splat(lacunarity);
            amp *= gain;
        }

        sum
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<PingPong<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            base:
                PingPong {
                    ref base,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                    ..
                },
            mut seed,
        } = self;

        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = (Seeded { base, seed }.sample(pos) + 1.0) * strength;
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= noise;

            pos *= splat(lacunarity);
            amp *= gain;
        }

        sum
    }
}
