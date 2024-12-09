with open(
    "/Users/vivek/Documents/github.com/advent-of-code/aoc-2024/day-09/input.txt"
) as input_file:
    disk_map = [int(elem) for elem in input_file.read().strip()]


def build_blocks(disk_map: list[int]) -> list[int]:
    blocks: list[int] = []
    file_id: int = 0
    for idx in range(len(disk_map)):
        if idx % 2 == 0:  # a file
            blocks.extend([file_id] * disk_map[idx])
            file_id += 1
        else:  # empty space
            blocks.extend([-1] * disk_map[idx])

    return blocks


# part 1
def find_compaction_checksum(disk_map: list[int]) -> int:
    blocks = build_blocks(disk_map)
    left, right = 0, len(blocks) - 1
    while left < right:
        while left < right and blocks[left] != -1:  # stop at empty spot
            left += 1

        while left < right and blocks[right] == -1:  # stop at occupied spot
            right -= 1

        # do the swap
        while (
            left < len(blocks)
            and right >= 0
            and blocks[left] == -1
            and blocks[right] != -1
        ):
            blocks[left] = blocks[right]
            blocks[right] = -1
            left += 1
            right -= 1

    return sum([idx * elem for idx, elem in enumerate(blocks) if elem != -1])


print(find_compaction_checksum(disk_map))
