use day_14::Landscape;

fn main() {
    let mut grid = parse_input(aoc::read_input_lines(14));
    slide_north(&mut grid);
    println!("{}", total_load(&grid));
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Landscape>> {
    input
        .iter()
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

fn next_empty_slot(grid: &Vec<Vec<Landscape>>, row: usize, col: usize) -> usize {
    (row..grid.len())
        .find(|&r| grid[r][col] == Landscape::EmptySpace)
        .unwrap_or(grid.len())
}

fn slide_north(grid: &mut Vec<Vec<Landscape>>) {
    for col in 0..grid[0].len() {
        for _ in 0..grid.len() {
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
}

fn total_load(grid: &Vec<Vec<Landscape>>) -> usize {
    grid.iter().enumerate().fold(0, |total, (row, line)| {
        total
            + line
                .iter()
                .filter(|&cell| *cell == Landscape::RoundedRock)
                .count()
                * (grid.len() - row)
    })
}
