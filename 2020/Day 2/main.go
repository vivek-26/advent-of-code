package main

import (
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"
)

type policy struct {
	min, max int
	char     string
}

type password struct {
	data   string
	policy *policy
}

func main() {
	partOne(input())
	partTwo(input())
}

func partOne(list []string) {
	validPassCount := 0
	passwd := &password{data: "", policy: &policy{}}
	var err error
	for _, element := range list {
		if err = parse(element, passwd); err != nil {
			log.Fatalf("invalid input: %v", element)
		}
		if partOneIsValid(passwd) {
			validPassCount++
		}
	}
	fmt.Println(validPassCount)
}

func partTwo(list []string) {
	validPassCount := 0
	passwd := &password{data: "", policy: &policy{}}
	var err error
	for _, element := range list {
		if err = parse(element, passwd); err != nil {
			log.Fatalf("invalid input: %v", element)
		}
		if partTwoIsValid(passwd) {
			validPassCount++
		}
	}
	fmt.Println(validPassCount)
}

func parse(data string, passwd *password) error {
	r := regexp.MustCompile(`(?m)^(\d*)-(\d*)\s([a-z]):\s([a-z]*)$`)
	match := r.FindAllStringSubmatch(data, -1)
	for i := range match {
		min, err := strconv.Atoi(match[i][1])
		if err != nil {
			return err
		}
		passwd.policy.min = min

		max, err := strconv.Atoi(match[i][2])
		if err != nil {
			return err
		}
		passwd.policy.max = max

		passwd.policy.char = match[i][3]
		passwd.data = match[i][4]
	}
	return nil
}

func partOneIsValid(passwd *password) bool {
	count := strings.Count(passwd.data, passwd.policy.char)
	return count >= passwd.policy.min && count <= passwd.policy.max
}

func partTwoIsValid(passwd *password) bool {
	matchCount := 0
	if passwd.data[passwd.policy.min-1:passwd.policy.min] == passwd.policy.char {
		matchCount++
	}
	if passwd.data[passwd.policy.max-1:passwd.policy.max] == passwd.policy.char {
		matchCount++
	}
	return matchCount == 1
}
