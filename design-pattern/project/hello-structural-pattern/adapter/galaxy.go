package adapter

import "fmt"

type galaxy struct {
}

func (w *galaxy) insertIntoUSBCPort() {
	fmt.Println("USB C connector is plugged into galaxy machine")
}
