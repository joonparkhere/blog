package flyweight

import (
	"fmt"
	"testing"
	"time"
)

func TestBefore(t *testing.T) {
	now := time.Now()

	game := newGame()
	for i := 0; i < 10; i++ {
		game.terrorists = append(game.terrorists, &player{
			dress:  newTerroristDress(),
			status: terrorist,
		})
	}
	for i := 0; i < 3; i++ {
		game.counterTerrorists = append(game.counterTerrorists, &player{
			dress:  newCounterTerroristDress(),
			status: countTerrorist,
		})
	}

	duration := time.Since(now)
	fmt.Printf("Duration: %s\n", duration) // Duration: 13.1093886s
}

func TestAfter(t *testing.T) {
	now := time.Now()

	game := newGame()
	for i := 0; i < 10; i++ {
		game.addTerrorist(terroristDressType)
	}
	for i := 0; i < 3; i++ {
		game.addCounterTerrorist(counterTerroristDressType)
	}

	duration := time.Since(now)
	fmt.Printf("Duration: %s\n", duration) // Duration: 2.0095587s
}
