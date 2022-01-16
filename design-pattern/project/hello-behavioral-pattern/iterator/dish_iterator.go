package iterator

type dishIterator struct {
	index  int
	dishes []*dish
}

func newDishIterator(dishes []*dish) *dishIterator {
	return &dishIterator{
		index:  0,
		dishes: dishes,
	}
}

func (i *dishIterator) hasNext() bool {
	if i.index < len(i.dishes) {
		return true
	}
	return false
}

func (i *dishIterator) getNext() *dish {
	if i.hasNext() {
		dish := i.dishes[i.index]
		i.index += 1
		return dish
	}
	return nil
}
