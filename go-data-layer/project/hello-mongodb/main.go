package main

import (
	"context"
	"encoding/json"
	"fmt"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func main() {
	client := mongoConnect()
	defer mongoDisconnect(client)

	collection := client.Database("test220103").Collection("person")

	var result bson.M
	err := collection.FindOne(
		context.TODO(),
		bson.D{{"title", "first"}},
	).Decode(&result)
	if err == mongo.ErrNoDocuments {
		fmt.Printf("No document was found with the title\n")
		return
	} else if err != nil {
		panic(err)
	}

	jsonData, err := json.MarshalIndent(result, "", "    ")
	if err != nil {
		panic(err)
	}

	fmt.Printf("json data: %s\n", jsonData)
}

func mongoConnect() *mongo.Client {
	dbURL := "mongodb://localhost:27017/test220103"
	credential := options.Credential{
		Username: "sa",
		Password: "1234",
	}

	clientOptions := options.Client().ApplyURI(dbURL).SetAuth(credential)

	client, err := mongo.Connect(context.TODO(), clientOptions)
	if err != nil {
		panic(err)
	}

	if err := client.Ping(context.TODO(), nil); err != nil {
		panic(err)
	}

	return client
}

func mongoDisconnect(client *mongo.Client) {
	if err := client.Disconnect(context.TODO()); err != nil {
		panic(err)
	}
}
