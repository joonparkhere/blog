package prototype

import (
	"fmt"
	"testing"
)

func Test(t *testing.T) {
	file := &file{name: "File"}
	folder := &folder{
		name:     "Folder",
		children: []inode{file},
	}
	cloneFolder := folder.clone()

	fmt.Println("Printing hierarchy for Folder")
	folder.print("    ")

	fmt.Println("Printing hierarchy for clone Folder")
	cloneFolder.print("    ")
}
