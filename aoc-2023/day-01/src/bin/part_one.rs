use aoc;

fn main() {
    let lines = aoc::read_input_lines(1);
    let mut sum = 0;

    for line in lines {
        let mut left_digit_iter = line.chars().filter_map(|c| c.to_digit(10));
        let mut right_digit_iter = line.chars().rev().filter_map(|c| c.to_digit(10));

        let num_one = left_digit_iter.next().unwrap_or(0);
        let num_two = right_digit_iter.next().unwrap_or(0);
        sum += num_one * 10 + num_two;
    }

    println!("{}", sum);
}
