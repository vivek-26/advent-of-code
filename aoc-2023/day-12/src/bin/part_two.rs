use itertools::Itertools;

use day_12::possible_ways;

fn main() {
    let lines = aoc::read_input_lines(12);
    let answer = lines
        .iter()
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
        .sum::<usize>();

    println!("{answer}");
}
