use std::collections::{HashMap, HashSet};

#[aoc_runner::main]
fn main() -> u32 {
    let mut sum = 0_u32;
    let mut copies = HashMap::new();
    copies.insert(1, 1);

    for (idx, line) in aoc::read_input_lines(4).iter().enumerate() {
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

        let current_multiplier = *copies.entry(idx).or_insert(1);
        sum += current_multiplier;

        for i in 1..matches + 1 {
            *copies.entry(idx + i).or_insert(1) += current_multiplier;
        }
    }

    sum
}
