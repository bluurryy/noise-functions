use super::{cell_neighbours, fast_min, hash2, hash3, round_to_int, JITTER_2D, JITTER_3D, PRIME_X, PRIME_Y, PRIME_Z, RAND_VECS_2D, RAND_VECS_3D};

#[cfg(feature = "nightly-simd")]
use super::{length_squared, splat};

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[inline(always)]
pub(crate) fn gen2(jitter: f32, [x, y]: [f32; 2], seed: i32) -> f32 {
    let jitter = jitter * JITTER_2D;

    let xr: i32 = round_to_int(x);
    let yr: i32 = round_to_int(y);

    let mut distance: f32 = 1e10;
    let mut x_primed: i32 = xr.wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base: i32 = yr.wrapping_sub(1).wrapping_mul(PRIME_Y);

    for xi in cell_neighbours(xr) {
        let mut y_primed: i32 = y_primed_base;

        for yi in cell_neighbours(yr) {
            let hash: i32 = hash2(seed, x_primed, y_primed);
            let [rand_x, rand_y] = *RAND_VECS_2D[hash].as_array();

            let vec_x: f32 = (xi as f32 - x) + rand_x * jitter;
            let vec_y: f32 = (yi as f32 - y) + rand_y * jitter;

            let new_distance: f32 = vec_x * vec_x + vec_y * vec_y;
            distance = fast_min(distance, new_distance);
            y_primed = y_primed.wrapping_add(PRIME_Y);
        }
        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    distance
}

#[inline(always)]
pub(crate) fn gen3(jitter: f32, [x, y, z]: [f32; 3], seed: i32) -> f32 {
    let jitter = jitter * JITTER_3D;

    let xr: i32 = round_to_int(x);
    let yr: i32 = round_to_int(y);
    let zr: i32 = round_to_int(z);

    let mut distance: f32 = 1e10;

    let mut x_primed: i32 = xr.wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base: i32 = yr.wrapping_sub(1).wrapping_mul(PRIME_Y);
    let z_primed_base: i32 = zr.wrapping_sub(1).wrapping_mul(PRIME_Z);

    for xi in cell_neighbours(xr) {
        let mut y_primed: i32 = y_primed_base;

        for yi in cell_neighbours(yr) {
            let mut z_primed: i32 = z_primed_base;

            for zi in cell_neighbours(zr) {
                let hash: i32 = hash3(seed, x_primed, y_primed, z_primed);
                let [rand_x, rand_y, rand_z, _] = *RAND_VECS_3D[hash].as_array();

                let vec_x: f32 = (xi as f32 - x) + rand_x * jitter;
                let vec_y: f32 = (yi as f32 - y) + rand_y * jitter;
                let vec_z: f32 = (zi as f32 - z) + rand_z * jitter;

                let new_distance: f32 = vec_x * vec_x + vec_y * vec_y + vec_z * vec_z;
                distance = fast_min(distance, new_distance);
                z_primed = z_primed.wrapping_add(PRIME_Z);
            }
            y_primed = y_primed.wrapping_add(PRIME_Y);
        }
        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    distance
}

#[inline(always)]
#[cfg(feature = "nightly-simd")]
pub(crate) fn gen2a(jitter: f32, point: f32x2, seed: i32) -> f32 {
    let jitter = jitter * JITTER_2D;

    let rounded = round_to_int(point);
    let mut distance: f32 = 1e10;

    let mut x_primed = rounded[0].wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base = rounded[1].wrapping_sub(1).wrapping_mul(PRIME_Y);

    for xi in cell_neighbours(rounded[0]) {
        let mut y_primed = y_primed_base;

        for yi in cell_neighbours(rounded[1]) {
            let hash = hash2(seed, x_primed, y_primed);
            let rand = RAND_VECS_2D[hash].0;
            let coord = f32x2::from_array([xi as f32, yi as f32]);
            let vec = (coord - point) + rand * splat(jitter);
            let new_distance = length_squared(vec);
            distance = fast_min(distance, new_distance);
            y_primed = y_primed.wrapping_add(PRIME_Y);
        }

        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    distance
}

#[inline(always)]
#[cfg(feature = "nightly-simd")]
pub(crate) fn gen3a(jitter: f32, point: f32x4, seed: i32) -> f32 {
    let jitter = jitter * JITTER_3D;

    let rounded = round_to_int(point);
    let mut distance: f32 = 1e10;

    let mut x_primed = rounded[0].wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base = rounded[1].wrapping_sub(1).wrapping_mul(PRIME_Y);
    let z_primed_base = rounded[2].wrapping_sub(1).wrapping_mul(PRIME_Z);

    for xi in cell_neighbours(rounded[0]) {
        let mut y_primed = y_primed_base;

        for yi in cell_neighbours(rounded[1]) {
            let mut z_primed = z_primed_base;

            for zi in cell_neighbours(rounded[2]) {
                let hash = hash3(seed, x_primed, y_primed, z_primed);
                let rand = RAND_VECS_3D[hash].0;
                let coor = f32x4::from_array([xi as f32, yi as f32, zi as f32, zi as f32]);
                let vec = (coor - point) + rand * splat(jitter);
                let new_distance = length_squared(vec);
                distance = fast_min(distance, new_distance);
                z_primed = z_primed.wrapping_add(PRIME_Z);
            }
            y_primed = y_primed.wrapping_add(PRIME_Y);
        }
        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    distance
}
