#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4, num::SimdFloat};

use crate::{Noise, Sample, SampleWithSeed};

#[cfg(feature = "nightly-simd")]
use crate::math::splat;

macro_rules! impl_improves {
    (
        $(#[$trait_attrs:meta])*
        trait $trait:ident {
            $($trait_members:tt)*
        }

        $(
            $(#[$improve_attrs:meta])*
            $improve_struct:ident $improve_fn:ident use $improve:ident $improve_a:ident;
        )*
    ) => {
        $(#[$trait_attrs])*
        pub trait $trait: Noise {
            $(
                $(#[$improve_attrs])*
                fn $improve_fn(self) -> $improve_struct<Self>
                where
                    Self: Sized,
                {
                    $improve_struct(self)
                }
            )*

            $($trait_members)*
        }

        $(
            $(#[$improve_attrs])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $improve_struct<OpenSimplexNoise>(pub OpenSimplexNoise);

            impl<N> Noise for $improve_struct<N> {}

            impl<N: OpenSimplexNoise> $trait for $improve_struct<N> {
                #[inline(always)]
                fn raw_sample2(&self, point: [f32; 2], seed: i32) -> f32 {
                    self.0.raw_sample2(point, seed)
                }

                #[inline(always)]
                #[cfg(feature = "nightly-simd")]
                fn raw_sample2a(&self, point: f32x2, seed: i32) -> f32 {
                    self.0.raw_sample2a(point, seed)
                }

                #[inline(always)]
                fn raw_sample3(&self, point: [f32; 3], seed: i32) -> f32 {
                    self.0.raw_sample3(point, seed)
                }

                #[inline(always)]
                #[cfg(feature = "nightly-simd")]
                fn raw_sample3a(&self, point: f32x4, seed: i32) -> f32 {
                    self.0.raw_sample3a(point, seed)
                }

                #[inline(always)]
                fn raw_sample4(&self, point: [f32; 4], seed: i32) -> f32 {
                    self.0.raw_sample4(point, seed)
                }

                #[inline(always)]
                #[cfg(feature = "nightly-simd")]
                fn raw_sample4a(&self, point: f32x4, seed: i32) -> f32 {
                    self.0.raw_sample4a(point, seed)
                }

                #[inline(always)]
                fn raw_improve3_xy(&self, point: [f32; 3]) -> [f32; 3] {
                    self.0.raw_improve3_xy(point)
                }

                #[inline(always)]
                #[cfg(feature = "nightly-simd")]
                fn raw_improve3a_xy(&self, point: f32x4) -> f32x4 {
                    self.0.raw_improve3a_xy(point)
                }

                #[inline(always)]
                fn raw_improve3_xz(&self, point: [f32; 3]) -> [f32; 3] {
                    self.0.raw_improve3_xz(point)
                }

                #[inline(always)]
                #[cfg(feature = "nightly-simd")]
                fn raw_improve3a_xz(&self, point: f32x4) -> f32x4 {
                    self.0.raw_improve3a_xz(point)
                }
            }

            impl<N: Sample<2>> Sample<2> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    self.0.sample(point)
                }
            }

            impl<N: SampleWithSeed<2>> SampleWithSeed<2> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 2], seed: i32) -> f32 {
                    self.0.sample_with_seed(point, seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: Sample<2, f32x2>> Sample<2, f32x2> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: f32x2) -> f32 {
                    self.0.sample(point)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: SampleWithSeed<2, f32x2>> SampleWithSeed<2, core::simd::f32x2> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: f32x2, seed: i32) -> f32 {
                    self.0.sample_with_seed(point, seed)
                }
            }

            impl<N: OpenSimplexNoise> Sample<3> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    self.0.raw_sample3(self.0.$improve(point), 0)
                }
            }

            impl<N: OpenSimplexNoise> SampleWithSeed<3> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
                    self.0.raw_sample3(self.0.$improve(point), seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: OpenSimplexNoise> Sample<3, f32x4> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: f32x4) -> f32 {
                    self.0.raw_sample3a(self.0.$improve_a(point), 0)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: OpenSimplexNoise> SampleWithSeed<3, f32x4> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
                    self.0.raw_sample3a(self.0.$improve_a(point), seed)
                }
            }

            impl<N: Sample<4>> Sample<4> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    self.0.sample(point)
                }
            }

            impl<N: SampleWithSeed<4>> SampleWithSeed<4> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 4], seed: i32) -> f32 {
                    self.0.sample_with_seed(point, seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: Sample<4, f32x4>> Sample<4, f32x4> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: f32x4) -> f32 {
                    self.0.sample(point)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: SampleWithSeed<4, f32x4>> SampleWithSeed<4, f32x4> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
                    self.0.sample_with_seed(point, seed)
                }
            }
        )*
    };
}

impl_improves! {
    /// Provides modifier methods for `OpenSimplex` noises.
    trait OpenSimplexNoise {
        /// Sample this OpenSimplexNoise unskewed.
        #[doc(hidden)]
        fn raw_sample2(&self, point: [f32; 2], seed: i32) -> f32;

        /// Sample this OpenSimplexNoise unskewed.
        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_sample2a(&self, point: f32x2, seed: i32) -> f32;

        /// Sample this OpenSimplexNoise unrotated.
        #[doc(hidden)]
        fn raw_sample3(&self, point: [f32; 3], seed: i32) -> f32;

        /// Sample this OpenSimplexNoise unrotated.
        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_sample3a(&self, point: f32x4, seed: i32) -> f32;

        /// Sample this OpenSimplexNoise unskewed.
        #[doc(hidden)]
        fn raw_sample4(&self, point: [f32; 4], seed: i32) -> f32;

        /// Sample this OpenSimplexNoise unskewed.
        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_sample4a(&self, point: f32x4, seed: i32) -> f32;

        #[doc(hidden)]
        fn raw_improve3_xy(&self, point: [f32; 3]) -> [f32; 3];

        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_improve3a_xy(&self, point: f32x4) -> f32x4;

        #[doc(hidden)]
        fn raw_improve3_xz(&self, point: [f32; 3]) -> [f32; 3];

        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_improve3a_xz(&self, point: f32x4) -> f32x4;
    }

    /// Improves 3D orientation for the `XY` plane.
    ImproveXy improve_xy use raw_improve3_xy raw_improve3a_xy;

    /// Improves 3D orientation for the `XZ` plane.
    ImproveXz improve_xz use raw_improve3_xz raw_improve3a_xz;
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

#[inline]
pub(crate) fn improve4_smooth([mut x, mut y, mut z, mut w]: [f32; 4]) -> [f32; 4] {
    const SKEW_4D: f32 = 0.309016994374947;
    let s = SKEW_4D * (x + y + z + w);
    x += s;
    y += s;
    z += s;
    w += s;
    [x, y, z, w]
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve4a_smooth(point: f32x4) -> f32x4 {
    const SKEW_4D: f32 = 0.309016994374947;
    let s = SKEW_4D * point.reduce_sum();
    point + splat(s)
}
