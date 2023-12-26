use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[aoc_runner::main(25)]
fn main(input: &str) -> usize {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    let mut edges = HashSet::new();
    for line in input.lines() {
        let (a, rest) = line.split_once(": ").unwrap();
        for b in rest.split(' ') {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
            edges.insert(if a < b { (a, b) } else { (b, a) });
        }
    }

    let mut dot = String::from("graph {\n");
    for (a, b) in edges.iter().sorted() {
        dot += &format!("  {} -- {};\n", a, b);
    }
    dot += "}";

    // Run the following to visualize the graph:
    //   dot -Tsvg -Kneato graph.dot > graph.svg
    // Manually find the three edges.
    // If dot command is not available, install graphviz.
    std::fs::write("graph.dot", dot).unwrap();

    for (a, b) in [("grd", "tqr"), ("dlv", "tqh"), ("bmd", "ngp")] {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }

    let size = component_size(&graph, "tqr");
    (graph.len() - size) * size
}

fn component_size(graph: &HashMap<&str, HashSet<&str>>, a: &str) -> usize {
    let (mut seen, mut s) = (HashSet::new(), vec![a]);
    while let Some(x) = s.pop() {
        if seen.insert(x) {
            s.extend(&graph[x]);
        }
    }

    seen.len()
}
