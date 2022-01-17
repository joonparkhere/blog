---
title: "[디자인 패턴 GURU] Command 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Command 패턴
date: 2022-01-15
pin: false
image: images/command-en.png
tags:
- Software Engineering
- Design Pattern
- Go
- Java
---

## Command

### Intent

Command 패턴은 클라이언트가 보내는 요청을 별도의 객체화하는 방법이다. 이를 통해 요청 객체를 메서드 파라미터로 전달하거나 요청의 실행을 제어하는 등의 처리가 가능해진다.

### Problem

![Problem Structure[^1]](images/command-problem2.png)

텍스트 에디터 서비스를 만드는 경우를 예시로 설명하자면, 다양한 동작들이 필요하겠지만, 그 중에서도 툴바에 있는 `Button`에 대해 살펴보고자 한다. 다양한 동작을 수행하는 `Button`이 있을 수 있다. 각 역할을 분리하기 위해 Sub-class화하는 방안을 떠올릴 수 있다. 그러나 기능이 늘어날 수록 유지보수가 어려워지며, 향후 `Button` 인터페이스에 대한 수정이 생길 경우 모든 Sub-class들의 수정을 해야하는 문제가 있다.

### Solution Structure

![Brief Solution Structure[^1]](images/command-solution3-en.png)

효과적인 소프트웨어 디자인에 있어서 중요한 것 중 하나는 관심사 분리 원칙 (Principle of Separation of Concerns) 를 지키는 것이다. Command 패턴은 요청을 직접 전달하지 않고, 관련 정보들을 객체화해서 파라미터로 전달한다. 각 동작들이 동일한 인터페이스를 따라야하며, 이를 통해 다양한 요청들을 구현체와의 커플링 없이 사용할 수 있게 된다.

![Solution Structure[^1]](images/command-structure.png)

1. `Client`는 `Concrete Command`를 만들고 `Invoker`를 통해 전달한다.

2. `Invoker` 클래스는 `Command` 객체를 저장한 후 `Receiver`에게 전달한다.

3. `Command` 인터페이스는 동작을 위한 단순 메서드가 존재한다.

   `Concrete Command`들은 다양한 요청을 구현한 객체이다. 이 객체 자체가 요청을 처리하도록 동작하지는 않지만, 관련 로직을 수행하는 (`Recevier`) 에게 전달한다.

4. `Recevier` 클래스는 동작을 수행하는 비즈니스 로직이다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/command)

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

Java의 `Runnable` 인터페이스가 실 적용 예제 중 하나이다.

```java
public interface Runnable {
    public abstract void run();
}
```

```java
public class ThreadCommand {
    public static void main(String args[]) {
        command = new MyRunnable()
        Thread t = new Thread(command);
        t.start()
    }
}

class MyRunnable implements Runnable {
    public void run() {
        // do something
    }
}
```

### Note

- 처리해야 하는 동작들을 객체로 만들어 파라미터화 시키고자 할 때 사용
- 요청을 스케줄링하거나 원격으로 처리해야 하는 경우 사용

[^1]: [Command Origin](https://refactoring.guru/design-patterns/command)
