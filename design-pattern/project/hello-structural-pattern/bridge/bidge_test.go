package bridge

import "testing"

func TestBefore(t *testing.T) {
	linuxComputerWithCanonPrinter := &linuxCanon{}
	linuxComputerWithCanonPrinter.print()
}

func TestAfter(t *testing.T) {
	macComputer := &mac{}
	windowsComputer := &windows{}

	epsonPrinter := &epson{}
	hpPrinter := &hp{}

	macComputer.setPrinter(epsonPrinter)
	macComputer.print()

	macComputer.setPrinter(hpPrinter)
	macComputer.print()

	windowsComputer.setPrinter(epsonPrinter)
	windowsComputer.print()

	windowsComputer.setPrinter(hpPrinter)
	windowsComputer.print()
}
