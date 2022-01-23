---
title: "[디자인 패턴 GURU] Observer 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Observer 패턴
date: 2022-01-20
pin: false
image: images/observer.png
tags:
- Software Engineering
- Design Pattern
- Go
---

## Observer

### Intent

이 패턴은 여러 객체가 관찰 (Observer / Subscribe) 하고 있는 어느 객체에 무언가 이벤트가 발생하면 알림을 보내는 매커니즘의 방법이다.

### Problem

`Customer`와 `Store` 두 종류의 객체가 있다고 하자. `Customer`가 특정 브랜드 제품 구매를 원한다면 두 가지 방법으로 확인이 가능하다.

- 매일 `Customer`가 `Store`에 방문해 해당 제품 구매 가능한지 확인
- 매일 `Store`가 모든 `Customer`에게 새 제품에 대한 정보 전송

두 경우 모두 문제가 있다. 첫번째는 불필요한 낭비가 발생한다는 점, 그리고 두번째는 관심없는 소비자들이 일명 스팸 메일을 받게된다는 점이다.

### Solution Structure

특정 이벤트가 발생하면 다른 객체들에게 알림을 보내는 객체를 **Publisher**라고 하며, 이런 publisher들의 상태 변화를 Tracking하는 객체들을 **Subscriber**라고 한다. Observer 패턴은 Subscription 매커니즘을 Publisher에게 할당하여 각 Subscriber 객체들은 Tracking하거나 Un-Tracking할 수 있도록 만든다.

![Soltuion Diagram[^1]](images/observer-solution2-en.png)

여기서 중요한 점은 Publisher와 Subscriber 사이의 커플링을 막기 위해서는 알림을 수신할 수 있는 인터페이스를 Subsriber 모두가 Implement해야 한다는 것이다.

![Solution Structure[^1]](images/observer-structure.png)

1. `Publisher`는 이벤트가 발생할 때 다른 객체들에게 알림을 보낸다. 더불어 Subscription에 Join 혹은 Leave 할 수 있는 메서드도 포함한다.
2. `Subscriber` 인터페이스는 알림을 수신할 수 있는 메서드를 포함한다.
3. `Concrete Subscriber`들은 수신한 알림에 따라 행동을 취한다. 이 클래스들은 `Publisher`가 `Subscriber`들에게 커플링 되지 않도록 `Subscriber` 인터페이스를 Implement 해야한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/observer)

간단한 `Customer` 객체와 `Item` 객체 간의 Subscriber / Publisher 관계에 대한 예제이다.

```go
type Subject interface {
	Join(Observer)
	Leave(Observer)
	NotifyAll()
}
```

```go
type Observer interface {
	Update(string)
	GetID() string
}
```

```go
type Item struct {
	observers []Observer
	name      string
	inStock   bool
}

func NewItem(name string) *Item {
	return &Item{
		name: name,
	}
}

func (i *Item) UpdateAvailability() {
	i.inStock = true
	fmt.Printf("Item %s is now in stock\n", i.name)
	i.NotifyAll()
}

func (i *Item) Join(o Observer) {
	i.observers = append(i.observers, o)
}

func (i *Item) Leave(o Observer) {
	i.observers = removeFromSlice(i.observers, o)
}

func (i *Item) NotifyAll() {
	for _, observer := range i.observers {
		observer.Update(i.name)
	}
}

func removeFromSlice(slice []Observer, target Observer) []Observer {
	length := len(slice)
	for i, obj := range slice {
		if target.GetID() == obj.GetID() {
			slice[length-1], slice[i] = slice[i], slice[length-1]
			return slice[:length-1]
		}
	}
	return slice
}
```

```go
type Customer struct {
	id string
}

func NewCustomer(id string) *Customer {
	return &Customer{
		id: id,
	}
}

func (c *Customer) Update(itemName string) {
	fmt.Printf("Sending email to customer %s for item %s\n", c.id, itemName)
}

func (c *Customer) GetID() string {
	return c.id
}
```

이어서 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	customerA := NewCustomer("abc@gmail.com")
	customerB := NewCustomer("xyz@gmail.com")

	iphone := NewItem("iPhone 13")
	iphone.Join(customerA)
	iphone.Join(customerB)

	iphone.UpdateAvailability()
}
```

### Real Example

Observer 패턴과 유사한 개발 패러다임 리액티브 프로그래밍이 존재한다. 스프링 WebFlux가 이 형태를 띠며, 발행 (Publisher) - 구독 (Subsciber) 패턴이라고도 한다. [리액티브 선언문](https://www.reactivemanifesto.org/ko)에서 아래와 같이 설명하고 있다.

> 근래의 애플리케이션은 모바일 기기에서 부터 클라우드 기반의 클러스터까지 모든 기기에 배포되고 있습니다. 데이터는 페타 바이트 단위로 측정되나, 사용자는 ms 정도의 응답 시간과 100% 가동률을 기대합니다. 이에 따라 응답이 잘 되고, 탄력적이며 유연하고 메시지 기반 동작 시스템의 필요성이 대두되며 이를 리액티브 시스템 (Reactive System) 라고 부릅니다.
>
> 리액티브 시스템으로 구축된 시스템은 보다 유연하고, 느슨한 결합을 갖고, 확장성이 있습니다.
>
> - 응답성 (Responsive)
>
>   신속하고 일관성 있는 응답 시간을 제공하고, 신뢰할 수 있는 상한선을 설정하여 일돤된 서비스 품질을 제공합니다.
>
> - 탄력성 (Resilient)
>
>   시스템이 장애에 직면하더라도 응답성을 유지하는 것을 탄력성이 있다고 합니다. 장애는 각각의 구성 요소에 포함되며 구성 요소들은 서로 분리되어 있기 때문에 전체 시스템을 위험하게 하지 않고 복구할 수 있도록 보장합니다.
>
> - 유연성 (Elastic)
>
>   작업량이 변화하더라도 시스템이 응답성을 유지하는 것을 유연성이라고 합니다. 이는 시스템에서 경쟁적인 부분이나 중앙 집중적인 병목 현상이 존재하지 않도록 설계하여 요청을 분산시키는 것을 의미합니다.
>
> - 메시지 구동 (Message Driven)
>
>   리액티브 시스템은 비동기 메시지 전달에 의존하여 구송 요소 사이에서 느슨한 결합, 격리, 위치 투명성을 보장하는 경계를 형성합니다.

![Difference between Observer Pattern and Reactive Programming[^2]](images/observer-difference-with-reactive-programming.png)

Observer 패턴과 Publisher - Subscriber 패턴의 가장 큰 차이는 메시지 송신자와 수진자가 직접적인 통신을 하느냐 마느냐이다. 리액티브 프로그래밍의 경우 둘 사이에 메시지 브로커 또는 이벤트 버스라고 불리는 제 3의 구성 요소가 위치한다. 이를 통해 Publisher와 Subscriber가 서로 알지 못하더라도 소통할 수 있도록 만들어준다.

### Note

- 어느 한 객체의 상태 변화가 다른 객체의 변화에 영향을 끼칠 때 사용

> Subscription에 의한 알림 수신 순서는 랜덤일 수 있다.

[^1]: [Observer Origin](https://refactoring.guru/design-patterns/observer)
[^2]: [zorba91 Tistory Posting](https://zorba91.tistory.com/291)