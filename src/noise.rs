use crate::{
    modifiers::{Fbm, Frequency, MulSeed, Ridged, Seeded, Tileable, TriangleWave},
    Sample,
};

/// Provides modifier methods for noise types.
pub trait Noise {
    // Samples the noise with seed 0.
    fn sample<const DIM: usize, Point>(&self, point: Point) -> f32
    where
        Self: Sample<DIM, Point> + Sized,
    {
        self.sample_with_seed(point, 0)
    }

    /// Sets a seed to be sampled with.
    ///
    /// This requires a noise that implements `SampleWithSeed`.
    #[inline(always)]
    fn seed(self, seed: i32) -> Seeded<Self>
    where
        Self: Sized,
    {
        Seeded { noise: self, seed }
    }

    /// Modifies a noise with a frequency multiplier.
    ///
    /// This multiplies the point by the provided `frequency` before sampling.
    #[inline(always)]
    fn frequency(self, frequency: f32) -> Frequency<Self>
    where
        Self: Sized,
    {
        Frequency { noise: self, frequency }
    }

    /// Creates a fractal from this noise with the provided `octaves`, `gain`
    /// and `lacunarity`.
    ///
    /// The seed, with which the fractal is sampled with, will be incremented
    /// after each octave.
    // TODO: explain the parameters
    #[inline(always)]
    fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> Fbm<Self>
    where
        Self: Sized,
    {
        Fbm::new(self, octaves, gain, lacunarity)
    }

    /// Modifies a noise to create a peak at 0.
    ///
    /// This outputs values is in the [-1, 1] range.
    ///
    /// **Note:** This modifier assumes the base noise returns values in the [-1, 1] range.
    #[inline(always)]
    fn ridged(self) -> Ridged<Self>
    where
        Self: Sized,
    {
        Ridged { noise: self }
    }

    /// Applies a triangle wave to the output of a base noise function.
    ///
    /// This outputs values is in the [-1, 1] range.
    ///
    /// **Note:** This modifier assumes the base noise returns values in the [-1, 1] range.
    #[inline(always)]
    fn triangle_wave(self, frequency: f32) -> TriangleWave<Self>
    where
        Self: Sized,
    {
        TriangleWave { noise: self, frequency }
    }

    /// Creates a tileable 2D noise from a 4D noise.
    ///
    /// The parameters `width` and `height` describe the size of the repeating tile.
    #[inline(always)]
    fn tileable(self, width: f32, height: f32) -> Tileable<Self>
    where
        Self: Sized,
    {
        Tileable::new(self, width, height)
    }

    /// Multiplies the seed by `value`.
    #[inline(always)]
    fn mul_seed(self, value: i32) -> MulSeed<Self>
    where
        Self: Sized,
    {
        MulSeed { noise: self, value }
    }
}

impl<N: Noise> Noise for &N {}
