package main

import "testing"

func TestMove(t *testing.T) {
	initialPosition := Vector{0, 0, 0}
	direction := Direction{
		"forward",
		3,
	}
	expectedPosition := Vector{3, 0, 0}

	actualPosition := move_without_aim(initialPosition, direction)

	if actualPosition != expectedPosition {
		t.Errorf("Expected %v, got %v", expectedPosition, actualPosition)
	}
}

func TestNavigateWithoutAim(t *testing.T) {
	initialPosition := Vector{0, 0, 0}
	directions := []Direction{
		{"forward", 5},
		{"down", 5},
		{"forward", 8},
		{"up", 3},
		{"down", 8},
		{"forward", 2},
	}

	expectedPosition := Vector{15, 10, 0}

	actualPosition := navigate(initialPosition, directions, move_without_aim)

	if actualPosition != expectedPosition {
		t.Errorf("Expected %v, got %v", expectedPosition, actualPosition)
	}
}
