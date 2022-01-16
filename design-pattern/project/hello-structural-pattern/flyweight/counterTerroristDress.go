package flyweight

import "time"

type counterTerroristDress struct {
	color string
}

func newCounterTerroristDress() *counterTerroristDress {
	time.Sleep(time.Second)
	return &counterTerroristDress{
		color: "green",
	}
}

func (c *counterTerroristDress) getColor() string {
	return c.color
}
