use crate::{from_fast_noise_2::reciprocal, max, min, mul_add};

use super::{
    cell::{calc_distance2, calc_distance3, calc_distance4, CellIndex, DistanceFn, DistanceReturnType, JITTER_2D, JITTER_3D, JITTER_4D, MAX_DISTANCE_COUNT},
    hash_primes2_hb, hash_primes3_hb, hash_primes4_hb, impl_noise234, inv_sqrt, primes,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellDistance {
    pub jitter: f32,
    pub distance_fn: DistanceFn,
    pub distance_indices: [CellIndex; 2],
    pub return_type: DistanceReturnType,
}

impl CellDistance {
    pub const fn jitter(mut self, jitter: f32) -> Self {
        self.jitter = jitter;
        self
    }

    pub const fn distance_fn(mut self, distance_fn: DistanceFn) -> Self {
        self.distance_fn = distance_fn;
        self
    }

    pub const fn distance_indices(mut self, distance_indices: [CellIndex; 2]) -> Self {
        self.distance_indices = distance_indices;
        self
    }

    pub const fn return_type(mut self, return_type: DistanceReturnType) -> Self {
        self.return_type = return_type;
        self
    }
}

impl_noise234!(CellDistance);

impl Default for CellDistance {
    fn default() -> Self {
        Self {
            jitter: 1.0,
            distance_fn: Default::default(),
            distance_indices: Default::default(),
            return_type: Default::default(),
        }
    }
}

fn gen2([x, y]: [f32; 2], seed: i32, settings: CellDistance) -> f32 {
    let jitter = settings.jitter * JITTER_2D;

    let mut distance = [f32::INFINITY; MAX_DISTANCE_COUNT];

    let mut xc = (x as i32).wrapping_sub(1);
    let mut yc_base = (y as i32).wrapping_sub(1);

    let mut xcf = (xc as f32) - x;
    let ycf_base = (yc_base as f32) - y;

    xc = xc.wrapping_mul(primes::X);
    yc_base = yc_base.wrapping_mul(primes::Y);

    for _xi in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;

        for _yi in 0..3 {
            let hash = hash_primes2_hb(seed, xc, yc);
            let mut xd = ((hash & 0xffff) as f32) - (0xffff as f32 / 2.0);
            let mut yd = (((hash >> 16) & 0xffff) as f32) - (0xffff as f32 / 2.0);

            let inv_mag = jitter * inv_sqrt(mul_add(xd, xd, yd * yd));
            xd = mul_add(xd, inv_mag, xcf);
            yd = mul_add(yd, inv_mag, ycf);

            let new_distance = calc_distance2(settings.distance_fn, xd, yd);

            let mut i = MAX_DISTANCE_COUNT - 1;

            loop {
                distance[i] = max(min(distance[i], new_distance), distance[i.wrapping_sub(1)]);

                i = i.wrapping_sub(1);

                if i == 0 {
                    break;
                }
            }

            distance[0] = min(distance[0], new_distance);

            ycf += 1.0;
            yc = yc.wrapping_add(primes::Y);
        }

        xcf += 1.0;
        xc = xc.wrapping_add(primes::X);
    }

    get_return(&mut distance, &settings)
}

fn gen3([x, y, z]: [f32; 3], seed: i32, settings: CellDistance) -> f32 {
    let jitter = settings.jitter * JITTER_3D;

    let mut distance = [f32::INFINITY; MAX_DISTANCE_COUNT];

    let mut xc = (x as i32).wrapping_sub(1);
    let mut yc_base = (y as i32).wrapping_sub(1);
    let mut zc_base = (z as i32).wrapping_sub(1);

    let mut xcf = (xc as f32) - x;
    let ycf_base = (yc_base as f32) - y;
    let zcf_base = (zc_base as f32) - z;

    xc = xc.wrapping_mul(primes::X);
    yc_base = yc_base.wrapping_mul(primes::Y);
    zc_base = zc_base.wrapping_mul(primes::Z);

    for _xi in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;

        for _yi in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;

            for _zi in 0..3 {
                let hash = hash_primes3_hb(seed, xc, yc, zc);
                let mut xd = ((hash & 0x3ff) as f32) - (0x3ff as f32 / 2.0);
                let mut yd = (((hash >> 10) & 0x3ff) as f32) - (0x3ff as f32 / 2.0);
                let mut zd = (((hash >> 20) & 0x3ff) as f32) - (0x3ff as f32 / 2.0);

                let inv_mag = jitter * inv_sqrt(mul_add(xd, xd, mul_add(yd, yd, zd * zd)));
                xd = mul_add(xd, inv_mag, xcf);
                yd = mul_add(yd, inv_mag, ycf);
                zd = mul_add(zd, inv_mag, zcf);

                let new_distance = calc_distance3(settings.distance_fn, xd, yd, zd);

                let mut i = MAX_DISTANCE_COUNT - 1;

                loop {
                    distance[i] = max(min(distance[i], new_distance), distance[i.wrapping_sub(1)]);

                    i = i.wrapping_sub(1);

                    if i == 0 {
                        break;
                    }
                }

                distance[0] = min(distance[0], new_distance);

                zcf += 1.0;
                zc = zc.wrapping_add(primes::Z);
            }

            ycf += 1.0;
            yc = yc.wrapping_add(primes::Y);
        }

        xcf += 1.0;
        xc = xc.wrapping_add(primes::X);
    }

    get_return(&mut distance, &settings)
}

fn gen4([x, y, z, w]: [f32; 4], seed: i32, settings: CellDistance) -> f32 {
    let jitter = settings.jitter * JITTER_4D;

    let mut distance = [f32::INFINITY; MAX_DISTANCE_COUNT];

    let mut xc = (x as i32).wrapping_sub(1);
    let mut yc_base = (y as i32).wrapping_sub(1);
    let mut zc_base = (z as i32).wrapping_sub(1);
    let mut wc_base = (w as i32).wrapping_sub(1);

    let mut xcf = (xc as f32) - x;
    let ycf_base = (yc_base as f32) - y;
    let zcf_base = (zc_base as f32) - z;
    let wcf_base = (wc_base as f32) - w;

    xc = xc.wrapping_mul(primes::X);
    yc_base = yc_base.wrapping_mul(primes::Y);
    zc_base = zc_base.wrapping_mul(primes::Z);
    wc_base = wc_base.wrapping_mul(primes::W);

    for _xi in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;

        for _yi in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;

            for _zi in 0..3 {
                let mut wcf = wcf_base;
                let mut wc = wc_base;

                for _wi in 0..3 {
                    let hash = hash_primes4_hb(seed, xc, yc, zc, wc);
                    let mut xd = ((hash & 0xff) as f32) - (0xff as f32 / 2.0);
                    let mut yd = (((hash >> 8) & 0xff) as f32) - (0xff as f32 / 2.0);
                    let mut zd = (((hash >> 16) & 0xff) as f32) - (0xff as f32 / 2.0);
                    let mut wd = (((hash >> 24) & 0xff) as f32) - (0xff as f32 / 2.0);

                    let inv_mag = jitter * inv_sqrt(mul_add(xd, xd, mul_add(yd, yd, mul_add(zd, zd, wd * wd))));
                    xd = mul_add(xd, inv_mag, xcf);
                    yd = mul_add(yd, inv_mag, ycf);
                    zd = mul_add(zd, inv_mag, zcf);
                    wd = mul_add(wd, inv_mag, wcf);

                    let new_distance = calc_distance4(settings.distance_fn, xd, yd, zd, wd);

                    let mut i = MAX_DISTANCE_COUNT - 1;

                    loop {
                        distance[i] = max(min(distance[i], new_distance), distance[i.wrapping_sub(1)]);

                        i = i.wrapping_sub(1);

                        if i == 0 {
                            break;
                        }
                    }

                    distance[0] = min(distance[0], new_distance);

                    wcf += 1.0;
                    wc = wc.wrapping_add(primes::W);
                }

                zcf += 1.0;
                zc = zc.wrapping_add(primes::Z);
            }

            ycf += 1.0;
            yc = yc.wrapping_add(primes::Y);
        }

        xcf += 1.0;
        xc = xc.wrapping_add(primes::X);
    }

    get_return(&mut distance, &settings)
}

fn get_return(distance: &mut [f32; MAX_DISTANCE_COUNT], settings: &CellDistance) -> f32 {
    let i0 = settings.distance_indices[0] as usize;
    let i1 = settings.distance_indices[1] as usize;

    if settings.distance_fn == DistanceFn::Euclidean {
        distance[i0] *= inv_sqrt(distance[i0]);
        distance[i1] *= inv_sqrt(distance[i1]);
    }

    match settings.return_type {
        DistanceReturnType::Index0 => distance[i0],
        DistanceReturnType::Index0Add1 => distance[i0] + distance[i1],
        DistanceReturnType::Index0Sub1 => distance[i0] - distance[i1],
        DistanceReturnType::Index0Mul1 => distance[i0] * distance[i1],
        DistanceReturnType::Index0Div1 => distance[i0] * reciprocal(distance[i1]),
    }
}
