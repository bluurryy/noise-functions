#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{math::lerp, Noise, Sample};

/// Linearly interpolates between `a` and `b`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lerp<A, B, T> {
    pub a: A,
    pub b: B,
    pub t: T,
}

impl<A, B, T> Noise for Lerp<A, B, T> {}

impl<const DIM: usize, A, B, T> Sample<DIM> for Lerp<A, B, T>
where
    A: Sample<DIM>,
    B: Sample<DIM>,
    T: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        lerp(self.a.sample_with_seed(point, seed), self.b.sample_with_seed(point, seed), self.t.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, A, B, T> Sample<DIM, Simd<f32, LANES>> for Lerp<A, B, T>
where
    A: Sample<DIM, Simd<f32, LANES>>,
    B: Sample<DIM, Simd<f32, LANES>>,
    T: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        lerp(self.a.sample_with_seed(point, seed), self.b.sample_with_seed(point, seed), self.t.sample_with_seed(point, seed))
    }
}
