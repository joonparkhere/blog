package adapter

import (
	"testing"
)

func TestBefore(t *testing.T) {
	client := &client{}

	client.insertLightningConnectorIntoPhone(&iphone{})
	client.insertLightningConnectorIntoPhone(&nokia{})
	//client.insertLightningConnectorIntoPhone(&galaxy{})	// compile error
}

func TestAfter(t *testing.T) {
	client := &client{}
	iphone := &iphone{}
	galaxy := &galaxy{}
	vega := &vega{}

	galaxyMachineAdapter := &galaxyAdapter{
		galaxyMachine: galaxy,
	}
	vegaMachineAdapter := &vegaAdapter{
		vegaMachine: vega,
	}

	client.insertLightningConnectorIntoPhone(iphone)
	client.insertLightningConnectorIntoPhone(galaxyMachineAdapter)
	client.insertLightningConnectorIntoPhone(vegaMachineAdapter)
}
