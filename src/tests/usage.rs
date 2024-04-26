use crate::*;

#[allow(dead_code)]
const fn can_sample<S>()
where
    S: Sample<2> + Sample<3> + Sample<2, f32x2> + Sample<3, f32x4>,
{
}

macro_rules! assert_noises {
    ($($noise:ident)*) => {
        $(
            can_sample::<$noise>();
            can_sample::<Seeded<$noise>>();
            can_sample::<Fbm<$noise>>();
            can_sample::<FbmWeighted<$noise>>();
            can_sample::<Ridged<$noise>>();
            can_sample::<RidgedWeighted<$noise>>();
            can_sample::<Seeded<Fbm<$noise>>>();
            can_sample::<Seeded<FbmWeighted<$noise>>>();
            can_sample::<Seeded<Ridged<$noise>>>();
            can_sample::<Seeded<RidgedWeighted<$noise>>>();
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
    }
};
