#[cfg(feature = "nightly-simd")]
use core::simd::Simd;

use crate::{math::cos, Noise, Sample};

/// Computes the cosine of the output value (in radians).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cos<A> {
    pub noise: A,
}

impl<N> Noise for Cos<N> {}

impl<const DIM: usize, N> Sample<DIM> for Cos<N>
where
    N: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        cos(self.noise.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, N> Sample<DIM, Simd<f32, LANES>> for Cos<N>
where
    N: Sample<DIM, Simd<f32, LANES>>,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        cos(self.noise.sample_with_seed(point, seed))
    }
}
