pub fn solve(input: &str, expansion: usize) -> usize {
    let grid: Vec<Vec<_>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let vertical_distances: Vec<_> = (0..grid.len())
        .map(|r| grid[r].iter().all(|&c| c == '.') as usize * (expansion - 1) + 1)
        .scan(0, |curr_state, v| {
            *curr_state += v;
            Some(*curr_state)
        })
        .collect();

    let horizontal_distances: Vec<_> = (0..grid[0].len())
        .map(|c| grid.iter().all(|row| row[c] == '.') as usize * (expansion - 1) + 1)
        .scan(0, |curr_state, h| {
            *curr_state += h;
            Some(*curr_state)
        })
        .collect();

    let mut galaxies = vec![];
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.push((r, c));
            }
        }
    }

    let mut dists = 0;
    for (i, &(r0, c0)) in galaxies.iter().enumerate() {
        for &(r1, c1) in galaxies.iter().skip(i + 1) {
            dists += vertical_distances[r0.max(r1)] - vertical_distances[r0.min(r1)];
            dists += horizontal_distances[c0.max(c1)] - horizontal_distances[c0.min(c1)];
        }
    }

    dists
}
