use crate::{Noise, Sample};

/// Wraps a noise with a seed.
///
/// This structs' [`Sample`] implementation will call [`sample_with_seed`] on the base noise with `self.seed` overwriting the `seed` provided by the callee.
///
/// [`sample_with_seed`]: Sample::sample_with_seed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seeded<Noise> {
    pub noise: Noise,
    pub seed: i32,
}

impl<N> Noise for Seeded<N> {}

impl<const DIM: usize, Point, Noise> Sample<DIM, Point> for Seeded<Noise>
where
    Noise: Sample<DIM, Point>,
{
    fn sample_with_seed(&self, point: Point, _seed: i32) -> f32 {
        self.noise.sample_with_seed(point, self.seed)
    }
}
