use core::ops::IndexMut;

use crate::{Noise, Sample};

/// Translates the point before it is being sampled by the base noise.
pub struct TranslateXy<Noise, X, Y> {
    pub noise: Noise,
    pub x: X,
    pub y: Y,
}

impl<N, X, Y> Noise for TranslateXy<N, X, Y> {}

impl<const DIM: usize, Point, N, X, Y> Sample<DIM, Point> for TranslateXy<N, X, Y>
where
    Point: IndexMut<usize, Output = f32> + Copy,
    N: Sample<DIM, Point>,
    X: Sample<DIM, Point>,
    Y: Sample<DIM, Point>,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        let mut translated = point;

        if DIM > 0 {
            translated[0] += self.x.sample_with_seed(point, seed);
        }

        if DIM > 1 {
            translated[1] += self.y.sample_with_seed(point, seed);
        }

        self.noise.sample_with_seed(translated, seed)
    }
}
