use crate::{Noise, OpenSimplex2, Sample};

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
