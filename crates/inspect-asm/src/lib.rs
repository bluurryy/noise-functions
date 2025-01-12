#![cfg_attr(feature = "nightly-simd", feature(portable_simd))]

use noise_functions::*;

#[cfg(feature = "nightly-simd")]
use std::simd::prelude::*;

macro_rules! noises {
	(
		$(
			for $noise:ident do
				$d2:ident,
				$d2_simd:ident,
				$d3:ident,
				$d3_simd:ident
				$(
					,$d4:ident
					,$d4_simd:ident
				)?
		)*
	) => {
		$(
			pub fn $d2(point: [f32; 2]) -> f32 {
				$noise.sample2(point)
			}

			#[cfg(feature = "nightly-simd")]
			pub fn $d2_simd(point: f32x2) -> f32 {
				$noise.sample2a(point)
			}

			pub fn $d3(point: [f32; 3]) -> f32 {
				$noise.sample3(point)
			}

			#[cfg(feature = "nightly-simd")]
			pub fn $d3_simd(point: f32x4) -> f32 {
				$noise.sample3a(point)
			}

			$(
				pub fn $d4(point: [f32; 4]) -> f32 {
					$noise.sample4(point)
				}

				#[cfg(feature = "nightly-simd")]
				pub fn $d4_simd(point: f32x4) -> f32 {
					$noise.sample4a(point)
				}
			)?
		)*
	};
}

noises! {
    for CellDistanceSq do
        cell_distance_sq_2d,
        cell_distance_sq_2d_simd,
        cell_distance_sq_3d,
        cell_distance_sq_3d_simd

    for CellDistance do
        cell_distance_2d,
        cell_distance_2d_simd,
        cell_distance_3d,
        cell_distance_3d_simd

    for CellValue do
        cell_value_2d,
        cell_value_2d_simd,
        cell_value_3d,
        cell_value_3d_simd

    for OpenSimplex2 do
        open_simplex2_2d,
        open_simplex2_2d_simd,
        open_simplex2_3d,
        open_simplex2_3d_simd

    for OpenSimplex2s do
        open_simplex2s_2d,
        open_simplex2s_2d_simd,
        open_simplex2s_3d,
        open_simplex2s_3d_simd

    for Perlin do
        perlin_2d,
        perlin_2d_simd,
        perlin_3d,
        perlin_3d_simd

    for ValueCubic do
        value_cubic_2d,
        value_cubic_2d_simd,
        value_cubic_3d,
        value_cubic_3d_simd

    for Value do
        value_2d,
        value_2d_simd,
        value_3d,
        value_3d_simd
}
