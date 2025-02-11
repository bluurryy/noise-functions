use crate::Noise;

/// Trait for sampling noises.
pub trait Sample<const DIM: usize, Point = [f32; DIM]>: Noise {
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32;
}

impl<const DIM: usize, Point, N> Sample<DIM, Point> for &N
where
    N: Sample<DIM, Point>,
{
    #[inline(always)]
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        N::sample_with_seed(self, point, seed)
    }
}

#[cfg(feature = "alloc")]
impl<const DIM: usize, Point> Sample<DIM, Point> for alloc::boxed::Box<dyn Sample<DIM, Point>>
where
    Self: Noise,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        (**self).sample_with_seed(point, seed)
    }
}
