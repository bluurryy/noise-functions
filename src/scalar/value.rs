use crate::private_prelude::*;

#[inline]
pub fn gen2([x, y]: [f32; 2], seed: i32) -> f32 {
    let mut x0: i32 = floor_to_int(x);
    let mut y0: i32 = floor_to_int(y);

    let xs: f32 = interp_hermite(x - x0 as f32);
    let ys: f32 = interp_hermite(y - y0 as f32);

    x0 = x0.wrapping_mul(PRIME_X);
    y0 = y0.wrapping_mul(PRIME_Y);

    let x1: i32 = x0.wrapping_add(PRIME_X);
    let y1: i32 = y0.wrapping_add(PRIME_Y);

    let xf0: f32 = lerp(value2(seed, x0, y0), value2(seed, x1, y0), xs);
    let xf1: f32 = lerp(value2(seed, x0, y1), value2(seed, x1, y1), xs);

    lerp(xf0, xf1, ys)
}

#[inline]
pub fn gen3([x, y, z]: [f32; 3], seed: i32) -> f32 {
    let mut x0: i32 = floor_to_int(x);
    let mut y0: i32 = floor_to_int(y);
    let mut z0: i32 = floor_to_int(z);

    let xs: f32 = interp_hermite(x - x0 as f32);
    let ys: f32 = interp_hermite(y - y0 as f32);
    let zs: f32 = interp_hermite(z - z0 as f32);

    x0 = x0.wrapping_mul(PRIME_X);
    y0 = y0.wrapping_mul(PRIME_Y);
    z0 = z0.wrapping_mul(PRIME_Z);

    let x1: i32 = x0.wrapping_add(PRIME_X);
    let y1: i32 = y0.wrapping_add(PRIME_Y);
    let z1: i32 = z0.wrapping_add(PRIME_Z);

    let xf00: f32 = lerp(value3(seed, x0, y0, z0), value3(seed, x1, y0, z0), xs);
    let xf10: f32 = lerp(value3(seed, x0, y1, z0), value3(seed, x1, y1, z0), xs);
    let xf01: f32 = lerp(value3(seed, x0, y0, z1), value3(seed, x1, y0, z1), xs);
    let xf11: f32 = lerp(value3(seed, x0, y1, z1), value3(seed, x1, y1, z1), xs);

    let yf0: f32 = lerp(xf00, xf10, ys);
    let yf1: f32 = lerp(xf01, xf11, ys);

    lerp(yf0, yf1, zs)
}
