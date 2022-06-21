package main

import (
	"fmt"
	"github.com/kamva/mgm/v3"
	"github.com/kamva/mgm/v3/operator"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo/options"
	"time"
)

func main() {
	if err := connect(); err != nil {
		panic(err)
	}

	ctx := mgm.Ctx()
	transferCollection := mgm.CollectionByName("eth.selected_event.transfer")

	countDocuments, err := transferCollection.CountDocuments(ctx, bson.M{})
	if err != nil {
		panic(err)
	}
	fmt.Printf("document count: %d\n", countDocuments) // 835941

	findCursor, err := transferCollection.Find(
		ctx,
		bson.M{},
		options.Find().SetLimit(10),
		options.Find().SetSkip(10),
	)
	if err != nil {
		panic(err)
	}

	transferEvents := make([]TransferEvent, 0)
	if err = findCursor.All(ctx, &transferEvents); err != nil {
		panic(err)
	}
	for _, transferEvent := range transferEvents {
		txHash := transferEvent.TransactionHash
		unixTime := time.Unix(transferEvent.CreatedTimestamp, 0)
		fmt.Printf("event's tx hash: %s at %s\n", txHash, unixTime)

		lastChar := txHash[len(txHash)-1:]
		rawTxCollectionName := "eth.raw_tx_" + lastChar
		rawTxCollection := mgm.CollectionByName(rawTxCollectionName)

		var rawTransaction RawTransaction
		err := rawTxCollection.First(
			bson.M{"hash": bson.M{
				operator.Eq: txHash,
			}},
			&rawTransaction,
		)
		if err != nil {
			panic(err)
		}
		fmt.Printf("matched tx hash: %s\n", rawTransaction.Hash)
	}
}

func connect() error {
	err := mgm.SetDefaultConfig(nil, "rawDB", options.Client().ApplyURI("mongodb://127.0.0.1:27017"))
	return err
}
