package command

type offCommand struct {
	device device
}

func newOffCommand(device device) *offCommand {
	return &offCommand{
		device: device,
	}
}

func (c *offCommand) execute() {
	c.device.off()
}
