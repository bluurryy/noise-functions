#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{math::sqrt, Noise, Sample};

/// Returns the square root of a number.
///
/// Returns NaN if `self` is a negative number other than `-0.0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sqrt<A> {
    pub noise: A,
}

impl<N> Noise for Sqrt<N> {}

impl<const DIM: usize, N> Sample<DIM> for Sqrt<N>
where
    N: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        sqrt(self.noise.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, N> Sample<DIM, Simd<f32, LANES>> for Sqrt<N>
where
    N: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        sqrt(self.noise.sample_with_seed(point, seed))
    }
}
