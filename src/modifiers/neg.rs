#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Computes the absolute value of the output value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Neg<A> {
    pub noise: A,
}

impl<N> Noise for Neg<N> {}

impl<const DIM: usize, N> Sample<DIM> for Neg<N>
where
    N: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        -self.noise.sample_with_seed(point, seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, N> Sample<DIM, Simd<f32, LANES>> for Neg<N>
where
    N: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        -self.noise.sample_with_seed(point, seed)
    }
}
