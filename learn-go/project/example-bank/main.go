package main

import (
	"fmt"
	"github.com/joonparkhere/study-project/Go/learn-go/example-bank/account"
	"log"
)

func main() {
	myAccount := account.NewAccount("joon")
	fmt.Println(myAccount)

	myAccount.Deposit(10)
	fmt.Println(myAccount.Balance())

	err := myAccount.Withdraw(5)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(myAccount.Balance())

	myAccount.ChangeOwner("seungho")
	fmt.Println(myAccount.Owner())
}
