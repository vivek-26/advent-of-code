use itertools::Itertools;
use z3::ast::{Ast, Int, Real};

#[aoc_runner::main(24)]
fn main(input: &str) -> usize {
    let hailstones = input
        .split('\n')
        .map(|line| {
            let (a, b) = line.split_once(" @ ").unwrap();
            let (x, y, z) = a
                .split(", ")
                .map(|w| w.parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (dx, dy, dz) = b
                .split(", ")
                .map(|w| w.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            ((x, y, z), (dx, dy, dz))
        })
        .collect::<Vec<_>>();

    solve(&hailstones)
}

fn solve(hailstones: &[((f64, f64, f64), (f64, f64, f64))]) -> usize {
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Real::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0).to_real();
    for (i, &((x, y, z), (dx, dy, dz))) in hailstones.iter().enumerate() {
        let [x, y, z, dx, dy, dz] =
            [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _).to_real());

        let t = Real::new_const(&ctx, format!("t{i}"));

        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }

    assert_eq!(s.check(), z3::SatResult::Sat);

    let res = s
        .get_model()
        .unwrap()
        .eval(&(&fx + &fy + &fz), true)
        .unwrap();

    res.to_string().strip_suffix(".0").unwrap().parse().unwrap()
}
