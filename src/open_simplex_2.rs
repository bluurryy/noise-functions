#[cfg(feature = "nightly-simd")]
use core::simd::f32x4;

macro_rules! impl_open_simplex_2 {
    ($noise:ident) => {
        impl $noise {
            $crate::open_simplex_2::impl_open_simplex_2_modifiers!();
        }

        $crate::open_simplex_2::impl_improve! {
            improve3_xy | improve3a_xy as ImproveXy for $noise
        }

        $crate::open_simplex_2::impl_improve! {
            improve3_xz | improve3a_xz as ImproveXz for $noise
        }
    };
}

pub(crate) use impl_open_simplex_2;

macro_rules! impl_open_simplex_2_modifiers {
    () => {
        /// Improves 3D orientation for the `XY` plane.
        pub const fn improve_xy(self) -> $crate::open_simplex_2::ImproveXy<Self> {
            $crate::open_simplex_2::ImproveXy(self)
        }

        /// Improves 3D orientation for the `XZ` plane.
        pub const fn improve_xz(self) -> $crate::open_simplex_2::ImproveXz<Self> {
            $crate::open_simplex_2::ImproveXz(self)
        }
    };
}

pub(crate) use impl_open_simplex_2_modifiers;

macro_rules! impl_improve {
    ($improve_fn:ident | $improve_simd_fn:ident as $improve:ident for $noise:ident) => {
        impl Sample<2> for $crate::open_simplex_2::$improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                self.0.gen2(point, 0)
            }
        }

        impl Sample<2> for Seeded<$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                self.noise.0.gen2(point, self.seed)
            }
        }

        impl Sample<2> for Seeded<&$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                self.noise.0.gen2(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for $crate::open_simplex_2::$improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                self.0.gen2a(point, 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                self.noise.0.gen2a(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<&$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                self.noise.0.gen2a(point, self.seed)
            }
        }

        impl Sample<3> for $crate::open_simplex_2::$improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                self.0.gen3($crate::open_simplex_2::$improve_fn(point), 0)
            }
        }

        impl Sample<3> for Seeded<$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                self.noise.0.gen3($crate::open_simplex_2::$improve_fn(point), self.seed)
            }
        }

        impl Sample<3> for Seeded<&$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                self.noise.0.gen3($crate::open_simplex_2::$improve_fn(point), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for $crate::open_simplex_2::$improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                self.0.gen3a($crate::open_simplex_2::$improve_simd_fn(point), 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                self.noise.0.gen3a($crate::open_simplex_2::$improve_simd_fn(point), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<&$crate::open_simplex_2::$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                self.noise.0.gen3a($crate::open_simplex_2::$improve_simd_fn(point), self.seed)
            }
        }
    };
}

pub(crate) use impl_improve;

use crate::impl_modifier_methods;

/// Improves 3D orientation for the `XY` plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImproveXy<OpenSimplexNoise>(pub OpenSimplexNoise);

/// Improves 3D orientation for the `XY` plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImproveXz<OpenSimplexNoise>(pub OpenSimplexNoise);

impl<OpenSimplexNoise> ImproveXy<OpenSimplexNoise> {
    impl_modifier_methods!();
}

impl<OpenSimplexNoise> ImproveXz<OpenSimplexNoise> {
    impl_modifier_methods!();
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
pub(crate) fn improve3_xy([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    let xy: f32 = x + y;
    let s2: f32 = xy * -0.211324865405187;
    z *= 0.577350269189626;
    x += s2 - z;
    y = y + s2 - z;
    z += xy * 0.577350269189626;
    [x, y, z]
}

#[inline]
pub(crate) fn improve3_xz([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    let xz: f32 = x + z;
    let s2: f32 = xz * -0.211324865405187;
    y *= 0.577350269189626;
    x += s2 - y;
    z += s2 - y;
    y += xz * 0.577350269189626;
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
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve3a_xy(point: f32x4) -> f32x4 {
    let &[x, y, z, _] = point.as_array();
    let [x, y, z] = improve3_xy([x, y, z]);
    f32x4::from_array([x, y, z, z])
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve3a_xz(point: f32x4) -> f32x4 {
    let &[x, y, z, _] = point.as_array();
    let [x, y, z] = improve3_xz([x, y, z]);
    f32x4::from_array([x, y, z, z])
}
