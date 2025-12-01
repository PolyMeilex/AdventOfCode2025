package day1

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Direction uint64

const (
	Left Direction = iota
	Right
)

func (d Direction) String() string {
	switch d {
	case Left:
		return "Left"
	case Right:
		return "Right"
	default:
		return "Unknown"
	}
}

func parseDirection(s string) Direction {
	switch s {
	case "L":
		return Left
	case "R":
		return Right
	default:
		panic("unreachable: invalid direction")
	}
}

type Command struct {
	value     uint64
	direction Direction
}

type Lock struct {
	value uint64
}

func (d *Lock) left() {
	if d.value == 0 {
		d.value = 99
	} else {
		d.value -= 1
	}
}

func (d *Lock) right() {
	d.value = (d.value + 1) % 100
}

func Run(part2 bool) {
	dat, err := os.ReadFile("./day1/input.txt")
	if err != nil {
		panic(err)
	}

	src := string(dat)
	src = strings.TrimSpace(src)

	lines := strings.Split(src, "\n")

	commands := make([]Command, len(lines))
	for i, line := range lines {
		direction := parseDirection(line[:1])
		value_str := line[1:]

		value, err := strconv.ParseUint(value_str, 10, 64)
		if err != nil {
			panic(err)
		}

		commands[i] = Command{value, direction}
	}

	lock := Lock{
		value: 50,
	}

	count := 0

	if part2 {
		for _, command := range commands {
			for range command.value {
				switch command.direction {
				case Left:
					lock.left()
				case Right:
					lock.right()
				}

				if lock.value == 0 {
					count += 1
				}
			}
		}
	} else {
		for _, command := range commands {
			for range command.value {
				switch command.direction {
				case Left:
					lock.left()
				case Right:
					lock.right()
				}
			}

			if lock.value == 0 {
				count += 1
			}
		}
	}

	fmt.Println(count)
}
