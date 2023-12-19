#[aoc_runner::main(18)]
fn main(input: &str) -> isize {
    let instructions = input.split('\n').map(|line| {
        let (len, _) = line[2..].split_once(' ').unwrap();
        (line.as_bytes()[0], len.parse::<isize>().unwrap())
    });

    shoelace_formula(instructions)
}

fn shoelace_formula(instructions: impl Iterator<Item = (u8, isize)>) -> isize {
    let (mut area, mut r, mut c) = (0, 0, 0);
    for (direction, len) in instructions {
        let (rr, cc) = (r, c);
        match direction {
            b'U' => r -= len,
            b'R' => c += len,
            b'D' => r += len,
            b'L' => c -= len,
            _ => unreachable!(),
        };

        area += (c + cc) * (r - rr) + len;
    }

    area / 2 + 1
}
