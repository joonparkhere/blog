package CoR

import "fmt"

type cashier struct {
	next department
}

func (c *cashier) execute(p *patient) {
	if p.paymentDone {
		fmt.Println("Payment Done")
	} else {
		fmt.Println("Cashier getting money from patient")
	}
}

func (c *cashier) setNext(next department) {
	c.next = next
}
