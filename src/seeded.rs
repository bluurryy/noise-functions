use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Seeded<Noise> {
    pub base: Noise,
    pub seed: i32,
}

impl<Noise> Seeded<Noise> {
    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { base: self, frequency }
    }
}
