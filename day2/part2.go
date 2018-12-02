package main

import (
	"bufio"
	"log"
	"os"
	"sort"
)

var ids []string

func findCorrectIDs() (string, string) {
	for _, sampleA := range ids {
		for _, sampleB := range ids {
			if sampleA != sampleB {
				difference := 0

				for i := range sampleA {
					letterA := string(sampleA[i])
					letterB := string(sampleB[i])

					if letterA != letterB {
						difference++
					}
				}

				if difference == 1 {
					return sampleA, sampleB
				}
			}
		}
	}

	return "", ""
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		ids = append(ids, text)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	sort.Strings(ids)

	idA, idB := findCorrectIDs()

	out := ""

	// Find letters that are different
	for i := range idA {
		letterA := string(idA[i])
		letterB := string(idB[i])

		if letterA != letterB {
			println(idA, idB)
		} else {
			out = out + letterA
		}
	}

	println(out)
}
