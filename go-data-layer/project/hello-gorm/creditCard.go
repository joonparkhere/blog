package main

import "gorm.io/gorm"

type CreditCard struct {
	gorm.Model
	Number string
	UserID uint
}
