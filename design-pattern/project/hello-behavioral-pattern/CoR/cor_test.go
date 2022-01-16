package CoR

import "testing"

func TestAfter(t *testing.T) {
	cashier := &cashier{}

	medical := &medical{}
	medical.setNext(cashier)

	doctor := &doctor{}
	doctor.setNext(medical)

	reception := &reception{}
	reception.setNext(doctor)

	patient := &patient{name: "joon"}
	reception.execute(patient)
}
