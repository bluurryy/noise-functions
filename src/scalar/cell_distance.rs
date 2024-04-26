use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(pos: [f32; 2], seed: i32) -> f32 {
    math::sqrt(crate::scalar::cell_distance_sq::gen2_distance_squared(pos, seed)) - 1.0
}

#[inline]
pub(crate) fn gen3(pos: [f32; 3], seed: i32) -> f32 {
    math::sqrt(crate::scalar::cell_distance_sq::gen3_distance_squared(pos, seed)) - 1.0
}
