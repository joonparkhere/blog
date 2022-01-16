package adapter

import "fmt"

type vega struct {
}

func (v *vega) insertMircoUSBPort() {
	fmt.Println("Mirco USB connector is plugged into vega machine")
}
