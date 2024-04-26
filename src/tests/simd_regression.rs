use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::private_prelude::*;

use crate::tests::test_n;

use super::test_seed;

#[test]
#[allow(clippy::type_complexity)]
pub fn simd_regression() {
    let seed = 1;
    let frequency = 0.1;
    let n = test_n();
    let mut rng = SmallRng::seed_from_u64(test_seed());

    let gen2s: &[(fn([f32; 2], i32) -> f32, fn(f32x2, i32) -> f32)] = &[
        (scalar::cell_distance_sq::gen2, simd::cell_distance_sq::gen2),
        (scalar::cell_distance::gen2, simd::cell_distance::gen2),
        (scalar::cell_value::gen2, simd::cell_value::gen2),
        (scalar::open_simplex_2::gen2, simd::open_simplex_2::gen2),
        (scalar::open_simplex_2s::gen2, simd::open_simplex_2s::gen2),
        (scalar::perlin::gen2, simd::perlin::gen2),
        (scalar::value_cubic::gen2, simd::value_cubic::gen2),
        (scalar::value::gen2, simd::value::gen2),
    ];

    for (gen_scalar, gen_simd) in gen2s {
        for i in 0..n {
            let [mut x, mut y] = if i == 0 { [0.0; 2] } else { rng.gen() };
            x *= frequency;
            y *= frequency;
            let value_scalar = gen_scalar([x, y], seed);
            let value_simd = gen_simd(f32x2::from_array([x, y]), seed);
            assert_eq!(value_scalar, value_simd);
        }
    }

    let gen3s: &[(fn([f32; 3], i32) -> f32, fn(f32x4, i32) -> f32)] = &[
        (scalar::cell_distance_sq::gen3, simd::cell_distance_sq::gen3),
        (scalar::cell_distance::gen3, simd::cell_distance::gen3),
        (scalar::cell_value::gen3, simd::cell_value::gen3),
        (scalar::open_simplex_2::gen3, simd::open_simplex_2::gen3),
        (scalar::open_simplex_2s::gen3, simd::open_simplex_2s::gen3),
        (scalar::perlin::gen3, simd::perlin::gen3),
        (scalar::value_cubic::gen3, simd::value_cubic::gen3),
        (scalar::value::gen3, simd::value::gen3),
    ];

    for (gen_scalar, gen_simd) in gen3s {
        for i in 0..n {
            let [mut x, mut y, mut z] = if i == 0 { [0.0; 3] } else { rng.gen() };
            x *= frequency;
            y *= frequency;
            z *= frequency;
            let value_scalar = gen_scalar([x, y, z], seed);
            let value_simd = gen_simd(f32x4::from_array([x, y, z, 0.0]), seed);
            assert_eq!(value_scalar, value_simd);
        }
    }
}
