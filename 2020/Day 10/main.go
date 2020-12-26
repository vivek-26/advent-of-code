package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	partOne(parseInput())
	partTwo(parseInput())
}

func partOne(outputJolts []int) {
	sort.Ints(outputJolts)
	outputJolts = append([]int{0}, outputJolts...)

	diffOneJolt := 0
	diffThreeJolts := 1
	for i := 1; i < len(outputJolts); i++ {
		currDiff := outputJolts[i] - outputJolts[i-1]
		switch currDiff {
		case 1:
			diffOneJolt++
		case 3:
			diffThreeJolts++
		default:
		}
	}
	fmt.Println(diffOneJolt * diffThreeJolts)
}

func partTwo(outputJolts []int) {
	sort.Ints(outputJolts)
	outputJolts = append([]int{0}, outputJolts...)
	outputJolts = append(outputJolts, outputJolts[len(outputJolts)-1]+3)
	fmt.Println(arrangements(outputJolts, 0, make(map[int]int)))
}

func arrangements(outputJolts []int, pos int, cache map[int]int) int {
	if pos == len(outputJolts)-1 {
		return 1
	}

	if ways, ok := cache[pos]; ok {
		return ways
	}

	numWays := 0
	if pos+1 < len(outputJolts) && outputJolts[pos+1]-outputJolts[pos] <= 3 {
		numWays = numWays + arrangements(outputJolts, pos+1, cache)
	}
	if pos+2 < len(outputJolts) && outputJolts[pos+2]-outputJolts[pos] <= 3 {
		numWays = numWays + arrangements(outputJolts, pos+2, cache)
	}
	if pos+3 < len(outputJolts) && outputJolts[pos+3]-outputJolts[pos] <= 3 {
		numWays = numWays + arrangements(outputJolts, pos+3, cache)
	}

	cache[pos] = numWays
	return numWays
}

func parseInput() []int {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data := make([]int, 0, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		numStr := scanner.Text()
		num, err := strconv.Atoi(numStr)
		if err != nil {
			log.Fatal("invalid input: ", numStr)
		}
		data = append(data, num)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return data
}
