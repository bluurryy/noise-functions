use crate::modifiers::{Fbm, Frequency, PingPong, Ridged, Seeded, Tileable};

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
    fn ridged(self, octaves: u32, gain: f32, lacunarity: f32) -> Ridged<Self> {
        Ridged::new(self, octaves, gain, lacunarity)
    }

    #[inline(always)]
    fn ping_pong(self, octaves: u32, gain: f32, lacunarity: f32, strength: f32) -> PingPong<Self> {
        PingPong::new(self, octaves, gain, lacunarity, strength)
    }

    #[inline(always)]
    fn tileable(self, width: f32, height: f32) -> Tileable<Self> {
        Tileable::new(self, width, height)
    }
}
