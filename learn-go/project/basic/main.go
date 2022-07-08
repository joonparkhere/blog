package main

import (
	"fmt"
	"strings"
)

func multiply(a, b int) int {
	return a * b
}

func lenAndUpper(name string) (int, string) {
	return len(name), strings.ToUpper(name)
}

func repeatMe(words ...string) {
	fmt.Println(words)
}

func lenAndLower(name string) (length int, lowercase string) {
	length = len(name)
	lowercase = strings.ToLower(name)
	return
}

func upperandLower(name string) (upper string, lower string) {
	fmt.Println("START!")
	defer fmt.Println("DONE!")
	upper = strings.ToUpper(name)
	lower = strings.ToLower(name)
	fmt.Println("END!")
	return
}

func supperAdd(numbers ...int) int {
	fmt.Println(numbers)
	for i := 0; i < len(numbers); i++ {
		fmt.Println(i, numbers[i])
	}
	for index, number := range numbers {
		fmt.Println(index, number)
	}
	total := 0
	for _, number := range numbers {
		total += number
	}
	return total
}

func canDrink(age int) bool {
	if koreanAge := age + 2; koreanAge < 18 {
		return false
	}
	return true
}

func canSmoke(age int) bool {
	switch koreanAge := age + 2; {
	case koreanAge < 18:
		return false
	case koreanAge > 50:
		return false
	}
	return true
}

type person struct {
	name string
	age int
	favoriteFood []string
}

//isPossible := false

func main() {
	/* variable
	fmt.Println("Hello world")
	something.SayHello()

	const name string = "joon"
	const isHappy bool = true

	var myname string = "seung"
	myname = "ho"
	fmt.Println(myname)

	lastname := "ho"
	lastname = "seung"
	fmt.Println(lastname)
	*/

	/* function
	fmt.Println(multiply(2, 2))

	totalLength, upperName := lenAndUpper("joon")
	fmt.Println(totalLength, upperName)
	fmt.Println(lenAndUpper("joon"))

	repeatMe("joon", "seung", "ho", "park")

	totalLen, lowerName := lenAndLower("JOON")
	fmt.Println(totalLen, lowerName)
	fmt.Println(lenAndLower("JOON"))

	fmt.Println(upperandLower("jOoN"))
	*/

	/* for loop
	total := supperAdd(2, 3, 4, 5, 6, 7, 8, 9)
	fmt.Println(total)
	*/

	/* if switch
	fmt.Println(canDrink(16))
	fmt.Println(canSmoke(49))
	*/

	/* pointer
	a := 2
	b := &a
	a = 5
	*b = 3
	fmt.Println(a, &a, &b, b, *b)
	*/

	/* array
	names := [5]string{"joon", "seung", "ho", "park"}
	names[3] = "lee"
	names[4] = "song"
	fmt.Println(names)

	mynames := []string{"joon", "seung", "ho", "park"}
	mynames[3] = "jung"
	fmt.Println(mynames)
	newnames := append(mynames, "kwang")
	fmt.Println(newnames)
	*/

	/* map
	me := map[string]string{"name" : "joon", "age" : "12"}
	fmt.Println(me)
	for _, value := range me {
		fmt.Println(value)
	}
	*/

	/* struct
	favoriteFood := []string{"kimchi", "ramen"}
	joon := person{name: "joon", age: 12, favoriteFood: favoriteFood}
	fmt.Println(joon, joon.name)
	*/
}

