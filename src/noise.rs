use crate::modifiers::{Add, Div, Fbm, Frequency, Mul, PingPong, Rem, Ridged, Seeded, Sub, Tileable};

pub trait Noise: Sized {
    #[inline(always)]
    fn seed(self, seed: i32) -> Seeded<Self> {
        Seeded { noise: self, seed }
    }

    #[inline(always)]
    fn frequency(self, frequency: f32) -> Frequency<Self> {
        Frequency { noise: self, frequency }
    }

    #[inline(always)]
    fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> Fbm<Self> {
        Fbm::new(self, octaves, gain, lacunarity)
    }

    #[inline(always)]
    fn ridged(self) -> Ridged<Self> {
        Ridged { noise: self }
    }

    #[inline(always)]
    fn ping_pong(self, octaves: u32, gain: f32, lacunarity: f32, strength: f32) -> PingPong<Self> {
        PingPong::new(self, octaves, gain, lacunarity, strength)
    }

    #[inline(always)]
    fn tileable(self, width: f32, height: f32) -> Tileable<Self> {
        Tileable::new(self, width, height)
    }

    #[inline(always)]
    fn add(self, value: f32) -> Add<Self> {
        Add { noise: self, value }
    }

    #[inline(always)]
    fn sub(self, value: f32) -> Sub<Self> {
        Sub { noise: self, value }
    }

    #[inline(always)]
    fn mul(self, value: f32) -> Mul<Self> {
        Mul { noise: self, value }
    }

    #[inline(always)]
    fn div(self, value: f32) -> Div<Self> {
        Div { noise: self, value }
    }

    #[inline(always)]
    fn rem(self, value: f32) -> Rem<Self> {
        Rem { noise: self, value }
    }
}
