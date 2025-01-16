mod fbm;
mod frequency;
pub mod open_simplex_2;
mod seeded;
mod tileable;

pub use fbm::Fbm;
pub use frequency::Frequency;
pub use seeded::Seeded;
pub use tileable::Tileable;

/// Modifies a fractal noise to make successive octaves have less impact the lower the output value of the previous one was.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weighted<Fractal> {
    pub fractal: Fractal,
    pub strength: f32,
}

impl<Fractal> Weighted<Fractal> {
    #[inline(always)]
    pub const fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { noise: self, seed }
    }

    #[inline(always)]
    pub const fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }
}

macro_rules! or_else {
    ({ } else { $($else:tt)* }) => { $($else)* };
    ({ $($tokens:tt)+ } else { $($else:tt)* }) => { $($tokens)* };
}

macro_rules! modifier_map {
    (
        $(#[$attr:meta])*
        pub struct $struct:ident {
            $($fields:tt)*
        }

        $self:ident, $seed:ident, $point:ident, $value:ident;
        $(map_seed: { $($map_seed:tt)* })?
        $(map_value: { $($map_value:tt)* })?
    ) => {
        $(#[$attr])*
        pub struct $struct<Noise> {
            pub noise: Noise,
            $($fields)*
        }

        const _: () = {
            use crate::{ Noise, Sample, SampleWithSeed };

            #[cfg(feature = "nightly-simd")]
            use core::simd::{ Simd, LaneCount, SupportedLaneCount };

            impl<N> Noise for $struct<N> {}

            impl<Noise, const DIM: usize> Sample<DIM, [f32; DIM]> for $struct<Noise>
            where
                Noise: SampleWithSeed<DIM, [f32; DIM]>,
            {
                #[inline]
                fn sample(&$self, $point: [f32; DIM]) -> f32 {
                    let $value = $self.noise.sample($point);
                    or_else!({$({$($map_value)*})?} else { $value })
                }
            }

            impl<Noise, const DIM: usize> SampleWithSeed<DIM, [f32; DIM]> for $struct<Noise>
            where
                Noise: SampleWithSeed<DIM, [f32; DIM]>,
            {
                #[inline]
                fn sample_with_seed(&$self, $point: [f32; DIM], $seed: i32) -> f32 {
                    let $value = $self.noise.sample_with_seed($point, or_else!({$({$($map_seed)*})?} else { $seed }));
                    or_else!({$({$($map_value)*})?} else { $value })
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<Noise, const DIM: usize, const LANES: usize> Sample<DIM, Simd<f32, LANES>> for $struct<Noise>
            where
                Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
                LaneCount<LANES>: SupportedLaneCount,
            {
                #[inline]
                fn sample(&$self, $point: Simd<f32, LANES>) -> f32 {
                    let $value = $self.noise.sample($point);
                    or_else!({$({$($map_value)*})?} else { $value })
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<Noise, const DIM: usize, const LANES: usize> SampleWithSeed<DIM, Simd<f32, LANES>> for $struct<Noise>
            where
                Noise: SampleWithSeed<DIM, Simd<f32, LANES>>,
                LaneCount<LANES>: SupportedLaneCount,
            {
                #[inline]
                fn sample_with_seed(&$self, $point: Simd<f32, LANES>, $seed: i32) -> f32 {
                    let $value = $self.noise.sample_with_seed($point, or_else!({$({$($map_seed)*})?} else { $seed }));
                    or_else!({$({$($map_value)*})?} else { $value })
                }
            }
        };
    };
}

pub(crate) use modifier_map;

use crate::math::floor;

modifier_map! {
    /// Modifies a noise to create a peak at value 0.
    ///
    /// Equal to `abs(x) * 2 - 1`.
    ///
    /// **Note:** This modifier assumes the base noise to return values in the [-1, 1] range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ridged {}

    self, seed, point, value;

    map_value: {
        fast_abs(value) * -2.0 + 1.0
    }
}

modifier_map! {
    /// Generates fractal noise and applies a triangle wave to the output of a base noise function.
    ///
    /// The output value is in the [-1, 1] range.
    ///
    /// **Note:** This modifier assumes the base noise to return values in the [-1, 1] range.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct TriangleWave {
        pub frequency: f32,
    }

    self, seed, point, value;

    map_value: {
        let v = (value + 1.0) * self.frequency;
        let v = v - floor(v * 0.5) * 2.0;
        let v = if v < 1.0 {
            v
        } else {
            2.0 - v
        };
        (v - 0.5) * 2.0
    }
}

modifier_map! {
    /// Multiplies the seed by `value`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MulSeed {
        pub value: i32,
    }

    self, seed, point, value;

    map_seed: {
        seed * self.value
    }
}

/// Calculates the `fractal_bounding` property for [`Fbm`].
///
#[inline(always)]
pub const fn fractal_bounding(octaves: u32, gain: f32) -> f32 {
    let gain = fast_abs(gain);
    let mut amp = gain;
    let mut amp_fractal = 1.0;
    let mut i = 0;

    while i < octaves {
        amp_fractal += amp;
        amp *= gain;
        i += 1;
    }

    1.0 / amp_fractal
}

#[inline(always)]
const fn fast_abs(f: f32) -> f32 {
    if f < 0.0 {
        -f
    } else {
        f
    }
}
