use itertools::Itertools;

use day_12::possible_ways;

#[aoc_runner::main]
fn main() -> usize {
    let lines = aoc::read_input(12);
    lines
        .split('\n')
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();

            let spring = std::iter::once(spring).cycle().take(5).join("?");

            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect_vec();
            let n = counts.len();

            possible_ways(&spring, counts.into_iter().cycle().take(5 * n))
        })
        .sum()
}
