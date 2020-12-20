package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// Operation ...
type Operation string

const (
	acc Operation = "acc"
	jmp Operation = "jmp"
	nop Operation = "nop"
)

// Instruction ...
type Instruction struct {
	Op  Operation
	Arg int
}

func main() {
	partOne(parseInput())
	partTwo(parseInput())
}

func partOne(instructions []*Instruction) {
	accumulator, _ := move(instructions)
	fmt.Println(accumulator)
}

func partTwo(instructions []*Instruction) {
	for _, instruction := range instructions {
		if instruction.Op == jmp || instruction.Op == nop {
			prevOp := instruction.Op
			instruction.Op = flipOp(prevOp)
			if accumulator, hasLoop := move(instructions); hasLoop {
				instruction.Op = prevOp
			} else {
				fmt.Println(accumulator)
				break
			}
		}
	}
}

func flipOp(op Operation) Operation {
	switch op {
	case jmp:
		return nop
	case nop:
		return jmp
	default:
		return op
	}
}

func move(instructions []*Instruction) (int, bool) {
	seen := make(map[int]struct{})
	hasLoop := false
	accumulator := 0
	line := 0
	for line >= 0 && line < len(instructions) {
		if _, ok := seen[line]; ok {
			hasLoop = true
			break // infinite loop found
		}

		seen[line] = struct{}{}
		switch inst := instructions[line]; inst.Op {
		case acc:
			accumulator += inst.Arg
			line++
		case jmp:
			line += inst.Arg
		case nop:
			line++
		}
	}

	return accumulator, hasLoop
}

func parseInput() []*Instruction {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	instructions := make([]*Instruction, 0, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		fields := strings.Split(line, " ")
		if len(fields) != 2 {
			log.Fatal("invalid input: ", line)
		}

		arg, err := strconv.Atoi(fields[1])
		if err != nil {
			log.Fatal("invalid argument: ", fields[1])
		}

		instruction := &Instruction{Op: Operation(fields[0]), Arg: arg}
		instructions = append(instructions, instruction)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return instructions
}
