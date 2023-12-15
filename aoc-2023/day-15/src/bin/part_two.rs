#[derive(Debug)]
enum Op {
    Dash,      // op -
    Equal(u8), // op =, with focal length
}

#[derive(Debug)]
struct Instruction<'a> {
    label: &'a str,
    op: Op,
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

fn main() {
    let input = aoc::read_input(15);
    let instructions = parse_instructions(input.as_str());

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    for instruction in instructions {
        match instruction.op {
            Op::Dash => {
                let box_idx = hash(instruction.label);
                if let Some((idx, _)) = boxes[box_idx]
                    .iter()
                    .enumerate()
                    .find(|(_, lens)| lens.label == instruction.label)
                {
                    boxes[box_idx].remove(idx);
                }
            }
            Op::Equal(focal_length) => {
                let box_idx = hash(instruction.label);
                if let Some((idx, _)) = boxes[box_idx]
                    .iter()
                    .enumerate()
                    .find(|(_, lens)| lens.label == instruction.label)
                {
                    boxes[box_idx][idx].focal_length = focal_length;
                } else {
                    boxes[box_idx].push(Lens {
                        label: instruction.label,
                        focal_length,
                    })
                }
            }
        }
    }

    let answer = boxes
        .iter()
        .enumerate()
        .map(|(bx_idx, bx)| {
            bx.iter()
                .enumerate()
                .map(|(l_idx, lens)| (bx_idx + 1) * (l_idx + 1) * (lens.focal_length as usize))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{answer}");
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|inst| match inst.chars().last().unwrap() {
            '-' => Instruction {
                label: inst.get(..inst.len() - 1).unwrap(),
                op: Op::Dash,
            },
            focal_length => Instruction {
                label: inst.get(..inst.len() - 2).unwrap(),
                op: Op::Equal(focal_length.to_digit(10).unwrap() as u8),
            },
        })
        .collect()
}

fn hash(step: &str) -> usize {
    let curr = step.chars().fold(0_u16, |mut acc, ch| {
        acc += ch as u16;
        acc *= 17;
        acc %= 256;
        acc
    });

    curr as usize
}
