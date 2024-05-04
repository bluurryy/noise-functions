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
}

#[cfg(feature = "libm")]
pub(crate) use libm_math::*;

#[cfg(not(feature = "libm"))]
pub(crate) use std_math::*;

mod traits;

pub(crate) use traits::*;
