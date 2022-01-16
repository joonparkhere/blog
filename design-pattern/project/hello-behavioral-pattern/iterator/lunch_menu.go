package iterator

type lunchMenu struct {
	dishes []*dish
}

func newLunchMenu() *lunchMenu {
	return &lunchMenu{}
}

func (l *lunchMenu) addDish(dish *dish) *lunchMenu {
	l.dishes = append(l.dishes, dish)
	return l
}

func (l *lunchMenu) createIterator() iterator {
	return newDishIterator(l.dishes)
}
