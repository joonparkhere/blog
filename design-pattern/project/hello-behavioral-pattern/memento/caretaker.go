package memento

type Caretaker struct {
	mementos []ConcreteMemento
}

func NewCareTaker() *Caretaker {
	return &Caretaker{
		mementos: make([]ConcreteMemento, 0),
	}
}

func (c *Caretaker) AddMemento(m ConcreteMemento) {
	c.mementos = append(c.mementos, m)
}

func (c *Caretaker) GetMemento(idx int) ConcreteMemento {
	return c.mementos[idx]
}
