use crate::{base::impl_noise, open_simplex_2::improve3, Sample, SampleWithSeed};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[cfg(feature = "nightly-simd")]
use crate::open_simplex_2::improve3a;

/// 2/3 dimensional OpenSimplex2s noise. Smooth variant.
///
/// When sampling in 3 Dimensions you can improve the visual isotropy in a the respective planes via [`improve_xy`] or [`improve_xz`].
///
/// [`improve_xy`]: Self::improve_xy
/// [`improve_xz`]: Self::improve_xz
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSimplex2s;

impl_noise!(2 OpenSimplex2s);

impl Sample<3> for OpenSimplex2s {
    #[inline(always)]
    fn sample(&self, point: [f32; 3]) -> f32 {
        self.gen3(improve3(point), 0)
    }
}

impl SampleWithSeed<3> for OpenSimplex2s {
    #[inline(always)]
    fn sample_with_seed(&self, point: [f32; 3], seed: i32) -> f32 {
        self.gen3(improve3(point), seed)
    }
}

#[cfg(feature = "nightly-simd")]
impl Sample<3, core::simd::f32x4> for OpenSimplex2s {
    #[inline(always)]
    fn sample(&self, point: core::simd::f32x4) -> f32 {
        self.gen3a(improve3a(point), 0)
    }
}

#[cfg(feature = "nightly-simd")]
impl SampleWithSeed<3, core::simd::f32x4> for OpenSimplex2s {
    #[inline(always)]
    fn sample_with_seed(&self, point: core::simd::f32x4, seed: i32) -> f32 {
        self.gen3a(improve3a(point), seed)
    }
}

impl OpenSimplex2s {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad2, open_simplex_2::improve2, PRIME_X, PRIME_Y};

        let [x, y] = improve2([x, y]);

        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const G2: f32 = (3.0 - SQRT3) / 6.0;

        let mut i: i32 = floor_to_int(x);
        let mut j: i32 = floor_to_int(y);

        let xi: f32 = x - i as f32;
        let yi: f32 = y - j as f32;

        i = i.wrapping_mul(PRIME_X);
        j = j.wrapping_mul(PRIME_Y);

        let i1: i32 = i.wrapping_add(PRIME_X);
        let j1: i32 = j.wrapping_add(PRIME_Y);

        let t: f32 = (xi + yi) * G2;
        let x0: f32 = xi - t;
        let y0: f32 = yi - t;

        let a0: f32 = (2.0 / 3.0) - x0 * x0 - y0 * y0;
        let mut value: f32 = (a0 * a0) * (a0 * a0) * grad2(seed, i, j, x0, y0);

        let a1: f32 = (2.0 * (1.0 - 2.0 * G2) * (1.0 / G2 - 2.0)) * t + ((-2.0 * (1.0 - 2.0 * G2) * (1.0 - 2.0 * G2)) + a0);
        let x1: f32 = x0 - (1.0 - 2.0 * G2);
        let y1: f32 = y0 - (1.0 - 2.0 * G2);
        value += (a1 * a1) * (a1 * a1) * grad2(seed, i1, j1, x1, y1);

        // Nested conditionals were faster than compact bit logic/arithmetic.
        let xmyi: f32 = xi - yi;
        if t > G2 {
            if xi + xmyi > 1.0 {
                let x2: f32 = x0 + (3.0 * G2 - 2.0);
                let y2: f32 = y0 + (3.0 * G2 - 1.0);
                let a2: f32 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, i.wrapping_add(PRIME_X << 1), j.wrapping_add(PRIME_Y), x2, y2);
                }
            } else {
                let x2: f32 = x0 + G2;
                let y2: f32 = y0 + (G2 - 1.0);
                let a2: f32 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, i, j.wrapping_add(PRIME_Y), x2, y2);
                }
            }

            if yi - xmyi > 1.0 {
                let x3: f32 = x0 + (3.0 * G2 - 1.0);
                let y3: f32 = y0 + (3.0 * G2 - 2.0);
                let a3: f32 = (2.0 / 3.0) - x3 * x3 - y3 * y3;
                if a3 > 0.0 {
                    value += (a3 * a3) * (a3 * a3) * grad2(seed, i.wrapping_add(PRIME_X), j.wrapping_add(PRIME_Y.wrapping_shl(1)), x3, y3);
                }
            } else {
                let x3: f32 = x0 + (G2 - 1.0);
                let y3: f32 = y0 + G2;
                let a3: f32 = (2.0 / 3.0) - x3 * x3 - y3 * y3;
                if a3 > 0.0 {
                    value += (a3 * a3) * (a3 * a3) * grad2(seed, i.wrapping_add(PRIME_X), j, x3, y3);
                }
            }
        } else {
            if xi + xmyi < 0.0 {
                let x2: f32 = x0 + (1.0 - G2);
                let y2: f32 = y0 - G2;
                let a2: f32 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, i.wrapping_sub(PRIME_X), j, x2, y2);
                }
            } else {
                let x2: f32 = x0 + (G2 - 1.0);
                let y2: f32 = y0 + G2;
                let a2: f32 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, i.wrapping_add(PRIME_X), j, x2, y2);
                }
            }

            if yi < xmyi {
                let x2: f32 = x0 - G2;
                let y2: f32 = y0 - (G2 - 1.0);
                let a2: f32 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, i, j.wrapping_sub(PRIME_Y), x2, y2);
                }
            } else {
                let x2: f32 = x0 + G2;
                let y2: f32 = y0 + (G2 - 1.0);
                let a2: f32 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
                if a2 > 0.0 {
                    value += (a2 * a2) * (a2 * a2) * grad2(seed, i, j.wrapping_add(PRIME_Y), x2, y2);
                }
            }
        }

        value * 18.24196194486065
    }

    #[inline]
    fn gen3(self, [x, y, z]: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{floor_to_int, grad3, PRIME_X, PRIME_Y, PRIME_Z};

        let mut i: i32 = floor_to_int(x);
        let mut j: i32 = floor_to_int(y);
        let mut k: i32 = floor_to_int(z);
        let xi: f32 = x - i as f32;
        let yi: f32 = y - j as f32;
        let zi: f32 = z - k as f32;

        i = i.wrapping_mul(PRIME_X);
        j = j.wrapping_mul(PRIME_Y);
        k = k.wrapping_mul(PRIME_Z);

        let seed2: i32 = seed.wrapping_add(1293373);

        let x_n_mask: i32 = (-0.5 - xi) as i32;
        let y_n_mask: i32 = (-0.5 - yi) as i32;
        let z_n_mask: i32 = (-0.5 - zi) as i32;

        let x0: f32 = xi + x_n_mask as f32;
        let y0: f32 = yi + y_n_mask as f32;
        let z0: f32 = zi + z_n_mask as f32;
        let a0: f32 = 0.75 - x0 * x0 - y0 * y0 - z0 * z0;
        let mut value: f32 = (a0 * a0)
            * (a0 * a0)
            * grad3(
                seed,
                i.wrapping_add(x_n_mask & PRIME_X),
                j.wrapping_add(y_n_mask & PRIME_Y),
                k.wrapping_add(z_n_mask & PRIME_Z),
                x0,
                y0,
                z0,
            );

        let x1: f32 = xi - 0.5;
        let y1: f32 = yi - 0.5;
        let z1: f32 = zi - 0.5;
        let a1: f32 = 0.75 - x1 * x1 - y1 * y1 - z1 * z1;
        value += (a1 * a1) * (a1 * a1) * grad3(seed2, i.wrapping_add(PRIME_X), j.wrapping_add(PRIME_Y), k.wrapping_add(PRIME_Z), x1, y1, z1);

        let x_a_flip_mask_0: f32 = (x_n_mask | 1).wrapping_shl(1) as f32 * x1;
        let y_a_flip_mask_0: f32 = (y_n_mask | 1).wrapping_shl(1) as f32 * y1;
        let z_a_flip_mask_0: f32 = (z_n_mask | 1).wrapping_shl(1) as f32 * z1;
        let x_a_flip_mask_1: f32 = (-2i32).wrapping_sub(x_n_mask.wrapping_shl(2)) as f32 * x1 - 1.0;
        let y_a_flip_mask_1: f32 = (-2i32).wrapping_sub(y_n_mask.wrapping_shl(2)) as f32 * y1 - 1.0;
        let z_a_flip_mask_1: f32 = (-2i32).wrapping_sub(z_n_mask.wrapping_shl(2)) as f32 * z1 - 1.0;

        let mut skip_5 = false;
        let a2: f32 = x_a_flip_mask_0 + a0;
        if a2 > 0.0 {
            let x2: f32 = x0 - (x_n_mask | 1) as f32;
            let y2: f32 = y0;
            let z2: f32 = z0;
            value += (a2 * a2)
                * (a2 * a2)
                * grad3(
                    seed,
                    i.wrapping_add(!x_n_mask & PRIME_X),
                    j.wrapping_add(y_n_mask & PRIME_Y),
                    k.wrapping_add(z_n_mask & PRIME_Z),
                    x2,
                    y2,
                    z2,
                );
        } else {
            let a3: f32 = y_a_flip_mask_0 + z_a_flip_mask_0 + a0;
            if a3 > 0.0 {
                let x3: f32 = x0;
                let y3: f32 = y0 - (y_n_mask | 1) as f32;
                let z3: f32 = z0 - (z_n_mask | 1) as f32;
                value += (a3 * a3)
                    * (a3 * a3)
                    * grad3(
                        seed,
                        i.wrapping_add(x_n_mask & PRIME_X),
                        j.wrapping_add(!y_n_mask & PRIME_Y),
                        k.wrapping_add(!z_n_mask & PRIME_Z),
                        x3,
                        y3,
                        z3,
                    );
            }

            let a4: f32 = x_a_flip_mask_1 + a1;
            if a4 > 0.0 {
                let x4: f32 = (x_n_mask | 1) as f32 + x1;
                let y4: f32 = y1;
                let z4: f32 = z1;
                value += (a4 * a4)
                    * (a4 * a4)
                    * grad3(
                        seed2,
                        i.wrapping_add(x_n_mask & (PRIME_X.wrapping_mul(2))),
                        j.wrapping_add(PRIME_Y),
                        k.wrapping_add(PRIME_Z),
                        x4,
                        y4,
                        z4,
                    );
                skip_5 = true;
            }
        }

        let mut skip_9 = false;
        let a6: f32 = y_a_flip_mask_0 + a0;
        if a6 > 0.0 {
            let x6: f32 = x0;
            let y6: f32 = y0 - (y_n_mask | 1) as f32;
            let z6: f32 = z0;
            value += (a6 * a6)
                * (a6 * a6)
                * grad3(
                    seed,
                    i.wrapping_add(x_n_mask & PRIME_X),
                    j.wrapping_add(!y_n_mask & PRIME_Y),
                    k.wrapping_add(z_n_mask & PRIME_Z),
                    x6,
                    y6,
                    z6,
                );
        } else {
            let a7: f32 = x_a_flip_mask_0 + z_a_flip_mask_0 + a0;
            if a7 > 0.0 {
                let x7: f32 = x0 - (x_n_mask | 1) as f32;
                let y7: f32 = y0;
                let z7: f32 = z0 - (z_n_mask | 1) as f32;
                value += (a7 * a7)
                    * (a7 * a7)
                    * grad3(
                        seed,
                        i.wrapping_add(!x_n_mask & PRIME_X),
                        j.wrapping_add(y_n_mask & PRIME_Y),
                        k.wrapping_add(!z_n_mask & PRIME_Z),
                        x7,
                        y7,
                        z7,
                    );
            }

            let a8: f32 = y_a_flip_mask_1 + a1;
            if a8 > 0.0 {
                let x8: f32 = x1;
                let y8: f32 = (y_n_mask | 1) as f32 + y1;
                let z8: f32 = z1;
                value += (a8 * a8) * (a8 * a8) * grad3(seed2, i.wrapping_add(PRIME_X), j.wrapping_add(y_n_mask & PRIME_Y.wrapping_shl(1)), k.wrapping_add(PRIME_Z), x8, y8, z8);
                skip_9 = true;
            }
        }

        let mut skip_d = false;
        let a_a: f32 = z_a_flip_mask_0 + a0;
        if a_a > 0.0 {
            let x_a: f32 = x0;
            let y_a: f32 = y0;
            let z_a: f32 = z0 - (z_n_mask | 1) as f32;
            value += (a_a * a_a)
                * (a_a * a_a)
                * grad3(
                    seed,
                    i.wrapping_add(x_n_mask & PRIME_X),
                    j.wrapping_add(y_n_mask & PRIME_Y),
                    k.wrapping_add(!z_n_mask & PRIME_Z),
                    x_a,
                    y_a,
                    z_a,
                );
        } else {
            let a_b: f32 = x_a_flip_mask_0 + y_a_flip_mask_0 + a0;
            if a_b > 0.0 {
                let x_b: f32 = x0 - (x_n_mask | 1) as f32;
                let y_b: f32 = y0 - (y_n_mask | 1) as f32;
                let z_b: f32 = z0;
                value += (a_b * a_b)
                    * (a_b * a_b)
                    * grad3(
                        seed,
                        i.wrapping_add(!x_n_mask & PRIME_X),
                        j.wrapping_add(!y_n_mask & PRIME_Y),
                        k.wrapping_add(z_n_mask & PRIME_Z),
                        x_b,
                        y_b,
                        z_b,
                    );
            }

            let a_c: f32 = z_a_flip_mask_1 + a1;
            if a_c > 0.0 {
                let x_c: f32 = x1;
                let y_c: f32 = y1;
                let z_c: f32 = (z_n_mask | 1) as f32 + z1;
                value += (a_c * a_c)
                    * (a_c * a_c)
                    * grad3(
                        seed2,
                        i.wrapping_add(PRIME_X),
                        j.wrapping_add(PRIME_Y),
                        k.wrapping_add(z_n_mask & PRIME_Z.wrapping_shl(1)),
                        x_c,
                        y_c,
                        z_c,
                    );
                skip_d = true;
            }
        }

        if !skip_5 {
            let a5: f32 = y_a_flip_mask_1 + z_a_flip_mask_1 + a1;
            if a5 > 0.0 {
                let x5: f32 = x1;
                let y5: f32 = (y_n_mask | 1) as f32 + y1;
                let z5: f32 = (z_n_mask | 1) as f32 + z1;
                value += (a5 * a5)
                    * (a5 * a5)
                    * grad3(
                        seed2,
                        i.wrapping_add(PRIME_X),
                        j.wrapping_add(y_n_mask & PRIME_Y.wrapping_shl(1)),
                        k.wrapping_add(z_n_mask & PRIME_Z.wrapping_shl(1)),
                        x5,
                        y5,
                        z5,
                    );
            }
        }

        if !skip_9 {
            let a9: f32 = x_a_flip_mask_1 + z_a_flip_mask_1 + a1;
            if a9 > 0.0 {
                let x9: f32 = (x_n_mask | 1) as f32 + x1;
                let y9: f32 = y1;
                let z9: f32 = (z_n_mask | 1) as f32 + z1;
                value += (a9 * a9)
                    * (a9 * a9)
                    * grad3(
                        seed2,
                        i.wrapping_add(x_n_mask & PRIME_X.wrapping_mul(2)),
                        j.wrapping_add(PRIME_Y),
                        k.wrapping_add(z_n_mask & PRIME_Z.wrapping_shl(1)),
                        x9,
                        y9,
                        z9,
                    );
            }
        }

        if !skip_d {
            let a_d: f32 = x_a_flip_mask_1 + y_a_flip_mask_1 + a1;
            if a_d > 0.0 {
                let x_d: f32 = (x_n_mask | 1) as f32 + x1;
                let y_d: f32 = (y_n_mask | 1) as f32 + y1;
                let z_d: f32 = z1;
                value += (a_d * a_d)
                    * (a_d * a_d)
                    * grad3(
                        seed2,
                        i.wrapping_add(x_n_mask & PRIME_X.wrapping_shl(1)),
                        j.wrapping_add(y_n_mask & PRIME_Y.wrapping_shl(1)),
                        k.wrapping_add(PRIME_Z),
                        x_d,
                        y_d,
                        z_d,
                    );
            }
        }

        value * 9.046026385208288
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        self.gen2(point.into(), seed)
    }

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        self.gen3(*crate::array_4_take_3(point.as_array()), seed)
    }
}
