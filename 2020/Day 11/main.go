package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

// Position ...
type Position string

// Grid positions
const (
	Floor    Position = "."
	Empty    Position = "L"
	Occupied Position = "#"
)

// Layout ...
type Layout struct {
	Row, Col int
	Data     Position
}

func main() {
	partOne(parseInput())
	partTwo(parseInput())
}

func partOne(seats [][]Position, row int, col int) {
	rounds := 1
	for applyRules(seats, row, col, true) {
		rounds++
	}

	numOccupied := 0
	for i := range seats {
		for j := range seats[i] {
			if seats[i][j] == Occupied {
				numOccupied++
			}
		}
	}
	fmt.Println(rounds, numOccupied)
}

func partTwo(seats [][]Position, row int, col int) {
	rounds := 1
	for applyRules(seats, row, col, false) {
		rounds++
	}

	numOccupied := 0
	for i := range seats {
		for j := range seats[i] {
			if seats[i][j] == Occupied {
				numOccupied++
			}
		}
	}
	fmt.Println(rounds, numOccupied)
}

func applyRules(seats [][]Position, row int, col int, isPartOne bool) bool {
	layoutChanges := make([]*Layout, 0, 0)

	for i := range seats {
		for j := range seats[i] {
			switch seats[i][j] {
			case Floor:
			case Empty:
				neighbourOccupied := false
				var neighbours [][]int
				if isPartOne {
					neighbours = getNeighboursPartOne(i, j, row, col)
				} else {
					neighbours = getNeighboursPartTwo(seats, i, j, row, col)
				}
				for nl := range neighbours {
					ni, nj := neighbours[nl][0], neighbours[nl][1]
					if seats[ni][nj] == Occupied {
						neighbourOccupied = true
						break
					}
				}
				if !neighbourOccupied {
					layoutChanges = append(layoutChanges, &Layout{i, j, Occupied})
				}
			case Occupied:
				numOccupied := 0
				var neighbours [][]int
				if isPartOne {
					neighbours = getNeighboursPartOne(i, j, row, col)
				} else {
					neighbours = getNeighboursPartTwo(seats, i, j, row, col)
				}
				for nl := range neighbours {
					ni, nj := neighbours[nl][0], neighbours[nl][1]
					if seats[ni][nj] == Occupied {
						numOccupied++
					}
				}

				var limit int
				if isPartOne {
					limit = 4
				} else {
					limit = 5
				}
				if numOccupied >= limit {
					layoutChanges = append(layoutChanges, &Layout{i, j, Empty})
				}
			}
		}
	}

	if len(layoutChanges) > 0 {
		for _, layout := range layoutChanges {
			seats[layout.Row][layout.Col] = layout.Data
		}
		return true
	}

	return false
}

func getNeighboursPartOne(i, j, r, c int) [][]int {
	neighbours := make([][]int, 0, 0)

	if i-1 >= 0 && i-1 < r && j-1 >= 0 && j-1 < c {
		neighbours = append(neighbours, []int{i - 1, j - 1})
	}

	if i-1 >= 0 && i-1 < r && j >= 0 && j < c {
		neighbours = append(neighbours, []int{i - 1, j})
	}

	if i-1 >= 0 && i-1 < r && j+1 >= 0 && j+1 < c {
		neighbours = append(neighbours, []int{i - 1, j + 1})
	}

	if i >= 0 && i < r && j+1 >= 0 && j+1 < c {
		neighbours = append(neighbours, []int{i, j + 1})
	}

	if i+1 >= 0 && i+1 < r && j+1 >= 0 && j+1 < c {
		neighbours = append(neighbours, []int{i + 1, j + 1})
	}

	if i+1 >= 0 && i+1 < r && j >= 0 && j < c {
		neighbours = append(neighbours, []int{i + 1, j})
	}

	if i+1 >= 0 && i+1 < r && j-1 >= 0 && j-1 < c {
		neighbours = append(neighbours, []int{i + 1, j - 1})
	}

	if i >= 0 && i < r && j-1 >= 0 && j-1 < c {
		neighbours = append(neighbours, []int{i, j - 1})
	}

	return neighbours
}

func getNeighboursPartTwo(seats [][]Position, m, n, r, c int) [][]int {
	neighbours := make([][]int, 0, 0)

	i, j := m, n
	for i-1 >= 0 && i-1 < r && j-1 >= 0 && j-1 < c {
		if seats[i-1][j-1] != Floor {
			neighbours = append(neighbours, []int{i - 1, j - 1})
			break
		}
		i = i - 1
		j = j - 1
	}

	i, j = m, n
	for i-1 >= 0 && i-1 < r && j >= 0 && j < c {
		if seats[i-1][j] != Floor {
			neighbours = append(neighbours, []int{i - 1, j})
			break
		}
		i = i - 1
	}

	i, j = m, n
	for i-1 >= 0 && i-1 < r && j+1 >= 0 && j+1 < c {
		if seats[i-1][j+1] != Floor {
			neighbours = append(neighbours, []int{i - 1, j + 1})
			break
		}
		i = i - 1
		j = j + 1
	}

	i, j = m, n
	for i >= 0 && i < r && j+1 >= 0 && j+1 < c {
		if seats[i][j+1] != Floor {
			neighbours = append(neighbours, []int{i, j + 1})
			break
		}
		j = j + 1
	}

	i, j = m, n
	for i+1 >= 0 && i+1 < r && j+1 >= 0 && j+1 < c {
		if seats[i+1][j+1] != Floor {
			neighbours = append(neighbours, []int{i + 1, j + 1})
			break
		}
		i = i + 1
		j = j + 1
	}

	i, j = m, n
	for i+1 >= 0 && i+1 < r && j >= 0 && j < c {
		if seats[i+1][j] != Floor {
			neighbours = append(neighbours, []int{i + 1, j})
			break
		}
		i = i + 1
	}

	i, j = m, n
	for i+1 >= 0 && i+1 < r && j-1 >= 0 && j-1 < c {
		if seats[i+1][j-1] != Floor {
			neighbours = append(neighbours, []int{i + 1, j - 1})
			break
		}
		i = i + 1
		j = j - 1
	}

	i, j = m, n
	for i >= 0 && i < r && j-1 >= 0 && j-1 < c {
		if seats[i][j-1] != Floor {
			neighbours = append(neighbours, []int{i, j - 1})
			break
		}
		j = j - 1
	}

	return neighbours
}

func parseInput() ([][]Position, int, int) {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data := make([][]Position, 0, 0)
	row, col := 0, 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.Split(strings.TrimSpace(scanner.Text()), "")
		posLine := make([]Position, 0, len(line))
		for _, elem := range line {
			posLine = append(posLine, Position(elem))
		}
		data = append(data, posLine)
		row++
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	col = len(data[0])
	return data, row, col
}
