package command

import "testing"

func TestAfter(t *testing.T) {
	tv := newTV()
	onCommand := newOnCommand(tv)
	offCommand := newOffCommand(tv)

	onAssistant := newGoogleAssistant(onCommand)
	onAssistant.call() // Turning tv on

	offAssistant := newGoogleAssistant(offCommand)
	offAssistant.call() // Turning tv off
}
