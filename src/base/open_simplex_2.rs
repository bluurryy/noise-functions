#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4, num::SimdFloat};

use crate::{Noise, OpenSimplexNoise, Sample, SampleWithSeed};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

/// 2/3/4 dimensional OpenSimplex2 noise. Fast variant.
///
/// When sampling in 3 Dimensions you can improve the visual isotropy in a the respective planes via [`improve_xy`] or [`improve_xz`].
///
/// [`improve_xy`]: crate::OpenSimplexNoise::improve_xy
/// [`improve_xz`]: crate::OpenSimplexNoise::improve_xz
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSimplex2;

impl Noise for OpenSimplex2 {}

impl Sample<2> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: [f32; 2]) -> f32 {
        self.raw_sample2(improve2(point), 0)
    }
}

impl SampleWithSeed<2> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: [f32; 2], seed: i32) -> f32 {
        self.raw_sample2(improve2(point), seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<2, f32x2> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: f32x2) -> f32 {
        self.raw_sample2a(improve2a(point), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl SampleWithSeed<2, f32x2> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: f32x2, seed: i32) -> f32 {
        self.raw_sample2a(improve2a(point), seed)
    }
}

impl Sample<3> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: [f32; 3]) -> f32 {
        self.raw_sample3(improve3(point), 0)
    }
}

impl SampleWithSeed<3> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
        self.raw_sample3(improve3(point), seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<3, f32x4> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: f32x4) -> f32 {
        self.raw_sample3a(improve3a(point), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl SampleWithSeed<3, f32x4> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
        self.raw_sample3a(improve3a(point), seed)
    }
}

impl Sample<4> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: [f32; 4]) -> f32 {
        self.raw_sample4(improve4(point), 0)
    }
}

impl SampleWithSeed<4> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: [f32; 4], seed: i32) -> f32 {
        self.raw_sample4(improve4(point), seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<4, f32x4> for OpenSimplex2 {
    #[inline(always)]
    fn sample(&self, point: f32x4) -> f32 {
        self.raw_sample4a(improve4a(point), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl SampleWithSeed<4, f32x4> for OpenSimplex2 {
    #[inline(always)]
    fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
        self.raw_sample4a(improve4a(point), seed)
    }
}

impl crate::open_simplex_2::Sealed for OpenSimplex2 {}

impl OpenSimplexNoise for OpenSimplex2 {
    #[inline]
    fn raw_sample2(&self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, PRIME_X, PRIME_Y};

        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const G2: f32 = (3.0 - SQRT3) / 6.0;

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
    #[cfg(feature = "nightly-simd")]
    fn raw_sample2a(&self, point: f32x2, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, grad2_simd, splat, PRIME_X, PRIME_XY, PRIME_Y};

        use core::simd::num::SimdInt;

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
    fn raw_sample3(&self, [x, y, z]: [f32; 3], mut seed: i32) -> f32 {
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
    fn raw_sample3a(&self, point: f32x4, mut seed: i32) -> f32 {
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

    #[inline]
    fn raw_sample4(&self, point: [f32; 4], seed: i32) -> f32 {
        crate::from_open_simplex_2::fast::noise4_UnskewedBase(point, seed)
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample4a(&self, point: f32x4, seed: i32) -> f32 {
        self.raw_sample4(point.into(), seed)
    }

    #[doc(hidden)]
    fn raw_improve3_xy(&self, [mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        let xy: f32 = x + y;
        let s2: f32 = xy * -0.211324865405187;
        z *= 0.577350269189626;
        x += s2 - z;
        y = y + s2 - z;
        z += xy * 0.577350269189626;
        [x, y, z]
    }

    #[doc(hidden)]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve3a_xy(&self, point: f32x4) -> f32x4 {
        let &[x, y, z, _] = point.as_array();
        let [x, y, z] = self.raw_improve3_xy([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }

    #[doc(hidden)]
    fn raw_improve3_xz(&self, [mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        let xz: f32 = x + z;
        let s2: f32 = xz * -0.211324865405187;
        y *= 0.577350269189626;
        x += s2 - y;
        z += s2 - y;
        y += xz * 0.577350269189626;
        [x, y, z]
    }

    #[doc(hidden)]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve3a_xz(&self, point: f32x4) -> f32x4 {
        let &[x, y, z, _] = point.as_array();
        let [x, y, z] = self.raw_improve3_xz([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }

    #[inline]
    fn raw_improve4_xyz(&self, [x, y, z, w]: [f32; 4]) -> [f32; 4] {
        let xyz = x + y + z;
        let ww = w * 0.2236067977499788;
        let s2 = xyz * -0.16666666666666666 + ww;
        let xs = x + s2;
        let ys = y + s2;
        let zs = z + s2;
        let ws = -0.5 * xyz + ww;
        [xs, ys, zs, ws]
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve4a_xyz(&self, point: f32x4) -> f32x4 {
        self.raw_improve4_xyz(*point.as_array()).into()
    }

    #[inline]
    fn raw_improve4_xyz_xy(&self, [x, y, z, w]: [f32; 4]) -> [f32; 4] {
        let xy = x + y;
        let s2 = xy * -0.21132486540518699998;
        let zz = z * 0.28867513459481294226;
        let ww = w * 0.2236067977499788;
        let xr = x + (zz + ww + s2);
        let yr = y + (zz + ww + s2);
        let zr = xy * -0.57735026918962599998 + (zz + ww);
        let wr = z * -0.866025403784439 + ww;
        [xr, yr, zr, wr]
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve4a_xyz_xy(&self, point: f32x4) -> f32x4 {
        self.raw_improve4_xyz_xy(*point.as_array()).into()
    }

    #[inline]
    fn raw_improve4_xyz_xz(&self, [x, y, z, w]: [f32; 4]) -> [f32; 4] {
        let xz = x + z;
        let s2 = xz * -0.21132486540518699998;
        let yy = y * 0.28867513459481294226;
        let ww = w * 0.2236067977499788;
        let xr = x + (yy + ww + s2);
        let zr = z + (yy + ww + s2);
        let yr = xz * -0.57735026918962599998 + (yy + ww);
        let wr = y * -0.866025403784439 + ww;
        [xr, yr, zr, wr]
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve4a_xyz_xz(&self, point: f32x4) -> f32x4 {
        self.raw_improve4_xyz_xz(*point.as_array()).into()
    }

    #[inline]
    fn raw_improve4_xy_zw(&self, [x, y, z, w]: [f32; 4]) -> [f32; 4] {
        let s2 = (x + y) * -0.178275657951399372 + (z + w) * 0.215623393288842828;
        let t2 = (z + w) * -0.403949762580207112 + (x + y) * -0.375199083010075342;
        let xs = x + s2;
        let ys = y + s2;
        let zs = z + t2;
        let ws = w + t2;
        [xs, ys, zs, ws]
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve4a_xy_zw(&self, point: f32x4) -> f32x4 {
        self.raw_improve4_xy_zw(*point.as_array()).into()
    }
}

#[inline]
pub(crate) fn improve2([mut x, mut y]: [f32; 2]) -> [f32; 2] {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    let t: f32 = (x + y) * F2;
    x += t;
    y += t;
    [x, y]
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve2a(point: f32x2) -> f32x2 {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    let t: f32 = (point[0] + point[1]) * F2;
    point + splat(t)
}

#[inline]
pub(crate) fn improve3([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    const R3: f32 = 2.0 / 3.0;
    let r: f32 = (x + y + z) * R3; // Rotation, not skew
    x = r - x;
    y = r - y;
    z = r - z;
    [x, y, z]
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve3a(point: f32x4) -> f32x4 {
    const R3: f32 = 2.0 / 3.0;
    let r: f32 = (point[0] + point[1] + point[2]) * R3; // Rotation, not skew
    f32x4::splat(r) - point
}

#[inline]
pub(crate) fn improve4([mut x, mut y, mut z, mut w]: [f32; 4]) -> [f32; 4] {
    const SKEW_4D: f32 = -0.138196601125011;
    let s = SKEW_4D * (x + y + z + w);
    x += s;
    y += s;
    z += s;
    w += s;
    [x, y, z, w]
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve4a(point: f32x4) -> f32x4 {
    const SKEW_4D: f32 = -0.138196601125011;
    let s = SKEW_4D * point.reduce_sum();
    point + splat(s)
}
