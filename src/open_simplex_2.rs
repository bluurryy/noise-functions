#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[cfg(feature = "nightly-simd")]
use crate::Sample3a;

use crate::{Sample, Sample2, Sample2a, Seeded};

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

macro_rules! impl_open_simplex_2_modifiers {
    () => {
        /// Improves 3D orientation for the `XY` plane.
        pub const fn improve_xy(self) -> ImproveXy<Self> {
            ImproveXy(self)
        }

        /// Improves 3D orientation for the `XZ` plane.
        pub const fn improve_xz(self) -> ImproveXz<Self> {
            ImproveXz(self)
        }
    };
}

/// Improves 3D orientation for the `XY` plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImproveXy<OpenSimplexNoise>(pub OpenSimplexNoise);

/// Improves 3D orientation for the `XY` plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImproveXz<OpenSimplexNoise>(pub OpenSimplexNoise);

impl<OpenSimplexNoise: Sample<2>> Sample<2> for ImproveXy<OpenSimplexNoise> {
    #[inline(always)]
    fn sample(&self, point: [f32; 2]) -> f32 {
        self.0.sample2(point)
    }
}

impl<OpenSimplexNoise> Sample<2> for Seeded<ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: [f32; 2]) -> f32 {
        // self.noise.0.seed(self.seed).sample2(point)
        todo!()
    }
}

impl<OpenSimplexNoise> Sample<2> for Seeded<&ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: [f32; 2]) -> f32 {
        // self.noise.0.seed(self.seed).sample2(point)
        todo!()
    }
}

impl<OpenSimplexNoise> Sample<3> for ImproveXy<OpenSimplexNoise> {
    #[inline(always)]
    fn sample(&self, point: [f32; 3]) -> f32 {
        let point = improve3_xy(point);
        // self.0.sample3(point)
        todo!()
    }
}

impl<OpenSimplexNoise> Sample<3> for Seeded<ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: [f32; 3]) -> f32 {
        let point = improve3_xy(point);
        // self.noise.0.seed(self.seed).sample3(point)
        todo!()
    }
}

impl<OpenSimplexNoise> Sample<3> for Seeded<&ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: [f32; 3]) -> f32 {
        let point = improve3_xy(point);
        // self.noise.0.seed(self.seed).sample3(point)
        todo!()
    }
}

#[cfg(feature = "nightly-simd")]
impl<OpenSimplexNoise: Sample<2, f32x2>> Sample<2, f32x2> for ImproveXy<OpenSimplexNoise> {
    #[inline(always)]
    fn sample(&self, point: f32x2) -> f32 {
        // self.0.sample2a(point)
        todo!()
    }
}

#[cfg(feature = "nightly-simd")]
impl<OpenSimplexNoise: Sample<2, f32x2> + Copy> Sample<2, f32x2> for Seeded<ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: f32x2) -> f32 {
        // self.noise.0.seed(self.seed).sample2a(point)
        todo!()
    }
}

#[cfg(feature = "nightly-simd")]
impl<OpenSimplexNoise> Sample<2, f32x2> for Seeded<&ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: f32x2) -> f32 {
        // self.noise.0.seed(self.seed).sample2a(point)
        todo!()
    }
}

#[cfg(feature = "nightly-simd")]
impl<OpenSimplexNoise: Sample<3, f32x4>> Sample<3, f32x4> for ImproveXy<OpenSimplexNoise> {
    #[inline(always)]
    fn sample(&self, point: f32x4) -> f32 {
        let point = improve3a_xy(point);
        self.0.sample3a(point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<OpenSimplexNoise> Sample<3, f32x4> for Seeded<ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: f32x4) -> f32 {
        let point = improve3a_xy(point);
        // self.noise.0.seed(self.seed).sample3a(point)
        todo!()
    }
}

#[cfg(feature = "nightly-simd")]
impl<OpenSimplexNoise> Sample<3, f32x4> for Seeded<&ImproveXy<OpenSimplexNoise>> {
    #[inline(always)]
    fn sample(&self, point: f32x4) -> f32 {
        let point = improve3a_xy(point);
        // self.noise.0.seed(self.seed).sample3a(point)
        todo!()
    }
}

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

#[inline]
#[cfg(feature = "nightly-simd")]
pub fn improve3a(point: f32x4) -> f32x4 {
    const R3: f32 = 2.0 / 3.0;
    let r: f32 = (point[0] + point[1] + point[2]) * R3; // Rotation, not skew
    f32x4::splat(r) - point
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub fn improve3a_xy(point: f32x4) -> f32x4 {
    let &[x, y, z, _] = point.as_array();
    let [x, y, z] = improve3_xy([x, y, z]);
    f32x4::from_array([x, y, z, z])
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub fn improve3a_xz(point: f32x4) -> f32x4 {
    let &[x, y, z, _] = point.as_array();
    let [x, y, z] = improve3_xz([x, y, z]);
    f32x4::from_array([x, y, z, z])
}
