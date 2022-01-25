---
title: "[디자인 패턴 GURU] Memento 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Memento 패턴
date: 2022-01-19
pin: false
image: images/memento-en.png
tags:
- Software Engineering
- Design Pattern
- Go
- Java
---

## Memento

### Intent

이 패턴은 객체의 이전 상태를 저장하고 복구할 때 내부 구현을 드러내지 않고 수행하도록 하는 방법이다.

### Problem

![Snapshot Problem[^1]](images/memento-problem2-en.png)

만약 텍스트 에디터 서비스를 개발한다고 할 때, 어떤 동작을 하던간에 유저가 이전 상태로 되돌릴 수 있도록 스냅샷을 찍어 놓아야 한다. 즉 동작이 수행되기 전마다 현 상태의 모든 필드값을 저장해야 한다는 것이다. 문제는 저장해야 하는 객체에 `private` 필드가 있는 등의 문제가 있으면 온전한 상태 저장이 어렵다. 만약 우회해서 저장했더라도 스냅샷에 담긴 정보는 `public`하기 때문에 원래 `private`이었던 필드가 `public`하게 된다는 문제도 있다. 이렇게 되면 향후 Side Effect가 발생할 여지가 충분하다.

### Solution Structure

![Example Solution[^1]](images/memento-solution-en.png)

위에서 발생한 문제들은 모두 객체의 Encapsulation을 깨는 것에서 비롯된다. Memento 패턴은 각 상태 저장을 실 객체가 수행하도록 스냅샷 작업을 위임한다. `memento`라는 객체는 실 객체의 상태들을 복사해 저장하며, 실 객체를 제외한 다른 객체에는 접근할 수 없다. 다른 객체들은 `memento` 객체들과 인터페이스를 통해 소통하며 스냅샷에 필요한 데이터를 제한적으로 가져올 수 있으나, 본연의 상태들에는 제약이 있다. `memento` 객체들과 인터페이스를 통해 제약된 데이터만을 주고 받는 객체를 `caretaker`라고 한다.

구현 방법에는 크게 3종류가 있다. 우선 첫번째로, 클래스 안에 내부 클래스를 만드는 방법이다.

![Structure 1[^1]](images/memento-structure1.png)

1. `Originator` 클래스는 갖고 있는 상태들에 대해 스냅샷을 만드며, 필요할 때는 복구한다.

2. `Memento` 클래스는 값 객체로, 생성자를 통해서만 필드 값을 주입받아서 Immutable한 성질을 띤다.

   이 클래스는 `Originator` 클래스 내부에 위치하여 외부에서는 `Memento`의 메타데이터 등 일부 값들을 제외한 필드에 접근할 수 없다.

3. `Caretaker` 클래스는 `Originator`의 기록들을 스택 꼴로 저장한다. 복구가 필요할 때는 스택에서 꺼내 `Memento`를 전달한다.

두번째 구조는 중간에 인터페이스를 두어 구현한다.

![Structure 2[^1]](images/memento-structure2.png)

1. 내부 클래스 사용 없이 `ConcreteMemento` 클래스 접근에 제한을 두기 위해 `Memento` 인터페이스를 사용한다. 이 인터페이스는 `ConcreteMemento`의 메타데이터 등의 약간의 값만 가져올 수 있다.
2. `Originator` 클래스는 `ConcreteMemento` 클래스 내의 값들에 모두 접근 가능해야 한다. 따라서 `ConcreteMemento` 클래스의 필드 접근 제어자는 `public`이다. 즉, 실제로는 `ConcreteMemento` 클래스의 필드에 외부에서 접근할 수 있으나 `Memento` 인터페이스 이용을 권장하는 셈이다.

마지막 구조는 보다 외부의 접근을 제약하는 방법이다.

![Structure 3[^1]](images/memento-structure3.png)

1. `Originator` 인터페이스와 `Memento` 인터페이스를 만듦으로써 여러 종류의 구현체가 존재할 수 있다. 각 `ConcreteOriginator` 클래스는 짝이 맞는 `ConcreteMemento` 클래스가 존재한다.

   `ConcreteMemento` 객체 생성시 `ConcreteOriginator`를 전달해야 한다. 이를 통해 `ConcreteMemento` 클래스가 `ConcreteOriginaotr`상태 필드에 모두 접근할 수 있게 되고, 상태 복구가 가능해진다.

2. `Caretaker` 클래스는 `ConcreteMemento` 클래스 접근에 확실한 제약을 받게 된다. 더불어 `Originator` 구현과도 의존적이지 않게 된다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/memento)

```go
type Originator interface {
	Save() ConcreteMemento
}
```

```go
type Memento interface {
	Restore()
}
```

```go
type ConcreteOriginator struct {
	state string
}

func NewConcreteOriginator(s string) *ConcreteOriginator {
	return &ConcreteOriginator{
		state: s,
	}
}

func (o *ConcreteOriginator) GetState() string {
	return o.state
}

func (o *ConcreteOriginator) SetState(s string) {
	o.state = s
}

func (o *ConcreteOriginator) Save() ConcreteMemento {
	return NewConcreteMemento(o, o.state)
}
```

```go
type ConcreteMemento struct {
	origin *ConcreteOriginator
	state  string
}

func NewConcreteMemento(o *ConcreteOriginator, s string) ConcreteMemento {
	return ConcreteMemento{
		origin: o,
		state:  s,
	}
}

func (m *ConcreteMemento) GetSavedState() string {
	return m.state
}

func (m *ConcreteMemento) Restore() {
	m.origin.SetState(m.state)
}
```

```go
type Caretaker struct {
	mementos []ConcreteMemento
}

func NewCareTaker() *Caretaker {
	return &Caretaker{
		mementos: make([]ConcreteMemento, 0),
	}
}

func (c *Caretaker) AddMemento(m ConcreteMemento) {
	c.mementos = append(c.mementos, m)
}

func (c *Caretaker) GetMemento(idx int) ConcreteMemento {
	return c.mementos[idx]
}
```

```go
func TestAfter(t *testing.T) {
	caretaker := NewCareTaker()
	originator := NewConcreteOriginator("A")

	fmt.Printf("Originator Current State: %s\n", originator.GetState())
	caretaker.AddMemento(originator.Save())

	originator.SetState("B")
	fmt.Printf("Originator Current State: %s\n", originator.GetState())
	caretaker.AddMemento(originator.Save())

	originator.SetState("C")
	fmt.Printf("Originator Current State: %s\n", originator.GetState())
	caretaker.AddMemento(originator.Save())

	memento := caretaker.GetMemento(1)
	memento.Restore()
	fmt.Printf("Restored to State: %s\n", originator.GetState())
}
```

### Real Example

- `java.util.Date`
- `java.io.Serializable`

### Note

- 이전 상태를 저장해 복구할 소요가 있을 때 사용
- 객체의 Getter, Setter 등이 Encapsulation을 위반할 때 사용

> 동적인 프로그래밍 언어들에서는 Memento 클래스에 대한 확실한 접근 제어가 어렵다.

[^1]: [Memento Origin](https://refactoring.guru/design-patterns/memento)
