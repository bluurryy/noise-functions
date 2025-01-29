#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{math::pow, Noise, Sample};

/// Raises the output value to a power.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pow<A, B> {
    pub lhs: A,
    pub rhs: B,
}

impl<A, B> Noise for Pow<A, B> {}

impl<const DIM: usize, A, B> Sample<DIM> for Pow<A, B>
where
    A: Sample<DIM>,
    B: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        pow(self.lhs.sample_with_seed(point, seed), self.rhs.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, A, B> Sample<DIM, Simd<f32, LANES>> for Pow<A, B>
where
    A: Sample<DIM, Simd<f32, LANES>>,
    B: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        pow(self.lhs.sample_with_seed(point, seed), self.rhs.sample_with_seed(point, seed))
    }
}
