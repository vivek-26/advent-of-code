#[aoc_runner::main(13)]
fn main(input: &str) -> i64 {
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

fn find_reflection(grid: Vec<Vec<char>>) -> (usize, usize) {
    for col in 1..grid[0].len() {
        let mut l = col as i64 - 1;
        let mut r = col;
        let mut valid = true;
        while l >= 0 && r < grid[0].len() {
            if get_col(&grid, l) != get_col(&grid, r as i64) {
                valid = false;
                break;
            }
            l -= 1;
            r += 1;
        }
        if valid {
            return (0, col);
        }
    }

    for row in 1..grid.len() {
        let mut u = row as i64 - 1;
        let mut d = row;
        let mut valid = true;
        while u >= 0 && d < grid.len() {
            if get_row(&grid, u) != get_row(&grid, d as i64) {
                valid = false;
                break;
            }
            u -= 1;
            d += 1;
        }
        if valid {
            return (row, 0);
        }
    }

    (0, 0)
}
