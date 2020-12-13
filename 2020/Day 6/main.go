package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	partOne(input())
	partTwo(input())
}

func partOne(groups []string) {
	countMap := make(map[string]struct{})
	sum := 0
	for _, group := range groups {
		for i := 0; i < len(group); i++ {
			countMap[group[i:i+1]] = struct{}{}
		}
		if _, ok := countMap[" "]; ok {
			sum += len(countMap) - 1
		} else {
			sum += len(countMap)
		}
		countMap = make(map[string]struct{})
	}
	fmt.Println(sum)
}

func partTwo(groups []string) {
	sum := 0
	for _, group := range groups {
		persons := strings.Split(group, " ")
		if len(persons) == 1 {
			sum += len(persons[0])
		} else {
			countMap := make(map[string]int)
			for _, person := range persons {
				for i := 0; i < len(person); i++ {
					countMap[person[i:i+1]]++
				}
			}
			for _, count := range countMap {
				if count == len(persons) {
					sum++
				}
			}
		}
	}
	fmt.Println(sum)
}

func input() []string {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data := make([]string, 0, 0)
	group := make([]string, 0, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		if text != "" {
			group = append(group, text)
		} else {
			data = append(data, strings.Join(group, " "))
			group = make([]string, 0, 0) // reset
		}
	}
	data = append(data, strings.Join(group, " "))

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return data
}
