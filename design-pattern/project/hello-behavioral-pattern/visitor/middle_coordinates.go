package visitor

import "fmt"

type MiddleCoordinates struct {
	x float64
	y float64
}

func NewMiddleCoordinates() *MiddleCoordinates {
	return &MiddleCoordinates{}
}

func (c *MiddleCoordinates) VisitForSquare(s *Square) {
	fmt.Println("Calculating middle point coordinates for square")
}

func (c *MiddleCoordinates) VisitForCircle(s *Circle) {
	fmt.Println("Calculating middle point coordinates for circle")
}

func (c *MiddleCoordinates) VisitForRectangle(s *Rectangle) {
	fmt.Println("Calculating middle point coordinates for rectangle")
}
