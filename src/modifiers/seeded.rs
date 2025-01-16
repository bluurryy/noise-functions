use crate::{Noise, Sample, SampleWithSeed};

/// Wraps a noise with a seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seeded<Noise> {
    pub noise: Noise,
    pub seed: i32,
}

impl<N> Noise for Seeded<N> {}

impl<const DIM: usize, Point, Noise> Sample<DIM, Point> for Seeded<Noise>
where
    Noise: SampleWithSeed<DIM, Point>,
{
    fn sample(&self, point: Point) -> f32 {
        self.noise.sample_with_seed(point, self.seed)
    }
}
