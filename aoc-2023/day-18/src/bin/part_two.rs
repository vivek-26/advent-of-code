#[aoc_runner::main(18)]
fn main(input: &str) -> isize {
    let instructions = input.split('\n').map(|line| {
        let (_, color) = line.split_once(" (#").unwrap();
        let direction = match color.as_bytes()[color.len() - 2] {
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            b'3' => b'U',
            _ => unreachable!(),
        };
        (
            direction,
            isize::from_str_radix(&color[0..color.len() - 2], 16).unwrap(),
        )
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
