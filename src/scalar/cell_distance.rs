use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(point: [f32; 2], seed: i32, jitter: f32) -> f32 {
    math::sqrt(crate::scalar::cell_distance_sq::gen2_distance_squared(point, seed, jitter)) - 1.0
}

#[inline]
pub(crate) fn gen3(point: [f32; 3], seed: i32, jitter: f32) -> f32 {
    math::sqrt(crate::scalar::cell_distance_sq::gen3_distance_squared(point, seed, jitter)) - 1.0
}
