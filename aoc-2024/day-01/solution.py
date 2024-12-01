from collections import Counter

with open("input.txt") as input_file:
    groups = [list(map(int, line.strip().split())) for line in input_file]

group_left = sorted(group[0] for group in groups)
group_right = sorted(group[1] for group in groups)

# part 1
distance = sum(abs(right - left) for left, right in zip(group_left, group_right))
print(distance)

# part 2
group_right_counter = Counter(group_right)
similarity_score = sum(left * group_right_counter[left] for left in group_left)
print(similarity_score)
