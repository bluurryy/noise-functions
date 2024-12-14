use image::{ImageBuffer, Rgb};
use noise_functions::*;

const SIZE: usize = 128;
const WIDTH: usize = SIZE;
const HEIGHT: usize = SIZE;
const FREQUENCY: f32 = 3.0;

fn noise_to_png(name: &str, s: impl Sample2) {
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let scalar = 1.0 / WIDTH.max(HEIGHT) as f32;
    let scalar_times_2 = scalar * 2.0;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let x = (x as f32 * scalar_times_2 - 1.0) * FREQUENCY;
        let y = (y as f32 * scalar_times_2 - 1.0) * FREQUENCY;
        let value = s.sample2([x, y]);
        let value = ((value * 0.5 + 0.5) * 255.0) as u8;
        *pixel = Rgb([value, value, value]);
    }

    image.save(format!("example-images/{name}.jpg")).unwrap();
}

fn main() {
    noise_to_png("cell_distance_sq", CellDistanceSq);
    noise_to_png("cell_distance", CellDistance);
    noise_to_png("cell_value", CellValue);
    noise_to_png("perlin", Perlin);
    noise_to_png("open_simplex_2", OpenSimplex2);
    noise_to_png("open_simplex_2s", OpenSimplex2s);
    noise_to_png("value", Value);
    noise_to_png("value_cubic", ValueCubic);

    noise_to_png("fbm", OpenSimplex2.fbm(3, 0.5, 2.0));
    noise_to_png("ridged", OpenSimplex2.ridged(3, 0.5, 2.0));
    noise_to_png("ping_pong", OpenSimplex2.ping_pong(3, 0.5, 2.0, 2.0));

    let warped = NoiseFn(|point: [f32; 2]| {
        let warp_x = OpenSimplex2s.seed(1).sample(point);
        let warp_y = OpenSimplex2s.seed(2).sample(point);
        let warped = [point[0] + warp_x, point[1] + warp_y];
        OpenSimplex2s.sample(warped)
    });

    noise_to_png("warped", warped);

    let warped_fbm = NoiseFn(|point: [f32; 2], seed: i32| {
        let warp_x = OpenSimplex2s.seed(seed + 100).sample(point);
        let warp_y = OpenSimplex2s.seed(seed + 200).sample(point);
        let warped = [point[0] + warp_x, point[1] + warp_y];
        OpenSimplex2s.sample(warped)
    })
    .fbm(3, 0.5, 1.5);

    noise_to_png("warped_fbm", warped_fbm);
}
