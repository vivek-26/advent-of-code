#[aoc_runner::main(6)]
fn main(input: &str) -> usize {
    let mut lines_iter = input.split('\n');

    let record_time: Vec<_> = parse_input(lines_iter.next().unwrap());
    let record_distance: Vec<_> = parse_input(lines_iter.next().unwrap());

    let mut ways = Vec::new();
    for idx in 0..record_time.len() {
        let mut beats_record = 0;

        for time in 0..=record_time[idx] / 2 {
            if time * (record_time[idx] - time) > record_distance[idx] {
                beats_record += 2;
            }
        }

        beats_record -= if record_time[idx] % 2 == 0 { 1 } else { 0 };
        ways.push(beats_record);
    }

    ways.iter().product()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap_or(0))
        .collect()
}
