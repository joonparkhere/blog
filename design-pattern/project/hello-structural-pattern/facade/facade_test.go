package facade

import (
	"fmt"
	"testing"
)

func TestAfter(t *testing.T) {
	client := newClient("1234-1234-1234-1234", 7890)

	fmt.Println()
	if err := client.addMoneyToWallet("1234-1234-1234-1234", 10000); err != nil {
		fmt.Printf("Add money failed: %s\n", err.Error())
	}

	fmt.Println()
	if err := client.deductMoneyFromWallet("1234-1234-1234-1234", 7890, 5000); err != nil {
		fmt.Printf("Deduct money failed: %s\n", err.Error())
	}

	fmt.Println()
	if err := client.deductMoneyFromWallet("1234-1234-1234-1234", 7890, 10000); err != nil {
		fmt.Printf("Deduct money failed: %s\n", err.Error())
	}
}
