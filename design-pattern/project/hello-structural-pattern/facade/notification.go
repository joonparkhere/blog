package facade

import "fmt"

type notification struct {
}

func newNotification() *notification {
	return &notification{}
}

func (n *notification) sendWalletDepositNotification() {
	fmt.Println("Sending wallet deposit notification")
}

func (n *notification) sendWalletWithdrawNotification() {
	fmt.Println("Sending wallet withdraw notification")
}
