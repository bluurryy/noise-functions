use crate::{impl_modifier_methods, impl_modifier_methods_tileable, Sample, SampleWithSeed};

/// Wraps a function to make it implement [`Sample`].
///
/// The function is expected to take one parameter for the point and optionally a seed parameter.
///
/// With a seed parameter it can be used for fractals:
///
/// ```rust
/// use noise_functions::{ NoiseFn, Sample2, OpenSimplex2s };
///
/// let warped = NoiseFn(|point: [f32; 2], seed: i32| {
///     let warp_x = OpenSimplex2s.seed(seed + 100).sample2(point);
///     let warp_y = OpenSimplex2s.seed(seed + 200).sample2(point);
///     let warped = [point[0] + warp_x, point[1] + warp_y];
///     OpenSimplex2s.sample2(warped)
/// });
///
/// let warped_fbm = warped.fbm(3, 0.5, 2.0);
///
/// let value = warped_fbm.sample2([1.0, 2.0]);
/// ```
pub struct NoiseFn<F, const WITH_SEED: bool>(pub F);

impl<F, const WITH_SEED: bool> NoiseFn<F, WITH_SEED> {
    impl_modifier_methods!();
    impl_modifier_methods_tileable!();
}

impl<const DIM: usize, Point, F> Sample<DIM, Point> for NoiseFn<F, false>
where
    F: Fn(Point) -> f32,
{
    fn sample(&self, point: Point) -> f32 {
        self.0(point)
    }
}

impl<const DIM: usize, Point, F> Sample<DIM, Point> for NoiseFn<F, true>
where
    F: Fn(Point, i32) -> f32,
{
    fn sample(&self, point: Point) -> f32 {
        self.0(point, 0)
    }
}

impl<const DIM: usize, Point, F> SampleWithSeed<DIM, Point> for NoiseFn<F, true>
where
    F: Fn(Point, i32) -> f32,
{
    fn sample_with_seed(&self, point: Point, seed: i32) -> f32 {
        self.0(point, seed)
    }
}
