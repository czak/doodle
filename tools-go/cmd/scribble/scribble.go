package main

import (
	"bufio"
	"io"
	"log"
	"os"
	"strings"

	"doodle-tools/cmd/scribble/data"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("usage: scribble filename.html")
	}

	filename := os.Args[1]
	file, _ := os.OpenFile(filename, os.O_RDWR|os.O_APPEND, 0)

	lines := countLines(file)
	tlines := countLines(strings.NewReader(data.Template))
	slines := countLines(strings.NewReader(data.Sonnets))

	if lines == tlines+slines {
		os.Truncate(filename, int64(len(data.Template)))
	} else {
		offset := lines - tlines
		line := getLine(data.Sonnets, offset)
		file.WriteString(line + "\n")
	}
}

func countLines(reader io.Reader) int {
	scanner := bufio.NewScanner(reader)
	lines := 0
	for scanner.Scan() {
		lines++
	}
	return lines
}

func getLine(s string, offset int) string {
	reader := strings.NewReader(s)
	scanner := bufio.NewScanner(reader)
	for i := 0; i <= offset; i++ {
		scanner.Scan()
	}
	return scanner.Text()
}
