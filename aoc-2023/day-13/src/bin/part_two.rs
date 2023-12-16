#[aoc_runner::timeit]
fn main() -> i64 {
    let input = aoc::read_input(13);
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut total: i64 = 0;
    for i in input {
        let grid = parse_grid(i);
        let (h, v) = find_reflection(grid);
        total += 100 * h as i64;
        total += v as i64;
    }

    total
}

fn parse_grid(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn get_row(grid: &[Vec<char>], row: i64) -> String {
    grid[row as usize].iter().collect()
}

fn get_col(grid: &[Vec<char>], col: i64) -> String {
    grid.iter().map(|row| row[col as usize]).collect()
}

fn compare_rows(grid: &[Vec<char>], row1: i64, row2: i64) -> i64 {
    get_row(grid, row1)
        .chars()
        .zip(get_row(grid, row2).chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count() as i64
}

fn compare_cols(grid: &[Vec<char>], col1: i64, col2: i64) -> i64 {
    get_col(grid, col1)
        .chars()
        .zip(get_col(grid, col2).chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count() as i64
}

fn find_reflection(grid: Vec<Vec<char>>) -> (usize, usize) {
    for col in 1..grid[0].len() {
        let mut l = col as i64 - 1;
        let mut r = col;
        let mut diffs = 0;
        while l >= 0 && r < grid[0].len() {
            if diffs > 1 {
                break;
            }
            diffs += compare_cols(&grid, l, r as i64);
            l -= 1;
            r += 1;
        }
        if diffs == 1 {
            return (0, col);
        }
    }
    for row in 1..grid.len() {
        let mut u = row as i64 - 1;
        let mut d = row;
        let mut diffs = 0;
        while u >= 0 && d < grid.len() {
            if diffs > 1 {
                break;
            }
            diffs += compare_rows(&grid, u, d as i64);
            u -= 1;
            d += 1;
        }
        if diffs == 1 {
            return (row, 0);
        }
    }

    (0, 0)
}
