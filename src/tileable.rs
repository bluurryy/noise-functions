use crate::{cos, impl_modifiers, sin, Sample, Seeded};

use core::f32::consts::TAU;

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tileable<Noise> {
    pub noise: Noise,
    tau_div_width: f32,
    tau_div_height: f32,
}

impl<Noise> Tileable<Noise> {
    pub const fn new(noise: Noise, width: f32, height: f32) -> Self {
        Self {
            noise,
            tau_div_width: TAU / width,
            tau_div_height: TAU / height,
        }
    }

    impl_modifiers!();

    fn map_point(&self, [x, y]: [f32; 2]) -> [f32; 4] {
        let nx = cos(x * self.tau_div_height);
        let ny = cos(y * self.tau_div_width);
        let nz = sin(x * self.tau_div_height);
        let nw = sin(y * self.tau_div_width);
        [nx, ny, nz, nw]
    }
}

impl<Noise> Sample<2> for Tileable<Noise>
where
    Noise: Sample<4>,
{
    fn sample(&self, point: [f32; 2]) -> f32 {
        self.noise.sample(self.map_point(point))
    }
}

impl<Noise> Sample<2> for Seeded<Tileable<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<4>,
{
    fn sample(&self, point: [f32; 2]) -> f32 {
        Seeded {
            noise: &self.noise.noise,
            seed: self.seed,
        }
        .sample(self.noise.map_point(point))
    }
}

impl<Noise> Sample<2> for Seeded<&Tileable<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<4>,
{
    fn sample(&self, point: [f32; 2]) -> f32 {
        Seeded {
            noise: &self.noise.noise,
            seed: self.seed,
        }
        .sample(self.noise.map_point(point))
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise> Sample<2, f32x2> for Tileable<Noise>
where
    Noise: Sample<4, f32x4>,
{
    fn sample(&self, point: f32x2) -> f32 {
        self.noise.sample(self.map_point(point.into()).into())
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise> Sample<2, f32x2> for Seeded<Tileable<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<4, f32x4>,
{
    fn sample(&self, point: f32x2) -> f32 {
        Seeded {
            noise: &self.noise.noise,
            seed: self.seed,
        }
        .sample(self.noise.map_point(point.into()).into())
    }
}

#[cfg(feature = "nightly-simd")]
impl<Noise> Sample<2, f32x2> for Seeded<&Tileable<Noise>>
where
    for<'a> Seeded<&'a Noise>: Sample<4, f32x4>,
{
    fn sample(&self, point: f32x2) -> f32 {
        Seeded {
            noise: &self.noise.noise,
            seed: self.seed,
        }
        .sample(self.noise.map_point(point.into()).into())
    }
}
