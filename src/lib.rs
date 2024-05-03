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
//! let warped_noise = |pos: [f32; 2]| {
//!     let warp_x = OpenSimplex2s.seed(1).sample2(pos);
//!     let warp_y = OpenSimplex2s.seed(2).sample2(pos);
//!     let warped = [pos[0] + warp_x, pos[1] + warp_y];
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

mod lookup;
mod math;
mod util;

mod scalar;

#[cfg(feature = "nightly-simd")]
mod simd;

mod private_prelude {
    pub(crate) use crate::lookup::*;
    pub(crate) use crate::math::*;
    pub(crate) use crate::util::*;
    pub(crate) use crate::*;

    #[cfg(feature = "nightly-simd")]
    pub(crate) use crate::simd::*;

    #[cfg(feature = "nightly-simd")]
    pub(crate) use core::simd::{LaneCount, SimdElement, SupportedLaneCount};

    #[cfg(feature = "nightly-simd")]
    pub(crate) use core::simd::prelude::*;

    #[cfg(feature = "nightly-simd")]
    pub(crate) use core::simd::num::SimdFloat;
}

#[cfg(feature = "nightly-simd")]
use core::simd::{prelude::*, LaneCount, SupportedLaneCount};

#[cfg(feature = "nightly-simd")]
use crate::simd::splat;

pub use crate::util::fractal_bounding;

#[derive(Debug, Clone, Copy)]
pub struct Frequency<Noise> {
    pub base: Noise,
    pub frequency: f32,
}

impl<const DIM: usize, Noise> Sample<DIM, [f32; DIM]> for Frequency<Noise>
where
    Noise: Sample<DIM, [f32; DIM]>,
{
    fn sample(&self, mut pos: [f32; DIM]) -> f32 {
        let frequency = self.frequency;

        for x in &mut pos {
            *x *= frequency;
        }

        self.base.sample(pos)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, Noise> Sample<DIM, Simd<f32, LANES>> for Frequency<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    fn sample(&self, mut pos: Simd<f32, LANES>) -> f32 {
        pos *= splat(self.frequency);
        self.base.sample(pos)
    }
}

const DEFAULT_JITTER_2D: f32 = 0.43701595;
const DEFAULT_JITTER_3D: f32 = 0.39614353;

pub mod cellular {
    #[derive(Debug, Clone, Copy)]
    pub struct Jitter<CellularNoise> {
        pub base: CellularNoise,
        pub jitter: f32,
    }
}

pub trait Sample<const DIM: usize, Pos = [f32; DIM]> {
    fn sample(&self, pos: Pos) -> f32;
}

impl<const DIM: usize, Pos, Noise> Sample<DIM, Pos> for &Noise
where
    Noise: Sample<DIM, Pos>,
{
    #[inline(always)]
    fn sample(&self, pos: Pos) -> f32 {
        Noise::sample(self, pos)
    }
}

macro_rules! helper_trait {
	($(#[$attr:meta])* $trait:ident, $fn:ident, $dim:literal as $ty:ty) => {
		#[doc = concat!(
			"Helper trait that provides `",
			stringify!($fn),
			"` as a shorthand for `Sample<",
			stringify!($dim),
			", ",
			stringify!($ty),
			">::sample`.",
		)]
		///
		#[doc = concat!(
			"It also works for any `impl Into<",
			stringify!($ty),
			">`.",
		)]
		$(#[$attr])*
		pub trait $trait {
			fn $fn(&self, pos: impl Into<$ty>) -> f32;
		}

		$(#[$attr])*
		impl<Noise> $trait for Noise
		where
			Noise: Sample<$dim, $ty>,
		{
			#[inline(always)]
			fn $fn(&self, pos: impl Into<$ty>) -> f32 {
				Noise::sample(self, pos.into())
			}
		}
	};
}

helper_trait!(Sample2, sample2, 2 as [f32; 2]);
helper_trait!(Sample3, sample3, 3 as [f32; 3]);
helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample2a,
    sample2a,
    2 as f32x2
);
helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample3a,
    sample3a,
    3 as f32x4
);

macro_rules! impl_modifiers {
    () => {
        #[inline(always)]
        pub const fn seed(self, seed: i32) -> Seeded<Self> {
            Seeded { base: self, seed }
        }

        #[inline(always)]
        pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
            Frequency { base: self, frequency }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> Fbm<Self> {
                Fbm {
                    base: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                }
            }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn fbm_weighted(self, octaves: u32, gain: f32, lacunarity: f32, weighted_strength: f32) -> FbmWeighted<Self> {
                FbmWeighted {
                    base: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                    weighted_strength,
                }
            }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn ridged(self, octaves: u32, gain: f32, lacunarity: f32) -> Ridged<Self> {
                Ridged {
                    base: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                }
            }
        }

        cfg_const_feature_float! {
            #[inline(always)]
            pub fn ridged_weighted(self, octaves: u32, gain: f32, lacunarity: f32, weighted_strength: f32) -> RidgedWeighted<Self> {
                RidgedWeighted {
                    base: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: fractal_bounding(octaves, gain),
                    weighted_strength,
                }
            }
        }
    };
}

/// Wraps a function to make it implement [`Sample`].
///
/// The function is expected to take one parameter for the position and optionally
/// a seed parameter.
///
/// With a seed parameter it can be used for fractals:
///
/// ```rust
/// use noise_functions::{ NoiseFn, Sample2, OpenSimplex2s };
///
/// let warped = NoiseFn(|pos: [f32; 2], seed: i32| {
///     let warp_x = OpenSimplex2s.seed(seed + 100).sample2(pos);
///     let warp_y = OpenSimplex2s.seed(seed + 200).sample2(pos);
///     let warped = [pos[0] + warp_x, pos[1] + warp_y];
///     OpenSimplex2s.sample2(warped)
/// });
///
/// let warped_fbm = warped.fbm(3, 0.5, 2.0);
///
/// let value = warped_fbm.sample2([1.0, 2.0]);
/// ```
pub struct NoiseFn<F>(pub F);

impl<F> NoiseFn<F> {
    impl_modifiers!();
}

impl<const DIM: usize, Pos, F> Sample<DIM, Pos> for NoiseFn<F>
where
    F: Fn(Pos) -> f32,
{
    fn sample(&self, pos: Pos) -> f32 {
        self.0(pos)
    }
}

impl<const DIM: usize, Pos, F> Sample<DIM, Pos> for Seeded<NoiseFn<F>>
where
    F: Fn(Pos, i32) -> f32,
{
    fn sample(&self, pos: Pos) -> f32 {
        let &Seeded { ref base, seed } = self;
        base.0(pos, seed)
    }
}

impl<const DIM: usize, Pos, F> Sample<DIM, Pos> for Seeded<&NoiseFn<F>>
where
    F: Fn(Pos, i32) -> f32,
{
    fn sample(&self, pos: Pos) -> f32 {
        let &Seeded { base, seed } = self;
        base.0(pos, seed)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Seeded<Noise> {
    pub base: Noise,
    pub seed: i32,
}

impl<Noise> Seeded<Noise> {
    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }
}

mod fbm;
mod fbm_weighted;
pub mod open_simplex;
mod ridged;
mod ridged_weighted;

pub use fbm::Fbm;
pub use fbm_weighted::FbmWeighted;
pub use ridged::Ridged;
pub use ridged_weighted::RidgedWeighted;

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
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    (*self).seed(0).sample(pos)
                }
            }

            impl Sample<2> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, self.seed)
                }
            }

            impl Sample<2> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, self.seed)
                }
            }

            impl Sample<3> for $ty {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    (*self).seed(0).sample(pos)
                }
            }

            impl Sample<3> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, self.seed)
                }
            }

            impl Sample<3> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, self.seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            pub mod simd {
                use super::*;

                impl Sample<2, f32x2> for $ty {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        (*self).seed(0).sample(pos)
                    }
                }

                impl Sample<2, f32x2> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, self.seed)
                    }
                }

                impl Sample<2, f32x2> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, self.seed)
                    }
                }

                impl Sample<3, f32x4> for $ty {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        (*self).seed(0).sample(pos)
                    }
                }

                impl Sample<3, f32x4> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, self.seed)
                    }
                }

                impl Sample<3, f32x4> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, self.seed)
                    }
                }
            }
        }

        pub use $mod::$ty;
    };
}

macro_rules! open_simplex {
    ($mod:ident::$ty:ident) => {
        impl $ty {
            /// Improves 3D orientation as a fallback.
            pub const fn improve3(self) -> open_simplex::Improve3<Self> {
                open_simplex::Improve3(self)
            }

            /// Improves 3D orientation for the `XY` plane.
            pub const fn improve3_xy(self) -> open_simplex::Improve3Xy<Self> {
                open_simplex::Improve3Xy(self)
            }

            /// Improves 3D orientation for the `XZ` plane.
            pub const fn improve3_xz(self) -> open_simplex::Improve3Xz<Self> {
                open_simplex::Improve3Xz(self)
            }
        }

        basic_noise!($mod::$ty);
    };
}

macro_rules! cellular {
    ($mod:ident::$ty:ident) => {
        pub use $mod::$ty;

        mod $mod {
            use super::*;
            use cellular::Jitter;

            noise!($ty);

            impl $ty {
                /// Multiplies jitter by the provided value.
                pub const fn jitter(self, jitter: f32) -> Jitter<Self> {
                    Jitter { base: self, jitter }
                }

                impl_modifiers!();
            }

            impl Sample<2> for $ty {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, 0, DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for $ty {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, 0, DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Jitter<$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, 0, self.jitter)
                }
            }

            impl Sample<3> for Jitter<$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, 0, self.jitter)
                }
            }

            impl Sample<2> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, 0, DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, 0, DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Seeded<Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, self.seed, self.base.jitter * DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, self.seed, self.base.jitter * DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, 0, DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<&$ty> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, 0, DEFAULT_JITTER_3D)
                }
            }

            impl Sample<2> for Seeded<&Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 2]) -> f32 {
                    crate::scalar::$mod::gen2(pos, self.seed, self.base.jitter * DEFAULT_JITTER_2D)
                }
            }

            impl Sample<3> for Seeded<&Jitter<$ty>> {
                #[inline(always)]
                fn sample(&self, pos: [f32; 3]) -> f32 {
                    crate::scalar::$mod::gen3(pos, self.seed, self.base.jitter * DEFAULT_JITTER_3D)
                }
            }

            #[cfg(feature = "nightly-simd")]
            pub mod simd {
                use super::*;

                impl Sample<2, f32x2> for $ty {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, 0, DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for $ty {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, 0, DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Jitter<$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, 0, self.jitter * DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Jitter<$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, 0, self.jitter * DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, 0, DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, 0, DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, self.seed, self.base.jitter * DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, self.seed, self.base.jitter * DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, 0, DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<&$ty> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, 0, DEFAULT_JITTER_3D)
                    }
                }

                impl Sample<2, f32x2> for Seeded<&Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x2) -> f32 {
                        crate::simd::$mod::gen2(pos, self.seed, self.base.jitter * DEFAULT_JITTER_2D)
                    }
                }

                impl Sample<3, f32x4> for Seeded<&Jitter<$ty>> {
                    #[inline(always)]
                    fn sample(&self, pos: f32x4) -> f32 {
                        crate::simd::$mod::gen3(pos, self.seed, self.base.jitter * DEFAULT_JITTER_3D)
                    }
                }
            }
        }
    };
}

pub(crate) use basic_noise;

cellular!(cell_distance::CellDistance);
cellular!(cell_distance_sq::CellDistanceSq);
cellular!(cell_value::CellValue);
open_simplex!(open_simplex_2::OpenSimplex2);
open_simplex!(open_simplex_2s::OpenSimplex2s);
basic_noise!(perlin::Perlin);
basic_noise!(value::Value);
basic_noise!(value_cubic::ValueCubic);

#[cfg(test)]
mod tests;
