#[aoc_runner::main(9)]
fn main(input: &str) -> isize {
    let mut histories: Vec<Vec<isize>> = input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse().ok())
                .collect()
        })
        .collect();

    for history in &mut histories {
        let steps = history.len() - 2;
        for step in 0..steps {
            for idx in (step + 1..history.len()).rev() {
                history[idx] -= history[idx - 1];
            }
        }
    }

    histories
        .iter()
        .map(|history| history.iter().rev().skip(2).fold(0, |num, &val| val - num))
        .sum()
}
