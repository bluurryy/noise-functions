use core::num::Wrapping;
#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4, num::SimdFloat};

use crate::{
    from_open_simplex_2::fast_luts::{GRADIENTS_2D, GRADIENTS_3D, GRADIENTS_4D},
    open_simplex_2::impl_open_simplex_noise,
    OpenSimplexNoise,
};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

/// 2/3/4 dimensional OpenSimplex2 noise. Fast variant.
///
/// You can improve the visual isotropy for certain orientations using the `improve_*` methods
/// provided by the [`OpenSimplexNoise`] trait.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSimplex2;

impl_open_simplex_noise!(234 OpenSimplex2);

impl OpenSimplexNoise for OpenSimplex2 {
    #[inline]
    fn raw_sample2(&self, [xs, ys]: [f32; 2], seed: i32) -> f32 {
        let seed = Wrapping(seed as i64);

        // Get base points and offsets.
        let xsb = fast_floor(xs);
        let ysb = fast_floor(ys);
        let xi = (xs - xsb as f32) as f32;
        let yi = (ys - ysb as f32) as f32;

        // Prime pre-multiplication for hash.
        let xsbp = Wrapping(xsb as i64) * Wrapping(PRIME_X);
        let ysbp = Wrapping(ysb as i64) * Wrapping(PRIME_Y);

        // Unskew.
        let t = (xi + yi) * UNSKEW_2D;
        let dx0 = xi + t;
        let dy0 = yi + t;

        // First vertex.
        let mut value = 0.0;
        let a0 = RSQUARED_2D - dx0 * dx0 - dy0 * dy0;
        if a0 > 0.0 {
            value = (a0 * a0) * (a0 * a0) * grad2(seed, xsbp, ysbp, dx0, dy0);
        }

        // Second vertex.
        let a1 = (2.0 * (1.0 + 2.0 * UNSKEW_2D) * (1.0 / UNSKEW_2D + 2.0)) * t + ((-2.0 * (1.0 + 2.0 * UNSKEW_2D) * (1.0 + 2.0 * UNSKEW_2D)) + a0);
        if a1 > 0.0 {
            let dx1 = dx0 - (1.0 + 2.0 * UNSKEW_2D);
            let dy1 = dy0 - (1.0 + 2.0 * UNSKEW_2D);
            value += (a1 * a1) * (a1 * a1) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp + Wrapping(PRIME_Y), dx1, dy1);
        }

        // Third vertex.
        if dy0 > dx0 {
            let dx2 = dx0 - UNSKEW_2D;
            let dy2 = dy0 - (UNSKEW_2D + 1.0);
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp + Wrapping(PRIME_Y), dx2, dy2);
            }
        } else {
            let dx2 = dx0 - (UNSKEW_2D + 1.0);
            let dy2 = dy0 - UNSKEW_2D;
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp, dx2, dy2);
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
        let mut seed = Wrapping(seed as i64);

        // Get base points and offsets.
        let xrb = fast_round(xr);
        let yrb = fast_round(yr);
        let zrb = fast_round(zr);
        let mut xri = (xr - xrb as f32) as f32;
        let mut yri = (yr - yrb as f32) as f32;
        let mut zri = (zr - zrb as f32) as f32;

        // -1 if positive, 1 if negative.
        let mut x_nsign = (-1.0 - xri) as i32 | 1;
        let mut y_nsign = (-1.0 - yri) as i32 | 1;
        let mut z_nsign = (-1.0 - zri) as i32 | 1;

        // Compute absolute values, using the above as a shortcut. This was faster in my tests for some reason.
        let mut ax0 = x_nsign as f32 * -xri;
        let mut ay0 = y_nsign as f32 * -yri;
        let mut az0 = z_nsign as f32 * -zri;

        // Prime pre-multiplication for hash.
        let mut xrbp = Wrapping(xrb as i64) * Wrapping(PRIME_X);
        let mut yrbp = Wrapping(yrb as i64) * Wrapping(PRIME_Y);
        let mut zrbp = Wrapping(zrb as i64) * Wrapping(PRIME_Z);

        // Loop: Pick an edge on each lattice copy.
        let mut value = 0.0;
        let mut a = (RSQUARED_3D - xri * xri) - (yri * yri + zri * zri);
        for l in 0.. {
            // Closest point on cube.
            if a > 0.0 {
                value += (a * a) * (a * a) * grad3(seed, xrbp, yrbp, zrbp, xri, yri, zri);
            }

            // Second-closest point.
            if ax0 >= ay0 && ax0 >= az0 {
                let mut b = a + ax0 + ax0;
                if b > 1.0 {
                    b -= 1.0;
                    value += (b * b) * (b * b) * grad3(seed, xrbp - Wrapping(x_nsign as i64) * Wrapping(PRIME_X), yrbp, zrbp, xri + x_nsign as f32, yri, zri);
                }
            } else if ay0 > ax0 && ay0 >= az0 {
                let mut b = a + ay0 + ay0;
                if b > 1.0 {
                    b -= 1.0;
                    value += (b * b) * (b * b) * grad3(seed, xrbp, yrbp - Wrapping(y_nsign as i64) * Wrapping(PRIME_Y), zrbp, xri, yri + y_nsign as f32, zri);
                }
            } else {
                let mut b = a + az0 + az0;
                if b > 1.0 {
                    b -= 1.0;
                    value += (b * b) * (b * b) * grad3(seed, xrbp, yrbp, zrbp - Wrapping(z_nsign as i64) * Wrapping(PRIME_Z), xri, yri, zri + z_nsign as f32);
                }
            }

            // Break from loop if we're done, skipping updates below.
            if l == 1 {
                break;
            }

            // Update absolute value.
            ax0 = 0.5 - ax0;
            ay0 = 0.5 - ay0;
            az0 = 0.5 - az0;

            // Update relative coordinate.
            xri = x_nsign as f32 * ax0;
            yri = y_nsign as f32 * ay0;
            zri = z_nsign as f32 * az0;

            // Update falloff.
            a += (0.75 - ax0) - (ay0 + az0);

            // Update prime for hash.
            xrbp += (x_nsign as i64 >> 1) & PRIME_X;
            yrbp += (y_nsign as i64 >> 1) & PRIME_Y;
            zrbp += (z_nsign as i64 >> 1) & PRIME_Z;

            // Update the reverse sign indicators.
            x_nsign = -x_nsign;
            y_nsign = -y_nsign;
            z_nsign = -z_nsign;

            // And finally update the seed for the other lattice copy.
            seed ^= SEED_FLIP_3D;
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
        let mut seed = Wrapping(seed as i64);

        // Get base points and offsets
        let xsb = fast_floor(xs);
        let ysb = fast_floor(ys);
        let zsb = fast_floor(zs);
        let wsb = fast_floor(ws);
        let mut xsi = (xs - xsb as f32) as f32;
        let mut ysi = (ys - ysb as f32) as f32;
        let mut zsi = (zs - zsb as f32) as f32;
        let mut wsi = (ws - wsb as f32) as f32;

        // Determine which lattice we can be confident has a contributing point its corresponding cell's base simplex.
        // We only look at the spaces between the diagonal planes. This proved effective in all of my tests.
        let si_sum = (xsi + ysi) + (zsi + wsi);
        let starting_lattice = (si_sum * 1.25) as i32;

        // Offset for seed based on first lattice copy.
        seed += Wrapping(starting_lattice as i64) * Wrapping(SEED_OFFSET_4D);

        // Offset for lattice point relative positions (skewed)
        let starting_lattice_offset = starting_lattice as f32 * -LATTICE_STEP_4D;
        xsi += starting_lattice_offset;
        ysi += starting_lattice_offset;
        zsi += starting_lattice_offset;
        wsi += starting_lattice_offset;

        // Prep for vertex contributions.
        let mut ssi = (si_sum + starting_lattice_offset * 4.0) * UNSKEW_4D;

        // Prime pre-multiplication for hash.
        let mut xsvp = Wrapping(xsb as i64) * Wrapping(PRIME_X);
        let mut ysvp = Wrapping(ysb as i64) * Wrapping(PRIME_Y);
        let mut zsvp = Wrapping(zsb as i64) * Wrapping(PRIME_Z);
        let mut wsvp = Wrapping(wsb as i64) * Wrapping(PRIME_W);

        // Five points to add, total, from five copies of the A4 lattice.
        let mut value = 0.0;
        for i in 0.. {
            // Next point is the closest vertex on the 4-simplex whose base vertex is the aforementioned vertex.
            let score0 = 1.0 + ssi * (-1.0 / UNSKEW_4D); // Seems slightly faster than 1.0-xsi-ysi-zsi-wsi
            if xsi >= ysi && xsi >= zsi && xsi >= wsi && xsi >= score0 {
                xsvp += PRIME_X;
                xsi -= 1.0;
                ssi -= UNSKEW_4D;
            } else if ysi > xsi && ysi >= zsi && ysi >= wsi && ysi >= score0 {
                ysvp += PRIME_Y;
                ysi -= 1.0;
                ssi -= UNSKEW_4D;
            } else if zsi > xsi && zsi > ysi && zsi >= wsi && zsi >= score0 {
                zsvp += PRIME_Z;
                zsi -= 1.0;
                ssi -= UNSKEW_4D;
            } else if wsi > xsi && wsi > ysi && wsi > zsi && wsi >= score0 {
                wsvp += PRIME_W;
                wsi -= 1.0;
                ssi -= UNSKEW_4D;
            }

            // gradient contribution with falloff.
            let dx = xsi + ssi;
            let dy = ysi + ssi;
            let dz = zsi + ssi;
            let dw = wsi + ssi;
            let mut a = (dx * dx + dy * dy) + (dz * dz + dw * dw);
            if a < RSQUARED_4D {
                a -= RSQUARED_4D;
                a *= a;
                value += a * a * grad4(seed, xsvp, ysvp, zsvp, wsvp, dx, dy, dz, dw);
            }

            // Break from loop if we're done, skipping updates below.
            if i == 4 {
                break;
            }

            // Update for next lattice copy shifted down by <-0.2, -0.2, -0.2, -0.2>.
            xsi += LATTICE_STEP_4D;
            ysi += LATTICE_STEP_4D;
            zsi += LATTICE_STEP_4D;
            wsi += LATTICE_STEP_4D;
            ssi += LATTICE_STEP_4D * 4.0 * UNSKEW_4D;
            seed -= SEED_OFFSET_4D;

            // Because we don't always start on the same lattice copy, there's a special reset case.
            if i == starting_lattice {
                xsvp -= PRIME_X;
                ysvp -= PRIME_Y;
                zsvp -= PRIME_Z;
                wsvp -= PRIME_W;
                seed += SEED_OFFSET_4D * 5;
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
        let s2 = xy * ROTATE_3D_ORTHOGONALIZER;
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
        let s2 = xz * ROTATE_3D_ORTHOGONALIZER;
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
    let r = FALLBACK_ROTATE_3D * (x + y + z);
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

fn fast_round(x: f32) -> i32 {
    if x < 0.0 {
        (x - 0.5) as i32
    } else {
        (x + 0.5) as i32
    }
}

const PRIME_X: i64 = 0x5205402B9270C86F;
const PRIME_Y: i64 = 0x598CD327003817B5;
const PRIME_Z: i64 = 0x5BCC226E9FA0BACB;
const PRIME_W: i64 = 0x56CC5227E58F554B;
const HASH_MULTIPLIER: i64 = 0x53A3F72DEEC546F5;
const SEED_FLIP_3D: i64 = -0x52D547B2E96ED629;
const SEED_OFFSET_4D: i64 = 0xE83DC3E0DA7164D;

const ROOT2OVER2: f32 = 0.7071067811865476;
const SKEW_2D: f32 = 0.366025403784439;
const UNSKEW_2D: f32 = -0.21132486540518713;

const ROOT3OVER3: f32 = 0.577350269189626;
const FALLBACK_ROTATE_3D: f32 = 2.0 / 3.0;
const ROTATE_3D_ORTHOGONALIZER: f32 = UNSKEW_2D;

const SKEW_4D: f32 = -0.138196601125011;
const UNSKEW_4D: f32 = 0.309016994374947;
const LATTICE_STEP_4D: f32 = 0.2;

const N_GRADS_2D_EXPONENT: i32 = 7;
const N_GRADS_3D_EXPONENT: i32 = 8;
const N_GRADS_4D_EXPONENT: i32 = 9;

const RSQUARED_2D: f32 = 0.5;
const RSQUARED_3D: f32 = 0.6;
const RSQUARED_4D: f32 = 0.6;
