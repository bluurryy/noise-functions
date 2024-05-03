use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(point: f32x2, seed: i32, jitter: f32) -> f32 {
    math::sqrt(crate::simd::cell_distance_sq::gen2_distance_squared(point, seed, jitter)) - 1.0
}

#[inline]
pub fn gen3(point: f32x4, seed: i32, jitter: f32) -> f32 {
    math::sqrt(crate::simd::cell_distance_sq::gen3_distance_squared(point, seed, jitter)) - 1.0
}
