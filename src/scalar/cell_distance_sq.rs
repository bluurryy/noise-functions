use crate::private_prelude::*;

#[inline]
pub fn gen2(pos: [f32; 2], seed: i32) -> f32 {
    gen2_distance_squared(pos, seed) - 1.0
}

#[inline]
pub fn gen3(pos: [f32; 3], seed: i32) -> f32 {
    gen3_distance_squared(pos, seed) - 1.0
}

#[inline(always)]
pub fn gen2_distance_squared([x, y]: [f32; 2], seed: i32) -> f32 {
    const JITTER: f32 = 0.43701595;

    let xr: i32 = round_to_int(x);
    let yr: i32 = round_to_int(y);

    let mut distance: f32 = 1e10;
    let mut x_primed: i32 = xr.wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base: i32 = yr.wrapping_sub(1).wrapping_mul(PRIME_Y);

    for xi in cell_neighbours(xr) {
        let mut y_primed: i32 = y_primed_base;

        for yi in cell_neighbours(yr) {
            let hash: i32 = hash2(seed, x_primed, y_primed);
            let [rand_x, rand_y] = *RAND_VECS_2D[Index2::new(hash)].as_array();

            let vec_x: f32 = (xi as f32 - x) + rand_x * JITTER;
            let vec_y: f32 = (yi as f32 - y) + rand_y * JITTER;

            let new_distance: f32 = vec_x * vec_x + vec_y * vec_y;
            distance = fast_min(distance, new_distance);
            y_primed = y_primed.wrapping_add(PRIME_Y);
        }
        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    distance
}

#[inline(always)]
pub fn gen3_distance_squared([x, y, z]: [f32; 3], seed: i32) -> f32 {
    const JITTER: f32 = 0.39614353;

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
                let [rand_x, rand_y, rand_z, _] = *RAND_VECS_3D[Index3::new(hash)].as_array();

                let vec_x: f32 = (xi as f32 - x) + rand_x * JITTER;
                let vec_y: f32 = (yi as f32 - y) + rand_y * JITTER;
                let vec_z: f32 = (zi as f32 - z) + rand_z * JITTER;

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
