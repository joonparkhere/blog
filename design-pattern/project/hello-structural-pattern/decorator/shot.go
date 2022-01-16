package decorator

type shot struct {
	beverage beverage
}

func (s *shot) getPrice() int {
	return s.beverage.getPrice() + 200
}
