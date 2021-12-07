package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	input, err := ReadInput("input2.txt")
	if err != nil {
		panic(err)
	}

	var vents []Vent
	for _, line := range input {
		vent, err := VentFromString(line)
		if err != nil {
			panic(err)
		}
		vents = append(vents, *vent)
	}

	ventField := NewVentField(VentBounds(vents))
	for _, v := range vents {
		err = ventField.AddVent(v)
		if err != nil {
			panic(err)
		}
	}

	for _, row := range ventField {
		fmt.Println(row)
	}

	points := ventField.PointsAboveThreshold(1)
	fmt.Println(points)
	fmt.Println(len(points))
}

// ReadInput reads from a file and returns the contents.
func ReadInput(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return []string{}, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines, nil
}

// VentBounds takes a list of vents and finds the maxima of their x and y values.
func VentBounds(vents []Vent) (int, int) {
	var xBound, yBound int

	for _, v := range vents {
		if v.X1 > xBound {
			xBound = v.X1
		}
		if v.X2 > xBound {
			xBound = v.X2
		}
		if v.Y1 > yBound {
			yBound = v.Y1
		}
		if v.Y2 > yBound {
			yBound = v.Y2
		}
	}

	return yBound, xBound
}
