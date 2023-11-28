package main

import (
	"flag"
	"fmt"
)

func main() {
	// x, y, op := parse()

	// sum := compute(x, y, op)

	// printResult(x, y, sum, op)

	c := Computer{}
	c.Run()
}

func parse() (x int, y int, op string) {
	px := flag.Int("x", 0, "x")
	py := flag.Int("y", 0, "y")
	pop := flag.String("op", "+", "operation")
	flag.Parse()
	return *px, *py, *pop
}

func compute(x, y int, op string) int {
	var sum int
	switch op {
	case "+":
		sum = x + y
	case "-":
		sum = x - y
	case "*":
		sum = x * y
	case "/":
		sum = x / y
	}
	return sum
}

func printResult(x, y, sum int, op string) {
	fmt.Printf("%d %s %d = %d\n", x, op, y, sum)
}

type Computer struct{
	x, y, result int
	op string
}

func (c *Computer) Run() {
	c.parse()
	c.compute()
	c.printResult()
}

func (c *Computer) parse() {
	px := flag.Int("x", 0, "x")
	py := flag.Int("y", 0, "y")
	pop := flag.String("op", "+", "operation")
	flag.Parse()

	c.x = *px
	c.y = *py
	c.op = *pop
}

func (c *Computer) compute() {
	switch c.op {
	case "+":
		c.result = c.x + c.y
	case "-":
		c.result = c.x - c.y
	case "*":
		c.result = c.x * c.y
	case "/":
		c.result = c.x / c.y
	}
}

func (c *Computer) printResult() {
	fmt.Printf("%d %s %d = %d\n", c.x, c.op, c.y, c.result)
}
