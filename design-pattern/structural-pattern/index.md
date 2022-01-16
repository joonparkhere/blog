---
title: 디자인 패턴 GURU - Structural Pattern
description: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Structural Pattern
date: 2022-01-15
image: images/index-design-patterns.png
categories:
- Software Engineering
tags:
- Design Pattern
- Go
---

## Adapter

### Intent

Adapter 패턴은 서로 호환이 되지 않는 인터페이스 간의 지원을 위해 사용한다.

### Problem

![문제점 예제[^1]](images/adapter-problem-en.png)

예를 들어 주식 시장 모니터링 서비스를 만든다고 해보자. 이 서비스는 관련 데이터를 XML 포맷으로 받아와서 서버에서 처리한다. 그러던 중, 주식 지표를 분석해주는 유용한 외부 라이브러리를 발견해 사용하려 한다. 다만 외부 라이브러리는 JSON 포맷만을 지원해서, 사용하려며 갖고 있는 XML 포맷을 JSON 포맷으로 변환해야 한다.

### Solution Structure

한 객체 인터페이스를 다른 객체가 알아먹을 수 있도록 변환해주는 역할을 지닌 Adapter를 사용하여 해결한다. Adapter는 객체를 Wrapping하여 복잡한 변환 과정을 추상화하고, Wrapping된 객체와 변환 과정 사이의 커플링을 제거한다.

![Solution using Adapter[^1]](images/adapter-solution-en.png)

위의 예제에서 발생한 문제점을 XML to JSON Adapter를 두어서 해결한다.

![Adapter Structure[^1]](images/adapter-structure.png)

1. `Client`는 기존의 서비스 비즈니스 로직을 담당하는 클래스이며, `Client Interface`는 `Client`의 역할을 명시한 인터페이스로, 이 인터페이스를 통해 객체와 소통할 수 있다.

2. `Service`는 사용하고자 하는 클래스이다. 구조가 달라서 `Client`가 직접적으로 호출할 수 없다.

3. `Adapter`는 서로 호환되지 않는 `Client`와 `Service`가 소통할 수 있도록 해준다.

   제 역할을 다하기 위해서는 `Client Interface`의 구현체 중 하나이어야 하고, 동시에 `Service` 객체를 Wrapping한 클래스이어야 한다. 여기서 Wrapping은 보통 `Service` 객체를 참조하는 필드를 가져야 함을 의미한다.

### [Code Example](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/adapter)

`phone` 인터페이스가 있고 `iphone`, `galaxy`, `vega` 구현체가 있으며 각각 Lightning, USB-C, Micro USB 포트를 사용한다. 여기서 `client`가 Lightning 케이블을 사용하는 경우에 대한 예제이다.

```go
type client struct {
}

func (c *client) insertLightningConnectorIntoPhone(ph phone) {
	fmt.Println("client inserts Lightning connector into phone")
	ph.insertIntoLightningPort()
}
```

```go
type phone interface {
	insertIntoLightningPort()
}
```

```go
type iphone struct {
}

func (m *iphone) insertIntoLightningPort() {
	fmt.Println("Lightning connector is plugged into iphone machine")
}
```

```go
type galaxy struct {
}

func (w *galaxy) insertIntoUSBCPort() {
	fmt.Println("USB C connector is plugged into galaxy machine")
}
```

```go
type vega struct {
}

func (v *vega) insertMircoUSBPort() {
	fmt.Println("Mirco USB connector is plugged into vega machine")
}
```

먼저 Adapter 패턴을 사용하지 않는 경우, `nokia`와 같이 구현체가 상호 호환을 위한 메서드를 구현하거나 컴파일 에러가 발생한다.

```go
type nokia struct {
}

func (n *nokia) insertIntoLightningPort() {
	n.convertLightningPortToUSBC()
	fmt.Println("USB C connector is plugged into galaxy machine")
}

func (n *nokia) convertLightningPortToUSBC() {
	fmt.Println("Lightning connector is converted to USB C port")
}
```

- 각 구현체마다 Convert 역할의 메서드를 구현하게 되면 코드의 중복 문제와 외부 객체으로의 의존성 문제가 발생한다.

```go
func TestBefore(t *testing.T) {
	client := &client{}

	client.insertLightningConnectorIntoPhone(&iphone{})
	client.insertLightningConnectorIntoPhone(&nokia{})
	//client.insertLightningConnectorIntoPhone(&galaxy{})	// compile error
}
```

이를 해결하기 위해 Adapter 역할을 하는 코드를 짠다.

```go
type galaxyAdapter struct {
	galaxyMachine *galaxy
}

func (g *galaxyAdapter) insertIntoLightningPort() {
	fmt.Println("Adapter converts Lightning signal to USB C")
	g.galaxyMachine.insertIntoUSBCPort()
}
```

````go
type vegaAdapter struct {
	vegaMachine *vega
}

func (v *vegaAdapter) insertIntoLightningPort() {
	fmt.Println("Adapter converts Lightning signal to Micro USB")
	v.vegaMachine.insertMircoUSBPort()
}
````

이를 이용한 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	client := &client{}
	iphone := &iphone{}
	galaxy := &galaxy{}
	vega := &vega{}

	galaxyMachineAdapter := &galaxyAdapter{
		galaxyMachine: galaxy,
	}
	vegaMachineAdapter := &vegaAdapter{
		vegaMachine: vega,
	}

	client.insertLightningConnectorIntoPhone(iphone)
	client.insertLightningConnectorIntoPhone(galaxyMachineAdapter)
	client.insertLightningConnectorIntoPhone(vegaMachineAdapter)
}
```

### Note

- 이미 구현된 객체가 다른 코드와 호환되지 않는 경우 사용

[^1]: [Adapter Origin](https://refactoring.guru/design-patterns/adapter)

---

## Bridge

### Intent

Bridge 패턴은 규모가 큰 클래스나 얀관성이 높은 클래스들에 대해 구조적인 변화를 주어, 각각 독립적으로 개발될 수 있도록 돕는다.

### Problem

이 패턴은 **Abstraction** & **Implementation**과 크게 다르지 않다. 책에서 설명하는 예제는 아래와 같다.

`Shape`라는 클래스가 있고, 하위 클래스로 `Circle`과 `Square`이 있다. 여기서 `Red`와 `Blue`를 추가하려 한다. 이런 경우 총 4개의 하위 클래스가 존재하게 된다.

![Bridge 예제[^2]](images/bridge-problem-en.png)

### Solution Structure

Bridge 패턴은 하나의 큰 클래스 묶음을 작은 단위로 쪼갠 후, 한 작은 단위의 클래스 묶음이 다른 묶음을 호출 (소유) 하도록 한다.

![Bridge 예제 문제 해결 구조[^2]](images/bridge-solution-en.png)

이렇게 작은 단위로 묶음을 쪼개면, 추후 발생할 확장 및 수정에 대해 더 유연해진다. 아래의 그림이 잘 와닿게 설명한다.

![Bridge 이점[^2]](images/bridge-3-en.png)

>  이 책에서는 Abstraction과 Implementation 용어가 너무 학술적인 면이 있고, 와닿지 않는 다는 점을 들며 위의 예제를 통해 설명했다고 한다. 그러나 개인적으로는 근본 용어의 정의와 쓰임을 아는 게 더 좋다고 생각한다.

### [Code Example](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/bridge)

`computer`와 `printer` 인터페이스가 있다. 각 인터페이스에는 2개의 구현체가 있으며 `computer`가 `printer`의 메서드를 호출하여 파일을 출력하고자 한다.

```go
type computer interface {
	print()
	setPrinter(printer)
}
```

```go
type printer interface {
	printFile()
}
```

먼저 Bridge 패턴을 사용하지 않았을 경우의 객체와 테스트 코드이다.

```go
type linuxCanon struct {
}

func (lc *linuxCanon) print() {
	fmt.Println("Print request for linux")
	lc.printFile()
}

func (lc *linuxCanon) printFile() {
	fmt.Println("Printing by a canon printer")
}	
```

```go
func TestBefore(t *testing.T) {
	linuxComputerWithCanonPrinter := &linuxCanon{}
	linuxComputerWithCanonPrinter.print()
}
```

- 여러 역할이 합쳐진 객체이다. 향후 더 다양한 구현체가 늘어날수록 코드의 중복과 유지보수가 어려워진다.

위 구조를 개선해 Bridge 패턴을 적용한 코드는 아래와 같다.

```go
type mac struct {
	printer printer
}

func (m *mac) print() {
	fmt.Println("Print request for mac")
	m.printer.printFile()
}

func (m *mac) setPrinter(p printer) {
	m.printer = p
}
```

```go
type windows struct {
	printer printer
}

func (w *windows) print() {
	fmt.Println("Print request for windows")
	w.printer.printFile()
}

func (w *windows) setPrinter(p printer) {
	w.printer = p
}
```

```go
type epson struct {
}

func (e *epson) printFile() {
	fmt.Println("Printing by a EPSON printer")
}
```

```go
type hp struct {
}

func (h *hp) printFile() {
	fmt.Println("Printing by a HP printer")
}
```

위의 구조를 토대로 한 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	macComputer := &mac{}
	windowsComputer := &windows{}

	epsonPrinter := &epson{}
	hpPrinter := &hp{}

	macComputer.setPrinter(epsonPrinter)
	macComputer.print()

	macComputer.setPrinter(hpPrinter)
	macComputer.print()

	windowsComputer.setPrinter(epsonPrinter)
	windowsComputer.print()

	windowsComputer.setPrinter(hpPrinter)
	windowsComputer.print()
}
```

### Note

- 다양한 역할과 책임을 지닌 모놀로틱한 클래스를 작게 나누고 관리하기 위해 사용
- 각 클래스를 독립적으로 확장하기 위해 사용

[^2]: [Bridge Origin](https://refactoring.guru/design-patterns/bridge)

---

## Composite

### Intent

Composite 패턴은 트리 구조로 객체들을 구성하고, 각 객체가 독립적으로 동작하도록 한다.

### Problem

만약 `Product`와 `Box` 객체가 있고, `Box`안에는 `Product` 혹은 또 하나의 `Box`가 있을 수 있다고 하자. 이러한 경우 아래 그림처럼 Recursive한 트리 구조 형태가 된다.

![Composite 문제 예제[^3]](images/composite-problem-en.png)

상자에 담긴 물건들의 가격의 총합을 알고 싶은 경우, 각각의 `Product` 구현 로직을 알고 있어야 함은 물론이고 내부 `Box`가 `Product` 혹은 `Box`를 포함하는 지에 대한 처리도 해야한다. 트리 구조의 규모가 거대해지면 사실상 불가능한 방안이다.

### Solution Structure

![Composite 구조[^3]](images/composite-structure-en.png)

1. `Component` 인터페이스는 트리 내에 위치한 요소들에 대한 공통 역할이다.

2. `Leaf`는 자식 요소가 존재하지 않는 객체이다. 실질적인 작업을 처리하게 된다.

3. `Container` (aka Composite) 는 자식 요소를 갖는 객체이다. 자식 요소는 보통 배열 형식의 필드값으로 갖는다.

   자식 요소의 구현 로직에 대해서는 의존하지 않되, `Component` 인터페이스를 통해 필요한 작업을 처리 혹은 위임할 수 있다.

### [Code Example](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/composite)

`file`과 `folder` 객체가 있고, `folder`는 또 다른 `folder` 혹은 `file`를 포함할 수 있다. `folder`에 포함된 모든 `file`을 검색하기 위해 공통 인터페이스 `component`가 필요하다.

```go
type component interface {
	search(string)
}
```

```go
type file struct {
	name string
}

func (f *file) search(keyword string) {
	fmt.Printf("Something or keyword %s in file %s\n", keyword, f.name)
}

func (f *file) getName() string {
	return f.name
}
```

```go
type folder struct {
	components []component
	name       string
}

func (f *folder) search(keyword string) {
	fmt.Printf("Searching recursively for keyword %s in folder %s\n", keyword, f.name)
	for _, composite := range f.components {
		composite.search(keyword)
	}
}

func (f *folder) add(c component) {
	f.components = append(f.components, c)
}
```

이를 이용한 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	file1 := &file{"file1"}
	file2 := &file{"file2"}
	file3 := &file{"file3"}

	folder1 := &folder{name: "folder1"}
	folder1.add(file1)

	folder2 := &folder{name: "folder2"}
	folder2.add(file2)
	folder2.add(file3)
	folder2.add(folder1)

	folder2.search("rose")
}
```

### Note

- 트리 구조의 객체들이 있고 간단하거나 복잡한 요소들을 동시에, 그리고 일관되게 처리하고자 할 때 사용

> 트리에서의 최하단 요소는 Component 인터페이스의 역할을 지니진 않지만 해당 인터페이스의 구현체이어야 하므로, ISP (Interface Segregation Principle) 을 위반한다.

[^3]: [Composite Origin](https://refactoring.guru/design-patterns/composite)

---

## Decorator

### Intent

Decorator 패턴은 주어진 상황에 따라 어떤 객체에 책임을 덧붙이는 (위임하는) 패턴으로, 해당 객체에 추가적인 요구 사항을 동적으로 추가한다. 이는 기능 확장이 필요할 때 Sub-class를 만들어 계층 구조를 갖게 하는 방법 대신 쓸 수 있는 유연한 대안이 될 수 있다.

### Problem

개발 중인 서비스에 알림 기능을 수행하는 `Notifier`가 있고, 처음에는 알림이 생성되면 유저의 이메일로 알려주는 기능만을 구현되어 있다. 하지만 유저의 요구에 따라 다른 알림 수단들 (SMS, Facebook, Slack 등) 도 추가된다. 이를 표현하는 간단한 방법은 `Notifier`라는 Super-class가 있고, 각 알림 수단이 하나의 Sub-class가 되도록 구조를 짜는 것이다.

![Decorator Problem[^4]](images/decorator-problem2.png)

그러나 만약 유저가 여러 알림 수단들을 동시에 원한다면? 위의 계층적인 구조로는 해결하기 어렵다. 굳이 계층 구조로 표현하고 싶다면 아래처럼 복잡한 형태를 띄게 된다.

![Decorator Prolem[^4]](images/decorator-problem3.png)

더불어 이후 알림 수단이 추가되면 Super-class의 수정이 불가피하고, 동시에 `Notifier` 클래스의 정의가 불분명해질 수 있다. 이는 OCP (Open-Closed Prinicple) 원칙을 위반한다고 할 수 있다. 클래스 확장을 위해서는 코드 변경이 필연적이기 때문이다. 이렇듯 클래스를 상속받아 구조를 짤 때는 아래 사항을 조심해야 한다.

- 상속이라는 성질은 정적이다. 즉, 런타임 상황에서 이미 존재하는 객체의 행동 (작업) 을 변경할 수 없다. 오로지 해당 객체를 통채로 다른 객체로 바꿔야만 한다.
- Sub-class는 대부분의 언어에서 오직 하나의 부모 클래스를 가진다.

### Solution Structure

위의 문제를 해결하는 대표적인 방안이 **Aggregation** 혹은 **Composition** 구조이다.

- Aggregation

  객체 A가 객체 B를 포함하며, 객체 B는 객체 A가 없더라도 존재할 수 있다.

- Composition

  객체 A는 객체 B를 구성 및 생명주기를 관리하고, 객체 B는 객체 A가 없다면 존재할 수 없다.

이 둘은 하나의 객체가 다른 객체를 참조하는 필드 값을 갖고, 특정 작업을 위임하는 방식으로 이루어진다. 이 방법으로 런타임에도 동적으로 객체의 행동을 변경할 수 있게 된다.

![Inheritance vs. Aggregation[^4]](images/decorator-solution1-en.png)

이런 구조에서 다른 객체를 연결해주는 객체를 **Wrapper**라고 부르며, Decorator 패턴의 메인 아이디어를 표현한 용어이다.

![Decorator Structure[^4]](images/decorator-structure.png)

1. `Component`는 Wrapper와 Wrappee 객체의 공통 인터페이스다.
2. `Concrete Component`는 기본 작업을 수행하며, Decorator들에 의해 Wrapping될 객체이다.
3. `Base Decorator`는 Wrapping된 객체를 연결시켜주는 클래스이다. 내부 객체를 참조하는 필드값은 `Component` 인터페이스이어야 한다.
4. `Concrete Decoratro`는 추가 작업을 수행하며, 동적으로 추가될 수 있는 객체이다.
5. `Client`는 `Concrete Component` 객체를 생성한 후, 상황에 따라 `Concrete Decorator` 객체를 추가할 수 있다.

### [Code Example](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/decorator)

별다방에서 `beverage`라는 음료를 주문하려고 한다. 기본 메뉴는 `americano`와 `latte`가 있으며, 추가 가능한 토핑은 `shot`, `whip`, `chip` 등이 있다.

```go
type beverage interface {
	getPrice() int
}
```

먼저 패턴을 적용하지 않은, 단순한 형태의 `espresso`와 테스트 케이스이다.

```go
type espresso struct {
	isShot bool
	isWhip bool
	isChip bool
}

func (e *espresso) getPrice() int {
	price := 3000
	if e.isShot {
		price += 200
	}
	if e.isWhip {
		price += 300
	}
	if e.isChip {
		price += 500
	}
	return price
}
```

```go
func TestBefore(t *testing.T) {
	chipWhipEspresso := &espresso{
		isWhip: true,
		isChip: true,
	}
	fmt.Printf("Price of espresso with whip and chip: %d\n", chipWhipEspresso.getPrice())
}
```

- 추가되는 토핑마다 개별 처리를 해줘야하며 이는 분명한 한계가 있다.

아래는 Decorator 패턴을 적용한 코드이다.

```go
type americano struct {
}

func (a *americano) getPrice() int {
	return 4000
}
```

```go
type latte struct {
}

func (l *latte) getPrice() int {
	return 4500
}
```

```go
type whip struct {
	beverage beverage
}

func (w *whip) getPrice() int {
	return w.beverage.getPrice() + 300
}
```

```go
type shot struct {
	beverage beverage
}

func (s *shot) getPrice() int {
	return s.beverage.getPrice() + 200
}
```

```go
type chip struct {
	beverage beverage
}

func (c *chip) getPrice() int {
	return c.beverage.getPrice() + 500
}
```

상속 구조와는 달리, 이 구조에서는 손님의 요구에 맞게 음료가 만들어질 수 있다.

```go
func TestAfter(t *testing.T) {
	americano := &americano{}
	whipAmericano := &whip{
		beverage: americano,
	}

	latte := &latte{}
	shotLatte := &shot{
		beverage: latte,
	}
	whipShotLatte := &whip{
		beverage: shotLatte,
	}
	chipWhipShotLatte := &chip{
		beverage: whipShotLatte,
	}

	fmt.Printf("Price of americano with whip: %d\n", whipAmericano.getPrice())
	fmt.Printf("Price of latte with shot, whip, and chip: %d\n", chipWhipShotLatte.getPrice())
}
```

### Note

- 런타임 시에 객체가 추가적인 작업을 수행하도록 하기 위해 사용
- 상속으로는 해결할 수 없을 때 사용

> 추가되는 Decorator들의 순서에 영향을 받지 않는 상황에서만 이 패턴을 써야 한다.

[^4]: [Decorator Origin](https://refactoring.guru/design-patterns/decorator)

---

## Facade

### Intent

Facade는 프랑스어 `Façade`에서 유래된 단어로, 건물의 외관이라는 뜻을 가진다. 디자인 패턴의 문맥에서 Facade는 외부에서 건물을 바라보면,  외벽만 보일 뿐 내부 구조는 보이지 않는다는 의미로 쓰인다. 즉, Facade 패턴은 어떤 Sub-system 혹은 일련의 Sub-system에 대해 통합된 인터페이스를 제공하는 방법이다.

### Problem

수많은 객체들을 이용해 복잡한 로직이 포함된 프레임워크를 구성하려 한다. 정상적으로 동작하기 위해서는 각 객체들이 초기화되어야 하고, 각각의 의존성들을 체크해야 하고, 메서드들이 올바른 순서로 동작하는 지 확인하는 등등 신경써야할 게 많다. 이는 결국 비즈니스 로직이 외부 객체와의 커플링 정도가 높아지게 되며, 서비스 전체의 이해와 유지/보수가 어려워 진다.

더불어서 유의해야하는 점은 **Law of Demeter** 원칙을 최대한 지켜려고 해야 한다. 정말 연관이 깊은 객체만 관계를 맺어야 한다는 것인데, 이는 의존성을 낮추어 관리를 용이하게 하기 위함이다.

### Solution Structure

Facade는 복잡한 Sub-system들의 구조 및 로직들을 간단화한 인터페이스를 제공한다.

![Facade Structure[^5]](images/facade-structure.png)

1. `Facade`는 요청에 맞는 Sub-system의 기능을 부분적으로 접근 및 호출한다.
2. `Subsystem`은 `Facade`의 존재와 상관없이, 해야할 작업에 대한 복잡한 로직이 구현되어 있다.
3. `Client`는 `Subsystem`의 기능을 직접 호출하는 것이 아닌, 한 차례 추상화된 `Facade`를 이용한다.

### [Code Example](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/facade)

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

[^5]: [Facade Origin](https://refactoring.guru/design-patterns/facade)

---

## Flyweight

### Intent

이 패턴은 다수의 객체들이 동일한 값을 갖는 데이터를 공유하도록 만들어 리소스를 절약할 수 있는 방법이다.

### Problem

![Example Problem[^6]](images/flyweight-problem-en.png)

간단한 슈팅 게임으로, 자그마한 총알들이 날아가는 게임을 만들고자 한다. 이때 총알에는 위치, 이동 방향, 속도, 색상, 형태 등의 정보들이 필요하다. 여기서 슈팅 게임이니만큼 많은 수의 총알들이 게임 내에서 필요할텐데, 메모리 용량이 적은 모바일 기기나 임베디드 기기 등은 게임에서 요구하는 메모리를 충족하지 못하고 중단될 것이다.

### Solution Structure

![Example Solution[^6]](images/flyweight-solution1-en.png)

총알 객체를 유심히 살펴보면 각 객체마다 다른 값을 갖는 필드 - 위치, 이동 방향, 속도 - 가 있고 그렇지 않는, 즉 공통의 값을 갖는 필드가 있다. 예를 들어 총알의 색상이나 형태를 결정하는 `color`,` sprite` 필드가 될 수 있는데, 특히나 `sprite` 필드는 큰 용량을 차지한다. 이러한 필드들을 묶어서 공유하도록 객체 구조를 수정하면 총알 객체들의 요구 메모리 용량이 줄어든다.

![Example Solution Structure[^6]](images/flyweight-solution3-en.png)

객체가 갖는 값 중, 주로 정적인 데이터들을 **Intrinsic State**라고 칭한다. 이들은 객체 내부에 위치해서 외부에서는 읽기만 가능하고, 변경할 수는 없다. 이와 대비되는 개념으로, 외부로부터 자주 변경되는 데이터들은 **Extrinsic State**라고 한다.

Flyweight 패턴은 Extrinsic State를 객체 내부에 보관하지 않도록 한다. 오롯이 Inttrinsic State만 객체 내에 존재하며, 이를 다른 Context에서도 공유하도록 한다. 따라서 Flyweight 패턴의 객체 내의 값은 수정되면 안된다. 처음 생성될 때 초기화된 이후로는 읽기만 가능해야 한다.

![Structure[^6]](images/flyweight-structure.png)

1. `Flyweight` 클래스는 원래 객체가 갖는 필드 중 여러 객체들에게 공유되는 값 (Intrinsic State) 을 갖는다. 메서드의 파라미터로 받아오는 값들은 Extrinsic State 이다.
2. `Context` 클래스는 각 객체마다 고유한 값을 갖는 Extrinsic State를 갖고, `Flyweight` 클래스를 참조한다. 이 두가지를 이용하여 원래 객체의 Full State를 나타낼 수 있다.
3. 객체가 해야하는 동작은 주로 `Flyweight` 클래스에 위치한다. 따라서 `operation()` 메서드를 호출할 때 Extrinsic State를 파라미터로 전달하는 형태를 취한다.
4. `Flyweight Factory`는 이미 존재하는 `Flyweight` Pool을 관리한다. `Client`가 직접 `Flyweight`를 생성하지 않고, 이 클래스를 통한다. `Flyweight`를 식별할 수 있는 약간의 `Instrinsic State`를 파라미터로 전달하여 이미 존재하는 객체이면 그걸 바로 반환하고, 존재하지 않는다면 새롭게 생성해서 반환한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/flyweight)

게임에서 `Terrorist`와 `CounterTerrorist` 역할이 있고, 각 플레이어는 복장, 역할, 게임 내에서의 위치 등의 필드를 갖는다. 이 중 복장 (`dress`) 은 플레이어 객체 사이에서 공유될 수 있는 값이기 때문에 Flyweight 패턴을 적용한다. 더불어 `dress` 객체 접근시 Context에서 공유될 수 있도록 Factory 패턴을 적용했다.

```go
type playerStatus string

const (
	terrorist      = playerStatus("T")
	countTerrorist = playerStatus("CT")
)

type player struct {
	dress     dress
	status    playerStatus
	latitude  int
	longitude int
}

func newPlayer(status playerStatus, dressType dressType) *player {
	factory := getDressFactoryInstance()
	dress, _ := factory.getDressByType(dressType)
	return &player{
		dress:  dress,
		status: status,
	}
}

func (p *player) newLocation(lat, long int) {
	p.latitude = lat
	p.longitude = long
}
```

```go
type dress interface {
	getColor() string
}
```

```go
type terroristDress struct {
	color string
}

func newTerroristDress() *terroristDress {
	time.Sleep(time.Second)
	return &terroristDress{
		color: "red",
	}
}

func (t *terroristDress) getColor() string {
	return t.color
}
```

```go
type counterTerroristDress struct {
	color string
}

func newCounterTerroristDress() *counterTerroristDress {
	time.Sleep(time.Second)
	return &counterTerroristDress{
		color: "green",
	}
}

func (c *counterTerroristDress) getColor() string {
	return c.color
}
```

- `dress`객체의 리소스를 표현하기 위해 `terroristDress`와 `counterTerroristDress`가 생성될 때 1초 동안 Sleep 하도록 설정했다.

```go
type dressType string

const (
	terroristDressType        = dressType("c")
	counterTerroristDressType = dressType("ct")
)

var (
	dressFactorySingleInstance = &dressFactory{
		dressMap: make(map[dressType]dress),
	}
)

type dressFactory struct {
	dressMap map[dressType]dress
}

func getDressFactoryInstance() *dressFactory {
	return dressFactorySingleInstance
}

func (d *dressFactory) getDressByType(dressType dressType) (dress, error) {
	if d.dressMap[dressType] != nil {
		return d.dressMap[dressType], nil
	}

	if dressType == terroristDressType {
		d.dressMap[dressType] = newTerroristDress()
		return d.dressMap[dressType], nil
	}
	if dressType == counterTerroristDressType {
		d.dressMap[dressType] = newCounterTerroristDress()
		return d.dressMap[dressType], nil
	}

	return nil, fmt.Errorf("Wrong dress type passed")
}
```

- `dressType`을 갖는 `dress` 객체가 만들어진 적이 없는 경우에만 생성하고, 이미 존재한다면 존재하는 객체를 반환하여 리소스를 절약한다.

```go
type game struct {
	terrorists        []*player
	counterTerrorists []*player
}

func newGame() *game {
	return &game{
		terrorists:        make([]*player, 0),
		counterTerrorists: make([]*player, 0),
	}
}

func (g *game) addTerrorist(dressType dressType) {
	player := newPlayer(terrorist, dressType)
	g.terrorists = append(g.terrorists, player)
}

func (g *game) addCounterTerrorist(dressType dressType) {
	player := newPlayer(countTerrorist, dressType)
	g.counterTerrorists = append(g.counterTerrorists, player)
}
```

```go
func TestBefore(t *testing.T) {
	now := time.Now()

	game := newGame()
	for i := 0; i < 10; i++ {
		game.terrorists = append(game.terrorists, &player{
			dress:  newTerroristDress(),
			status: terrorist,
		})
	}
	for i := 0; i < 3; i++ {
		game.counterTerrorists = append(game.counterTerrorists, &player{
			dress:  newCounterTerroristDress(),
			status: countTerrorist,
		})
	}

	duration := time.Since(now)
	fmt.Printf("Duration: %s\n", duration) // Duration: 13.1093886s
}

func TestAfter(t *testing.T) {
	now := time.Now()

	game := newGame()
	for i := 0; i < 10; i++ {
		game.addTerrorist(terroristDressType)
	}
	for i := 0; i < 3; i++ {
		game.addCounterTerrorist(counterTerroristDressType)
	}

	duration := time.Since(now)
	fmt.Printf("Duration: %s\n", duration) // Duration: 2.0095587s
}
```

- Factory 패턴과 Flyweight 패턴을 적용해 리소스 낭비를 막고 공유하여 게임 실행에 필요한 리소스가 줄어듬을 확인할 수 있다.

### Real Example



### Note

- 애플리케이션에 의해 생성되는 객체의 수가 많으며, 객체들이 공통된 값을 갖는 경우일 때 사용

[^6]: [Flyweight](https://refactoring.guru/design-patterns/flyweight)

---

## Proxy

### Intent

Proxy 패턴은 어느 한 객체를 대리해서 접근 제어를 한다. 실 객체에 요청이 들어와서 동작을 수행하기 전이나 후에 무언가 진행되어야 하는 것들을 설정할 수 있다.

### Problem

DB Connnection과 같이 많은 리소스를 요구하는 거대한 객체가 있다고 하자. 보통 DB에 접근할 일은 때때로 있어서 상시 접근할 필요는 없다. Lazy Initialization 방식을 적용해서 발생할 수 있는 문제를 일부 해결할 수 있지만, 객체마다 코드 중복이 발생하게 된다.

### Solution Structure

Proxy 패턴은 제어하고자 하는 개체와 동일한 인터페이스를 구현해야 하고, 클라이언트는 Proxy 객체를 통해 실 객체에 요청을 보낸다. 클라이언트로부터 요청을 발생할 때까지 Proxy 객체는 실 객체의 껍데기만을 갖고 있다가, 요청이 들어오면 실 객체를 생성한다.

![Structure[^7]](images/proxy-structure.png)

1. `ServiceInterface`는 실 객체인 `Service`의 인터페이스이다. `Proxy` 객체 또한 이 언테페이스를 따라야 한다.
2. `Proxy` 객체는 실 객체를 참조하는 필드를 가져야 한다. Proxy 과정을 마치면 실 객체의 메서드를 호출해 `Client`로부터의 요청을 전달한다. 주로 `Proxy` 객체가 실 객체의 생애주기를 관리한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-structural-pattern/proxy)

실 서비스를 담당하는 `application` 객체가 있고, 이를 Proxy하는 `nginx` 객체가 있다. Proxy 객체는 다수 요청을 방지하기 위한 접근 제어를 선 처리 후 실 동작을 `sever`에게 위임한다.

```go
type server interface {
	handleRequest(string, string) (int, string)
}
```

```go
type application struct {
}

func newApplication() *application {
	return &application{}
}

func (a *application) handleRequest(url, method string) (int, string) {
	if url == "/app/status" && method == "GET" {
		return 200, "OK"
	}
	if url == "/create/user" && method == "POST" {
		return 201, "User Created"
	}
	return 404, "Not Ok"
}
```

```go
type nginx struct {
	application       *application
	maxAllowedRequest int
	rateLimiter       map[string]int
}

func newNginxServer() *nginx {
	return &nginx{
		application:       newApplication(),
		maxAllowedRequest: 2,
		rateLimiter:       make(map[string]int),
	}
}

func (n *nginx) handleRequest(url, method string) (int, string) {
	allowed := n.checkRateLimiting(url)
	if !allowed {
		return 403, "Not Allowed"
	}
	return n.application.handleRequest(url, method)
}

func (n *nginx) checkRateLimiting(url string) bool {
	if _, ok := n.rateLimiter[url]; ok {
		n.rateLimiter[url] = 1
	}
	if n.rateLimiter[url] > n.maxAllowedRequest {
		return false
	}
	n.rateLimiter[url] += 1
	return true
}
```

- 이외에도 로깅을 담당하거나 리소스 관리 등의 동작을 추가할 수 있다.

```go
func TestAfter(t *testing.T) {
	nginxServer := newNginxServer()
	appStatusURL := "/app/status"
	createUserURL := "/create/user"

	httpCode, body := nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(createUserURL, "POST")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 201, Body: User Created

	httpCode, body = nginxServer.handleRequest(createUserURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 404, Body: Not Ok
}
```

### Real Example



### Note

- **Lazy Initialization (Virtual Proxy)**

  무거운 작업을 담당하는 객체가 리소스를 낭비하는 경우 사용

- **Access Control (Protection Proxy)**

  특정 클라이언트만이 객체를 접근하도록 제어하고자 하는 경우 사용

- **Local Execution of a Remote Service (Remote Proxy)**

- **Logging Requests (Logging Proxy)**

- **Cacching Request Results (Caching Proxy)**

[^7]: [Proxy Origin](https://refactoring.guru/design-patterns/proxy)

