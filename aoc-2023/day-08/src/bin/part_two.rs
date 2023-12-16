use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Direction<'a> {
    left: &'a str,
    right: &'a str,
}

#[aoc_runner::timeit]
fn main() -> usize {
    let lines = aoc::read_input_lines(8);

    let instructions: Vec<_> = lines[0]
        .chars()
        .map(|ch| match ch {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("invalid instruction"),
        })
        .collect();

    let mut network = HashMap::new();
    lines[2..].iter().for_each(|line| {
        let mut node_parts = line.splitn(2, '=');
        let node = node_parts.next().unwrap().trim();

        let mut connected_to = node_parts
            .next()
            .unwrap()
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',');

        let node_left = connected_to.next().unwrap().trim();
        let node_right = connected_to.next().unwrap().trim();

        network.insert(
            node,
            Direction {
                left: node_left,
                right: node_right,
            },
        );
    });

    let mut answer = 1_usize;
    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| {
            let mut curr_steps = 0_usize;
            let mut curr_node = *node;
            let mut idx = 0_usize;

            while !curr_node.ends_with('Z') {
                curr_steps += 1;
                match instructions[idx % instructions.len()] {
                    Instruction::Left => curr_node = network.get(curr_node).unwrap().left,
                    Instruction::Right => curr_node = network.get(curr_node).unwrap().right,
                }

                idx += 1;
            }

            curr_steps
        })
        .for_each(|steps| answer = num::integer::lcm(steps, answer));

    answer
}
