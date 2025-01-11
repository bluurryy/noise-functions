use crate::{cos, impl_modifiers, sin, Sample, Seeded};

use core::f32::consts::TAU;

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tileable<Noise> {
    pub noise: Noise,
    pub width: f32,
    pub height: f32,
    pub inv_width: f32,
    pub inv_height: f32,
}

impl<Noise> Tileable<Noise> {
    impl_modifiers!();

    fn map_point(&self, [x, y]: [f32; 2]) -> [f32; 4] {
        let s = x * self.inv_height;
        let t = y * self.inv_width;

        let nx = cos(s * TAU) * self.height;
        let ny = cos(t * TAU) * self.width;
        let nz = sin(s * TAU) * self.height;
        let nw = sin(t * TAU) * self.width;

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
