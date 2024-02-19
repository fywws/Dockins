package main

import (
	"fmt"
	"os"
	"strings"
)
func main() {
	args := os.Args 
	lastArgumentIndex := len(args)-1 
	wordToSay := strings.TrimSpace(args[lastArgumentIndex])
	fmt.Println("Hello "+wordToSay+"!") 
}