use std::boxed::Box;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{private_prelude::*, tests::test_n};

use super::test_seed;

trait SampleAll: Sample<2> + Sample<3> + Sample<2, f32x2> + Sample<3, f32x4> {}
impl<T> SampleAll for T where T: Sample<2> + Sample<3> + Sample<2, f32x2> + Sample<3, f32x4> {}

#[test]
#[allow(clippy::type_complexity)]
pub fn simd_regression() {
    let seed = 1;
    let frequency = 0.1;
    let n = test_n();
    let mut rng = SmallRng::seed_from_u64(test_seed());

    let noises: &[Box<dyn SampleAll>] = &[
        Box::new(CellDistanceSq.seed(seed)),
        Box::new(CellDistance.seed(seed)),
        Box::new(CellValue.seed(seed)),
        Box::new(OpenSimplex2.seed(seed)),
        Box::new(OpenSimplex2s.seed(seed)),
        Box::new(Perlin.seed(seed)),
        Box::new(Value.seed(seed)),
        Box::new(ValueCubic.seed(seed)),
    ];

    for noise in noises {
        for i in 0..n {
            let [mut x, mut y] = if i == 0 { [0.0; 2] } else { rng.gen() };
            x *= frequency;
            y *= frequency;
            let value_scalar = noise.sample([x, y]);
            let value_simd = noise.sample(f32x2::from_array([x, y]));
            assert_eq!(value_scalar, value_simd);
        }
    }
    for noise in noises {
        for i in 0..n {
            let [mut x, mut y, mut z] = if i == 0 { [0.0; 3] } else { rng.gen() };
            x *= frequency;
            y *= frequency;
            z *= frequency;
            let value_scalar = noise.sample([x, y, z]);
            let value_simd = noise.sample(f32x4::from_array([x, y, z, 0.0]));
            assert_eq!(value_scalar, value_simd);
        }
    }
}
