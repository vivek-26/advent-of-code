from collections import deque


class Solution:
    def __init__(self):
        self.stack = [  # hard code initial stack from input
            [s for s in "CZNBMWQV"],
            [s for s in "HZRWCB"],
            [s for s in "FQRJ"],
            [s for s in "ZSWHFNMT"],
            [s for s in "GFWLNQP"],
            [s for s in "LPW"],
            [s for s in "VBDRGCQJ"],
            [s for s in "ZQNBW"],
            [s for s in "HLFCGTJ"],
        ]

    def solve(self, part):
        
        for line in open("input.txt"):
            instruction = line.strip().split(" ")
            move = int(instruction[1])
            frm = int(instruction[3]) - 1
            to = int(instruction[5]) - 1

            if part == 1:
                for i in range(move):
                    self.stack[to].append(self.stack[frm].pop())
            elif part == 2:
                elem = deque()
                for i in range(move):
                    elem.appendleft(self.stack[frm].pop())

                for i in range(len(elem)):
                    self.stack[to].append(elem[i])

        result = "".join([self.stack[i].pop() for i in range(len(self.stack))])
        print(result)


def main():
    Solution().solve(1)
    Solution().solve(2)


if __name__ == "__main__":
    main()
