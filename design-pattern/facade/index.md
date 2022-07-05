## Facade

### Intent

Facade는 프랑스어 `Façade`에서 유래된 단어로, 건물의 외관이라는 뜻을 가진다. 디자인 패턴의 문맥에서 Facade는 외부에서 건물을 바라보면,  외벽만 보일 뿐 내부 구조는 보이지 않는다는 의미로 쓰인다. 즉, Facade 패턴은 어떤 Sub-system 혹은 일련의 Sub-system에 대해 통합된 인터페이스를 제공하는 방법이다.

### Problem

수많은 객체들을 이용해 복잡한 로직이 포함된 프레임워크를 구성하려 한다. 정상적으로 동작하기 위해서는 각 객체들이 초기화되어야 하고, 각각의 의존성들을 체크해야 하고, 메서드들이 올바른 순서로 동작하는 지 확인하는 등등 신경써야할 게 많다. 이는 결국 비즈니스 로직이 외부 객체와의 커플링 정도가 높아지게 되며, 서비스 전체의 이해와 유지/보수가 어려워 진다.

더불어서 유의해야하는 점은 **Law of Demeter** 원칙을 최대한 지켜려고 해야 한다. 정말 연관이 깊은 객체만 관계를 맺어야 한다는 것인데, 이는 의존성을 낮추어 관리를 용이하게 하기 위함이다.

### Solution Structure

Facade는 복잡한 Sub-system들의 구조 및 로직들을 간단화한 인터페이스를 제공한다.

![Facade Structure[^1]](images/facade-structure.png)

1. `Facade`는 요청에 맞는 Sub-system의 기능을 부분적으로 접근 및 호출한다.
2. `Subsystem`은 `Facade`의 존재와 상관없이, 해야할 작업에 대한 복잡한 로직이 구현되어 있다.
3. `Client`는 `Subsystem`의 기능을 직접 호출하는 것이 아닌, 한 차례 추상화된 `Facade`를 이용한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/facade)

어느 은행의 키오스크에서 새로운 유저를 만들고 돈을 입금하거나 출금하는 기능을 만들고자 한다. 내부의 Sub-system들은 대략 계좌번호, 비밀번호, 잔액, 거래기록, 알림을 담당하는 객체가 있을 것이다.

```go
type account struct {
	digit string
}

func newAccount(accountDigit string) *account {
	return &account{
		digit: accountDigit,
	}
}

func (a *account) checkAccount(accountDigit string) error {
	if a.digit != accountDigit {
		return fmt.Errorf("Acount Digit is incorrect")
	}
	fmt.Println("Account Verified")
	return nil
}
```

```go
type pin struct {
	code int
}

func newPin(code int) *pin {
	return &pin{
		code: code,
	}
}

func (p *pin) checkPin(pinCode int) error {
	if p.code != pinCode {
		return fmt.Errorf("Pin code is incorrect")
	}
	fmt.Println("Pin code verified")
	return nil
}
```

```go
type wallet struct {
	balance int
}

func newWallet() *wallet {
	return &wallet{
		balance: 0,
	}
}

func (w *wallet) deposit(amount int) {
	w.balance += amount
	fmt.Println("Wallet balance deposit successfully")
}

func (w *wallet) withdraw(amount int) error {
	if w.balance < amount {
		return fmt.Errorf("Balance is not sufficient")
	}
	w.balance -= amount
	fmt.Println("Wallet balance withdraw successfully")
	return nil
}
```

```go
type ledger struct {
}

func newLedger() *ledger {
	return &ledger{}
}

func (l *ledger) makeEntry(accountID, txType string, amount int) {
	fmt.Printf("Make ledger entry for account id %s with transaction type %s for amount %d\n", accountID, txType, amount)
}
```

```go
type notification struct {
}

func newNotification() *notification {
	return &notification{}
}

func (n *notification) sendWalletDepositNotification() {
	fmt.Println("Sending wallet deposit notification")
}

func (n *notification) sendWalletWithdrawNotification() {
	fmt.Println("Sending wallet withdraw notification")
}
```

아래는 이 Sub-system 기능들을 한 단계 추상화시켜 유저에게 제공해주는 Facade이다.

```go
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
```

이어서 테스트 케이스이다.

```go
func TestAfter(t *testing.T) {
	client := newClient("1234-1234-1234-1234", 7890)

	fmt.Println()
	if err := client.addMoneyToWallet("1234-1234-1234-1234", 10000); err != nil {
		fmt.Printf("Add money failed: %s\n", err.Error())
	}

	fmt.Println()
	if err := client.deductMoneyFromWallet("1234-1234-1234-1234", 7890, 5000); err != nil {
		fmt.Printf("Deduct money failed: %s\n", err.Error())
	}

	fmt.Println()
	if err := client.deductMoneyFromWallet("1234-1234-1234-1234", 7890, 10000); err != nil {
		fmt.Printf("Deduct money failed: %s\n", err.Error())
	}
}
```

### Note

- 복잡한 Sub-system를 추상화하여 접근을 제한하면서 내부 로직은 알 필요 없게 만들고자할 때 사용

> 본래 목적과 달리, Facade가 제공하는 인터페이스가 점점 많아지고 무거워질수록, 많은 Sub-system과 커플링된 **God Object**가 될 수 있다.

[^1]: [Facade Origin](https://refactoring.guru/design-patterns/facade)
