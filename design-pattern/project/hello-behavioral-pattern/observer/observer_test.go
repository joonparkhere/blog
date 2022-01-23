package observer

import "testing"

func TestAfter(t *testing.T) {
	customerA := NewCustomer("abc@gmail.com")
	customerB := NewCustomer("xyz@gmail.com")

	iphone := NewItem("iPhone 13")
	iphone.Join(customerA)
	iphone.Join(customerB)

	iphone.UpdateAvailability()
}
