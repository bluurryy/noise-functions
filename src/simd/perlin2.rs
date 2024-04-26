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
    use super::*;

	pub const X: i32 = 501125321;
	pub const Y: i32 = 1136930381;
	pub const Z: i32 = 1720413743;

	pub const XY: i32x2 = i32x2::from_array([X, Y]);
	pub const XYZ: i32x4 = i32x4::from_array([X, Y, Z, 0]);

}

const ROOT2: f32 = 1.4142135623730950488;
// const ROOT3: f32 = 1.7320508075688772935;

#[inline]
pub fn gen2(v: f32x2, seed: i32) -> f32 {
	let vs = v.floor();
	let v0 = vs.cast() * primes::XY;
	let v1 = v0 + primes::XY;
	let vf0 = v - vs;
	let vf1 = vf0 - splat(1.0);
	let vs = interp_quintic(vf0);

	0.579106986522674560546875
		* lerp(
			lerp(gradient_dot2(hash_primes2(seed, v0[0], v0[1]), vf0[0], vf0[1]), gradient_dot2(hash_primes2(seed, v1[0], v0[1]), vf1[0], vf0[1]), vs[0]),
			lerp(gradient_dot2(hash_primes2(seed, v0[0], v1[1]), vf0[0], vf1[1]), gradient_dot2(hash_primes2(seed, v1[0], v1[1]), vf1[0], vf1[1]), vs[0]),
			vs[1],
		)
}

#[inline]
pub fn gen3(v: f32x4, seed: i32) -> f32 {
	let vs = v.floor();
	let v0 = vs.cast() * primes::XYZ;
	let v1 = v0 + primes::XYZ;
	let vf0 = v - vs;
	let vf1 = vf0 - splat(1.0);
	let vs = interp_quintic(vf0);

	0.964921414852142333984375
		* lerp(
			lerp(
				lerp(gradient_dot3(hash_primes3(seed, v0[0], v0[1], v0[2]), vf0[0], vf0[1], vf0[2]), gradient_dot3(hash_primes3(seed, v1[0], v0[1], v0[2]), vf1[0], vf0[1], vf0[2]), vs[0]),
				lerp(gradient_dot3(hash_primes3(seed, v0[0], v1[1], v0[2]), vf0[0], vf1[1], vf0[2]), gradient_dot3(hash_primes3(seed, v1[0], v1[1], v0[2]), vf1[0], vf1[1], vf0[2]), vs[0]),
				vs[1],			),
			lerp(
				lerp(gradient_dot3(hash_primes3(seed, v0[0], v0[1], v1[2]), vf0[0], vf0[1], vf1[2]), gradient_dot3(hash_primes3(seed, v1[0], v0[1], v1[2]), vf1[0], vf0[1], vf1[2]), vs[0]),
				lerp(gradient_dot3(hash_primes3(seed, v0[0], v1[1], v1[2]), vf0[0], vf1[1], vf1[2]), gradient_dot3(hash_primes3(seed, v1[0], v1[1], v1[2]), vf1[0], vf1[1], vf1[2]), vs[0]),
				vs[1],
			),
			vs[2],
		)
}
