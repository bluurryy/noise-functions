use crate::{
    private_prelude::*,
    simd::{grad2_simd, grad3_simd},
};

#[inline]
pub(crate) fn gen2(point: f32x2, seed: i32) -> f32 {
    gen2_not_improved(improve2(point), seed)
}

#[inline]
pub fn gen2_not_improved(point: f32x2, seed: i32) -> f32 {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const G2: f32 = (3.0 - SQRT3) / 6.0;

    let i = floor_to_int(point);
    let vi = point - i.cast();

    let t = (vi[0] + vi[1]) * G2;
    let v0 = vi - splat(t);

    let i = i * PRIME_XY;

    let n0: f32;
    let n1: f32;
    let n2: f32;

    let a: f32 = 0.5 - v0[0] * v0[0] - v0[1] * v0[1];
    if a <= 0.0 {
        n0 = 0.0;
    } else {
        n0 = (a * a) * (a * a) * grad2_simd(seed, i, v0);
    }

    let c: f32 = (2.0 * (1.0 - 2.0 * G2) * (1.0 / G2 - 2.0)) * t + ((-2.0 * (1.0 - 2.0 * G2) * (1.0 - 2.0 * G2)) + a);

    if c <= 0.0 {
        n2 = 0.0;
    } else {
        let v2 = v0 + splat(2.0 * G2 - 1.0);
        n2 = (c * c) * (c * c) * grad2_simd(seed, i + PRIME_XY, v2);
    }

    if v0[1] > v0[0] {
        let x1 = v0[0] + G2;
        let y1 = v0[1] + (G2 - 1.0);
        let b = 0.5 - x1 * x1 - y1 * y1;
        if b <= 0.0 {
            n1 = 0.0;
        } else {
            n1 = (b * b) * (b * b) * grad2(seed, i[0], i[1].wrapping_add(PRIME_Y), x1, y1);
        }
    } else {
        let x1 = v0[0] + (G2 - 1.0);
        let y1 = v0[1] + G2;
        let b = 0.5 - x1 * x1 - y1 * y1;
        if b <= 0.0 {
            n1 = 0.0;
        } else {
            n1 = (b * b) * (b * b) * grad2(seed, i[0].wrapping_add(PRIME_X), i[1], x1, y1);
        }
    }

    (n0 + n1 + n2) * 99.83685446303647
}

#[inline]
pub fn improve2(point: f32x2) -> f32x2 {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    let t: f32 = (point[0] + point[1]) * F2;
    point + splat(t)
}

#[inline]
pub fn gen3(point: f32x4, mut seed: i32) -> f32 {
    let i = round_to_int(point);
    let mut v0 = point - i.cast();
    let mut nsign = (splat(-1.0) - v0).cast::<i32>() | splat(1);
    let mut a0 = nsign.cast() * -v0;
    let mut i = i * PRIME_XYZ;

    let mut value: f32 = 0.0;
    let mut a: f32 = (0.6 - v0[0] * v0[0]) - (v0[1] * v0[1] + v0[2] * v0[2]);

    for l in 0..2 {
        if a > 0.0 {
            value += (a * a) * (a * a) * grad3_simd(seed, i, v0);
        }

        let mut b: f32 = a + 1.0;
        let mut i1 = i;
        let mut v1 = v0;

        if a0[0] >= a0[1] && a0[0] >= a0[2] {
            v1[0] += nsign[0] as f32;
            b -= nsign[0] as f32 * 2.0 * v1[0];
            i1[0] = i1[0].wrapping_sub(nsign[0].wrapping_mul(PRIME_X));
        } else if a0[1] > a0[0] && a0[1] >= a0[2] {
            v1[1] += nsign[1] as f32;
            b -= nsign[1] as f32 * 2.0 * v1[1];
            i1[1] = i1[1].wrapping_sub(nsign[1].wrapping_mul(PRIME_Y));
        } else {
            v1[2] += nsign[2] as f32;
            b -= nsign[2] as f32 * 2.0 * v1[2];
            i1[2] = i1[2].wrapping_sub(nsign[2].wrapping_mul(PRIME_Z));
        }

        if b > 0.0 {
            value += (b * b) * (b * b) * grad3_simd(seed, i1, v1);
        }

        if l == 1 {
            break;
        }

        a0 = splat(0.5) - a0;
        v0 = nsign.cast() * a0;
        a += (0.75 - a0[0]) - (a0[1] + a0[2]);
        i += (nsign >> splat(1)) & PRIME_XYZ;
        nsign = -nsign;
        seed = !seed;
    }

    value * 32.69428253173828125
}
