#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Modifies a noise with a frequency multiplier.
///
/// This multiplies the point by the provided `frequency` before sampling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frequency<N, F> {
    pub noise: N,
    pub frequency: F,
}

impl<N, F> Noise for Frequency<N, F> {}

impl<const DIM: usize, N, F> Sample<DIM> for Frequency<N, F>
where
    N: Sample<DIM>,
    F: Sample<DIM>,
{
    fn sample_with_seed(&self, mut point: [f32; DIM], seed: i32) -> f32 {
        let frequency = self.frequency.sample_with_seed(point, seed);

        for x in &mut point {
            *x *= frequency;
        }

        self.noise.sample_with_seed(point, seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, N, F> Sample<DIM, Simd<f32, LANES>> for Frequency<N, F>
where
    N: Sample<DIM, Simd<f32, LANES>>,
    F: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    fn sample_with_seed(&self, mut point: Simd<f32, LANES>, seed: i32) -> f32 {
        point *= Simd::splat(self.frequency.sample_with_seed(point, seed));
        self.noise.sample_with_seed(point, seed)
    }
}
