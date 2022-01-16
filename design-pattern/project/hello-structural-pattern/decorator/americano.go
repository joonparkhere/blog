package decorator

type americano struct {
}

func (a *americano) getPrice() int {
	return 4000
}
