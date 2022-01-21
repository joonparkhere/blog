package memento

type ConcreteMemento struct {
	origin *ConcreteOriginator
	state  string
}

func NewConcreteMemento(o *ConcreteOriginator, s string) ConcreteMemento {
	return ConcreteMemento{
		origin: o,
		state:  s,
	}
}

func (m *ConcreteMemento) GetSavedState() string {
	return m.state
}

func (m *ConcreteMemento) Restore() {
	m.origin.SetState(m.state)
}
