package main

import (
	"fmt"
	"log"
	"os"

	"github.com/antchfx/htmlquery"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("Bad!")
	}

	username := os.Args[1]

	doc, err := htmlquery.LoadURL("https://github.com/" + username)
	if err != nil {
		panic(err)
	}

	rect := htmlquery.FindOne(doc, "//g[last()]/rect[last()]")
	fmt.Println(htmlquery.SelectAttr(rect, "data-count"))
}
