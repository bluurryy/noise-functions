#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

use crate::{Noise, Sample, SampleWithSeed};

pub(crate) trait Sealed {}

macro_rules! r#if {
    (if 3 == 3 { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (if 3 == 4 { $($then:tt)* } else { $($else:tt)* }) => { $($else)* };
    (if 4 == 3 { $($then:tt)* } else { $($else:tt)* }) => { $($else)* };
    (if 4 == 4 { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
}

pub(crate) use r#if;

macro_rules! impl_improves {
    (
        $(#[$trait_attrs:meta])*
        trait $trait:ident {
            $($trait_members:tt)*
        }

        $(
            $(#[$improve_attrs:meta])*
            $improve_struct:ident $improve_dim:tt $improve_fn:ident use $improve:ident $improve_a:ident;
        )*
    ) => {
        $(#[$trait_attrs])*
        #[expect(private_bounds)]
        pub trait $trait: Sealed + Noise {
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

            impl<N: OpenSimplexNoise> Sealed for $improve_struct<N> {}

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

                #[doc(hidden)]
                fn raw_improve4_xyz_xy(&self, point: [f32; 4]) -> [f32; 4] {
                    self.0.raw_improve4_xyz_xy(point)
                }

                #[doc(hidden)]
                #[cfg(feature = "nightly-simd")]
                fn raw_improve4a_xyz_xy(&self, point: f32x4) -> f32x4 {
                    self.0.raw_improve4a_xyz_xy(point)
                }

                #[doc(hidden)]
                fn raw_improve4_xyz_xz(&self, point: [f32; 4]) -> [f32; 4] {
                    self.0.raw_improve4_xyz_xz(point)
                }

                #[doc(hidden)]
                #[cfg(feature = "nightly-simd")]
                fn raw_improve4a_xyz_xz(&self, point: f32x4) -> f32x4 {
                    self.0.raw_improve4a_xyz_xz(point)
                }

                #[doc(hidden)]
                fn raw_improve4_xyz(&self, point: [f32; 4]) -> [f32; 4] {
                    self.0.raw_improve4_xyz(point)
                }

                #[doc(hidden)]
                #[cfg(feature = "nightly-simd")]
                fn raw_improve4a_xyz(&self, point: f32x4) -> f32x4 {
                    self.0.raw_improve4a_xyz(point)
                }

                #[doc(hidden)]
                fn raw_improve4_xy_zw(&self, point: [f32; 4]) -> [f32; 4] {
                    self.0.raw_improve4_xy_zw(point)
                }

                #[doc(hidden)]
                #[cfg(feature = "nightly-simd")]
                fn raw_improve4a_xy_zw(&self, point: f32x4) -> f32x4 {
                    self.0.raw_improve4a_xy_zw(point)
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

            $crate::open_simplex_2::r#if! {
                if $improve_dim == 3 {
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
                } else {
                    impl<N: Sample<3>> Sample<3> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample(&self, point: [f32; 3]) -> f32 {
                            self.0.sample(point)
                        }
                    }

                    impl<N: SampleWithSeed<3>> SampleWithSeed<3> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
                            self.0.sample_with_seed(point, seed)
                        }
                    }

                    #[cfg(feature = "nightly-simd")]
                    impl<N: Sample<3, f32x4>> Sample<3, f32x4> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample(&self, point: f32x4) -> f32 {
                            self.0.sample(point)
                        }
                    }

                    #[cfg(feature = "nightly-simd")]
                    impl<N: SampleWithSeed<3, f32x4>> SampleWithSeed<3, f32x4> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
                            self.0.sample_with_seed(point, seed)
                        }
                    }
                }
            }

            $crate::open_simplex_2::r#if! {
                if $improve_dim == 4 {
                    impl<N: OpenSimplexNoise> Sample<4> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample(&self, point: [f32; 4]) -> f32 {
                            self.0.raw_sample4(self.0.$improve(point), 0)
                        }
                    }

                    impl<N: OpenSimplexNoise> SampleWithSeed<4> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample_with_seed(&self, point: [f32; 4], seed: i32) -> f32 {
                            self.0.raw_sample4(self.0.$improve(point), seed)
                        }
                    }

                    #[cfg(feature = "nightly-simd")]
                    impl<N: OpenSimplexNoise> Sample<4, f32x4> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample(&self, point: f32x4) -> f32 {
                            self.0.raw_sample4a(self.0.$improve_a(point), 0)
                        }
                    }

                    #[cfg(feature = "nightly-simd")]
                    impl<N: OpenSimplexNoise> SampleWithSeed<4, f32x4> for $improve_struct<N> {
                        #[inline(always)]
                        fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
                            self.0.raw_sample4a(self.0.$improve_a(point), seed)
                        }
                    }
                } else {
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

        #[doc(hidden)]
        fn raw_improve4_xyz(&self, point: [f32; 4]) -> [f32; 4];

        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_improve4a_xyz(&self, point: f32x4) -> f32x4;

        #[doc(hidden)]
        fn raw_improve4_xyz_xy(&self, point: [f32; 4]) -> [f32; 4];

        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_improve4a_xyz_xy(&self, point: f32x4) -> f32x4;

        #[doc(hidden)]
        fn raw_improve4_xyz_xz(&self, point: [f32; 4]) -> [f32; 4];

        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_improve4a_xyz_xz(&self, point: f32x4) -> f32x4;

        #[doc(hidden)]
        fn raw_improve4_xy_zw(&self, point: [f32; 4]) -> [f32; 4];

        #[doc(hidden)]
        #[cfg(feature = "nightly-simd")]
        fn raw_improve4a_xy_zw(&self, point: f32x4) -> f32x4;
    }

    /// Improves 3D orientation for the `XY` plane.
    ImproveXy 3 improve_xy use raw_improve3_xy raw_improve3a_xy;

    /// Improves 3D orientation for the `XZ` plane.
    ImproveXz 3 improve_xz use raw_improve3_xz raw_improve3a_xz;

    /// Improves 4D orientation so XYZ is oriented like the default 3D noise
    /// and W for an extra degree of freedom. W repeats eventually.
    ///
    /// Recommended for time-varied animations which texture a 3D object with W as time
    /// where there isn't a clear distinction between horizontal and vertical.
    ImproveXyz 4 improve_xyz use raw_improve4_xyz raw_improve4a_xyz;

    /// Improves 4D orientation so XYZ is oriented like the 3D `improve_xy` noise
    /// and W for an extra degree of freedom. W repeats eventually.
    ///
    /// Recommended for time-varied animations which texture a 3D object with W as time
    /// in a space where Z is vertical.
    ImproveXyzXy 4 improve_xyz_xy use raw_improve4_xyz_xy raw_improve4a_xyz_xy;

    /// Improves 4D orientation so XYZ is oriented like the 3D `improve_xz` noise
    /// and W for an extra degree of freedom. W repeats eventually.
    ///
    /// Recommended for time-varied animations which texture a 3D object with W as time
    /// in a space where Y is vertical.
    ImproveXyzXz 4 improve_xyz_xz use raw_improve4_xyz_xz raw_improve4a_xyz_xz;

    /// Improves 4D orientation so XY and ZW form orthogonal triangular-based planes.
    ///
    /// Recommended for 3D terrain, where X and Y or (Z and W) are horizontal.
    ///
    /// Recommended for the noise(x, y, sin(time), cos(time)) trick.
    ImproveXyZw 4 improve_xy_zw use raw_improve4_xy_zw raw_improve4a_xy_zw;
}
