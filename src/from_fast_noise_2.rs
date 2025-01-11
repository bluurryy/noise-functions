//! Code ported from [FastNoise2](https://github.com/Auburn/FastNoise2).
//!
//! FastNoise2 is licensed under the following MIT License:
//!
//! ```text
//! MIT License
//!
//! Copyright (c) 2020 Jordan Peck
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! ```

#![deny(arithmetic_overflow, clippy::arithmetic_side_effects)]

mod open_simplex_2;
mod open_simplex_2s;
mod perlin;
mod value;

pub use open_simplex_2::OpenSimplex2;
pub use open_simplex_2s::OpenSimplex2s;
pub use perlin::Perlin;
pub use value::Value;

mod primes {
    pub(super) const X: i32 = 501125321;
    pub(super) const Y: i32 = 1136930381;
    pub(super) const Z: i32 = 1720413743;
    pub(super) const W: i32 = 1066037191;
}

const ROOT2: f32 = 1.4142135623730950488;
const ROOT3: f32 = 1.7320508075688772935;

use crate::{interp_hermite, interp_quintic, lerp};

fn hash_primes2(seed: i32, x: i32, y: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y;
    hash = hash.wrapping_mul(0x27d4eb2d);
    (hash >> 15) ^ hash
}

fn hash_primes3(seed: i32, x: i32, y: i32, z: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z;
    hash = hash.wrapping_mul(0x27d4eb2d);
    (hash >> 15) ^ hash
}

fn hash_primes4(seed: i32, x: i32, y: i32, z: i32, w: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z ^ w;
    hash = hash.wrapping_mul(0x27d4eb2d);
    (hash >> 15) ^ hash
}

fn value_coord2(seed: i32, x: i32, y: i32) -> f32 {
    let mut hash = seed;
    hash ^= x ^ y;
    hash = hash.wrapping_mul(hash.wrapping_mul(0x27d4eb2d));
    hash as f32 * (1.0 / i32::MAX as f32)
}

fn value_coord3(seed: i32, x: i32, y: i32, z: i32) -> f32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z;
    hash = hash.wrapping_mul(hash.wrapping_mul(0x27d4eb2d));
    hash as f32 * (1.0 / i32::MAX as f32)
}

fn value_coord4(seed: i32, x: i32, y: i32, z: i32, w: i32) -> f32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z ^ w;
    hash = hash.wrapping_mul(hash.wrapping_mul(0x27d4eb2d));
    hash as f32 * (1.0 / i32::MAX as f32)
}

fn gradient_dot2(hash: i32, x: f32, y: f32) -> f32 {
    // ( 1+R2, 1 ) ( -1-R2, 1 ) ( 1+R2, -1 ) ( -1-R2, -1 )
    // ( 1, 1+R2 ) ( 1, -1-R2 ) ( -1, 1+R2 ) ( -1, -1-R2 )

    let bit1 = hash << 31;
    let bit2 = (hash >> 1) << 31;
    let bit4 = (hash & (1 << 2)) != 0;

    let x = f32::from_bits(x.to_bits() ^ bit1 as u32);
    let y = f32::from_bits(y.to_bits() ^ bit2 as u32);

    let a = if bit4 { y } else { x };
    let b = if bit4 { x } else { y };

    (1.0 + ROOT2) * a + b
}

fn gradient_dot3(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let hasha13 = hash & 13;

    // if h < 8 then x, else y
    let u = if hasha13 < 8 { x } else { y };

    // if h < 4 then y else if h is 12 or 14 then x else z
    let v = if hasha13 == 12 { x } else { z };
    let v = if hasha13 < 2 { y } else { v };

    // if h1 then -u else u
    // if h2 then -v else v
    let h1 = hash << 31;
    let h2 = (hash & 2) << 30;

    // then add them
    f32::from_bits(u.to_bits() ^ h1 as u32) + f32::from_bits(v.to_bits() ^ h2 as u32)
}

fn gradient_dot4(hash: i32, x: f32, y: f32, z: f32, w: f32) -> f32 {
    let p = hash & (3 << 3);

    let a = if p > 0 { x } else { y };
    let b = if p > (1 << 3) { y } else { z };
    let c = if p > (2 << 3) { z } else { w };

    let a_sign = (hash as u32) << 31;
    let b_sign = ((hash as u32) << 30) & 0x80000000;
    let c_sign = ((hash as u32) << 29) & 0x80000000;

    f32::from_bits(a.to_bits() ^ a_sign) + f32::from_bits(b.to_bits() ^ b_sign) + f32::from_bits(c.to_bits() ^ c_sign)
}

fn gradient_dot2_fancy(hash: i32, x: f32, y: f32) -> f32 {
    let index = (((hash & 0x3FFFFF) as f32) * 1.3333333333333333) as i32;

    let xy = index & (1 << 2) != 0;

    let mut a = if xy { y } else { x };
    let mut b = if xy { x } else { y };

    // Bit-1 = b flip sign
    b = f32::from_bits(b.to_bits() ^ (index << 31) as u32);

    // Bit-2 = Mul a by 2 or Root3
    let a_mul_2 = (index & (1 << 1)) != 0;

    a *= if a_mul_2 { 2.0 } else { ROOT3 };
    // b zero value if a mul 2
    b = if a_mul_2 { 0.0 } else { b };

    // Bit-8 = Flip sign of a + b
    f32::from_bits((a + b).to_bits() ^ ((index >> 3) << 31) as u32)
}

macro_rules! noise23 {
    ($(#[$attr:meta])* $struct:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $struct;

        impl $struct {
            #[inline(always)]
            pub const fn seed(self, seed: i32) -> $crate::Seeded<Self> {
                $crate::Seeded { noise: self, seed }
            }

            #[inline(always)]
            pub const fn frequency(self, frequency: f32) -> $crate::Frequency<Self> {
                $crate::Frequency { noise: self, frequency }
            }

            #[inline(always)]
            pub const fn tileable(self, width: f32, height: f32) -> $crate::Tileable<Self> {
                $crate::Tileable {
                    noise: self,
                    width,
                    height,
                    inv_width: 1.0 / width,
                    inv_height: 1.0 / height,
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
                gen2(point, 0)
            }
        }

        impl $crate::Sample<2> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                gen2(point, self.seed)
            }
        }

        impl $crate::Sample<2> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                gen2(point, self.seed)
            }
        }

        impl $crate::Sample<3> for $struct {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                gen3(point, 0)
            }
        }

        impl $crate::Sample<3> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                gen3(point, self.seed)
            }
        }

        impl $crate::Sample<3> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                gen3(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                gen2(*point.as_array(), 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                gen2(*point.as_array(), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                gen2(*point.as_array(), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen3(*$crate::array_4_take_3(point.as_array()), 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen3(*$crate::array_4_take_3(point.as_array()), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen3(*$crate::array_4_take_3(point.as_array()), self.seed)
            }
        }
    };
}

pub(crate) use noise23;

macro_rules! noise234 {
    ($(#[$attr:meta])* $struct:ident) => {
        $crate::from_fast_noise_2::noise23! {
            $(#[$attr])*
            $struct
        }

        impl $crate::Sample<4> for $struct {
            #[inline(always)]
            fn sample(&self, point: [f32; 4]) -> f32 {
                gen4(point, 0)
            }
        }

        impl $crate::Sample<4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 4]) -> f32 {
                gen4(point, self.seed)
            }
        }

        impl $crate::Sample<4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 4]) -> f32 {
                gen4(point, self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<4, core::simd::f32x4> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen4(*point.as_array(), 0)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<4, core::simd::f32x4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen4(*point.as_array(), self.seed)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<4, core::simd::f32x4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen4(*point.as_array(), self.seed)
            }
        }
    };
}

pub(crate) use noise234;
