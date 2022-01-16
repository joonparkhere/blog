package facade

import "fmt"

type pin struct {
	code int
}

func newPin(code int) *pin {
	return &pin{
		code: code,
	}
}

func (p *pin) checkPin(pinCode int) error {
	if p.code != pinCode {
		return fmt.Errorf("Pin code is incorrect")
	}
	fmt.Println("Pin code verified")
	return nil
}
