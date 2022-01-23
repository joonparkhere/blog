package state

import "fmt"

type NoItemState struct {
	machine *VendingMachine
}

func NewNoItemState(m *VendingMachine) *NoItemState {
	return &NoItemState{
		machine: m,
	}
}

func (s *NoItemState) RequestItem() error {
	return fmt.Errorf("Item out of stock")
}

func (s *NoItemState) AddItem(count int) error {
	s.machine.IncrementItemCount(count)
	s.machine.SetState(s.machine.hasItem)
	return nil
}

func (s *NoItemState) InsertMoney(money int) error {
	return fmt.Errorf("Item out of stock")
}

func (s *NoItemState) DispenseItem() error {
	return fmt.Errorf("Item out of stock")
}
