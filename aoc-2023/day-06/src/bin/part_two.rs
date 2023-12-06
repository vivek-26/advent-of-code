fn main() {
    let lines = aoc::read_input_lines(6);
    let mut lines_iter = lines.iter();

    let recorded_time = parse_input(lines_iter.next().unwrap());
    let recorded_distance = parse_input(lines_iter.next().unwrap());

    let mut beat_record = 0;
    for t in 0..=recorded_time {
        if t * (recorded_time - t) > recorded_distance {
            beat_record += 1;
        }
    }

    println!("{}", beat_record);
}

fn parse_input(input: &str) -> usize {
    input
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}
