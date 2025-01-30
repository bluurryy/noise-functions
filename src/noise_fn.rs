use crate::{Noise, Sample};

/// Wraps a function to make it implement [`Sample`].
///
/// The function is expected to take one parameter for the point and optionally a seed parameter.
pub struct NoiseFn<F, const WITH_SEED: bool>(pub F);

impl<F, const WITH_SEED: bool> Noise for NoiseFn<F, WITH_SEED> {}

impl<const DIM: usize, Point, F> Sample<DIM, Point> for NoiseFn<F, false>
where
    F: Fn(Point) -> f32,
{
    fn sample_with_seed(&self, point: Point, _seed: i32) -> f32 {
        self.0(point)
    }
}

impl<const DIM: usize, Point, F> Sample<DIM, Point> for NoiseFn<F, true>
where
    F: Fn(Point, i32) -> f32,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        self.0(point, seed)
    }
}
