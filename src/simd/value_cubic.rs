use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(pos: f32x2, seed: i32) -> f32 {
    let v1 = floor_to_int(pos);
    let s = pos - v1.cast();
    let v1 = v1 * PRIME_XY;
    let v0 = v1 - PRIME_XY;
    let v2 = v1 + PRIME_XY;
    let v3 = v1 + (PRIME_XY << splat(1));

    cubic_lerp(
        cubic_lerp(value2(seed, v0[0], v0[1]), value2(seed, v1[0], v0[1]), value2(seed, v2[0], v0[1]), value2(seed, v3[0], v0[1]), s[0]),
        cubic_lerp(value2(seed, v0[0], v1[1]), value2(seed, v1[0], v1[1]), value2(seed, v2[0], v1[1]), value2(seed, v3[0], v1[1]), s[0]),
        cubic_lerp(value2(seed, v0[0], v2[1]), value2(seed, v1[0], v2[1]), value2(seed, v2[0], v2[1]), value2(seed, v3[0], v2[1]), s[0]),
        cubic_lerp(value2(seed, v0[0], v3[1]), value2(seed, v1[0], v3[1]), value2(seed, v2[0], v3[1]), value2(seed, v3[0], v3[1]), s[0]),
        s[1],
    ) * (1.0 / (1.5 * 1.5))
}

#[inline]
pub fn gen3(pos: f32x4, seed: i32) -> f32 {
    let v1 = floor_to_int(pos);
    let s = pos - v1.cast();
    let v1 = v1 * PRIME_XYZ;
    let v0 = v1 - PRIME_XYZ;
    let v2 = v1 + PRIME_XYZ;
    let v3 = v1 + (PRIME_XYZ << splat(1));

    cubic_lerp(
        cubic_lerp(
            cubic_lerp(
                value3(seed, v0[0], v0[1], v0[2]),
                value3(seed, v1[0], v0[1], v0[2]),
                value3(seed, v2[0], v0[1], v0[2]),
                value3(seed, v3[0], v0[1], v0[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v1[1], v0[2]),
                value3(seed, v1[0], v1[1], v0[2]),
                value3(seed, v2[0], v1[1], v0[2]),
                value3(seed, v3[0], v1[1], v0[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v2[1], v0[2]),
                value3(seed, v1[0], v2[1], v0[2]),
                value3(seed, v2[0], v2[1], v0[2]),
                value3(seed, v3[0], v2[1], v0[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v3[1], v0[2]),
                value3(seed, v1[0], v3[1], v0[2]),
                value3(seed, v2[0], v3[1], v0[2]),
                value3(seed, v3[0], v3[1], v0[2]),
                s[0],
            ),
            s[1],
        ),
        cubic_lerp(
            cubic_lerp(
                value3(seed, v0[0], v0[1], v1[2]),
                value3(seed, v1[0], v0[1], v1[2]),
                value3(seed, v2[0], v0[1], v1[2]),
                value3(seed, v3[0], v0[1], v1[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v1[1], v1[2]),
                value3(seed, v1[0], v1[1], v1[2]),
                value3(seed, v2[0], v1[1], v1[2]),
                value3(seed, v3[0], v1[1], v1[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v2[1], v1[2]),
                value3(seed, v1[0], v2[1], v1[2]),
                value3(seed, v2[0], v2[1], v1[2]),
                value3(seed, v3[0], v2[1], v1[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v3[1], v1[2]),
                value3(seed, v1[0], v3[1], v1[2]),
                value3(seed, v2[0], v3[1], v1[2]),
                value3(seed, v3[0], v3[1], v1[2]),
                s[0],
            ),
            s[1],
        ),
        cubic_lerp(
            cubic_lerp(
                value3(seed, v0[0], v0[1], v2[2]),
                value3(seed, v1[0], v0[1], v2[2]),
                value3(seed, v2[0], v0[1], v2[2]),
                value3(seed, v3[0], v0[1], v2[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v1[1], v2[2]),
                value3(seed, v1[0], v1[1], v2[2]),
                value3(seed, v2[0], v1[1], v2[2]),
                value3(seed, v3[0], v1[1], v2[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v2[1], v2[2]),
                value3(seed, v1[0], v2[1], v2[2]),
                value3(seed, v2[0], v2[1], v2[2]),
                value3(seed, v3[0], v2[1], v2[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v3[1], v2[2]),
                value3(seed, v1[0], v3[1], v2[2]),
                value3(seed, v2[0], v3[1], v2[2]),
                value3(seed, v3[0], v3[1], v2[2]),
                s[0],
            ),
            s[1],
        ),
        cubic_lerp(
            cubic_lerp(
                value3(seed, v0[0], v0[1], v3[2]),
                value3(seed, v1[0], v0[1], v3[2]),
                value3(seed, v2[0], v0[1], v3[2]),
                value3(seed, v3[0], v0[1], v3[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v1[1], v3[2]),
                value3(seed, v1[0], v1[1], v3[2]),
                value3(seed, v2[0], v1[1], v3[2]),
                value3(seed, v3[0], v1[1], v3[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v2[1], v3[2]),
                value3(seed, v1[0], v2[1], v3[2]),
                value3(seed, v2[0], v2[1], v3[2]),
                value3(seed, v3[0], v2[1], v3[2]),
                s[0],
            ),
            cubic_lerp(
                value3(seed, v0[0], v3[1], v3[2]),
                value3(seed, v1[0], v3[1], v3[2]),
                value3(seed, v2[0], v3[1], v3[2]),
                value3(seed, v3[0], v3[1], v3[2]),
                s[0],
            ),
            s[1],
        ),
        s[2],
    ) * (1.0 / (1.5 * 1.5 * 1.5))
}
