use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(point: f32x2, seed: i32, jitter: f32) -> f32 {
    gen2_distance_squared(point, seed, jitter) - 1.0
}

#[inline]
pub fn gen3(point: f32x4, seed: i32, jitter: f32) -> f32 {
    gen3_distance_squared(point, seed, jitter) - 1.0
}

#[inline(always)]
pub fn gen2_distance_squared(point: f32x2, seed: i32, jitter: f32) -> f32 {
    let rounded = round_to_int(point);
    let mut distance: f32 = 1e10;

    let mut x_primed = rounded[0].wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base = rounded[1].wrapping_sub(1).wrapping_mul(PRIME_Y);

    for xi in cell_neighbours(rounded[0]) {
        let mut y_primed = y_primed_base;

        for yi in cell_neighbours(rounded[1]) {
            let hash = hash2(seed, x_primed, y_primed);
            let rand = RAND_VECS_2D[Index2::new(hash)].0;
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
pub fn gen3_distance_squared(point: f32x4, seed: i32, jitter: f32) -> f32 {
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
                let rand = RAND_VECS_3D[Index3::new(hash)].0;
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
