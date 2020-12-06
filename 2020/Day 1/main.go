package main

import (
	"fmt"
	"log"
	"sort"
)

func main() {
	partOne(input())
	partTwo(input())
}

func partOne(entries []int) {
	seen := make(map[int]struct{})
	for _, entry := range entries {
		if _, ok := seen[2020-entry]; ok {
			fmt.Println(entry * (2020 - entry))
			return
		}
		seen[entry] = struct{}{}
	}
	log.Fatal("invalid input")
}

func partTwo(entries []int) {
	sort.Ints(entries)
	for i := 0; i < len(entries)-2; i++ {
		left := i + 1
		right := len(entries) - 1
		for left < right {
			if entries[left]+entries[right]+entries[i] == 2020 {
				fmt.Println(entries[left] * entries[right] * entries[i])
				return
			} else if entries[left]+entries[right]+entries[i] < 2020 {
				left++
			} else {
				right--
			}
		}
	}

	log.Fatal("invalid input")
}
