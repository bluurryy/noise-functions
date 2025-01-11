#[cfg(feature = "libm")]
mod libm_math {
    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        libm::sqrtf(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        libm::truncf(f)
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
    pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
        a * b + c
    }
}

#[cfg(not(feature = "libm"))]
mod std_math {
    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        f32::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        f32::trunc(f)
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
