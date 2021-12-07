package main

import "fmt"

type VentField [][]int

// AddVent adds a line of vents to the field.
func (vf VentField) AddVent(vent Vent) error {
	if vent.X1 < 0 || vent.X1 >= len(vf) || vent.X2 < 0 || vent.X2 >= len(vf) || vent.Y1 < 0 || vent.Y1 >= len(vf[0]) || vent.Y2 < 0 || vent.Y2 >= len(vf[0]) {
		return fmt.Errorf("Vent out of bounds")
	}

	orientation := vent.Orientation()

	if orientation == Vertical {
		var from, to int
		if vent.Y1 > vent.Y2 {
			from, to = vent.Y2, vent.Y1
		} else {
			from, to = vent.Y1, vent.Y2
		}

		for y := from; y <= to; y++ {
			vf[y][vent.X1] += 1
		}
	}

	if orientation == Horizontal {
		var from, to int
		if vent.X1 > vent.X2 {
			from, to = vent.X2, vent.X1
		} else {
			from, to = vent.X1, vent.X2
		}

		for x := from; x <= to; x++ {
			vf[vent.Y1][x] += 1
		}
	}

	if orientation == Diagonal {
		var fromX, fromY, toX int
		if vent.X1 > vent.X2 {
			fromX, toX = vent.X2, vent.X1
		} else {
			fromX, toX = vent.X1, vent.X2
		}

		if vent.Y1 > vent.Y2 {
			fromY = vent.Y2
		} else {
			fromY = vent.Y1
		}

		for x, y := fromX, fromY; x <= toX; x++ {
			vf[y][x] += 1
			y++
		}
	}

	return nil
}

// PointsAboveThreshold returns the locations of vents above the given threshold.
func (vf VentField) PointsAboveThreshold(threshold int) []struct{ X, Y int } {
	points := make([]struct{ X, Y int }, 0, len(vf)*len(vf[0]))

	for row := range vf {
		for col := range vf[row] {
			if vf[row][col] > threshold {
				points = append(points, struct{ X, Y int }{row, col})
			}
		}
	}

	return points
}

// NewVentField creates a new empty vent field with the given dimensions.
func NewVentField(width, height int) VentField {
	v := make([][]int, width+1)
	for i := range v {
		v[i] = make([]int, height+1)
	}
	return v
}
