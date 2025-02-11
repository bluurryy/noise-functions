#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{math::ceil, Noise, Sample};

/// Computes the smallest integer greater than or equal to self.
pub struct Ceil<A> {
    pub noise: A,
}

impl<N> Noise for Ceil<N> {}

impl<const DIM: usize, N> Sample<DIM> for Ceil<N>
where
    N: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        ceil(self.noise.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, N> Sample<DIM, Simd<f32, LANES>> for Ceil<N>
where
    N: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        ceil(self.noise.sample_with_seed(point, seed))
    }
}
