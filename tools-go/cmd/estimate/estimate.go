package main

import (
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/antchfx/htmlquery"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("usage: estimate username")
	}

	username := os.Args[1]

	fmt.Println(getCurrent(username))
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
