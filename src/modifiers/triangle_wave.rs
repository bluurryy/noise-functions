#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{math::floor, Noise, Sample};

/// Applies a triangle wave to the output of a noise function.
///
/// This outputs values is in the [-1, 1] range.
///
/// **Note:** This modifier assumes the base noise returns values in the [-1, 1] range.
pub struct TriangleWave<Noise> {
    pub noise: Noise,
    pub frequency: f32,
}

impl<N> Noise for TriangleWave<N> {}

impl<Noise, const DIM: usize> Sample<DIM> for TriangleWave<Noise>
where
    Noise: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        apply(self.noise.sample_with_seed(point, seed), self.frequency)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for TriangleWave<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        apply(self.noise.sample_with_seed(point, seed), self.frequency)
    }
}

#[inline]
fn apply(value: f32, frequency: f32) -> f32 {
    let v = (value + 1.0) * frequency;
    let v = v - floor(v * 0.5) * 2.0;
    let v = if v < 1.0 { v } else { 2.0 - v };
    (v - 0.5) * 2.0
}
