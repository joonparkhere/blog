package memento

type ConcreteOriginator struct {
	state string
}

func NewConcreteOriginator(s string) *ConcreteOriginator {
	return &ConcreteOriginator{
		state: s,
	}
}

func (o *ConcreteOriginator) GetState() string {
	return o.state
}

func (o *ConcreteOriginator) SetState(s string) {
	o.state = s
}

func (o *ConcreteOriginator) Save() ConcreteMemento {
	return NewConcreteMemento(o, o.state)
}
