package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
)

func main() {
	seatIDs := partOne(input())
	partTwo(seatIDs)
}

func partOne(passes []string) []int {
	highestSeatID := 0
	seatIDs := make([]int, 0, 0)
	for _, pass := range passes {
		if len(pass) != 10 {
			log.Fatal("invalid input:", pass)
		}

		row := binaryPartition(pass[:7], "F", "B", false)
		col := binaryPartition(pass[7:], "L", "R", true)
		seatID := (row * 8) + col
		seatIDs = append(seatIDs, seatID)
		highestSeatID = max(highestSeatID, seatID)
	}
	fmt.Println(highestSeatID)
	return seatIDs
}

func partTwo(seatIDs []int) {
	sort.Ints(seatIDs)
	missingSeatID := 0
	for i := 1; i < len(seatIDs); i++ {
		if seatIDs[i]-seatIDs[i-1] == 2 {
			missingSeatID = seatIDs[i] - 1
			break
		}
	}

	fmt.Println(missingSeatID)
}

func binaryPartition(data, lowerHalf, upperHalf string, isCol bool) int {
	min := 0
	max := 127
	if isCol {
		max = 7
	}
	length := len(data)

	for i := 0; i < length-1; i++ {
		switch data[i : i+1] {
		case lowerHalf:
			max = (max + min) / 2
		case upperHalf:
			min = ((min + max) / 2) + 1
		}
	}
	if data[length-1:length] == lowerHalf {
		return min
	}
	return max
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
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
