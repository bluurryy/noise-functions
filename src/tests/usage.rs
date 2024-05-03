use crate::cellular::Jitter;
use crate::*;

#[allow(dead_code)]
const fn can_sample2<S>()
where
    S: Sample<2>,
{
}

#[allow(dead_code)]
const fn can_sample2a<S>()
where
    S: Sample<2, f32x2>,
{
}

#[allow(dead_code)]
const fn can_sample3<S>()
where
    S: Sample<3>,
{
}

#[allow(dead_code)]
const fn can_sample3a<S>()
where
    S: Sample<3, f32x4>,
{
}

#[allow(dead_code)]
const fn can_sample<S>()
where
    S: Sample<2> + Sample<3> + Sample<2, f32x2> + Sample<3, f32x4>,
{
}

macro_rules! assert_noises {
    ($($noise:ty)*) => {
        $(
            can_sample::<$noise>();
            can_sample::<Seeded<$noise>>();
            can_sample::<Fbm<$noise>>();
            can_sample::<Weighted<Fbm<$noise>>>();
            can_sample::<Ridged<$noise>>();
            can_sample::<Weighted<Ridged<$noise>>>();
            can_sample::<Seeded<Fbm<$noise>>>();
            can_sample::<Seeded<Weighted<Fbm<$noise>>>>();
            can_sample::<Seeded<Ridged<$noise>>>();
            can_sample::<Seeded<Weighted<Ridged<$noise>>>>();
            can_sample::<Seeded<PingPong<$noise>>>();
            can_sample::<Seeded<Weighted<PingPong<$noise>>>>();
        )*
    };
}

const _STATIC_ASSERTS: () = {
    assert_noises! {
        CellDistance
        CellDistanceSq
        CellValue
        OpenSimplex2
        OpenSimplex2s
        Perlin
        Value
        ValueCubic

        Jitter<CellDistance>
        Jitter<CellDistanceSq>
        Jitter<CellValue>
    }
};

const _STATIC_ASSERTS_NOISE_FN: () = {
    can_sample2::<Fbm<NoiseFn<Box<dyn Fn([f32; 2], i32) -> f32>>>>();
    can_sample2a::<Fbm<NoiseFn<Box<dyn Fn(f32x2, i32) -> f32>>>>();
    can_sample3::<Fbm<NoiseFn<Box<dyn Fn([f32; 3], i32) -> f32>>>>();
    can_sample3a::<Fbm<NoiseFn<Box<dyn Fn(f32x4, i32) -> f32>>>>();
};
