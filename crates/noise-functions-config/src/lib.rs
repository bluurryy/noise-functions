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

use noise_functions::{Noise as _, *};

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

#[derive(Debug, Clone, Copy)]
pub struct EnumFromStrError;

impl core::fmt::Display for EnumFromStrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("can't convert string to enum")
    }
}

impl core::error::Error for EnumFromStrError {}

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
        FastCellValue,
        FastCellDistance,
        FastCellDistanceSq,
    }
}

simple_enum! {
    enum Fractal {
        #[default]
        None,
        Fbm,
        Ridged,
        PingPong,
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

    // fractal
    pub fractal: Fractal,
    pub lacunarity: f32,
    pub octaves: u32,
    pub gain: f32,
    pub ping_pong_strength: f32,
    pub weighted_strength: f32,

    // open simplex 2
    pub improve: Improve,

    // cell
    pub jitter: f32,
    pub value_index: CellIndex,
    pub distance_fn: DistanceFn,
    pub distance_indices: [CellIndex; 2],
    pub distance_return_type: DistanceReturnType,

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

            // fractal
            fractal: Default::default(),
            lacunarity: 2.0,
            octaves: 3,
            gain: 0.5,
            ping_pong_strength: 2.0,
            weighted_strength: 0.0,

            // open simplex 2
            improve: Default::default(),

            // cell
            jitter: 1.0,
            value_index: CellIndex::I0,
            distance_fn: Default::default(),
            distance_indices: [CellIndex::I0, CellIndex::I1],
            distance_return_type: Default::default(),

            // tiling
            tileable: false,
            tile_width: 1.0,
            tile_height: 1.0,
        }
    }
}

macro_rules! make {
    ($self:ident, $noise:expr) => {
        Some(Box::new($noise.seed($self.seed).frequency($self.frequency)))
    };
}

macro_rules! make_fbm {
    ($self:ident, $noise:expr) => {
        make!($self, $noise.fbm($self.octaves, $self.gain, $self.lacunarity).weighted($self.weighted_strength))
    };
}

macro_rules! make_ridged {
    ($self:ident, $noise:expr) => {
        make!($self, $noise.ridged($self.octaves, $self.gain, $self.lacunarity).weighted($self.weighted_strength))
    };
}

macro_rules! make_ping_pong {
    ($self:ident, $noise:expr) => {
        make!(
            $self,
            $noise
                .ping_pong($self.octaves, $self.gain, $self.lacunarity, $self.ping_pong_strength)
                .weighted($self.weighted_strength)
        )
    };
}

macro_rules! make_fractal2 {
    ($self:ident, $macro:ident) => {
        if $self.tileable {
            match $self.noise {
                Noise::Value => $macro!($self, Value.tileable($self.tile_width, $self.tile_height)),
                Noise::Perlin => $macro!($self, Perlin.tileable($self.tile_width, $self.tile_height)),
                Noise::Simplex => $macro!($self, Simplex.tileable($self.tile_width, $self.tile_height)),
                Noise::CellValue => $macro!($self, $self.new_cell_value().tileable($self.tile_width, $self.tile_height)),
                Noise::CellDistance => $macro!($self, $self.new_cell_distance().tileable($self.tile_width, $self.tile_height)),
                Noise::FastCellValue => $macro!($self, FastCellValue::default().jitter($self.jitter).tileable($self.tile_width, $self.tile_height)),
                Noise::FastCellDistance => $macro!($self, FastCellDistance::default().jitter($self.jitter).tileable($self.tile_width, $self.tile_height)),
                Noise::FastCellDistanceSq => $macro!($self, FastCellDistanceSq::default().jitter($self.jitter).tileable($self.tile_width, $self.tile_height)),
                _ => None,
            }
        } else {
            match $self.noise {
                Noise::Value => $macro!($self, Value),
                Noise::ValueCubic => $macro!($self, ValueCubic),
                Noise::Perlin => $macro!($self, Perlin),
                Noise::Simplex => $macro!($self, Simplex),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => $macro!($self, OpenSimplex2),
                    Improve::Xy => $macro!($self, OpenSimplex2.improve_xy()),
                    Improve::Xz => $macro!($self, OpenSimplex2.improve_xz()),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => $macro!($self, OpenSimplex2s),
                    Improve::Xy => $macro!($self, OpenSimplex2s.improve_xy()),
                    Improve::Xz => $macro!($self, OpenSimplex2s.improve_xz()),
                },
                Noise::CellValue => $macro!($self, $self.new_cell_value()),
                Noise::CellDistance => $macro!($self, $self.new_cell_distance()),
                Noise::FastCellValue => $macro!($self, FastCellValue::default().jitter($self.jitter)),
                Noise::FastCellDistance => $macro!($self, FastCellDistance::default().jitter($self.jitter)),
                Noise::FastCellDistanceSq => $macro!($self, FastCellDistanceSq::default().jitter($self.jitter)),
            }
        }
    };
}

macro_rules! sampler2 {
    ($self:ident) => {
        match $self.fractal {
            Fractal::None => make_fractal2!($self, make),
            Fractal::Fbm => make_fractal2!($self, make_fbm),
            Fractal::Ridged => make_fractal2!($self, make_ridged),
            Fractal::PingPong => make_fractal2!($self, make_ping_pong),
        }
    };
}

macro_rules! make_fractal3 {
    ($self:ident, $macro:ident) => {
        if $self.tileable {
            None
        } else {
            match $self.noise {
                Noise::Value => $macro!($self, Value),
                Noise::ValueCubic => $macro!($self, ValueCubic),
                Noise::Perlin => $macro!($self, Perlin),
                Noise::Simplex => $macro!($self, Simplex),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => $macro!($self, OpenSimplex2),
                    Improve::Xy => $macro!($self, OpenSimplex2.improve_xy()),
                    Improve::Xz => $macro!($self, OpenSimplex2.improve_xz()),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => $macro!($self, OpenSimplex2s),
                    Improve::Xy => $macro!($self, OpenSimplex2s.improve_xy()),
                    Improve::Xz => $macro!($self, OpenSimplex2s.improve_xz()),
                },
                Noise::CellValue => $macro!($self, $self.new_cell_value()),
                Noise::CellDistance => $macro!($self, $self.new_cell_distance()),
                Noise::FastCellValue => $macro!($self, FastCellValue::default().jitter($self.jitter)),
                Noise::FastCellDistance => $macro!($self, FastCellDistance::default().jitter($self.jitter)),
                Noise::FastCellDistanceSq => $macro!($self, FastCellDistanceSq::default().jitter($self.jitter)),
            }
        }
    };
}

macro_rules! sampler3 {
    ($self:ident) => {
        match $self.fractal {
            Fractal::None => make_fractal3!($self, make),
            Fractal::Fbm => make_fractal3!($self, make_fbm),
            Fractal::Ridged => make_fractal3!($self, make_ridged),
            Fractal::PingPong => make_fractal3!($self, make_ping_pong),
        }
    };
}

macro_rules! make_fractal4 {
    ($self:ident, $macro:ident) => {
        match $self.noise {
            Noise::Value => $macro!($self, Value),
            Noise::Perlin => $macro!($self, Perlin),
            Noise::Simplex => $macro!($self, Simplex),
            Noise::CellValue => $macro!($self, $self.new_cell_value()),
            Noise::CellDistance => $macro!($self, $self.new_cell_distance()),
            Noise::FastCellValue => $macro!($self, FastCellValue::default().jitter($self.jitter)),
            Noise::FastCellDistance => $macro!($self, FastCellDistance::default().jitter($self.jitter)),
            Noise::FastCellDistanceSq => $macro!($self, FastCellDistanceSq::default().jitter($self.jitter)),
            _ => None,
        }
    };
}

macro_rules! sampler4 {
    ($self:ident) => {
        match $self.fractal {
            Fractal::None => make_fractal4!($self, make),
            Fractal::Fbm => make_fractal4!($self, make_fbm),
            Fractal::Ridged => make_fractal4!($self, make_ridged),
            Fractal::PingPong => make_fractal4!($self, make_ping_pong),
        }
    };
}

impl Config {
    fn new_cell_value(&self) -> CellValue {
        CellValue {
            jitter: self.jitter,
            distance_fn: self.distance_fn,
            value_index: self.value_index,
        }
    }

    fn new_cell_distance(&self) -> CellDistance {
        CellDistance {
            jitter: self.jitter,
            distance_fn: self.distance_fn,
            distance_indices: self.distance_indices,
            return_type: self.distance_return_type,
        }
    }

    pub fn sampler2(&self) -> Option<Box<dyn Sample<2>>> {
        sampler2!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler2a(&self) -> Option<Box<dyn Sample<2, f32x2>>> {
        sampler2!(self)
    }

    pub fn sampler3(&self) -> Option<Box<dyn Sample<3>>> {
        sampler3!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler3a(&self) -> Option<Box<dyn Sample<3, f32x4>>> {
        sampler3!(self)
    }

    pub fn sampler4(&self) -> Option<Box<dyn Sample<4>>> {
        sampler4!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler4a(&self) -> Option<Box<dyn Sample<4, f32x4>>> {
        sampler4!(self)
    }
}
