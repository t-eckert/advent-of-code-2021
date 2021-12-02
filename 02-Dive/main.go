package main

import (
	"bufio"
	"os"
)

func main() {
	directions := make([]Direction, 0, 1024)
	for _, line := range read_file("input.txt") {
		direction, err := DirectionFromString(line)
		if err != nil {
			panic(err)
		}

		directions = append(directions, direction)
	}

	finalVectorWithoutAim := navigate(Vector{0, 0, 0}, directions, move_without_aim)

	println("Final vector without aim:")
	println("Position:", finalVectorWithoutAim.Position)
	println("Depth:", finalVectorWithoutAim.Depth)
	println("Product:", finalVectorWithoutAim.Position*finalVectorWithoutAim.Depth, "\n")

	finalVectorWithAim := navigate(Vector{0, 0, 0}, directions, move_with_aim)
	println("Final vector with aim:")
	println("Position:", finalVectorWithAim.Position)
	println("Depth:", finalVectorWithAim.Depth)
	println("Aim:", finalVectorWithAim.Aim)
	println("Product:", finalVectorWithAim.Position*finalVectorWithAim.Depth, "\n")
}

func read_file(filename string) []string {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines
}

func move_without_aim(vector Vector, direction Direction) Vector {
	switch direction.Heading {
	case "forward":
		return Vector{vector.Position + direction.Distance, vector.Depth, vector.Aim}
	case "up":
		return Vector{vector.Position, vector.Depth - direction.Distance, vector.Aim}
	case "down":
		return Vector{vector.Position, vector.Depth + direction.Distance, vector.Aim}
	}

	return vector
}

func move_with_aim(vector Vector, direction Direction) Vector {
	switch direction.Heading {
	case "forward":
		return Vector{
			vector.Position + direction.Distance,
			vector.Depth + vector.Aim*direction.Distance,
			vector.Aim,
		}
	case "up":
		return Vector{vector.Position, vector.Depth, vector.Aim - direction.Distance}
	case "down":
		return Vector{vector.Position, vector.Depth, vector.Aim + direction.Distance}
	}

	return vector
}

func navigate(position Vector, directions []Direction, move func(Vector, Direction) Vector) Vector {
	for _, direction := range directions {
		position = move(position, direction)
	}

	return position
}
