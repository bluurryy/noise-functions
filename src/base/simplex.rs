use crate::{base::impl_noise, floor};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

/// 2/3/4 dimensional Simplex noise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simplex;

impl_noise!(234 Simplex);

impl Simplex {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoise2
        use crate::from_fast_noise_2::{gradient_dot2, hash_primes2, max, mul_add, primes};

        const SQRT3: f32 = 1.7320508075688772935274463415059;
        const F2: f32 = 0.5 * (SQRT3 - 1.0);
        const G2: f32 = (3.0 - SQRT3) / 6.0;

        let f = F2 * (x + y);
        let mut x0 = floor(x + f);
        let mut y0 = floor(y + f);

        let i = (x0 as i32).wrapping_mul(primes::X);
        let j = (y0 as i32).wrapping_mul(primes::Y);

        let g = G2 * (x0 + y0);
        x0 = x - (x0 - g);
        y0 = y - (y0 - g);

        let i1 = x0 > y0;

        let x1 = if i1 { x0 - 1.0 } else { x0 } + G2;
        let y1 = if i1 { y0 } else { y0 - 1.0 } + G2;

        let x2 = x0 + (G2 * 2.0 - 1.0);
        let y2 = y0 + (G2 * 2.0 - 1.0);

        let mut t0 = 0.5 - (x0 * x0) - (y0 * y0);
        let mut t1 = 0.5 - (x1 * x1) - (y1 * y1);
        let mut t2 = 0.5 - (x2 * x2) - (y2 * y2);

        t0 = max(t0, 0.0);
        t1 = max(t1, 0.0);
        t2 = max(t2, 0.0);

        t0 *= t0;
        t0 *= t0;

        t1 *= t1;
        t1 *= t1;

        t2 *= t2;
        t2 *= t2;

        let n0 = gradient_dot2(hash_primes2(seed, i, j), x0, y0);
        let n1 = gradient_dot2(hash_primes2(seed, if i1 { i.wrapping_add(primes::X) } else { i }, if i1 { j } else { j.wrapping_add(primes::Y) }), x1, y1);
        let n2 = gradient_dot2(hash_primes2(seed, i.wrapping_add(primes::X), j.wrapping_add(primes::Y)), x2, y2);

        38.283687591552734375 * mul_add(n0, t0, mul_add(n1, t1, n2 * t2))
    }

    #[inline]
    fn gen3(self, [mut x, mut y, mut z]: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoise2
        use crate::from_fast_noise_2::{gradient_dot3, hash_primes3, masked_add, masked_sub, max, mul_add, nmasked_add, nmasked_sub, nmul_add, primes};

        const F3: f32 = 1.0 / 3.0;
        const G3: f32 = 1.0 / 2.0;

        let s = F3 * (x + y + z);
        x += s;
        y += s;
        z += s;

        let mut x0 = floor(x);
        let mut y0 = floor(y);
        let mut z0 = floor(z);
        let xi = x - x0;
        let yi = y - y0;
        let zi = z - z0;

        let i = (x0 as i32).wrapping_mul(primes::X);
        let j = (y0 as i32).wrapping_mul(primes::Y);
        let k = (z0 as i32).wrapping_mul(primes::Z);

        let x_ge_y = xi >= yi;
        let y_ge_z = yi >= zi;
        let x_ge_z = xi >= zi;

        let g = G3 * (xi + yi + zi);
        x0 = xi - g;
        y0 = yi - g;
        z0 = zi - g;

        let i1 = x_ge_y & x_ge_z;
        let j1 = y_ge_z & !x_ge_y;
        let k1 = !x_ge_z & !y_ge_z;

        let i2 = x_ge_y | x_ge_z;
        let j2 = !x_ge_y | y_ge_z;
        let k2 = x_ge_z & y_ge_z; //NMasked

        let x1 = masked_sub(x0, 1.0, i1) + G3;
        let y1 = masked_sub(y0, 1.0, j1) + G3;
        let z1 = masked_sub(z0, 1.0, k1) + G3;
        let x2 = masked_sub(x0, 1.0, i2) + G3 * 2.0;
        let y2 = masked_sub(y0, 1.0, j2) + G3 * 2.0;
        let z2 = nmasked_sub(z0, 1.0, k2) + G3 * 2.0;
        let x3 = x0 + (G3 * 3.0 - 1.0);
        let y3 = y0 + (G3 * 3.0 - 1.0);
        let z3 = z0 + (G3 * 3.0 - 1.0);

        let mut t0 = nmul_add(x0, x0, nmul_add(y0, y0, nmul_add(z0, z0, 0.6)));
        let mut t1 = nmul_add(x1, x1, nmul_add(y1, y1, nmul_add(z1, z1, 0.6)));
        let mut t2 = nmul_add(x2, x2, nmul_add(y2, y2, nmul_add(z2, z2, 0.6)));
        let mut t3 = nmul_add(x3, x3, nmul_add(y3, y3, nmul_add(z3, z3, 0.6)));

        t0 = max(t0, 0.0);
        t1 = max(t1, 0.0);
        t2 = max(t2, 0.0);
        t3 = max(t3, 0.0);

        t0 *= t0;
        t0 *= t0;
        t1 *= t1;
        t1 *= t1;
        t2 *= t2;
        t2 *= t2;
        t3 *= t3;
        t3 *= t3;

        let n0 = gradient_dot3(hash_primes3(seed, i, j, k), x0, y0, z0);
        let n1 = gradient_dot3(hash_primes3(seed, masked_add(i, primes::X, i1), masked_add(j, primes::Y, j1), masked_add(k, primes::Z, k1)), x1, y1, z1);
        let n2 = gradient_dot3(hash_primes3(seed, masked_add(i, primes::X, i2), masked_add(j, primes::Y, j2), nmasked_add(k, primes::Z, k2)), x2, y2, z2);
        let n3 = gradient_dot3(hash_primes3(seed, i.wrapping_add(primes::X), j.wrapping_add(primes::Y), k.wrapping_add(primes::Z)), x3, y3, z3);

        32.69428253173828125 * mul_add(n0, t0, mul_add(n1, t1, mul_add(n2, t2, n3 * t3)))
    }

    #[inline]
    fn gen4(self, [mut x, mut y, mut z, mut w]: [f32; 4], seed: i32) -> f32 {
        // implementation from FastNoise2
        use crate::from_fast_noise_2::{gradient_dot4, hash_primes4, masked_add, masked_inc, masked_sub, max, mul_add, nmul_add, primes};

        const SQRT5: f32 = 2.236067977499;
        const F4: f32 = (SQRT5 - 1.0) / 4.0;
        const G4: f32 = (5.0 - SQRT5) / 20.0;

        let s = F4 * (x + y + z + w);
        x += s;
        y += s;
        z += s;
        w += s;

        let mut x0 = floor(x);
        let mut y0 = floor(y);
        let mut z0 = floor(z);
        let mut w0 = floor(w);
        let xi = x - x0;
        let yi = y - y0;
        let zi = z - z0;
        let wi = w - w0;

        let i = (x0 as i32).wrapping_mul(primes::X);
        let j = (y0 as i32).wrapping_mul(primes::Y);
        let k = (z0 as i32).wrapping_mul(primes::Z);
        let l = (w0 as i32).wrapping_mul(primes::W);

        let g = G4 * (xi + yi + zi + wi);
        x0 = xi - g;
        y0 = yi - g;
        z0 = zi - g;
        w0 = wi - g;

        let mut rankx = 0;
        let mut ranky = 0;
        let mut rankz = 0;
        let mut rankw = 0;

        let x_ge_y = x0 >= y0;
        rankx = masked_inc(rankx, x_ge_y);
        ranky = masked_inc(ranky, !x_ge_y);

        let x_ge_z = x0 >= z0;
        rankx = masked_inc(rankx, x_ge_z);
        rankz = masked_inc(rankz, !x_ge_z);

        let x_ge_w = x0 >= w0;
        rankx = masked_inc(rankx, x_ge_w);
        rankw = masked_inc(rankw, !x_ge_w);

        let y_ge_z = y0 >= z0;
        ranky = masked_inc(ranky, y_ge_z);
        rankz = masked_inc(rankz, !y_ge_z);

        let y_ge_w = y0 >= w0;
        ranky = masked_inc(ranky, y_ge_w);
        rankw = masked_inc(rankw, !y_ge_w);

        let z_ge_w = z0 >= w0;
        rankz = masked_inc(rankz, z_ge_w);
        rankw = masked_inc(rankw, !z_ge_w);

        let i1 = rankx > 2;
        let j1 = ranky > 2;
        let k1 = rankz > 2;
        let l1 = rankw > 2;

        let i2 = rankx > 1;
        let j2 = ranky > 1;
        let k2 = rankz > 1;
        let l2 = rankw > 1;

        let i3 = rankx > 0;
        let j3 = ranky > 0;
        let k3 = rankz > 0;
        let l3 = rankw > 0;

        let x1 = masked_sub(x0, 1.0, i1) + G4;
        let y1 = masked_sub(y0, 1.0, j1) + G4;
        let z1 = masked_sub(z0, 1.0, k1) + G4;
        let w1 = masked_sub(w0, 1.0, l1) + G4;
        let x2 = masked_sub(x0, 1.0, i2) + G4 * 2.0;
        let y2 = masked_sub(y0, 1.0, j2) + G4 * 2.0;
        let z2 = masked_sub(z0, 1.0, k2) + G4 * 2.0;
        let w2 = masked_sub(w0, 1.0, l2) + G4 * 2.0;
        let x3 = masked_sub(x0, 1.0, i3) + G4 * 3.0;
        let y3 = masked_sub(y0, 1.0, j3) + G4 * 3.0;
        let z3 = masked_sub(z0, 1.0, k3) + G4 * 3.0;
        let w3 = masked_sub(w0, 1.0, l3) + G4 * 3.0;
        let x4 = x0 + (G4 * 4.0 - 1.0);
        let y4 = y0 + (G4 * 4.0 - 1.0);
        let z4 = z0 + (G4 * 4.0 - 1.0);
        let w4 = w0 + (G4 * 4.0 - 1.0);

        let mut t0 = nmul_add(x0, x0, nmul_add(y0, y0, nmul_add(z0, z0, nmul_add(w0, w0, 0.6))));
        let mut t1 = nmul_add(x1, x1, nmul_add(y1, y1, nmul_add(z1, z1, nmul_add(w1, w1, 0.6))));
        let mut t2 = nmul_add(x2, x2, nmul_add(y2, y2, nmul_add(z2, z2, nmul_add(w2, w2, 0.6))));
        let mut t3 = nmul_add(x3, x3, nmul_add(y3, y3, nmul_add(z3, z3, nmul_add(w3, w3, 0.6))));
        let mut t4 = nmul_add(x4, x4, nmul_add(y4, y4, nmul_add(z4, z4, nmul_add(w4, w4, 0.6))));

        t0 = max(t0, 0.0);
        t1 = max(t1, 0.0);
        t2 = max(t2, 0.0);
        t3 = max(t3, 0.0);
        t4 = max(t4, 0.0);

        t0 *= t0;
        t0 *= t0;
        t1 *= t1;
        t1 *= t1;
        t2 *= t2;
        t2 *= t2;
        t3 *= t3;
        t3 *= t3;
        t4 *= t4;
        t4 *= t4;

        let n0 = gradient_dot4(hash_primes4(seed, i, j, k, l), x0, y0, z0, w0);
        let n1 = gradient_dot4(
            hash_primes4(seed, masked_add(i, primes::X, i1), masked_add(j, primes::Y, j1), masked_add(k, primes::Z, k1), masked_add(l, primes::W, l1)),
            x1,
            y1,
            z1,
            w1,
        );
        let n2 = gradient_dot4(
            hash_primes4(seed, masked_add(i, primes::X, i2), masked_add(j, primes::Y, j2), masked_add(k, primes::Z, k2), masked_add(l, primes::W, l2)),
            x2,
            y2,
            z2,
            w2,
        );
        let n3 = gradient_dot4(
            hash_primes4(seed, masked_add(i, primes::X, i3), masked_add(j, primes::Y, j3), masked_add(k, primes::Z, k3), masked_add(l, primes::W, l3)),
            x3,
            y3,
            z3,
            w3,
        );
        let n4 = gradient_dot4(
            hash_primes4(seed, i.wrapping_add(primes::X), j.wrapping_add(primes::Y), k.wrapping_add(primes::Z), l.wrapping_add(primes::W)),
            x4,
            y4,
            z4,
            w4,
        );

        27.0 * mul_add(n0, t0, mul_add(n1, t1, mul_add(n2, t2, mul_add(n3, t3, n4 * t4))))
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

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen4a(self, point: f32x4, seed: i32) -> f32 {
        self.gen4(point.into(), seed)
    }
}
