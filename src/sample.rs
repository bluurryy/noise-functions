#[cfg(feature = "nightly-simd")]
use crate::private_prelude::*;

pub trait Sample<const DIM: usize, Pos = [f32; DIM]> {
    fn sample(&self, pos: Pos) -> f32;
}

impl<const DIM: usize, Pos, Noise> Sample<DIM, Pos> for &Noise
where
    Noise: Sample<DIM, Pos>,
{
    #[inline(always)]
    fn sample(&self, pos: Pos) -> f32 {
        Noise::sample(self, pos)
    }
}

macro_rules! helper_trait {
	($(#[$attr:meta])* $trait:ident, $fn:ident, $dim:literal as $ty:ty) => {
		#[doc = concat!(
			"Helper trait that provides `",
			stringify!($fn),
			"` as a shorthand for `Sample<",
			stringify!($dim),
			", ",
			stringify!($ty),
			">::sample`.",
		)]
		///
		#[doc = concat!(
			"It also works for any `impl Into<",
			stringify!($ty),
			">`.",
		)]
		$(#[$attr])*
		pub trait $trait {
			fn $fn(&self, pos: impl Into<$ty>) -> f32;
		}

		$(#[$attr])*
		impl<Noise> $trait for Noise
		where
			Noise: Sample<$dim, $ty>,
		{
			#[inline(always)]
			fn $fn(&self, pos: impl Into<$ty>) -> f32 {
				Noise::sample(self, pos.into())
			}
		}
	};
}

helper_trait!(Sample2, sample2, 2 as [f32; 2]);
helper_trait!(Sample3, sample3, 3 as [f32; 3]);
helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample2a,
    sample2a,
    2 as f32x2
);
helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample3a,
    sample3a,
    3 as f32x4
);
