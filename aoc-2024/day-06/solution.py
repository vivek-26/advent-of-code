import json


with open("input.txt") as input_file:
    grid = [list(row.strip()) for row in input_file.read().splitlines()]


directions = ((-1, 0), (0, 1), (1, 0), (0, -1))
rows, cols = len(grid), len(grid[0])


def get_guard_pos(_map) -> tuple[int, int]:
    rows, cols = len(_map), len(_map[0])
    for i in range(rows):
        for j in range(cols):
            if _map[i][j] == "^":
                return (i, j)


def patrol(_map: list[list[str]], pos: tuple[int, int] = None, idx: int = None):
    if not pos:
        pos = get_guard_pos(_map)

    if not idx:
        idx = 0

    visited = set()
    visited.add((pos[0], pos[1]))

    visited_entry = {}  # for part 2, mark the entry point of the visited node

    while True:
        d = directions[idx]
        n = (pos[0] + d[0], pos[1] + d[1])

        if n[0] < 0 or n[0] >= rows or n[1] < 0 or n[1] >= cols:
            return True, visited, visited_entry  # leaving the map

        if _map[n[0]][n[1]] == "#":
            idx = (idx + 1) % 4
            continue
        else:
            visited.add((n[0], n[1]))
            if n not in visited_entry:
                visited_entry[n] = (pos, idx)
            elif visited_entry[n] == (pos, idx):
                return False, None, None  # loop detected
            pos = n


# part 1
def count_unique_visited_positions(grid: list[list[str]]):
    _, visited, _ = patrol(grid)
    return len(visited)


print(count_unique_visited_positions(grid))


# part 2
def count_obstruction_loops(grid: list[list[str]]):
    _, visited, visited_entry = patrol(grid)

    visited.remove(
        get_guard_pos(grid)
    )  # avoid the guard position, you can not put obstruction there
    loop_count = 0

    _map_dump = json.dumps(grid)  # json dumps/loads faster than deepcopy

    # don't have to test every empty space, just the visited ones
    # because the obstruction must be on the visited path
    for vi, vj in visited:
        _map_copy = json.loads(_map_dump)
        _map_copy[vi][vj] = "#"

        pos = visited_entry[(vi, vj)][0]
        idx = visited_entry[(vi, vj)][1]

        is_leaving_copy, _, _ = patrol(_map_copy, pos, idx)
        if not is_leaving_copy:  # not leaving, because of the loop
            loop_count += 1

    return loop_count


print(count_obstruction_loops(grid))
