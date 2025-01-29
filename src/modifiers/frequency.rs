#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Modifies a noise with a frequency multiplier.
///
/// This multiplies the point by the provided `frequency` before sampling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frequency<Noise> {
    pub noise: Noise,
    pub frequency: f32,
}

impl<N> Noise for Frequency<N> {}

impl<const DIM: usize, Noise> Sample<DIM> for Frequency<Noise>
where
    Noise: Sample<DIM>,
{
    fn sample_with_seed(&self, mut point: [f32; DIM], seed: i32) -> f32 {
        let frequency = self.frequency;

        for x in &mut point {
            *x *= frequency;
        }

        self.noise.sample_with_seed(point, seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, Noise> Sample<DIM, Simd<f32, LANES>> for Frequency<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    fn sample_with_seed(&self, mut point: Simd<f32, LANES>, seed: i32) -> f32 {
        point *= Simd::splat(self.frequency);
        self.noise.sample_with_seed(point, seed)
    }
}
