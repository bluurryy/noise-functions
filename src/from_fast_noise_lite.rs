//! Code ported from [FastNoiseLite](https://github.com/Auburn/FastNoiseLite).
//!
//! FastNoiseLite is licensed under the following MIT License:
//!
//! ```text
//! MIT License
//!
//! Copyright(c) 2020 Jordan Peck (jordan.me2@gmail.com)
//! Copyright(c) 2020 Contributors
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

pub(crate) mod cell_distance_euclidean_squared;
mod lookup;

use core::ops::RangeInclusive;

pub(crate) use lookup::*;

pub(crate) use crate::math::{floor_to_int, interp_hermite, interp_quintic, round_to_int};

#[cfg(feature = "nightly-simd")]
pub(crate) use crate::math::splat;

#[cfg(feature = "nightly-simd")]
pub(crate) use crate::lookup_table::{Index4, Index4x4};

pub trait Dot {
    type Output;
    fn dot(lhs: Self, rhs: Self) -> Self::Output;
}

#[inline(always)]
pub fn dot<T: Dot>(lhs: T, rhs: T) -> T::Output {
    Dot::dot(lhs, rhs)
}

#[inline(always)]
pub fn length_squared<T: Dot + Copy>(value: T) -> T::Output {
    dot(value, value)
}

#[allow(dead_code)]
pub fn length<T: Dot<Output = f32> + Copy>(value: T) -> T::Output {
    crate::math::sqrt(length_squared(value))
}

#[inline(always)]
pub fn fast_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn fast_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline(always)]
pub fn cubic_lerp(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let p = (d - c) - (a - b);
    t * t * t * p + t * t * ((a - b) - p) + t * (c - a) + b
}

pub(crate) const PRIME_X: i32 = 501125321;
pub(crate) const PRIME_Y: i32 = 1136930381;
pub(crate) const PRIME_Z: i32 = 1720413743;

pub(crate) const JITTER_2D: f32 = 0.43701595;
pub(crate) const JITTER_3D: f32 = 0.39614353;

#[inline(always)]
pub fn hash2(seed: i32, x_primed: i32, y_primed: i32) -> i32 {
    let mut hash = seed ^ x_primed ^ y_primed;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

#[inline(always)]
pub fn hash3(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32) -> i32 {
    let mut hash = seed ^ x_primed ^ y_primed ^ z_primed;
    hash = hash.wrapping_mul(0x27d4eb2d);
    hash
}

#[inline(always)]
pub fn value2(seed: i32, x_primed: i32, y_primed: i32) -> f32 {
    let mut hash: i32 = hash2(seed, x_primed, y_primed);
    hash = hash.wrapping_mul(hash);
    hash ^= hash.wrapping_shl(19);
    hash as f32 * (1.0 / 2147483648.0)
}

#[inline(always)]
pub fn value3(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32) -> f32 {
    let mut hash: i32 = hash3(seed, x_primed, y_primed, z_primed);
    hash = hash.wrapping_mul(hash);
    hash ^= hash.wrapping_shl(19);
    hash as f32 * (1.0 / 2147483648.0)
}

#[inline(always)]
pub fn grad2(seed: i32, x_primed: i32, y_primed: i32, xd: f32, yd: f32) -> f32 {
    let mut hash = hash2(seed, x_primed, y_primed);
    hash ^= hash.wrapping_shr(15);

    let [xg, yg] = GRADIENTS_2D[hash].as_array();
    xd * xg + yd * yg
}

#[inline(always)]
pub fn grad3(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32, xd: f32, yd: f32, zd: f32) -> f32 {
    let mut hash: i32 = hash3(seed, x_primed, y_primed, z_primed);
    hash ^= hash.wrapping_shr(15);

    let &[xg, yg, zg, _] = GRADIENTS_3D[hash].as_array();
    xd * xg + yd * yg + zd * zg
}

#[inline(always)]
pub fn cell_neighbours(cell: i32) -> RangeInclusive<i32> {
    cell.wrapping_sub(1)..=cell.wrapping_add(1)
}

#[cfg(feature = "nightly-simd")]
mod simd {
    use core::simd::{f32x2, f32x4, i32x2, i32x4, simd_swizzle};

    use super::{Dot, Index4, GRADIENTS_3D, PRIME_X, PRIME_Y, PRIME_Z};

    pub(crate) const PRIME_XY: i32x2 = i32x2::from_array([PRIME_X, PRIME_Y]);
    pub(crate) const PRIME_XYZ: i32x4 = i32x4::from_array([PRIME_X, PRIME_Y, PRIME_Z, 0]);

    impl Dot for f32x2 {
        type Output = f32;

        #[inline(always)]
        fn dot(lhs: Self, rhs: Self) -> f32 {
            lhs[0] * rhs[0] + lhs[1] * rhs[1]
        }
    }

    impl Dot for f32x4 {
        type Output = f32;

        #[inline(always)]
        fn dot(lhs: Self, rhs: Self) -> f32 {
            // code taken from https://github.com/bitshifter/glam-rs
            let x2_y2_z2_w2 = lhs * rhs;
            let y2_0_0_0 = simd_swizzle!(x2_y2_z2_w2, [1, 0, 0, 0]);
            let z2_0_0_0 = simd_swizzle!(x2_y2_z2_w2, [2, 0, 0, 0]);
            let x2y2_0_0_0 = x2_y2_z2_w2 + y2_0_0_0;
            let dot3 = x2y2_0_0_0 + z2_0_0_0;
            dot3[0]
        }
    }

    #[inline(always)]
    pub(crate) fn grad3_with_hash(hash: Index4<64>, dx: f32, dy: f32, dz: f32) -> f32 {
        let g = GRADIENTS_3D[hash].0;
        dx * g[0] + dy * g[1] + dz * g[2]
    }
}

#[cfg(feature = "nightly-simd")]
pub(crate) use simd::*;
