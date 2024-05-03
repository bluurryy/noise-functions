use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(point: f32x2, seed: i32) -> f32 {
    let v0 = floor_to_int(point);
    let s = interp_hermite(point - v0.cast::<f32>());
    let v0 = v0 * PRIME_XY;
    let v1 = v0 + PRIME_XY;
    let xf0 = lerp(value2(seed, v0[0], v0[1]), value2(seed, v1[0], v0[1]), s[0]);
    let xf1 = lerp(value2(seed, v0[0], v1[1]), value2(seed, v1[0], v1[1]), s[0]);
    lerp(xf0, xf1, s[1])
}

#[inline]
pub fn gen3(point: f32x4, seed: i32) -> f32 {
    let v0 = floor_to_int(point);
    let s = interp_hermite(point - v0.cast());
    let v0 = v0 * PRIME_XYZ;
    let v1 = v0 + PRIME_XYZ;

    let xf00 = lerp(value3(seed, v0[0], v0[1], v0[2]), value3(seed, v1[0], v0[1], v0[2]), s[0]);
    let xf10 = lerp(value3(seed, v0[0], v1[1], v0[2]), value3(seed, v1[0], v1[1], v0[2]), s[0]);
    let xf01 = lerp(value3(seed, v0[0], v0[1], v1[2]), value3(seed, v1[0], v0[1], v1[2]), s[0]);
    let xf11 = lerp(value3(seed, v0[0], v1[1], v1[2]), value3(seed, v1[0], v1[1], v1[2]), s[0]);

    let yf0 = lerp(xf00, xf10, s[1]);
    let yf1 = lerp(xf01, xf11, s[1]);

    lerp(yf0, yf1, s[2])
}
