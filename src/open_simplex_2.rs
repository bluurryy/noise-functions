use crate::private_prelude::*;

macro_rules! impl_fallback {
    ($noise:ident in $noise_mod:ident) => {
        impl Sample<2> for $noise {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, 0)
            }
        }

        impl Sample<2> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, self.seed)
            }
        }

        impl Sample<2> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, self.seed)
            }
        }

        impl Sample<3> for $noise {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(scalar_improve::improve3(point), 0)
            }
        }

        impl Sample<3> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(scalar_improve::improve3(point), self.seed)
            }
        }

        impl Sample<3> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(scalar_improve::improve3(point), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for $noise {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for $noise {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(simd_improve::improve3(point), 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(simd_improve::improve3(point), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(simd_improve::improve3(point), self.seed)
            }
        }
    };
}

macro_rules! impl_improve {
    ($improve_fn:ident as $improve:ident for $noise:ident in $noise_mod:ident) => {
        impl Sample<2> for $improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, 0)
            }
        }

        impl Sample<2> for Seeded<$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, self.seed)
            }
        }

        impl Sample<2> for Seeded<&$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, self.seed)
            }
        }

        impl Sample<3> for $improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(scalar_improve::$improve_fn(point), 0)
            }
        }

        impl Sample<3> for Seeded<$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(scalar_improve::$improve_fn(point), self.seed)
            }
        }

        impl Sample<3> for Seeded<&$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(scalar_improve::$improve_fn(point), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for $improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<&$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for $improve<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(simd_improve::$improve_fn(point), 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(simd_improve::$improve_fn(point), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<&$improve<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(simd_improve::$improve_fn(point), self.seed)
            }
        }
    };
}

macro_rules! improve_wrapper {
    (
        $(#[$attr:meta])*
        $name:ident
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<OpenSimplexNoise>(pub OpenSimplexNoise);

        impl<OpenSimplexNoise> $name<OpenSimplexNoise> {
            impl_modifiers!();
        }
    };
}

macro_rules! base {
    (
        $(#[$attr:meta])*
        $noise:ident in $noise_mod:ident
    ) => {
        $(#[$attr])*
        ///
        /// When sampling in 3 Dimensions you can improve the visual isotropy in a the respective planes via [`improve_xy`] or [`improve_xz`].
        ///
        /// [`improve_xy`]: Self::improve_xy
        /// [`improve_xz`]: Self::improve_xz
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $noise;

        impl $noise {
            /// Improves 3D orientation for the `XY` plane.
            pub const fn improve_xy(self) -> ImproveXy<Self> {
                ImproveXy(self)
            }

            /// Improves 3D orientation for the `XZ` plane.
            pub const fn improve_xz(self) -> ImproveXz<Self> {
                ImproveXz(self)
            }
        }

        impl $noise {
            impl_modifiers!();
        }

        impl_fallback! {
            $noise in $noise_mod
        }

        impl_improve! {
            improve3_xy as ImproveXy for $noise in $noise_mod
        }

        impl_improve! {
            improve3_xz as ImproveXz for $noise in $noise_mod
        }
    };
}

base! {
    /// 2/3 dimensional OpenSimplex2 noise. Fast variant.
    OpenSimplex2 in open_simplex_2
}

base! {
    /// 2/3 dimensional OpenSimplex2s noise. Smooth variant.
    OpenSimplex2s in open_simplex_2s
}

improve_wrapper! {
    /// Improves 3D orientation for the `XY` plane.
    ImproveXy
}

improve_wrapper! {
    /// Improves 3D orientation for the `XZ` plane.
    ImproveXz
}

mod scalar_improve {
    #[inline]
    pub fn improve3([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        const R3: f32 = 2.0 / 3.0;
        let r: f32 = (x + y + z) * R3; // Rotation, not skew
        x = r - x;
        y = r - y;
        z = r - z;
        [x, y, z]
    }

    #[inline]
    pub fn improve3_xy([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        let xy: f32 = x + y;
        let s2: f32 = xy * -0.211324865405187;
        z *= 0.577350269189626;
        x += s2 - z;
        y = y + s2 - z;
        z += xy * 0.577350269189626;
        [x, y, z]
    }

    #[inline]
    pub fn improve3_xz([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        let xz: f32 = x + z;
        let s2: f32 = xz * -0.211324865405187;
        y *= 0.577350269189626;
        x += s2 - y;
        z += s2 - y;
        y += xz * 0.577350269189626;
        [x, y, z]
    }
}

#[cfg(feature = "nightly-simd")]
mod simd_improve {
    use super::*;

    #[inline]
    pub fn improve3(point: f32x4) -> f32x4 {
        const R3: f32 = 2.0 / 3.0;
        let r: f32 = (point[0] + point[1] + point[2]) * R3; // Rotation, not skew
        splat(r) - point
    }

    #[inline]
    pub fn improve3_xy(point: f32x4) -> f32x4 {
        let &[x, y, z, _] = point.as_array();
        let [x, y, z] = super::scalar_improve::improve3_xy([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }

    #[inline]
    pub fn improve3_xz(point: f32x4) -> f32x4 {
        let &[x, y, z, _] = point.as_array();
        let [x, y, z] = super::scalar_improve::improve3_xz([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }
}
