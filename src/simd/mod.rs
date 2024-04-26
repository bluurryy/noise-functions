pub mod cell_distance;
pub mod cell_distance_sq;
pub mod cell_value;
pub mod open_simplex_2;
pub mod open_simplex_2s;
pub mod perlin;
pub mod value;
pub mod value_cubic;

use crate::private_prelude::*;

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
fn hash2_simd(seed: i32, primed: i32x2) -> i32 {
    (seed ^ (primed[0] ^ primed[1])).wrapping_mul(0x27d4eb2d)
}

#[inline(always)]
fn hash3_simd(seed: i32, primed: i32x4) -> i32 {
    (seed ^ primed[0] ^ primed[1] ^ primed[2]).wrapping_mul(0x27d4eb2d)
}

#[inline(always)]
fn grad2_simd(seed: i32, primed: i32x2, delta: f32x2) -> f32 {
    let mut hash = hash2_simd(seed, primed);
    hash ^= hash.wrapping_shr(15);

    let gradient = GRADIENTS_2D[Index2::new(hash)].0;
    dot(delta, gradient)
}

#[inline(always)]
fn grad3_simd(seed: i32, primed: i32x4, delta: f32x4) -> f32 {
    let mut hash = hash3_simd(seed, primed);
    hash ^= hash.wrapping_shr(15);

    let gradient = GRADIENTS_3D[Index3::new(hash)].0;
    dot(delta, gradient)
}
