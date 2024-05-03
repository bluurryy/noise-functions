use crate::private_prelude::*;

/// Wraps a function to make it implement [`Sample`].
///
/// The function is expected to take one parameter for the position and optionally
/// a seed parameter.
///
/// With a seed parameter it can be used for fractals:
///
/// ```rust
/// use noise_functions::{ NoiseFn, Sample2, OpenSimplex2s };
///
/// let warped = NoiseFn(|pos: [f32; 2], seed: i32| {
///     let warp_x = OpenSimplex2s.seed(seed + 100).sample2(pos);
///     let warp_y = OpenSimplex2s.seed(seed + 200).sample2(pos);
///     let warped = [pos[0] + warp_x, pos[1] + warp_y];
///     OpenSimplex2s.sample2(warped)
/// });
///
/// let warped_fbm = warped.fbm(3, 0.5, 2.0);
///
/// let value = warped_fbm.sample2([1.0, 2.0]);
/// ```
pub struct NoiseFn<F>(pub F);

impl<F> NoiseFn<F> {
    impl_modifiers!();
}

impl<const DIM: usize, Pos, F> Sample<DIM, Pos> for NoiseFn<F>
where
    F: Fn(Pos) -> f32,
{
    fn sample(&self, pos: Pos) -> f32 {
        self.0(pos)
    }
}

impl<const DIM: usize, Pos, F> Sample<DIM, Pos> for Seeded<NoiseFn<F>>
where
    F: Fn(Pos, i32) -> f32,
{
    fn sample(&self, pos: Pos) -> f32 {
        let &Seeded { ref noise, seed } = self;
        noise.0(pos, seed)
    }
}

impl<const DIM: usize, Pos, F> Sample<DIM, Pos> for Seeded<&NoiseFn<F>>
where
    F: Fn(Pos, i32) -> f32,
{
    fn sample(&self, pos: Pos) -> f32 {
        let &Seeded { noise, seed } = self;
        noise.0(pos, seed)
    }
}
