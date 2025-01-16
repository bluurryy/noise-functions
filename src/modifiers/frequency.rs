#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::Sample;

/// Wraps a noise and modifies its frequency.
///
/// This multiplies the point by the provided `frequency` before sampling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frequency<Noise> {
    pub noise: Noise,
    pub frequency: f32,
}

impl<const DIM: usize, Noise> Sample<DIM, [f32; DIM]> for Frequency<Noise>
where
    Noise: Sample<DIM, [f32; DIM]>,
{
    fn sample(&self, mut point: [f32; DIM]) -> f32 {
        let frequency = self.frequency;

        for x in &mut point {
            *x *= frequency;
        }

        self.noise.sample(point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, Noise> Sample<DIM, Simd<f32, LANES>> for Frequency<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    fn sample(&self, mut point: Simd<f32, LANES>) -> f32 {
        point *= Simd::splat(self.frequency);
        self.noise.sample(point)
    }
}
