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

#[aoc_runner::main(3)]
fn main(input: &str) -> u64 {
    let board: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rows = board.len();
    let cols = board[0].len();
    let mut answer = 0;

    for r in 0..rows {
        let mut num = 0;
        let mut is_valid = false;

        for c in 0..cols {
            if !board[r][c].is_ascii_digit() {
                if is_valid {
                    answer += num;
                    is_valid = false;
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

                    if board[rx][cx] != '.' && !board[rx][cx].is_ascii_digit() {
                        is_valid = true;
                    }
                }
            }
        }

        // leftover number at the end of the row
        if is_valid {
            answer += num;
        }
    }

    answer
}
