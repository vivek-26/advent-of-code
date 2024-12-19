inp = open("input.txt").read().strip()
p1 = 0
p2 = 0
words, targets = inp.split("\n\n")
words = words.split(", ")

DP = {}


def ways(words, target):
    if target in DP:
        return DP[target]
    ans = 0
    if not target:
        ans = 1
    for word in words:
        if target.startswith(word):
            ans += ways(words, target[len(word) :])
    DP[target] = ans
    return ans


for target in targets.split("\n"):
    target_ways = ways(words, target)
    if target_ways > 0:
        p1 += 1
    p2 += target_ways

print(p1)
print(p2)
