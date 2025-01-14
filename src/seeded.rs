use crate::{Frequency, Sample, SampleWithSeed, Tileable};

/// Wraps a noise with a seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seeded<Noise> {
    pub noise: Noise,
    pub seed: i32,
}

impl<Noise> Seeded<Noise> {
    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }

    #[inline(always)]
    pub const fn tileable(self, width: f32, height: f32) -> Tileable<Self> {
        Tileable::new(self, width, height)
    }
}

impl<const DIM: usize, Point, Noise> Sample<DIM, Point> for Seeded<Noise>
where
    Noise: SampleWithSeed<DIM, Point>,
{
    fn sample(&self, point: Point) -> f32 {
        self.noise.sample_with_seed(point, self.seed)
    }
}
