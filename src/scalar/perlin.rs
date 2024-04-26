use crate::private_prelude::*;

#[rustfmt::skip]
	#[inline]
	pub(crate) fn gen2([x, y]: [f32; 2], seed: i32) -> f32 {
		let mut x0: i32 = floor_to_int(x);
		let mut y0: i32 = floor_to_int(y);

		let xd0: f32 = x - x0 as f32;
		let yd0: f32 = y - y0 as f32;

		let xd1: f32 = xd0 - 1.0;
		let yd1: f32 = yd0 - 1.0;

		let xs: f32 = interp_quintic(xd0);
		let ys: f32 = interp_quintic(yd0);

		x0 = x0.wrapping_mul(PRIME_X);
		y0 = y0.wrapping_mul(PRIME_Y);

		let x1: i32 = x0.wrapping_add(PRIME_X);
		let y1: i32 = y0.wrapping_add(PRIME_Y);

		let xf0: f32 = lerp(
			grad2(seed, x0, y0, xd0, yd0),
			grad2(seed, x1, y0, xd1, yd0),
			xs,
		);
		let xf1: f32 = lerp(
			grad2(seed, x0, y1, xd0, yd1),
			grad2(seed, x1, y1, xd1, yd1),
			xs,
		);

		lerp(xf0, xf1, ys) * 1.4247691104677813
	}

#[rustfmt::skip]
	#[inline]
	pub(crate) fn gen3([x, y, z]: [f32;3], seed: i32) -> f32 {
		let mut x0: i32 = floor_to_int(x);
		let mut y0: i32 = floor_to_int(y);
		let mut z0: i32 = floor_to_int(z);

		let xd0: f32 = x - x0 as f32;
		let yd0: f32 = y - y0 as f32;
		let zd0: f32 = z - z0 as f32;

		let xd1: f32 = xd0 - 1.0;
		let yd1: f32 = yd0 - 1.0;
		let zd1: f32 = zd0 - 1.0;

		let xs: f32 = interp_quintic(xd0);
		let ys: f32 = interp_quintic(yd0);
		let zs: f32 = interp_quintic(zd0);

		x0 = x0.wrapping_mul(PRIME_X);
		y0 = y0.wrapping_mul(PRIME_Y);
		z0 = z0.wrapping_mul(PRIME_Z);

		let x1: i32 = x0.wrapping_add(PRIME_X);
		let y1: i32 = y0.wrapping_add(PRIME_Y);
		let z1: i32 = z0.wrapping_add(PRIME_Z);

		let xf00: f32 = lerp(
			grad3(seed, x0, y0, z0, xd0, yd0, zd0),
			grad3(seed, x1, y0, z0, xd1, yd0, zd0),
			xs,
		);
		let xf10: f32 = lerp(
			grad3(seed, x0, y1, z0, xd0, yd1, zd0),
			grad3(seed, x1, y1, z0, xd1, yd1, zd0),
			xs,
		);
		let xf01: f32 = lerp(
			grad3(seed, x0, y0, z1, xd0, yd0, zd1),
			grad3(seed, x1, y0, z1, xd1, yd0, zd1),
			xs,
		);
		let xf11: f32 = lerp(
			grad3(seed, x0, y1, z1, xd0, yd1, zd1),
			grad3(seed, x1, y1, z1, xd1, yd1, zd1),
			xs,
		);

		let yf0: f32 = lerp(xf00, xf10, ys);
		let yf1: f32 = lerp(xf01, xf11, ys);

		lerp(yf0, yf1, zs) * 0.964921414852142333984375
	}
