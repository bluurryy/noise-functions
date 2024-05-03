use crate::private_prelude::*;

/// Improves 3D orientation as a fallback.
pub struct Improve3<Noise>(pub Noise);

/// Improves 3D orientation for the `XY` plane.
pub struct Improve3Xy<Noise>(pub Noise);

/// Improves 3D orientation for the `XZ` plane.
pub struct Improve3Xz<Noise>(pub Noise);

mod scalar_improve {
    use super::*;

    impl<Noise> Sample<3> for Improve3<Noise>
    where
        Noise: Sample<3>,
    {
        fn sample(&self, pos: [f32; 3]) -> f32 {
            self.0.sample(improve3(pos))
        }
    }

    impl<Noise> Sample<3> for Improve3Xy<Noise>
    where
        Noise: Sample<3>,
    {
        fn sample(&self, pos: [f32; 3]) -> f32 {
            self.0.sample(improve3_xy(pos))
        }
    }

    impl<Noise> Sample<3> for Improve3Xz<Noise>
    where
        Noise: Sample<3>,
    {
        fn sample(&self, pos: [f32; 3]) -> f32 {
            self.0.sample(improve3_xz(pos))
        }
    }

    #[inline]
    pub fn improve3([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        const R3: f32 = 2.0 / 3.0;
        let r: f32 = (x + y + z) * R3; // Rotation, not skew
        x = r - x;
        y = r - y;
        z = r - z;
        [x, y, z]
    }

    #[inline]
    pub fn improve3_xy([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        let xy: f32 = x + y;
        let s2: f32 = xy * -0.211324865405187;
        z *= 0.577350269189626;
        x += s2 - z;
        y = y + s2 - z;
        z += xy * 0.577350269189626;
        [x, y, z]
    }

    #[inline]
    pub fn improve3_xz([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
        let xz: f32 = x + z;
        let s2: f32 = xz * -0.211324865405187;
        y *= 0.577350269189626;
        x += s2 - y;
        z += s2 - y;
        y += xz * 0.577350269189626;
        [x, y, z]
    }
}

#[cfg(feature = "nightly-simd")]
mod simd_improve {
    use super::*;

    impl<Noise> Sample<3, f32x4> for Improve3<Noise>
    where
        Noise: Sample<3, f32x4>,
    {
        fn sample(&self, pos: f32x4) -> f32 {
            self.0.sample(improve3(pos))
        }
    }

    impl<Noise> Sample<3, f32x4> for Improve3Xy<Noise>
    where
        Noise: Sample<3, f32x4>,
    {
        fn sample(&self, pos: f32x4) -> f32 {
            self.0.sample(improve3_xy(pos))
        }
    }

    impl<Noise> Sample<3, f32x4> for Improve3Xz<Noise>
    where
        Noise: Sample<3, f32x4>,
    {
        fn sample(&self, pos: f32x4) -> f32 {
            self.0.sample(improve3_xz(pos))
        }
    }

    #[inline]
    pub fn improve3(pos: f32x4) -> f32x4 {
        const R3: f32 = 2.0 / 3.0;
        let r: f32 = (pos[0] + pos[1] + pos[2]) * R3; // Rotation, not skew
        splat(r) - pos
    }

    #[inline]
    pub fn improve3_xy(pos: f32x4) -> f32x4 {
        let &[x, y, z, _] = pos.as_array();
        let [x, y, z] = super::scalar_improve::improve3_xy([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }

    #[inline]
    pub fn improve3_xz(pos: f32x4) -> f32x4 {
        let &[x, y, z, _] = pos.as_array();
        let [x, y, z] = super::scalar_improve::improve3_xz([x, y, z]);
        f32x4::from_array([x, y, z, z])
    }
}
