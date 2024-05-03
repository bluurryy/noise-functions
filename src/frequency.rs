use crate::private_prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Frequency<Noise> {
    pub noise: Noise,
    pub frequency: f32,
}

impl<const DIM: usize, Noise> Sample<DIM, [f32; DIM]> for Frequency<Noise>
where
    Noise: Sample<DIM, [f32; DIM]>,
{
    fn sample(&self, mut point: [f32; DIM]) -> f32 {
        let frequency = self.frequency;

        for x in &mut point {
            *x *= frequency;
        }

        self.noise.sample(point)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const DIM: usize, const LANES: usize, Noise> Sample<DIM, Simd<f32, LANES>> for Frequency<Noise>
where
    Noise: Sample<DIM, Simd<f32, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    fn sample(&self, mut point: Simd<f32, LANES>) -> f32 {
        point *= splat(self.frequency);
        self.noise.sample(point)
    }
}
