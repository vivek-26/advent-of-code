fn main() {
    let input = aoc::read_input(15);
    let answer: usize = input.split(',').map(|step| hash(step) as usize).sum();
    println!("{answer}");
}

fn hash(step: &str) -> u8 {
    let curr = step.chars().fold(0_u16, |mut acc, ch| {
        acc += ch as u16;
        acc *= 17;
        acc %= 256;
        acc
    });

    curr as u8
}
