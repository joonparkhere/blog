package facade

import "fmt"

type client struct {
	account      *account
	pin          *pin
	wallet       *wallet
	ledger       *ledger
	notification *notification
}

func newClient(accountID string, code int) *client {
	fmt.Println("Starting create client")
	client := &client{
		account:      newAccount(accountID),
		pin:          newPin(code),
		wallet:       newWallet(),
		ledger:       newLedger(),
		notification: newNotification(),
	}
	fmt.Println("Client created")
	return client
}

func (c *client) addMoneyToWallet(accountID string, amount int) error {
	fmt.Println("Starting add money to wallet")
	if err := c.account.checkAccount(accountID); err != nil {
		return err
	}
	c.wallet.deposit(amount)
	c.ledger.makeEntry(accountID, "deposit", amount)
	c.notification.sendWalletDepositNotification()
	return nil
}

func (c *client) deductMoneyFromWallet(accountID string, code, amount int) error {
	fmt.Println("Starting deduct money from wallet")
	if err := c.account.checkAccount(accountID); err != nil {
		return err
	}
	if err := c.pin.checkPin(code); err != nil {
		return err
	}
	if err := c.wallet.withdraw(amount); err != nil {
		return err
	}
	c.ledger.makeEntry(accountID, "withdraw", amount)
	c.notification.sendWalletWithdrawNotification()
	return nil
}
