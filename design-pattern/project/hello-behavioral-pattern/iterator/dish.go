package iterator

type dish struct {
	name  string
	price int
}

func newDish(name string, price int) *dish {
	return &dish{
		name:  name,
		price: price,
	}
}
