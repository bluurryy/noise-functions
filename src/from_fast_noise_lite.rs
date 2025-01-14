pub(crate) mod cell_distance_euclidean_squared;
mod lookup;
mod table2;
mod table4;

use core::ops::RangeInclusive;

pub(crate) use lookup::*;
pub(crate) use table2::*;
pub(crate) use table4::*;

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

pub trait FloorToInt {
    type Output;
    fn floor_to_int(self) -> Self::Output;
}

impl FloorToInt for f32 {
    type Output = i32;

    #[inline(always)]
    fn floor_to_int(self) -> Self::Output {
        if self >= 0.0 {
            self as i32
        } else {
            (self as i32).wrapping_sub(1)
        }
    }
}

#[inline(always)]
pub fn floor_to_int<T: FloorToInt>(value: T) -> T::Output {
    FloorToInt::floor_to_int(value)
}

pub trait RoundToInt {
    type Output;
    fn round_to_int(self) -> Self::Output;
}

impl RoundToInt for f32 {
    type Output = i32;

    #[inline(always)]
    fn round_to_int(self) -> Self::Output {
        if self >= 0.0 {
            (self + 0.5) as i32
        } else {
            (self - 0.5) as i32
        }
    }
}

#[inline(always)]
pub fn round_to_int<T: RoundToInt>(value: T) -> T::Output {
    RoundToInt::round_to_int(value)
}

pub trait InterpQuintic {
    fn interp_quintic(self) -> Self;
}

impl InterpQuintic for f32 {
    #[inline(always)]
    fn interp_quintic(self) -> Self {
        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

#[inline(always)]
pub fn interp_quintic<T: InterpQuintic>(value: T) -> T {
    InterpQuintic::interp_quintic(value)
}

pub trait InterpHermite {
    fn interp_hermite(self) -> Self;
}

impl InterpHermite for f32 {
    #[inline(always)]
    fn interp_hermite(self) -> Self {
        self * self * (3.0 - 2.0 * self)
    }
}

#[inline(always)]
pub fn interp_hermite<T: InterpHermite>(value: T) -> T {
    InterpHermite::interp_hermite(value)
}

pub trait Lerp<T = Self> {
    type Output;
    fn lerp(a: Self, b: Self, t: T) -> Self::Output;
}

impl Lerp for f32 {
    type Output = Self;

    #[inline(always)]
    fn lerp(a: Self, b: Self, t: Self) -> Self::Output {
        a + t * (b - a)
    }
}

#[inline(always)]
pub fn lerp<T, V: Lerp<T>>(a: V, b: V, t: T) -> V::Output {
    Lerp::lerp(a, b, t)
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
pub const fn fast_abs(f: f32) -> f32 {
    if f < 0.0 {
        -f
    } else {
        f
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
pub fn cell_neighbours(cell: i32) -> RangeInclusive<i32> {
    cell.wrapping_sub(1)..=cell.wrapping_add(1)
}

pub(crate) mod open_simplex_2 {
    #[cfg(feature = "nightly-simd")]
    use core::simd::f32x2;

    #[cfg(feature = "nightly-simd")]
    use super::splat;

    #[inline]
    pub(crate) fn improve2([mut x, mut y]: [f32; 2]) -> [f32; 2] {
        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const F2: f32 = 0.5 * (SQRT3 - 1.0);
        let t: f32 = (x + y) * F2;
        x += t;
        y += t;
        [x, y]
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    pub(crate) fn improve2a(point: f32x2) -> f32x2 {
        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const F2: f32 = 0.5 * (SQRT3 - 1.0);
        let t: f32 = (point[0] + point[1]) * F2;
        point + splat(t)
    }
}

#[cfg(feature = "nightly-simd")]
mod simd {
    use core::simd::{cmp::SimdPartialOrd, f32x2, f32x4, i32x2, i32x4, num::SimdFloat, simd_swizzle, LaneCount, Simd, SimdElement, SupportedLaneCount};

    use super::{dot, Dot, FloorToInt, Index2, Index3, InterpHermite, InterpQuintic, Lerp, RoundToInt, GRADIENTS_2D, GRADIENTS_3D, PRIME_X, PRIME_Y, PRIME_Z};

    pub(crate) const PRIME_XY: i32x2 = i32x2::from_array([PRIME_X, PRIME_Y]);
    pub(crate) const PRIME_XYZ: i32x4 = i32x4::from_array([PRIME_X, PRIME_Y, PRIME_Z, 0]);

    impl<const LANES: usize> FloorToInt for Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        type Output = Simd<i32, LANES>;

        #[inline(always)]
        fn floor_to_int(self) -> Self::Output {
            let int = unsafe { self.to_int_unchecked::<i32>() };
            int - self.simd_ge(splat(0.0)).select(splat(0), splat(1))
        }
    }

    impl<const LANES: usize> RoundToInt for Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        type Output = Simd<i32, LANES>;

        #[inline(always)]
        fn round_to_int(self) -> Self::Output {
            let f = self + self.simd_ge(splat(0.0)).select(splat(0.5), splat(-0.5));
            unsafe { f.to_int_unchecked() }
        }
    }

    impl<const LANES: usize> InterpQuintic for Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        #[inline(always)]
        fn interp_quintic(self) -> Self {
            self * self * self * (self * (self * splat(6.0) - splat(15.0)) + splat(10.0))
        }
    }

    impl<const LANES: usize> InterpHermite for Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        #[inline(always)]
        fn interp_hermite(self) -> Self {
            self * self * (splat(3.0) - splat(2.0) * self)
        }
    }

    impl<const LANES: usize> Lerp for Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        type Output = Self;

        #[inline(always)]
        fn lerp(a: Self, b: Self, t: Self) -> Self::Output {
            a + t * (b - a)
        }
    }

    impl<const LANES: usize> Lerp<f32> for Simd<f32, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        type Output = Self;

        #[inline(always)]
        fn lerp(a: Self, b: Self, t: f32) -> Self::Output {
            a + Simd::splat(t) * (b - a)
        }
    }

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

    pub(crate) fn splat<T, const LANES: usize>(value: T) -> Simd<T, LANES>
    where
        T: SimdElement,
        LaneCount<LANES>: SupportedLaneCount,
    {
        Simd::splat(value)
    }

    #[inline(always)]
    pub(crate) fn hash2_simd(seed: i32, primed: i32x2) -> i32 {
        (seed ^ (primed[0] ^ primed[1])).wrapping_mul(0x27d4eb2d)
    }

    #[inline(always)]
    pub(crate) fn hash3_simd(seed: i32, primed: i32x4) -> i32 {
        (seed ^ primed[0] ^ primed[1] ^ primed[2]).wrapping_mul(0x27d4eb2d)
    }

    #[inline(always)]
    pub(crate) fn grad2_simd(seed: i32, primed: i32x2, delta: f32x2) -> f32 {
        let mut hash = hash2_simd(seed, primed);
        hash ^= hash.wrapping_shr(15);

        let gradient = GRADIENTS_2D[Index2::new(hash)].0;
        dot(delta, gradient)
    }

    #[inline(always)]
    pub(crate) fn grad3_simd(seed: i32, primed: i32x4, delta: f32x4) -> f32 {
        let mut hash = hash3_simd(seed, primed);
        hash ^= hash.wrapping_shr(15);

        let gradient = GRADIENTS_3D[Index3::new(hash)].0;
        dot(delta, gradient)
    }

    #[inline(always)]
    pub(crate) fn grad3_with_hash(hash: Index3<64>, dx: f32, dy: f32, dz: f32) -> f32 {
        let g = GRADIENTS_3D[hash].0;
        dx * g[0] + dy * g[1] + dz * g[2]
    }
}

#[cfg(feature = "nightly-simd")]
pub(crate) use simd::*;
