package main

import "github.com/kamva/mgm/v3"

type RawTransaction struct {
	mgm.DefaultModel
	BlockHash            string `json:"blockHash" bson:"blockHash"`
	BlockNumber          string `json:"blockNumber" bson:"blockNumber"`
	BlockTimestamp       string `json:"block_timestamp" bson:"block_timestamp"`
	ChainId              string `json:"chainId" bson:"chainId"`
	CreatedTimestamp     int64  `json:"created_timestamp" bson:"created_timestamp"`
	From                 string `json:"from" bson:"from"`
	Gas                  string `json:"gas" bson:"gas"`
	GasPrice             string `json:"gasPrice" bson:"gasPrice"`
	Hash                 string `json:"hash" bson:"hash"`
	MaxFeePerGas         string `json:"maxFeePerGas" bson:"maxFeePerGas"`
	MaxPriorityFeePerGas string `json:"maxPriorityFeePerGas" bson:"maxPriorityFeePerGas"`
	Nonce                string `json:"nonce" bson:"nonce"`
	To                   string `json:"to" bson:"to"`
	TransactionIndex     string `json:"transactionIndex" bson:"transactionIndex"`
	Type                 string `json:"type" bson:"type"`
	UpdatedTimestamp     int64  `json:"updated_timestamp" bson:"updated_timestamp"`
	Value                string `json:"value" bson:"value"`
}
