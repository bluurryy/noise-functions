#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

use crate::{
    modifiers::{
        Abs, Add, AddSeed, Ceil, Clamp, Div, Fbm, Floor, Frequency, Lerp, Map, Max, Min, Mul, MulSeed, Neg, Pow, Rem, Ridged, Round, Seeded, Sqrt, Sub, Tileable, TranslateX, TranslateXy,
        TranslateXyz, TranslateXyzw, TriangleWave,
    },
    Sample, ValueOrNoise,
};

/// Provides utility methods for noise types.
pub trait Noise {
    /// Samples the noise in 2D.
    fn sample2<Point>(&self, point: Point) -> f32
    where
        Self: Sized + Sample<2>,
        Point: Into<[f32; 2]>,
    {
        self.sample_with_seed(point.into(), 0)
    }

    /// Samples the noise in 3D.
    fn sample3<Point>(&self, point: Point) -> f32
    where
        Self: Sized + Sample<3>,
        Point: Into<[f32; 3]>,
    {
        self.sample_with_seed(point.into(), 0)
    }

    /// Samples the noise in 3D.
    fn sample4<Point>(&self, point: Point) -> f32
    where
        Self: Sized + Sample<4>,
        Point: Into<[f32; 4]>,
    {
        self.sample_with_seed(point.into(), 0)
    }

    /// Samples the noise in 2D.
    #[cfg(feature = "nightly-simd")]
    fn sample2a<Point>(&self, point: Point) -> f32
    where
        Self: Sized + Sample<2, f32x2>,
        Point: Into<f32x2>,
    {
        self.sample_with_seed(point.into(), 0)
    }

    /// Samples the noise in 3D.
    #[cfg(feature = "nightly-simd")]
    fn sample3a<Point>(&self, point: Point) -> f32
    where
        Self: Sized + Sample<3, f32x4>,
        Point: Into<f32x4>,
    {
        self.sample_with_seed(point.into(), 0)
    }

    /// Samples the noise in 4D.
    #[cfg(feature = "nightly-simd")]
    fn sample4a<Point>(&self, point: Point) -> f32
    where
        Self: Sized + Sample<4, f32x4>,
        Point: Into<f32x4>,
    {
        self.sample_with_seed(point.into(), 0)
    }

    /// Overwrites the seed to be sampled with.
    ///
    /// For the sake of composition it can be better to use [`add_seed`](Self::add_seed) instead.
    #[inline(always)]
    fn seed(self, seed: i32) -> Seeded<Self>
    where
        Self: Sized,
    {
        Seeded { noise: self, seed }
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

    /// Modifies a noise with a frequency multiplier.
    ///
    /// This multiplies the point by the provided `frequency` before sampling.
    #[inline(always)]
    fn frequency<F>(self, frequency: F) -> Frequency<Self, F::Noise>
    where
        Self: Sized,
        F: ValueOrNoise,
    {
        Frequency {
            noise: self,
            frequency: frequency.into_noise(),
        }
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
    /// **Note:** This modifier assumes `self` returns values in the [-1, 1] range.
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
    /// **Note:** This modifier assumes `self` returns values in the [-1, 1] range.
    #[inline(always)]
    fn triangle_wave<F>(self, frequency: F) -> TriangleWave<Self, F::Noise>
    where
        Self: Sized,
        F: ValueOrNoise,
    {
        TriangleWave {
            noise: self,
            frequency: frequency.into_noise(),
        }
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

    /// Translates the point before it is used to sample `self`.
    fn translate_x<X>(self, x: X) -> TranslateX<Self, X::Noise>
    where
        Self: Sized,
        X: ValueOrNoise,
    {
        TranslateX { noise: self, x: x.into_noise() }
    }

    /// Translates the point before it is used to sample `self`.
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

    /// Translates the point before it is used to sample `self`.
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

    /// Translates the point before it is used to sample `self`.
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

    /// Computes the minimum of the two output values.
    fn min<Rhs>(self, rhs: Rhs) -> Min<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Min { lhs: self, rhs: rhs.into_noise() }
    }

    /// Computes the maximum of the two output values.
    fn max<Rhs>(self, rhs: Rhs) -> Max<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Max { lhs: self, rhs: rhs.into_noise() }
    }

    /// Returns `max` if `value` is greater than `max` and `min` if `value` is less than `min`.
    /// Otherwise this will return `value`.
    ///
    /// Unlike [`f32::clamp`], this modifier won't panic if `!(min <= max)`.
    fn clamp<Min, Max>(self, min: Min, max: Max) -> Clamp<Self, Min::Noise, Max::Noise>
    where
        Self: Sized,
        Min: ValueOrNoise,
        Max: ValueOrNoise,
    {
        Clamp {
            value: self,
            min: min.into_noise(),
            max: max.into_noise(),
        }
    }

    /// Linearly interpolates between `self` and `b`.
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

    /// Raises the output value to a power.
    fn pow<Rhs>(self, rhs: Rhs) -> Pow<Self, Rhs::Noise>
    where
        Self: Sized,
        Rhs: ValueOrNoise,
    {
        Pow { lhs: self, rhs: rhs.into_noise() }
    }

    /// Performs negation on the output value.
    fn neg(self) -> Neg<Self>
    where
        Self: Sized,
    {
        Neg { noise: self }
    }

    /// Computes the absolute value of the output value.
    fn abs(self) -> Abs<Self>
    where
        Self: Sized,
    {
        Abs { noise: self }
    }

    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    fn sqrt(self) -> Sqrt<Self>
    where
        Self: Sized,
    {
        Sqrt { noise: self }
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

    /// Maps the output value.
    #[inline(always)]
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(f32) -> f32,
    {
        Map { noise: self, f }
    }

    /// Returns the `Noise` by reference.
    ///
    /// This reference also implements `Noise`.
    fn by_ref(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

impl<N: Noise> Noise for &N {}

#[cfg(feature = "alloc")]
impl<N: Noise + ?Sized> Noise for alloc::boxed::Box<N> {}
