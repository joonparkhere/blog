package singleton

import (
	"fmt"
	"sync"
)

var lock = &sync.Mutex{}

type single struct {
	count int
}

var singleInstance *single

func getInstance() *single {
	if singleInstance != nil {
		singleInstance.count += 1
		fmt.Printf("Single instance already created %d\n", singleInstance.count)
		return singleInstance
	}

	lock.Lock()
	defer lock.Unlock()

	if singleInstance != nil {
		singleInstance.count += 1
		fmt.Printf("Single instance already created %d\n", singleInstance.count)
		return singleInstance
	}

	singleInstance = &single{}
	singleInstance.count += 1
	fmt.Printf("Creating single instance now %d\n", singleInstance.count)
	return singleInstance
}
