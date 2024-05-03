mod fbm;
mod ping_pong;
mod ridged;

pub use fbm::Fbm;
pub use ping_pong::PingPong;
pub use ridged::Ridged;

pub use crate::util::fractal_bounding;

use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Weighted<Fractal> {
    pub fractal: Fractal,
    pub strength: f32,
}

impl<Fractal> Weighted<Fractal> {
    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }
}
