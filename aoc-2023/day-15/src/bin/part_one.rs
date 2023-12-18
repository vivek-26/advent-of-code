#[aoc_runner::main(15)]
fn main(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0_usize, |acc, ch| ((acc + ch as usize) * 17) % 256)
}
