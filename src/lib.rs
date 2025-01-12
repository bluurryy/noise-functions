//!
//! Fast and lightweight noise algorithm implementations.
//!
//! Check out the [live demo](https://bluurryy.github.io/noise-functions-demo/)!
//!
//! The implementation of these noise functions are from FastNoiseLite ([github](https://github.com/Auburn/FastNoiseLite)/[crate](https://docs.rs/fastnoise-lite/latest/fastnoise_lite/)).
//!
//! ## Examples
//! ```
//! use noise_functions::{ Perlin, CellDistance, OpenSimplex2s, Sample2, Sample3, NoiseFn };
//!
//! let point = [1.0, 2.0];
//!
//! // perlin noise
//! let value = Perlin.sample2(point);
//!
//! // seeded perlin noise
//! let value = Perlin.seed(42).sample2(point);
//!
//! // fractal perlin noise
//! let value = Perlin.fbm(3, 0.5, 3.0).sample2(point);
//!
//! // seeded fractal perlin noise
//! let value = Perlin.fbm(3, 0.5, 3.0).seed(42).sample2(point);
//!
//! // perlin noise with adjusted frequency
//! let value = Perlin.frequency(3.0).sample2(point);
//!
//! // cell distance (voronoi/worley) noise
//! let value = CellDistance.sample2(point);
//!
//! // cell distance (voronoi/worley) noise with jitter multiplier
//! let value = CellDistance.jitter(0.5).sample2(point);
//!
//! // domain warped OpenSimplex2s noise
//! let warped_noise = |point: [f32; 2]| {
//!     let warp_x = OpenSimplex2s.seed(1).sample2(point);
//!     let warp_y = OpenSimplex2s.seed(2).sample2(point);
//!     let warped = [point[0] + warp_x, point[1] + warp_y];
//!     OpenSimplex2s.sample2(warped)
//! };
//!
//! let value = warped_noise(point);
//!
//! let point = [1.0, 2.0, 3.0];
//!
//! // 3d OpenSimplex2s noise
//! let value = OpenSimplex2s.sample3(point);
//!
//! // 3d OpenSimplex2s noise with improved isotropy in the xy plane
//! let value = OpenSimplex2s.improve_xy().sample3(point);
//! ```
//!
//! ## Feature flags
#![no_std]
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(feature = "nightly-simd", feature(portable_simd))]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide), doc(cfg_hide(no_global_oom_handling, feature = "nightly-const-fn-float")))]
#![allow(clippy::excessive_precision, clippy::needless_late_init, clippy::too_many_arguments)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!(r#"`noise-functions` crate: either the "std" or "libm" feature must be enabled"#);

/// Cellular noise functions and combinators.
pub mod cellular;

/// Fractal noise combinators.
pub mod fractal;
mod frequency;
mod lookup;
mod math;
mod noise_fn;

/// This is experimental and not part of the public api.
#[doc(hidden)]
pub mod from_fast_noise_2;

/// OpenSimplex2 noise functions and combinators.
pub mod open_simplex_2;
mod sample;
mod scalar;
mod seeded;
#[cfg(feature = "nightly-simd")]
mod simd;
mod tileable;
mod util;

#[doc(inline)]
pub use cellular::{CellDistance, CellDistanceSq, CellValue};
pub use frequency::Frequency;
pub use noise_fn::NoiseFn;
pub use tileable::Tileable;

#[doc(inline)]
pub use open_simplex_2::{OpenSimplex2, OpenSimplex2s};
pub use sample::{Sample, Sample2, Sample3, Sample4};
pub use seeded::Seeded;

#[cfg(feature = "nightly-simd")]
pub use sample::{Sample2a, Sample3a, Sample4a};

mod private_prelude {
    pub(crate) use crate::fractal::*;
    pub(crate) use crate::lookup::*;
    pub(crate) use crate::math::*;
    pub(crate) use crate::util::*;
    pub(crate) use crate::*;

    #[cfg(test)]
    #[cfg(feature = "nightly-simd")]
    pub(crate) use crate::open_simplex_2::*;

    #[cfg(test)]
    #[cfg(feature = "nightly-simd")]
    pub(crate) use crate::cellular::*;

    #[cfg(feature = "nightly-simd")]
    pub(crate) use crate::simd::{PRIME_XY, PRIME_XYZ};

    #[cfg(feature = "nightly-simd")]
    pub(crate) use core::simd::{LaneCount, SimdElement, SupportedLaneCount};

    #[cfg(feature = "nightly-simd")]
    pub(crate) use core::simd::prelude::*;

    #[cfg(feature = "nightly-simd")]
    pub(crate) use core::simd::num::SimdFloat;

    #[cfg(feature = "nightly-simd")]
    pub(crate) use crate::simd::splat;
}

use crate::private_prelude::*;

const DEFAULT_JITTER_2D: f32 = 0.43701595;
const DEFAULT_JITTER_3D: f32 = 0.39614353;

macro_rules! impl_modifiers {
    () => {
        #[inline(always)]
        pub const fn seed(self, seed: i32) -> $crate::Seeded<Self> {
            $crate::Seeded { noise: self, seed }
        }

        #[inline(always)]
        pub const fn frequency(self, frequency: f32) -> $crate::Frequency<Self> {
            $crate::Frequency { noise: self, frequency }
        }

        #[inline(always)]
        pub const fn tileable(self, width: f32, height: f32) -> $crate::Tileable<Self> {
            $crate::Tileable::new(self, width, height)
        }

        #[inline(always)]
        pub const fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> $crate::Fbm<Self> {
            $crate::Fbm {
                noise: self,
                octaves,
                gain,
                lacunarity,
                fractal_bounding: $crate::fractal_bounding(octaves, gain),
            }
        }

        #[inline(always)]
        pub const fn ridged(self, octaves: u32, gain: f32, lacunarity: f32) -> $crate::Ridged<Self> {
            $crate::Ridged {
                noise: self,
                octaves,
                gain,
                lacunarity,
                fractal_bounding: $crate::fractal_bounding(octaves, gain),
            }
        }

        #[inline(always)]
        pub const fn ping_pong(self, octaves: u32, gain: f32, lacunarity: f32, strength: f32) -> $crate::PingPong<Self> {
            $crate::PingPong {
                noise: self,
                octaves,
                gain,
                lacunarity,
                fractal_bounding: $crate::fractal_bounding(octaves, gain),
                strength,
            }
        }
    };
}

pub(crate) use impl_modifiers;

macro_rules! basic_noise {
    ($(#[$attr:meta])* $noise:ident in $noise_mod:ident $(use $noise_4d:expr)?) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $noise;

        impl $noise {
            impl_modifiers!();
        }

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
                crate::scalar::$noise_mod::gen3(point, 0)
            }
        }

        impl Sample<3> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, self.seed)
            }
        }

        impl Sample<3> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, self.seed)
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
                crate::simd::$noise_mod::gen3(point, 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, self.seed)
            }
        }

        $(
            impl Sample<4> for $noise {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    $noise_4d.sample4(point)
                }
            }

            impl Sample<4> for Seeded<$noise> {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    $noise_4d.seed(self.seed).sample4(point)
                }
            }

            impl Sample<4> for Seeded<&$noise> {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    $noise_4d.seed(self.seed).sample4(point)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl Sample<4, f32x4> for $noise {
                #[inline(always)]
                fn sample(&self, point: f32x4) -> f32 {
                    $noise_4d.sample4(point)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl Sample<4, f32x4> for Seeded<$noise> {
                #[inline(always)]
                fn sample(&self, point: f32x4) -> f32 {
                    $noise_4d.seed(self.seed).sample4(point)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl Sample<4, f32x4> for Seeded<&$noise> {
                #[inline(always)]
                fn sample(&self, point: f32x4) -> f32 {
                    $noise_4d.seed(self.seed).sample4(point)
                }
            }
        )?
    };
}

pub(crate) use basic_noise;

basic_noise! {
    /// 2/3/4 dimensional Perlin noise
    Perlin in perlin use from_fast_noise_2::Perlin
}

basic_noise! {
    /// 2/3/4 dimensional Value noise
    Value in value use from_fast_noise_2::Value
}

basic_noise! {
    /// 2/3 dimensional Cubic Value noise
    ValueCubic in value_cubic
}

#[inline(always)]
#[cfg(feature = "nightly-simd")]
fn array_4_take_3<T>(array: &[T; 4]) -> &[T; 3] {
    array[..3].try_into().unwrap()
}

macro_rules! simple_enum {
	(
		enum $name:ident {
			$(
                $(#[$variant_attr:meta])*
                $variant:ident $(= $variant_expr:expr)?
            ),* $(,)?
		}
	) => {
		#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub enum $name {
			$(
                $(#[$variant_attr])*
                $variant $(= $variant_expr)?,
            )*
		}

		impl core::str::FromStr for $name {
			type Err = $crate::EnumFromStrError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Ok(match s {
					$(stringify!($variant) => Self::$variant,)*
					_ => return Err($crate::EnumFromStrError),
				})
			}
		}

		impl $name {
			pub const VARIANTS: &'static [Self] = &[
				$(Self::$variant,)*
			];

			pub fn to_str(self) -> &'static str {
				[$(stringify!($variant)),*][self as usize]
			}
		}

		impl core::fmt::Debug for $name {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				f.write_str(self.to_str())
			}
		}

		impl core::fmt::Display for $name {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				f.write_str(self.to_str())
			}
		}
	};
}

pub(crate) use simple_enum;

#[derive(Debug, Clone, Copy)]
pub struct EnumFromStrError;

impl core::fmt::Display for EnumFromStrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("can't convert string to enum")
    }
}

#[cfg(test)]
mod tests;
