fn main() {
    let input = aoc::read_input(15);
    let answer: usize = input.split(',').map(hash).sum();
    println!("{answer}");
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0_usize, |acc, ch| ((acc + ch as usize) * 17) % 256)
}
