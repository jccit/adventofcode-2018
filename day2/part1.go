package main

import (
	"bufio"
	"log"
	"os"
)

var doubles = 0
var triples = 0

func processLine(line string) {
	var seenLetters = make(map[string]int)
	hasDouble := false
	hasTriple := false

	for _, r := range line {
		letter := string(r)

		num := seenLetters[letter] + 1
		seenLetters[letter] = num
	}

	for _, count := range seenLetters {
		if count == 2 {
			hasDouble = true
		}

		if count == 3 {
			hasTriple = true
		}
	}

	if hasDouble {
		doubles++
	}

	if hasTriple {
		triples++
	}
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		processLine(scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	checksum := doubles * triples
	println(checksum)
}
