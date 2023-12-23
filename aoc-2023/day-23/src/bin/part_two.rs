use hashbrown::HashMap;

#[aoc_runner::main(23)]
fn main(input: &str) -> usize {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let mut graph = HashMap::<_, Vec<_>>::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'#' {
                continue;
            }

            let neighbors: &[_] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
            let e = graph.entry((r, c)).or_default();

            for (dr, dc) in neighbors {
                let rr = (r as isize + dr) as usize;
                let cc = (c as isize + dc) as usize;
                let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
                    continue;
                };
                if tile != b'#' {
                    e.push((rr, cc, 1));
                }
            }
        }
    }

    while let Some((&(r, c), _)) = graph.iter().find(|(_, n)| n.len() == 2) {
        let neighbors = graph.remove(&(r, c)).unwrap();
        let (r1, c1, d1) = neighbors[0];
        let (r2, c2, d2) = neighbors[1];
        let n1 = graph.entry((r1, c1)).or_default();
        if let Some(i) = n1.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n1[i] = (r2, c2, d1 + d2);
        }
        let n2 = graph.entry((r2, c2)).or_default();
        if let Some(i) = n2.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n2[i] = (r1, c1, d1 + d2);
        }
    }

    dfs(
        &graph,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
        (0, 1),
    )
    .unwrap()
}

#[allow(clippy::type_complexity)]
fn dfs(
    graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    seen: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
) -> Option<usize> {
    if r == seen.len() - 1 {
        return Some(0);
    }

    let mut max_dist = None;
    for &(rr, cc, d) in &graph[&(r, c)] {
        if !seen[rr][cc] {
            seen[rr][cc] = true;
            if let Some(dist) = dfs(graph, seen, (rr, cc)) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[rr][cc] = false;
        }
    }

    max_dist
}
