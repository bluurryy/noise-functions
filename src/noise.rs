use crate::{
    modifiers::{
        Abs, Add, AddSeed, Ceil, Div, Fbm, Floor, Frequency, Lerp, Max, Min, Mul, MulSeed, Rem, Ridged, Round, Seeded, Sub, Tileable, TranslateX, TranslateXy, TranslateXyz, TranslateXyzw,
        TriangleWave,
    },
    Sample, ValueOrNoise,
};

/// Provides modifier methods for noise types.
pub trait Noise {
    /// Samples the noise with seed 0.
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

    /// Adds the output values.
    fn add<Rhs>(self, rhs: Rhs) -> Add<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Add { lhs: self, rhs: rhs.into_noise() }
    }

    /// Subtracts one output value from the other.
    fn sub<Rhs>(self, rhs: Rhs) -> Sub<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Sub { lhs: self, rhs: rhs.into_noise() }
    }

    /// Multiplies the output values.
    fn mul<Rhs>(self, rhs: Rhs) -> Mul<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Mul { lhs: self, rhs: rhs.into_noise() }
    }

    /// Divides one output value by the other.
    fn div<Rhs>(self, rhs: Rhs) -> Div<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Div { lhs: self, rhs: rhs.into_noise() }
    }

    /// Calculates the remainder from dividing one output value by the other.
    fn rem<Rhs>(self, rhs: Rhs) -> Rem<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Rem { lhs: self, rhs: rhs.into_noise() }
    }

    /// Computes the minimum of the two output values, ignoring NaN.
    fn min<Rhs>(self, rhs: Rhs) -> Min<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Min { lhs: self, rhs: rhs.into_noise() }
    }

    /// Computes the maximum of the two output values, ignoring NaN.
    fn max<Rhs>(self, rhs: Rhs) -> Max<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Max { lhs: self, rhs: rhs.into_noise() }
    }

    /// Linearly interpolates between `a` and `b`.
    #[doc(alias = "mix")]
    #[doc(alias = "blend")]
    fn lerp<B, T>(self, b: B, t: T) -> Lerp<Self, B::Noise, T::Noise>
    where
        Self: Sized,
        B: ValueOrNoise,
        T: ValueOrNoise,
    {
        Lerp {
            a: self,
            b: b.into_noise(),
            t: t.into_noise(),
        }
    }

    /// Computes the absolute value of the output value.
    fn abs(self) -> Abs<Self>
    where
        Self: Sized,
    {
        Abs { noise: self }
    }

    /// Computes the largest integer less than or equal to the output value.
    fn floor(self) -> Floor<Self>
    where
        Self: Sized,
    {
        Floor { noise: self }
    }

    /// Computes the smallest integer greater than or equal to self.
    fn ceil(self) -> Ceil<Self>
    where
        Self: Sized,
    {
        Ceil { noise: self }
    }

    /// Computes the nearest integer to the output value.
    /// If a value is half-way between two integers, round away from 0.0.
    fn round(self) -> Round<Self>
    where
        Self: Sized,
    {
        Round { noise: self }
    }
}

impl<N: Noise> Noise for &N {}
