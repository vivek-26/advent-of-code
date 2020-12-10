package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	partOne(input())
	partTwo(input())
}

func partOne(areaMap []string) {
	fmt.Println(countTrees(areaMap, 3, 1))
}

func partTwo(areaMap []string) {
	fmt.Println(
		countTrees(areaMap, 1, 1) *
			countTrees(areaMap, 3, 1) *
			countTrees(areaMap, 5, 1) *
			countTrees(areaMap, 7, 1) *
			countTrees(areaMap, 1, 2),
	)
}

func countTrees(areaMap []string, right, down int) int {
	level, pos, trees := 0, 0, 0
	for level < len(areaMap)-1 {
		pos = moveRightCircular(pos, right, len(areaMap[level]))
		level += down
		if areaMap[level][pos:pos+1] == "#" {
			trees++
		}
	}
	return trees
}

func moveRightCircular(current, steps, length int) int {
	return (current + steps) % length
}

func input() []string {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data := make([]string, 0, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		data = append(data, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return data
}
