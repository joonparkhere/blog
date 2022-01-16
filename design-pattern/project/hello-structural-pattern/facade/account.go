package facade

import "fmt"

type account struct {
	digit string
}

func newAccount(accountDigit string) *account {
	return &account{
		digit: accountDigit,
	}
}

func (a *account) checkAccount(accountDigit string) error {
	if a.digit != accountDigit {
		return fmt.Errorf("Acount Digit is incorrect")
	}
	fmt.Println("Account Verified")
	return nil
}
