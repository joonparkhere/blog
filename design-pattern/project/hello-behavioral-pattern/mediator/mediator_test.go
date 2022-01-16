package mediator

import (
	"sync"
	"testing"
)

func TestAfter(t *testing.T) {
	trainChan := make(chan train)
	wg := sync.WaitGroup{}
	wg.Add(10)

	for i := 0; i < 10; i++ {
		go func(i int) {
			defer wg.Done()
			stationManager := newStationManager()
			if i&2 == 0 {
				train := newPassengerTrain(stationManager)
				train.depart()
				trainChan <- train
			} else {
				train := newFreightTrain(stationManager)
				train.depart()
				trainChan <- train
			}
		}(i)
	}

	go func() {
		wg.Wait()
		close(trainChan)
	}()

	for train := range trainChan {
		train.arrive()
	}
}
