use crate::floor;

use super::{gradient_dot2, gradient_dot3, gradient_dot4, hash_primes2, hash_primes3, hash_primes4, impl_noise234, interp_quintic, lerp, primes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Perlin;

impl_noise234!(Perlin);

#[inline]
fn gen2([x, y]: [f32; 2], seed: i32, _: Perlin) -> f32 {
    let xs = floor(x);
    let ys = floor(y);

    let x0 = (xs as i32).wrapping_mul(primes::X);
    let y0 = (ys as i32).wrapping_mul(primes::Y);

    let x1 = x0.wrapping_add(primes::X);
    let y1 = y0.wrapping_add(primes::Y);

    let xf0 = x - xs;
    let yf0 = y - ys;

    let xf1 = xf0 - 1.0;
    let yf1 = yf0 - 1.0;

    let xs = interp_quintic(xf0);
    let ys = interp_quintic(yf0);

    0.579106986522674560546875
        * lerp(
            lerp(gradient_dot2(hash_primes2(seed, x0, y0), xf0, yf0), gradient_dot2(hash_primes2(seed, x1, y0), xf1, yf0), xs),
            lerp(gradient_dot2(hash_primes2(seed, x0, y1), xf0, yf1), gradient_dot2(hash_primes2(seed, x1, y1), xf1, yf1), xs),
            ys,
        )
}

#[inline]
fn gen3([x, y, z]: [f32; 3], seed: i32, _: Perlin) -> f32 {
    let xs = floor(x);
    let ys = floor(y);
    let zs = floor(z);

    let x0 = (xs as i32).wrapping_mul(primes::X);
    let y0 = (ys as i32).wrapping_mul(primes::Y);
    let z0 = (zs as i32).wrapping_mul(primes::Z);

    let x1 = x0.wrapping_add(primes::X);
    let y1 = y0.wrapping_add(primes::Y);
    let z1 = z0.wrapping_add(primes::Z);

    let xf0 = x - xs;
    let yf0 = y - ys;
    let zf0 = z - zs;

    let xf1 = xf0 - 1.0;
    let yf1 = yf0 - 1.0;
    let zf1 = zf0 - 1.0;

    let xs = interp_quintic(xf0);
    let ys = interp_quintic(yf0);
    let zs = interp_quintic(zf0);

    0.964921414852142333984375
        * lerp(
            lerp(
                lerp(gradient_dot3(hash_primes3(seed, x0, y0, z0), xf0, yf0, zf0), gradient_dot3(hash_primes3(seed, x1, y0, z0), xf1, yf0, zf0), xs),
                lerp(gradient_dot3(hash_primes3(seed, x0, y1, z0), xf0, yf1, zf0), gradient_dot3(hash_primes3(seed, x1, y1, z0), xf1, yf1, zf0), xs),
                ys,
            ),
            lerp(
                lerp(gradient_dot3(hash_primes3(seed, x0, y0, z1), xf0, yf0, zf1), gradient_dot3(hash_primes3(seed, x1, y0, z1), xf1, yf0, zf1), xs),
                lerp(gradient_dot3(hash_primes3(seed, x0, y1, z1), xf0, yf1, zf1), gradient_dot3(hash_primes3(seed, x1, y1, z1), xf1, yf1, zf1), xs),
                ys,
            ),
            zs,
        )
}

#[inline]
fn gen4([x, y, z, w]: [f32; 4], seed: i32, _: Perlin) -> f32 {
    let xs = floor(x);
    let ys = floor(y);
    let zs = floor(z);
    let ws = floor(w);

    let x0 = (xs as i32).wrapping_mul(primes::X);
    let y0 = (ys as i32).wrapping_mul(primes::Y);
    let z0 = (zs as i32).wrapping_mul(primes::Z);
    let w0 = (ws as i32).wrapping_mul(primes::W);
    let x1 = x0.wrapping_add(primes::X);
    let y1 = y0.wrapping_add(primes::Y);
    let z1 = z0.wrapping_add(primes::Z);
    let w1 = w0.wrapping_add(primes::W);

    let xf0 = x - xs;
    let yf0 = y - ys;
    let zf0 = z - zs;
    let wf0 = w - ws;
    let xf1 = xf0 - 1.0;
    let yf1 = yf0 - 1.0;
    let zf1 = zf0 - 1.0;
    let wf1 = wf0 - 1.0;

    let xs = interp_quintic(xf0);
    let ys = interp_quintic(yf0);
    let zs = interp_quintic(zf0);
    let ws = interp_quintic(wf0);

    0.964921414852142333984375f32
        * lerp(
            lerp(
                lerp(
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y0, z0, w0), xf0, yf0, zf0, wf0),
                        gradient_dot4(hash_primes4(seed, x1, y0, z0, w0), xf1, yf0, zf0, wf0),
                        xs,
                    ),
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y1, z0, w0), xf0, yf1, zf0, wf0),
                        gradient_dot4(hash_primes4(seed, x1, y1, z0, w0), xf1, yf1, zf0, wf0),
                        xs,
                    ),
                    ys,
                ),
                lerp(
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y0, z1, w0), xf0, yf0, zf1, wf0),
                        gradient_dot4(hash_primes4(seed, x1, y0, z1, w0), xf1, yf0, zf1, wf0),
                        xs,
                    ),
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y1, z1, w0), xf0, yf1, zf1, wf0),
                        gradient_dot4(hash_primes4(seed, x1, y1, z1, w0), xf1, yf1, zf1, wf0),
                        xs,
                    ),
                    ys,
                ),
                zs,
            ),
            lerp(
                lerp(
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y0, z0, w1), xf0, yf0, zf0, wf1),
                        gradient_dot4(hash_primes4(seed, x1, y0, z0, w1), xf1, yf0, zf0, wf1),
                        xs,
                    ),
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y1, z0, w1), xf0, yf1, zf0, wf1),
                        gradient_dot4(hash_primes4(seed, x1, y1, z0, w1), xf1, yf1, zf0, wf1),
                        xs,
                    ),
                    ys,
                ),
                lerp(
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y0, z1, w1), xf0, yf0, zf1, wf1),
                        gradient_dot4(hash_primes4(seed, x1, y0, z1, w1), xf1, yf0, zf1, wf1),
                        xs,
                    ),
                    lerp(
                        gradient_dot4(hash_primes4(seed, x0, y1, z1, w1), xf0, yf1, zf1, wf1),
                        gradient_dot4(hash_primes4(seed, x1, y1, z1, w1), xf1, yf1, zf1, wf1),
                        xs,
                    ),
                    ys,
                ),
                zs,
            ),
            ws,
        )
}
