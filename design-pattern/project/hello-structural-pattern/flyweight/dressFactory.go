package flyweight

import "fmt"

type dressType string

const (
	terroristDressType        = dressType("c")
	counterTerroristDressType = dressType("ct")
)

var (
	dressFactorySingleInstance = &dressFactory{
		dressMap: make(map[dressType]dress),
	}
)

type dressFactory struct {
	dressMap map[dressType]dress
}

func getDressFactoryInstance() *dressFactory {
	return dressFactorySingleInstance
}

func (d *dressFactory) getDressByType(dressType dressType) (dress, error) {
	if d.dressMap[dressType] != nil {
		return d.dressMap[dressType], nil
	}

	if dressType == terroristDressType {
		d.dressMap[dressType] = newTerroristDress()
		return d.dressMap[dressType], nil
	}
	if dressType == counterTerroristDressType {
		d.dressMap[dressType] = newCounterTerroristDress()
		return d.dressMap[dressType], nil
	}

	return nil, fmt.Errorf("Wrong dress type passed")
}
