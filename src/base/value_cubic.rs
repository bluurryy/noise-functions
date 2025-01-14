use crate::base::impl_noise;

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueCubic;

impl_noise!(23 ValueCubic);

impl ValueCubic {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cubic_lerp, floor_to_int, value2, PRIME_X, PRIME_Y};

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
    fn gen3(self, [x, y, z]: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cubic_lerp, floor_to_int, value3, PRIME_X, PRIME_Y, PRIME_Z};

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

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cubic_lerp, floor_to_int, splat, value2, PRIME_XY};

        use core::simd::num::SimdInt;

        let v1 = floor_to_int(point);
        let s = point - v1.cast();
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
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cubic_lerp, floor_to_int, splat, value3, PRIME_XYZ};

        use core::simd::num::SimdInt;

        let v1 = floor_to_int(point);
        let s = point - v1.cast();
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
}
