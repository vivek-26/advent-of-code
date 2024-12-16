import heapq


directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]  # up right down left
G = open("input.txt").read().strip().splitlines()
R = len(G)
C = len(G[0])
G = [[G[r][c] for c in range(C)] for r in range(R)]

for r in range(R):
    for c in range(C):
        if G[r][c] == "S":
            sr, sc = r, c
        if G[r][c] == "E":
            er, ec = r, c


# part 1
Q = []
seen = set()
heapq.heappush(Q, (0, sr, sc, 1))
dist = {}

best = None
while Q:
    d, r, c, dir = heapq.heappop(Q)
    if (r, c, dir) not in dist:
        dist[(r, c, dir)] = d
    if r == er and c == ec and best is None:
        best = d
    if (r, c, dir) in seen:
        continue
    seen.add((r, c, dir))
    dr, dc = directions[dir]
    rr, cc = r + dr, c + dc
    if 0 <= cc < C and 0 <= rr < R and G[rr][cc] != "#":
        heapq.heappush(Q, (d + 1, rr, cc, dir))
    heapq.heappush(Q, (d + 1000, r, c, (dir + 1) % 4))
    heapq.heappush(Q, (d + 1000, r, c, (dir + 3) % 4))

print(best)

Q = []
seen = set()
for dir in range(4):
    heapq.heappush(Q, (0, er, ec, dir))

dist2 = {}
while Q:
    d, r, c, dir = heapq.heappop(Q)
    if (r, c, dir) not in dist2:
        dist2[(r, c, dir)] = d
    if (r, c, dir) in seen:
        continue
    seen.add((r, c, dir))
    # going backwards instead of forwards here
    dr, dc = directions[(dir + 2) % 4]
    rr, cc = r + dr, c + dc
    if 0 <= cc < C and 0 <= rr < R and G[rr][cc] != "#":
        heapq.heappush(Q, (d + 1, rr, cc, dir))
    heapq.heappush(Q, (d + 1000, r, c, (dir + 1) % 4))
    heapq.heappush(Q, (d + 1000, r, c, (dir + 3) % 4))

OK = set()
for r in range(R):
    for c in range(C):
        for dir in range(4):
            # (r,c,dir) is on an optimal path if the distance from start to end equals the distance from start to (r,c,dir) plus the distance from (r,c,dir) to end.
            if (
                (r, c, dir) in dist
                and (r, c, dir) in dist2
                and dist[(r, c, dir)] + dist2[(r, c, dir)] == best
            ):
                OK.add((r, c))
print(len(OK))
