use hashbrown::HashSet;
use itertools::Itertools;

#[aoc_runner::main(21)]
fn main(input: &str) -> usize {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == b'S')
        .map(|(r, c)| (r as isize, c as isize))
        .unwrap();

    bfs(&grid, start, 64)
}

fn bfs(grid: &[&[u8]], start: (isize, isize), steps: usize) -> usize {
    let mut pos = HashSet::from_iter([start]);
    let mut new_pos = HashSet::new();

    for _ in 0..steps {
        new_pos.clear();
        for &(r, c) in &pos {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (rr, cc) = (r + dr, c + dc);
                let tile = grid[rr as usize % grid.len()][cc as usize % grid.len()];
                if tile != b'#' {
                    new_pos.insert((rr, cc));
                }
            }
        }

        (pos, new_pos) = (new_pos, pos);
    }

    pos.len()
}
