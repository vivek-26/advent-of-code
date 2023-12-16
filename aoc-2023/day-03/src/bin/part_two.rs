use std::collections::HashMap;

const DIR: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[aoc_runner::timeit]
fn main() -> u64 {
    let board: Vec<Vec<char>> = aoc::read_input_lines(3)
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rows = board.len();
    let cols = board[0].len();
    let mut gear_ratios = HashMap::new();

    for r in 0..rows {
        let mut num = 0;
        let mut is_valid = false;
        let (mut gear_row, mut gear_col) = (0, 0);

        for c in 0..cols {
            if !board[r][c].is_ascii_digit() {
                if is_valid {
                    is_valid = false;

                    gear_ratios
                        .entry((gear_row, gear_col))
                        .or_insert(vec![])
                        .push(num);
                }

                num = 0;
                continue;
            }

            num *= 10;
            num += (board[r][c].to_digit(10).unwrap()) as u64;

            if !is_valid {
                for (dr, dc) in DIR.iter().copied() {
                    let Some(rx) = r.checked_add_signed(dr) else {
                        continue;
                    };
                    let Some(cx) = c.checked_add_signed(dc) else {
                        continue;
                    };
                    if rx >= rows || cx >= cols {
                        continue;
                    }

                    if board[rx][cx] == '*' {
                        gear_row = rx;
                        gear_col = cx;
                        is_valid = true;
                    }
                }
            }
        }

        // leftover number at the end of the row
        if is_valid {
            gear_ratios
                .entry((gear_row, gear_col))
                .or_insert(vec![])
                .push(num);
        }
    }

    gear_ratios
        .into_values()
        .filter(|x| x.len() == 2)
        .map(|x| x[0] * x[1])
        .sum()
}
