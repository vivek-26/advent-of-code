#[aoc_runner::main(6)]
fn main(input: &str) -> i32 {
    let mut lines_iter = input.split('\n');

    let record_time = parse_input(lines_iter.next().unwrap());
    let record_distance = parse_input(lines_iter.next().unwrap());

    let mut beats_record = 0;
    for t in 0..=record_time / 2 {
        if t * (record_time - t) > record_distance {
            beats_record += 2;
        }
    }

    beats_record -= if record_time % 2 == 0 { 1 } else { 0 };

    beats_record
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
