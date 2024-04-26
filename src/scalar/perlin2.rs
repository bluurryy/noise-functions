use crate::private_prelude_simd::*;

fn hash_primes2(seed: i32, x: i32, y: i32) -> i32 {
	let mut hash = seed;
	hash ^= x ^ y;
	hash *= 0x27d4eb2d;
	(hash >> 16) ^ hash
}

fn hash_primes3(seed: i32, x: i32, y: i32, z: i32) -> i32 {
	let mut hash = seed;
	hash ^= x ^ y ^ z;
	hash *= 0x27d4eb2d;
	(hash >> 16) ^ hash
}

fn gradient_dot2(hash: i32, x: f32, y: f32) -> f32 {
	// ( 1+R2, 1 ) ( -1-R2, 1 ) ( 1+R2, -1 ) ( -1-R2, -1 )
	// ( 1, 1+R2 ) ( 1, -1-R2 ) ( -1, 1+R2 ) ( -1, -1-R2 )

	let bit1 = hash << 31;
	let bit2 = (hash >> 1) << 31;

	let bit4 = (hash & (1 << 2)) != 0;

	let x = f32::from_bits(x.to_bits() ^ bit1 as u32);
	let y = f32::from_bits(y.to_bits() ^ bit2 as u32);

	let a = if bit4 { y } else { x };
	let b = if bit4 { x } else { y };

	(1.0 + ROOT2) * a + b
}

fn gradient_dot3(hash: i32, x: f32, y: f32, z: f32) -> f32 {
	let hasha13 = hash & 13;

	// if h < 8 then x, else y
	let u = if hasha13 < 8 { x } else { y };

	// if h < 4 then y else if h is 12 or 14 then x else z
	let v = if hasha13 == 12 { x } else { z };
	let v = if hasha13 < 2 { y } else { v };

	// if h1 then -u else u
	// if h2 then -v else v
	let h1 = (hash << 31) as f32;
	let h2 = ((hash & 2) << 30) as f32;

	// then add them
	f32::from_bits(u.to_bits() ^ h1.to_bits()) + f32::from_bits(v.to_bits() ^ h2.to_bits())
}

mod primes {
	pub const X: i32 = 501125321;
	pub const Y: i32 = 1136930381;
	pub const Z: i32 = 1720413743;
}

const ROOT2: f32 = 1.4142135623730950488;
// const ROOT3: f32 = 1.7320508075688772935;

#[inline]
pub fn gen2([x, y]: [f32; 2], seed: i32) -> f32 {
	let xs = x.floor();
	let ys = y.floor();

	let x0 = (xs as i32) * primes::X;
	let y0 = (ys as i32) * primes::Y;

	let x1 = x0 + primes::X;
	let y1 = y0 + primes::Y;

	let xf0 = x - xs;
	let yf0 = y - ys;

	let xf1 = xf0 - 1.0;
	let yf1 = yf0 - 1.0;

	let xs = interp_quintic(xf0);
	let ys = interp_quintic(yf0);

	0.579106986522674560546875
		* lerp(
			lerp(gradient_dot2(hash_primes2(seed, x0, y0), xf0, yf0), gradient_dot2(hash_primes2(seed, x1, y0), xf1, yf0), xs),
			lerp(gradient_dot2(hash_primes2(seed, x0, y1), xf0, yf1), gradient_dot2(hash_primes2(seed, x1, y1), xf1, yf1), xs),
			ys,
		)
}

#[inline]
pub fn gen3([x, y, z]: [f32; 3], seed: i32) -> f32 {
	let xs = x.floor();
	let ys = y.floor();
	let zs = z.floor();

	let x0 = xs as i32 * primes::X;
	let y0 = ys as i32 * primes::Y;
	let z0 = zs as i32 * primes::Z;

	let x1 = x0 + primes::X;
	let y1 = y0 + primes::Y;
	let z1 = z0 + primes::Z;

	let xf0 = x - xs;
	let yf0 = y - ys;
	let zf0 = z - zs;

	let xf1 = xf0 - 1.0;
	let yf1 = yf0 - 1.0;
	let zf1 = zf0 - 1.0;

	let xs = interp_quintic(xf0);
	let ys = interp_quintic(yf0);
	let zs = interp_quintic(zf0);

	0.964921414852142333984375
		* lerp(
			lerp(
				lerp(gradient_dot3(hash_primes3(seed, x0, y0, z0), xf0, yf0, zf0), gradient_dot3(hash_primes3(seed, x1, y0, z0), xf1, yf0, zf0), xs),
				lerp(gradient_dot3(hash_primes3(seed, x0, y1, z0), xf0, yf1, zf0), gradient_dot3(hash_primes3(seed, x1, y1, z0), xf1, yf1, zf0), xs),
				ys,
			),
			lerp(
				lerp(gradient_dot3(hash_primes3(seed, x0, y0, z1), xf0, yf0, zf1), gradient_dot3(hash_primes3(seed, x1, y0, z1), xf1, yf0, zf1), xs),
				lerp(gradient_dot3(hash_primes3(seed, x0, y1, z1), xf0, yf1, zf1), gradient_dot3(hash_primes3(seed, x1, y1, z1), xf1, yf1, zf1), xs),
				ys,
			),
			zs,
		)
}
