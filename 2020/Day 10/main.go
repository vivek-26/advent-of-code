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
