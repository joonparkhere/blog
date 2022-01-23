package state

type state interface {
	AddItem(int) error
	RequestItem() error
	InsertMoney(int) error
	DispenseItem() error
}
