package flyweight

import "time"

type terroristDress struct {
	color string
}

func newTerroristDress() *terroristDress {
	time.Sleep(time.Second)
	return &terroristDress{
		color: "red",
	}
}

func (t *terroristDress) getColor() string {
	return t.color
}
