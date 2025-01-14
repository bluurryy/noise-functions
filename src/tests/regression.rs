use std::boxed::Box;

use fastnoise_lite::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    private_prelude::*,
    tests::{test_n, test_seed},
};

trait SampleBoth: Sample<2> + Sample<3> {}
impl<T> SampleBoth for T where T: Sample<2> + Sample<3> {}

struct Settings {
    name: &'static str,
    cellular_distance_function: CellularDistanceFunction,
    cellular_return_type: CellularReturnType,
    noise_type: NoiseType,
    sampler: Option<Box<dyn SampleBoth>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: "",
            cellular_distance_function: CellularDistanceFunction::Euclidean,
            cellular_return_type: CellularReturnType::Distance,
            noise_type: NoiseType::Value,
            sampler: None,
        }
    }
}

pub fn default<T: Default>() -> T {
    Default::default()
}

macro_rules! settings {
    ($name:ident as { $($field:ident: $value:expr),* $(,)? }) => {{
      #[allow(unused_imports)] use NoiseType::*;
      #[allow(unused_imports)] use CellularDistanceFunction::*;
      #[allow(unused_imports)] use CellularReturnType::*;
      #[allow(clippy::needless_update)]
      Settings {
        name: stringify!($name),
        sampler: Some(Box::new(crate::$name.seed(SEED))),
        $(
          $field: $value,
        )*
        ..default()
      }
    }};
}

#[test]
fn simple() {
    const SEED: i32 = 0;
    const FREQUENCY: f32 = 0.01;

    let settings = [
        settings!(Perlin as { noise_type: Perlin }),
        settings!(OpenSimplex2 as { noise_type: OpenSimplex2 }),
        settings!(OpenSimplex2s as { noise_type: OpenSimplex2S }),
        settings!(Value as { noise_type: Value }),
        settings!(ValueCubic as { noise_type: ValueCubic }),
        settings!(CellDistance as { noise_type: Cellular, cellular_distance_function: Euclidean }),
        settings!(CellDistanceSq as { noise_type: Cellular, cellular_distance_function: EuclideanSq }),
        settings!(CellValue as { noise_type: Cellular, cellular_distance_function: EuclideanSq, cellular_return_type: CellValue }),
    ];

    let n = test_n();
    let mut rng = SmallRng::seed_from_u64(test_seed());

    for setting in settings {
        let mut noise = FastNoiseLite::with_seed(SEED);
        noise.set_frequency(Some(FREQUENCY));
        noise.set_noise_type(Some(setting.noise_type));
        noise.set_cellular_distance_function(Some(setting.cellular_distance_function));
        noise.set_cellular_return_type(Some(setting.cellular_return_type));

        if let Some(sampler) = setting.sampler.as_deref() {
            let gen2 = |mut x: f32, mut y: f32| {
                x *= FREQUENCY;
                y *= FREQUENCY;
                sampler.sample([x, y])
            };

            for i in 0..n {
                let [x, y] = if i == 0 { [0.0; 2] } else { rng.gen() };

                let reference = noise.get_noise_2d(x, y);
                let ours = gen2(x, y);

                if reference != ours {
                    let reference = noise.get_noise_2d(x, y);
                    let ours = gen2(x, y);
                    assert_eq!(reference, ours, "{}::gen2 x={} y={}", setting.name, x, y);
                }
            }
            let gen3 = |mut x: f32, mut y: f32, mut z: f32| {
                x *= FREQUENCY;
                y *= FREQUENCY;
                z *= FREQUENCY;
                sampler.sample([x, y, z])
            };

            for i in 0..n {
                let [x, y, z] = if i == 0 { [0.0; 3] } else { rng.gen() };

                let reference = noise.get_noise_3d(x, y, z);
                let ours = gen3(x, y, z);

                if reference != ours {
                    let reference = noise.get_noise_3d(x, y, z);
                    let ours = gen3(x, y, z);
                    assert_eq!(reference, ours, "{}::gen3 x={} y={} z={}", setting.name, x, y, z);
                }
            }
        }
    }
}

#[cfg(any())]
#[test]
fn fbm_weighted() {
    fn range(min: f32, max: f32, steps: u32) -> impl Iterator<Item = f32> {
        let delta = max - min;
        (0..steps).map(move |i| min + (i as f32 * delta))
    }

    const SEED: i32 = 1;
    const FREQUENCY: f32 = 0.01;
    const SIZE: u32 = 5;

    for gain in range(0.0, 1.0, 5) {
        for weighted_strength in range(0.0, 1.0, 5) {
            for lacunarity in range(0.0, 10.0, 10) {
                for octaves in 3..=5 {
                    let mut noise = FastNoiseLite::new(SEED);
                    basic_noise.set_frequency(FREQUENCY);
                    basic_noise.set_fractal_gain(gain);
                    basic_noise.set_fractal_weighted_strength(weighted_strength);
                    basic_noise.set_fractal_lacunarity(lacunarity);
                    basic_noise.set_fractal_octaves(octaves as usize);
                    basic_noise.set_fractal_type(FractalType::FBm);
                    basic_noise.set_noise_type(NoiseType::Perlin);

                    let gen3 = |mut x: f32, mut y: f32, mut z: f32| {
                        x *= FREQUENCY;
                        y *= FREQUENCY;
                        z *= FREQUENCY;

                        let settings = fbm_weighted_settings::settings(octaves, gain, lacunarity, weighted_strength);
                        crate::fbm_weighted(settings, perlin::gen3)([x, y, z], SEED)
                    };

                    for z in 0..SIZE {
                        for y in 0..SIZE {
                            for x in 0..SIZE {
                                let x = x as f32;
                                let y = y as f32;
                                let z = z as f32;
                                let reference = basic_noise.get_noise_3d(x, y, z);
                                let ours = gen3(x, y, z);

                                if reference != ours {
                                    let reference = basic_noise.get_noise_3d(x, y, z);
                                    let ours = gen3(x, y, z);
                                    assert_eq!(
                                        reference, ours,
                                        "gain={gain} weighted_strength={weighted_strength} lacunarity={lacunarity} octaves={octaves} x={x} y={y} z={z}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
