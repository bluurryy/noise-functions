#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Multiplies the seed by `value`.
pub struct MulSeed<Noise> {
    pub noise: Noise,
    pub value: i32,
}

impl<N> Noise for MulSeed<N> {}

impl<Noise, const DIM: usize> Sample<DIM> for MulSeed<Noise>
where
    Noise: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        self.noise.sample_with_seed(point, seed * self.value)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for MulSeed<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        self.noise.sample_with_seed(point, seed * self.value)
    }
}
