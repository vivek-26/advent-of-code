class Solution:
    def __init__(self):
        self.input = open("input.txt").readline().strip()

    def solve(self, n: int):
        for i in range(len(self.input) - n):
            if len(set(self.input[i:i + n])) == n:
                return i + n


def main():
    print(Solution().solve(4))
    print(Solution().solve(14))


if __name__ == "__main__":
    main()
