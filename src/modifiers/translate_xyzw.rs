use core::ops::IndexMut;

use crate::{Noise, Sample};

/// Translates the point before it is being sampled by the base noise.
pub struct TranslateXyzw<Noise, X, Y, Z, W> {
    pub noise: Noise,
    pub x: X,
    pub y: Y,
    pub z: Z,
    pub w: W,
}

impl<N, X, Y, Z, W> Noise for TranslateXyzw<N, X, Y, Z, W> {}

impl<const DIM: usize, Point, N, X, Y, Z, W> Sample<DIM, Point> for TranslateXyzw<N, X, Y, Z, W>
where
    Point: IndexMut<usize, Output = f32> + Copy,
    N: Sample<DIM, Point>,
    X: Sample<DIM, Point>,
    Y: Sample<DIM, Point>,
    Z: Sample<DIM, Point>,
    W: Sample<DIM, Point>,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        let mut translated = point;

        if DIM > 0 {
            translated[0] += self.x.sample_with_seed(point, seed);
        }

        if DIM > 1 {
            translated[1] += self.y.sample_with_seed(point, seed);
        }

        if DIM > 2 {
            translated[2] += self.z.sample_with_seed(point, seed);
        }

        if DIM > 3 {
            translated[3] += self.w.sample_with_seed(point, seed);
        }

        self.noise.sample_with_seed(translated, seed)
    }
}
