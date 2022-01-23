package visitor

type Circle struct {
	radius int
}

func NewCircle(r int) *Circle {
	return &Circle{
		radius: r,
	}
}

func (c *Circle) Accept(v Visitor) {
	v.VisitForCircle(c)
}

func (c *Circle) GetType() string {
	return "Circle"
}
