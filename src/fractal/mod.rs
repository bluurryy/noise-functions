mod fbm;
mod ping_pong;
mod ridged;

pub use fbm::Fbm;
pub use ping_pong::PingPong;
pub use ridged::Ridged;

use crate::{Frequency, Seeded};

/// Wraps a fractal noise to
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weighted<Fractal> {
    pub fractal: Fractal,
    pub strength: f32,
}

impl<Fractal> Weighted<Fractal> {
    #[inline(always)]
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { noise: self, seed }
    }

    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }
}

/// Calculates the `fractal_bounding` property for [`Fbm`], [`Ridged`] and [`PingPong`].
///
#[inline(always)]
pub const fn fractal_bounding(octaves: u32, gain: f32) -> f32 {
    let gain = fast_abs(gain);
    let mut amp = gain;
    let mut amp_fractal = 1.0;
    let mut i = 0;

    while i < octaves {
        amp_fractal += amp;
        amp *= gain;
        i += 1;
    }

    1.0 / amp_fractal
}

#[inline(always)]
const fn fast_abs(f: f32) -> f32 {
    if f < 0.0 {
        -f
    } else {
        f
    }
}
