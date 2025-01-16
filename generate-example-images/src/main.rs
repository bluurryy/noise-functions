use image::{ImageBuffer, Rgb};
use noise_functions::*;

const SIZE: usize = 128;
const WIDTH: usize = SIZE;
const HEIGHT: usize = SIZE;
const FREQUENCY: f32 = 3.0;

/// Creates an image from the coordinates x and y in the range of -1..+1.
/// Maps values in a range of -1..+1 to black..white.
fn noise_to_image(noise: impl Sample2) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let scalar = 1.0 / WIDTH.max(HEIGHT) as f32;
    let scalar_times_2 = scalar * 2.0;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let x = (x as f32 * scalar_times_2 - 1.0) * FREQUENCY;
        let y = (y as f32 * scalar_times_2 - 1.0) * FREQUENCY;
        let value = noise.sample2([x, y]);
        let value = ((value * 0.5 + 0.5) * 255.0) as u8;
        *pixel = Rgb([value, value, value]);
    }

    image
}

fn save_jpg(name: &str, noise: impl Sample2) {
    noise_to_image(noise).save(format!("example-images/{name}.jpg")).unwrap();
}

fn coords_01(noise: impl Sample2) -> impl Sample2 {
    NoiseFn(move |point: [f32; 2]| noise.sample2(point.map(|x| x * 0.5)))
}

fn from_01(noise: impl Sample2) -> impl Sample2 {
    NoiseFn(move |point| noise.sample2(point) * 2.0 - 1.0)
}

fn main() {
    // Cell distances start at 0 unlike the others that are in the -1..+1 range.
    // So we use `from_01` to modify their output to be in closer to that range
    // to be able to use the same `save_jpg` function.

    save_jpg("cell_distance_sq", from_01(CellDistanceSq::default()));
    save_jpg("cell_distance", from_01(CellDistance::default()));

    save_jpg("cell_value", CellValue::default());
    save_jpg("perlin", Perlin);
    save_jpg("open_simplex_2", OpenSimplex2);
    save_jpg("open_simplex_2s", OpenSimplex2s);
    save_jpg("value", Value);
    save_jpg("value_cubic", ValueCubic);

    save_jpg("fbm", OpenSimplex2.fbm(3, 0.5, 2.0));
    save_jpg("ridged", OpenSimplex2.ridged().fbm(3, 0.5, 2.0));
    save_jpg("ping_pong", OpenSimplex2.triangle_wave(2.0).fbm(3, 0.5, 2.0).weighted(1.0));

    save_jpg(
        "warped",
        NoiseFn(|point: [f32; 2]| {
            let warp_x = OpenSimplex2s.seed(1).sample(point);
            let warp_y = OpenSimplex2s.seed(2).sample(point);
            let warped = [point[0] + warp_x, point[1] + warp_y];
            OpenSimplex2s.sample(warped)
        }),
    );

    save_jpg(
        "warped_fbm",
        NoiseFn(|point: [f32; 2], seed: i32| {
            let warp_x = OpenSimplex2s.seed(seed + 100).sample(point);
            let warp_y = OpenSimplex2s.seed(seed + 200).sample(point);
            let warped = [point[0] + warp_x, point[1] + warp_y];
            OpenSimplex2s.sample(warped)
        })
        .fbm(3, 0.5, 1.5),
    );

    save_jpg("tileable_perlin", coords_01(Perlin.tileable(FREQUENCY, FREQUENCY).frequency(2.0)));

    save_jpg("tileable_value", coords_01(Value.seed(12).tileable(FREQUENCY, FREQUENCY).frequency(2.0)));

    save_jpg(
        "tileable_cell_value",
        coords_01(CustomCellValue::default().seed(12).tileable(2.15, 2.15).frequency(2.15 * 2.0 / FREQUENCY)),
    );

    save_jpg(
        "tileable_cell_distance_sq",
        from_01(coords_01(NoiseFn(|point: [f32; 2]| {
            let value = CellDistanceSq::default().seed(12).tileable(2.0, 2.0).frequency(2.0 * 2.0 / FREQUENCY).sample2(point);
            value * 1.25
        }))),
    );
}
