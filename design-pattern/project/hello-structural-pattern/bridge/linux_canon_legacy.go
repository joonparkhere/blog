package bridge

import "fmt"

type linuxCanon struct {
}

func (lc *linuxCanon) print() {
	fmt.Println("Print request for linux")
	lc.printFile()
}

func (lc *linuxCanon) printFile() {
	fmt.Println("Printing by a canon printer")
}
