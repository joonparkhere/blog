package mediator

import "fmt"

type passengerTrain struct {
	mediator mediator
}

func newPassengerTrain(mediator mediator) *passengerTrain {
	return &passengerTrain{
		mediator: mediator,
	}
}

func (p *passengerTrain) arrive() {
	if p.mediator.canArrive(p) {
		fmt.Println("Passenger train: Arrived")
		return
	}
	fmt.Println("Passenger train: Arrival blocked, waiting")
}

func (p *passengerTrain) depart() {
	fmt.Println("Passenger train: Leaving")
	p.mediator.notifyDeparture()
}

func (p *passengerTrain) permit() {
	fmt.Println("Passenger train: Arrival permitted, arriving")
	p.arrive()
}
