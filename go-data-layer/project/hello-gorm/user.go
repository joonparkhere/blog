package main

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Age         int
	Name        string
	CreditCards []CreditCard
}
