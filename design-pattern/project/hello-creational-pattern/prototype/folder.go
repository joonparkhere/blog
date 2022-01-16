package prototype

import "fmt"

type folder struct {
	name     string
	children []inode
}

func (f *folder) print(identation string) {
	fmt.Println(identation + f.name)
	for _, i := range f.children {
		i.print(identation + identation)
	}
}

func (f *folder) clone() inode {
	cloneFolder := &folder{
		name: f.name + "_clone",
	}

	tempChildren := make([]inode, 0)
	for _, i := range f.children {
		tempChild := i.clone()
		tempChildren = append(tempChildren, tempChild)
	}
	cloneFolder.children = tempChildren

	return cloneFolder
}
