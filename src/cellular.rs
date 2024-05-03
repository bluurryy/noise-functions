use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Jitter<CellularNoise> {
    pub noise: CellularNoise,
    pub jitter: f32,
}

impl<CellularNoise> Jitter<CellularNoise> {
    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }
}
