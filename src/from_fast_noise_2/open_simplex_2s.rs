use crate::{fast_max, floor, mul_add};

use super::{gradient_dot2_fancy, gradient_dot3, hash_primes2, hash_primes3, noise23, primes};

noise23!(OpenSimplex2s);

#[inline]
fn gen2([x, y]: [f32; 2], seed: i32) -> f32 {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    const G2: f32 = (SQRT3 - 3.0) / 6.0;

    let s = F2 * (x + y);
    let xs = x + s;
    let ys = y + s;
    let xsb = floor(xs);
    let ysb = floor(ys);
    let xsi = xs - xsb;
    let ysi = ys - ysb;
    let xsbp = (xsb as i32).wrapping_mul(primes::X);
    let ysbp = (ysb as i32).wrapping_mul(primes::Y);

    let forward_xy = xsi + ysi > 1.0;
    let boundary_xy = if forward_xy { -1.0 } else { 0.0 };
    let forward_x = mul_add(xsi, -2.0, ysi) < boundary_xy;
    let forward_y = mul_add(ysi, -2.0, xsi) < boundary_xy;

    let t = G2 * (xsi + ysi);
    let xi = xsi + t;
    let yi = ysi + t;

    let h0 = hash_primes2(seed, xsbp, ysbp);
    let v0 = gradient_dot2_fancy(h0, xi, yi);
    let a = -(xi * xi) + (-(yi * yi) + 2.0 / 3.0);
    let mut a0 = a;
    a0 *= a0;
    a0 *= a0;
    let value = a0 * v0;

    let h1 = hash_primes2(seed, xsbp.wrapping_add(primes::X), ysbp.wrapping_add(primes::Y));
    let v1 = gradient_dot2_fancy(h1, xi - (2.0 * G2 + 1.0), yi - (2.0 * G2 + 1.0));
    let mut a1 = mul_add(2.0f32 * (1.0 + 2.0 * G2) * (1.0 / G2 + 2.0), t, a + (-2.0 * (1.0 + 2.0 * G2) * (1.0 + 2.0 * G2)));
    a1 *= a1;
    a1 *= a1;
    let value = mul_add(a1, v1, value);

    let xy_delta = if forward_xy { G2 + 1.0 } else { -G2 };
    let xi = xi - xy_delta;
    let yi = yi - xy_delta;

    let h2 = hash_primes2(seed, nmasked_sub(masked_add(xsbp, primes::X * 2, forward_x), primes::X, forward_xy), masked_add(ysbp, primes::Y, forward_xy));
    let xi2 = xi - if forward_x { 1.0 + 2.0 * G2 } else { -1.0 };
    let yi2 = masked_sub(yi, 2.0 * G2, forward_x);
    let v2 = gradient_dot2_fancy(h2, xi2, yi2);
    let mut a2 = fast_max(-(xi2 * xi2) + (-(yi2 * yi2) + 2.0 / 3.0), 0.0);
    a2 *= a2;
    a2 *= a2;
    let value = mul_add(a2, v2, value);

    let h3 = hash_primes2(
        seed,
        masked_add(xsbp, primes::X, forward_xy),
        nmasked_sub(masked_add(ysbp, (primes::Y as i64).wrapping_mul(2) as i32, forward_y), primes::Y, forward_xy),
    );
    let xi3 = masked_sub(xi, 2.0 * G2, forward_y);
    let yi3 = yi - if forward_y { 1.0 + 2.0 * G2 } else { -1.0 };
    let v3 = gradient_dot2_fancy(h3, xi3, yi3);
    let mut a3 = fast_max(-(xi3 * xi3) + (-(yi3 * yi3) + 2.0 / 3.0), 0.0);
    a3 *= a3;
    a3 *= a3;
    let value = mul_add(a3, v3, value);

    9.28993664146183 * value
}

// TODO: this is broken
#[inline]
fn gen3([x, y, z]: [f32; 3], mut seed: i32) -> f32 {
    let f = (2.0 / 3.0) * (x + y + z);
    let xr = f - x;
    let yr = f - y;
    let zr = f - z;

    let xrb = floor(xr);
    let yrb = floor(yr);
    let zrb = floor(zr);
    let mut xri = xr - xrb;
    let mut yri = yr - yrb;
    let mut zri = zr - zrb;
    let mut xrbp = (xrb as i32).wrapping_mul(primes::X);
    let mut yrbp = (yrb as i32).wrapping_mul(primes::Y);
    let mut zrbp = (zrb as i32).wrapping_mul(primes::Z);

    let mut value = 0.0f32;
    let mut i = 0;

    loop {
        let mut a = (-(xri * xri) + (-(yri * yri) + (-(zri * zri) + 0.75))) * 0.5;

        let p0 = zri + yri + xri - 1.5;
        let flip0 = p0 >= 0.0;
        let mut a0 = fast_max(masked_add(a, p0, flip0), 0.0);
        a0 *= a0;
        a0 *= a0;
        let h0 = hash_primes3(seed, masked_add(xrbp, primes::X, flip0), masked_add(yrbp, primes::Y, flip0), masked_add(zrbp, primes::Z, flip0));
        let v0 = gradient_dot3(h0, masked_sub(xri, 1.0, flip0), masked_sub(yri, 1.0, flip0), masked_sub(zri, 1.0, flip0));
        value = mul_add(a0, v0, value);
        a -= 0.5;

        let p1 = zri + yri - xri - 0.5;
        let flip1 = p1 >= 0.0;
        let mut a1 = fast_max(masked_add(a + xri, p1, flip1), 0.0);
        a1 *= a1;
        a1 *= a1;
        let h1 = hash_primes3(seed, masked_add(xrbp, primes::X, flip1), masked_add(yrbp, primes::Y, flip1), masked_add(zrbp, primes::Z, flip1));
        let v1 = gradient_dot3(h1, nmasked_sub(xri, 1.0, flip1), masked_sub(yri, 1.0, flip1), masked_sub(zri, 1.0, flip1));
        value = mul_add(a1, v1, value);

        let p2 = xri - 0.5 + (zri - yri);
        let flip2 = p2 >= 0.0;
        let mut a2 = fast_max(masked_add(a + yri, p2, flip2), 0.0);
        a2 *= a2;
        a2 *= a2;
        let h2 = hash_primes3(seed, masked_add(xrbp, primes::X, flip2), masked_add(yrbp, primes::Y, flip2), masked_add(zrbp, primes::Z, flip2));
        let v2 = gradient_dot3(h2, masked_sub(xri, 1.0, flip2), nmasked_sub(yri, 1.0, flip2), masked_sub(zri, 1.0, flip2));
        value = mul_add(a2, v2, value);

        let p3 = xri - 0.5 - (zri - yri);
        let flip3 = p3 >= 0.0;
        let mut a3 = fast_max(masked_add(a + zri, p3, flip3), 0.0);
        a3 *= a3;
        a3 *= a3;
        let h3 = hash_primes3(seed, masked_add(xrbp, primes::X, flip3), masked_add(yrbp, primes::Y, flip3), masked_add(zrbp, primes::Z, flip3));
        let v3 = gradient_dot3(h3, masked_sub(xri, 1.0, flip3), masked_sub(yri, 1.0, flip3), nmasked_sub(zri, 1.0, flip3));
        value = mul_add(a3, v3, value);

        if i == 1 {
            break;
        }

        let side_x = xri >= 0.5;
        let side_y = yri >= 0.5;
        let side_z = zri >= 0.5;

        xrbp = masked_add(xrbp, primes::X, side_x);
        yrbp = masked_add(yrbp, primes::Y, side_y);
        zrbp = masked_add(zrbp, primes::Z, side_z);

        xri += if side_x { -0.5 } else { 0.5 };
        yri += if side_y { -0.5 } else { 0.5 };
        zri += if side_z { -0.5 } else { 0.5 };

        seed = !seed;
        i = i.wrapping_add(1);
    }

    144.736422163332608 * value
}

trait WrappingOps {
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
}

impl WrappingOps for i32 {
    fn wrapping_add(self, other: Self) -> Self {
        i32::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        i32::wrapping_sub(self, other)
    }
}

impl WrappingOps for f32 {
    fn wrapping_add(self, other: Self) -> Self {
        self + other
    }
    fn wrapping_sub(self, other: Self) -> Self {
        self - other
    }
}

fn masked_add<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a.wrapping_add(b)
    } else {
        a
    }
}

fn masked_sub<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a.wrapping_sub(b)
    } else {
        a
    }
}

fn nmasked_sub<T: WrappingOps>(a: T, b: T, m: bool) -> T {
    if m {
        a
    } else {
        a.wrapping_sub(b)
    }
}
