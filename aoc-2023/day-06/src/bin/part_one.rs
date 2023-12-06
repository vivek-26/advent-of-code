fn main() {
    let lines = aoc::read_input_lines(6);
    let mut lines_iter = lines.iter();

    let recorded_time: Vec<_> = parse_input(lines_iter.next().unwrap());
    println!("recorded time: {:?}", recorded_time);

    let recorded_distance: Vec<_> = parse_input(lines_iter.next().unwrap());
    println!("recorded distance: {:?}", recorded_distance);

    let mut ways = Vec::new();
    for idx in 0..recorded_time.len() {
        let mut beat_record = 0;

        for t in 0..=recorded_time[idx] {
            if t * (recorded_time[idx] - t) > recorded_distance[idx] {
                beat_record += 1;
            }
        }

        ways.push(beat_record);
    }

    let answer = ways.iter().fold(1, |acc, &w| acc * w);
    println!("{answer}");
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
