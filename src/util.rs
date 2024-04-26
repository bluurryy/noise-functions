use core::ops::RangeInclusive;

use crate::private_prelude::*;

cfg_const_feature_float! {
    /// Calculates the `fractal_bounding` property for [`Fbm`](crate::Fbm), [`FbmWeighted`](crate::FbmWeighted), [`Ridged`](crate::Ridged) and [`RidgedWeighted`](crate::RidgedWeighted).
    ///
    #[inline(always)]
    pub fn fractal_bounding(octaves: u32, gain: f32) -> f32 {
        let gain = fast_abs(gain);
        let mut amp = gain;
        let mut amp_fractal = 1.0;
        let mut i = 0;

        while i < octaves {
            amp_fractal += amp;
            amp *= gain;
            i += 1;
        }

        1.0 / amp_fractal
    }
}

cfg_const_feature_float! {
    #[inline(always)]
    pub fn fast_abs(f: f32) -> f32 {
        if f < 0.0 {
            -f
        } else {
            f
        }
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

    let [xg, yg] = GRADIENTS_2D[Index2::new(hash)].as_array();
    xd * xg + yd * yg
}

#[inline(always)]
pub fn grad3(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32, xd: f32, yd: f32, zd: f32) -> f32 {
    let mut hash: i32 = hash3(seed, x_primed, y_primed, z_primed);
    hash ^= hash.wrapping_shr(15);

    let &[xg, yg, zg, _] = GRADIENTS_3D[Index3::new(hash)].as_array();
    xd * xg + yd * yg + zd * zg
}

#[inline(always)]
#[cfg(feature = "nightly-simd")]
pub(crate) fn grad3_with_hash(hash: Index3<64>, dx: f32, dy: f32, dz: f32) -> f32 {
    let g = GRADIENTS_3D[hash].0;
    dx * g[0] + dy * g[1] + dz * g[2]
}

#[inline(always)]
pub fn cell_neighbours(cell: i32) -> RangeInclusive<i32> {
    cell.wrapping_sub(1)..=cell.wrapping_add(1)
}
