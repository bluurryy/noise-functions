//! Code ported from [FastNoise2](https://github.com/Auburn/FastNoise2).
//! This includes just the functions we actually use for this library.
//! For a more complete port (of the scalar algorithms) have a look at <https://github.com/bluurryy/fast-noise2-port/>.
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

pub(crate) mod primes {
    pub(crate) const X: i32 = 501125321;
    pub(crate) const Y: i32 = 1136930381;
    pub(crate) const Z: i32 = 1720413743;
    pub(crate) const W: i32 = 1066037191;
}

pub(crate) const ROOT2: f32 = 1.4142135623730950488;

pub(crate) use crate::{interp_hermite, interp_quintic, max, min, mul_add};

pub(crate) fn hash_primes2(seed: i32, x: i32, y: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y;
    hash = hash.wrapping_mul(0x27d4eb2d);
    (hash >> 15) ^ hash
}

pub(crate) fn hash_primes3(seed: i32, x: i32, y: i32, z: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z;
    hash = hash.wrapping_mul(0x27d4eb2d);
    (hash >> 15) ^ hash
}

pub(crate) fn hash_primes4(seed: i32, x: i32, y: i32, z: i32, w: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z ^ w;
    hash = hash.wrapping_mul(0x27d4eb2d);
    (hash >> 15) ^ hash
}

pub(crate) fn hash_primes2_hb(seed: i32, x: i32, y: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

pub(crate) fn hash_primes3_hb(seed: i32, x: i32, y: i32, z: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

pub(crate) fn hash_primes4_hb(seed: i32, x: i32, y: i32, z: i32, w: i32) -> i32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z ^ w;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

pub(crate) fn value_coord4(seed: i32, x: i32, y: i32, z: i32, w: i32) -> f32 {
    let mut hash = seed;
    hash ^= x ^ y ^ z ^ w;
    hash = hash.wrapping_mul(hash.wrapping_mul(0x27d4eb2d));
    hash as f32 * (1.0 / i32::MAX as f32)
}

pub(crate) fn gradient_dot2(hash: i32, x: f32, y: f32) -> f32 {
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

pub(crate) fn gradient_dot3(hash: i32, x: f32, y: f32, z: f32) -> f32 {
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

pub(crate) fn gradient_dot4(hash: i32, x: f32, y: f32, z: f32, w: f32) -> f32 {
    let p = hash & (3 << 3);

    let a = if p > 0 { x } else { y };
    let b = if p > (1 << 3) { y } else { z };
    let c = if p > (2 << 3) { z } else { w };

    let a_sign = (hash as u32) << 31;
    let b_sign = ((hash as u32) << 30) & 0x80000000;
    let c_sign = ((hash as u32) << 29) & 0x80000000;

    f32::from_bits(a.to_bits() ^ a_sign) + f32::from_bits(b.to_bits() ^ b_sign) + f32::from_bits(c.to_bits() ^ c_sign)
}

pub(crate) fn inv_sqrt(mut a: f32) -> f32 {
    let x_half = 0.5 * a;
    a = f32::from_bits((0x5f3759dfi32.wrapping_sub(a.to_bits() as i32 >> 1)) as u32);
    a *= 1.5 - x_half * a * a;
    a
}

pub(crate) fn reciprocal(mut a: f32) -> f32 {
    // pow( pow(x,-0.5), 2 ) = pow( x, -1 ) = 1.0 / x
    a = f32::from_bits(0xbe6eb3beu32.wrapping_sub(a.to_bits()) >> 1);
    a * a
}

pub(crate) fn nmul_add(a: f32, b: f32, c: f32) -> f32 {
    -(a * b) + c
}

pub(crate) trait WrappingOps {
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

pub(crate) fn masked_inc<T: WrappingOps + From<u8>>(a: T, m: bool) -> T {
    if m {
        a.wrapping_add(1.into())
    } else {
        a
    }
}

pub(crate) fn masked_add<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a.wrapping_add(b)
    } else {
        a
    }
}

pub(crate) fn nmasked_add<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a
    } else {
        a.wrapping_add(b)
    }
}

pub(crate) fn masked_sub<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a.wrapping_sub(b)
    } else {
        a
    }
}

pub(crate) fn nmasked_sub<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a
    } else {
        a.wrapping_sub(b)
    }
}

pub mod cell {
    use crate::{abs, max, mul_add};

    pub use crate::base::cell::{CellIndex, DistanceFn, DistanceReturnType};

    use super::inv_sqrt;

    pub(crate) const JITTER_2D: f32 = 0.437016;
    pub(crate) const JITTER_3D: f32 = 0.396144;
    pub(crate) const JITTER_4D: f32 = 0.366025;

    pub(crate) const MAX_DISTANCE_COUNT: usize = 4;

    pub(crate) fn calc_distance2(distance_fn: DistanceFn, x: f32, y: f32) -> f32 {
        match distance_fn {
            DistanceFn::Euclidean => {
                let dist_sqr = mul_add(y, y, x * x);
                inv_sqrt(dist_sqr) * dist_sqr
            }
            DistanceFn::EuclideanSquared => mul_add(y, y, x * x),
            DistanceFn::Manhattan => abs(x) + abs(y),
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
            DistanceFn::Manhattan => abs(x) + abs(y) + abs(z),
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
            DistanceFn::Manhattan => abs(x) + abs(y) + abs(z) + abs(w),
            DistanceFn::Hybrid => mul_add(x, x, abs(x)) + mul_add(y, y, abs(y)) + mul_add(z, z, abs(z)) + mul_add(w, w, abs(w)),
            DistanceFn::MaxAxis => max(max(max(abs(x), abs(y)), abs(z)), abs(w)),
        }
    }
}
