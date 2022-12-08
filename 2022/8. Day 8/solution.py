directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]


def solve_part_1(grid):
    r, c = len(grid), len(grid[0])
    total = 0

    def is_visible(m, n):
        for di, dj in directions:
            p, q = di + m, dj + n
            while 0 <= p < r and 0 <= q < c and grid[m][n] > grid[p][q]:
                p += di
                q += dj

            if (p == -1 or p == r) or (q == -1 or q == c):  # we reached the end of the grid
                return True

        return False

    for i in range(1, r - 1):  # ignore borders of the grid
        for j in range(1, c - 1):
            if is_visible(i, j) is True:
                total += 1

    border = (r * 2) + (c * 2) - 4
    print(total + border)


def solve_part_2(grid):
    r, c = len(grid), len(grid[0])
    highest_score = 0

    def calc_score(m, n):
        score = 1
        for di, dj in directions:
            p, q = di + m, dj + n
            num_tree = 0
            while 0 <= p < r and 0 <= q < c:
                num_tree += 1
                if grid[p][q] >= grid[m][n]:
                    break
                p += di
                q += dj

            score *= num_tree
        return score

    for i in range(1, r - 1):
        for j in range(c - 1):
            highest_score = max(highest_score, calc_score(i, j))

    print(highest_score)


if __name__ == "__main__":
    lines = []
    with open("input.txt", "r") as f:
        for line in f.readlines():
            lines.append(list(map(int, line.strip())))

    solve_part_1(lines)
    solve_part_2(lines)
