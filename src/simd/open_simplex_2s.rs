use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(pos: f32x2, seed: i32) -> f32 {
    // TODO
    crate::scalar::open_simplex_2s::gen2([pos[0], pos[1]], seed)
}

#[inline]
pub fn gen3(pos: f32x4, seed: i32) -> f32 {
    // TODO
    crate::scalar::open_simplex_2s::gen3([pos[0], pos[1], pos[2]], seed)
}
