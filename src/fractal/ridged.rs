use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Ridged<Noise> {
    pub base: Noise,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    pub fractal_bounding: f32,
}

impl<Noise> Ridged<Noise> {
    #[inline(always)]
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { base: self, seed }
    }

    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }

    #[inline(always)]
    pub const fn weighted(self, strength: f32) -> Weighted<Self> {
        Weighted { fractal: self, strength }
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Ridged<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, pos: [f32; DIM]) -> f32 {
        let &Ridged {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            ..
        } = self;

        ridged(base, octaves, gain, lacunarity, fractal_bounding, 0, pos)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<Ridged<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, pos: [f32; DIM]) -> f32 {
        let &Seeded {
            base: Ridged {
                ref base,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
                ..
            },
            seed,
        } = self;

        ridged(base, octaves, gain, lacunarity, fractal_bounding, seed, pos)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Weighted<Ridged<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, pos: [f32; DIM]) -> f32 {
        let &Weighted {
            fractal: Ridged {
                ref base,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_ridged(base, octaves, gain, lacunarity, fractal_bounding, weighted_strength, 0, pos)
    }
}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Seeded<Weighted<Ridged<Noise>>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, pos: [f32; DIM]) -> f32 {
        let &Seeded {
            base:
                Weighted {
                    fractal:
                        Ridged {
                            ref base,
                            octaves,
                            gain,
                            lacunarity,
                            fractal_bounding,
                        },
                    strength: weighted_strength,
                },
            seed,
        } = self;

        weighted_ridged(base, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, pos)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Ridged<Noise>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, pos: Simd<f32, LANES>) -> f32 {
        let &Ridged {
            ref base,
            octaves,
            gain,
            lacunarity,
            fractal_bounding,
            ..
        } = self;

        ridged_a(base, octaves, gain, lacunarity, fractal_bounding, 0, pos)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<Ridged<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, pos: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            base: Ridged {
                ref base,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
                ..
            },
            seed,
        } = self;

        ridged_a(base, octaves, gain, lacunarity, fractal_bounding, seed, pos)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Weighted<Ridged<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, pos: Simd<f32, LANES>) -> f32 {
        let &Weighted {
            fractal: Ridged {
                ref base,
                octaves,
                gain,
                lacunarity,
                fractal_bounding,
            },
            strength: weighted_strength,
        } = self;

        weighted_ridged_a(base, octaves, gain, lacunarity, fractal_bounding, weighted_strength, 0, pos)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Seeded<Weighted<Ridged<Noise>>>
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, pos: Simd<f32, LANES>) -> f32 {
        let &Seeded {
            base:
                Weighted {
                    fractal:
                        Ridged {
                            ref base,
                            octaves,
                            gain,
                            lacunarity,
                            fractal_bounding,
                        },
                    strength: weighted_strength,
                },
            seed,
        } = self;

        weighted_ridged_a(base, octaves, gain, lacunarity, fractal_bounding, weighted_strength, seed, pos)
    }
}

#[inline(always)]
fn ridged<Noise, const DIM: usize>(base: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut pos: [f32; DIM]) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = fast_abs(Seeded { base, seed }.sample(pos));
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;

        for x in &mut pos {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn ridged_a<Noise, const DIM: usize, const LANES: usize>(base: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, mut seed: i32, mut pos: Simd<f32, LANES>) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = Seeded { base, seed }.sample(pos);
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;

        pos *= splat(lacunarity);
        amp *= gain;
    }

    sum
}

#[inline(always)]
fn weighted_ridged<Noise, const DIM: usize>(base: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, weighted_strength: f32, mut seed: i32, mut pos: [f32; DIM]) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, [f32; DIM]>,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = fast_abs(Seeded { base, seed }.sample(pos));
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;
        amp *= lerp(1.0, 1.0 - noise, weighted_strength);

        for x in &mut pos {
            *x *= lacunarity;
        }

        amp *= gain;
    }

    sum
}

#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn weighted_ridged_a<Noise, const DIM: usize, const LANES: usize>(base: &Noise, octaves: u32, gain: f32, lacunarity: f32, fractal_bounding: f32, weighted_strength: f32, mut seed: i32, mut pos: Simd<f32, LANES>) -> f32
where
    for<'a> Seeded<&'a Noise>: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut sum = 0.0;
    let mut amp = fractal_bounding;

    for _ in 0..octaves {
        let noise = fast_abs(Seeded { base, seed }.sample(pos));
        seed = seed.wrapping_add(1);
        sum += (noise * -2.0 + 1.0) * amp;
        amp *= lerp(1.0, 1.0 - noise, weighted_strength);

        pos *= splat(lacunarity);
        amp *= gain;
    }

    sum
}
