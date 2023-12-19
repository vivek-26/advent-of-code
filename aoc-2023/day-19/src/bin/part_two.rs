use std::collections::HashMap;

type WorkFlows<'a> = HashMap<&'a str, (Vec<(char, bool, usize, &'a str)>, &'a str)>;

#[aoc_runner::main(19)]
fn main(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();
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

    count_accepted(
        &workflows,
        "in",
        std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>()),
    )
}

fn count_accepted(workflows: &WorkFlows<'_>, curr: &str, mut ranges: [Vec<usize>; 4]) -> usize {
    if curr == "A" {
        return ranges.iter().map(|v| v.len()).product();
    }
    if curr == "R" {
        return 0;
    }

    let mut ans = 0;
    let workflow = &workflows[curr];

    for &(p, lt, n, label) in &workflow.0 {
        let i = "xmas".chars().position(|c| c == p).unwrap();
        let mut newranges = ranges.clone();
        (newranges[i], ranges[i]) = ranges[i]
            .iter()
            .partition(|&&val| if lt { val < n } else { val > n });
        ans += count_accepted(workflows, label, newranges);
    }

    ans += count_accepted(workflows, workflow.1, ranges);
    ans
}
