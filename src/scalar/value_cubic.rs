use crate::private_prelude::*;

#[inline]
pub fn gen2([x, y]: [f32; 2], seed: i32) -> f32 {
    let mut x1: i32 = floor_to_int(x);
    let mut y1: i32 = floor_to_int(y);

    let xs: f32 = x - x1 as f32;
    let ys: f32 = y - y1 as f32;

    x1 = x1.wrapping_mul(PRIME_X);
    y1 = y1.wrapping_mul(PRIME_Y);

    let x0: i32 = x1.wrapping_sub(PRIME_X);
    let y0: i32 = y1.wrapping_sub(PRIME_Y);
    let x2: i32 = x1.wrapping_add(PRIME_X);
    let y2: i32 = y1.wrapping_add(PRIME_Y);
    let x3: i32 = x1.wrapping_add(((PRIME_X as i64).wrapping_shl(1)) as i32);
    let y3: i32 = y1.wrapping_add(((PRIME_Y as i64).wrapping_shl(1)) as i32);

    cubic_lerp(
        cubic_lerp(value2(seed, x0, y0), value2(seed, x1, y0), value2(seed, x2, y0), value2(seed, x3, y0), xs),
        cubic_lerp(value2(seed, x0, y1), value2(seed, x1, y1), value2(seed, x2, y1), value2(seed, x3, y1), xs),
        cubic_lerp(value2(seed, x0, y2), value2(seed, x1, y2), value2(seed, x2, y2), value2(seed, x3, y2), xs),
        cubic_lerp(value2(seed, x0, y3), value2(seed, x1, y3), value2(seed, x2, y3), value2(seed, x3, y3), xs),
        ys,
    ) * (1.0 / (1.5 * 1.5))
}

#[inline]
pub fn gen3([x, y, z]: [f32; 3], seed: i32) -> f32 {
    let mut x1: i32 = floor_to_int(x);
    let mut y1: i32 = floor_to_int(y);
    let mut z1: i32 = floor_to_int(z);

    let xs: f32 = x - x1 as f32;
    let ys: f32 = y - y1 as f32;
    let zs: f32 = z - z1 as f32;

    x1 = x1.wrapping_mul(PRIME_X);
    y1 = y1.wrapping_mul(PRIME_Y);
    z1 = z1.wrapping_mul(PRIME_Z);

    let x0: i32 = x1.wrapping_sub(PRIME_X);
    let y0: i32 = y1.wrapping_sub(PRIME_Y);
    let z0: i32 = z1.wrapping_sub(PRIME_Z);

    let x2: i32 = x1.wrapping_add(PRIME_X);
    let y2: i32 = y1.wrapping_add(PRIME_Y);
    let z2: i32 = z1.wrapping_add(PRIME_Z);

    let x3: i32 = x1.wrapping_add((PRIME_X as i64).wrapping_shl(1) as i32);
    let y3: i32 = y1.wrapping_add((PRIME_Y as i64).wrapping_shl(1) as i32);
    let z3: i32 = z1.wrapping_add((PRIME_Z as i64).wrapping_shl(1) as i32);

    cubic_lerp(
        cubic_lerp(
            cubic_lerp(value3(seed, x0, y0, z0), value3(seed, x1, y0, z0), value3(seed, x2, y0, z0), value3(seed, x3, y0, z0), xs),
            cubic_lerp(value3(seed, x0, y1, z0), value3(seed, x1, y1, z0), value3(seed, x2, y1, z0), value3(seed, x3, y1, z0), xs),
            cubic_lerp(value3(seed, x0, y2, z0), value3(seed, x1, y2, z0), value3(seed, x2, y2, z0), value3(seed, x3, y2, z0), xs),
            cubic_lerp(value3(seed, x0, y3, z0), value3(seed, x1, y3, z0), value3(seed, x2, y3, z0), value3(seed, x3, y3, z0), xs),
            ys,
        ),
        cubic_lerp(
            cubic_lerp(value3(seed, x0, y0, z1), value3(seed, x1, y0, z1), value3(seed, x2, y0, z1), value3(seed, x3, y0, z1), xs),
            cubic_lerp(value3(seed, x0, y1, z1), value3(seed, x1, y1, z1), value3(seed, x2, y1, z1), value3(seed, x3, y1, z1), xs),
            cubic_lerp(value3(seed, x0, y2, z1), value3(seed, x1, y2, z1), value3(seed, x2, y2, z1), value3(seed, x3, y2, z1), xs),
            cubic_lerp(value3(seed, x0, y3, z1), value3(seed, x1, y3, z1), value3(seed, x2, y3, z1), value3(seed, x3, y3, z1), xs),
            ys,
        ),
        cubic_lerp(
            cubic_lerp(value3(seed, x0, y0, z2), value3(seed, x1, y0, z2), value3(seed, x2, y0, z2), value3(seed, x3, y0, z2), xs),
            cubic_lerp(value3(seed, x0, y1, z2), value3(seed, x1, y1, z2), value3(seed, x2, y1, z2), value3(seed, x3, y1, z2), xs),
            cubic_lerp(value3(seed, x0, y2, z2), value3(seed, x1, y2, z2), value3(seed, x2, y2, z2), value3(seed, x3, y2, z2), xs),
            cubic_lerp(value3(seed, x0, y3, z2), value3(seed, x1, y3, z2), value3(seed, x2, y3, z2), value3(seed, x3, y3, z2), xs),
            ys,
        ),
        cubic_lerp(
            cubic_lerp(value3(seed, x0, y0, z3), value3(seed, x1, y0, z3), value3(seed, x2, y0, z3), value3(seed, x3, y0, z3), xs),
            cubic_lerp(value3(seed, x0, y1, z3), value3(seed, x1, y1, z3), value3(seed, x2, y1, z3), value3(seed, x3, y1, z3), xs),
            cubic_lerp(value3(seed, x0, y2, z3), value3(seed, x1, y2, z3), value3(seed, x2, y2, z3), value3(seed, x3, y2, z3), xs),
            cubic_lerp(value3(seed, x0, y3, z3), value3(seed, x1, y3, z3), value3(seed, x2, y3, z3), value3(seed, x3, y3, z3), xs),
            ys,
        ),
        zs,
    ) * (1.0 / (1.5 * 1.5 * 1.5))
}
