package adapter

import "fmt"

type client struct {
}

func (c *client) insertLightningConnectorIntoPhone(ph phone) {
	fmt.Println("client inserts Lightning connector into phone")
	ph.insertIntoLightningPort()
}
