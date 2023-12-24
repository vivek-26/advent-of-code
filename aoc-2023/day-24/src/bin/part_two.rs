use itertools::Itertools;
use z3::ast::{Ast, Int};

#[aoc_runner::main(24)]
fn main(input: &str) -> i64 {
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

fn solve(hailstones: &[((f64, f64, f64), (f64, f64, f64))]) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let s = z3::Solver::new(&ctx);

    let (fx, fy, fz) = (
        Int::new_const(&ctx, "fx"),
        Int::new_const(&ctx, "fy"),
        Int::new_const(&ctx, "fz"),
    );
    let (fdx, fdy, fdz) = (
        Int::new_const(&ctx, "fdx"),
        Int::new_const(&ctx, "fdy"),
        Int::new_const(&ctx, "fdz"),
    );

    let zero = Int::from_i64(&ctx, 0);
    for (i, &((x, y, z), (dx, dy, dz))) in hailstones.iter().enumerate() {
        let (x, y, z) = (
            Int::from_i64(&ctx, x as _),
            Int::from_i64(&ctx, y as _),
            Int::from_i64(&ctx, z as _),
        );
        let (dx, dy, dz) = (
            Int::from_i64(&ctx, dx as _),
            Int::from_i64(&ctx, dy as _),
            Int::from_i64(&ctx, dz as _),
        );

        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }

    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();

    res.as_i64().unwrap()
}
