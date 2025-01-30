use crate::{Constant, Noise, OpenSimplex2, Sample, Sample2};

#[test]
fn translate() {
    Sample::<0>::sample_with_seed(&Constant(0.0).translate_x(0.0), [0.0; 0], 0);
    Sample::<0>::sample_with_seed(&Constant(0.0).translate_xy(0.0, 0.0), [0.0; 0], 0);
    Sample::<0>::sample_with_seed(&Constant(0.0).translate_xyz(0.0, 0.0, 0.0), [0.0; 0], 0);
    Sample::<0>::sample_with_seed(&Constant(0.0).translate_xyzw(0.0, 0.0, 0.0, 0.0), [0.0; 0], 0);

    Sample::<1>::sample_with_seed(&Constant(0.0).translate_x(0.0), [0.0; 1], 0);
    Sample::<1>::sample_with_seed(&Constant(0.0).translate_xy(0.0, 0.0), [0.0; 1], 0);
    Sample::<1>::sample_with_seed(&Constant(0.0).translate_xyz(0.0, 0.0, 0.0), [0.0; 1], 0);
    Sample::<1>::sample_with_seed(&Constant(0.0).translate_xyzw(0.0, 0.0, 0.0, 0.0), [0.0; 1], 0);

    Sample::<2>::sample_with_seed(&Constant(0.0).translate_x(0.0), [0.0; 2], 0);
    Sample::<2>::sample_with_seed(&Constant(0.0).translate_xy(0.0, 0.0), [0.0; 2], 0);
    Sample::<2>::sample_with_seed(&Constant(0.0).translate_xyz(0.0, 0.0, 0.0), [0.0; 2], 0);
    Sample::<2>::sample_with_seed(&Constant(0.0).translate_xyzw(0.0, 0.0, 0.0, 0.0), [0.0; 2], 0);

    Sample::<3>::sample_with_seed(&Constant(0.0).translate_x(0.0), [0.0; 3], 0);
    Sample::<3>::sample_with_seed(&Constant(0.0).translate_xy(0.0, 0.0), [0.0; 3], 0);
    Sample::<3>::sample_with_seed(&Constant(0.0).translate_xyz(0.0, 0.0, 0.0), [0.0; 3], 0);
    Sample::<3>::sample_with_seed(&Constant(0.0).translate_xyzw(0.0, 0.0, 0.0, 0.0), [0.0; 3], 0);

    Sample::<4>::sample_with_seed(&Constant(0.0).translate_x(0.0), [0.0; 4], 0);
    Sample::<4>::sample_with_seed(&Constant(0.0).translate_xy(0.0, 0.0), [0.0; 4], 0);
    Sample::<4>::sample_with_seed(&Constant(0.0).translate_xyz(0.0, 0.0, 0.0), [0.0; 4], 0);
    Sample::<4>::sample_with_seed(&Constant(0.0).translate_xyzw(0.0, 0.0, 0.0, 0.0), [0.0; 4], 0);
}

#[test]
fn clamp() {
    assert_eq!(Constant(0.0).clamp(1.0, 2.0).sample2([0.0; 2]), 1.0);
    assert_eq!(Constant(1.5).clamp(1.0, 2.0).sample2([0.0; 2]), 1.5);
    assert_eq!(Constant(3.0).clamp(1.0, 2.0).sample2([0.0; 2]), 2.0);
    assert!(Constant(f32::NAN).clamp(1.0, 2.0).sample2([0.0; 2]).is_nan());

    // these would panic if we used `f32::clamp`, but our clamp must not panic
    Constant(0.0).clamp(2.0, 1.0).sample2([0.0; 2]);
    assert_eq!(Constant(0.0).clamp(f32::NAN, f32::NAN).sample2([0.0; 2]), 0.0);
}

#[test]
fn modifiers_manual_ridged() {
    let base = OpenSimplex2;
    let a = base.ridged();
    let b = base.abs().mul(-2.0).add(1.0);
    assert_eq(a, b);
}

fn assert_eq<A, B>(a: A, b: B)
where
    A: Sample<2>,
    B: Sample<2>,
{
    assert_approx_eq(0.0, a, b);
}

fn assert_approx_eq<A, B>(epsilon: f32, a: A, b: B)
where
    A: Sample<2>,
    B: Sample<2>,
{
    let mut max_error = 0.0f32;

    for x in 0..100 {
        for y in 0..100 {
            let x = x as f32 * 1.34900342;
            let y = y as f32 * 0.93124235;
            let p = [x, y];
            let av = a.sample(p);
            let bv = b.sample(p);
            max_error = max_error.max((av - bv).abs());
        }
    }

    std::println!("max error: {max_error}");

    if max_error > epsilon {
        panic!("error too big!");
    }
}
