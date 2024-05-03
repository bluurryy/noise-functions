use crate::private_prelude::*;

/// Wraps a cellular noise to multiply its cell jitter.
///
/// This applies for [`CellValue`], [`CellDistance`] or [`CellDistanceSq`].
#[derive(Debug, Clone, Copy)]
pub struct Jitter<CellularNoise> {
    pub noise: CellularNoise,
    pub jitter: f32,
}

impl<CellularNoise> Jitter<CellularNoise> {
    impl_modifiers!();
}
