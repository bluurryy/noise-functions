use crate::{base::impl_noise, floor, lerp};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Perlin;

impl_noise!(234 Perlin);

impl Perlin {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, interp_quintic, PRIME_X, PRIME_Y};

        let mut x0 = floor_to_int(x);
        let mut y0 = floor_to_int(y);

        let xd0 = x - x0 as f32;
        let yd0 = y - y0 as f32;

        let xd1 = xd0 - 1.0;
        let yd1 = yd0 - 1.0;

        let xs = interp_quintic(xd0);
        let ys = interp_quintic(yd0);

        x0 = x0.wrapping_mul(PRIME_X);
        y0 = y0.wrapping_mul(PRIME_Y);

        let x1 = x0.wrapping_add(PRIME_X);
        let y1 = y0.wrapping_add(PRIME_Y);

        let xf0 = lerp(grad2(seed, x0, y0, xd0, yd0), grad2(seed, x1, y0, xd1, yd0), xs);
        let xf1 = lerp(grad2(seed, x0, y1, xd0, yd1), grad2(seed, x1, y1, xd1, yd1), xs);

        lerp(xf0, xf1, ys) * 1.4247691104677813
    }

    #[inline]
    fn gen3(self, [x, y, z]: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad3, interp_quintic, PRIME_X, PRIME_Y, PRIME_Z};

        let mut x0 = floor_to_int(x);
        let mut y0 = floor_to_int(y);
        let mut z0 = floor_to_int(z);

        let xd0 = x - x0 as f32;
        let yd0 = y - y0 as f32;
        let zd0 = z - z0 as f32;

        let xd1 = xd0 - 1.0;
        let yd1 = yd0 - 1.0;
        let zd1 = zd0 - 1.0;

        let xs = interp_quintic(xd0);
        let ys = interp_quintic(yd0);
        let zs = interp_quintic(zd0);

        x0 = x0.wrapping_mul(PRIME_X);
        y0 = y0.wrapping_mul(PRIME_Y);
        z0 = z0.wrapping_mul(PRIME_Z);

        let x1 = x0.wrapping_add(PRIME_X);
        let y1 = y0.wrapping_add(PRIME_Y);
        let z1 = z0.wrapping_add(PRIME_Z);

        let xf00 = lerp(grad3(seed, x0, y0, z0, xd0, yd0, zd0), grad3(seed, x1, y0, z0, xd1, yd0, zd0), xs);
        let xf10 = lerp(grad3(seed, x0, y1, z0, xd0, yd1, zd0), grad3(seed, x1, y1, z0, xd1, yd1, zd0), xs);
        let xf01 = lerp(grad3(seed, x0, y0, z1, xd0, yd0, zd1), grad3(seed, x1, y0, z1, xd1, yd0, zd1), xs);
        let xf11 = lerp(grad3(seed, x0, y1, z1, xd0, yd1, zd1), grad3(seed, x1, y1, z1, xd1, yd1, zd1), xs);

        let yf0 = lerp(xf00, xf10, ys);
        let yf1 = lerp(xf01, xf11, ys);

        lerp(yf0, yf1, zs) * 0.964921414852142333984375
    }

    #[inline]
    fn gen4(self, [x, y, z, w]: [f32; 4], seed: i32) -> f32 {
        // implementation from FastNoise2
        use crate::from_fast_noise_2::{gradient_dot4, hash_primes4, interp_quintic, primes};

        let xs = floor(x);
        let ys = floor(y);
        let zs = floor(z);
        let ws = floor(w);

        let x0 = (xs as i32).wrapping_mul(primes::X);
        let y0 = (ys as i32).wrapping_mul(primes::Y);
        let z0 = (zs as i32).wrapping_mul(primes::Z);
        let w0 = (ws as i32).wrapping_mul(primes::W);
        let x1 = x0.wrapping_add(primes::X);
        let y1 = y0.wrapping_add(primes::Y);
        let z1 = z0.wrapping_add(primes::Z);
        let w1 = w0.wrapping_add(primes::W);

        let xf0 = x - xs;
        let yf0 = y - ys;
        let zf0 = z - zs;
        let wf0 = w - ws;
        let xf1 = xf0 - 1.0;
        let yf1 = yf0 - 1.0;
        let zf1 = zf0 - 1.0;
        let wf1 = wf0 - 1.0;

        let xs = interp_quintic(xf0);
        let ys = interp_quintic(yf0);
        let zs = interp_quintic(zf0);
        let ws = interp_quintic(wf0);

        0.964921414852142333984375f32
            * lerp(
                lerp(
                    lerp(
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y0, z0, w0), xf0, yf0, zf0, wf0),
                            gradient_dot4(hash_primes4(seed, x1, y0, z0, w0), xf1, yf0, zf0, wf0),
                            xs,
                        ),
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y1, z0, w0), xf0, yf1, zf0, wf0),
                            gradient_dot4(hash_primes4(seed, x1, y1, z0, w0), xf1, yf1, zf0, wf0),
                            xs,
                        ),
                        ys,
                    ),
                    lerp(
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y0, z1, w0), xf0, yf0, zf1, wf0),
                            gradient_dot4(hash_primes4(seed, x1, y0, z1, w0), xf1, yf0, zf1, wf0),
                            xs,
                        ),
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y1, z1, w0), xf0, yf1, zf1, wf0),
                            gradient_dot4(hash_primes4(seed, x1, y1, z1, w0), xf1, yf1, zf1, wf0),
                            xs,
                        ),
                        ys,
                    ),
                    zs,
                ),
                lerp(
                    lerp(
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y0, z0, w1), xf0, yf0, zf0, wf1),
                            gradient_dot4(hash_primes4(seed, x1, y0, z0, w1), xf1, yf0, zf0, wf1),
                            xs,
                        ),
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y1, z0, w1), xf0, yf1, zf0, wf1),
                            gradient_dot4(hash_primes4(seed, x1, y1, z0, w1), xf1, yf1, zf0, wf1),
                            xs,
                        ),
                        ys,
                    ),
                    lerp(
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y0, z1, w1), xf0, yf0, zf1, wf1),
                            gradient_dot4(hash_primes4(seed, x1, y0, z1, w1), xf1, yf0, zf1, wf1),
                            xs,
                        ),
                        lerp(
                            gradient_dot4(hash_primes4(seed, x0, y1, z1, w1), xf0, yf1, zf1, wf1),
                            gradient_dot4(hash_primes4(seed, x1, y1, z1, w1), xf1, yf1, zf1, wf1),
                            xs,
                        ),
                        ys,
                    ),
                    zs,
                ),
                ws,
            )
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, interp_quintic, splat, PRIME_XY};

        use core::simd::num::SimdInt;

        let v0 = floor_to_int(point);
        let d0 = point - v0.cast::<f32>();
        let d1 = d0 - splat(1.0);
        let vs = interp_quintic(d0);
        let v0 = v0 * PRIME_XY;
        let v1 = v0 + PRIME_XY;

        let xf0 = lerp(grad2(seed, v0[0], v0[1], d0[0], d0[1]), grad2(seed, v1[0], v0[1], d1[0], d0[1]), vs[0]);
        let xf1 = lerp(grad2(seed, v0[0], v1[1], d0[0], d1[1]), grad2(seed, v1[0], v1[1], d1[0], d1[1]), vs[0]);

        lerp(xf0, xf1, vs[1]) * 1.4247691104677813
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use core::simd::{i32x4, num::SimdInt};

        use crate::from_fast_noise_lite::{floor_to_int, grad3_with_hash, interp_quintic, Index3x4, PRIME_XYZ};

        #[inline(always)]
        fn create_hash(mut hash: i32x4) -> Index3x4<64> {
            hash *= i32x4::splat(0x27d4eb2d);
            hash ^= hash >> i32x4::splat(15);
            Index3x4::new(hash)
        }

        let mut v0 = floor_to_int(point);
        let d0 = point - v0.cast::<f32>();
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

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen4a(self, point: f32x4, seed: i32) -> f32 {
        self.gen4(point.into(), seed)
    }
}
