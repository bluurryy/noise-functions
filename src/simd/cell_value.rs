use crate::private_prelude::*;

#[inline]
pub(crate) fn gen2(pos: f32x2, seed: i32, jitter: f32) -> f32 {
    let rounded = round_to_int(pos);
    let mut distance: f32 = 1e10;
    let mut closest_hash: i32 = 0;

    let mut x_primed = rounded[0].wrapping_sub(1).wrapping_mul(PRIME_X);
    let y_primed_base = rounded[1].wrapping_sub(1).wrapping_mul(PRIME_Y);

    for xi in cell_neighbours(rounded[0]) {
        let mut y_primed = y_primed_base;

        for yi in cell_neighbours(rounded[1]) {
            let hash = hash2(seed, x_primed, y_primed);
            let rand = RAND_VECS_2D[Index2::new(hash)].0;
            let coor = f32x2::from_array([xi as f32, yi as f32]);
            let vec = (coor - pos) + rand * splat(jitter);
            let new_distance = length_squared(vec);

            if new_distance < distance {
                distance = new_distance;
                closest_hash = hash;
            }

            y_primed = y_primed.wrapping_add(PRIME_Y);
        }
        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    closest_hash as f32 * (1.0 / 2147483648.0)
}

#[inline]
pub fn gen3(pos: f32x4, seed: i32, jitter: f32) -> f32 {
    let rounded = round_to_int(pos);

    let mut distance: f32 = 1e10;
    let mut closest_hash: i32 = 0;

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
                let vec = (coor - pos) + rand * splat(jitter);
                let new_distance = length_squared(vec);

                if new_distance < distance {
                    distance = new_distance;
                    closest_hash = hash;
                }

                z_primed = z_primed.wrapping_add(PRIME_Z);
            }
            y_primed = y_primed.wrapping_add(PRIME_Y);
        }
        x_primed = x_primed.wrapping_add(PRIME_X);
    }

    closest_hash as f32 * (1.0 / 2147483648.0)
}
