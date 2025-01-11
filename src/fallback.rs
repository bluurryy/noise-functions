use crate::{Sample, Seeded};

#[cfg(feature = "nightly-simd")]
use crate::array_4_take_3;

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Fallback;

impl Sample<2> for Fallback {
    fn sample(&self, point: [f32; 2]) -> f32 {
        gen(&point, 0)
    }
}

impl Sample<2> for Seeded<Fallback> {
    fn sample(&self, point: [f32; 2]) -> f32 {
        gen(&point, self.seed)
    }
}

impl Sample<2> for Seeded<&Fallback> {
    fn sample(&self, point: [f32; 2]) -> f32 {
        gen(&point, self.seed)
    }
}

impl Sample<3> for Fallback {
    fn sample(&self, point: [f32; 3]) -> f32 {
        gen(&point, 0)
    }
}

impl Sample<3> for Seeded<Fallback> {
    fn sample(&self, point: [f32; 3]) -> f32 {
        gen(&point, self.seed)
    }
}

impl Sample<3> for Seeded<&Fallback> {
    fn sample(&self, point: [f32; 3]) -> f32 {
        gen(&point, self.seed)
    }
}

impl Sample<4> for Fallback {
    fn sample(&self, point: [f32; 4]) -> f32 {
        gen(&point, 0)
    }
}

impl Sample<4> for Seeded<Fallback> {
    fn sample(&self, point: [f32; 4]) -> f32 {
        gen(&point, self.seed)
    }
}

impl Sample<4> for Seeded<&Fallback> {
    fn sample(&self, point: [f32; 4]) -> f32 {
        gen(&point, self.seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<2, f32x2> for Fallback {
    fn sample(&self, point: f32x2) -> f32 {
        gen(point.as_array(), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<2, f32x2> for Seeded<Fallback> {
    fn sample(&self, point: f32x2) -> f32 {
        gen(point.as_array(), self.seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<2, f32x2> for Seeded<&Fallback> {
    fn sample(&self, point: f32x2) -> f32 {
        gen(point.as_array(), self.seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<3, f32x4> for Fallback {
    fn sample(&self, point: f32x4) -> f32 {
        gen(array_4_take_3(point.as_array()), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<3, f32x4> for Seeded<Fallback> {
    fn sample(&self, point: f32x4) -> f32 {
        gen(array_4_take_3(point.as_array()), self.seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<3, f32x4> for Seeded<&Fallback> {
    fn sample(&self, point: f32x4) -> f32 {
        gen(array_4_take_3(point.as_array()), self.seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<4, f32x4> for Fallback {
    fn sample(&self, point: f32x4) -> f32 {
        gen(point.as_array(), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<4, f32x4> for Seeded<Fallback> {
    fn sample(&self, point: f32x4) -> f32 {
        gen(point.as_array(), self.seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<4, f32x4> for Seeded<&Fallback> {
    fn sample(&self, point: f32x4) -> f32 {
        gen(point.as_array(), self.seed)
    }
}

fn gen(slice: &[f32], seed: i32) -> f32 {
    let shift = seed as f32 * 0.01;
    let multiplier = 1.0 / slice.len() as f32;
    slice.iter().map(|v| (v + shift % 0.1) * multiplier).sum()
}
