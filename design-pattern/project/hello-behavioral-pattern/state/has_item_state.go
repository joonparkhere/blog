package state

import "fmt"

type HasItemState struct {
	machine *VendingMachine
}

func NewHasItemState(m *VendingMachine) *HasItemState {
	return &HasItemState{
		machine: m,
	}
}

func (s *HasItemState) RequestItem() error {
	if s.machine.itemCnt == 0 {
		s.machine.SetState(s.machine.noItem)
		return fmt.Errorf("No item present")
	}

	fmt.Println("Item requested")
	s.machine.SetState(s.machine.itemReq)
	return nil
}

func (s *HasItemState) AddItem(count int) error {
	fmt.Printf("%d items addes\n", count)
	s.machine.IncrementItemCount(count)
	return nil
}

func (s *HasItemState) InsertMoney(money int) error {
	return fmt.Errorf("Please select item first")
}

func (s *HasItemState) DispenseItem() error {
	return fmt.Errorf("Please select item first")
}
