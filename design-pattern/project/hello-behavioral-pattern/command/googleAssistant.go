package command

type googleAssistant struct {
	command command
}

func newGoogleAssistant(command command) *googleAssistant {
	return &googleAssistant{
		command: command,
	}
}

func (g *googleAssistant) call() {
	g.command.execute()
}
