#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample, SampleWithSeed};

use super::fast_abs;

/// Modifies a noise to create a peak at value 0.
///
/// Equal to `abs(x) * 2 - 1`.
///
/// **Note:** This modifier assumes the base noise to return values in the [-1, 1] range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ridged<Noise> {
    pub noise: Noise,
}

impl<N> Noise for Ridged<N> {}

impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample(&self, point: [f32; DIM]) -> f32 {
        fast_abs(self.noise.sample(point)) * -2.0 + 1.0
    }
}

impl<Noise, const DIM: usize> SampleWithSeed<DIM, [f32; DIM]> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, [f32; DIM]>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        fast_abs(self.noise.sample_with_seed(point, seed)) * -2.0 + 1.0
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample(&self, point: Simd<f32, LANES>) -> f32 {
        fast_abs(self.noise.sample(point)) * -2.0 + 1.0
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for Ridged<Noise>
where
    Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        fast_abs(self.noise.sample_with_seed(point, seed)) * -2.0 + 1.0
    }
}
