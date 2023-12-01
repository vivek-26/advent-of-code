def get_blocked_positions():
    blocked = set()

    for line in open("input.txt"):
        pairs = [tuple(map(int, x.split(","))) for x in line.strip().split(" -> ")]

        if len(pairs) == 1:
            blocked.add(pairs[0])
            continue

        for i in range(len(pairs) - 1):
            blocked.add(pairs[i])
            blocked.add(pairs[i + 1])

            dx = abs(pairs[i][0] - pairs[i + 1][0])
            dy = abs(pairs[i][1] - pairs[i + 1][1])

            for j in range(1, dx + 1):
                xx = pairs[i][0] + j if pairs[i][0] < pairs[i + 1][0] else pairs[i][0] - j
                blocked.add((xx, pairs[i][1]))

            for k in range(1, dy + 1):
                yy = pairs[i][1] + k if pairs[i][1] < pairs[i + 1][1] else pairs[i][1] - k
                blocked.add((pairs[i][0], yy))

    return blocked


def part_one():
    blocked = get_blocked_positions()
    start = (500, 0)
    total = 0
    floor = max(s[1] for s in blocked)  # abyss

    curr = start
    while True:
        if curr[1] > floor:  # reached abyss
            break

        if (curr[0], curr[1] + 1) not in blocked:
            curr = (curr[0], curr[1] + 1)
            continue

        if (curr[0] - 1, curr[1] + 1) not in blocked:
            curr = (curr[0] - 1, curr[1] + 1)
            continue

        if (curr[0] + 1, curr[1] + 1) not in blocked:
            curr = (curr[0] + 1, curr[1] + 1)
            continue

        blocked.add(curr)
        total += 1
        curr = start

    print(total)


def part_two():
    blocked = get_blocked_positions()
    start = (500, 0)
    total = 0
    floor = 2 + max(s[1] for s in blocked)  # abyss

    curr = start
    while start not in blocked:
        if curr[1] + 1 == floor:
            blocked.add(curr)
            total += 1
            curr = start
            continue

        if (curr[0], curr[1] + 1) not in blocked:
            curr = (curr[0], curr[1] + 1)
            continue

        if (curr[0] - 1, curr[1] + 1) not in blocked:
            curr = (curr[0] - 1, curr[1] + 1)
            continue

        if (curr[0] + 1, curr[1] + 1) not in blocked:
            curr = (curr[0] + 1, curr[1] + 1)
            continue

        blocked.add(curr)
        total += 1
        curr = start

    print(total)


part_one()
part_two()
