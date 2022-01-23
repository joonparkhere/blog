package state

import (
	"fmt"
	"log"
	"testing"
)

func TestAfter(t *testing.T) {
	machine := NewVendingMachine(1, 10)

	if err := machine.RequestItem(); err != nil {
		log.Fatalf(err.Error())
	}

	if err := machine.InsertMoney(10); err != nil {
		log.Fatalf(err.Error())
	}

	if err := machine.DispenseItem(); err != nil {
		log.Fatalf(err.Error())
	}

	fmt.Println()

	if err := machine.RequestItem(); err != nil {
		log.Fatalf(err.Error())
	}

	if err := machine.InsertMoney(10); err != nil {
		log.Fatalf(err.Error())
	}

	if err := machine.DispenseItem(); err != nil {
		log.Fatalf(err.Error())
	}
}