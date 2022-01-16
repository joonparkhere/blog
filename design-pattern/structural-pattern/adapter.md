---
title: "[디자인 패턴 GURU] Adapter 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Adapter 패턴
date: 2022-01-05
pin: false
image: images/adapter-en.png
tags:
- Software Engineering
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

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/adapter)

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

