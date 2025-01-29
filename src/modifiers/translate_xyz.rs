use core::ops::IndexMut;

use crate::{Noise, Sample};

/// Translates the point before it is being sampled by the base noise.
pub struct TranslateXyz<Noise, X, Y, Z> {
    pub noise: Noise,
    pub x: X,
    pub y: Y,
    pub z: Z,
}

impl<N, X, Y, Z> Noise for TranslateXyz<N, X, Y, Z> {}

impl<const DIM: usize, Point, N, X, Y, Z> Sample<DIM, Point> for TranslateXyz<N, X, Y, Z>
where
    Point: IndexMut<usize, Output = f32> + Copy,
    N: Sample<DIM, Point>,
    X: Sample<DIM, Point>,
    Y: Sample<DIM, Point>,
    Z: Sample<DIM, Point>,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        let mut translated = point;
        translated[0] += self.x.sample_with_seed(point, seed);
        translated[1] += self.y.sample_with_seed(point, seed);
        translated[2] += self.z.sample_with_seed(point, seed);
        self.noise.sample_with_seed(translated, seed)
    }
}
