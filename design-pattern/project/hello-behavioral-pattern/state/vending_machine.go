package state

import "fmt"

type VendingMachine struct {
	noItem   state
	hasItem  state
	itemReq  state
	hasMoney state

	curState state

	itemCnt   int
	itemPrice int
}

func NewVendingMachine(itemCnt, itemPrice int) *VendingMachine {
	machine := &VendingMachine{
		itemCnt:   itemCnt,
		itemPrice: itemPrice,
	}

	machine.noItem = NewNoItemState(machine)
	machine.hasItem = NewHasItemState(machine)
	machine.itemReq = NewItemRequestState(machine)
	machine.hasMoney = NewHasMoneyState(machine)

	machine.SetState(machine.hasItem)
	return machine
}

func (m *VendingMachine) SetState(s state) {
	m.curState = s
}

func (m *VendingMachine) RequestItem() error {
	return m.curState.RequestItem()
}

func (m *VendingMachine) AddItem(count int) error {
	return m.curState.AddItem(count)
}

func (m *VendingMachine) InsertMoney(money int) error {
	return m.curState.InsertMoney(money)
}

func (m *VendingMachine) DispenseItem() error {
	return m.curState.DispenseItem()
}

func (m *VendingMachine) IncrementItemCount(count int) {
	m.itemCnt += count
	fmt.Printf("Adding %d items\n", count)
}
