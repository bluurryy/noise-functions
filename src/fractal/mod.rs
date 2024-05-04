mod fbm;
mod ping_pong;
mod ridged;

pub use fbm::Fbm;
pub use ping_pong::PingPong;
pub use ridged::Ridged;

pub use crate::util::fractal_bounding;

use crate::private_prelude::*;

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
