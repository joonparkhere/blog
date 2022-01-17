---
title: "[디자인 패턴 GURU] Mediator 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Mediator 패턴
date: 2022-01-16
pin: false
image: images/mediator.png
tags:
- Software Engineering
- Design Pattern
- Go
---

## Mediator

### Intent

이 패턴은 여러 객체들간의 혼란스러운 의존 상황을 해결하고자 한다. 객체 서로 간의 직접적인 호출을 중지시키고 Mediator 객체를 통해서만 소통하도록 만든다.

### Problem

![Problem Diagram[^1]](images/mediator-problem1-en.png)

어느 서비스의 회원을 관리할 때 이용되는 Component들이다. 각 Component의 상황에 맞게 다른 Component의 필요유무가 달라진다. 예를 들어 회원이 견주라고 `Checkbox`에 표시할 경우, 반려동물에 대한 정보를 기록하는 `TextField`가 나타나는 꼴이다. 이런 상황이 쌓이고 점점 거대해질수록, 각 Component들은 서로의 구현에 의존하게 되고 관리가 어려워진다.

### Solution Structure

![Solution Diagram[^1]](images/mediator-solution1-en.png)

Mediator 패턴은 각 Component들간의 직접 호출을 제지시키고 Mediator 객체를 통해서 모든 소통을 하도록 만든다. 이렇게 함으로써 Component들은 오로지 Mediator 객체만을 의존하면 되어 이전보다 커플링 정도가 약해진다. 더불어 각 Component들은 다른 객체에 대한 정보를 모르고 소통할 수 있으므로 캡슐화 효과도 있다.

![Mediator Structure[^1]](images/mediator-structure.png)

1. `Component`들은 비즈니스 로직을 갖는 다양한 객체들이다. 각 객체는 `Mediator` 인터페이스를 참조함으로써 다른 객체와의 소통을 할 수 있다.
2. `Mediator` 인터페이스는 서로 다른 객체 간 소통을 할 수 있는 메서드를 정의한다.
3. `Concrete Mediator`은 `Component`들 간의 관계를 캡슐화한다. 보통 이 클래스는 모든 `Component`들을 참조하고 있으며, 생명주기를 관리하기도 한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/mediator)

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

![MVC Example[^2]](images/mediator-mvc.png)

MVC 패턴에서 **Front Controller (Dispatcher Servlet)**이 **Controller**와 **View Template** 사이의 Mediator 역할을 수행한다.

### Note

- 객체들간의 커플링 정도가 높아서 변경이 어려운 경우 사용
- 서로의 의존성이 높아서 다른 곳에서 재사용하기 어려운 경우 사용

> Mediator 객체는 God Object가 되어야 함을 유의해야 한다.

[^1]: [Mediator Origin](https://refactoring.guru/design-patterns/mediator)
[^2]: [DZone Article](https://dzone.com/articles/mediator-pattern-1)
