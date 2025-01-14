use crate::{base::impl_noise, from_fast_noise_lite::cell_distance_euclidean_squared, math::sqrt};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

use super::CellDistance;

/// 2/3/4 dimensional noise of the distance to the closest cell
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastCellDistance {
    pub jitter: f32,
}

impl FastCellDistance {
    pub const fn jitter(mut self, jitter: f32) -> Self {
        self.jitter = jitter;
        self
    }
}

impl_noise!(234 FastCellDistance);

impl Default for FastCellDistance {
    fn default() -> Self {
        Self { jitter: 1.0 }
    }
}

impl FastCellDistance {
    #[inline]
    fn gen2(self, point: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        sqrt(cell_distance_euclidean_squared::gen2(self.jitter, point, seed)) - 1.0
    }

    #[inline]
    fn gen3(self, point: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        sqrt(cell_distance_euclidean_squared::gen3(self.jitter, point, seed)) - 1.0
    }

    #[inline]
    fn gen4(self, point: [f32; 4], seed: i32) -> f32 {
        CellDistance::default().jitter(self.jitter).gen4(point, seed)
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        // implementation from FastNoiseLite
        sqrt(cell_distance_euclidean_squared::gen2a(self.jitter, point, seed)) - 1.0
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        // implementation from FastNoiseLite
        sqrt(cell_distance_euclidean_squared::gen3a(self.jitter, point, seed)) - 1.0
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen4a(self, point: f32x4, seed: i32) -> f32 {
        CellDistance::default().jitter(self.jitter).gen4a(point, seed)
    }
}
