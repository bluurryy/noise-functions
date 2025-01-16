#[cfg(feature = "libm")]
mod libm_math {
    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        libm::fabsf(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        libm::sqrtf(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        libm::floorf(f)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f32) -> f32 {
        libm::sinf(f)
    }

    #[inline(always)]
    pub(crate) fn cos(f: f32) -> f32 {
        libm::cosf(f)
    }

    #[inline(always)]
    pub(crate) fn min(a: f32, b: f32) -> f32 {
        libm::fminf(a, b)
    }

    #[inline(always)]
    pub(crate) fn max(a: f32, b: f32) -> f32 {
        libm::fmaxf(a, b)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
        a * b + c
    }
}

#[cfg(not(feature = "libm"))]
mod std_math {
    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        f32::abs(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        f32::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        f32::floor(f)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f32) -> f32 {
        f32::sin(f)
    }

    #[inline(always)]
    pub(crate) fn cos(f: f32) -> f32 {
        f32::cos(f)
    }

    #[inline(always)]
    pub(crate) fn min(a: f32, b: f32) -> f32 {
        f32::min(a, b)
    }

    #[inline(always)]
    pub(crate) fn max(a: f32, b: f32) -> f32 {
        f32::max(a, b)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
        f32::mul_add(a, b, c)
    }
}

#[cfg(feature = "libm")]
pub(crate) use libm_math::*;

#[cfg(not(feature = "libm"))]
pub(crate) use std_math::*;

mod traits;

pub(crate) use traits::*;
