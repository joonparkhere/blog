package adapter

import "fmt"

type nokia struct {
}

func (n *nokia) insertIntoLightningPort() {
	n.convertLightningPortToUSBC()
	fmt.Println("USB C connector is plugged into galaxy machine")
}

func (n *nokia) convertLightningPortToUSBC() {
	fmt.Println("Lightning connector is converted to USB C port")
}
