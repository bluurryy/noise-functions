use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(pos: f32x2, seed: i32) -> f32 {
    let v0 = floor_to_int(pos);
    let d0 = pos - v0.cast::<f32>();
    let d1 = d0 - splat(1.0);
    let vs = interp_quintic(d0);
    let v0 = v0 * PRIME_XY;
    let v1 = v0 + PRIME_XY;

    let xf0 = lerp(grad2(seed, v0[0], v0[1], d0[0], d0[1]), grad2(seed, v1[0], v0[1], d1[0], d0[1]), vs[0]);
    let xf1 = lerp(grad2(seed, v0[0], v1[1], d0[0], d1[1]), grad2(seed, v1[0], v1[1], d1[0], d1[1]), vs[0]);

    lerp(xf0, xf1, vs[1]) * 1.4247691104677813
}

#[inline]
pub fn gen3(pos: f32x4, seed: i32) -> f32 {
    #[inline(always)]
    fn create_hash(mut hash: i32x4) -> Index3x4<64> {
        hash *= i32x4::splat(0x27d4eb2d);
        hash ^= hash >> i32x4::splat(15);
        Index3x4::new(hash)
    }

    let mut v0 = floor_to_int(pos);
    let d0 = pos - v0.cast::<f32>();
    let d1 = d0 - f32x4::splat(1.0);
    let vs = interp_quintic(d0);
    v0 *= PRIME_XYZ;
    let v1 = v0 + PRIME_XYZ;

    let hs = i32x4::from_array([v0[1] ^ v0[2], v1[1] ^ v0[2], v0[1] ^ v1[2], v1[1] ^ v1[2]]);

    let h = create_hash(hs ^ i32x4::splat(v0[0] ^ seed));

    let t0 = f32x4::from_array([
        grad3_with_hash(h[0], d0[0], d0[1], d0[2]),
        grad3_with_hash(h[1], d0[0], d1[1], d0[2]),
        grad3_with_hash(h[2], d0[0], d0[1], d1[2]),
        grad3_with_hash(h[3], d0[0], d1[1], d1[2]),
    ]);

    let h = create_hash(hs ^ i32x4::splat(v1[0] ^ seed));

    let t1 = f32x4::from_array([
        grad3_with_hash(h[0], d1[0], d0[1], d0[2]),
        grad3_with_hash(h[1], d1[0], d1[1], d0[2]),
        grad3_with_hash(h[2], d1[0], d0[1], d1[2]),
        grad3_with_hash(h[3], d1[0], d1[1], d1[2]),
    ]);

    let vfx = lerp(t0, t1, f32x4::splat(vs[0]));

    let yf0: f32 = lerp(vfx[0], vfx[1], vs[1]);
    let yf1: f32 = lerp(vfx[2], vfx[3], vs[1]);

    lerp(yf0, yf1, vs[2]) * 0.964921414852142333984375
}
