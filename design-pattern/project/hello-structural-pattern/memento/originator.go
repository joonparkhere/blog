package memento

type originator interface {
	Save() *concreteMemento
}
