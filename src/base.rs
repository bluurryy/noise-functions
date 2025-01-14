pub mod cell;
mod cell_distance;
mod cell_value;
mod fast_cell_distance;
mod fast_cell_distance_sq;
mod fast_cell_value;
mod open_simplex_2;
mod open_simplex_2s;
mod perlin;
mod simplex;
mod value;

pub use cell_distance::CellDistance;
pub use cell_value::CellValue;
pub use open_simplex_2::OpenSimplex2;
pub use perlin::Perlin;
pub use simplex::Simplex;
pub use value::Value;

macro_rules! if_has_dim_4 {
    (234; $($tt:tt)*) => { $($tt)* };
    ($dims:literal; $($tt:tt)*) => {};
}

macro_rules! impl_noise {
    ($dims:tt $struct:ident) => {
        impl $struct {
            #[inline(always)]
            pub const fn seed(self, seed: i32) -> $crate::Seeded<Self> {
                $crate::Seeded { noise: self, seed }
            }

            #[inline(always)]
            pub const fn frequency(self, frequency: f32) -> $crate::Frequency<Self> {
                $crate::Frequency { noise: self, frequency }
            }

            $crate::base::if_has_dim_4! { $dims;
                #[inline(always)]
                pub const fn tileable(self, width: f32, height: f32) -> $crate::Tileable<Self> {
                    $crate::Tileable::new(self, width, height)
                }
            }

            #[inline(always)]
            pub const fn fbm(self, octaves: u32, gain: f32, lacunarity: f32) -> $crate::Fbm<Self> {
                $crate::Fbm {
                    noise: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: $crate::fractal_bounding(octaves, gain),
                }
            }

            #[inline(always)]
            pub const fn ridged(self, octaves: u32, gain: f32, lacunarity: f32) -> $crate::Ridged<Self> {
                $crate::Ridged {
                    noise: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: $crate::fractal_bounding(octaves, gain),
                }
            }

            #[inline(always)]
            pub const fn ping_pong(self, octaves: u32, gain: f32, lacunarity: f32, strength: f32) -> $crate::PingPong<Self> {
                $crate::PingPong {
                    noise: self,
                    octaves,
                    gain,
                    lacunarity,
                    fractal_bounding: $crate::fractal_bounding(octaves, gain),
                    strength,
                }
            }
        }

        impl $crate::Sample<2> for $struct {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                self.gen2(point, 0)
            }
        }

        impl $crate::Sample<2> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                self.noise.gen2(point, self.seed)
            }
        }

        impl $crate::Sample<2> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                self.noise.gen2(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                self.gen2a(point, 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                self.noise.gen2a(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                self.noise.gen2a(point, self.seed)
            }
        }

        impl $crate::Sample<3> for $struct {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                self.gen3(point, 0)
            }
        }

        impl $crate::Sample<3> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                self.noise.gen3(point, self.seed)
            }
        }

        impl $crate::Sample<3> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                self.noise.gen3(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                self.gen3a(point, 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                self.noise.gen3a(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                self.noise.gen3a(point, self.seed)
            }
        }

        $crate::base::if_has_dim_4! { $dims;
            impl $crate::Sample<4> for $struct {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    self.gen4(point, 0)
                }
            }

            impl $crate::Sample<4> for $crate::Seeded<$struct> {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    self.noise.gen4(point, self.seed)
                }
            }

            impl $crate::Sample<4> for $crate::Seeded<&$struct> {
                #[inline(always)]
                fn sample(&self, point: [f32; 4]) -> f32 {
                    self.noise.gen4(point, self.seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::Sample<4, core::simd::f32x4> for $struct {
                #[inline(always)]
                fn sample(&self, point: core::simd::f32x4) -> f32 {
                    self.gen4a(point, 0)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::Sample<4, core::simd::f32x4> for $crate::Seeded<$struct> {
                #[inline(always)]
                fn sample(&self, point: core::simd::f32x4) -> f32 {
                    self.noise.gen4a(point, self.seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl $crate::Sample<4, core::simd::f32x4> for $crate::Seeded<&$struct> {
                #[inline(always)]
                fn sample(&self, point: core::simd::f32x4) -> f32 {
                    self.noise.gen4a(point, self.seed)
                }
            }
        }
    };
}

pub(crate) use if_has_dim_4;
pub(crate) use impl_noise;
