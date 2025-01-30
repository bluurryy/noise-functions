#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{math::round, Noise, Sample};

/// Computes the nearest integer to the output value.
/// If a value is half-way between two integers, round away from 0.0.
pub struct Round<A> {
    pub noise: A,
}

impl<N> Noise for Round<N> {}

impl<const DIM: usize, N> Sample<DIM> for Round<N>
where
    N: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        round(self.noise.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, N> Sample<DIM, Simd<f32, LANES>> for Round<N>
where
    N: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        round(self.noise.sample_with_seed(point, seed))
    }
}
