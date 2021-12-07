package main

import (
	"fmt"
	"strconv"
	"strings"
)

// Orientations of vents
const (
	Horizontal = "horizontal"
	Vertical   = "vertical"
	Diagonal   = "diagonal"
)

// A vent spans between two points and spews dangerously hot water that should be avoided.
type Vent struct {
	// One end of the vent.
	X1, Y1 int
	// The other end of the vent.
	X2, Y2 int
}

func (v *Vent) Orientation() string {
	if v.X1 == v.X2 {
		return Vertical
	}

	if v.Y1 == v.Y2 {
		return Horizontal
	}

	return Diagonal
}

func VentFromString(s string) (*Vent, error) {
	ends := strings.Split(s, " -> ")
	if len(ends) != 2 {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	start := strings.Split(ends[0], ",")
	if len(start) != 2 {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	end := strings.Split(ends[1], ",")
	if len(end) != 2 {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	X1, err := strconv.Atoi(start[0])
	if err != nil {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	Y1, err := strconv.Atoi(start[1])
	if err != nil {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	X2, err := strconv.Atoi(end[0])
	if err != nil {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	Y2, err := strconv.Atoi(end[1])
	if err != nil {
		return nil, fmt.Errorf("invalid vent string: %s", s)
	}

	return &Vent{X1, Y1, X2, Y2}, nil
}
