from collections import deque


inp = open("input.txt").read().strip()

directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]  # up right down left
N = 71
G = [["." for _ in range(N)] for _ in range(N)]
for i, line in enumerate(inp.split("\n")):
    x, y = [int(x) for x in line.split(",")]
    if 0 <= y < N and 0 <= x < N:
        G[y][x] = "#"

    Q = deque([(0, 0, 0)])
    SEEN = set()
    ok = False
    while Q:
        d, r, c = Q.popleft()
        if (r, c) == (N - 1, N - 1):
            if i == 1023:
                print(d)
            ok = True
            break
        if (r, c) in SEEN:
            continue
        SEEN.add((r, c))
        for dr, dc in directions:
            rr = r + dr
            cc = c + dc
            if 0 <= rr < N and 0 <= cc < N and G[rr][cc] != "#":
                Q.append((d + 1, rr, cc))
    if not ok:
        print(f"{x},{y}")
        break
