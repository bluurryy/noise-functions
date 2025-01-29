#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

use super::abs;

/// Applies a triangle wave to the output of a noise function.
///
/// This outputs values is in the [-1, 1] range.
///
/// **Note:** This modifier assumes the base noise returns values in the [-1, 1] range.
pub struct Ridged<Noise> {
    pub noise: Noise,
}

impl<N> Noise for Ridged<N> {}

impl<Noise, const DIM: usize> Sample<DIM> for Ridged<Noise>
where
    Noise: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        apply(self.noise.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Ridged<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        apply(self.noise.sample_with_seed(point, seed))
    }
}

#[inline]
fn apply(value: f32) -> f32 {
    abs(value) * -2.0 + 1.0
}
