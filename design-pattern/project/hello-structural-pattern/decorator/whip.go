package decorator

type whip struct {
	beverage beverage
}

func (w *whip) getPrice() int {
	return w.beverage.getPrice() + 300
}
