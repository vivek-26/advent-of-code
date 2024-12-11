from functools import cache


with open("input.txt") as input_file:
    stones = input_file.read().split()


@cache
def blink(stone, i):
    if i == 0:
        return 1
    if stone == 0:
        return blink(1, i - 1)
    if len(s := str(stone)) % 2 == 0:
        return blink(int(s[: len(s) // 2]), i - 1) + blink(int(s[len(s) // 2 :]), i - 1)
    return blink(stone * 2024, i - 1)


# part 1
print(sum(blink(int(stone), 25) for stone in stones))


# part 2
print(sum(blink(int(stone), 75) for stone in stones))
