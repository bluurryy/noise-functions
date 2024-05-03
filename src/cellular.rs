use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Jitter<CellularNoise> {
    pub noise: CellularNoise,
    pub jitter: f32,
}

impl<CellularNoise> Jitter<CellularNoise> {
    impl_modifiers!();
}
