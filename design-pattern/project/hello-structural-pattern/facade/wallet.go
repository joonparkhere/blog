package facade

import "fmt"

type wallet struct {
	balance int
}

func newWallet() *wallet {
	return &wallet{
		balance: 0,
	}
}

func (w *wallet) deposit(amount int) {
	w.balance += amount
	fmt.Println("Wallet balance deposit successfully")
}

func (w *wallet) withdraw(amount int) error {
	if w.balance < amount {
		return fmt.Errorf("Balance is not sufficient")
	}
	w.balance -= amount
	fmt.Println("Wallet balance withdraw successfully")
	return nil
}
