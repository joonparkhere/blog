package visitor

import (
	"fmt"
	"testing"
)

func TestAfter(t *testing.T) {
	square := NewSquare(2)
	circle := NewCircle(3)
	rectangle := NewRectangle(2, 3)

	areaCalculator := NewAreaCalculator()

	square.Accept(areaCalculator)
	circle.Accept(areaCalculator)
	rectangle.Accept(areaCalculator)

	fmt.Println()

	middleCoordinates := NewMiddleCoordinates()

	square.Accept(middleCoordinates)
	circle.Accept(middleCoordinates)
	rectangle.Accept(middleCoordinates)
}
