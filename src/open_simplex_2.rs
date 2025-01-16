#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

use crate::{Noise, OpenSimplex2, OpenSimplex2s, Sample, SampleWithSeed};

macro_rules! impl_improves {
    (
        $(#[$trait_attrs:meta])*
        $trait:ident for $($noise:ident),*;
        $(
            $(#[$improve_attrs:meta])*
            $improve_struct:ident $improve_fn:ident use $improve:ident $improve_a:ident;
        )*
    ) => {
        $(#[$trait_attrs])*
        pub trait $trait: Noise {
            $(
                $(#[$improve_attrs])*
                fn $improve_fn(self) -> $improve_struct<Self>
                where
                    Self: Sized,
                {
                    $improve_struct(self)
                }
            )*
        }

        $(
            impl $trait for $noise {}
        )*

        $(
            $(#[$improve_attrs])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $improve_struct<OpenSimplexNoise>(pub OpenSimplexNoise);

            impl<N> Noise for $improve_struct<N> {}
            impl<N> $trait for $improve_struct<N> {}

            impl<N: Sample<2>> Sample<2> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: [f32; 2]) -> f32 {
                    self.0.sample(point)
                }
            }

            impl<N: SampleWithSeed<2>> SampleWithSeed<2> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 2], seed: i32) -> f32 {
                    self.0.sample_with_seed(point, seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: Sample<2, f32x2>> Sample<2, f32x2> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: f32x2) -> f32 {
                    self.0.sample(point)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: SampleWithSeed<2, f32x2>> SampleWithSeed<2, core::simd::f32x2> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: f32x2, seed: i32) -> f32 {
                    self.0.sample_with_seed(point, seed)
                }
            }

            impl<N: Sample<3>> Sample<3> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: [f32; 3]) -> f32 {
                    self.0.sample($improve(point))
                }
            }

            impl<N: SampleWithSeed<3>> SampleWithSeed<3> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
                    self.0.sample_with_seed($improve(point), seed)
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: Sample<3, f32x4>> Sample<3, f32x4> for $improve_struct<N> {
                #[inline(always)]
                fn sample(&self, point: f32x4) -> f32 {
                    self.0.sample($improve_a(point))
                }
            }

            #[cfg(feature = "nightly-simd")]
            impl<N: SampleWithSeed<3, f32x4>> SampleWithSeed<3, f32x4> for $improve_struct<N> {
                #[inline(always)]
                fn sample_with_seed(&self, point: f32x4, seed: i32) -> f32 {
                    self.0.sample_with_seed($improve_a(point), seed)
                }
            }
        )*
    };
}

impl_improves! {
    /// Provides modifier methods for `OpenSimplex` noises.
    OpenSimplexNoise for OpenSimplex2, OpenSimplex2s;

    /// Improves 3D orientation for the `XY` plane.
    ImproveXy improve_xy use improve3_xy improve3a_xy;

    /// Improves 3D orientation for the `XZ` plane.
    ImproveXz improve_xz use improve3_xz improve3a_xz;
}

#[inline]
pub(crate) fn improve3([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    const R3: f32 = 2.0 / 3.0;
    let r: f32 = (x + y + z) * R3; // Rotation, not skew
    x = r - x;
    y = r - y;
    z = r - z;
    [x, y, z]
}

#[inline]
pub(crate) fn improve3_xy([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    let xy: f32 = x + y;
    let s2: f32 = xy * -0.211324865405187;
    z *= 0.577350269189626;
    x += s2 - z;
    y = y + s2 - z;
    z += xy * 0.577350269189626;
    [x, y, z]
}

#[inline]
pub(crate) fn improve3_xz([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    let xz: f32 = x + z;
    let s2: f32 = xz * -0.211324865405187;
    y *= 0.577350269189626;
    x += s2 - y;
    z += s2 - y;
    y += xz * 0.577350269189626;
    [x, y, z]
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve3a(point: f32x4) -> f32x4 {
    const R3: f32 = 2.0 / 3.0;
    let r: f32 = (point[0] + point[1] + point[2]) * R3; // Rotation, not skew
    f32x4::splat(r) - point
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve3a_xy(point: f32x4) -> f32x4 {
    let &[x, y, z, _] = point.as_array();
    let [x, y, z] = improve3_xy([x, y, z]);
    f32x4::from_array([x, y, z, z])
}

#[inline]
#[cfg(feature = "nightly-simd")]
pub(crate) fn improve3a_xz(point: f32x4) -> f32x4 {
    let &[x, y, z, _] = point.as_array();
    let [x, y, z] = improve3_xz([x, y, z]);
    f32x4::from_array([x, y, z, z])
}
