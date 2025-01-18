use crate::{base::impl_noise, open_simplex_2::improve3, OpenSimplexNoise, Sample, SampleWithSeed};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[cfg(feature = "nightly-simd")]
use crate::open_simplex_2::improve3a;

/// 2/3 dimensional OpenSimplex2 noise. Fast variant.
///
/// When sampling in 3 Dimensions you can improve the visual isotropy in a the respective planes via [`improve_xy`] or [`improve_xz`].
///
/// [`improve_xy`]: crate::OpenSimplexNoise::improve_xy
/// [`improve_xz`]: crate::OpenSimplexNoise::improve_xz
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSimplex2;

impl_noise!(2 OpenSimplex2);

impl Sample<3> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: [f32; 3]) -> f32 {
        self.gen3(improve3(point), 0)
    }
}

impl SampleWithSeed<3> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
        self.gen3(improve3(point), seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<3, core::simd::f32x4> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: core::simd::f32x4) -> f32 {
        self.gen3a(improve3a(point), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl SampleWithSeed<3, core::simd::f32x4> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: core::simd::f32x4, seed: i32) -> f32 {
        self.gen3a(improve3a(point), seed)
    }
}

impl OpenSimplexNoise for OpenSimplex2 {
    #[inline(always)]
    fn raw_sample2(&self, point: [f32; 2], seed: i32) -> f32 {
        self.gen2(point, seed)
    }

    #[inline(always)]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample2a(&self, point: f32x2, seed: i32) -> f32 {
        self.gen2a(point, seed)
    }

    #[inline(always)]
    fn raw_sample3(&self, point: [f32; 3], seed: i32) -> f32 {
        self.gen3(point, seed)
    }

    #[inline(always)]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample3a(&self, point: f32x4, seed: i32) -> f32 {
        self.gen3a(point, seed)
    }
}

impl OpenSimplex2 {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, open_simplex_2::improve2, PRIME_X, PRIME_Y};

        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const G2: f32 = (3.0 - SQRT3) / 6.0;

        let [x, y] = improve2([x, y]);

        let mut i: i32 = floor_to_int(x);
        let mut j: i32 = floor_to_int(y);

        let xi: f32 = x - i as f32;
        let yi: f32 = y - j as f32;

        let t: f32 = (xi + yi) * G2;
        let x0: f32 = xi - t;
        let y0: f32 = yi - t;

        i = i.wrapping_mul(PRIME_X);
        j = j.wrapping_mul(PRIME_Y);

        let n0: f32;
        let n1: f32;
        let n2: f32;

        let a: f32 = 0.5 - x0 * x0 - y0 * y0;
        if a <= 0.0 {
            n0 = 0.0;
        } else {
            n0 = (a * a) * (a * a) * grad2(seed, i, j, x0, y0);
        }

        let c: f32 = (2.0 * (1.0 - 2.0 * G2) * (1.0 / G2 - 2.0)) * t + ((-2.0 * (1.0 - 2.0 * G2) * (1.0 - 2.0 * G2)) + a);
        if c <= 0.0 {
            n2 = 0.0;
        } else {
            let x2: f32 = x0 + (2.0 * G2 - 1.0);
            let y2: f32 = y0 + (2.0 * G2 - 1.0);
            n2 = (c * c) * (c * c) * grad2(seed, i.wrapping_add(PRIME_X), j.wrapping_add(PRIME_Y), x2, y2);
        }

        if y0 > x0 {
            let x1: f32 = x0 + G2;
            let y1: f32 = y0 + (G2 - 1.0);
            let b: f32 = 0.5 - x1 * x1 - y1 * y1;
            if b <= 0.0 {
                n1 = 0.0;
            } else {
                n1 = (b * b) * (b * b) * grad2(seed, i, j.wrapping_add(PRIME_Y), x1, y1);
            }
        } else {
            let x1: f32 = x0 + (G2 - 1.0);
            let y1: f32 = y0 + G2;
            let b: f32 = 0.5 - x1 * x1 - y1 * y1;
            if b <= 0.0 {
                n1 = 0.0;
            } else {
                n1 = (b * b) * (b * b) * grad2(seed, i.wrapping_add(PRIME_X), j, x1, y1);
            }
        }

        (n0 + n1 + n2) * 99.83685446303647
    }

    #[inline]
    fn gen3(self, [x, y, z]: [f32; 3], mut seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{grad3, round_to_int, PRIME_X, PRIME_Y, PRIME_Z};

        let mut i: i32 = round_to_int(x);
        let mut j: i32 = round_to_int(y);
        let mut k: i32 = round_to_int(z);
        let mut x0: f32 = x - i as f32;
        let mut y0: f32 = y - j as f32;
        let mut z0: f32 = z - k as f32;

        let mut x_nsign: i32 = (-1.0 - x0) as i32 | 1;
        let mut y_nsign: i32 = (-1.0 - y0) as i32 | 1;
        let mut z_nsign: i32 = (-1.0 - z0) as i32 | 1;

        let mut ax0: f32 = x_nsign as f32 * -x0;
        let mut ay0: f32 = y_nsign as f32 * -y0;
        let mut az0: f32 = z_nsign as f32 * -z0;

        i = i.wrapping_mul(PRIME_X);
        j = j.wrapping_mul(PRIME_Y);
        k = k.wrapping_mul(PRIME_Z);

        let mut value: f32 = 0.0;
        let mut a: f32 = (0.6 - x0 * x0) - (y0 * y0 + z0 * z0);

        for l in 0..2 {
            if a > 0.0 {
                value += (a * a) * (a * a) * grad3(seed, i, j, k, x0, y0, z0);
            }

            let mut b: f32 = a + 1.0;
            let mut i1 = i;
            let mut j1 = j;
            let mut k1 = k;
            let mut x1: f32 = x0;
            let mut y1: f32 = y0;
            let mut z1: f32 = z0;

            if ax0 >= ay0 && ax0 >= az0 {
                x1 += x_nsign as f32;
                b -= x_nsign as f32 * 2.0 * x1;
                i1 = i1.wrapping_sub(x_nsign.wrapping_mul(PRIME_X));
            } else if ay0 > ax0 && ay0 >= az0 {
                y1 += y_nsign as f32;
                b -= y_nsign as f32 * 2.0 * y1;
                j1 = j1.wrapping_sub(y_nsign.wrapping_mul(PRIME_Y));
            } else {
                z1 += z_nsign as f32;
                b -= z_nsign as f32 * 2.0 * z1;
                k1 = k1.wrapping_sub(z_nsign.wrapping_mul(PRIME_Z));
            }

            if b > 0.0 {
                value += (b * b) * (b * b) * grad3(seed, i1, j1, k1, x1, y1, z1);
            }

            if l == 1 {
                break;
            }

            ax0 = 0.5 - ax0;
            ay0 = 0.5 - ay0;
            az0 = 0.5 - az0;

            x0 = x_nsign as f32 * ax0;
            y0 = y_nsign as f32 * ay0;
            z0 = z_nsign as f32 * az0;

            a += (0.75 - ax0) - (ay0 + az0);

            i = i.wrapping_add(x_nsign.wrapping_shr(1) & PRIME_X);
            j = j.wrapping_add(y_nsign.wrapping_shr(1) & PRIME_Y);
            k = k.wrapping_add(z_nsign.wrapping_shr(1) & PRIME_Z);

            x_nsign = x_nsign.wrapping_neg();
            y_nsign = y_nsign.wrapping_neg();
            z_nsign = z_nsign.wrapping_neg();

            seed = !seed;
        }

        value * 32.69428253173828125
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, grad2_simd, open_simplex_2::improve2a, splat, PRIME_X, PRIME_XY, PRIME_Y};

        use core::simd::num::SimdInt;

        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const G2: f32 = (3.0 - SQRT3) / 6.0;

        let point = improve2a(point);

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
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, mut seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{grad3_simd, round_to_int, splat, PRIME_X, PRIME_XYZ, PRIME_Y, PRIME_Z};

        use core::simd::num::{SimdFloat, SimdInt};

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
}
