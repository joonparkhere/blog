package memento

type Originator interface {
	Save() ConcreteMemento
}
