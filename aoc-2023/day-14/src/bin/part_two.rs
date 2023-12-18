use std::collections::HashMap;

use day_14::Landscape;

#[aoc_runner::main(14)]
fn main(input: &str) -> usize {
    let mut grid = parse_input(input);
    let total_cycles = 1000000000_usize;
    let mut map = HashMap::new();
    let mut index = 0_usize;
    map.insert(grid.clone(), index);

    let cycle_start: usize;
    loop {
        cycle(&mut grid);
        index += 1;

        if let Some(&idx) = map.get(&grid) {
            cycle_start = idx;
            break;
        }

        map.insert(grid.clone(), index);
    }

    let mut answer = 0;
    let cycle_length = map.len() - cycle_start;
    let last_grid_idx = cycle_start + (total_cycles - cycle_start) % cycle_length;
    for (grid, idx) in map {
        if idx == last_grid_idx {
            answer = total_load_north_beam(&grid);
            break;
        }
    }

    answer
}

fn parse_input(input: &str) -> Vec<Vec<Landscape>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'O' => Landscape::RoundedRock,
                    '#' => Landscape::CubeRock,
                    '.' => Landscape::EmptySpace,
                    _ => panic!("invalid landscape"),
                })
                .collect()
        })
        .collect()
}

fn cycle(grid: &mut Vec<Vec<Landscape>>) {
    slide_north(grid);
    slide_west(grid);
    slide_south(grid);
    slide_east(grid);
}

fn slide_north(grid: &mut Vec<Vec<Landscape>>) {
    fn next_empty_slot(grid: &[Vec<Landscape>], row: usize, col: usize) -> usize {
        (row..grid.len())
            .find(|&r| grid[r][col] == Landscape::EmptySpace)
            .unwrap_or(grid.len())
    }

    for col in 0..grid[0].len() {
        let mut ptr = next_empty_slot(grid, 0, col);
        let mut curr = ptr + 1;

        while curr < grid.len() {
            match grid[curr][col] {
                Landscape::RoundedRock => {
                    grid[ptr][col] = Landscape::RoundedRock;
                    grid[curr][col] = Landscape::EmptySpace;
                    ptr += 1;
                    curr += 1;
                }
                Landscape::CubeRock => {
                    ptr = next_empty_slot(grid, curr, col);
                    curr = ptr + 1;
                }
                Landscape::EmptySpace => {
                    curr += 1;
                }
            };
        }
    }
}

fn slide_west(grid: &mut Vec<Vec<Landscape>>) {
    fn next_empty_slot(grid: &[Vec<Landscape>], row: usize, col: usize) -> usize {
        (col..grid[0].len())
            .find(|&c| grid[row][c] == Landscape::EmptySpace)
            .unwrap_or(grid[0].len())
    }

    for row in 0..grid.len() {
        let mut ptr = next_empty_slot(grid, row, 0);
        let mut curr = ptr + 1;

        while curr < grid[0].len() {
            match grid[row][curr] {
                Landscape::RoundedRock => {
                    grid[row][ptr] = Landscape::RoundedRock;
                    grid[row][curr] = Landscape::EmptySpace;
                    ptr += 1;
                    curr += 1;
                }
                Landscape::CubeRock => {
                    ptr = next_empty_slot(grid, row, curr);
                    curr = ptr + 1;
                }
                Landscape::EmptySpace => {
                    curr += 1;
                }
            };
        }
    }
}

fn slide_south(grid: &mut Vec<Vec<Landscape>>) {
    fn next_empty_slot(grid: &[Vec<Landscape>], row: usize, col: usize) -> isize {
        (0..=row as isize)
            .rev()
            .find(|&r| grid[r as usize][col] == Landscape::EmptySpace)
            .unwrap_or(-1)
    }

    for col in 0..grid[0].len() {
        let mut ptr = next_empty_slot(grid, grid.len() - 1, col);
        let mut curr = ptr - 1;

        while curr >= 0 {
            match grid[curr as usize][col] {
                Landscape::RoundedRock => {
                    grid[ptr as usize][col] = Landscape::RoundedRock;
                    grid[curr as usize][col] = Landscape::EmptySpace;
                    ptr -= 1;
                    curr -= 1;
                }
                Landscape::CubeRock => {
                    ptr = next_empty_slot(grid, curr as usize, col);
                    curr = ptr - 1;
                }
                Landscape::EmptySpace => {
                    curr -= 1;
                }
            };
        }
    }
}

fn slide_east(grid: &mut Vec<Vec<Landscape>>) {
    fn next_empty_slot(grid: &[Vec<Landscape>], row: usize, col: usize) -> isize {
        (0..=col as isize)
            .rev()
            .find(|&c| grid[row][c as usize] == Landscape::EmptySpace)
            .unwrap_or(-1)
    }

    for row in 0..grid.len() {
        let mut ptr = next_empty_slot(grid, row, grid[0].len() - 1);
        let mut curr = ptr - 1;

        while curr >= 0 {
            match grid[row][curr as usize] {
                Landscape::RoundedRock => {
                    grid[row][ptr as usize] = Landscape::RoundedRock;
                    grid[row][curr as usize] = Landscape::EmptySpace;
                    ptr -= 1;
                    curr -= 1;
                }
                Landscape::CubeRock => {
                    ptr = next_empty_slot(grid, row, curr as usize);
                    curr = ptr - 1;
                }
                Landscape::EmptySpace => {
                    curr -= 1;
                }
            };
        }
    }
}

fn total_load_north_beam(grid: &[Vec<Landscape>]) -> usize {
    grid.iter().enumerate().fold(0, |total, (row, line)| {
        total
            + line
                .iter()
                .filter(|&cell| *cell == Landscape::RoundedRock)
                .count()
                * (grid.len() - row)
    })
}
