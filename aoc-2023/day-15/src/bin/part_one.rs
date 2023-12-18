#[aoc_runner::main]
fn main() -> usize {
    let input = aoc::read_input(15);
    input.split(',').map(hash).sum()
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0_usize, |acc, ch| ((acc + ch as usize) * 17) % 256)
}
