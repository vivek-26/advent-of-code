use std::collections::{HashSet, VecDeque};

use day_10::{Direction, PipeDirection};

fn main() {
    let lines = aoc::read_input_lines(10);

    let mut starting_pos = (0_usize, 0_usize);
    let grid = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '|' => PipeDirection::Vertical,
                    '-' => PipeDirection::Horizontal,
                    'L' => PipeDirection::NorthToEast,
                    'J' => PipeDirection::NorthToWest,
                    '7' => PipeDirection::SouthToWest,
                    'F' => PipeDirection::SouthToEast,
                    '.' => PipeDirection::Ground,
                    'S' => {
                        starting_pos = (row, col);
                        PipeDirection::StartingPos
                    }
                    _ => panic!("invalid pipe direction"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    // bfs
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(starting_pos);

    'bfs_loop: while !queue.is_empty() {
        let curr_len = queue.len();
        for _ in 0..curr_len {
            let curr = queue.pop_front().unwrap();

            if curr == starting_pos {
                if visited.contains(&curr) {
                    break 'bfs_loop;
                } else {
                    visited.insert(curr);
                    let initial_direction = initial_direction(starting_pos, &grid).get_coords();
                    queue.push_back((
                        (curr.0 as isize + initial_direction.0) as usize,
                        (curr.1 as isize + initial_direction.1) as usize,
                    ));
                    continue;
                }
            }

            visited.insert(curr);

            for direction in grid[curr.0][curr.1].can_go_to() {
                let coords = direction.get_coords();
                let (r, c) = (curr.0 as isize + coords.0, curr.1 as isize + coords.1);
                if r >= 0
                    && r < grid.len() as isize
                    && c >= 0
                    && c < grid[0].len() as isize
                    && !visited.contains(&(r as usize, c as usize))
                {
                    queue.push_back((r as usize, c as usize));
                }
            }
        }
    }

    // find inside points
    let mut inside_count = 0_usize;
    for (x, line) in grid.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut crosses = 0_usize;
            let (mut x2, mut y2) = (x, y);
            while y2 < grid[0].len() && x2 < grid.len() {
                if visited.contains(&(x2, y2))
                    && grid[x2][y2] != PipeDirection::NorthToEast
                    && grid[x2][y2] != PipeDirection::SouthToWest
                {
                    crosses += 1;
                }

                y2 += 1;
                x2 += 1;
            }

            if crosses % 2 == 1 {
                inside_count += 1;
            }
        }
    }

    println!("{inside_count}");
}

fn initial_direction(starting_pos: (usize, usize), grid: &Vec<Vec<PipeDirection>>) -> Direction {
    for direction in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        let coords = direction.get_coords();
        let (r, c) = (
            starting_pos.0 as isize + coords.0,
            starting_pos.1 as isize + coords.1,
        );

        if r >= 0
            && r < grid.len() as isize
            && c >= 0
            && c < grid[0].len() as isize
            && grid[r as usize][c as usize]
                .can_go_to()
                .contains(&direction.opposite())
        {
            return direction;
        }
    }

    panic!("invalid starting position");
}
