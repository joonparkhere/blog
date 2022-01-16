---
title: 디자인 패턴 GURU - Behavioral Pattern
description: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Behavioral Pattern
date: 2022-01-16
image: images/index-design-patterns.png
categories:
- Software Engineering
tags:
- Design Pattern
- Go
---

## Chain of Responsibility

### Intent

이 패턴은 클라이언트가 보낸 요청을 처리할 때, 핸들러들이 묶여있는 체인을 따라서 처리하는 방법이다. 각 핸들러는 요청에 따라 직접 처리할 지, 아니면 다음 핸들러로 넘길지 결정한다.

### Problem

사용자 인증 요청을 담당하는 서버가 있다고 하자. 초기에는 유저가 입력한 Credential이 올바른지 확인해서 결과를 반환하면 되지 않나라고 생각할 수 있다. 하지만 Credential이 올바르지 않은 경우 예외 처리, 반복적인 요청을 탐지해서 블록하는 동작, 리소스의 효율적인 활용을 위한 Cache 활용 동작 등등 사용자 인증 요청 처리 과정에서 해야하는 동작들이 잇따라 존재할 수 있다.

### Solution Structure

![Brief Structure[^1]](images/cor-solution1-en.png)

다른 Behavioral Pattern과 마찬가지로, Chain of Responsibility 패턴 또한 Handler라고 불리는 각 객체로 분리해 작업을 처리하도록 만든다. 각 Handler들을 연결하는 방식으로는 체인 형태를 띤다. 클라이언트가 보낸 요청은 결과를 반환받을 때까지 체인을 돌아다니며 처리된다. 이때 중요한 점은 각 Handler가 전달받은 요청을 직접 처리해서 결과를 반환할지, 다음으로 연결된 Handler에게 전달할지를 결정한다는 것이다.

![Solution Structure[^1]](images/cor-structure.png)

1. `Handler` 인터페이스는 각 구현체가 처리해야 하는 메서드를 정의한다. 더불어 다음 `Handler`를 참조할 수 있는 메서드도 있다.
2. `Base Handler`는 각 구현체가 공통적으로 처리하는 부분 (Boilerplate Code) 을 묶은 클래스다.
3. `Concrete Handler`들은 실제 요청을 처리한다. 전달받은 요청에 따라 직접 처리할지 다음 `Handler`에게 넘길지 결정한다. 주로 필요한 값들은 생성될 때 주입받고, 객체 생성 이후에는 Immutable하게 동작한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-behavioral-pattern/CoR)

`patient`가 병원에 방문해 거쳐야 하는 일련의 과정 (`reception` - `docotor` - `medical` - `cashier`) 을 CoR 패턴을 적용하는 예제이다.

```go
type patient struct {
	name              string
	registrationDone  bool
	doctorCheckUpDone bool
	medicineDone      bool
	paymentDone       bool
}
```

```go
type department interface {
	execute(*patient)
	setNext(department)
}
```

```go
type reception struct {
	next department
}

func (r *reception) execute(p *patient) {
	if p.registrationDone {
		fmt.Println("Registration already done")
	} else {
		fmt.Println("Reception registering patient")
		p.registrationDone = true
	}
	r.next.execute(p)
}

func (r *reception) setNext(next department) {
	r.next = next
}
```

```go
type doctor struct {
	next department
}

func (d *doctor) execute(p *patient) {
	if p.doctorCheckUpDone {
		fmt.Println("Doctor checkup already done")
	} else {
		fmt.Println("Doctor checking patient")
		p.doctorCheckUpDone = true
	}
	d.next.execute(p)
}

func (d *doctor) setNext(next department) {
	d.next = next
}
```

```go
type medical struct {
	next department
}

func (m *medical) execute(p *patient) {
	if p.medicineDone {
		fmt.Println("Medicine already given to patient")
	} else {
		fmt.Println("Medical giving medicine to patient")
		p.medicineDone = true
	}
	m.next.execute(p)
}

func (m *medical) setNext(next department) {
	m.next = next
}
```

```go
type cashier struct {
	next department
}

func (c *cashier) execute(p *patient) {
	if p.paymentDone {
		fmt.Println("Payment Done")
	} else {
		fmt.Println("Cashier getting money from patient")
	}
}

func (c *cashier) setNext(next department) {
	c.next = next
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	cashier := &cashier{}

	medical := &medical{}
	medical.setNext(cashier)

	doctor := &doctor{}
	doctor.setNext(medical)

	reception := &reception{}
	reception.setNext(doctor)

	patient := &patient{name: "joon"}
	reception.execute(patient)
}
```

### Real Example



### Note

- 클라이언트가 보내는 요청이 다양한 방식으로 처리되어야 할 때 사용
- 특정 순서에 따라 요청이 처리되어야 할 때 사용

> Handler Chain에서 처리되지 않는 종류의 요청에 대한 예외 처리가 필요하다.

[^1]: [Chain of Responsibility Origin](https://refactoring.guru/design-patterns/chain-of-responsibility)

---

## Command

### Intent

Command 패턴은 클라이언트가 보내는 요청을 별도의 객체화하는 방법이다. 이를 통해 요청 객체를 메서드 파라미터로 전달하거나 요청의 실행을 제어하는 등의 처리가 가능해진다.

### Problem

![Problem Structure[^2]](images/command-problem2.png)

텍스트 에디터 서비스를 만드는 경우를 예시로 설명하자면, 다양한 동작들이 필요하겠지만, 그 중에서도 툴바에 있는 `Button`에 대해 살펴보고자 한다. 다양한 동작을 수행하는 `Button`이 있을 수 있다. 각 역할을 분리하기 위해 Sub-class화하는 방안을 떠올릴 수 있다. 그러나 기능이 늘어날 수록 유지보수가 어려워지며, 향후 `Button` 인터페이스에 대한 수정이 생길 경우 모든 Sub-class들의 수정을 해야하는 문제가 있다.

### Solution Structure

![Brief Solution Structure[^2]](images/command-solution3-en.png)

효과적인 소프트웨어 디자인에 있어서 중요한 것 중 하나는 관심사 분리 원칙 (Principle of Separation of Concerns) 를 지키는 것이다. Command 패턴은 요청을 직접 전달하지 않고, 관련 정보들을 객체화해서 파라미터로 전달한다. 각 동작들이 동일한 인터페이스를 따라야하며, 이를 통해 다양한 요청들을 구현체와의 커플링 없이 사용할 수 있게 된다.

![Solution Structure[^2]](images/command-structure.png)

1. `Client`는 `Concrete Command`를 만들고 `Invoker`를 통해 전달한다.

2. `Invoker` 클래스는 `Command` 객체를 저장한 후 `Receiver`에게 전달한다.

3. `Command` 인터페이스는 동작을 위한 단순 메서드가 존재한다.

   `Concrete Command`들은 다양한 요청을 구현한 객체이다. 이 객체 자체가 요청을 처리하도록 동작하지는 않지만, 관련 로직을 수행하는 (`Recevier`) 에게 전달한다.

4. `Recevier` 클래스는 동작을 수행하는 비즈니스 로직이다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-behavioral-pattern/command)

Invoker인 `googleAssistant`를 통해 `tv`를 켜거나 끄는 `onCommand`, `offCommand` 객체를 전달해 동작하도록 하는 예제이다.

```go
type device interface {
	on()
	off()
}
```

```go
type tv struct {
	isRunning bool
}

func newTV() *tv {
	return &tv{}
}

func (t *tv) on() {
	t.isRunning = true
	fmt.Println("Turning tv on")
}

func (t *tv) off() {
	t.isRunning = false
	fmt.Println("Turning tv off")
}
```

```go
type command interface {
	execute()
}
```

```go
type onCommand struct {
	device device
}

func newOnCommand(device device) *onCommand {
	return &onCommand{
		device: device,
	}
}

func (c *onCommand) execute() {
	c.device.on()
}
```

```go
type offCommand struct {
	device device
}

func newOffCommand(device device) *offCommand {
	return &offCommand{
		device: device,
	}
}

func (c *offCommand) execute() {
	c.device.off()
}
```

```go
type googleAssistant struct {
	command command
}

func newGoogleAssistant(command command) *googleAssistant {
	return &googleAssistant{
		command: command,
	}
}

func (g *googleAssistant) call() {
	g.command.execute()
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	tv := newTV()
	onCommand := newOnCommand(tv)
	offCommand := newOffCommand(tv)

	onAssistant := newGoogleAssistant(onCommand)
	onAssistant.call() // Turning tv on

	offAssistant := newGoogleAssistant(offCommand)
	offAssistant.call() // Turning tv off
}
```

### Real Example



### Note

- 처리해야 하는 동작들을 객체로 만들어 파라미터화 시키고자 할 때 사용
- 요청을 스케줄링하거나 원격으로 처리해야 하는 경우 사용

[^2]: [Command Origin](https://refactoring.guru/design-patterns/command)

---

## Iterator

### Intent

이 패턴은 Collection에 대한 내부 정보없이 안에 속한 Element들을 훑기 위해 사용하는 방법이다.

### Problem

단순한 구조의 Collection을 넘어서, Tree 구조처럼 일련의 순서가 모호한 Collection인 경우 내부 Element를 훑는 순서는 BFS, DFS 등 다양할 수 있다. Collection 내 Element들을 접근하는 다양한 알고리즘들을 구현하다보면 Collection 클래스는 그 자체의 역할이 모호해지며, 각 알고리즘들은 Collectino마다 구현 방법에 따라 달라지기 때문에 해당 알고리즘들을 Genric하게 뽑아내기도 쉽지 않다.

### Solution Structure

![Iterator Brief Structure[^3]](images/iterator-solution1.png)

주요 포인트는 Element들을 훑는 로직을 Collection 클래스와 분리하고자 하는 점이다.

![Iterator Structure[^3]](images/iterator-structure.png)

1. `Iterator` 인터페이스는 내부 Element들을 훑기 위한 동작의 정의가 되어있다.
2. `Concrete Iterator`들은 특정 알고리즘을 구현한다. 다른 `Iterator`와는 독립적으로 동작하여 모든 Element들을 훑도록 보장되어야 한다.
3. `Collection` 인터페이스는 `Iterator`에 접근할 수 있는 메서드를 포함한다.
4. `Concrete Collection`은 유저가 전달한 요청에 맞게 `Concrete Iterator`를 참조하는 `Collection`이다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-behavioral-pattern/iterator)

어느 식당에서 요리 메뉴 (Element) 가 있고 점심 메뉴 (Collection) 를 판매한다고 할 때, 판매중인 메뉴들을 모두 훑는 (Iterator) 예제이다.

```go
type dish struct {
	name  string
	price int
}

func newDish(name string, price int) *dish {
	return &dish{
		name:  name,
		price: price,
	}
}
```

```go
type iterator interface {
	hasNext() bool
	getNext() *dish
}
```

```go
type dishIterator struct {
	index  int
	dishes []*dish
}

func newDishIterator(dishes []*dish) *dishIterator {
	return &dishIterator{
		index:  0,
		dishes: dishes,
	}
}

func (i *dishIterator) hasNext() bool {
	if i.index < len(i.dishes) {
		return true
	}
	return false
}

func (i *dishIterator) getNext() *dish {
	if i.hasNext() {
		dish := i.dishes[i.index]
		i.index += 1
		return dish
	}
	return nil
}
```

```go
type menu interface {
	createIterator() iterator
}
```

```go
type lunchMenu struct {
	dishes []*dish
}

func newLunchMenu() *lunchMenu {
	return &lunchMenu{}
}

func (l *lunchMenu) addDish(dish *dish) *lunchMenu {
	l.dishes = append(l.dishes, dish)
	return l
}

func (l *lunchMenu) createIterator() iterator {
	return newDishIterator(l.dishes)
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	soup := newDish("soup", 3000)
	beef := newDish("beef", 10000)
	fish := newDish("fish", 8000)
	cake := newDish("cake", 5000)

	lunchMenu := newLunchMenu()
	lunchMenu.addDish(soup).
		addDish(beef).
		addDish(fish).
		addDish(cake)

	iterator := lunchMenu.createIterator()
	for iterator.hasNext() {
		dish := iterator.getNext()
		fmt.Printf("Dish is %s and price is %d\n", dish.name, dish.price)
	}
}
```

### Real Example



### Note

- 복잡한 자료구조 내부를 추상화하며 활용할 수 있도록 하고 싶을 때 사용
- 

[^3]: [Iteratort Origin](https://refactoring.guru/design-patterns/iterator)

---

## Mediator

### Intent

이 패턴은 여러 객체들간의 혼란스러운 의존 상황을 해결하고자 한다. 객체 서로 간의 직접적인 호출을 중지시키고 Mediator 객체를 통해서만 소통하도록 만든다.

### Problem

![Problem Diagram[^4]](images/mediator-problem1-en.png)

어느 서비스의 회원을 관리할 때 이용되는 Component들이다. 각 Component의 상황에 맞게 다른 Component의 필요유무가 달라진다. 예를 들어 회원이 견주라고 `Checkbox`에 표시할 경우, 반려동물에 대한 정보를 기록하는 `TextField`가 나타나는 꼴이다. 이런 상황이 쌓이고 점점 거대해질수록, 각 Component들은 서로의 구현에 의존하게 되고 관리가 어려워진다.

### Solution Structure

![Solution Diagram[^4]](images/mediator-solution1-en.png)

Mediator 패턴은 각 Component들간의 직접 호출을 제지시키고 Mediator 객체를 통해서 모든 소통을 하도록 만든다. 이렇게 함으로써 Component들은 오로지 Mediator 객체만을 의존하면 되어 이전보다 커플링 정도가 약해진다. 더불어 각 Component들은 다른 객체에 대한 정보를 모르고 소통할 수 있으므로 캡슐화 효과도 있다.

![Mediator Structure[^4]](images/mediator-structure.png)

1. `Component`들은 비즈니스 로직을 갖는 다양한 객체들이다. 각 객체는 `Mediator` 인터페이스를 참조함으로써 다른 객체와의 소통을 할 수 있다.
2. `Mediator` 인터페이스는 서로 다른 객체 간 소통을 할 수 있는 메서드를 정의한다.
3. `Concrete Mediator`은 `Component`들 간의 관계를 캡슐화한다. 보통 이 클래스는 모든 `Component`들을 참조하고 있으며, 생명주기를 관리하기도 한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-behavioral-pattern/mediator)

기차역에 탑승객을 태우는 열차와 화물 열차가 도착할 수 있을 때, 관제소에서 역에 열차가 접근해도 되는지 관리하는 예제이다.

```go
type train interface {
	arrive()
	depart()
	permit()
}
```

```go
type mediator interface {
	canArrive(train) bool
	notifyDeparture()
}
```

```go
type passengerTrain struct {
	mediator mediator
}

func newPassengerTrain(mediator mediator) *passengerTrain {
	return &passengerTrain{
		mediator: mediator,
	}
}

func (p *passengerTrain) arrive() {
	if p.mediator.canArrive(p) {
		fmt.Println("Passenger train: Arrived")
		return
	}
	fmt.Println("Passenger train: Arrival blocked, waiting")
}

func (p *passengerTrain) depart() {
	fmt.Println("Passenger train: Leaving")
	p.mediator.notifyDeparture()
}

func (p *passengerTrain) permit() {
	fmt.Println("Passenger train: Arrival permitted, arriving")
	p.arrive()
}
```

```go
type freightTrain struct {
	mediator mediator
}

func newFreightTrain(mediator mediator) *freightTrain {
	return &freightTrain{
		mediator: mediator,
	}
}

func (f *freightTrain) arrive() {
	if f.mediator.canArrive(f) {
		fmt.Println("Freight train: Arrived")
		return
	}
	fmt.Println("Freight train: Arrival blocked, waiting")
}

func (f *freightTrain) depart() {
	fmt.Println("Freight train: Leaving")
	f.mediator.notifyDeparture()
}

func (f *freightTrain) permit() {
	fmt.Println("Freight train: Arrival permitted, arriving")
	f.arrive()
}
```

```go
type stationManager struct {
	isPlatformFree bool
	trainQueue     []train
}

func newStationManager() *stationManager {
	return &stationManager{
		isPlatformFree: true,
		trainQueue:     make([]train, 0),
	}
}

func (s *stationManager) canArrive(t train) bool {
	if s.isPlatformFree {
		s.isPlatformFree = false
		return true
	}
	s.trainQueue = append(s.trainQueue, t)
	return false
}

func (s *stationManager) notifyDeparture() {
	if !s.isPlatformFree {
		s.isPlatformFree = true
	}
	if len(s.trainQueue) > 0 {
		firstTrain := s.trainQueue[0]
		s.trainQueue = s.trainQueue[1:]
		firstTrain.permit()
	}
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	trainChan := make(chan train)
	wg := sync.WaitGroup{}
	wg.Add(10)

	for i := 0; i < 10; i++ {
		go func(i int) {
			defer wg.Done()
			stationManager := newStationManager()
			if i&2 == 0 {
				train := newPassengerTrain(stationManager)
				train.depart()
				trainChan <- train
			} else {
				train := newFreightTrain(stationManager)
				train.depart()
				trainChan <- train
			}
		}(i)
	}

	go func() {
		wg.Wait()
		close(trainChan)
	}()

	for train := range trainChan {
		train.arrive()
	}
}
```

### Real Example



### Note

- 객체들간의 커플링 정도가 높아서 변경이 어려운 경우 사용
- 서로의 의존성이 높아서 다른 곳에서 재사용하기 어려운 경우 사용

> Mediator 객체는 God Object가 되어야 함을 유의해야 한다.

[^4]: [Mediator Origin](https://refactoring.guru/design-patterns/mediator)

---



































