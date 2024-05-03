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

use noise_functions::{
    fractal::*,
    open_simplex::{Improve3, Improve3Xy, Improve3Xz},
    *,
};

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
        Default,
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
    fn sample(&self, pos: [f32; 2]) -> f32 {
        self.sampler2().sample(pos)
    }
}

impl Sample<3> for Config {
    fn sample(&self, pos: [f32; 3]) -> f32 {
        self.sampler3().sample(pos)
    }
}

macro_rules! make {
    ($self:ident, $base:expr) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: Seeded { seed: $self.seed, base: $base },
        })
    };
    ($self:ident, $base:expr, $improve:ident) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: $improve(Seeded { seed: $self.seed, base: $base }),
        })
    };
}

macro_rules! make_fbm {
    ($self:ident, $base:expr) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: Seeded {
                seed: $self.seed,
                base: Fbm {
                    base: $base,
                    octaves: $self.octaves,
                    gain: $self.gain,
                    lacunarity: $self.lacunarity,
                    fractal_bounding: fractal_bounding($self.octaves, $self.gain),
                }
                .weighted($self.weighted_strength),
            },
        })
    };
    ($self:ident, $base:expr, $improve:ident) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: $improve(Seeded {
                seed: $self.seed,
                base: Fbm {
                    base: $base,
                    octaves: $self.octaves,
                    gain: $self.gain,
                    lacunarity: $self.lacunarity,
                    fractal_bounding: fractal_bounding($self.octaves, $self.gain),
                }
                .weighted($self.weighted_strength),
            }),
        })
    };
}

macro_rules! make_ridged {
    ($self:ident, $base:expr) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: Seeded {
                seed: $self.seed,
                base: Ridged {
                    base: $base,
                    octaves: $self.octaves,
                    gain: $self.gain,
                    lacunarity: $self.lacunarity,
                    fractal_bounding: fractal_bounding($self.octaves, $self.gain),
                }
                .weighted($self.weighted_strength),
            },
        })
    };
    ($self:ident, $base:expr, $improve:ident) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: $improve(Seeded {
                seed: $self.seed,
                base: Ridged {
                    base: $base,
                    octaves: $self.octaves,
                    gain: $self.gain,
                    lacunarity: $self.lacunarity,
                    fractal_bounding: fractal_bounding($self.octaves, $self.gain),
                }
                .weighted($self.weighted_strength),
            }),
        })
    };
}

macro_rules! make_ping_pong {
    ($self:ident, $base:expr) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: Seeded {
                seed: $self.seed,
                base: PingPong {
                    base: $base,
                    octaves: $self.octaves,
                    gain: $self.gain,
                    lacunarity: $self.lacunarity,
                    fractal_bounding: fractal_bounding($self.octaves, $self.gain),
                    strength: $self.ping_pong_strength,
                }
                .weighted($self.weighted_strength),
            },
        })
    };
    ($self:ident, $base:expr, $improve:ident) => {
        Box::new(Frequency {
            frequency: $self.frequency,
            base: $improve(Seeded {
                seed: $self.seed,
                base: PingPong {
                    base: $base,
                    octaves: $self.octaves,
                    gain: $self.gain,
                    lacunarity: $self.lacunarity,
                    fractal_bounding: fractal_bounding($self.octaves, $self.gain),
                    strength: $self.ping_pong_strength,
                }
                .weighted($self.weighted_strength),
            }),
        })
    };
}

macro_rules! sampler2 {
    ($self:ident) => {
        match $self.fractal {
            Fractal::None => match $self.noise {
                Noise::CellDistanceSq => make!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => make!($self, OpenSimplex2),
                Noise::OpenSimplex2s => make!($self, OpenSimplex2s),
                Noise::Perlin => make!($self, Perlin),
                Noise::ValueCubic => make!($self, ValueCubic),
                Noise::Value => make!($self, Value),
            },
            Fractal::Fbm => match $self.noise {
                Noise::CellDistanceSq => make_fbm!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make_fbm!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make_fbm!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => make_fbm!($self, OpenSimplex2),
                Noise::OpenSimplex2s => make_fbm!($self, OpenSimplex2s),
                Noise::Perlin => make_fbm!($self, Perlin),
                Noise::ValueCubic => make_fbm!($self, ValueCubic),
                Noise::Value => make_fbm!($self, Value),
            },
            Fractal::Ridged => match $self.noise {
                Noise::CellDistanceSq => make_ridged!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make_ridged!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make_ridged!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => make_ridged!($self, OpenSimplex2),
                Noise::OpenSimplex2s => make_ridged!($self, OpenSimplex2s),
                Noise::Perlin => make_ridged!($self, Perlin),
                Noise::ValueCubic => make_ridged!($self, ValueCubic),
                Noise::Value => make_ridged!($self, Value),
            },
            Fractal::PingPong => match $self.noise {
                Noise::CellDistanceSq => make_ping_pong!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make_ping_pong!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make_ping_pong!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => make_ping_pong!($self, OpenSimplex2),
                Noise::OpenSimplex2s => make_ping_pong!($self, OpenSimplex2s),
                Noise::Perlin => make_ping_pong!($self, Perlin),
                Noise::ValueCubic => make_ping_pong!($self, ValueCubic),
                Noise::Value => make_ping_pong!($self, Value),
            },
        }
    };
}

macro_rules! sampler3 {
    ($self:ident) => {
        match $self.fractal {
            Fractal::None => match $self.noise {
                Noise::CellDistanceSq => make!($self, CellDistanceSq.jitter($self.jitter)),
                Noise::CellDistance => make!($self, CellDistance.jitter($self.jitter)),
                Noise::CellValue => make!($self, CellValue.jitter($self.jitter)),
                Noise::OpenSimplex2 => match $self.improve {
                    Improve::None => make!($self, OpenSimplex2),
                    Improve::Default => make!($self, OpenSimplex2, Improve3),
                    Improve::Xy => make!($self, OpenSimplex2, Improve3Xy),
                    Improve::Xz => make!($self, OpenSimplex2, Improve3Xz),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make!($self, OpenSimplex2s),
                    Improve::Default => make!($self, OpenSimplex2s, Improve3),
                    Improve::Xy => make!($self, OpenSimplex2s, Improve3Xy),
                    Improve::Xz => make!($self, OpenSimplex2s, Improve3Xz),
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
                    Improve::Default => make_fbm!($self, OpenSimplex2, Improve3),
                    Improve::Xy => make_fbm!($self, OpenSimplex2, Improve3Xy),
                    Improve::Xz => make_fbm!($self, OpenSimplex2, Improve3Xz),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make_fbm!($self, OpenSimplex2s),
                    Improve::Default => make_fbm!($self, OpenSimplex2s, Improve3),
                    Improve::Xy => make_fbm!($self, OpenSimplex2s, Improve3Xy),
                    Improve::Xz => make_fbm!($self, OpenSimplex2s, Improve3Xz),
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
                    Improve::Default => make_ridged!($self, OpenSimplex2, Improve3),
                    Improve::Xy => make_ridged!($self, OpenSimplex2, Improve3Xy),
                    Improve::Xz => make_ridged!($self, OpenSimplex2, Improve3Xz),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make_ridged!($self, OpenSimplex2s),
                    Improve::Default => make_ridged!($self, OpenSimplex2s, Improve3),
                    Improve::Xy => make_ridged!($self, OpenSimplex2s, Improve3Xy),
                    Improve::Xz => make_ridged!($self, OpenSimplex2s, Improve3Xz),
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
                    Improve::Default => make_ping_pong!($self, OpenSimplex2, Improve3),
                    Improve::Xy => make_ping_pong!($self, OpenSimplex2, Improve3Xy),
                    Improve::Xz => make_ping_pong!($self, OpenSimplex2, Improve3Xz),
                },
                Noise::OpenSimplex2s => match $self.improve {
                    Improve::None => make_ping_pong!($self, OpenSimplex2s),
                    Improve::Default => make_ping_pong!($self, OpenSimplex2s, Improve3),
                    Improve::Xy => make_ping_pong!($self, OpenSimplex2s, Improve3Xy),
                    Improve::Xz => make_ping_pong!($self, OpenSimplex2s, Improve3Xz),
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
        sampler2!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler2a(&self) -> Box<dyn Sample<2, f32x2>> {
        sampler2!(self)
    }

    pub fn sampler3(&self) -> Box<dyn Sample<3>> {
        sampler3!(self)
    }

    #[cfg(feature = "nightly-simd")]
    pub fn sampler3a(&self) -> Box<dyn Sample<3, f32x4>> {
        sampler3!(self)
    }
}
