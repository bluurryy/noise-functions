use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct PingPongWeighted<Noise> {
    pub base: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
    pub strength: f32,
    pub weighted_strength: f32,
}

impl<Noise> PingPongWeighted<Noise> {
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { base: self, seed }
    }

    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for PingPongWeighted<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let &PingPongWeighted {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            weighted_strength,
            ..
        } = self;

        let mut seed = 0;
        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = ping_pong((Seeded { base, seed }.sample(pos) + 1.0) * strength);
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= lerp(1.0, noise, weighted_strength);

            for x in &mut pos {
                *x *= lacunarity;
            }

            amp *= gain;
        }

        sum
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<PingPongWeighted<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let &Seeded {
            base:
                PingPongWeighted {
                    ref base,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                    weighted_strength,
                    ..
                },
            mut seed,
        } = self;

        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = ping_pong((Seeded { base, seed }.sample(pos) + 1.0) * strength);
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= lerp(1.0, noise, weighted_strength);

            for x in &mut pos {
                *x *= lacunarity;
            }

            amp *= gain;
        }

        sum
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for PingPongWeighted<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        let &PingPongWeighted {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            strength,
            weighted_strength,
            ..
        } = self;

        let mut seed = 0;
        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = ping_pong((Seeded { base, seed }.sample(pos) + 1.0) * strength);
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= lerp(1.0, noise, weighted_strength);

            pos *= splat(lacunarity);
            amp *= gain;
        }

        sum
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<PingPongWeighted<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            base:
                PingPongWeighted {
                    ref base,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding,
                    strength,
                    weighted_strength,
                    ..
                },
            mut seed,
        } = self;

        let mut sum = 0.0;
        let mut amp = fractal_bounding;

        for _ in 0..octaves {
            let noise = ping_pong((Seeded { base, seed }.sample(pos) + 1.0) * strength);
            seed = seed.wrapping_add(1);
            sum += (noise - 0.5) * 2.0 * amp;
            amp *= lerp(1.0, noise, weighted_strength);

            pos *= splat(lacunarity);
            amp *= gain;
        }

        sum
    }
}

#[inline(always)]
fn ping_pong(t: f32) -> f32 {
    let t = t - (t * 0.5).trunc() * 2.0;
    if t < 1.0 {
        t
    } else {
        2.0 - t
    }
}
