use itertools::Itertools;
use std::collections::HashMap;

type WorkFlows<'a> = HashMap<&'a str, (Vec<(char, bool, usize, &'a str)>, &'a str)>;

#[aoc_runner::main(19)]
fn main(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .split('\n')
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let (rest, label) = rest[0..rest.len() - 1].split_at(rest.rfind(',').unwrap());
            let rules = rest
                .split(',')
                .map(|rule| {
                    let (rest, label) = rule.split_once(':').unwrap();
                    let lt = rest.contains('<');
                    let (name, n) = rest.split_once(if lt { '<' } else { '>' }).unwrap();
                    (
                        name.as_bytes()[0] as char,
                        lt,
                        n.parse::<usize>().unwrap(),
                        label,
                    )
                })
                .collect::<Vec<_>>();
            (name, (rules, &label[1..]))
        })
        .collect::<HashMap<_, _>>();

    parts
        .split('\n')
        .map(|l| {
            l.split(|c: char| !c.is_ascii_digit())
                .filter(|l| !l.is_empty())
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .filter(|&(x, m, a, s)| process_parts(&workflows, [x, m, a, s]))
        .map(|(x, m, a, s)| x + m + a + s)
        .sum()
}

fn process_parts(workflows: &WorkFlows<'_>, vals: [usize; 4]) -> bool {
    let mut curr = "in";
    while curr != "A" && curr != "R" {
        let workflow = &workflows[curr];
        curr = workflow
            .0
            .iter()
            .find(|&&(p, lt, n, _)| {
                let i = "xmas".chars().position(|c| c == p).unwrap();
                if lt {
                    vals[i] < n
                } else {
                    vals[i] > n
                }
            })
            .map(|&(_, _, _, label)| label)
            .unwrap_or(workflow.1);
    }

    curr == "A"
}
