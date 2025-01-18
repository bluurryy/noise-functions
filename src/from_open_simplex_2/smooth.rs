//! K.jpg's OpenSimplex 2, smooth variant ("SuperSimplex")

use super::smooth_luts::{GRADIENTS_2D, GRADIENTS_3D, GRADIENTS_4D, LOOKUP_4D_A, LOOKUP_4D_B};

use core::num::Wrapping;

const PRIME_X: i64 = 0x5205402B9270C86F;
const PRIME_Y: i64 = 0x598CD327003817B5;
const PRIME_Z: i64 = 0x5BCC226E9FA0BACB;
const PRIME_W: i64 = 0x56CC5227E58F554B;
const HASH_MULTIPLIER: i64 = 0x53A3F72DEEC546F5;
const SEED_FLIP_3D: i64 = -0x52D547B2E96ED629;

const ROOT2OVER2: f32 = 0.7071067811865476;
const SKEW_2D: f32 = 0.366025403784439;
const UNSKEW_2D: f32 = -0.21132486540518713;

const ROOT3OVER3: f32 = 0.577350269189626;
const FALLBACK_ROTATE3: f32 = 2.0 / 3.0;
const ROTATE3_ORTHOGONALIZER: f32 = UNSKEW_2D;

const SKEW_4D: f32 = 0.309016994374947;
const UNSKEW_4D: f32 = -0.138196601125011;

const N_GRADS_2D_EXPONENT: i32 = 7;
const N_GRADS_3D_EXPONENT: i32 = 8;
const N_GRADS_4D_EXPONENT: i32 = 9;
const N_GRADS_2D: i32 = 1 << N_GRADS_2D_EXPONENT;
const N_GRADS_3D: i32 = 1 << N_GRADS_3D_EXPONENT;
const N_GRADS_4D: i32 = 1 << N_GRADS_4D_EXPONENT;

const RSQUARED_2D: f32 = 2.0 / 3.0;
const RSQUARED_3D: f32 = 3.0 / 4.0;
const RSQUARED_4D: f32 = 4.0 / 5.0;

/// 2D OpenSimplex2S/SuperSimplex noise, standard lattice orientation.
pub fn noise2([x, y]: [f32; 2], seed: i64) -> f32 {
    // Get points for A2* lattice
    let s = SKEW_2D * (x + y);
    let xs = x + s;
    let ys = y + s;

    noise2_UnskewedBase([xs, ys], seed)
}

/// 2D OpenSimplex2S/SuperSimplex noise, with Y pointing down the main diagonal.
/// Might be better for a 2D sandbox style game, where Y is vertical.
/// Probably slightly less optimal for heightmaps or continent maps,
/// unless your map is centered around an equator. It's a slight
/// difference, but the option is here to make it easy.
pub fn noise2_ImproveX([x, y]: [f32; 2], seed: i64) -> f32 {
    // Skew transform and rotation baked into one.
    let xx = x * ROOT2OVER2;
    let yy = y * (ROOT2OVER2 * (1.0 + 2.0 * SKEW_2D));

    noise2_UnskewedBase([yy + xx, yy - xx], seed)
}

/// 2D OpenSimplex2S/SuperSimplex noise base.
fn noise2_UnskewedBase([xs, ys]: [f32; 2], seed: i64) -> f32 {
    let seed = Wrapping(seed);

    // Get base points and offsets.
    let xsb = fastFloor(xs);
    let ysb = fastFloor(ys);
    let xi = xs - xsb as f32;
    let yi = ys - ysb as f32;

    // Prime pre-multiplication for hash.
    let xsbp = Wrapping(xsb as i64) * Wrapping(PRIME_X);
    let ysbp = Wrapping(ysb as i64) * Wrapping(PRIME_Y);

    // Unskew.
    let t = (xi + yi) * UNSKEW_2D;
    let dx0 = xi + t;
    let dy0 = yi + t;

    // First vertex.
    let a0 = RSQUARED_2D - dx0 * dx0 - dy0 * dy0;
    let mut value = (a0 * a0) * (a0 * a0) * grad2(seed, xsbp, ysbp, dx0, dy0);

    // Second vertex.
    let a1 = (2.0 * (1.0 + 2.0 * UNSKEW_2D) * (1.0 / UNSKEW_2D + 2.0)) * t + ((-2.0 * (1.0 + 2.0 * UNSKEW_2D) * (1.0 + 2.0 * UNSKEW_2D)) + a0);
    let dx1 = dx0 - (1.0 + 2.0 * UNSKEW_2D);
    let dy1 = dy0 - (1.0 + 2.0 * UNSKEW_2D);
    value += (a1 * a1) * (a1 * a1) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp + Wrapping(PRIME_Y), dx1, dy1);

    // Third and fourth vertices.
    // Nested conditionals were faster than compact bit logic/arithmetic.
    let xmyi = xi - yi;
    if t < UNSKEW_2D {
        if xi + xmyi > 1.0 {
            let dx2 = dx0 - (3.0 * UNSKEW_2D + 2.0);
            let dy2 = dy0 - (3.0 * UNSKEW_2D + 1.0);
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp + Wrapping(PRIME_X << 1), ysbp + Wrapping(PRIME_Y), dx2, dy2);
            }
        } else {
            let dx2 = dx0 - UNSKEW_2D;
            let dy2 = dy0 - (UNSKEW_2D + 1.0);
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp + Wrapping(PRIME_Y), dx2, dy2);
            }
        }

        if yi - xmyi > 1.0 {
            let dx3 = dx0 - (3.0 * UNSKEW_2D + 1.0);
            let dy3 = dy0 - (3.0 * UNSKEW_2D + 2.0);
            let a3 = RSQUARED_2D - dx3 * dx3 - dy3 * dy3;
            if a3 > 0.0 {
                value += (a3 * a3) * (a3 * a3) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp + Wrapping(PRIME_Y << 1), dx3, dy3);
            }
        } else {
            let dx3 = dx0 - (UNSKEW_2D + 1.0);
            let dy3 = dy0 - UNSKEW_2D;
            let a3 = RSQUARED_2D - dx3 * dx3 - dy3 * dy3;
            if a3 > 0.0 {
                value += (a3 * a3) * (a3 * a3) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp, dx3, dy3);
            }
        }
    } else {
        if xi + xmyi < 0.0 {
            let dx2 = dx0 + (1.0 + UNSKEW_2D);
            let dy2 = dy0 + UNSKEW_2D;
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp - Wrapping(PRIME_X), ysbp, dx2, dy2);
            }
        } else {
            let dx2 = dx0 - (UNSKEW_2D + 1.0);
            let dy2 = dy0 - UNSKEW_2D;
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp + Wrapping(PRIME_X), ysbp, dx2, dy2);
            }
        }

        if yi < xmyi {
            let dx2 = dx0 + UNSKEW_2D;
            let dy2 = dy0 + (UNSKEW_2D + 1.0);
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp - Wrapping(PRIME_Y), dx2, dy2);
            }
        } else {
            let dx2 = dx0 - UNSKEW_2D;
            let dy2 = dy0 - (UNSKEW_2D + 1.0);
            let a2 = RSQUARED_2D - dx2 * dx2 - dy2 * dy2;
            if a2 > 0.0 {
                value += (a2 * a2) * (a2 * a2) * grad2(seed, xsbp, ysbp + Wrapping(PRIME_Y), dx2, dy2);
            }
        }
    }

    value
}

/// 3D OpenSimplex2S/SuperSimplex noise, with better visual isotropy in (X, Y).
/// Recommended for 3D terrain and time-varied animations.
/// The Z coordinate should always be the "different" coordinate in whatever your use case is.
/// If Y is vertical in world coordinates, call noise3_ImproveXZ(x, z, Y) or use noise3_XZBeforeY.
/// If Z is vertical in world coordinates, call noise3_ImproveXZ(x, y, Z).
/// For a time varied animation, call noise3_ImproveXY(x, y, T).
pub fn noise3_ImproveXY([x, y, z]: [f32; 3], seed: i64) -> f32 {
    // Re-orient the cubic lattices without skewing, so Z points up the main lattice diagonal,
    // and the planes formed by XY are moved far out of alignment with the cube faces.
    // Orthonormal rotation. Not a skew transform.
    let xy = x + y;
    let s2 = xy * ROTATE3_ORTHOGONALIZER;
    let zz = z * ROOT3OVER3;
    let xr = x + s2 + zz;
    let yr = y + s2 + zz;
    let zr = xy * -ROOT3OVER3 + zz;

    // Evaluate both lattices to form a BCC lattice.
    noise3_UnrotatedBase([xr, yr, zr], seed)
}

/// 3D OpenSimplex2S/SuperSimplex noise, with better visual isotropy in (X, Z).
/// Recommended for 3D terrain and time-varied animations.
/// The Y coordinate should always be the "different" coordinate in whatever your use case is.
/// If Y is vertical in world coordinates, call noise3_ImproveXZ(x, Y, z).
/// If Z is vertical in world coordinates, call noise3_ImproveXZ(x, Z, y) or use noise3_ImproveXY.
/// For a time varied animation, call noise3_ImproveXZ(x, T, y) or use noise3_ImproveXY.
pub fn noise3_ImproveXZ([x, y, z]: [f32; 3], seed: i64) -> f32 {
    // Re-orient the cubic lattices without skewing, so Y points up the main lattice diagonal,
    // and the planes formed by XZ are moved far out of alignment with the cube faces.
    // Orthonormal rotation. Not a skew transform.
    let xz = x + z;
    let s2 = xz * -0.211324865405187;
    let yy = y * ROOT3OVER3;
    let xr = x + s2 + yy;
    let zr = z + s2 + yy;
    let yr = xz * -ROOT3OVER3 + yy;

    // Evaluate both lattices to form a BCC lattice.
    noise3_UnrotatedBase([xr, yr, zr], seed)
}

/// 3D OpenSimplex2S/SuperSimplex noise, fallback rotation option
/// Use noise3_ImproveXY or noise3_ImproveXZ instead, wherever appropriate.
/// They have less diagonal bias. This function's best use is as a fallback.
pub fn noise3_Fallback([x, y, z]: [f32; 3], seed: i64) -> f32 {
    // Re-orient the cubic lattices via rotation, to produce a familiar look.
    // Orthonormal rotation. Not a skew transform.
    let r = FALLBACK_ROTATE3 * (x + y + z);
    let xr = r - x;
    let yr = r - y;
    let zr = r - z;

    // Evaluate both lattices to form a BCC lattice.
    noise3_UnrotatedBase([xr, yr, zr], seed)
}

/// Generate overlapping cubic lattices for 3D Re-oriented BCC noise.
/// Lookup table implementation inspired by DigitalShadow.
/// It was actually faster to narrow down the points in the loop itself,
/// than to build up the index with enough info to isolate 8 points.
pub fn noise3_UnrotatedBase([xr, yr, zr]: [f32; 3], seed: i64) -> f32 {
    let seed = Wrapping(seed);

    // Get base points and offsets.
    let xrb = fastFloor(xr);
    let yrb = fastFloor(yr);
    let zrb = fastFloor(zr);
    let xi = (xr - xrb as f32) as f32;
    let yi = (yr - yrb as f32) as f32;
    let zi = (zr - zrb as f32) as f32;

    // Prime pre-multiplication for hash. Also flip seed for second lattice copy.
    let xrbp = Wrapping(xrb as i64) * Wrapping(PRIME_X);
    let yrbp = Wrapping(yrb as i64) * Wrapping(PRIME_Y);
    let zrbp = Wrapping(zrb as i64) * Wrapping(PRIME_Z);
    let seed2 = seed ^ Wrapping(SEED_FLIP_3D);

    // -1 if positive, 0 if negative.
    let xNMask = (-0.5 - xi) as i32;
    let yNMask = (-0.5 - yi) as i32;
    let zNMask = (-0.5 - zi) as i32;

    // First vertex.
    let x0 = xi + xNMask as f32;
    let y0 = yi + yNMask as f32;
    let z0 = zi + zNMask as f32;
    let a0 = RSQUARED_3D - x0 * x0 - y0 * y0 - z0 * z0;
    let mut value = (a0 * a0)
        * (a0 * a0)
        * grad3(
            seed,
            xrbp + (Wrapping(xNMask as i64) & Wrapping(PRIME_X)),
            yrbp + (Wrapping(yNMask as i64) & Wrapping(PRIME_Y)),
            zrbp + (Wrapping(zNMask as i64) & Wrapping(PRIME_Z)),
            x0,
            y0,
            z0,
        );

    // Second vertex.
    let x1 = xi - 0.5;
    let y1 = yi - 0.5;
    let z1 = zi - 0.5;
    let a1 = RSQUARED_3D - x1 * x1 - y1 * y1 - z1 * z1;
    value += (a1 * a1) * (a1 * a1) * grad3(seed2, xrbp + Wrapping(PRIME_X), yrbp + Wrapping(PRIME_Y), zrbp + Wrapping(PRIME_Z), x1, y1, z1);

    // Shortcuts for building the remaining falloffs.
    // Derived by subtracting the polynomials with the offsets plugged in.
    let xAFlipMask0 = ((xNMask | 1) << 1) as f32 * x1;
    let yAFlipMask0 = ((yNMask | 1) << 1) as f32 * y1;
    let zAFlipMask0 = ((zNMask | 1) << 1) as f32 * z1;
    let xAFlipMask1 = (-2 - (xNMask << 2)) as f32 * x1 - 1.0;
    let yAFlipMask1 = (-2 - (yNMask << 2)) as f32 * y1 - 1.0;
    let zAFlipMask1 = (-2 - (zNMask << 2)) as f32 * z1 - 1.0;

    let mut skip5 = false;
    let a2 = xAFlipMask0 + a0;
    if a2 > 0.0 {
        let x2 = x0 - (xNMask | 1) as f32;
        let y2 = y0;
        let z2 = z0;
        value += (a2 * a2)
            * (a2 * a2)
            * grad3(
                seed,
                xrbp + (Wrapping(!xNMask as i64) & Wrapping(PRIME_X)),
                yrbp + (Wrapping(yNMask as i64) & Wrapping(PRIME_Y)),
                zrbp + (Wrapping(zNMask as i64) & Wrapping(PRIME_Z)),
                x2,
                y2,
                z2,
            );
    } else {
        let a3 = yAFlipMask0 + zAFlipMask0 + a0;
        if a3 > 0.0 {
            let x3 = x0;
            let y3 = y0 - (yNMask | 1) as f32;
            let z3 = z0 - (zNMask | 1) as f32;
            value += (a3 * a3)
                * (a3 * a3)
                * grad3(
                    seed,
                    xrbp + (Wrapping(xNMask as i64) & Wrapping(PRIME_X)),
                    yrbp + (Wrapping(!yNMask as i64) & Wrapping(PRIME_Y)),
                    zrbp + (Wrapping(!zNMask as i64) & Wrapping(PRIME_Z)),
                    x3,
                    y3,
                    z3,
                );
        }

        let a4 = xAFlipMask1 + a1;
        if a4 > 0.0 {
            let x4 = (xNMask | 1) as f32 + x1;
            let y4 = y1;
            let z4 = z1;
            value += (a4 * a4)
                * (a4 * a4)
                * grad3(
                    seed2,
                    xrbp + (Wrapping(xNMask as i64) & (Wrapping(PRIME_X) << 1)),
                    yrbp + Wrapping(PRIME_Y),
                    zrbp + Wrapping(PRIME_Z),
                    x4,
                    y4,
                    z4,
                );
            skip5 = true;
        }
    }

    let mut skip9 = false;
    let a6 = yAFlipMask0 + a0;
    if a6 > 0.0 {
        let x6 = x0;
        let y6 = y0 - (yNMask | 1) as f32;
        let z6 = z0;
        value += (a6 * a6)
            * (a6 * a6)
            * grad3(
                seed,
                xrbp + (Wrapping(xNMask as i64) & Wrapping(PRIME_X)),
                yrbp + (Wrapping(!yNMask as i64) & Wrapping(PRIME_Y)),
                zrbp + (Wrapping(zNMask as i64) & Wrapping(PRIME_Z)),
                x6,
                y6,
                z6,
            );
    } else {
        let a7 = xAFlipMask0 + zAFlipMask0 + a0;
        if a7 > 0.0 {
            let x7 = x0 - (xNMask | 1) as f32;
            let y7 = y0;
            let z7 = z0 - (zNMask | 1) as f32;
            value += (a7 * a7)
                * (a7 * a7)
                * grad3(
                    seed,
                    xrbp + (Wrapping(!xNMask as i64) & Wrapping(PRIME_X)),
                    yrbp + (Wrapping(yNMask as i64) & Wrapping(PRIME_Y)),
                    zrbp + (Wrapping(!zNMask as i64) & Wrapping(PRIME_Z)),
                    x7,
                    y7,
                    z7,
                );
        }

        let a8 = yAFlipMask1 + a1;
        if a8 > 0.0 {
            let x8 = x1;
            let y8 = (yNMask | 1) as f32 + y1;
            let z8 = z1;
            value += (a8 * a8)
                * (a8 * a8)
                * grad3(
                    seed2,
                    xrbp + Wrapping(PRIME_X),
                    yrbp + (Wrapping(yNMask as i64) & (Wrapping(PRIME_Y) << 1)),
                    zrbp + Wrapping(PRIME_Z),
                    x8,
                    y8,
                    z8,
                );
            skip9 = true;
        }
    }

    let mut skipD = false;
    let aA = zAFlipMask0 + a0;
    if aA > 0.0 {
        let xA = x0;
        let yA = y0;
        let zA = z0 - (zNMask | 1) as f32;
        value += (aA * aA)
            * (aA * aA)
            * grad3(
                seed,
                xrbp + (Wrapping(xNMask as i64) & Wrapping(PRIME_X)),
                yrbp + (Wrapping(yNMask as i64) & Wrapping(PRIME_Y)),
                zrbp + (Wrapping(!zNMask as i64) & Wrapping(PRIME_Z)),
                xA,
                yA,
                zA,
            );
    } else {
        let aB = xAFlipMask0 + yAFlipMask0 + a0;
        if aB > 0.0 {
            let xB = x0 - (xNMask | 1) as f32;
            let yB = y0 - (yNMask | 1) as f32;
            let zB = z0;
            value += (aB * aB)
                * (aB * aB)
                * grad3(
                    seed,
                    xrbp + (Wrapping(!xNMask as i64) & Wrapping(PRIME_X)),
                    yrbp + (Wrapping(!yNMask as i64) & Wrapping(PRIME_Y)),
                    zrbp + (Wrapping(zNMask as i64) & Wrapping(PRIME_Z)),
                    xB,
                    yB,
                    zB,
                );
        }

        let aC = zAFlipMask1 + a1;
        if aC > 0.0 {
            let xC = x1;
            let yC = y1;
            let zC = (zNMask | 1) as f32 + z1;
            value += (aC * aC)
                * (aC * aC)
                * grad3(
                    seed2,
                    xrbp + Wrapping(PRIME_X),
                    yrbp + Wrapping(PRIME_Y),
                    zrbp + (Wrapping(zNMask as i64) & (Wrapping(PRIME_Z) << 1)),
                    xC,
                    yC,
                    zC,
                );
            skipD = true;
        }
    }

    if !skip5 {
        let a5 = yAFlipMask1 + zAFlipMask1 + a1;
        if a5 > 0.0 {
            let x5 = x1;
            let y5 = (yNMask | 1) as f32 + y1;
            let z5 = (zNMask | 1) as f32 + z1;
            value += (a5 * a5)
                * (a5 * a5)
                * grad3(
                    seed2,
                    xrbp + Wrapping(PRIME_X),
                    yrbp + (Wrapping(yNMask as i64) & (Wrapping(PRIME_Y) << 1)),
                    zrbp + (Wrapping(zNMask as i64) & (Wrapping(PRIME_Z) << 1)),
                    x5,
                    y5,
                    z5,
                );
        }
    }

    if !skip9 {
        let a9 = xAFlipMask1 + zAFlipMask1 + a1;
        if a9 > 0.0 {
            let x9 = (xNMask | 1) as f32 + x1;
            let y9 = y1;
            let z9 = (zNMask | 1) as f32 + z1;
            value += (a9 * a9)
                * (a9 * a9)
                * grad3(
                    seed2,
                    xrbp + (Wrapping(xNMask as i64) & (Wrapping(PRIME_X) << 1)),
                    yrbp + Wrapping(PRIME_Y),
                    zrbp + (Wrapping(zNMask as i64) & (Wrapping(PRIME_Z) << 1)),
                    x9,
                    y9,
                    z9,
                );
        }
    }

    if !skipD {
        let aD = xAFlipMask1 + yAFlipMask1 + a1;
        if aD > 0.0 {
            let xD = (xNMask | 1) as f32 + x1;
            let yD = (yNMask | 1) as f32 + y1;
            let zD = z1;
            value += (aD * aD)
                * (aD * aD)
                * grad3(
                    seed2,
                    xrbp + (Wrapping(xNMask as i64) & (Wrapping(PRIME_X) << 1)),
                    yrbp + (Wrapping(yNMask as i64) & (Wrapping(PRIME_Y) << 1)),
                    zrbp + Wrapping(PRIME_Z),
                    xD,
                    yD,
                    zD,
                );
        }
    }

    value
}

/// 4D SuperSimplex noise, with XYZ oriented like noise3_ImproveXY
/// and W for an extra degree of freedom. W repeats eventually.
/// Recommended for time-varied animations which texture a 3D object (W=time)
/// in a space where Z is vertical
pub fn noise4_ImproveXYZ_ImproveXY([x, y, z, w]: [f32; 4], seed: i64) -> f32 {
    let xy = x + y;
    let s2 = xy * -0.21132486540518699998;
    let zz = z * 0.28867513459481294226;
    let ww = w * 1.118033988749894;
    let xr = x + (zz + ww + s2);
    let yr = y + (zz + ww + s2);
    let zr = xy * -0.57735026918962599998 + (zz + ww);
    let wr = z * -0.866025403784439 + ww;

    noise4_UnskewedBase([xr, yr, zr, wr], seed)
}

/// 4D SuperSimplex noise, with XYZ oriented like noise3_ImproveXZ
/// and W for an extra degree of freedom. W repeats eventually.
/// Recommended for time-varied animations which texture a 3D object (W=time)
/// in a space where Y is vertical
pub fn noise4_ImproveXYZ_ImproveXZ([x, y, z, w]: [f32; 4], seed: i64) -> f32 {
    let xz = x + z;
    let s2 = xz * -0.21132486540518699998;
    let yy = y * 0.28867513459481294226;
    let ww = w * 1.118033988749894;
    let xr = x + (yy + ww + s2);
    let zr = z + (yy + ww + s2);
    let yr = xz * -0.57735026918962599998 + (yy + ww);
    let wr = y * -0.866025403784439 + ww;

    noise4_UnskewedBase([xr, yr, zr, wr], seed)
}

/// 4D SuperSimplex noise, with XYZ oriented like noise3_Fallback
/// and W for an extra degree of freedom. W repeats eventually.
/// Recommended for time-varied animations which texture a 3D object (W=time)
/// where there isn't a clear distinction between horizontal and vertical
pub fn noise4_ImproveXYZ([x, y, z, w]: [f32; 4], seed: i64) -> f32 {
    let xyz = x + y + z;
    let ww = w * 1.118033988749894;
    let s2 = xyz * -0.16666666666666666 + ww;
    let xs = x + s2;
    let ys = y + s2;
    let zs = z + s2;
    let ws = -0.5 * xyz + ww;

    noise4_UnskewedBase([xs, ys, zs, ws], seed)
}

/// 4D SuperSimplex noise, with XY and ZW forming orthogonal triangular-based planes.
/// Recommended for 3D terrain, where X and Y (or Z and W) are horizontal.
/// Recommended for noise(x, y, sin(time), cos(time)) trick.
pub fn noise4_ImproveXY_ImproveZW([x, y, z, w]: [f32; 4], seed: i64) -> f32 {
    let s2 = (x + y) * -0.28522513987434876941 + (z + w) * 0.83897065470611435718;
    let t2 = (z + w) * 0.21939749883706435719 + (x + y) * -0.48214856493302476942;
    let xs = x + s2;
    let ys = y + s2;
    let zs = z + t2;
    let ws = w + t2;

    noise4_UnskewedBase([xs, ys, zs, ws], seed)
}

/// 4D SuperSimplex noise, fallback lattice orientation.
pub fn noise4_Fallback([x, y, z, w]: [f32; 4], seed: i64) -> f32 {
    // Get points for A4 lattice
    let s = SKEW_4D * (x + y + z + w);
    let xs = x + s;
    let ys = y + s;
    let zs = z + s;
    let ws = w + s;

    noise4_UnskewedBase([xs, ys, zs, ws], seed)
}

/// 4D SuperSimplex noise base.
/// Using ultra-simple 4x4x4x4 lookup partitioning.
/// This isn't as elegant or SIMD/GPU/etc. portable as other approaches,
/// but it competes performance-wise with optimized 2014 OpenSimplex.
pub fn noise4_UnskewedBase([xs, ys, zs, ws]: [f32; 4], seed: i64) -> f32 {
    let seed = Wrapping(seed);

    // Get base points and offsets
    let xsb = fastFloor(xs);
    let ysb = fastFloor(ys);
    let zsb = fastFloor(zs);
    let wsb = fastFloor(ws);
    let xsi = (xs - xsb as f32) as f32;
    let ysi = (ys - ysb as f32) as f32;
    let zsi = (zs - zsb as f32) as f32;
    let wsi = (ws - wsb as f32) as f32;

    // Unskewed offsets
    let ssi = (xsi + ysi + zsi + wsi) * UNSKEW_4D;
    let xi = xsi + ssi;
    let yi = ysi + ssi;
    let zi = zsi + ssi;
    let wi = wsi + ssi;

    // Prime pre-multiplication for hash.
    let xsvp = Wrapping(xsb as i64) * Wrapping(PRIME_X);
    let ysvp = Wrapping(ysb as i64) * Wrapping(PRIME_Y);
    let zsvp = Wrapping(zsb as i64) * Wrapping(PRIME_Z);
    let wsvp = Wrapping(wsb as i64) * Wrapping(PRIME_W);

    // Index into initial table.
    let index = ((fastFloor(xs * 4.0) & 3) << 0) | ((fastFloor(ys * 4.0) & 3) << 2) | ((fastFloor(zs * 4.0) & 3) << 4) | ((fastFloor(ws * 4.0) & 3) << 6);

    // Point contributions
    let mut value = 0.0;
    let secondaryIndexStartAndStop = LOOKUP_4D_A[index as usize];
    let secondaryIndexStart = secondaryIndexStartAndStop & 0xFFFF;
    let secondaryIndexStop = secondaryIndexStartAndStop >> 16;
    for i in secondaryIndexStart..secondaryIndexStop {
        let c = &LOOKUP_4D_B[i];
        let dx = xi + c.dx;
        let dy = yi + c.dy;
        let dz = zi + c.dz;
        let dw = wi + c.dw;
        let mut a = (dx * dx + dy * dy) + (dz * dz + dw * dw);
        if a < RSQUARED_4D {
            a -= RSQUARED_4D;
            a *= a;
            value += a * a * grad4(seed, xsvp + Wrapping(c.xsvp), ysvp + Wrapping(c.ysvp), zsvp + Wrapping(c.zsvp), wsvp + Wrapping(c.wsvp), dx, dy, dz, dw);
        }
    }
    value
}

fn grad2(seed: Wrapping<i64>, xsvp: Wrapping<i64>, ysvp: Wrapping<i64>, dx: f32, dy: f32) -> f32 {
    let mut hash = seed ^ xsvp ^ ysvp;
    hash *= HASH_MULTIPLIER;
    hash ^= hash.0 >> (64 - N_GRADS_2D_EXPONENT + 1);
    let gi = (hash.0 as i32 & ((N_GRADS_2D - 1) << 1)) as usize;
    let grads = GRADIENTS_2D;
    grads[gi | 0] * dx + grads[gi | 1] * dy
}

fn grad3(seed: Wrapping<i64>, xrvp: Wrapping<i64>, yrvp: Wrapping<i64>, zrvp: Wrapping<i64>, dx: f32, dy: f32, dz: f32) -> f32 {
    let mut hash = (seed ^ xrvp) ^ (yrvp ^ zrvp);
    hash *= HASH_MULTIPLIER;
    hash ^= hash.0 >> (64 - N_GRADS_3D_EXPONENT + 2);
    let gi = (hash.0 as i32 & ((N_GRADS_3D - 1) << 2)) as usize;
    let grads = GRADIENTS_3D;
    grads[gi | 0] * dx + grads[gi | 1] * dy + grads[gi | 2] * dz
}

fn grad4(seed: Wrapping<i64>, xsvp: Wrapping<i64>, ysvp: Wrapping<i64>, zsvp: Wrapping<i64>, wsvp: Wrapping<i64>, dx: f32, dy: f32, dz: f32, dw: f32) -> f32 {
    let mut hash = seed ^ (xsvp ^ ysvp) ^ (zsvp ^ wsvp);
    hash *= HASH_MULTIPLIER;
    hash ^= hash.0 >> (64 - N_GRADS_4D_EXPONENT + 2);
    let gi = (hash.0 as i32 & ((N_GRADS_4D - 1) << 2)) as usize;
    let grads = GRADIENTS_4D;
    (grads[gi | 0] * dx + grads[gi | 1] * dy) + (grads[gi | 2] * dz + grads[gi | 3] * dw)
}

fn fastFloor(x: f32) -> i32 {
    let xi = x as i32;
    if x < xi as f32 {
        xi - 1
    } else {
        xi
    }
}

pub(super) struct LatticeVertex4D {
    pub(super) dx: f32,
    pub(super) dy: f32,
    pub(super) dz: f32,
    pub(super) dw: f32,
    pub(super) xsvp: i64,
    pub(super) ysvp: i64,
    pub(super) zsvp: i64,
    pub(super) wsvp: i64,
}
