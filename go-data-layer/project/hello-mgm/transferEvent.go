package main

import "github.com/kamva/mgm/v3"

type TransferEvent struct {
	mgm.DefaultModel
	Address          string `json:"address" bson:"address"`
	BlockHash        string `json:"block_hash" bson:"block_hash"`
	BlockNumber      int64  `json:"block_number" bson:"block_number"`
	BlockTimestamp   int64  `json:"block_timestamp" bson:"block_timestamp"`
	CreatedTimestamp int64  `json:"created_timestamp" bson:"created_timestamp"`
	FromAddress      string `json:"from_address" bson:"from_address"`
	LogIndex         int64  `json:"log_index" bson:"log_index"`
	MethodID         string `json:"method_id" bson:"method_id"`
	ToAddress        string `json:"to_address" bson:"to_address"`
	TokenID          string `json:"token_id" bson:"token_id"`
	TransactionHash  string `json:"transaction_hash" bson:"transaction_hash"`
	UpdatedTimestamp int64  `json:"updated_timestamp" bson:"updated_timestamp"`
}
