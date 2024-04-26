use core::convert::identity;

use crate::{
    private_prelude::*,
    tests::{test_n, test_seed},
};
use fastnoise_lite::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

struct Settings {
    name: &'static str,
    cellular_distance_function: CellularDistanceFunction,
    cellular_return_type: CellularReturnType,
    noise_type: NoiseType,
    gen2: Option<fn([f32; 2], i32) -> f32>,
    gen3: Option<fn([f32; 3], i32) -> f32>,
    improve3: Option<fn([f32; 3]) -> [f32; 3]>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: "",
            cellular_distance_function: CellularDistanceFunction::Euclidean,
            cellular_return_type: CellularReturnType::Distance,
            noise_type: NoiseType::Value,
            gen2: None,
            gen3: None,
            improve3: None,
        }
    }
}

pub fn default<T: Default>() -> T {
    Default::default()
}

macro_rules! settings {
    ($($module:ident)::* as { $($field:ident: $value:expr),* $(,)? }) => {{
      #[allow(unused_imports)] use NoiseType::*;
      #[allow(unused_imports)] use CellularDistanceFunction::*;
      #[allow(unused_imports)] use CellularReturnType::*;

      Settings {
        name: stringify!($($module)*),
        gen2: Some(scalar::$($module::)*gen2),
        gen3: Some(scalar::$($module::)*gen3),
        $(
          $field: $value,
        )*
        ..default()
      }
    }};
}

#[test]
fn simple() {
    const SEED: i32 = 1;
    const FREQUENCY: f32 = 0.01;

    let settings = [
        settings!(perlin as { noise_type: Perlin }),
        settings!(open_simplex_2 as { noise_type: OpenSimplex2, improve3: Some(scalar::open_simplex_2::improve3) }),
        settings!(open_simplex_2s as { noise_type: OpenSimplex2S, improve3: Some(scalar::open_simplex_2s::improve3) }),
        settings!(value as { noise_type: Value }),
        settings!(value_cubic as { noise_type: ValueCubic }),
        settings!(cell_distance as { noise_type: Cellular, cellular_distance_function: Euclidean }),
        settings!(cell_distance_sq as { noise_type: Cellular, cellular_distance_function: EuclideanSq }),
        settings!(cell_value as { noise_type: Cellular, cellular_distance_function: EuclideanSq, cellular_return_type: CellValue }),
    ];

    let n = test_n();
    let mut rng = SmallRng::seed_from_u64(test_seed());

    for setting in settings {
        let mut noise = FastNoiseLite::with_seed(SEED);
        noise.set_frequency(Some(FREQUENCY));
        noise.set_noise_type(Some(setting.noise_type));
        noise.set_cellular_distance_function(Some(setting.cellular_distance_function));
        noise.set_cellular_return_type(Some(setting.cellular_return_type));

        if let Some(gen) = setting.gen2 {
            let gen2 = |mut x: f32, mut y: f32| {
                x *= FREQUENCY;
                y *= FREQUENCY;
                gen([x, y], SEED)
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
        }

        if let Some(gen) = setting.gen3 {
            let improve3 = setting.improve3.unwrap_or(identity);
            let gen3 = |mut x: f32, mut y: f32, mut z: f32| {
                x *= FREQUENCY;
                y *= FREQUENCY;
                z *= FREQUENCY;
                gen(improve3([x, y, z]), SEED)
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
                    noise.set_frequency(FREQUENCY);
                    noise.set_fractal_gain(gain);
                    noise.set_fractal_weighted_strength(weighted_strength);
                    noise.set_fractal_lacunarity(lacunarity);
                    noise.set_fractal_octaves(octaves as usize);
                    noise.set_fractal_type(FractalType::FBm);
                    noise.set_noise_type(NoiseType::Perlin);

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
                                let reference = noise.get_noise_3d(x, y, z);
                                let ours = gen3(x, y, z);

                                if reference != ours {
                                    let reference = noise.get_noise_3d(x, y, z);
                                    let ours = gen3(x, y, z);
                                    assert_eq!(reference, ours, "gain={gain} weighted_strength={weighted_strength} lacunarity={lacunarity} octaves={octaves} x={x} y={y} z={z}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
