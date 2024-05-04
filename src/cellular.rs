use crate::private_prelude::*;

/// Wraps a cellular noise to multiply its cell jitter.
///
/// This applies for [`CellValue`], [`CellDistance`] or [`CellDistanceSq`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Jitter<CellularNoise> {
    pub noise: CellularNoise,
    pub jitter: f32,
}

impl<CellularNoise> Jitter<CellularNoise> {
    impl_modifiers!();
}

macro_rules! cellular {
    (
        $(#[$attr:meta])*
        $noise:ident in $noise_mod:ident
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $noise;

        impl $noise {
            /// Multiplies jitter by the provided value.
            pub const fn jitter(self, jitter: f32) -> Jitter<Self> {
                Jitter { noise: self, jitter }
            }
        }

        impl $noise {
            impl_modifiers!();
        }

        impl Sample<2> for $noise {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, 0, DEFAULT_JITTER_2D)
            }
        }

        impl Sample<3> for $noise {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, 0, DEFAULT_JITTER_3D)
            }
        }

        impl Sample<2> for Jitter<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, 0, self.jitter)
            }
        }

        impl Sample<3> for Jitter<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, 0, self.jitter)
            }
        }

        impl Sample<2> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, 0, DEFAULT_JITTER_2D)
            }
        }

        impl Sample<3> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, 0, DEFAULT_JITTER_3D)
            }
        }

        impl Sample<2> for Seeded<Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
            }
        }

        impl Sample<3> for Seeded<Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
            }
        }

        impl Sample<2> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, 0, DEFAULT_JITTER_2D)
            }
        }

        impl Sample<3> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, 0, DEFAULT_JITTER_3D)
            }
        }

        impl Sample<2> for Seeded<&Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                crate::scalar::$noise_mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
            }
        }

        impl Sample<3> for Seeded<&Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                crate::scalar::$noise_mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for $noise {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, 0, DEFAULT_JITTER_2D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for $noise {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, 0, DEFAULT_JITTER_3D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Jitter<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, 0, self.jitter * DEFAULT_JITTER_2D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Jitter<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, 0, self.jitter * DEFAULT_JITTER_3D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, 0, DEFAULT_JITTER_2D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, 0, DEFAULT_JITTER_3D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, 0, DEFAULT_JITTER_2D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<&$noise> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, 0, DEFAULT_JITTER_3D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<2, f32x2> for Seeded<&Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x2) -> f32 {
                crate::simd::$noise_mod::gen2(point, self.seed, self.noise.jitter * DEFAULT_JITTER_2D)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl Sample<3, f32x4> for Seeded<&Jitter<$noise>> {
            #[inline(always)]
            fn sample(&self, point: f32x4) -> f32 {
                crate::simd::$noise_mod::gen3(point, self.seed, self.noise.jitter * DEFAULT_JITTER_3D)
            }
        }
    };
}

cellular! {
    /// 2/3 dimensional noise of the distance to the closest cell
    CellDistance in cell_distance
}
cellular! {
    /// 2/3 dimensional noise of the squared distance to the closest cell
    CellDistanceSq in cell_distance_sq
}
cellular! {
    /// 2/3 dimensional noise of the value of the closest cell
    CellValue in cell_value
}
