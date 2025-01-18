#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4, num::SimdFloat};

use crate::{open_simplex_2::impl_open_simplex_noise, OpenSimplexNoise};

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
    fn raw_sample2(&self, point: [f32; 2], seed: i32) -> f32 {
        crate::from_open_simplex_2::fast::noise2_UnskewedBase(point, seed)
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample2a(&self, point: f32x2, seed: i32) -> f32 {
        self.raw_sample2(point.into(), seed)
    }

    #[inline]
    fn raw_sample3(&self, point: [f32; 3], seed: i32) -> f32 {
        crate::from_open_simplex_2::fast::noise3_UnrotatedBase(point, seed)
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn raw_sample3a(&self, point: f32x4, seed: i32) -> f32 {
        self.raw_sample3(*crate::array_4_take_3(point.as_array()), seed)
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

const ROOT2OVER2: f32 = 0.7071067811865476;
const SKEW_2D: f32 = 0.366025403784439;
const UNSKEW_2D: f32 = -0.21132486540518713;

const ROOT3OVER3: f32 = 0.577350269189626;
const FALLBACK_ROTATE_3D: f32 = 2.0 / 3.0;
const ROTATE_3D_ORTHOGONALIZER: f32 = UNSKEW_2D;
