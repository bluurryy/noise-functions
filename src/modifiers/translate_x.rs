use core::ops::IndexMut;

use crate::{Noise, Sample};

/// Translates the point before it is being sampled by the base noise.
pub struct TranslateX<Noise, X> {
    pub noise: Noise,
    pub x: X,
}

impl<N, X> Noise for TranslateX<N, X> {}

impl<const DIM: usize, Point, N, X> Sample<DIM, Point> for TranslateX<N, X>
where
    Point: IndexMut<usize, Output = f32> + Copy,
    N: Sample<DIM, Point>,
    X: Sample<DIM, Point>,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        let mut translated = point;
        translated[0] += self.x.sample_with_seed(point, seed);
        self.noise.sample_with_seed(translated, seed)
    }
}
