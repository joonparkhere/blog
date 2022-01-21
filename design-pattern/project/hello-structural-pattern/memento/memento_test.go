package memento

import (
	"fmt"
	"testing"
)

func TestAfter(t *testing.T) {
	caretaker := NewCareTaker()
	originator := NewConcreteOriginator("A")

	fmt.Printf("Originator Current State: %s\n", originator.GetState())
	caretaker.AddMemento(originator.Save())

	originator.SetState("B")
	fmt.Printf("Originator Current State: %s\n", originator.GetState())
	caretaker.AddMemento(originator.Save())

	originator.SetState("C")
	fmt.Printf("Originator Current State: %s\n", originator.GetState())
	caretaker.AddMemento(originator.Save())

	memento := caretaker.GetMemento(1)
	memento.Restore()
	fmt.Printf("Restored to State: %s\n", originator.GetState())
}
