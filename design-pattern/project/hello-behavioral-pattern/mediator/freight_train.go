package mediator

import "fmt"

type freightTrain struct {
	mediator mediator
}

func newFreightTrain(mediator mediator) *freightTrain {
	return &freightTrain{
		mediator: mediator,
	}
}

func (f *freightTrain) arrive() {
	if f.mediator.canArrive(f) {
		fmt.Println("Freight train: Arrived")
		return
	}
	fmt.Println("Freight train: Arrival blocked, waiting")
}

func (f *freightTrain) depart() {
	fmt.Println("Freight train: Leaving")
	f.mediator.notifyDeparture()
}

func (f *freightTrain) permit() {
	fmt.Println("Freight train: Arrival permitted, arriving")
	f.arrive()
}
