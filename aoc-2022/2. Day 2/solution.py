class Solution:
    def __init__(self):
        self.rounds: list[str] = []

        for line in open("input.txt", "r"):
            self.rounds.append(line.replace(" ", "").strip())

        self.strategy: dict[str, int] = {
            "AX": 3, "AY": 6, "AZ": 0,
            "BX": 0, "BY": 3, "BZ": 6,
            "CX": 6, "CY": 0, "CZ": 3,
        }

        self.score_me: dict[str, int] = {"X": 1, "Y": 2, "Z": 3}
        self.score_op: dict[str, int] = {"A": 1, "B": 2, "C": 3}

        self.defeats: dict[str, str] = {"A": "C", "C": "B", "B": "A"}
        self.losses_to: dict[str, str] = {v: k for k, v in self.defeats.items()}  # reverse map

    def part_one(self):
        score: int = 0

        for r in self.rounds:
            score += self.score_me[r[1]] + self.strategy[r]

        print(score)

    def part_two(self):
        score: int = 0

        for r in self.rounds:
            if r[1] == "X":
                score += 0 + self.score_op[self.defeats[r[0]]]
            elif r[1] == "Y":
                score += 3 + self.score_op[r[0]]
            elif r[1] == "Z":
                score += 6 + self.score_op[self.losses_to[r[0]]]

        print(score)


def main():
    solution = Solution()
    solution.part_one()
    solution.part_two()


if __name__ == "__main__":
    main()
