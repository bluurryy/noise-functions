use crate::base::impl_noise;

#[cfg(feature = "nightly-simd")]
use core::simd::{f32x2, f32x4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastCellValue {
    pub jitter: f32,
}

impl_noise!(23 FastCellValue);

impl FastCellValue {
    #[inline]
    fn gen2(self, [x, y]: [f32; 2], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cell_neighbours, hash2, round_to_int, Index2, PRIME_X, PRIME_Y, RAND_VECS_2D};

        let xr: i32 = round_to_int(x);
        let yr: i32 = round_to_int(y);

        let mut distance0: f32 = 1e10;
        let mut closest_hash: i32 = 0;

        let mut x_primed: i32 = xr.wrapping_sub(1).wrapping_mul(PRIME_X);
        let y_primed_base: i32 = yr.wrapping_sub(1).wrapping_mul(PRIME_Y);

        for xi in cell_neighbours(xr) {
            let mut y_primed: i32 = y_primed_base;

            for yi in cell_neighbours(yr) {
                let hash: i32 = hash2(seed, x_primed, y_primed);
                let [rand_x, rand_y] = *RAND_VECS_2D[Index2::new(hash)].as_array();

                let vec_x: f32 = (xi as f32 - x) + rand_x * self.jitter;
                let vec_y: f32 = (yi as f32 - y) + rand_y * self.jitter;

                let new_distance: f32 = vec_x * vec_x + vec_y * vec_y;

                if new_distance < distance0 {
                    distance0 = new_distance;
                    closest_hash = hash;
                }

                y_primed = y_primed.wrapping_add(PRIME_Y);
            }
            x_primed = x_primed.wrapping_add(PRIME_X);
        }

        closest_hash as f32 * (1.0 / 2147483648.0)
    }

    #[inline]
    fn gen3(self, [x, y, z]: [f32; 3], seed: i32) -> f32 {
        // implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cell_neighbours, hash3, round_to_int, Index3, PRIME_X, PRIME_Y, PRIME_Z, RAND_VECS_3D};

        let xr: i32 = round_to_int(x);
        let yr: i32 = round_to_int(y);
        let zr: i32 = round_to_int(z);

        let mut distance0: f32 = 1e10;
        let mut closest_hash: i32 = 0;

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

                    let vec_x: f32 = (xi as f32 - x) + rand_x * self.jitter;
                    let vec_y: f32 = (yi as f32 - y) + rand_y * self.jitter;
                    let vec_z: f32 = (zi as f32 - z) + rand_z * self.jitter;

                    let new_distance: f32 = vec_x * vec_x + vec_y * vec_y + vec_z * vec_z;

                    if new_distance < distance0 {
                        distance0 = new_distance;
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

    #[inline]
    #[cfg(feature = "nightly-simd")]
    fn gen2a(self, point: f32x2, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cell_neighbours, hash2, length_squared, round_to_int, splat, Index2, PRIME_X, PRIME_Y, RAND_VECS_2D};

        let rounded = round_to_int(point);
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
                let vec = (coor - point) + rand * splat(self.jitter);
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
    #[cfg(feature = "nightly-simd")]
    fn gen3a(self, point: f32x4, seed: i32) -> f32 {
        // based on the implementation from FastNoiseLite
        use crate::from_fast_noise_lite::{cell_neighbours, hash3, length_squared, round_to_int, splat, Index3, PRIME_X, PRIME_Y, PRIME_Z, RAND_VECS_3D};

        let rounded = round_to_int(point);

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
                    let vec = (coor - point) + rand * splat(self.jitter);
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
}
