from collections import defaultdict


with open("input.txt") as input_file:
    ordering_rules, updates_str = input_file.read().split("\n\n")


updates = [list(map(int, line.split(","))) for line in updates_str.splitlines()]
ordering_rules_map: dict[int, list[int]] = defaultdict(list)

for rule in ordering_rules.splitlines():
    before, after = rule.split("|")
    ordering_rules_map[int(before)].append(int(after))

part1 = 0
part2 = 0

for pages in updates:
    sorted_pages = sorted(
        pages,
        key=lambda page: -len(
            [order for order in ordering_rules_map[page] if order in pages]
        ),
    )
    if pages == sorted_pages:
        part1 += pages[len(pages) // 2]
    else:
        part2 += sorted_pages[len(sorted_pages) // 2]


print(part1)
print(part2)
