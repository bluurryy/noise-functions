use crate::{
    math::{cos, sin},
    Noise, Sample,
};

use core::f32::consts::{PI, TAU};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

/// Creates a tileable 2D noise from a 4D noise.
///
/// The parameters `width` and `height` describe the size of the repeating tile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tileable<Noise> {
    pub noise: Noise,
    width_div_pi: f32,
    height_div_pi: f32,
    tau_div_width: f32,
    tau_div_height: f32,
}

impl<Noise> Tileable<Noise> {
    pub const fn new(noise: Noise, width: f32, height: f32) -> Self {
        Self {
            noise,
            width_div_pi: width / PI,
            height_div_pi: height / PI,
            tau_div_width: TAU / width,
            tau_div_height: TAU / height,
        }
    }

    fn map_point(&self, [x, y]: [f32; 2]) -> [f32; 4] {
        let nx = cos(x * self.tau_div_width) * self.width_div_pi;
        let ny = cos(y * self.tau_div_height) * self.height_div_pi;
        let nz = sin(x * self.tau_div_width) * self.width_div_pi;
        let nw = sin(y * self.tau_div_height) * self.height_div_pi;
        [nx, ny, nz, nw]
    }
}

impl<N> Noise for Tileable<N> {}

impl<Noise> Sample<2> for Tileable<Noise>
where
    Noise: Sample<4>,
{
    fn sample_with_seed(&self, point: [f32; 2], seed: i32) -> f32 {
        self.noise.sample_with_seed(self.map_point(point), seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise> Sample<2, f32x2> for Tileable<Noise>
where
    Noise: Sample<4, f32x4>,
{
    fn sample_with_seed(&self, point: f32x2, seed: i32) -> f32 {
        self.noise.sample_with_seed(self.map_point(point.into()).into(), seed)
    }
}
