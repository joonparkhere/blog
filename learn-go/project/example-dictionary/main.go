package main

import (
	"fmt"
	"github.com/joonparkhere/study-project/Go/learn-go/example-dictionary/mydict"
)

func main() {
	dictionary := mydict.Dictionary{"first" : "First word"}
	fmt.Println(dictionary)

	definition, errSearch := dictionary.Search("second")
	if errSearch != nil {
		fmt.Println(errSearch)
	}
	fmt.Println(definition)

	errAdd := dictionary.Add("greet", "hello")
	if errAdd != nil {
		fmt.Println(errAdd)
	}
	fmt.Println(dictionary)

	errUpdate := dictionary.Update("greet", "hi")
	if errUpdate != nil {
		fmt.Println(errUpdate)
	}
	word, _ := dictionary.Search("greet")
	fmt.Println(word)

	dictionary.Delete("greet")
	fmt.Println(dictionary)
}
