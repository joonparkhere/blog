package CoR

import "fmt"

type reception struct {
	next department
}

func (r *reception) execute(p *patient) {
	if p.registrationDone {
		fmt.Println("Registration already done")
	} else {
		fmt.Println("Reception registering patient")
		p.registrationDone = true
	}
	r.next.execute(p)
}

func (r *reception) setNext(next department) {
	r.next = next
}
