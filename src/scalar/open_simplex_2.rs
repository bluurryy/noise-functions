use crate::private_prelude::*;

#[inline]
pub fn gen2(pos: [f32; 2], seed: i32) -> f32 {
    gen2_not_improved(improve2(pos), seed)
}

#[inline]
pub fn gen2_not_improved([x, y]: [f32; 2], seed: i32) -> f32 {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const G2: f32 = (3.0 - SQRT3) / 6.0;

    let mut i: i32 = floor_to_int(x);
    let mut j: i32 = floor_to_int(y);

    let xi: f32 = x - i as f32;
    let yi: f32 = y - j as f32;

    let t: f32 = (xi + yi) * G2;
    let x0: f32 = xi - t;
    let y0: f32 = yi - t;

    i = i.wrapping_mul(PRIME_X);
    j = j.wrapping_mul(PRIME_Y);

    let n0: f32;
    let n1: f32;
    let n2: f32;

    let a: f32 = 0.5 - x0 * x0 - y0 * y0;
    if a <= 0.0 {
        n0 = 0.0;
    } else {
        n0 = (a * a) * (a * a) * grad2(seed, i, j, x0, y0);
    }

    let c: f32 = (2.0 * (1.0 - 2.0 * G2) * (1.0 / G2 - 2.0)) * t + ((-2.0 * (1.0 - 2.0 * G2) * (1.0 - 2.0 * G2)) + a);
    if c <= 0.0 {
        n2 = 0.0;
    } else {
        let x2: f32 = x0 + (2.0 * G2 - 1.0);
        let y2: f32 = y0 + (2.0 * G2 - 1.0);
        n2 = (c * c) * (c * c) * grad2(seed, i.wrapping_add(PRIME_X), j.wrapping_add(PRIME_Y), x2, y2);
    }

    if y0 > x0 {
        let x1: f32 = x0 + G2;
        let y1: f32 = y0 + (G2 - 1.0);
        let b: f32 = 0.5 - x1 * x1 - y1 * y1;
        if b <= 0.0 {
            n1 = 0.0;
        } else {
            n1 = (b * b) * (b * b) * grad2(seed, i, j.wrapping_add(PRIME_Y), x1, y1);
        }
    } else {
        let x1: f32 = x0 + (G2 - 1.0);
        let y1: f32 = y0 + G2;
        let b: f32 = 0.5 - x1 * x1 - y1 * y1;
        if b <= 0.0 {
            n1 = 0.0;
        } else {
            n1 = (b * b) * (b * b) * grad2(seed, i.wrapping_add(PRIME_X), j, x1, y1);
        }
    }

    (n0 + n1 + n2) * 99.83685446303647
}

#[inline]
pub fn improve2([mut x, mut y]: [f32; 2]) -> [f32; 2] {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    let t: f32 = (x + y) * F2;
    x += t;
    y += t;
    [x, y]
}

#[inline]
pub fn gen3([x, y, z]: [f32; 3], mut seed: i32) -> f32 {
    let mut i: i32 = round_to_int(x);
    let mut j: i32 = round_to_int(y);
    let mut k: i32 = round_to_int(z);
    let mut x0: f32 = x - i as f32;
    let mut y0: f32 = y - j as f32;
    let mut z0: f32 = z - k as f32;

    let mut x_nsign: i32 = (-1.0 - x0) as i32 | 1;
    let mut y_nsign: i32 = (-1.0 - y0) as i32 | 1;
    let mut z_nsign: i32 = (-1.0 - z0) as i32 | 1;

    let mut ax0: f32 = x_nsign as f32 * -x0;
    let mut ay0: f32 = y_nsign as f32 * -y0;
    let mut az0: f32 = z_nsign as f32 * -z0;

    i = i.wrapping_mul(PRIME_X);
    j = j.wrapping_mul(PRIME_Y);
    k = k.wrapping_mul(PRIME_Z);

    let mut value: f32 = 0.0;
    let mut a: f32 = (0.6 - x0 * x0) - (y0 * y0 + z0 * z0);

    for l in 0..2 {
        if a > 0.0 {
            value += (a * a) * (a * a) * grad3(seed, i, j, k, x0, y0, z0);
        }

        let mut b: f32 = a + 1.0;
        let mut i1 = i;
        let mut j1 = j;
        let mut k1 = k;
        let mut x1: f32 = x0;
        let mut y1: f32 = y0;
        let mut z1: f32 = z0;

        if ax0 >= ay0 && ax0 >= az0 {
            x1 += x_nsign as f32;
            b -= x_nsign as f32 * 2.0 * x1;
            i1 = i1.wrapping_sub(x_nsign.wrapping_mul(PRIME_X));
        } else if ay0 > ax0 && ay0 >= az0 {
            y1 += y_nsign as f32;
            b -= y_nsign as f32 * 2.0 * y1;
            j1 = j1.wrapping_sub(y_nsign.wrapping_mul(PRIME_Y));
        } else {
            z1 += z_nsign as f32;
            b -= z_nsign as f32 * 2.0 * z1;
            k1 = k1.wrapping_sub(z_nsign.wrapping_mul(PRIME_Z));
        }

        if b > 0.0 {
            value += (b * b) * (b * b) * grad3(seed, i1, j1, k1, x1, y1, z1);
        }

        if l == 1 {
            break;
        }

        ax0 = 0.5 - ax0;
        ay0 = 0.5 - ay0;
        az0 = 0.5 - az0;

        x0 = x_nsign as f32 * ax0;
        y0 = y_nsign as f32 * ay0;
        z0 = z_nsign as f32 * az0;

        a += (0.75 - ax0) - (ay0 + az0);

        i = i.wrapping_add(x_nsign.wrapping_shr(1) & PRIME_X);
        j = j.wrapping_add(y_nsign.wrapping_shr(1) & PRIME_Y);
        k = k.wrapping_add(z_nsign.wrapping_shr(1) & PRIME_Z);

        x_nsign = x_nsign.wrapping_neg();
        y_nsign = y_nsign.wrapping_neg();
        z_nsign = z_nsign.wrapping_neg();

        seed = !seed;
    }

    value * 32.69428253173828125
}

#[cfg(test)]
#[inline]
pub fn improve3([mut x, mut y, mut z]: [f32; 3]) -> [f32; 3] {
    const R3: f32 = 2.0 / 3.0;
    let r: f32 = (x + y + z) * R3; // Rotation, not skew
    x = r - x;
    y = r - y;
    z = r - z;
    [x, y, z]
}
