package flyweight

type playerStatus string

const (
	terrorist      = playerStatus("T")
	countTerrorist = playerStatus("CT")
)

type player struct {
	dress     dress
	status    playerStatus
	latitude  int
	longitude int
}

func newPlayer(status playerStatus, dressType dressType) *player {
	factory := getDressFactoryInstance()
	dress, _ := factory.getDressByType(dressType)
	return &player{
		dress:  dress,
		status: status,
	}
}

func (p *player) newLocation(lat, long int) {
	p.latitude = lat
	p.longitude = long
}
