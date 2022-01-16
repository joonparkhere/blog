package decorator

type espresso struct {
	isShot bool
	isWhip bool
	isChip bool
}

func (e *espresso) getPrice() int {
	price := 3000
	if e.isShot {
		price += 200
	}
	if e.isWhip {
		price += 300
	}
	if e.isChip {
		price += 500
	}
	return price
}
