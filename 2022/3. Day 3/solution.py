class Solution:
    def __init__(self):
        self.rucksacks: list[str] = []
        for line in open("input.txt", "r"):
            self.rucksacks.append(line.strip())

    def part_one(self):
        total: int = 0
        for r in self.rucksacks:
            first, second = r[:len(r) // 2], r[len(r) // 2:]
            common = "".join(set(first).intersection(second))
            if common.islower():
                total += ord(common) - 96
            else:
                total += ord(common) - 38

        print(total)

    def part_two(self):
        total: int = 0
        group: list[str] = []

        for r in self.rucksacks:
            group.append(r)
            if len(group) == 3:
                common = "".join(set(group[0]).intersection(group[1]).intersection(group[2]))
                if common.islower():
                    total += ord(common) - 96
                else:
                    total += ord(common) - 38

                group = []  # reset group
                
        print(total)


def main():
    solution = Solution()
    solution.part_one()
    solution.part_two()


if __name__ == "__main__":
    main()
