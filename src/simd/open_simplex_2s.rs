use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(point: f32x2, seed: i32) -> f32 {
    // TODO
    crate::scalar::open_simplex_2s::gen2([point[0], point[1]], seed)
}

#[inline]
pub fn gen3(point: f32x4, seed: i32) -> f32 {
    // TODO
    crate::scalar::open_simplex_2s::gen3([point[0], point[1], point[2]], seed)
}
