package observer

import "fmt"

type Item struct {
	observers []Observer
	name      string
	inStock   bool
}

func NewItem(name string) *Item {
	return &Item{
		name: name,
	}
}

func (i *Item) UpdateAvailability() {
	i.inStock = true
	fmt.Printf("Item %s is now in stock\n", i.name)
	i.NotifyAll()
}

func (i *Item) Join(o Observer) {
	i.observers = append(i.observers, o)
}

func (i *Item) Leave(o Observer) {
	i.observers = removeFromSlice(i.observers, o)
}

func (i *Item) NotifyAll() {
	for _, observer := range i.observers {
		observer.Update(i.name)
	}
}

func removeFromSlice(slice []Observer, target Observer) []Observer {
	length := len(slice)
	for i, obj := range slice {
		if target.GetID() == obj.GetID() {
			slice[length-1], slice[i] = slice[i], slice[length-1]
			return slice[:length-1]
		}
	}
	return slice
}
