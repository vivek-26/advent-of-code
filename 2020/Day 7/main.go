package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// Bag ...
type Bag struct {
	Color string
	Holds []*BagHolds
}

// BagHolds ...
type BagHolds struct {
	Color    string
	Quantity int
}

func main() {
	partOne(parseInput())
	partTwo(parseInput())
}

func partOne(bagList []*Bag) {
	target := "shinygold"
	bagCache := make(map[string]bool)
	count := 0
	bags := make(map[string][]*BagHolds)
	for _, bag := range bagList {
		bags[bag.Color] = bag.Holds
	}
	for _, bag := range bagList {
		if searchBag(target, bag.Color, bags, bagCache) {
			count++
		}
	}
	fmt.Println(count)
}

func partTwo(bagList []*Bag) {
	bagCount := 0
	bags := make(map[string][]*BagHolds)
	for _, bag := range bagList {
		bags[bag.Color] = bag.Holds
	}

	shinyGoldBagHolds := bags["shinygold"]
	for _, innerBag := range shinyGoldBagHolds {
		bagCount += innerBag.Quantity
		bagCount += (innerBag.Quantity * getBagCount(innerBag.Color, bags))
	}
	fmt.Println(bagCount)
}

func getBagCount(target string, bags map[string][]*BagHolds) int {
	if len(bags[target]) == 0 {
		return 0
	}

	bagCount := 0
	for _, innerBag := range bags[target] {
		bagCount += innerBag.Quantity
		bagCount += (innerBag.Quantity * getBagCount(innerBag.Color, bags))
	}
	return bagCount
}

func searchBag(target, outerBag string, bags map[string][]*BagHolds, bagCache map[string]bool) bool {
	// search in cache
	if contains, ok := bagCache[outerBag]; ok {
		return contains
	}

	if len(bags[outerBag]) == 0 {
		bagCache[outerBag] = false
		return false
	}

	for _, innerBag := range bags[outerBag] {
		if innerBag.Color == target {
			bagCache[outerBag] = true
			return true
		}
	}

	for _, innerBag := range bags[outerBag] {
		if searchBag(target, innerBag.Color, bags, bagCache) == true {
			return true
		}
	}

	bagCache[outerBag] = false
	return false
}

func parseInput() []*Bag {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	bags := make([]*Bag, 0, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		line = line[:len(line)-1] // remove full stop
		parts := strings.Split(line, " bags contain ")

		bag := &Bag{
			Color: strings.Join(strings.Split(parts[0], " "), ""),
			Holds: []*BagHolds{},
		}

		if strings.Contains(parts[1], "no ") {
			bags = append(bags, bag)
			continue
		}

		innerBags := strings.Split(parts[1], ",")
		for _, innerBag := range innerBags {

			innerBag = strings.TrimSpace(innerBag)
			details := strings.Split(innerBag, " ")
			details = details[:len(details)-1]

			qty, err := strconv.Atoi(details[0])
			if err != nil {
				log.Fatal("invalid quantity, line: ", line)
			}

			bag.Holds = append(bag.Holds, &BagHolds{
				Color:    strings.Join(details[1:], ""),
				Quantity: qty,
			})
		}
		bags = append(bags, bag)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return bags
}

func printBag(bag *Bag) {
	fmt.Printf("Color: %v, Holds: ", bag.Color)
	fmt.Printf("[")
	for _, innerBag := range bag.Holds {
		fmt.Printf("%v %v, ", innerBag.Quantity, innerBag.Color)
	}
	fmt.Printf("]\n")
}
