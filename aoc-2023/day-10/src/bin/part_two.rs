use std::collections::{HashSet, VecDeque};

use day_10::{Coordinate, Direction, PipeDirection};

fn main() {
    let lines = aoc::read_input_lines(10);

    let mut starting_pos = Coordinate(0, 0);
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
                        starting_pos = Coordinate(row as isize, col as isize);
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
                    let initial_direction = initial_direction(&grid, starting_pos);
                    queue.push_back(curr.go_to(&initial_direction));
                    continue;
                }
            }

            visited.insert(curr);

            for direction in grid[curr.0 as usize][curr.1 as usize].can_go_to() {
                let coords = curr.go_to(&direction);
                if check_grid_bounds(&grid, coords) && !visited.contains(&coords) {
                    queue.push_back(coords);
                }
            }
        }
    }

    // find inside points
    let mut inside_count = 0_usize;
    for (x, line) in grid.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            let curr_coord = Coordinate(x as isize, y as isize);
            if visited.contains(&curr_coord) {
                continue;
            }

            let mut crosses = 0_usize;
            let mut new_coord = curr_coord;
            while new_coord.1 < grid[0].len() as isize && new_coord.0 < grid.len() as isize {
                if visited.contains(&new_coord)
                    && grid[new_coord.0 as usize][new_coord.1 as usize]
                        != PipeDirection::NorthToEast
                    && grid[new_coord.0 as usize][new_coord.1 as usize]
                        != PipeDirection::SouthToWest
                {
                    crosses += 1;
                }

                new_coord.add(Coordinate(1, 1));
            }

            if crosses % 2 == 1 {
                inside_count += 1;
            }
        }
    }

    println!("{inside_count}");
}

fn initial_direction(grid: &Vec<Vec<PipeDirection>>, starting_pos: Coordinate) -> Direction {
    for direction in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        let coords = starting_pos.go_to(&direction);

        if check_grid_bounds(grid, coords)
            && grid[coords.0 as usize][coords.1 as usize]
                .can_go_to()
                .contains(&direction.opposite())
        {
            return direction;
        }
    }

    panic!("invalid starting position");
}

fn check_grid_bounds(grid: &Vec<Vec<PipeDirection>>, coord: Coordinate) -> bool {
    coord.0 >= 0
        && coord.0 < grid.len() as isize
        && coord.1 >= 0
        && coord.1 < grid[0].len() as isize
}
