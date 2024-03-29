use std::collections::{HashMap, VecDeque};

enum Node<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster,
}

#[aoc_runner::main(20)]
fn main(input: &str) -> usize {
    let mut g = HashMap::new();
    let mut state = HashMap::new();
    for line in input.split('\n') {
        let (src, rest) = line.split_once(" -> ").unwrap();
        let connections = rest.split(", ").collect::<Vec<_>>();
        let (node, state_type) = match src.as_bytes()[0] as char {
            '%' => (&src[1..], Node::FlipFlop(false)),
            '&' => (&src[1..], Node::Conjunction(HashMap::new())),
            'b' => (src, Node::Broadcaster),
            _ => unreachable!(),
        };

        g.insert(node, connections);
        state.insert(node, state_type);
    }

    let mut rx_conjunction = "";
    for (&node, connections) in &g {
        for &n in connections {
            match state.get_mut(n) {
                Some(Node::Conjunction(m)) => {
                    m.insert(node, false);
                }
                Some(_) => {}
                None => rx_conjunction = node,
            }
        }
    }

    let mut cycles = match &state[rx_conjunction] {
        Node::Conjunction(m) => m
            .iter()
            .map(|(&node, _)| (node, None))
            .collect::<HashMap<_, _>>(),
        _ => unreachable!(),
    };

    let mut p1 = [0, 0];
    let mut q = VecDeque::new();
    'outer: for t in 1.. {
        q.push_back(("broadcaster", "button", false));

        while let Some((node, prev, high)) = q.pop_front() {
            if t <= 1000 {
                p1[high as usize] += 1;
            }

            if high && node == rx_conjunction {
                let v = cycles.get_mut(prev).unwrap();
                if v.is_none() {
                    *v = Some(t);
                    if cycles.values().all(|o| o.is_some()) {
                        break 'outer;
                    }
                }
            }

            let pulse = match state.get_mut(node) {
                Some(Node::FlipFlop(_)) if high => continue,
                Some(Node::FlipFlop(on)) => {
                    *on = !*on;
                    *on
                }
                Some(Node::Conjunction(m)) => {
                    m.insert(prev, high);
                    m.values().any(|&b| !b)
                }
                Some(Node::Broadcaster) => false,
                None => continue,
            };

            q.extend(g[node].iter().map(|&n| (n, node, pulse)));
        }
    }

    cycles.values().map(|o| o.unwrap()).product()
}
