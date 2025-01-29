use crate::{
    modifiers::{AddSeed, Fbm, Frequency, MulSeed, Ridged, Seeded, Tileable, TranslateX, TranslateXy, TranslateXyz, TranslateXyzw, TriangleWave},
    Sample, ValueOrNoise,
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

    /// Applies a triangle wave to the output of a noise function.
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

    /// Adds `value` to the seed.
    #[inline(always)]
    fn add_seed(self, value: i32) -> AddSeed<Self>
    where
        Self: Sized,
    {
        AddSeed { noise: self, value }
    }

    /// Multiplies the seed by `value`.
    #[inline(always)]
    fn mul_seed(self, value: i32) -> MulSeed<Self>
    where
        Self: Sized,
    {
        MulSeed { noise: self, value }
    }

    /// Translates the point before it is being sampled by the base noise.
    fn translate_x<X>(self, x: X) -> TranslateX<Self, X::Noise>
    where
        Self: Sized,
        X: ValueOrNoise,
    {
        TranslateX { noise: self, x: x.into_noise() }
    }

    /// Translates the point before it is being sampled by the base noise.
    fn translate_xy<X, Y>(self, x: X, y: Y) -> TranslateXy<Self, X::Noise, Y::Noise>
    where
        Self: Sized,
        X: ValueOrNoise,
        Y: ValueOrNoise,
    {
        TranslateXy {
            noise: self,
            x: x.into_noise(),
            y: y.into_noise(),
        }
    }

    /// Translates the point before it is being sampled by the base noise.
    fn translate_xyz<X, Y, Z>(self, x: X, y: Y, z: Z) -> TranslateXyz<Self, X::Noise, Y::Noise, Z::Noise>
    where
        Self: Sized,
        X: ValueOrNoise,
        Y: ValueOrNoise,
        Z: ValueOrNoise,
    {
        TranslateXyz {
            noise: self,
            x: x.into_noise(),
            y: y.into_noise(),
            z: z.into_noise(),
        }
    }

    /// Translates the point before it is being sampled by the base noise.
    fn translate_xyzw<X, Y, Z, W>(self, x: X, y: Y, z: Z, w: W) -> TranslateXyzw<Self, X::Noise, Y::Noise, Z::Noise, W::Noise>
    where
        Self: Sized,
        X: ValueOrNoise,
        Y: ValueOrNoise,
        Z: ValueOrNoise,
        W: ValueOrNoise,
    {
        TranslateXyzw {
            noise: self,
            x: x.into_noise(),
            y: y.into_noise(),
            z: z.into_noise(),
            w: w.into_noise(),
        }
    }
}

impl<N: Noise> Noise for &N {}
