package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"time"

	"github.com/antchfx/htmlquery"
)

var (
	pattern    []int = []int{0, 8, 16, 24, 32, 40, 48, 54, 60, 66, 72, 78}
	patternLen int   = 84
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("usage: estimate username")
	}

	current := getCurrent(os.Args[1])
	target := getTarget()

	if target > current {
		fmt.Println(target - current)
	} else {
		fmt.Println(0)
	}
}

func getTarget() int {
	start := time.Date(2020, 1, 5, 0, 0, 0, 0, time.UTC)
	offset := (int(time.Since(start).Hours()) / 24) % patternLen

	if contains(pattern, offset) {
		return 10
	} else {
		return 4
	}
}

func getCurrent(username string) int {
	doc, err := htmlquery.LoadURL("https://github.com/" + username)
	if err != nil {
		log.Fatal(err)
	}

	rect := htmlquery.FindOne(doc, "//g[last()]/rect[last()]")
	attr := htmlquery.SelectAttr(rect, "data-count")
	n, err := strconv.Atoi(attr)
	if err != nil {
		log.Fatal(err)
	}

	return n
}

func contains(s []int, n int) bool {
	for _, m := range s {
		if m == n {
			return true
		}
	}
	return false
}
