#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Subtracts one output value from the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sub<A, B> {
    pub lhs: A,
    pub rhs: B,
}

impl<A, B> Noise for Sub<A, B> {}

impl<const DIM: usize, A, B> Sample<DIM> for Sub<A, B>
where
    A: Sample<DIM>,
    B: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        self.lhs.sample_with_seed(point, seed) - self.rhs.sample_with_seed(point, seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, A, B> Sample<DIM, Simd<f32, LANES>> for Sub<A, B>
where
    A: Sample<DIM, Simd<f32, LANES>>,
    B: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        self.lhs.sample_with_seed(point, seed) - self.rhs.sample_with_seed(point, seed)
    }
}
