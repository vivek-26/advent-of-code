class Solution:
    def __init__(self):
        self.input: list[str] = []
        for line in open("input.txt", "r"):
            self.input.append(line.strip())

    def part_one(self):
        result: int = 0
        for line in self.input:
            parts = line.split(",")
            elf1, elf2 = list(map(int, parts[0].split("-"))), list(map(int, parts[1].split("-")))
            if elf2[0] >= elf1[0] and elf2[1] <= elf1[1]:
                result = result + 1
            elif elf1[0] >= elf2[0] and elf1[1] <= elf2[1]:
                result = result + 1

        print(result)

    def part_two(self):
        result: int = 0
        for line in self.input:
            parts = line.split(",")
            elf1, elf2 = list(map(int, parts[0].split("-"))), list(map(int, parts[1].split("-")))
            if elf2[0] <= elf1[1] and elf2[1] >= elf1[0]:
                result = result + 1

        print(result)


def main():
    solution = Solution()
    solution.part_one()
    solution.part_two()


if __name__ == "__main__":
    main()
