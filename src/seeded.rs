use crate::private_prelude::*;

/// Wraps a noise with a seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seeded<Noise> {
    pub noise: Noise,
    pub seed: i32,
}

impl<Noise> Seeded<Noise> {
    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }
}
