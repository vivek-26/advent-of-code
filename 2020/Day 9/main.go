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
	target := partOne(parseInput())
	partTwo(parseInput(), target)
}

func partOne(numbers []int) int {
	preambleLen := 25
	preamble := numbers[0:preambleLen]

	for i := preambleLen; i < len(numbers); i++ {
		if checkXMASProperty(preamble, numbers[i]) {
			preamble = numbers[i-preambleLen+1 : i+1]
		} else {
			fmt.Println(numbers[i])
			return numbers[i]
		}
	}
	return 1
}

func partTwo(numbers []int, target int) {
	ws := 0
	we := 1
	wsum := numbers[ws] + numbers[we]
	for we < len(numbers) {
		if wsum < target {
			we++
			wsum = wsum + numbers[we]
		} else if wsum > target {
			wsum = wsum - numbers[ws]
			ws++
		} else {
			window := numbers[ws : we+1]
			sort.Ints(window)
			fmt.Println(window[0] + window[len(window)-1])
			break
		}
	}
}

func checkXMASProperty(preamble []int, number int) bool {
	preambleMap := make(map[int]struct{})
	for i := 0; i < len(preamble); i++ {
		if _, ok := preambleMap[number-preamble[i]]; !ok {
			preambleMap[preamble[i]] = struct{}{}
		} else {
			return true
		}
	}
	return false
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
