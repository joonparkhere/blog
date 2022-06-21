package main

import (
	"fmt"
	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

func main() {
	dsn := "root:1234@tcp(127.0.0.1:3306)/gormDB?charset=utf8mb4&parseTime=True&loc=Local"
	client, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		panic(err)
	}

	if err := client.AutoMigrate(
		&User{},
		&CreditCard{},
	); err != nil {
		panic(err)
	}

	/*
		newUser, err := CreateUser(client)
		if err != nil {
			panic(err)
		}
		fmt.Printf("Create User: %d - %s\n", newUser.Age, newUser.Name)

		readUser, err := ReadUser(client)
		if err != nil {
			panic(err)
		}
		fmt.Printf("Read User: %d - %s\n", readUser.Age, readUser.Name)

		updateUser, err := UpdateUser(client, readUser.ID)
		if err != nil {
			panic(err)
		}
		fmt.Printf("Update User: %d - %s\n", updateUser.Age, updateUser.Name)
	*/

	newUser, err := CreateUserWithCreditCard(client)
	if err != nil {
		panic(err)
	}

	err = FindAssociation(client, newUser)
	if err != nil {
		panic(err)
	}
}

func CreateUser(client *gorm.DB) (User, error) {
	joon := User{
		Age:  20,
		Name: "joon",
	}

	result := client.Create(&joon)
	return joon, result.Error
}

func ReadUser(client *gorm.DB) (User, error) {
	var joon User

	result := client.Where(&User{
		Name: "joon",
	}).Take(&joon)

	return joon, result.Error
}

func UpdateUser(client *gorm.DB, id uint) (User, error) {
	var joon User
	client.Find(&joon, id)

	joon.Age = 40
	result := client.Save(&joon)

	return joon, result.Error
}

func CreateUserWithCreditCard(client *gorm.DB) (User, error) {
	joon := User{
		Age:  20,
		Name: "joon",
		CreditCards: []CreditCard{
			{Number: "1234-1234-1234-1234"},
			{Number: "5678-5678-5678-5678"},
		},
	}

	result := client.Create(&joon)
	return joon, result.Error
}

func FindAssociation(client *gorm.DB, user User) error {
	creditCards := make([]CreditCard, 0)
	err := client.Model(&user).
		Association("CreditCards").
		Find(&creditCards)
	if err != nil {
		return err
	}

	for _, creditCard := range creditCards {
		fmt.Printf("Association Credit Card: %s own %d\n", creditCard.Number, creditCard.UserID)
	}
	return nil
}
