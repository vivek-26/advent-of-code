import collections
import math

grid = []
with open("input.txt", "r") as f:
    for line in f.readlines():
        grid.append(list(map(ord, line.strip())))

directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]
start = ord("S")
target = ord("E")
R, C = len(grid), len(grid[0])
starting_point = None
starting_points = []
ending_point = None

for i in range(R):
    for j in range(C):
        if grid[i][j] == start:
            starting_point = (i, j)
            grid[i][j] = ord("a")
            starting_points.append((i, j))
        elif grid[i][j] == target:
            ending_point = (i, j)
            grid[i][j] = ord("z")
        elif grid[i][j] == ord("a"):
            starting_points.append((i, j))

assert starting_point is not None
assert len(starting_points) != 0
assert ending_point is not None


def is_valid_position(m, n):
    return 0 <= m < R and 0 <= n < C


def bfs(deq, visited):
    while deq:
        p, q, steps = deq.popleft()
        if (p, q) in visited:
            continue

        visited.add((p, q))

        if (p, q) == ending_point:
            print(steps)
            continue

        for di, dj in directions:
            x, y = p + di, q + dj
            if is_valid_position(x, y) and grid[x][y] <= 1 + grid[p][q]:
                deq.append((x, y, steps + 1))


def dfs(xx, yy, steps, visited):
    if (xx, yy) in visited:
        return math.inf

    if (xx, yy) == ending_point:
        return steps

    dist = []
    for di, dj in directions:
        x, y = xx + di, yy + dj
        if is_valid_position(x, y) and (xx, yy) not in visited and grid[x][y] <= 1 + grid[xx][yy]:
            visited.add((xx, yy))
            dist.append(dfs(x, y, steps + 1, visited))
            visited.remove((xx, yy))

    return min(dist)


# part 1
deq = collections.deque()
deq.append((starting_point[0], starting_point[1], 0))
bfs(deq, set())
# print(dfs(starting_point[0], starting_point[1], 0, set()))

# part 2
deq = collections.deque()
for m, n in starting_points:
    deq.append((m, n, 0))
bfs(deq, set())
