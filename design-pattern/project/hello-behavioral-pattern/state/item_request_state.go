package state

import "fmt"

type ItemRequestState struct {
	machine *VendingMachine
}

func NewItemRequestState(m *VendingMachine) *ItemRequestState {
	return &ItemRequestState{
		machine: m,
	}
}

func (s *ItemRequestState) RequestItem() error {
	return fmt.Errorf("Item already requested")
}

func (s *ItemRequestState) AddItem(count int) error {
	return fmt.Errorf("Item dispense in progress")
}

func (s *ItemRequestState) InsertMoney(money int) error {
	if money < s.machine.itemPrice {
		return fmt.Errorf("Inserted money is less. Please insert %d", s.machine.itemPrice)
	}

	fmt.Println("Money entered is ok")
	s.machine.SetState(s.machine.hasMoney)
	return nil
}

func (s *ItemRequestState) DispenseItem() error {
	return fmt.Errorf("Please insert money first")
}
