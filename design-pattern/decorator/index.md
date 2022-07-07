---
title: "[디자인 패턴 GURU] Decorator 패턴"
date: 2022-01-07
pin: false
tags:
- Software Engineering
- Design Pattern
- Go
---

## Decorator

### Intent

Decorator 패턴은 주어진 상황에 따라 어떤 객체에 책임을 덧붙이는 (위임하는) 패턴으로, 해당 객체에 추가적인 요구 사항을 동적으로 추가한다. 이는 기능 확장이 필요할 때 Sub-class를 만들어 계층 구조를 갖게 하는 방법 대신 쓸 수 있는 유연한 대안이 될 수 있다.

### Problem

개발 중인 서비스에 알림 기능을 수행하는 `Notifier`가 있고, 처음에는 알림이 생성되면 유저의 이메일로 알려주는 기능만을 구현되어 있다. 하지만 유저의 요구에 따라 다른 알림 수단들 (SMS, Facebook, Slack 등) 도 추가된다. 이를 표현하는 간단한 방법은 `Notifier`라는 Super-class가 있고, 각 알림 수단이 하나의 Sub-class가 되도록 구조를 짜는 것이다.

![Decorator Problem[^1]](images/decorator-problem2.png)

그러나 만약 유저가 여러 알림 수단들을 동시에 원한다면? 위의 계층적인 구조로는 해결하기 어렵다. 굳이 계층 구조로 표현하고 싶다면 아래처럼 복잡한 형태를 띄게 된다.

![Decorator Prolem[^1]](images/decorator-problem3.png)

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

![Decorator Structure[^1]](images/decorator-structure.png)

1. `Component`는 Wrapper와 Wrappee 객체의 공통 인터페이스다.
2. `Concrete Component`는 기본 작업을 수행하며, Decorator들에 의해 Wrapping될 객체이다.
3. `Base Decorator`는 Wrapping된 객체를 연결시켜주는 클래스이다. 내부 객체를 참조하는 필드값은 `Component` 인터페이스이어야 한다.
4. `Concrete Decoratro`는 추가 작업을 수행하며, 동적으로 추가될 수 있는 객체이다.
5. `Client`는 `Concrete Component` 객체를 생성한 후, 상황에 따라 `Concrete Decorator` 객체를 추가할 수 있다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/decorator)

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

[^1]: [Decorator Origin](https://refactoring.guru/design-patterns/decorator)
