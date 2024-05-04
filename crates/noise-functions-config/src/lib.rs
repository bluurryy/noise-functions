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
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide), doc(cfg_hide(no_global_oom_handling, feature = "nightly-const-fn-float")))]

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

#[derive(Debug)]
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
}

impl Sample<2> for Config {
    fn sample(&self, point: [f32; 2]) -> f32 {
        self.sampler2().sample(point)
    }
}

impl Sample<3> for Config {
    fn sample(&self, point: [f32; 3]) -> f32 {
        self.sampler3().sample(point)
    }
}

macro_rules! make {
    ($self:ident, $noise:expr) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            noise: Seeded { seed: $self.seed, noise: $noise },
        })
    };
}

macro_rules! make_fbm {
    ($self:ident, $noise:expr) => {
        Box::new(
            $noise
                .fbm($self.octaves, $self.gain, $self.lacunarity)
                .weighted($self.weighted_strength)
                .seed($self.seed)
                .frequency($self.frequency),
        )
    };
}

macro_rules! make_ridged {
    ($self:ident, $noise:expr) => {
        Box::new(
            $noise
                .ridged($self.octaves, $self.gain, $self.lacunarity)
                .weighted($self.weighted_strength)
                .seed($self.seed)
                .frequency($self.frequency),
        )
    };
}

macro_rules! make_ping_pong {
    ($self:ident, $noise:expr) => {
        Box::new(
            $noise
                .ping_pong($self.octaves, $self.gain, $self.lacunarity, $self.ping_pong_strength)
                .weighted($self.weighted_strength)
                .seed($self.seed)
                .frequency($self.frequency),
        )
    };
}

macro_rules! sampler {
    ($self:ident) => {
        match $self.fractal {
            Fractal::None => match $self.noise {
                Noise::CellDistanceSq => make!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => make!($self, OpenSimplex2),
                    Improve::Xy => make!($self, OpenSimplex2.improve_xy()),
                    Improve::Xz => make!($self, OpenSimplex2.improve_xz()),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make!($self, OpenSimplex2s),
                    Improve::Xy => make!($self, OpenSimplex2s.improve_xy()),
                    Improve::Xz => make!($self, OpenSimplex2s.improve_xz()),
                },
                Noise::Perlin => make!($self, Perlin),
                Noise::ValueCubic => make!($self, ValueCubic),
                Noise::Value => make!($self, Value),
            },
            Fractal::Fbm => match $self.noise {
                Noise::CellDistanceSq => make_fbm!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make_fbm!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make_fbm!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => make_fbm!($self, OpenSimplex2),
                    Improve::Xy => make_fbm!($self, OpenSimplex2.improve_xy()),
                    Improve::Xz => make_fbm!($self, OpenSimplex2.improve_xz()),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make_fbm!($self, OpenSimplex2s),
                    Improve::Xy => make_fbm!($self, OpenSimplex2s.improve_xy()),
                    Improve::Xz => make_fbm!($self, OpenSimplex2s.improve_xz()),
                },
                Noise::Perlin => make_fbm!($self, Perlin),
                Noise::ValueCubic => make_fbm!($self, ValueCubic),
                Noise::Value => make_fbm!($self, Value),
            },
            Fractal::Ridged => match $self.noise {
                Noise::CellDistanceSq => make_ridged!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make_ridged!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make_ridged!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => make_ridged!($self, OpenSimplex2),
                    Improve::Xy => make_ridged!($self, OpenSimplex2.improve_xy()),
                    Improve::Xz => make_ridged!($self, OpenSimplex2.improve_xz()),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make_ridged!($self, OpenSimplex2s),
                    Improve::Xy => make_ridged!($self, OpenSimplex2s.improve_xy()),
                    Improve::Xz => make_ridged!($self, OpenSimplex2s.improve_xz()),
                },
                Noise::Perlin => make_ridged!($self, Perlin),
                Noise::ValueCubic => make_ridged!($self, ValueCubic),
                Noise::Value => make_ridged!($self, Value),
            },
            Fractal::PingPong => match $self.noise {
                Noise::CellDistanceSq => make_ping_pong!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make_ping_pong!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make_ping_pong!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => make_ping_pong!($self, OpenSimplex2),
                    Improve::Xy => make_ping_pong!($self, OpenSimplex2.improve_xy()),
                    Improve::Xz => make_ping_pong!($self, OpenSimplex2.improve_xz()),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make_ping_pong!($self, OpenSimplex2s),
                    Improve::Xy => make_ping_pong!($self, OpenSimplex2s.improve_xy()),
                    Improve::Xz => make_ping_pong!($self, OpenSimplex2s.improve_xz()),
                },
                Noise::Perlin => make_ping_pong!($self, Perlin),
                Noise::ValueCubic => make_ping_pong!($self, ValueCubic),
                Noise::Value => make_ping_pong!($self, Value),
            },
        }
    };
}

impl Config {
    pub fn sampler2(&self) -> Box<dyn Sample<2>> {
        sampler!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler2a(&self) -> Box<dyn Sample<2, f32x2>> {
        sampler!(self)
    }

    pub fn sampler3(&self) -> Box<dyn Sample<3>> {
        sampler!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler3a(&self) -> Box<dyn Sample<3, f32x4>> {
        sampler!(self)
    }
}
