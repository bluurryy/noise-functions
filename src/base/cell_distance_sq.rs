use crate::{base::impl_noise, cellular::DistanceFn, from_fast_noise_lite::cell_distance_euclidean_squared};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

use super::CustomCellDistance;

/// 2/3/4 dimensional noise of the squared distance to the closest cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellDistanceSq {
    pub jitter: f32,
}

impl CellDistanceSq {
    pub const fn jitter(mut self, jitter: f32) -> Self {
        self.jitter = jitter;
        self
    }
}

impl Default for CellDistanceSq {
    fn default() -> Self {
        Self { jitter: 1.0 }
    }
}

impl_noise!(234 CellDistanceSq);

impl CellDistanceSq {
    #[inline]
    fn gen2(self, point: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        cell_distance_euclidean_squared::gen2(self.jitter, point, seed) - 1.0
    }

    #[inline]
    fn gen3(self, point: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        cell_distance_euclidean_squared::gen3(self.jitter, point, seed) - 1.0
    }

    #[inline]
    fn gen4(self, point: [f32; 4], seed: i32) -> f32 {
        CustomCellDistance::default().jitter(self.jitter).distance_fn(DistanceFn::EuclideanSquared).gen4(point, seed) - 1.0
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        // implementation from FastNoiseLite
        cell_distance_euclidean_squared::gen2a(self.jitter, point, seed) - 1.0
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        // implementation from FastNoiseLite
        cell_distance_euclidean_squared::gen3a(self.jitter, point, seed) - 1.0
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen4a(self, point: f32x4, seed: i32) -> f32 {
        CustomCellDistance::default().jitter(self.jitter).distance_fn(DistanceFn::EuclideanSquared).gen4a(point, seed) - 1.0
    }
}
