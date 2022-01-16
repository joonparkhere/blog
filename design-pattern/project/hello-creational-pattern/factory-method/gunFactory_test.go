package factory_method

import (
	"fmt"
	"testing"
)

func Test(t *testing.T) {
	ak47 := getGun("ak47")
	musket := getGun("musket")

	printDetails(ak47)
	printDetails(musket)
}

func printDetails(g iGun) {
	fmt.Printf("Gun: %s\n", g.getName())
	fmt.Printf("Power: %d\n", g.getPower())
}
