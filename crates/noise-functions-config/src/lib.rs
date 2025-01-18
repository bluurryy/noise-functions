//! Configurable noise generator struct for the [`noise-functions`](https://docs.rs/noise-functions) crate.
//!
//! Every `enum` of this crate implements `FromStr`, `to_str` and has a `VARIANTS` constant.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly-simd", feature(portable_simd))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

use alloc::boxed::Box;

use noise_functions::{Noise as _, OpenSimplexNoise as _, *};

#[cfg(feature = "nightly-simd")]
use core::simd::*;

pub use noise_functions;

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
			type Err = noise_functions::errors::EnumFromStrError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Ok(match s {
					$(stringify!($variant) => Self::$variant,)*
					_ => return Err(noise_functions::errors::EnumFromStrError),
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

simple_enum! {
    enum Noise {
        Value,
        ValueCubic,
        #[default]
        Perlin,
        Simplex,
        OpenSimplex2,
        OpenSimplex2s,
        CellValue,
        CellDistance,
        CellDistanceSq,
    }
}

simple_enum! {
    enum Modifier {
        #[default]
        None,
        Ridged,
        TriangleWave,
    }
}

simple_enum! {
    enum Improve {
        None,
        #[default]
        Xy,
        Xz,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    pub noise: Noise,
    pub seed: i32,
    pub frequency: f32,

    // modifiers
    pub modifier: Modifier,
    pub triangle_wave_frequency: f32,

    // fractal
    pub fractal: bool,
    pub lacunarity: f32,
    pub octaves: u32,
    pub gain: f32,
    pub weighted_strength: f32,

    // open simplex 2
    pub improve: Improve,

    // cell
    pub jitter: f32,

    // tiling
    pub tileable: bool,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            noise: Default::default(),
            seed: Default::default(),
            frequency: 1.0,

            // modifiers
            modifier: Modifier::default(),
            triangle_wave_frequency: 2.0,

            // fractal
            fractal: false,
            lacunarity: 2.0,
            octaves: 3,
            gain: 0.5,
            weighted_strength: 0.0,

            // open simplex 2
            improve: Default::default(),

            // cell
            jitter: 1.0,

            // tiling
            tileable: false,
            tile_width: 1.0,
            tile_height: 1.0,
        }
    }
}

macro_rules! finish {
    ($self:ident, $noise:expr) => {
        Some(Box::new($noise.seed($self.seed).frequency($self.frequency)))
    };
}

macro_rules! fractal {
    ($self:ident, $noise:expr) => {
        if $self.fractal {
            finish!($self, $noise.fbm($self.octaves, $self.gain, $self.lacunarity).weighted($self.weighted_strength))
        } else {
            finish!($self, $noise)
        }
    };
}

macro_rules! modifier {
    ($self:ident, $noise:expr) => {
        match $self.modifier {
            Modifier::None => fractal!($self, $noise),
            Modifier::Ridged => fractal!($self, $noise.ridged()),
            Modifier::TriangleWave => fractal!($self, $noise.triangle_wave($self.triangle_wave_frequency)),
        }
    };
}

macro_rules! apply_tileable {
    ($self:ident, $noise:expr) => {
        if $self.tileable {
            modifier!($self, $noise.tileable($self.tile_width, $self.tile_height))
        } else {
            modifier!($self, $noise)
        }
    };
}

macro_rules! dont_apply_tileable {
    ($self:ident, $noise:expr) => {
        if $self.tileable {
            None
        } else {
            modifier!($self, $noise)
        }
    };
}

macro_rules! if_supports_4d {
    (Value { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (ValueCubic { $($then:tt)* } else { $($else:tt)* }) => { $($else)* };
    (Perlin { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (Simplex { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (OpenSimplex2 { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (OpenSimplex2s { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (CellValue { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (CellDistance { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (CellDistanceSq { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
}

macro_rules! tileable {
    (2, $self:ident, $noise_name:ident $($noise_rest:tt)*) => {
        if_supports_4d!($noise_name {
            apply_tileable!($self, $noise_name $($noise_rest)*)
        } else {
            dont_apply_tileable!($self, $noise_name $($noise_rest)*)
        })
    };
    (3, $self:ident, $noise_name:ident $($noise_rest:tt)*) => {
        dont_apply_tileable!($self, $noise_name $($noise_rest)*)
    };
    (4, $self:ident, $noise_name:ident $($noise_rest:tt)*) => {
        if_supports_4d!($noise_name {
            dont_apply_tileable!($self, $noise_name $($noise_rest)*)
        } else {
            None
        })
    };
}

macro_rules! base {
    ($dim:tt, $self:ident) => {
        match $self.noise {
            Noise::Value => tileable!($dim, $self, Value),
            Noise::ValueCubic => tileable!($dim, $self, ValueCubic),
            Noise::Perlin => tileable!($dim, $self, Perlin),
            Noise::Simplex => tileable!($dim, $self, Simplex),
            Noise::OpenSimplex2 => match $self.improve {
                Improve::None => tileable!($dim, $self, OpenSimplex2),
                Improve::Xy => tileable!($dim, $self, OpenSimplex2.improve_xy()),
                Improve::Xz => tileable!($dim, $self, OpenSimplex2.improve_xz()),
            },
            Noise::OpenSimplex2s => match $self.improve {
                Improve::None => tileable!($dim, $self, OpenSimplex2s),
                Improve::Xy => tileable!($dim, $self, OpenSimplex2s.improve_xy()),
                Improve::Xz => tileable!($dim, $self, OpenSimplex2s.improve_xz()),
            },
            Noise::CellValue => tileable!($dim, $self, CellValue::default().jitter($self.jitter)),
            Noise::CellDistance => tileable!($dim, $self, CellDistance::default().jitter($self.jitter)),
            Noise::CellDistanceSq => tileable!($dim, $self, CellDistanceSq::default().jitter($self.jitter)),
        }
    };
}

macro_rules! sampler {
    ($dim:tt, $self:ident) => {
        base!($dim, $self)
    };
}

impl Config {
    pub fn sampler2(&self) -> Option<Box<dyn Sample<2>>> {
        sampler!(2, self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler2a(&self) -> Option<Box<dyn Sample<2, f32x2>>> {
        sampler!(2, self)
    }

    pub fn sampler3(&self) -> Option<Box<dyn Sample<3>>> {
        sampler!(3, self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler3a(&self) -> Option<Box<dyn Sample<3, f32x4>>> {
        sampler!(3, self)
    }

    pub fn sampler4(&self) -> Option<Box<dyn Sample<4>>> {
        sampler!(4, self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler4a(&self) -> Option<Box<dyn Sample<4, f32x4>>> {
        sampler!(4, self)
    }
}
