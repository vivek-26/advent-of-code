from collections import Counter

group_left: list[int] = []
group_right: list[int] = []

with open("input.txt") as input_file:
    for line in input_file:
        groups = line.strip().split(" ")
        group_left.append(int(groups[0]))
        group_right.append(int(groups[-1]))


# part 1
group_left.sort()
group_right.sort()
assert len(group_left) == len(group_right)

distance = 0
for left, right in zip(group_left, group_right):
    distance += abs(right - left)

print(distance)


# part 2
group_right_counter = Counter(group_right)
similarity_score = 0

for left in group_left:
    similarity_score += left * group_right_counter[left]

print(similarity_score)
