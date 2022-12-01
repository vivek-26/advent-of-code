import heapq


class Solution:
    def __init__(self):
        # calorie cannot be negative values
        self.max: int = -1
        self.calories_min_heap: list[int] = [-1]

        calorie: int = 0
        for line in open("input.txt", "r"):
            if line != "\n":
                calorie += int(line)
            else:
                self.process_calorie(calorie)
                calorie = 0  # reset

        if calorie != 0:  # edge case to handle last line
            self.process_calorie(calorie)

    def process_calorie(self, calorie: int):
        self.max = max(self.max, calorie)

        # maintain only top 3 largest values in heap
        if calorie > self.calories_min_heap[0]:
            heapq.heappush(self.calories_min_heap, calorie)

        while len(self.calories_min_heap) > 3:
            heapq.heappop(self.calories_min_heap)

    def part_one(self) -> int:
        return self.max

    def part_two(self) -> int:
        total_calories: int = 0

        while self.calories_min_heap:
            total_calories += heapq.heappop(self.calories_min_heap)

        return total_calories


def main():
    solution = Solution()
    print(solution.part_one())
    print(solution.part_two())


if __name__ == "__main__":
    main()
