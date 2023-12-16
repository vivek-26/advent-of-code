use std::collections::HashSet;

#[aoc_runner::timeit]
fn main() -> u32 {
    let mut sum = 0_u32;

    for line in aoc::read_input_lines(4) {
        let numbers: Vec<&str> = line.split(':').nth(1).unwrap().split('|').collect();

        let winning_numbers: HashSet<u32> = numbers[0]
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();

        let matches = numbers[1]
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .filter(|n| winning_numbers.contains(n))
            .count();

        if let Some(matches) = matches.checked_sub(1) {
            sum += u32::pow(2, matches as u32);
        }
    }

    sum
}
