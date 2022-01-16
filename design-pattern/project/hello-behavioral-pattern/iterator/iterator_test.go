package iterator

import (
	"fmt"
	"testing"
)

func TestAfter(t *testing.T) {
	soup := newDish("soup", 3000)
	beef := newDish("beef", 10000)
	fish := newDish("fish", 8000)
	cake := newDish("cake", 5000)

	lunchMenu := newLunchMenu()
	lunchMenu.addDish(soup).
		addDish(beef).
		addDish(fish).
		addDish(cake)

	iterator := lunchMenu.createIterator()
	for iterator.hasNext() {
		dish := iterator.getNext()
		fmt.Printf("Dish is %s and price is %d\n", dish.name, dish.price)
	}
}
