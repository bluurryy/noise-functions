use crate::{abs, floor, max, mul_add, round};

use super::{gradient_dot2_fancy, gradient_dot3, hash_primes2, hash_primes3, impl_noise23, masked_add, masked_sub, nmasked_add, nmasked_sub, nmul_add, primes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSimplex2;

impl_noise23!(OpenSimplex2);

#[inline]
fn gen2([x, y]: [f32; 2], seed: i32, _: OpenSimplex2) -> f32 {
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
    //let mut j1 = ~i1; //NMasked funcs

    let x1 = masked_sub(x0, 1.0, i1) + G2;
    let y1 = nmasked_sub(y0, 1.0, i1) + G2;
    let x2 = x0 + ((G2 * 2.0) - 1.0);
    let y2 = y0 + ((G2 * 2.0) - 1.0);

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

    let n0 = gradient_dot2_fancy(hash_primes2(seed, i, j), x0, y0);
    let n1 = gradient_dot2_fancy(hash_primes2(seed, masked_add(i, primes::X, i1), nmasked_add(j, primes::Y, i1)), x1, y1);
    let n2 = gradient_dot2_fancy(hash_primes2(seed, i.wrapping_add(primes::X), j.wrapping_add(primes::Y)), x2, y2);

    49.918426513671875 * mul_add(n0, t0, mul_add(n1, t1, n2 * t2))
}

#[inline]
fn gen3([x, y, z]: [f32; 3], mut seed: i32, _: OpenSimplex2) -> f32 {
    let f = (2.0 / 3.0) * (x + y + z);
    let mut xr = f - x;
    let mut yr = f - y;
    let mut zr = f - z;

    let mut val = 0.0;
    let mut i = 0usize;

    loop {
        let v0xr = round(xr);
        let v0yr = round(yr);
        let v0zr = round(zr);
        let d0xr = xr - v0xr;
        let d0yr = yr - v0yr;
        let d0zr = zr - v0zr;

        let score0xr = abs(d0xr);
        let score0yr = abs(d0yr);
        let score0zr = abs(d0zr);
        let dir0xr = max(score0yr, score0zr) <= score0xr;
        let dir0yr = (max(score0zr, score0xr) <= score0yr) & !dir0xr;
        let dir0zr = !(dir0xr | dir0yr);
        let v1xr = masked_add(v0xr, f32::from_bits(f32::to_bits(1.0) | (f32::to_bits(-1.0) & d0xr.to_bits())), dir0xr);
        let v1yr = masked_add(v0yr, f32::from_bits(f32::to_bits(1.0) | (f32::to_bits(-1.0) & d0yr.to_bits())), dir0yr);
        let v1zr = masked_add(v0zr, f32::from_bits(f32::to_bits(1.0) | (f32::to_bits(-1.0) & d0zr.to_bits())), dir0zr);
        let d1xr = xr - v1xr;
        let d1yr = yr - v1yr;
        let d1zr = zr - v1zr;

        let hv0xr = (v0xr as i32).wrapping_mul(primes::X);
        let hv0yr = (v0yr as i32).wrapping_mul(primes::Y);
        let hv0zr = (v0zr as i32).wrapping_mul(primes::Z);

        let hv1xr = (v1xr as i32).wrapping_mul(primes::X);
        let hv1yr = (v1yr as i32).wrapping_mul(primes::Y);
        let hv1zr = (v1zr as i32).wrapping_mul(primes::Z);

        let mut t0 = nmul_add(d0zr, d0zr, nmul_add(d0yr, d0yr, nmul_add(d0xr, d0xr, 0.6)));
        let mut t1 = nmul_add(d1zr, d1zr, nmul_add(d1yr, d1yr, nmul_add(d1xr, d1xr, 0.6)));
        t0 = max(t0, 0.0);
        t1 = max(t1, 0.0);
        t0 *= t0;
        t0 *= t0;
        t1 *= t1;
        t1 *= t1;

        let v0 = gradient_dot3(hash_primes3(seed, hv0xr, hv0yr, hv0zr), d0xr, d0yr, d0zr);
        let v1 = gradient_dot3(hash_primes3(seed, hv1xr, hv1yr, hv1zr), d1xr, d1yr, d1zr);

        val = mul_add(v0, t0, mul_add(v1, t1, val));

        if i == 1 {
            break;
        }

        xr += 0.5;
        yr += 0.5;
        zr += 0.5;
        seed = !seed;

        i = i.wrapping_add(1);
    }

    32.69428253173828125 * val
}
