with open("input.txt") as input_file:
    grid = [list(row.strip()) for row in input_file]


# part 1
word = "XMAS"
all_directions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)]


def count_xmas_occurrences(grid: list[list[str]]) -> int:
    rows, cols = len(grid), len(grid[0])
    word_len = len(word)
    total_count = 0

    # helper function to check if a word exists starting from (r, c) in a given direction
    def check_direction(r: int, c: int, dr: int, dc: int) -> bool:
        for i in range(word_len):
            nr, nc = r + i * dr, c + i * dc
            if nr < 0 or nc < 0 or nr >= rows or nc >= cols or grid[nr][nc] != word[i]:
                return False
        return True

    for r in range(rows):
        for c in range(cols):
            # check all 8 possible directions
            for dr, dc in all_directions:
                if check_direction(r, c, dr, dc):
                    total_count += 1

    return total_count


print(count_xmas_occurrences(grid))


# part 2
def count_xmas_shape_occurrences(grid: list[list[str]]) -> int:
    rows, cols = len(grid), len(grid[0])
    total_count = 0

    for r in range(1, rows - 1):
        for c in range(1, cols - 1):
            if grid[r][c] != "A":
                continue

            top, bottom = (
                grid[r - 1][c - 1] + grid[r + 1][c + 1],
                grid[r - 1][c + 1] + grid[r + 1][c - 1],
            )

            if sorted(top) == sorted(bottom) == ["M", "S"]:
                total_count += 1

    return total_count


print(count_xmas_shape_occurrences(grid))
