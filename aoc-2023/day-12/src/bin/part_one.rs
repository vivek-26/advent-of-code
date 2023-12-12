use day_12::possible_ways;

fn main() {
    let lines = aoc::read_input_lines(12);
    let answer = lines
        .iter()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap());

            possible_ways(spring, counts)
        })
        .sum::<usize>();

    println!("{answer}");
}
