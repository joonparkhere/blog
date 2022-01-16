---
title: "[디자인 패턴 GURU] Bridge 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Bridge 패턴
date: 2022-01-05
pin: false
image: images/bridge.png
tags:
- Software Engineering
- Design Pattern
- Go
---

## Bridge

### Intent

Bridge 패턴은 규모가 큰 클래스나 얀관성이 높은 클래스들에 대해 구조적인 변화를 주어, 각각 독립적으로 개발될 수 있도록 돕는다.

### Problem

이 패턴은 **Abstraction** & **Implementation**과 크게 다르지 않다. 책에서 설명하는 예제는 아래와 같다.

`Shape`라는 클래스가 있고, 하위 클래스로 `Circle`과 `Square`이 있다. 여기서 `Red`와 `Blue`를 추가하려 한다. 이런 경우 총 4개의 하위 클래스가 존재하게 된다.

![Bridge 예제[^1]](images/bridge-problem-en.png)

### Solution Structure

Bridge 패턴은 하나의 큰 클래스 묶음을 작은 단위로 쪼갠 후, 한 작은 단위의 클래스 묶음이 다른 묶음을 호출 (소유) 하도록 한다.

![Bridge 예제 문제 해결 구조[^1]](images/bridge-solution-en.png)

이렇게 작은 단위로 묶음을 쪼개면, 추후 발생할 확장 및 수정에 대해 더 유연해진다. 아래의 그림이 잘 와닿게 설명한다.

![Bridge 이점[^1]](images/bridge-3-en.png)

>  이 책에서는 Abstraction과 Implementation 용어가 너무 학술적인 면이 있고, 와닿지 않는 다는 점을 들며 위의 예제를 통해 설명했다고 한다. 그러나 개인적으로는 근본 용어의 정의와 쓰임을 아는 게 더 좋다고 생각한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/bridge)

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

[^1]: [Bridge Origin](https://refactoring.guru/design-patterns/bridge)
