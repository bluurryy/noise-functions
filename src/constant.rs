use crate::{Noise, Sample};

/// Returns a constant value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Constant(pub f32);

impl Noise for Constant {}

impl<const DIM: usize, T> Sample<DIM, T> for Constant {
    fn sample_with_seed(&self, _point: T, _seed: i32) -> f32 {
        self.0
    }
}
