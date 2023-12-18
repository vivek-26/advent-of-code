use day_12::possible_ways;

#[aoc_runner::main]
fn main() -> usize {
    let lines = aoc::read_input(12);
    lines
        .split('\n')
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap());

            possible_ways(spring, counts)
        })
        .sum()
}
