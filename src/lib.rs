//!
//! Fast and lightweight noise algorithm implementations.
//!
//! Check out the [live demo](https://bluurryy.github.io/noise-functions-demo/)!
//!
//! ## Examples
//! ```
//! use noise_functions::{ Noise, OpenSimplexNoise, Perlin, CellDistance, OpenSimplex2s, Sample2, Sample3, NoiseFn };
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
//! let value = CellDistance::default().sample2(point);
//!
//! // cell distance (voronoi/worley) noise with jitter multiplier
//! let value = CellDistance::default().jitter(0.5).sample2(point);
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

mod base;
mod cellular;
/// This is experimental and not part of the public api.
#[doc(hidden)]
pub mod from_fast_noise_2;
mod from_fast_noise_lite;
mod math;
/// Noise modifiers.
pub mod modifiers;
mod noise;
mod noise_fn;
mod open_simplex_2;
mod sample;

pub use base::{CustomCellDistance, CustomCellValue, FastCellDistance, FastCellDistanceSq, FastCellValue, OpenSimplex2, OpenSimplex2s, Perlin, Simplex, Value, ValueCubic};
pub use cellular::{CellIndex, DistanceFn, DistanceReturnType};
pub use noise::Noise;
pub use noise_fn::NoiseFn;
pub use open_simplex_2::OpenSimplexNoise;
pub use sample::{Sample, Sample2, Sample3, Sample4, SampleWithSeed};
#[cfg(feature = "nightly-simd")]
pub use sample::{Sample2a, Sample3a, Sample4a};

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
			type Err = $crate::errors::EnumFromStrError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Ok(match s {
					$(stringify!($variant) => Self::$variant,)*
					_ => return Err($crate::errors::EnumFromStrError),
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

/// Error types.
pub mod errors {
    #[derive(Debug, Clone, Copy)]
    pub struct EnumFromStrError;

    impl core::fmt::Display for EnumFromStrError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("can't convert string to enum")
        }
    }
}

#[cfg(test)]
mod tests;
