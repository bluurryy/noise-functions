#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{Noise, Sample};

/// Returns `max` if `value` is greater than `max` and `min` if `value` is less than `min`.
/// Otherwise this will return `value`.
///
/// Unlike [`f32::clamp`], this modifier won't panic if `!(min <= max)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clamp<Value, Min, Max> {
    pub value: Value,
    pub min: Min,
    pub max: Max,
}

impl<Value, Min, Max> Noise for Clamp<Value, Min, Max> {}

impl<const DIM: usize, Value, Min, Max> Sample<DIM> for Clamp<Value, Min, Max>
where
    Value: Sample<DIM>,
    Min: Sample<DIM>,
    Max: Sample<DIM>,
{
    #[inline]
    fn sample_with_seed(&self, point: [f32; DIM], seed: i32) -> f32 {
        clamp(self.value.sample_with_seed(point, seed), self.min.sample_with_seed(point, seed), self.max.sample_with_seed(point, seed))
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, Value, Min, Max> Sample<DIM, Simd<f32, LANES>> for Clamp<Value, Min, Max>
where
    Value: Sample<DIM, Simd<f32, LANES>>,
    Min: Sample<DIM, Simd<f32, LANES>>,
    Max: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sample_with_seed(&self, point: Simd<f32, LANES>, seed: i32) -> f32 {
        clamp(self.value.sample_with_seed(point, seed), self.min.sample_with_seed(point, seed), self.max.sample_with_seed(point, seed))
    }
}

#[inline(always)]
fn clamp(mut value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        value = min;
    }

    if value > max {
        value = max;
    }

    value
}
