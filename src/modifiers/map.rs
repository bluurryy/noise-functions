#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Maps the output value.
pub struct Map<N, F> {
    pub noise: N,
    pub f: F,
}

impl<N, F> Noise for Map<N, F> {}

impl<N, F, const DIM: usize> Sample<DIM> for Map<N, F>
where
    N: Sample<DIM>,
    F: Fn(f32) -> f32,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        (self.f)(self.noise.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<N, F, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Map<N, F>
where
    N: Sample<DIM, Simd<f32, LANES>>,
    F: Fn(f32) -> f32,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        (self.f)(self.noise.sample_with_seed(point, seed))
    }
}
