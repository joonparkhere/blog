package main

import (
	"context"
	"entgo.io/ent/dialect"
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"hello-ent/ent"
	"hello-ent/ent/car"
	"hello-ent/ent/user"
	"time"
)

func main() {
	client, err := ent.Open(dialect.MySQL, "root:1234@tcp(127.0.0.1:3306)/entDB?parseTime=True")
	if err != nil {
		panic(err)
	}
	defer client.Close()

	ctx := context.Background()

	if err := client.Schema.Create(ctx); err != nil {
		panic(err)
	}

	/*
		if _, err := CreateUser(ctx, client); err != nil {
			panic(err)
		}

		findUser, err := ReadUser(ctx, client)
		if err != nil {
			panic(err)
		}
		fmt.Printf("Read User age, name: %d, %s\n", findUser.Age, findUser.Name)

		UpdateUser, err := UpdateUser(ctx, client, findUser.ID)
		if err != nil {
			panic(err)
		}
		fmt.Printf("Update User age, name: %d, %s\n", UpdateUser.Age, UpdateUser.Name)
	*/

	/*
		newUser, err := CreateUserWithCars(ctx, client)
		if err != nil {
			panic(err)
		}

		newUserCars, err := newUser.QueryCars().All(ctx)
		if err != nil {
			panic(err)
		}
		for _, userCar := range newUserCars {
			fmt.Printf("New User Car: %s (%s)\n", userCar.Model, userCar.RegisteredAt)
		}
	*/

	if err := CreateGraph(ctx, client); err != nil {
		panic(err)
	}

	if err := ReadJoonCars(ctx, client); err != nil {
		panic(err)
	}
}

func CreateUser(ctx context.Context, client *ent.Client) (*ent.User, error) {
	newUser, err := client.User.
		Create().
		SetAge(25).
		SetName("joonpark").
		Save(ctx)
	if err != nil {
		return nil, err
	}

	return newUser, nil
}

func ReadUser(ctx context.Context, client *ent.Client) (*ent.User, error) {
	findUser, err := client.User.
		Query().
		Where(user.Name("joonpark")).
		Only(ctx)
	if err != nil {
		return nil, err
	}

	return findUser, nil
}

func UpdateUser(ctx context.Context, client *ent.Client, id int) (*ent.User, error) {
	updateUser, err := client.User.
		UpdateOneID(id).
		SetAge(20).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	return updateUser, nil
}

func CreateUserWithCars(ctx context.Context, client *ent.Client) (*ent.User, error) {
	tesla, err := client.Car.
		Create().
		SetModel("Tesla").
		SetRegisteredAt(time.Now()).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	ford, err := client.Car.
		Create().
		SetModel("Ford").
		SetRegisteredAt(time.Now()).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	joon, err := client.User.
		Create().
		SetAge(15).
		SetName("joon-edge").
		AddCars(tesla, ford).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	return joon, nil
}

func CreateGraph(ctx context.Context, client *ent.Client) error {
	joon, err := client.User.
		Create().
		SetAge(30).
		SetName("joon").
		Save(ctx)
	if err != nil {
		return err
	}
	park, err := client.User.
		Create().
		SetAge(40).
		SetName("park").
		Save(ctx)
	if err != nil {
		return err
	}

	err = client.Car.
		Create().
		SetModel("Tesla").
		SetRegisteredAt(time.Now()).
		SetOwner(joon).
		Exec(ctx)
	if err != nil {
		return err
	}
	err = client.Car.
		Create().
		SetModel("Mazda").
		SetRegisteredAt(time.Now()).
		SetOwner(joon).
		Exec(ctx)
	if err != nil {
		return err
	}
	err = client.Car.
		Create().
		SetModel("Ford").
		SetRegisteredAt(time.Now()).
		SetOwner(park).
		Exec(ctx)
	if err != nil {
		return err
	}

	err = client.Group.
		Create().
		SetName("Netlify").
		AddUsers(park, joon).
		Exec(ctx)
	if err != nil {
		return err
	}
	err = client.Group.
		Create().
		SetName("GitHub").
		AddUsers(joon).
		Exec(ctx)
	if err != nil {
		return err
	}

	return nil
}

func ReadJoonCars(ctx context.Context, client *ent.Client) error {
	joon := client.User.
		Query().
		Where(
			user.HasCars(),
			user.Name("joon"),
		).
		OnlyX(ctx)

	joonCars, err := joon.
		QueryGroups().
		QueryUsers().
		QueryCars().
		Where(
			car.Not(
				car.Model("Mazda"),
			),
		).
		All(ctx)
	if err != nil {
		return err
	}

	for _, joonCar := range joonCars {
		fmt.Printf("Joon Own Car - Car: %s\n", joonCar.Model)
	}

	return nil
}
