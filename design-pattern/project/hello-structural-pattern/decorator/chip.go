package decorator

type chip struct {
	beverage beverage
}

func (c *chip) getPrice() int {
	return c.beverage.getPrice() + 500
}
