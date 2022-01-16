package adapter

import "fmt"

type iphone struct {
}

func (m *iphone) insertIntoLightningPort() {
	fmt.Println("Lightning connector is plugged into iphone machine")
}
