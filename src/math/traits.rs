#[cfg(feature = "nightly-simd")]
use core::simd::{num::SimdFloat, prelude::SimdPartialOrd, Select as _, Simd, SimdElement};

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

#[cfg(feature = "nightly-simd")]
impl<const LANES: usize> FloorToInt for Simd<f32, LANES> {
    type Output = Simd<i32, LANES>;

    #[inline(always)]
    fn floor_to_int(self) -> Self::Output {
        let int = simd_float_to_int_saturating(self);
        int - self.simd_ge(splat(0.0)).select(splat(0), splat(1))
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

#[cfg(feature = "nightly-simd")]
impl<const LANES: usize> RoundToInt for Simd<f32, LANES> {
    type Output = Simd<i32, LANES>;

    #[inline(always)]
    fn round_to_int(self) -> Self::Output {
        simd_float_to_int_saturating(self + self.simd_ge(splat(0.0)).select(splat(0.5), splat(-0.5)))
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

#[cfg(feature = "nightly-simd")]
impl<const LANES: usize> InterpQuintic for Simd<f32, LANES> {
    #[inline(always)]
    fn interp_quintic(self) -> Self {
        self * self * self * (self * (self * splat(6.0) - splat(15.0)) + splat(10.0))
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

#[cfg(feature = "nightly-simd")]
impl<const LANES: usize> InterpHermite for Simd<f32, LANES> {
    #[inline(always)]
    fn interp_hermite(self) -> Self {
        self * self * (splat(3.0) - splat(2.0) * self)
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

#[cfg(feature = "nightly-simd")]
impl<const LANES: usize> Lerp for Simd<f32, LANES> {
    type Output = Self;

    #[inline(always)]
    fn lerp(a: Self, b: Self, t: Self) -> Self::Output {
        a + t * (b - a)
    }
}

#[cfg(feature = "nightly-simd")]
impl<const LANES: usize> Lerp<f32> for Simd<f32, LANES> {
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
{
    Simd::splat(value)
}

/// Acts like `float_value as i32` (rounds down and saturates when out of bounds) but on a SIMD vector.
///
/// Differs in that values above i32::MAX are clamped slightly below it (for efficient implementation).
#[cfg(feature = "nightly-simd")]
fn simd_float_to_int_saturating<const LANES: usize>(floats: Simd<f32, LANES>) -> Simd<i32, LANES> {
    let not_nan = floats.is_nan().select(Simd::<f32, LANES>::splat(0.0), floats);

    let lower_bound = const { Simd::splat(i32::MIN as f32) };
    // i32::MAX gets rounded up when converted to f32, so we have to take a step down to stay in bounds of i32
    let upper_bound = const { Simd::splat((i32::MAX as f32).next_down()) };
    let clamped = not_nan.simd_clamp(lower_bound, upper_bound);

    // SAFETY: we have adjusted the input to be representable as i32
    unsafe { clamped.to_int_unchecked::<i32>() }
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly-simd")]
    use super::*;

    /// Note: this test is meant to be run under Miri to check soundness of the `to_int_unchecked` usage.
    #[test]
    #[cfg(feature = "nightly-simd")]
    fn float_to_int() {
        use core::{
            f32,
            simd::{f32x4, i32x4},
        };

        #[track_caller]
        fn check_case(input: f32, expected_output: i32) {
            assert_eq!(input as i32, expected_output, "expectation of {expected_output:?} doesn't match `{input:?} as i32`");
            check_case_without_comparison(input, expected_output);
        }
        #[track_caller]
        fn check_case_without_comparison(input: f32, expected_output: i32) {
            // values in other lanes confirm the lanes are acting independently
            assert_eq!(
                simd_float_to_int_saturating(f32x4::from([input, 0.0, 1.0, 3.5])),
                i32x4::from([expected_output, 0, 1, 3]),
                "simd {input}"
            );
        }

        check_case(f32::NAN, 0);
        check_case(-f32::INFINITY, i32::MIN);
        check_case(const { (i32::MIN as f32).next_down() }, i32::MIN);
        check_case(const { i32::MIN as f32 }, i32::MIN);
        check_case(const { (i32::MIN as f32).next_up() }, -2147483520);
        check_case(const { i32::MIN as f32 }, i32::MIN);
        check_case(-1.5, -1);
        check_case(-0.0, 0);
        check_case(0.0, 0);
        check_case(1.5, 1);
        check_case(const { (i32::MAX as f32).next_down() }, 2147483520);
        check_case_without_comparison(const { i32::MAX as f32 }, 2147483520);
        check_case_without_comparison(const { (i32::MAX as f32).next_up() }, 2147483520);
    }
}
