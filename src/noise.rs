use crate::modifiers::{Fbm, Frequency, MulSeed, Ridged, Seeded, Tileable, TriangleWave};

/// Provides modifier methods for noise types.
pub trait Noise {
    #[inline(always)]
    fn seed(self, seed: i32) -> Seeded<Self>
    where
        Self: Sized,
    {
        Seeded { noise: self, seed }
    }

    #[inline(always)]
    fn frequency(self, frequency: f32) -> Frequency<Self>
    where
        Self: Sized,
    {
        Frequency { noise: self, frequency }
    }

    #[inline(always)]
    fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> Fbm<Self>
    where
        Self: Sized,
    {
        Fbm::new(self, octaves, gain, lacunarity)
    }

    #[inline(always)]
    fn ridged(self) -> Ridged<Self>
    where
        Self: Sized,
    {
        Ridged { noise: self }
    }

    #[inline(always)]
    fn triangle_wave(self, frequency: f32) -> TriangleWave<Self>
    where
        Self: Sized,
    {
        TriangleWave { noise: self, frequency }
    }

    #[inline(always)]
    fn tileable(self, width: f32, height: f32) -> Tileable<Self>
    where
        Self: Sized,
    {
        Tileable::new(self, width, height)
    }

    #[inline(always)]
    fn mul_seed(self, value: i32) -> MulSeed<Self>
    where
        Self: Sized,
    {
        MulSeed { noise: self, value }
    }
}

impl<N: Noise> Noise for &N {}
