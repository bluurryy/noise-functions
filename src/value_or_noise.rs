use crate::{Constant, Noise};

/// Either a `f32` or a noise.
///
/// Used for [`Noise`] modifier methods.
pub trait ValueOrNoise {
    type Noise;
    fn into_noise(self) -> Self::Noise;
}

impl ValueOrNoise for f32 {
    type Noise = Constant;

    fn into_noise(self) -> Self::Noise {
        Constant(self)
    }
}

impl<N: Noise> ValueOrNoise for N {
    type Noise = Self;

    fn into_noise(self) -> Self::Noise {
        self
    }
}
