mod cell_distance;
mod cell_distance_sq;
mod cell_value;
mod custom_cell_distance;
mod custom_cell_value;
mod open_simplex_2;
mod open_simplex_2s;
mod perlin;
mod simplex;
mod value;
mod value_cubic;

pub use cell_distance::CellDistance;
pub use cell_distance_sq::CellDistanceSq;
pub use cell_value::CellValue;
use custom_cell_distance::CustomCellDistance;
use custom_cell_value::CustomCellValue;
pub use open_simplex_2::OpenSimplex2;
pub use open_simplex_2s::OpenSimplex2s;
pub use perlin::Perlin;
pub use simplex::Simplex;
pub use value::Value;
pub use value_cubic::ValueCubic;

macro_rules! if_has_dim {
    (2 in ; $($tt:tt)*) => {};
    (2 in 2; $($tt:tt)*) => { $($tt)* };
    (2 in 23; $($tt:tt)*) => { $($tt)* };
    (2 in 234; $($tt:tt)*) => { $($tt)* };
    (3 in ; $($tt:tt)*) => {};
    (3 in 2; $($tt:tt)*) => {};
    (3 in 23; $($tt:tt)*) => { $($tt)* };
    (3 in 234; $($tt:tt)*) => { $($tt)* };
    (4 in ; $($tt:tt)*) => {};
    (4 in 2; $($tt:tt)*) => {};
    (4 in 23; $($tt:tt)*) => {};
    (4 in 234; $($tt:tt)*) => { $($tt)* };
}

macro_rules! impl_noise {
    ($dims:tt $struct:ident) => {
        impl $crate::Noise for $struct {}

        $crate::base::if_has_dim! { 2 in $dims;
            impl $crate::Sample<2> for $struct {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    self.gen2(point, 0)
                }
            }

            impl $crate::SampleWithSeed<2> for $struct {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 2], seed: i32) -> f32 {
                    self.gen2(point, seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::Sample<2, core::simd::f32x2> for $struct {
                #[inline(always)]
                fn sample(&self, point: core::simd::f32x2) -> f32 {
                    self.gen2a(point, 0)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::SampleWithSeed<2, core::simd::f32x2> for $struct {
                #[inline(always)]
                fn sample_with_seed(&self, point: core::simd::f32x2, seed: i32) -> f32 {
                    self.gen2a(point, seed)
                }
            }
        }

        $crate::base::if_has_dim! { 3 in $dims;
            impl $crate::Sample<3> for $struct {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    self.gen3(point, 0)
                }
            }

            impl $crate::SampleWithSeed<3> for $struct {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
                    self.gen3(point, seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::Sample<3, core::simd::f32x4> for $struct {
                #[inline(always)]
                fn sample(&self, point: core::simd::f32x4) -> f32 {
                    self.gen3a(point, 0)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::SampleWithSeed<3, core::simd::f32x4> for $struct {
                #[inline(always)]
                fn sample_with_seed(&self, point: core::simd::f32x4, seed: i32) -> f32 {
                    self.gen3a(point, seed)
                }
            }
        }

        $crate::base::if_has_dim! { 4 in $dims;
            impl $crate::Sample<4> for $struct {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    self.gen4(point, 0)
                }
            }

            impl $crate::SampleWithSeed<4> for $struct {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 4], seed: i32) -> f32 {
                    self.gen4(point, seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::Sample<4, core::simd::f32x4> for $struct {
                #[inline(always)]
                fn sample(&self, point: core::simd::f32x4) -> f32 {
                    self.gen4a(point, 0)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::SampleWithSeed<4, core::simd::f32x4> for $struct {
                #[inline(always)]
                fn sample_with_seed(&self, point: core::simd::f32x4, seed: i32) -> f32 {
                    self.gen4a(point, seed)
                }
            }
        }
    };
}

pub(crate) use if_has_dim;
pub(crate) use impl_noise;
