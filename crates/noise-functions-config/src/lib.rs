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

use noise_functions::*;

#[cfg(feature = "nightly-simd")]
use core::simd::*;

pub use noise_functions;

macro_rules! simple_enum {
	(
		enum $name:ident {
			$($variant:ident),* $(,)?
		}
	) => {
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub enum $name {
			$($variant,)*
		}

		impl core::str::FromStr for $name {
			type Err = EnumFromStrError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Ok(match s {
					$(stringify!($variant) => Self::$variant,)*
					_ => return Err(EnumFromStrError),
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

#[cfg(feature = "std")]
impl std::error::Error for EnumFromStrError {}

simple_enum! {
    enum Noise {
        CellDistance,
        CellDistanceSq,
        CellValue,
        OpenSimplex2,
        OpenSimplex2s,
        Perlin,
        Value,
        ValueCubic,
        NewPerlin,
        NewValue,
        NewOpenSimplex2,
    }
}

simple_enum! {
    enum Fractal {
        None,
        Fbm,
        Ridged,
        PingPong,
    }
}

simple_enum! {
    enum Improve {
        None,
        Xy,
        Xz,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    pub noise: Noise,
    pub fractal: Fractal,
    pub improve: Improve,
    pub lacunarity: f32,
    pub octaves: u32,
    pub gain: f32,
    pub ping_pong_strength: f32,
    pub weighted_strength: f32,
    pub seed: i32,
    pub frequency: f32,
    pub jitter: f32,
    pub tileable: bool,
    pub tile_width: f32,
    pub tile_height: f32,
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
        make!($self, $noise.ping_pong($self.octaves, $self.gain, $self.lacunarity, $self.ping_pong_strength).weighted($self.weighted_strength))
    };
}

macro_rules! make_fractal2 {
    ($self:ident, $macro:ident) => {
        if $self.tileable {
            match $self.noise {
                Noise::NewPerlin => $macro!($self, from_fast_noise_2::Perlin.tileable($self.tile_width, $self.tile_height)),
                Noise::NewValue => $macro!($self, from_fast_noise_2::Value.tileable($self.tile_width, $self.tile_height)),
                Noise::NewOpenSimplex2 => $macro!($self, from_fast_noise_2::OpenSimplex2.tileable($self.tile_width, $self.tile_height)),
                _ => None,
            }
        } else {
            match $self.noise {
                Noise::CellDistanceSq => $macro!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => $macro!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => $macro!($self, CellValue.jitter($self.jitter)),
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
                Noise::Perlin => $macro!($self, Perlin),
                Noise::ValueCubic => $macro!($self, ValueCubic),
                Noise::Value => $macro!($self, Value),
                Noise::NewPerlin => $macro!($self, from_fast_noise_2::Perlin),
                Noise::NewValue => $macro!($self, from_fast_noise_2::Value),
                Noise::NewOpenSimplex2 => $macro!($self, from_fast_noise_2::OpenSimplex2),
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
                Noise::CellDistanceSq => $macro!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => $macro!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => $macro!($self, CellValue.jitter($self.jitter)),
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
                Noise::Perlin => $macro!($self, Perlin),
                Noise::ValueCubic => $macro!($self, ValueCubic),
                Noise::Value => $macro!($self, Value),
                Noise::NewPerlin => $macro!($self, from_fast_noise_2::Perlin),
                Noise::NewValue => $macro!($self, from_fast_noise_2::Value),
                Noise::NewOpenSimplex2 => $macro!($self, from_fast_noise_2::OpenSimplex2),
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
            Noise::NewPerlin => $macro!($self, from_fast_noise_2::Perlin),
            Noise::NewValue => $macro!($self, from_fast_noise_2::Value),
            Noise::NewOpenSimplex2 => $macro!($self, from_fast_noise_2::OpenSimplex2),
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

pub trait AnySample: Sample<2> + Sample<3> {}
impl<T> AnySample for T where T: Sample<2> + Sample<3> {}

#[cfg(feature = "nightly-simd")]
pub trait AnySampleA: Sample<2> + Sample<3> + Sample<2, f32x2> + Sample<3, f32x4> {}

#[cfg(feature = "nightly-simd")]
impl<T> AnySampleA for T where T: Sample<2> + Sample<3> + Sample<2, f32x2> + Sample<3, f32x4> {}

impl Config {
    pub fn sampler(&self) -> Option<Box<dyn AnySample>> {
        sampler3!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler_a(&self) -> Option<Box<dyn AnySampleA>> {
        sampler3!(self)
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
