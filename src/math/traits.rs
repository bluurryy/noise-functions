#[cfg(feature = "nightly-simd")]
use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

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

#[cfg(feature = "nightly-simd")]
pub(crate) fn splat<T, const LANES: usize>(value: T) -> Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    Simd::splat(value)
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
