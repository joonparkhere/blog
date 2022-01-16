package flyweight

type game struct {
	terrorists        []*player
	counterTerrorists []*player
}

func newGame() *game {
	return &game{
		terrorists:        make([]*player, 0),
		counterTerrorists: make([]*player, 0),
	}
}

func (g *game) addTerrorist(dressType dressType) {
	player := newPlayer(terrorist, dressType)
	g.terrorists = append(g.terrorists, player)
}

func (g *game) addCounterTerrorist(dressType dressType) {
	player := newPlayer(countTerrorist, dressType)
	g.counterTerrorists = append(g.counterTerrorists, player)
}
