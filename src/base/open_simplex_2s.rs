use core::num::Wrapping;
#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4, num::SimdFloat};

use crate::{
    from_open_simplex_2::smooth_luts::{GRADIENTS_2D, GRADIENTS_3D, GRADIENTS_4D, LOOKUP_4D_A, LOOKUP_4D_B},
    open_simplex_2::impl_open_simplex_noise,
    OpenSimplexNoise,
};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

/// 2/3/4 dimensional OpenSimplex2 noise. Smooth variant.
///
/// You can improve the visual isotropy for certain orientations using the `improve_*` methods
/// provided by the [`OpenSimplexNoise`] trait.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSimplex2s;

impl_open_simplex_noise!(234 OpenSimplex2s);

impl OpenSimplexNoise for OpenSimplex2s {
    #[inline]
    fn raw_sample2(&self, [xs, ys]: [f32; 2], seed: i32) -> f32 {
        let seed = Wrapping(seed as i64);

        // Get base points and offsets.
        let xsb = fast_floor(xs);
        let ysb = fast_floor(ys);
        let xi = xs - xsb as f32;
        let yi = ys - ysb as f32;

        // Prime pre-multiplication for hash.
        let xsbp = Wrapping(xsb as i64) * Wrapping(PRIME_X);
        let ysbp = Wrapping(ysb as i64) * Wrapping(PRIME_Y);

        // Unskew.
        let t = (xi + yi) * UNSKEW_2D;
        let dx0 = xi + t;
        let dy0 = yi + t;

        // First vertex.
        let a0 = RSQUARED_2D - dx0 * dx0 - dy0 * dy0;
        let mut value = (a0 * a0) * (a0 * a0) * grad2(seed, xsbp, ysbp, dx0, dy0);

        // Second vertex.
        let a1 = (2.0 * (1.0 + 2.0 * UNSKEW_2D) * (1.0 / UNSKEW_2D + 2.0)) * t + ((-2.0 * (1.0 + 2.0 * UNSKEW_2D) * (1.0 + 2.0 * UNSKEW_2D)) + a0);
        let dx1 = dx0 - (1.0 + 2.0 * UNSKEW_2D);
        let dy1 = dy0 - (1.0 + 2.0 * UNSKEW_2D);
        value += (a1 * a1) * (a1 * a1) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp + Wrapping(PRIME_Y), dx1, dy1);

        // Third and fourth vertices.
        // Nested conditionals were faster than compact bit logic/arithmetic.
        let xmyi = xi - yi;
        if t < UNSKEW_2D {
            if xi + xmyi > 1.0 {
                let dx2 = dx0 - (3.0 * UNSKEW_2D + 2.0);
                let dy2 = dy0 - (3.0 * UNSKEW_2D + 1.0);
                let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp + Wrapping(PRIME_X << 1), ysbp + Wrapping(PRIME_Y), dx2, dy2);
                }
            } else {
                let dx2 = dx0 - UNSKEW_2D;
                let dy2 = dy0 - (UNSKEW_2D + 1.0);
                let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp + Wrapping(PRIME_Y), dx2, dy2);
                }
            }

            if yi - xmyi > 1.0 {
                let dx3 = dx0 - (3.0 * UNSKEW_2D + 1.0);
                let dy3 = dy0 - (3.0 * UNSKEW_2D + 2.0);
                let a3 = RSQUARED_2D - dx3 * dx3 - dy3 * dy3;
                if a3 > 0.0 {
                    value += (a3 * a3) * (a3 * a3) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp + Wrapping(PRIME_Y << 1), dx3, dy3);
                }
            } else {
                let dx3 = dx0 - (UNSKEW_2D + 1.0);
                let dy3 = dy0 - UNSKEW_2D;
                let a3 = RSQUARED_2D - dx3 * dx3 - dy3 * dy3;
                if a3 > 0.0 {
                    value += (a3 * a3) * (a3 * a3) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp, dx3, dy3);
                }
            }
        } else {
            if xi + xmyi < 0.0 {
                let dx2 = dx0 + (1.0 + UNSKEW_2D);
                let dy2 = dy0 + UNSKEW_2D;
                let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp - Wrapping(PRIME_X), ysbp, dx2, dy2);
                }
            } else {
                let dx2 = dx0 - (UNSKEW_2D + 1.0);
                let dy2 = dy0 - UNSKEW_2D;
                let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp, dx2, dy2);
                }
            }

            if yi < xmyi {
                let dx2 = dx0 + UNSKEW_2D;
                let dy2 = dy0 + (UNSKEW_2D + 1.0);
                let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp - Wrapping(PRIME_Y), dx2, dy2);
                }
            } else {
                let dx2 = dx0 - UNSKEW_2D;
                let dy2 = dy0 - (UNSKEW_2D + 1.0);
                let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp + Wrapping(PRIME_Y), dx2, dy2);
                }
            }
        }

        value
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample2a(&self, point: f32x2, seed: i32) -> f32 {
        self.raw_sample2(point.into(), seed)
    }

    #[inline]
    fn raw_sample3(&self, [xr, yr, zr]: [f32; 3], seed: i32) -> f32 {
        let seed = Wrapping(seed as i64);

        // Get base points and offsets.
        let xrb = fast_floor(xr);
        let yrb = fast_floor(yr);
        let zrb = fast_floor(zr);
        let xi = (xr - xrb as f32) as f32;
        let yi = (yr - yrb as f32) as f32;
        let zi = (zr - zrb as f32) as f32;

        // Prime pre-multiplication for hash. Also flip seed for second lattice copy.
        let xrbp = Wrapping(xrb as i64) * Wrapping(PRIME_X);
        let yrbp = Wrapping(yrb as i64) * Wrapping(PRIME_Y);
        let zrbp = Wrapping(zrb as i64) * Wrapping(PRIME_Z);
        let seed2 = seed ^ Wrapping(SEED_FLIP_3D);

        // -1 if positive, 0 if negative.
        let x_nmask = (-0.5 - xi) as i32;
        let y_nmask = (-0.5 - yi) as i32;
        let z_nmask = (-0.5 - zi) as i32;

        // First vertex.
        let x0 = xi + x_nmask as f32;
        let y0 = yi + y_nmask as f32;
        let z0 = zi + z_nmask as f32;
        let a0 = RSQUARED_3D - x0 * x0 - y0 * y0 - z0 * z0;
        let mut value = (a0 * a0)
            * (a0 * a0)
            * grad3(
                seed,
                xrbp + (Wrapping(x_nmask as i64) & Wrapping(PRIME_X)),
                yrbp + (Wrapping(y_nmask as i64) & Wrapping(PRIME_Y)),
                zrbp + (Wrapping(z_nmask as i64) & Wrapping(PRIME_Z)),
                x0,
                y0,
                z0,
            );

        // Second vertex.
        let x1 = xi - 0.5;
        let y1 = yi - 0.5;
        let z1 = zi - 0.5;
        let a1 = RSQUARED_3D - x1 * x1 - y1 * y1 - z1 * z1;
        value += (a1 * a1) * (a1 * a1) * grad3(seed2, xrbp + Wrapping(PRIME_X), yrbp + Wrapping(PRIME_Y), zrbp + Wrapping(PRIME_Z), x1, y1, z1);

        // Shortcuts for building the remaining falloffs.
        // Derived by subtracting the polynomials with the offsets plugged in.
        let x_aflip_mask0 = ((x_nmask | 1) << 1) as f32 * x1;
        let y_aflip_mask0 = ((y_nmask | 1) << 1) as f32 * y1;
        let z_aflip_mask0 = ((z_nmask | 1) << 1) as f32 * z1;
        let x_aflip_mask1 = (-2 - (x_nmask << 2)) as f32 * x1 - 1.0;
        let y_aflip_mask1 = (-2 - (y_nmask << 2)) as f32 * y1 - 1.0;
        let z_aflip_mask1 = (-2 - (z_nmask << 2)) as f32 * z1 - 1.0;

        let mut skip5 = false;
        let a2 = x_aflip_mask0 + a0;
        if a2 > 0.0 {
            let x2 = x0 - (x_nmask | 1) as f32;
            let y2 = y0;
            let z2 = z0;
            value += (a2 * a2)
                * (a2 * a2)
                * grad3(
                    seed,
                    xrbp + (Wrapping(!x_nmask as i64) & Wrapping(PRIME_X)),
                    yrbp + (Wrapping(y_nmask as i64) & Wrapping(PRIME_Y)),
                    zrbp + (Wrapping(z_nmask as i64) & Wrapping(PRIME_Z)),
                    x2,
                    y2,
                    z2,
                );
        } else {
            let a3 = y_aflip_mask0 + z_aflip_mask0 + a0;
            if a3 > 0.0 {
                let x3 = x0;
                let y3 = y0 - (y_nmask | 1) as f32;
                let z3 = z0 - (z_nmask | 1) as f32;
                value += (a3 * a3)
                    * (a3 * a3)
                    * grad3(
                        seed,
                        xrbp + (Wrapping(x_nmask as i64) & Wrapping(PRIME_X)),
                        yrbp + (Wrapping(!y_nmask as i64) & Wrapping(PRIME_Y)),
                        zrbp + (Wrapping(!z_nmask as i64) & Wrapping(PRIME_Z)),
                        x3,
                        y3,
                        z3,
                    );
            }

            let a4 = x_aflip_mask1 + a1;
            if a4 > 0.0 {
                let x4 = (x_nmask | 1) as f32 + x1;
                let y4 = y1;
                let z4 = z1;
                value += (a4 * a4)
                    * (a4 * a4)
                    * grad3(
                        seed2,
                        xrbp + (Wrapping(x_nmask as i64) & (Wrapping(PRIME_X) << 1)),
                        yrbp + Wrapping(PRIME_Y),
                        zrbp + Wrapping(PRIME_Z),
                        x4,
                        y4,
                        z4,
                    );
                skip5 = true;
            }
        }

        let mut skip9 = false;
        let a6 = y_aflip_mask0 + a0;
        if a6 > 0.0 {
            let x6 = x0;
            let y6 = y0 - (y_nmask | 1) as f32;
            let z6 = z0;
            value += (a6 * a6)
                * (a6 * a6)
                * grad3(
                    seed,
                    xrbp + (Wrapping(x_nmask as i64) & Wrapping(PRIME_X)),
                    yrbp + (Wrapping(!y_nmask as i64) & Wrapping(PRIME_Y)),
                    zrbp + (Wrapping(z_nmask as i64) & Wrapping(PRIME_Z)),
                    x6,
                    y6,
                    z6,
                );
        } else {
            let a7 = x_aflip_mask0 + z_aflip_mask0 + a0;
            if a7 > 0.0 {
                let x7 = x0 - (x_nmask | 1) as f32;
                let y7 = y0;
                let z7 = z0 - (z_nmask | 1) as f32;
                value += (a7 * a7)
                    * (a7 * a7)
                    * grad3(
                        seed,
                        xrbp + (Wrapping(!x_nmask as i64) & Wrapping(PRIME_X)),
                        yrbp + (Wrapping(y_nmask as i64) & Wrapping(PRIME_Y)),
                        zrbp + (Wrapping(!z_nmask as i64) & Wrapping(PRIME_Z)),
                        x7,
                        y7,
                        z7,
                    );
            }

            let a8 = y_aflip_mask1 + a1;
            if a8 > 0.0 {
                let x8 = x1;
                let y8 = (y_nmask | 1) as f32 + y1;
                let z8 = z1;
                value += (a8 * a8)
                    * (a8 * a8)
                    * grad3(
                        seed2,
                        xrbp + Wrapping(PRIME_X),
                        yrbp + (Wrapping(y_nmask as i64) & (Wrapping(PRIME_Y) << 1)),
                        zrbp + Wrapping(PRIME_Z),
                        x8,
                        y8,
                        z8,
                    );
                skip9 = true;
            }
        }

        let mut skip_d = false;
        let a_a = z_aflip_mask0 + a0;
        if a_a > 0.0 {
            let x_a = x0;
            let y_a = y0;
            let z_a = z0 - (z_nmask | 1) as f32;
            value += (a_a * a_a)
                * (a_a * a_a)
                * grad3(
                    seed,
                    xrbp + (Wrapping(x_nmask as i64) & Wrapping(PRIME_X)),
                    yrbp + (Wrapping(y_nmask as i64) & Wrapping(PRIME_Y)),
                    zrbp + (Wrapping(!z_nmask as i64) & Wrapping(PRIME_Z)),
                    x_a,
                    y_a,
                    z_a,
                );
        } else {
            let a_b = x_aflip_mask0 + y_aflip_mask0 + a0;
            if a_b > 0.0 {
                let x_b = x0 - (x_nmask | 1) as f32;
                let y_b = y0 - (y_nmask | 1) as f32;
                let z_b = z0;
                value += (a_b * a_b)
                    * (a_b * a_b)
                    * grad3(
                        seed,
                        xrbp + (Wrapping(!x_nmask as i64) & Wrapping(PRIME_X)),
                        yrbp + (Wrapping(!y_nmask as i64) & Wrapping(PRIME_Y)),
                        zrbp + (Wrapping(z_nmask as i64) & Wrapping(PRIME_Z)),
                        x_b,
                        y_b,
                        z_b,
                    );
            }

            let a_c = z_aflip_mask1 + a1;
            if a_c > 0.0 {
                let x_c = x1;
                let y_c = y1;
                let z_c = (z_nmask | 1) as f32 + z1;
                value += (a_c * a_c)
                    * (a_c * a_c)
                    * grad3(
                        seed2,
                        xrbp + Wrapping(PRIME_X),
                        yrbp + Wrapping(PRIME_Y),
                        zrbp + (Wrapping(z_nmask as i64) & (Wrapping(PRIME_Z) << 1)),
                        x_c,
                        y_c,
                        z_c,
                    );
                skip_d = true;
            }
        }

        if !skip5 {
            let a5 = y_aflip_mask1 + z_aflip_mask1 + a1;
            if a5 > 0.0 {
                let x5 = x1;
                let y5 = (y_nmask | 1) as f32 + y1;
                let z5 = (z_nmask | 1) as f32 + z1;
                value += (a5 * a5)
                    * (a5 * a5)
                    * grad3(
                        seed2,
                        xrbp + Wrapping(PRIME_X),
                        yrbp + (Wrapping(y_nmask as i64) & (Wrapping(PRIME_Y) << 1)),
                        zrbp + (Wrapping(z_nmask as i64) & (Wrapping(PRIME_Z) << 1)),
                        x5,
                        y5,
                        z5,
                    );
            }
        }

        if !skip9 {
            let a9 = x_aflip_mask1 + z_aflip_mask1 + a1;
            if a9 > 0.0 {
                let x9 = (x_nmask | 1) as f32 + x1;
                let y9 = y1;
                let z9 = (z_nmask | 1) as f32 + z1;
                value += (a9 * a9)
                    * (a9 * a9)
                    * grad3(
                        seed2,
                        xrbp + (Wrapping(x_nmask as i64) & (Wrapping(PRIME_X) << 1)),
                        yrbp + Wrapping(PRIME_Y),
                        zrbp + (Wrapping(z_nmask as i64) & (Wrapping(PRIME_Z) << 1)),
                        x9,
                        y9,
                        z9,
                    );
            }
        }

        if !skip_d {
            let a_d = x_aflip_mask1 + y_aflip_mask1 + a1;
            if a_d > 0.0 {
                let x_d = (x_nmask | 1) as f32 + x1;
                let y_d = (y_nmask | 1) as f32 + y1;
                let z_d = z1;
                value += (a_d * a_d)
                    * (a_d * a_d)
                    * grad3(
                        seed2,
                        xrbp + (Wrapping(x_nmask as i64) & (Wrapping(PRIME_X) << 1)),
                        yrbp + (Wrapping(y_nmask as i64) & (Wrapping(PRIME_Y) << 1)),
                        zrbp + Wrapping(PRIME_Z),
                        x_d,
                        y_d,
                        z_d,
                    );
            }
        }

        value
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample3a(&self, point: f32x4, seed: i32) -> f32 {
        self.raw_sample3(*crate::array_4_take_3(point.as_array()), seed)
    }

    #[inline]
    fn raw_sample4(&self, [xs, ys, zs, ws]: [f32; 4], seed: i32) -> f32 {
        let seed = Wrapping(seed as i64);

        // Get base points and offsets
        let xsb = fast_floor(xs);
        let ysb = fast_floor(ys);
        let zsb = fast_floor(zs);
        let wsb = fast_floor(ws);
        let xsi = (xs - xsb as f32) as f32;
        let ysi = (ys - ysb as f32) as f32;
        let zsi = (zs - zsb as f32) as f32;
        let wsi = (ws - wsb as f32) as f32;

        // Unskewed offsets
        let ssi = (xsi + ysi + zsi + wsi) * UNSKEW_4D;
        let xi = xsi + ssi;
        let yi = ysi + ssi;
        let zi = zsi + ssi;
        let wi = wsi + ssi;

        // Prime pre-multiplication for hash.
        let xsvp = Wrapping(xsb as i64) * Wrapping(PRIME_X);
        let ysvp = Wrapping(ysb as i64) * Wrapping(PRIME_Y);
        let zsvp = Wrapping(zsb as i64) * Wrapping(PRIME_Z);
        let wsvp = Wrapping(wsb as i64) * Wrapping(PRIME_W);

        // Index into initial table.
        let index = (fast_floor(xs * 4.0) & 3) | ((fast_floor(ys * 4.0) & 3) << 2) | ((fast_floor(zs * 4.0) & 3) << 4) | ((fast_floor(ws * 4.0) & 3) << 6);

        // Point contributions
        let mut value = 0.0;
        let [secondary_index_start, secondary_index_stop] = LOOKUP_4D_A[index as usize];
        for i in secondary_index_start..secondary_index_stop {
            let c = &LOOKUP_4D_B[usize::from(i)];
            let dx = xi + c.dx;
            let dy = yi + c.dy;
            let dz = zi + c.dz;
            let dw = wi + c.dw;
            let mut a = (dx * dx + dy * dy) + (dz * dz + dw * dw);
            if a < RSQUARED_4D {
                a -= RSQUARED_4D;
                a *= a;
                value += a * a * grad4(seed, xsvp + Wrapping(c.xsvp), ysvp + Wrapping(c.ysvp), zsvp + Wrapping(c.zsvp), wsvp + Wrapping(c.wsvp), dx, dy, dz, dw);
            }
        }
        value
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample4a(&self, point: f32x4, seed: i32) -> f32 {
        self.raw_sample4(point.into(), seed)
    }

    #[inline(always)]
    fn raw_improve2_x(&self, [x, y]: [f32; 2]) -> [f32; 2] {
        // Skew transform and rotation baked into one.
        let xx = x * ROOT2OVER2;
        let yy = y * (ROOT2OVER2 * (1.0 + 2.0 * SKEW_2D));

        [yy + xx, yy - xx]
    }

    #[inline(always)]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve2a_x(&self, point: f32x2) -> f32x2 {
        self.raw_improve2_x(point.into()).into()
    }

    #[doc(hidden)]
    fn raw_improve3_xy(&self, [x, y, z]: [f32; 3]) -> [f32; 3] {
        // Re-orient the cubic lattices without skewing, so Z points up the main lattice diagonal,
        // and the planes formed by XY are moved far out of alignment with the cube faces.
        // Orthonormal rotation. Not a skew transform.
        let xy = x + y;
        let s2 = xy * ROTATE3_ORTHOGONALIZER;
        let zz = z * ROOT3OVER3;
        let xr = x + s2 + zz;
        let yr = y + s2 + zz;
        let zr = xy * -ROOT3OVER3 + zz;

        // Evaluate both lattices to form a BCC lattice.
        [xr, yr, zr]
    }

    #[doc(hidden)]
    #[cfg(feature = "nightly-simd")]
    fn raw_improve3a_xy(&self, point: f32x4) -> f32x4 {
        let &[x, y, z, _] = point.as_array();
        let [x, y, z] = self.raw_improve3_xy([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }

    #[doc(hidden)]
    fn raw_improve3_xz(&self, [x, y, z]: [f32; 3]) -> [f32; 3] {
        // Re-orient the cubic lattices without skewing, so Y points up the main lattice diagonal,
        // and the planes formed by XZ are moved far out of alignment with the cube faces.
        // Orthonormal rotation. Not a skew transform.
        let xz = x + z;
        let s2 = xz * -0.211324865405187;
        let yy = y * ROOT3OVER3;
        let xr = x + s2 + yy;
        let zr = z + s2 + yy;
        let yr = xz * -ROOT3OVER3 + yy;

        // Evaluate both lattices to form a BCC lattice.
        [xr, yr, zr]
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
        let ww = w * 1.118033988749894;
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
        let ww = w * 1.118033988749894;
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
        let ww = w * 1.118033988749894;
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
        let s2 = (x + y) * -0.28522513987434876941 + (z + w) * 0.83897065470611435718;
        let t2 = (z + w) * 0.21939749883706435719 + (x + y) * -0.48214856493302476942;
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
pub(crate) fn improve2([x, y]: [f32; 2]) -> [f32; 2] {
    // Get points for A2* lattice
    let s = SKEW_2D * (x + y);
    let xs = x + s;
    let ys = y + s;
    [xs, ys]
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve2a(point: f32x2) -> f32x2 {
    improve2(point.into()).into()
}

#[inline]
pub(crate) fn improve3([x, y, z]: [f32; 3]) -> [f32; 3] {
    // Re-orient the cubic lattices via rotation, to produce a familiar look.
    // Orthonormal rotation. Not a skew transform.
    let r = FALLBACK_ROTATE3 * (x + y + z);
    let xr = r - x;
    let yr = r - y;
    let zr = r - z;

    // Evaluate both lattices to form a BCC lattice.
    [xr, yr, zr]
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
    let s = SKEW_4D * point.reduce_sum();
    point + splat(s)
}

fn grad2(seed: Wrapping<i64>, xsvp: Wrapping<i64>, ysvp: Wrapping<i64>, dx: f32, dy: f32) -> f32 {
    let mut hash = seed ^ xsvp ^ ysvp;
    hash *= HASH_MULTIPLIER;
    hash ^= hash.0 >> (64 - N_GRADS_2D_EXPONENT + 1);
    let [grad_x, grad_y] = *GRADIENTS_2D[hash.0].as_array();
    grad_x * dx + grad_y * dy
}

fn grad3(seed: Wrapping<i64>, xrvp: Wrapping<i64>, yrvp: Wrapping<i64>, zrvp: Wrapping<i64>, dx: f32, dy: f32, dz: f32) -> f32 {
    let mut hash = (seed ^ xrvp) ^ (yrvp ^ zrvp);
    hash *= HASH_MULTIPLIER;
    hash ^= hash.0 >> (64 - N_GRADS_3D_EXPONENT + 2);
    let [grad_x, grad_y, grad_z, ..] = *GRADIENTS_3D[hash.0].as_array();
    grad_x * dx + grad_y * dy + grad_z * dz
}

fn grad4(seed: Wrapping<i64>, xsvp: Wrapping<i64>, ysvp: Wrapping<i64>, zsvp: Wrapping<i64>, wsvp: Wrapping<i64>, dx: f32, dy: f32, dz: f32, dw: f32) -> f32 {
    let mut hash = seed ^ (xsvp ^ ysvp) ^ (zsvp ^ wsvp);
    hash *= HASH_MULTIPLIER;
    hash ^= hash.0 >> (64 - N_GRADS_4D_EXPONENT + 2);
    let [grad_x, grad_y, grad_z, grad_w] = *GRADIENTS_4D[hash.0].as_array();
    grad_x * dx + grad_y * dy + grad_z * dz + grad_w * dw
}

fn fast_floor(x: f32) -> i32 {
    let xi = x as i32;
    if x < xi as f32 {
        xi - 1
    } else {
        xi
    }
}

const PRIME_X: i64 = 0x5205402B9270C86F;
const PRIME_Y: i64 = 0x598CD327003817B5;
const PRIME_Z: i64 = 0x5BCC226E9FA0BACB;
const PRIME_W: i64 = 0x56CC5227E58F554B;
const HASH_MULTIPLIER: i64 = 0x53A3F72DEEC546F5;
const SEED_FLIP_3D: i64 = -0x52D547B2E96ED629;

const ROOT2OVER2: f32 = 0.7071067811865476;
const SKEW_2D: f32 = 0.366025403784439;
const UNSKEW_2D: f32 = -0.21132486540518713;

const ROOT3OVER3: f32 = 0.577350269189626;
const FALLBACK_ROTATE3: f32 = 2.0 / 3.0;
const ROTATE3_ORTHOGONALIZER: f32 = UNSKEW_2D;

const SKEW_4D: f32 = 0.309016994374947;
const UNSKEW_4D: f32 = -0.138196601125011;

const N_GRADS_2D_EXPONENT: i32 = 7;
const N_GRADS_3D_EXPONENT: i32 = 8;
const N_GRADS_4D_EXPONENT: i32 = 9;

const RSQUARED_2D: f32 = 2.0 / 3.0;
const RSQUARED_3D: f32 = 3.0 / 4.0;
const RSQUARED_4D: f32 = 4.0 / 5.0;
