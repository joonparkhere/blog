package adapter

import "fmt"

type vegaAdapter struct {
	vegaMachine *vega
}

func (v *vegaAdapter) insertIntoLightningPort() {
	fmt.Println("Adapter converts Lightning signal to Micro USB")
	v.vegaMachine.insertMircoUSBPort()
}
