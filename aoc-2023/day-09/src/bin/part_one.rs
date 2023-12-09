fn main() {
    let lines = aoc::read_input_lines(9);
    let mut histories: Vec<Vec<isize>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse().ok())
                .collect()
        })
        .collect();

    for history in &mut histories {
        let steps = history.len() - 2;
        for step in 0..steps {
            for idx in 0..history.len() - step - 1 {
                history[idx] = history[idx + 1] - history[idx];
            }
        }
    }

    let answer: isize = histories.iter().flat_map(|history| history.iter()).sum();
    println!("{answer}");
}
