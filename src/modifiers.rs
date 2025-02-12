mod abs;
mod add;
mod add_seed;
mod ceil;
mod clamp;
mod div;
mod fbm;
mod floor;
mod frequency;
mod lerp;
mod map;
mod max;
mod min;
mod mul;
mod mul_seed;
mod neg;
pub mod open_simplex_2;
mod pow;
mod rem;
mod ridged;
mod round;
mod seeded;
mod sqrt;
mod sub;
mod tileable;
mod translate_x;
mod translate_xy;
mod translate_xyz;
mod translate_xyzw;
mod triangle_wave;

pub use abs::Abs;
pub use add::Add;
pub use add_seed::AddSeed;
pub use ceil::Ceil;
pub use clamp::Clamp;
pub use div::Div;
pub use fbm::Fbm;
pub use floor::Floor;
pub use frequency::Frequency;
pub use lerp::Lerp;
pub use map::Map;
pub use max::Max;
pub use min::Min;
pub use mul::Mul;
pub use mul_seed::MulSeed;
pub use neg::Neg;
pub use pow::Pow;
pub use rem::Rem;
pub use ridged::Ridged;
pub use round::Round;
pub use seeded::Seeded;
pub use sqrt::Sqrt;
pub use sub::Sub;
pub use tileable::Tileable;
pub use translate_x::TranslateX;
pub use translate_xy::TranslateXy;
pub use translate_xyz::TranslateXyz;
pub use translate_xyzw::TranslateXyzw;
pub use triangle_wave::TriangleWave;

use crate::Noise;

/// Modifies a fractal noise to make successive octaves have less impact the lower the output value of the previous one was.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weighted<Fractal> {
    pub fractal: Fractal,
    pub strength: f32,
}

impl<Fractal> Noise for Weighted<Fractal> {}

/// Calculates the `fractal_bounding` property for [`Fbm`].
///
#[inline(always)]
pub const fn fractal_bounding(octaves: u32, gain: f32) -> f32 {
    let gain = crate::math::const_abs(gain);
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
