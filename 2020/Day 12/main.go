package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// Direction ...
type Direction string

// All Directions
const (
	North   Direction = "N"
	East    Direction = "E"
	South   Direction = "S"
	West    Direction = "W"
	Left    Direction = "L"
	Right   Direction = "R"
	Forward Direction = "F"
)

// Instruction ...
type Instruction struct {
	Action Direction
	Value  int
}

// Waypoint ...
type Waypoint struct {
	X, Y int
}

func main() {
	partOne(parseInput())
}

func partOne(instructions []*Instruction) {
	fmt.Println(manhattanDistance(instructions))
}

func manhattanDistance(instructions []*Instruction) int {
	directions := []*Instruction{
		{North, 0}, {East, 0}, {South, 0}, {West, 0},
	}
	currDirection := 1 // initial direction of ship is East

	for _, instruction := range instructions {
		switch instruction.Action {
		case North:
			directions[0].Value += instruction.Value
		case East:
			directions[1].Value += instruction.Value
		case South:
			directions[2].Value += instruction.Value
		case West:
			directions[3].Value += instruction.Value
		case Forward:
			directions[currDirection].Value += instruction.Value
		case Left, Right:
			currDirection = newDirection(instruction, currDirection, len(directions))
		}
	}

	return absDiff(directions[0].Value, directions[2].Value) +
		absDiff(directions[1].Value, directions[3].Value)
}

func newDirection(instruction *Instruction, currDirection, totalDirections int) int {
	var newDirection int
	steps := instruction.Value / 90
	switch instruction.Action {
	case Left:
		newDirection = (currDirection - steps) % totalDirections
		if newDirection < 0 {
			newDirection = totalDirections + newDirection
		}
	case Right:
		newDirection = (currDirection + steps) % totalDirections
	}
	return newDirection
}

func absDiff(a, b int) int {
	if a > b {
		return a - b
	}
	return b - a
}

func parseInput() []*Instruction {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data := make([]*Instruction, 0, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		inst := &Instruction{}
		inst.Action = Direction(line[0:1])
		val, err := strconv.Atoi(line[1:])
		if err != nil {
			log.Fatal("invalid input: ", line)
		}

		inst.Value = val
		data = append(data, inst)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return data
}
