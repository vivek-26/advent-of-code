from collections import deque
from dataclasses import dataclass
from math import prod, lcm


@dataclass
class Monkey:
    items: deque
    op: str
    div: int
    test_t: int
    test_f: int
    items_inspected: int


def parse_input() -> list[Monkey]:
    monkeys: list[Monkey] = []

    with open("input.txt", "r") as f:
        lines = [s.strip() for s in f.readlines() if s != "\n"]
    k = 0
    while k + 6 <= len(lines):
        _, items, op, div, test_t, test_f = lines[k:k + 6]
        k = k + 6

        items = deque(list(map(int, items.replace("Starting items:", "").strip().split(","))))
        op = op.split("=")[1].strip()
        div = int(div.split("by")[1].strip())
        test_t = int(test_t.split("monkey")[1].strip())
        test_f = int(test_f.split("monkey")[1].strip())

        monkeys.append(Monkey(items, op, div, test_t, test_f, 0))

    return monkeys


# part one and two
parts = [1, 2]
for part in parts:
    rounds = 20 if part == 1 else 10000

    monkeys = parse_input()
    divisor = prod([lcm(mk.div) for mk in monkeys])

    for i in range(rounds):
        for idx, mk in enumerate(monkeys):
            for item in mk.items:
                mk.items_inspected += 1
                worry = eval(mk.op.replace("old", "item"))
                if part == 1:
                    worry = worry // 3
                else:
                    worry = worry % divisor
                if worry % mk.div == 0:
                    monkeys[mk.test_t].items.append(worry)
                else:
                    monkeys[mk.test_f].items.append(worry)
            mk.items = deque()

    x, y = sorted([x.items_inspected for x in monkeys], reverse=True)[:2]
    print(x * y)
