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
	partOne(parseInput(true))
	partTwo(parseInput(false))
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

func partTwo(est int, buses []int) {
	time := 0
	stepSize := buses[0]

	for i := 1; i < len(buses); i++ {
		bus := buses[i]
		for (time+i)%bus != 0 {
			time = time + stepSize
		}

		stepSize = stepSize * bus
	}

	fmt.Println(time)
}

func parseInput(isPartOne bool) (int, []int) {
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
					if !isPartOne {
						buses = append(buses, 1)
					}
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
