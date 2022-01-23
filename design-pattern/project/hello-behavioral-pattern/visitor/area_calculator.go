package visitor

import (
	"fmt"
	"math"
)

type AreaCalculator struct {
	area float64
}

func NewAreaCalculator() *AreaCalculator {
	return &AreaCalculator{}
}

func (c *AreaCalculator) VisitForSquare(s *Square) {
	c.area = float64(s.side * s.side)
	fmt.Println("Calculating area for square")
}

func (c *AreaCalculator) VisitForCircle(s *Circle) {
	c.area = float64(s.radius*s.radius) * math.Pi
	fmt.Println("Calculating area for circle")
}

func (c *AreaCalculator) VisitForRectangle(s *Rectangle) {
	c.area = float64(s.height * s.width)
	fmt.Println("Calculating area for rectangle")
}
