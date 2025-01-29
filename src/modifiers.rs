mod add_seed;
mod fbm;
mod frequency;
mod mul_seed;
pub mod open_simplex_2;
mod ridged;
mod seeded;
mod tileable;
mod translate_x;
mod translate_xy;
mod translate_xyz;
mod translate_xyzw;
mod triangle_wave;

pub use add_seed::AddSeed;
pub use fbm::Fbm;
pub use frequency::Frequency;
pub use mul_seed::MulSeed;
pub use ridged::Ridged;
pub use seeded::Seeded;
pub use tileable::Tileable;
pub use translate_x::TranslateX;
pub use translate_xy::TranslateXy;
pub use translate_xyz::TranslateXyz;
pub use translate_xyzw::TranslateXyzw;
pub use triangle_wave::TriangleWave;

use crate::{
    math::{abs, const_abs},
    Noise,
};

/// Modifies a fractal noise to make successive octaves have less impact the lower the output value of the previous one was.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weighted<Fractal> {
    pub fractal: Fractal,
    pub strength: f32,
}

impl<Fractal> Noise for Weighted<Fractal> {}

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

use crate::math::floor;

/// Calculates the `fractal_bounding` property for [`Fbm`].
///
#[inline(always)]
pub const fn fractal_bounding(octaves: u32, gain: f32) -> f32 {
    let gain = const_abs(gain);
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
