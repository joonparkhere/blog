package decorator

type latte struct {
}

func (l *latte) getPrice() int {
	return 4500
}
