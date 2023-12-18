use day_12::possible_ways;

#[aoc_runner::main(12)]
fn main(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap());

            possible_ways(spring, counts)
        })
        .sum()
}
