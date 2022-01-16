package singleton

import (
	"fmt"
	"testing"
)

func Test(t *testing.T) {
	for i := 0; i < 30; i++ {
		go getInstance()
	}

	fmt.Scanln()
}
