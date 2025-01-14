use crate::{base::impl_noise, floor, lerp};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

/// 2/3/4 dimensional Value noise
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value;

impl_noise!(234 Value);

impl Value {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, interp_hermite, value2, PRIME_X, PRIME_Y};

        let mut x0 = floor_to_int(x);
        let mut y0 = floor_to_int(y);

        let xs = interp_hermite(x - x0 as f32);
        let ys = interp_hermite(y - y0 as f32);

        x0 = x0.wrapping_mul(PRIME_X);
        y0 = y0.wrapping_mul(PRIME_Y);

        let x1 = x0.wrapping_add(PRIME_X);
        let y1 = y0.wrapping_add(PRIME_Y);

        let xf0 = lerp(value2(seed, x0, y0), value2(seed, x1, y0), xs);
        let xf1 = lerp(value2(seed, x0, y1), value2(seed, x1, y1), xs);

        lerp(xf0, xf1, ys)
    }

    #[inline]
    fn gen3(self, [x, y, z]: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, interp_hermite, value3, PRIME_X, PRIME_Y, PRIME_Z};

        let mut x0 = floor_to_int(x);
        let mut y0 = floor_to_int(y);
        let mut z0 = floor_to_int(z);

        let xs = interp_hermite(x - x0 as f32);
        let ys = interp_hermite(y - y0 as f32);
        let zs = interp_hermite(z - z0 as f32);

        x0 = x0.wrapping_mul(PRIME_X);
        y0 = y0.wrapping_mul(PRIME_Y);
        z0 = z0.wrapping_mul(PRIME_Z);

        let x1 = x0.wrapping_add(PRIME_X);
        let y1 = y0.wrapping_add(PRIME_Y);
        let z1 = z0.wrapping_add(PRIME_Z);

        let xf00 = lerp(value3(seed, x0, y0, z0), value3(seed, x1, y0, z0), xs);
        let xf10 = lerp(value3(seed, x0, y1, z0), value3(seed, x1, y1, z0), xs);
        let xf01 = lerp(value3(seed, x0, y0, z1), value3(seed, x1, y0, z1), xs);
        let xf11 = lerp(value3(seed, x0, y1, z1), value3(seed, x1, y1, z1), xs);

        let yf0 = lerp(xf00, xf10, ys);
        let yf1 = lerp(xf01, xf11, ys);

        lerp(yf0, yf1, zs)
    }

    #[inline]
    fn gen4(self, [x, y, z, w]: [f32; 4], seed: i32) -> f32 {
        // implementation from FastNoise2
        use crate::from_fast_noise_2::{interp_hermite, primes, value_coord4};

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

        let xs = interp_hermite(x - xs);
        let ys = interp_hermite(y - ys);
        let zs = interp_hermite(z - zs);
        let ws = interp_hermite(w - ws);

        lerp(
            lerp(
                lerp(
                    lerp(value_coord4(seed, x0, y0, z0, w0), value_coord4(seed, x1, y0, z0, w0), xs),
                    lerp(value_coord4(seed, x0, y1, z0, w0), value_coord4(seed, x1, y1, z0, w0), xs),
                    ys,
                ),
                lerp(
                    lerp(value_coord4(seed, x0, y0, z1, w0), value_coord4(seed, x1, y0, z1, w0), xs),
                    lerp(value_coord4(seed, x0, y1, z1, w0), value_coord4(seed, x1, y1, z1, w0), xs),
                    ys,
                ),
                zs,
            ),
            lerp(
                lerp(
                    lerp(value_coord4(seed, x0, y0, z0, w1), value_coord4(seed, x1, y0, z0, w1), xs),
                    lerp(value_coord4(seed, x0, y1, z0, w1), value_coord4(seed, x1, y1, z0, w1), xs),
                    ys,
                ),
                lerp(
                    lerp(value_coord4(seed, x0, y0, z1, w1), value_coord4(seed, x1, y0, z1, w1), xs),
                    lerp(value_coord4(seed, x0, y1, z1, w1), value_coord4(seed, x1, y1, z1, w1), xs),
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
        use crate::from_fast_noise_lite::{floor_to_int, interp_hermite, value2, PRIME_XY};

        use core::simd::num::SimdInt;

        let v0 = floor_to_int(point);
        let s = interp_hermite(point - v0.cast::<f32>());
        let v0 = v0 * PRIME_XY;
        let v1 = v0 + PRIME_XY;
        let xf0 = lerp(value2(seed, v0[0], v0[1]), value2(seed, v1[0], v0[1]), s[0]);
        let xf1 = lerp(value2(seed, v0[0], v1[1]), value2(seed, v1[0], v1[1]), s[0]);
        lerp(xf0, xf1, s[1])
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use core::simd::num::SimdInt;

        use crate::from_fast_noise_lite::{floor_to_int, interp_hermite, value3, PRIME_XYZ};

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

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen4a(self, point: f32x4, seed: i32) -> f32 {
        self.gen4(point.into(), seed)
    }
}
