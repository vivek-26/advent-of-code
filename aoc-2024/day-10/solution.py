from collections import deque

with open("input.txt", "r") as input_file:
    grid = {}
    for y, line in enumerate(input_file.read().splitlines()):
        for x, char in enumerate(line):
            grid[x + y * 1j] = int(char)

trailheads = [key for key, value in grid.items() if value == 0]


def get_score(trailhead, count_distinct_paths=False):
    score = 0
    queue = deque([trailhead])
    encountered_positions = set()
    while queue:
        position = queue.popleft()
        if position in encountered_positions:
            continue

        if not count_distinct_paths:
            encountered_positions.add(position)
        elevation = grid[position]

        if elevation == 9:
            score += 1
            continue

        surrounding_positions = [
            position + direction
            for direction in [1, -1, 1j, -1j]
            if position + direction in grid
            and grid[position + direction] == elevation + 1
        ]

        queue.extend(surrounding_positions)

    return score


print(sum(get_score(trailhead) for trailhead in trailheads))
print(
    sum(get_score(trailhead, count_distinct_paths=True) for trailhead in trailheads),
)
