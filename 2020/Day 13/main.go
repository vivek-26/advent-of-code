package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	partOne(parseInput())
}

func partOne(est int, buses []int) {
	smallestDiff := math.MaxInt64
	busID := 0
	for idx, bus := range buses {
		factor := (est / bus) + 1
		if (bus*factor)-est < smallestDiff {
			smallestDiff = (bus * factor) - est
			busID = idx
		}
	}
	fmt.Println(buses[busID] * smallestDiff)
}

func parseInput() (int, []int) {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	earliestTimestamp := 0
	buses := make([]int, 0, 0)
	line := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		if line%2 == 0 {
			var err error
			inputStr := strings.TrimSpace(scanner.Text())
			earliestTimestamp, err = strconv.Atoi(inputStr)
			if err != nil {
				log.Fatal("invalid input: ", inputStr)
			}
		} else {
			inputStr := strings.TrimSpace(scanner.Text())
			ids := strings.Split(inputStr, ",")
			for _, id := range ids {
				switch id {
				case "x":
				default:
					busID, err := strconv.Atoi(id)
					if err != nil {
						log.Fatal("invalid input: ", inputStr)
					}
					buses = append(buses, busID)
				}
			}
		}
		line++
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return earliestTimestamp, buses
}
