use image::{ImageBuffer, Rgb};
use noise_functions::*;

const SIZE: usize = 128;
const WIDTH: usize = SIZE;
const HEIGHT: usize = SIZE;
const FREQUENCY: f32 = 3.0;

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

fn noise_to_image_tileable(noise: impl Sample2) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let scalar = 1.0 / WIDTH.max(HEIGHT) as f32;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let x = (x as f32 * scalar) * FREQUENCY;
        let y = (y as f32 * scalar) * FREQUENCY;
        let value = noise.sample2([x, y]);
        let value = ((value * 0.5 + 0.5) * 255.0) as u8;
        *pixel = Rgb([value, value, value]);
    }

    image
}

fn save_jpg(name: &str, noise: impl Sample2) {
    noise_to_image(noise).save(format!("example-images/{name}.jpg")).unwrap();
}

fn save_jpg_tileable(name: &str, noise: impl Sample2) {
    noise_to_image_tileable(noise).save(format!("example-images/{name}.jpg")).unwrap();
}

fn main() {
    save_jpg("cell_distance_sq", FastCellDistanceSq::default());
    save_jpg("cell_distance", FastCellDistance::default());
    save_jpg("cell_value", FastCellValue::default());
    save_jpg("perlin", Perlin);
    save_jpg("open_simplex_2", OpenSimplex2);
    save_jpg("open_simplex_2s", OpenSimplex2s);
    save_jpg("value", Value);
    save_jpg("value_cubic", ValueCubic);

    save_jpg("fbm", OpenSimplex2.fbm(3, 0.5, 2.0));
    save_jpg("ridged", OpenSimplex2.ridged(3, 0.5, 2.0));
    save_jpg("ping_pong", OpenSimplex2.ping_pong(3, 0.5, 2.0, 2.0));

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

    save_jpg_tileable("tileable_perlin", Perlin.tileable(3.0, 3.0).frequency(2.0));

    save_jpg_tileable("tileable_value", Value.seed(12).tileable(3.0, 3.0).frequency(2.0));

    save_jpg_tileable("tileable_cell_value", CellValue::default().seed(12).tileable(2.15, 2.15).frequency(2.15 * 2.0 / 3.0));

    save_jpg_tileable(
        "tileable_cell_distance_sq",
        NoiseFn(|point: [f32; 2]| (CellDistance::default().seed(12).tileable(2.0, 2.0).frequency(2.0 * 2.0 / 3.0).sample2(point) - 0.4) * 2.5),
    );
}
