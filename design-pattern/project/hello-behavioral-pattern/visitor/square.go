package visitor

type Square struct {
	side int
}

func NewSquare(side int) *Square {
	return &Square{
		side: side,
	}
}

func (s *Square) Accept(v Visitor) {
	v.VisitForSquare(s)
}

func (s *Square) GetType() string {
	return "Square"
}
