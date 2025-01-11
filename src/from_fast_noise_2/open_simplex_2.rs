use crate::fast_max;

use super::{gradient_dot2, gradient_dot3, gradient_dot4, hash_primes2, hash_primes3, hash_primes4, noise, primes};

noise!(OpenSimplex2);

#[inline]
fn gen2([x, y]: [f32; 2], seed: i32) -> f32 {
    const SQRT3: f32 = 1.7320508075688772935274463415059;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    const G2: f32 = (3.0 - SQRT3) / 6.0;

    let f = F2 * (x + y);
    let x0 = (x + f).floor();
    let y0 = (y + f).floor();

    let i = (x0 as i32).wrapping_mul(primes::X);
    let j = (y0 as i32).wrapping_mul(primes::Y);

    let g = G2 * (x0 + y0);
    let x0 = x - (x0 - g);
    let y0 = y - (y0 - g);

    let i1 = x0 > y0;

    let x1 = if i1 { x0 - 1.0 } else { x0 } + G2;
    let y1 = if i1 { y0 } else { y0 - 1.0 } + G2;

    let x2 = x0 + (G2 * 2.0 - 1.0);
    let y2 = y0 + (G2 * 2.0 - 1.0);

    let t0 = -(x0 * x0) + (-(y0 * y0) + 0.5);
    let t1 = -(x1 * x1) + (-(y1 * y1) + 0.5);
    let t2 = -(x2 * x2) + (-(y2 * y2) + 0.5);

    let mut t0 = fast_max(t0, 0.0);
    let mut t1 = fast_max(t1, 0.0);
    let mut t2 = fast_max(t2, 0.0);

    t0 *= t0;
    t0 *= t0;

    t1 *= t1;
    t1 *= t1;

    t2 *= t2;
    t2 *= t2;

    let n0 = gradient_dot2(hash_primes2(seed, i, j), x0, y0);
    let n1 = gradient_dot2(hash_primes2(seed, if i1 { i.wrapping_add(primes::X) } else { i }, if i1 { j } else { j.wrapping_add(primes::Y) }), x1, y1);
    let n2 = gradient_dot2(hash_primes2(seed, i.wrapping_add(primes::X), j.wrapping_add(primes::Y)), x2, y2);

    38.283687591552734375 * (n0 * t0 + (n1 * t1 + (n2 * t2)))
}

#[inline]
fn gen3([x, y, z]: [f32; 3], seed: i32) -> f32 {
    const F3: f32 = 1.0 / 3.0;
    const G3: f32 = 1.0 / 2.0;

    let s = F3 * (x + y + z);
    let x = x + s;
    let y = y + s;
    let z = z + s;

    let x0 = x.floor();
    let y0 = y.floor();
    let z0 = z.floor();
    let xi = x - x0;
    let yi = y - y0;
    let zi = z - z0;

    let i = (x0 as i32).wrapping_mul(primes::X);
    let j = (y0 as i32).wrapping_mul(primes::X);
    let k = (z0 as i32).wrapping_mul(primes::X);

    let x_ge_y = xi >= yi;
    let y_ge_z = yi >= zi;
    let x_ge_z = xi >= zi;

    let g = G3 * (xi + yi + zi);
    let x0 = xi - g;
    let y0 = yi - g;
    let z0 = zi - g;

    let i1 = x_ge_y & x_ge_z;
    let j1 = y_ge_z & !x_ge_y;
    let k1 = !x_ge_z & !y_ge_z;

    let i2 = x_ge_y | x_ge_z;
    let j2 = !x_ge_y | y_ge_z;
    let k2 = x_ge_z & y_ge_z;

    let x1 = if i1 { x0 - 1.0 } else { x0 } + G3;
    let y1 = if j1 { y0 - 1.0 } else { y0 } + G3;
    let z1 = if k1 { z0 - 1.0 } else { z0 } + G3;
    let x2 = if i2 { x0 - 1.0 } else { x0 } + G3 * 2.0;
    let y2 = if j2 { y0 - 1.0 } else { y0 } + G3 * 2.0;
    let z2 = if k2 { z0 } else { z0 - 1.0 } + G3 * 2.0;
    let x3 = x0 + (G3 * 3.0 - 1.0);
    let y3 = y0 + (G3 * 3.0 - 1.0);
    let z3 = z0 + (G3 * 3.0 - 1.0);

    let t0 = -(x0 * x0) + (-(y0 * y0) + (-(z0 * z0) + 0.6));
    let t1 = -(x1 * x1) + (-(y1 * y1) + (-(z1 * z1) + 0.6));
    let t2 = -(x2 * x2) + (-(y2 * y2) + (-(z2 * z2) + 0.6));
    let t3 = -(x3 * x3) + (-(y3 * y3) + (-(z3 * z3) + 0.6));

    let mut t0 = fast_max(t0, 0.0);
    let mut t1 = fast_max(t1, 0.0);
    let mut t2 = fast_max(t2, 0.0);
    let mut t3 = fast_max(t3, 0.0);

    t0 *= t0;
    t0 *= t0;

    t1 *= t1;
    t1 *= t1;

    t2 *= t2;
    t2 *= t2;

    t3 *= t3;
    t3 *= t3;

    let n0 = gradient_dot3(hash_primes3(seed, i, j, k), x0, y0, z0);
    let n1 = gradient_dot3(
        hash_primes3(
            seed,
            if i1 { i.wrapping_add(primes::X) } else { i },
            if j1 { j.wrapping_add(primes::Y) } else { j },
            if k1 { k.wrapping_add(primes::Z) } else { k },
        ),
        x1,
        y1,
        z1,
    );
    let n2 = gradient_dot3(
        hash_primes3(
            seed,
            if i2 { i.wrapping_add(primes::X) } else { i },
            if j2 { j.wrapping_add(primes::Y) } else { j },
            if k2 { k.wrapping_add(primes::Z) } else { k },
        ),
        x2,
        y2,
        z2,
    );
    let n3 = gradient_dot3(hash_primes3(seed, i.wrapping_add(primes::X), j.wrapping_add(primes::Y), k.wrapping_add(primes::Z)), x3, y3, z3);

    32.69428253173828125 * (n0 * t0 + (n1 * t1 + (n2 * t2 + n3 * t3)))
}

#[inline]
fn gen4([x, y, z, w]: [f32; 4], seed: i32) -> f32 {
    const SQRT5: f32 = 2.236067977499;
    const F4: f32 = (SQRT5 - 1.0) / 4.0;
    const G4: f32 = (5.0 - SQRT5) / 20.0;

    let s = F4 * (x + y + z + w);
    let x = x + s;
    let y = y + s;
    let z = z + s;
    let w = w + s;

    let x0 = x.floor();
    let y0 = y.floor();
    let z0 = z.floor();
    let w0 = w.floor();
    let xi = x - x0;
    let yi = y - y0;
    let zi = z - z0;
    let wi = w - w0;

    let i = (x0 as i32).wrapping_mul(primes::X);
    let j = (y0 as i32).wrapping_mul(primes::Y);
    let k = (z0 as i32).wrapping_mul(primes::Z);
    let l = (w0 as i32).wrapping_mul(primes::W);

    let g = G4 * (xi + yi + zi + wi);
    let x0 = xi - g;
    let y0 = yi - g;
    let z0 = zi - g;
    let w0 = wi - g;

    let rank_x = 0u32;
    let rank_y = 0u32;
    let rank_z = 0u32;
    let rank_w = 0u32;

    let x_ge_y = x0 >= y0;
    let rank_x = if x_ge_y { rank_x.wrapping_add(1) } else { rank_x };
    let rank_y = if x_ge_y { rank_y } else { rank_y.wrapping_add(1) };

    let x_ge_z = x0 >= z0;
    let rank_x = if x_ge_z { rank_x.wrapping_add(1) } else { rank_x };
    let rank_z = if x_ge_z { rank_z } else { rank_z.wrapping_add(1) };

    let x_ge_w = x0 >= w0;
    let rank_x = if x_ge_w { rank_x.wrapping_add(1) } else { rank_x };
    let rank_w = if x_ge_w { rank_w } else { rank_w.wrapping_add(1) };

    let y_ge_z = y0 >= z0;
    let rank_y = if y_ge_z { rank_y.wrapping_add(1) } else { rank_y };
    let rank_z = if y_ge_z { rank_z } else { rank_z.wrapping_add(1) };

    let y_ge_w = y0 >= w0;
    let rank_y = if y_ge_w { rank_y.wrapping_add(1) } else { rank_y };
    let rank_w = if y_ge_w { rank_w } else { rank_w.wrapping_add(1) };

    let z_ge_w = z0 >= w0;
    let rank_z = if z_ge_w { rank_z.wrapping_add(1) } else { rank_z };
    let rank_w = if z_ge_w { rank_w } else { rank_w.wrapping_add(1) };

    let i1 = rank_x > 2;
    let j1 = rank_y > 2;
    let k1 = rank_z > 2;
    let l1 = rank_w > 2;

    let i2 = rank_x > 1;
    let j2 = rank_y > 1;
    let k2 = rank_z > 1;
    let l2 = rank_w > 1;

    let i3 = rank_x > 0;
    let j3 = rank_y > 0;
    let k3 = rank_z > 0;
    let l3 = rank_w > 0;

    let x1 = if i1 { x0 - 1.0 } else { x0 } + (G4);
    let y1 = if j1 { y0 - 1.0 } else { y0 } + (G4);
    let z1 = if k1 { z0 - 1.0 } else { z0 } + (G4);
    let w1 = if l1 { w0 - 1.0 } else { w0 } + (G4);
    let x2 = if i2 { x0 - 1.0 } else { x0 } + (G4 * 2.0);
    let y2 = if j2 { y0 - 1.0 } else { y0 } + (G4 * 2.0);
    let z2 = if k2 { z0 - 1.0 } else { z0 } + (G4 * 2.0);
    let w2 = if l2 { w0 - 1.0 } else { w0 } + (G4 * 2.0);
    let x3 = if i3 { x0 - 1.0 } else { x0 } + (G4 * 3.0);
    let y3 = if j3 { y0 - 1.0 } else { y0 } + (G4 * 3.0);
    let z3 = if k3 { z0 - 1.0 } else { z0 } + (G4 * 3.0);
    let w3 = if l3 { w0 - 1.0 } else { w0 } + (G4 * 3.0);
    let x4 = x0 + (G4 * 4.0 - 1.0);
    let y4 = y0 + (G4 * 4.0 - 1.0);
    let z4 = z0 + (G4 * 4.0 - 1.0);
    let w4 = w0 + (G4 * 4.0 - 1.0);

    let mut t0 = -(x0 * x0) + (-(y0 * y0) + (-(z0 * z0) + (-(w0 * w0) + 0.6)));
    let mut t1 = -(x1 * x1) + (-(y1 * y1) + (-(z1 * z1) + (-(w1 * w1) + 0.6)));
    let mut t2 = -(x2 * x2) + (-(y2 * y2) + (-(z2 * z2) + (-(w2 * w2) + 0.6)));
    let mut t3 = -(x3 * x3) + (-(y3 * y3) + (-(z3 * z3) + (-(w3 * w3) + 0.6)));
    let mut t4 = -(x4 * x4) + (-(y4 * y4) + (-(z4 * z4) + (-(w4 * w4) + 0.6)));

    t0 = fast_max(t0, 0.0);
    t1 = fast_max(t1, 0.0);
    t2 = fast_max(t2, 0.0);
    t3 = fast_max(t3, 0.0);
    t4 = fast_max(t4, 0.0);

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
        hash_primes4(
            seed,
            if i1 { i.wrapping_add(primes::X) } else { i },
            if j1 { j.wrapping_add(primes::Y) } else { j },
            if k1 { k.wrapping_add(primes::Z) } else { k },
            if l1 { l.wrapping_add(primes::W) } else { l },
        ),
        x1,
        y1,
        z1,
        w1,
    );
    let n2 = gradient_dot4(
        hash_primes4(
            seed,
            if i2 { i.wrapping_add(primes::X) } else { i },
            if j2 { j.wrapping_add(primes::Y) } else { j },
            if k2 { k.wrapping_add(primes::Z) } else { k },
            if l2 { l.wrapping_add(primes::W) } else { l },
        ),
        x2,
        y2,
        z2,
        w2,
    );
    let n3 = gradient_dot4(
        hash_primes4(
            seed,
            if i3 { i.wrapping_add(primes::X) } else { i },
            if j3 { j.wrapping_add(primes::Y) } else { j },
            if k3 { k.wrapping_add(primes::Z) } else { k },
            if l3 { l.wrapping_add(primes::W) } else { l },
        ),
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

    27.0 * n0.mul_add(t0, n1.mul_add(t1, n2.mul_add(t2, n3.mul_add(t3, n4 * t4))))
}
