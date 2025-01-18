#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

use crate::{Noise, Sample, SampleWithSeed};

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
