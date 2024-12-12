from collections import defaultdict, deque


with open(
    "/Users/vivek/Documents/github.com/advent-of-code/aoc-2024/day-12/input.txt"
) as input_file:
    grid_lines = input_file.read().splitlines()
    R, C = len(grid_lines), len(grid_lines[0])
    directions = [(-1, 0), (0, -1), (0, 1), (1, 0)]
    grid = {
        (row, col): char
        for row, line in enumerate(grid_lines)
        for col, char in enumerate(line)
    }


# part 1
def fencing_cost(grid: dict[tuple[int, int], str], R: int, C: int) -> int:
    total_cost: int = 0
    visited: set[tuple[int, int]] = set()

    def bfs(r: int, c: int) -> int:
        area: int = 0
        perimeter: int = 0
        dq: deque[tuple[int, int]] = deque()
        dq.append((r, c))

        while len(dq) > 0:
            x, y = dq.popleft()
            if (x, y) in visited:
                continue
            visited.add((x, y))
            area += 1
            curr_perimeter = 4
            for dx, dy in directions:
                nr, nc = x + dx, y + dy
                if (nr, nc) not in grid:
                    continue
                if grid[(nr, nc)] == grid[(x, y)]:
                    dq.append((nr, nc))
                    curr_perimeter -= 1
            perimeter += curr_perimeter

        return area * perimeter

    for r in range(R):
        for c in range(C):
            if (r, c) not in visited:
                total_cost += bfs(r, c)

    return total_cost


# part 2
def bulk_discount_cost(grid: dict[tuple[int, int], str], R: int, C: int) -> int:
    visited = set()
    total_cost = 0

    for r in range(R):
        for c in range(C):
            if (r, c) in visited:
                continue
            dq = deque([(r, c)])
            area = 0
            perimeter = 0
            perim = defaultdict(set)

            while dq:
                r2, c2 = dq.popleft()
                if (r2, c2) in visited:
                    continue
                visited.add((r2, c2))
                area += 1
                for dr, dc in directions:
                    rr, cc = r2 + dr, c2 + dc
                    if 0 <= rr < R and 0 <= cc < C and grid[(rr, cc)] == grid[(r2, c2)]:
                        dq.append((rr, cc))
                    else:
                        perimeter += 1
                        perim[(dr, dc)].add((r2, c2))

            sides = 0
            for _, vs in perim.items():
                seen_perim = set()
                for pr, pc in vs:
                    if (pr, pc) not in seen_perim:
                        sides += 1
                        dq = deque([(pr, pc)])
                        while dq:
                            r2, c2 = dq.popleft()
                            if (r2, c2) in seen_perim:
                                continue
                            seen_perim.add((r2, c2))
                            for dr, dc in directions:
                                rr, cc = r2 + dr, c2 + dc
                                if (rr, cc) in vs:
                                    dq.append((rr, cc))

            total_cost += area * sides

    return total_cost


print(fencing_cost(grid, R, C))
print(bulk_discount_cost(grid, R, C))
