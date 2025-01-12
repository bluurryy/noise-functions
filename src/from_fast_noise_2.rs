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

mod cell_distance;
mod cell_value;
mod open_simplex_2;
mod open_simplex_2s;
mod perlin;
mod value;

pub use cell_distance::CellDistance;
pub use cell_value::CellValue;
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

fn hash_primes2_hb(seed: i32, x: i32, y: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

fn hash_primes3_hb(seed: i32, x: i32, y: i32, z: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

fn hash_primes4_hb(seed: i32, x: i32, y: i32, z: i32, w: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z ^ w;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
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

fn inv_sqrt(mut a: f32) -> f32 {
    let x_half = 0.5 * a;
    a = f32::from_bits((0x5f3759dfi32.wrapping_sub(a.to_bits() as i32 >> 1)) as u32);
    a *= 1.5 - x_half * a * a;
    a
}

fn reciprocal(mut a: f32) -> f32 {
    // pow( pow(x,-0.5), 2 ) = pow( x, -1 ) = 1.0 / x
    a = f32::from_bits(0xbe6eb3beu32.wrapping_sub(a.to_bits()) >> 1);
    a * a
}

fn nmul_add(a: f32, b: f32, c: f32) -> f32 {
    -(a * b) + c
}

trait WrappingOps {
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
}

impl WrappingOps for i32 {
    fn wrapping_add(self, other: Self) -> Self {
        i32::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        i32::wrapping_sub(self, other)
    }
}

impl WrappingOps for f32 {
    fn wrapping_add(self, other: Self) -> Self {
        self + other
    }
    fn wrapping_sub(self, other: Self) -> Self {
        self - other
    }
}

fn select<T>(m: bool, a: T, b: T) -> T {
    if m {
        a
    } else {
        b
    }
}

fn mask<T: From<u8>>(a: T, m: bool) -> T {
    if m {
        a
    } else {
        0.into()
    }
}

fn masked_add<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a.wrapping_add(b)
    } else {
        a
    }
}

fn nmasked_add<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a
    } else {
        a.wrapping_add(b)
    }
}

fn masked_sub<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a.wrapping_sub(b)
    } else {
        a
    }
}

fn nmasked_sub<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a
    } else {
        a.wrapping_sub(b)
    }
}

pub mod cell {
    use crate::{abs, max, mul_add, simple_enum};

    use super::inv_sqrt;

    pub(crate) const JITTER_2D: f32 = 0.437016;
    pub(crate) const JITTER_3D: f32 = 0.396144;
    pub(crate) const JITTER_4D: f32 = 0.366025;

    pub(crate) const MAX_DISTANCE_COUNT: usize = 4;

    simple_enum! {
        enum DistanceFn {
            #[default]
            Euclidean,
            EuclideanSquared,
            Manhatten,
            Hybrid,
            MaxAxis,
        }
    }

    simple_enum! {
        enum CellIndex {
            #[default]
            I0 = 0,
            I1 = 1,
            I2 = 2,
            I3 = 3,
        }
    }

    simple_enum! {
        enum DistanceReturnType {
            #[default]
            Index0,
            Index0Add1,
            Index0Sub1,
            Index0Mul1,
            Index0Div1,
        }
    }

    pub(crate) fn calc_distance2(distance_fn: DistanceFn, x: f32, y: f32) -> f32 {
        match distance_fn {
            DistanceFn::Euclidean => {
                let dist_sqr = mul_add(y, y, x * x);
                inv_sqrt(dist_sqr) * dist_sqr
            }
            DistanceFn::EuclideanSquared => mul_add(y, y, x * x),
            DistanceFn::Manhatten => abs(x) + abs(y),
            DistanceFn::Hybrid => mul_add(x, x, abs(x)) + mul_add(y, y, abs(y)),
            DistanceFn::MaxAxis => max(abs(x), abs(y)),
        }
    }

    pub(crate) fn calc_distance3(distance_fn: DistanceFn, x: f32, y: f32, z: f32) -> f32 {
        match distance_fn {
            DistanceFn::Euclidean => {
                let dist_sqr = mul_add(z, z, mul_add(y, y, x * x));
                inv_sqrt(dist_sqr) * dist_sqr
            }
            DistanceFn::EuclideanSquared => mul_add(z, z, mul_add(y, y, x * x)),
            DistanceFn::Manhatten => abs(x) + abs(y) + abs(z),
            DistanceFn::Hybrid => mul_add(x, x, abs(x)) + mul_add(y, y, abs(y)) + mul_add(z, z, abs(z)),
            DistanceFn::MaxAxis => max(max(abs(x), abs(y)), abs(z)),
        }
    }

    pub(crate) fn calc_distance4(distance_fn: DistanceFn, x: f32, y: f32, z: f32, w: f32) -> f32 {
        match distance_fn {
            DistanceFn::Euclidean => {
                let dist_sqr = mul_add(w, w, mul_add(z, z, mul_add(y, y, x * x)));
                inv_sqrt(dist_sqr) * dist_sqr
            }
            DistanceFn::EuclideanSquared => mul_add(w, w, mul_add(z, z, mul_add(y, y, x * x))),
            DistanceFn::Manhatten => abs(x) + abs(y) + abs(z) + abs(w),
            DistanceFn::Hybrid => mul_add(x, x, abs(x)) + mul_add(y, y, abs(y)) + mul_add(z, z, abs(z)) + mul_add(w, w, abs(w)),
            DistanceFn::MaxAxis => max(max(max(abs(x), abs(y)), abs(z)), abs(w)),
        }
    }
}

macro_rules! impl_noise23 {
    ($struct:ident) => {
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
                gen2(point, 0, *self)
            }
        }

        impl $crate::Sample<2> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                gen2(point, self.seed, self.noise)
            }
        }

        impl $crate::Sample<2> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 2]) -> f32 {
                gen2(point, self.seed, *self.noise)
            }
        }

        impl $crate::Sample<3> for $struct {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                gen3(point, 0, *self)
            }
        }

        impl $crate::Sample<3> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                gen3(point, self.seed, self.noise)
            }
        }

        impl $crate::Sample<3> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 3]) -> f32 {
                gen3(point, self.seed, *self.noise)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                gen2(*point.as_array(), 0, *self)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                gen2(*point.as_array(), self.seed, self.noise)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<2, core::simd::f32x2> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x2) -> f32 {
                gen2(*point.as_array(), self.seed, *self.noise)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen3(*$crate::array_4_take_3(point.as_array()), 0, *self)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen3(*$crate::array_4_take_3(point.as_array()), self.seed, self.noise)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<3, core::simd::f32x4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen3(*$crate::array_4_take_3(point.as_array()), self.seed, *self.noise)
            }
        }
    };
}

pub(crate) use impl_noise23;

macro_rules! impl_noise234 {
    ($struct:ident) => {
        $crate::from_fast_noise_2::impl_noise23!($struct);

        impl $crate::Sample<4> for $struct {
            #[inline(always)]
            fn sample(&self, point: [f32; 4]) -> f32 {
                gen4(point, 0, *self)
            }
        }

        impl $crate::Sample<4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 4]) -> f32 {
                gen4(point, self.seed, self.noise)
            }
        }

        impl $crate::Sample<4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: [f32; 4]) -> f32 {
                gen4(point, self.seed, *self.noise)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<4, core::simd::f32x4> for $struct {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen4(*point.as_array(), 0, *self)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<4, core::simd::f32x4> for $crate::Seeded<$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen4(*point.as_array(), self.seed, self.noise)
            }
        }

        #[cfg(feature = "nightly-simd")]
        impl $crate::Sample<4, core::simd::f32x4> for $crate::Seeded<&$struct> {
            #[inline(always)]
            fn sample(&self, point: core::simd::f32x4) -> f32 {
                gen4(*point.as_array(), self.seed, *self.noise)
            }
        }
    };
}

pub(crate) use impl_noise234;
