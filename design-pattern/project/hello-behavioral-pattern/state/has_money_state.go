package state

import "fmt"

type HasMoneyState struct {
	machine *VendingMachine
}

func NewHasMoneyState(m *VendingMachine) *HasMoneyState {
	return &HasMoneyState{
		machine: m,
	}
}

func (s *HasMoneyState) RequestItem() error {
	return fmt.Errorf("Item dispense in progress")
}

func (s *HasMoneyState) AddItem(count int) error {
	return fmt.Errorf("Item dispense in progress")
}

func (s *HasMoneyState) InsertMoney(money int) error {
	return fmt.Errorf("Money already entered")
}

func (s *HasMoneyState) DispenseItem() error {
	fmt.Println("Dispensing item")
	s.machine.itemCnt -= 1

	if s.machine.itemCnt == 0 {
		s.machine.SetState(s.machine.noItem)
	} else {
		s.machine.SetState(s.machine.hasItem)
	}
	return nil
}
