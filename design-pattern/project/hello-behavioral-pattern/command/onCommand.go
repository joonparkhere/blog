package command

type onCommand struct {
	device device
}

func newOnCommand(device device) *onCommand {
	return &onCommand{
		device: device,
	}
}

func (c *onCommand) execute() {
	c.device.on()
}
