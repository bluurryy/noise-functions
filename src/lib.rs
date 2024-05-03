//!
//! Fast and lightweight noise algorithm implementations.
//!
//! Check out the [live demo](https://bluurryy.github.io/noise-functions-demo/)!
//!
//! ```
//! use noise_functions::{ Perlin, CellDistance, OpenSimplex2s, Sample2, NoiseFn };
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
//! // cell distance (voronoi) noise
//! let value = CellDistance.sample2(point);
//!
//! // cell distance (voronoi) noise with jitter multiplier
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
//! ```
//!
//! The implementation of these noise functions are from FastNoiseLite ([github](https://github.com/Auburn/FastNoiseLite)/[crate](https://docs.rs/fastnoise-lite/latest/fastnoise_lite/)).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly-simd", feature(portable_simd))]
#![cfg_attr(feature = "nightly-const-fn-float", feature(const_fn_floating_point_arithmetic))]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide), doc(cfg_hide(no_global_oom_handling, feature = "nightly-const-fn-float")))]
#![allow(clippy::excessive_precision, clippy::needless_late_init, clippy::too_many_arguments)]

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!(r#"`noise-functions` crate: either the "std" or "libm" feature must be enabled"#);

#[cfg(feature = "alloc")]
extern crate alloc;

/// Cellular noise functions and combinators.
pub mod cellular;

/// Fractal noise combinators.
pub mod fractal;
mod frequency;
mod lookup;
mod math;
mod noise_fn;

/// OpenSimplex2 noise functions and combinators.
pub mod open_simplex;
mod sample;
mod scalar;
mod seeded;
#[cfg(feature = "nightly-simd")]
mod simd;
mod util;

pub use frequency::Frequency;
pub use noise_fn::NoiseFn;
pub use open_simplex::{OpenSimplex2, OpenSimplex2s};
pub use sample::{Sample, Sample2, Sample3};
pub use seeded::Seeded;

#[cfg(feature = "nightly-simd")]
pub use sample::{Sample2a, Sample3a};

mod private_prelude {
    pub(crate) use crate::fractal::*;
    pub(crate) use crate::lookup::*;
    pub(crate) use crate::math::*;
    pub(crate) use crate::util::*;
    pub(crate) use crate::*;

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
        pub const fn seed(self, seed: i32) -> Seeded<Self> {
            Seeded { noise: self, seed }
        }

        #[inline(always)]
        pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
            Frequency { noise: self, frequency }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> Fbm<Self> {
                Fbm {
                    noise: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                }
            }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn ridged(self, octaves: u32, gain: f32, lacunarity: f32) -> Ridged<Self> {
                Ridged {
                    noise: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                }
            }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn ping_pong(self, octaves: u32, gain: f32, lacunarity: f32, strength: f32) -> PingPong<Self> {
                PingPong {
                    noise: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                    strength,
                }
            }
        }
    };
}

pub(crate) use impl_modifiers;

macro_rules! cfg_const {
	(
		#[cfg_const($($tt:tt)*)]
		$(#[$attr:meta])*
		$vis:vis fn $ident:ident($($params:tt)*) $(-> $result:ty)? $body:block
	) => {
		#[cfg($($tt)*)]
		$(#[$attr])*
		$vis const fn $ident($($params)*) $(-> $result)? $body

		#[cfg(not($($tt)*))]
		$(#[$attr])*
		$vis fn $ident($($params)*) $(-> $result)? $body
	};
}

macro_rules! cfg_const_feature {
	(
		#[cfg_const_feature($feature:literal)]
		$(#[$attr:meta])*
		$vis:vis fn $ident:ident($($params:tt)*) $(-> $result:ty)? $body:block
	) => {
		$crate::cfg_const! {
			#[cfg_const(feature = $feature)]
			$(#[$attr])*
			#[doc = concat!("*This function is `const` if the feature `", $feature, "` is enabled.*")]
			$vis fn $ident($($params)*) $(-> $result)? $body
		}
	};
}

macro_rules! cfg_const_feature_float {
	(

		$(#[$attr:meta])*
		$vis:vis fn $ident:ident($($params:tt)*) $(-> $result:ty)? $body:block
	) => {
		$crate::cfg_const_feature! {
			#[cfg_const_feature("nightly-const-fn-float")]
			$(#[$attr])*
			$vis fn $ident($($params)*) $(-> $result)? $body
		}
	};
}

pub(crate) use cfg_const;
pub(crate) use cfg_const_feature;
pub(crate) use cfg_const_feature_float;

macro_rules! noise {
    ($(#[$attr:meta])* $ty:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $ty;
    };
}

macro_rules! basic_noise {
    ($(#[$attr:meta])* $mod:ident::$ty:ident) => {
        mod $mod {
            use super::*;

            noise!($(#[$attr])* $ty);

            impl $ty {
                impl_modifiers!();
            }

            impl Sample<2> for $ty {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    (*self).seed(0).sample(point)
                }
            }

            impl Sample<2> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, self.seed)
                }
            }

            impl Sample<2> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, self.seed)
                }
            }

            impl Sample<3> for $ty {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    (*self).seed(0).sample(point)
                }
            }

            impl Sample<3> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, self.seed)
                }
            }

            impl Sample<3> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, self.seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            pub mod simd {
                use super::*;

                impl Sample<2, f32x2> for $ty {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        (*self).seed(0).sample(point)
                    }
                }

                impl Sample<2, f32x2> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, self.seed)
                    }
                }

                impl Sample<2, f32x2> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, self.seed)
                    }
                }

                impl Sample<3, f32x4> for $ty {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        (*self).seed(0).sample(point)
                    }
                }

                impl Sample<3, f32x4> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, self.seed)
                    }
                }

                impl Sample<3, f32x4> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, self.seed)
                    }
                }
            }
        }

        pub use $mod::$ty;
    };
}

macro_rules! cellular {
    ($(#[$attr:meta])* $mod:ident::$ty:ident) => {
        pub use $mod::$ty;

        mod $mod {
            use super::*;
            use cellular::Jitter;

            noise!($(#[$attr])* $ty);

            impl $ty {
                /// Multiplies jitter by the provided value.
                pub const fn jitter(self, jitter: f32) -> Jitter<Self> {
                    Jitter { noise: self, jitter }
                }

                impl_modifiers!();
            }

            impl Sample<2> for $ty {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, 0, DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for $ty {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, 0, DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Jitter<$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, 0, self.jitter)
                }
            }

            impl Sample<3> for Jitter<$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, 0, self.jitter)
                }
            }

            impl Sample<2> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, 0, DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, 0, DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Seeded<Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, 0, DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, 0, DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Seeded<&Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<&Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
                }
            }

            #[cfg(feature = "nightly-simd")]
            pub mod simd {
                use super::*;

                impl Sample<2, f32x2> for $ty {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, 0, DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for $ty {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, 0, DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Jitter<$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, 0, self.jitter * DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Jitter<$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, 0, self.jitter * DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, 0, DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, 0, DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, 0, DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, 0, DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<&Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, point: f32x2) -> f32 {
                        crate::simd::$mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<&Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, point: f32x4) -> f32 {
                        crate::simd::$mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
                    }
                }
            }
        }
    };
}

pub(crate) use basic_noise;

cellular! {
    /// 2/3 dimensional noise of the distance to the closest cell
    cell_distance::CellDistance
}
cellular! {
    /// 2/3 dimensional noise of the squared distance to the closest cell
    cell_distance_sq::CellDistanceSq
}
cellular! {
    /// 2/3 dimensional noise of the value of the closest cell
    cell_value::CellValue
}

basic_noise! {
    /// 2/3 dimensional Perlin noise
    perlin::Perlin
}
basic_noise! {
    /// 2/3 dimensional Value noise
    value::Value
}
basic_noise! {
    /// 2/3 dimensional Cubic Value noise
    value_cubic::ValueCubic
}

#[cfg(test)]
mod tests;
