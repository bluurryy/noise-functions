#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

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

macro_rules! helper_trait {
	($(#[$attr:meta])* $trait:ident, $fn:ident, $dim:literal as $ty:ty $(as $ty_param:ty)?) => {
		#[doc = concat!(
			"Helper trait that provides `",
			stringify!($fn),
			"` for every `Sample<",
			stringify!($dim),
			$(", ", stringify!($ty_param),)?
			">`.",
		)]
		///
		#[doc = concat!(
			"It also works for any `impl Into<",
			stringify!($ty),
			">`.",
		)]
		$(#[$attr])*
		pub trait $trait: Sample<$dim $(, $ty_param)?> {
			fn $fn(&self, point: impl Into<$ty>) -> f32;
		}

		$(#[$attr])*
		impl<N> $trait for N
		where
			N: Sample<$dim $(, $ty_param)?>,
		{
			#[inline(always)]
			fn $fn(&self, point: impl Into<$ty>) -> f32 {
				N::sample_with_seed(self, point.into(), 0)
			}
		}
	};
}

helper_trait!(Sample2, sample2, 2 as [f32; 2]);
helper_trait!(Sample3, sample3, 3 as [f32; 3]);
helper_trait!(Sample4, sample4, 4 as [f32; 4]);

helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample2a,
    sample2a,
    2 as f32x2 as f32x2
);

helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample3a,
    sample3a,
    3 as f32x4 as f32x4
);

helper_trait!(
    #[cfg(feature = "nightly-simd")]
    Sample4a,
    sample4a,
    4 as f32x4 as f32x4
);
