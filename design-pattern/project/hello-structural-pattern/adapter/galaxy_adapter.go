package adapter

import "fmt"

type galaxyAdapter struct {
	galaxyMachine *galaxy
}

func (g *galaxyAdapter) insertIntoLightningPort() {
	fmt.Println("Adapter converts Lightning signal to USB C")
	g.galaxyMachine.insertIntoUSBCPort()
}
