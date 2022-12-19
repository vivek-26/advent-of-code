import re
from collections import defaultdict

graph = {}
rates = {}
for line in open("input.txt").read().strip().split("\n"):
    front, back = re.split(r"; tunnels? leads? to valves? ", line)
    x = front.split(" ")[1]
    rates[x] = int(front.split("=")[-1])
    graph[x] = back.split(", ")

nodeId = defaultdict(lambda: len(nodeId))
[nodeId[u] for u in rates if rates[u]]  # only assign consecutive ids to non-zero rates
ALL_MASK = (1 << len(nodeId)) - 1

cache = defaultdict(lambda: [[-1 for mask in range(ALL_MASK + 1)] for t in range(31)])


def dp(u, t, mask):
    if t == 0:
        return 0
    if cache[u][t][mask] == -1:
        best = max(dp(v, t - 1, mask) for v in graph[u])
        bit = 1 << nodeId[u]
        if bit & mask:
            best = max(best, dp(u, t - 1, mask - bit) + rates[u] * (t - 1))
        cache[u][t][mask] = best
    return cache[u][t][mask]


print("Part1", dp("AA", 30, ALL_MASK))
print("Part2", max(dp("AA", 26, ALL_MASK - mask) + dp("AA", 26, mask) for mask in range(ALL_MASK + 1)))
