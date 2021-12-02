package main

import (
	"errors"
	"strconv"
	"strings"
)

type Direction struct {
	Heading  string
	Distance int
}

func DirectionFromString(s string) (Direction, error) {
	parts := strings.Fields(s)

	if len(parts) != 2 {
		return Direction{}, errors.New("Input not split into two parts.")
	}

	heading := parts[0]

	distance, err := strconv.Atoi(parts[1])
	if err != nil {
		return Direction{}, err
	}

	return Direction{
		Heading:  heading,
		Distance: distance,
	}, nil
}
