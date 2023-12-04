fn main() {
    let lines = aoc::read_input_lines(1);
    let mut sum = 0;

    for line in lines {
        let line = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");

        let mut left_digit_iter = line.chars().filter_map(|c| c.to_digit(10));
        let mut right_digit_iter = line.chars().rev().filter_map(|c| c.to_digit(10));

        let num_one = left_digit_iter.next().unwrap_or(0);
        let num_two = right_digit_iter.next().unwrap_or(0);
        sum += num_one * 10 + num_two;
    }

    println!("{}", sum);
}
