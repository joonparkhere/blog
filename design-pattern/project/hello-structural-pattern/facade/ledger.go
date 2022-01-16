package facade

import "fmt"

type ledger struct {
}

func newLedger() *ledger {
	return &ledger{}
}

func (l *ledger) makeEntry(accountID, txType string, amount int) {
	fmt.Printf("Make ledger entry for account id %s with transaction type %s for amount %d\n", accountID, txType, amount)
}
