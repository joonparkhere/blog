## Vistor

### Intent

이 패턴은 분리되어 있는 로직을 처리해야 하는 객체가 수행할 수 있도록 하는 방법이다.

### Problem

지리적 정보가 담긴 데이터를 XML 파일로 추출하려 한다. 지리적 정보에는 각 건물의 특성, 건물 간 연결되어 있는 도로 등등이 각 역할과 책임에 맞게 객체화되어 있는 상태이다. 이때 지리적 정보를 담당하는 객체에 XML 형태로 필드값을 뽑아내는 메서드를 추가하면 손쉽게 할 수 있지만, 객체가 비대해지고 책임이 모호해지는 것을 막기 위해 기존 객체에 손대지 않고 외부 로직을 추가하려 한다.

### Solution Structure

Vistor 패턴은 취해야 하는 동작을 별도의 클래스 **Visitor**로 분리하고, 행동을 처리해야 하는 객체가 Vistor의 메서드를 호출하도록 하여 기존 코드 수정을 최소화한다.

![Solution Structure[^1]](images/visitor-structure-en.png)

1. `Visitor` 인터페이스는 `Element` 종류에 따라 수행해야 하는 로직을 담은 메서드들을 오버로딩을 이용해 정의한다.

2. `Element` 인터페이스는 `Visitor`를 호출할 수 있는 메서드를 정의한다.

   이 메서드를 통해 적절한 동작을 `Visitor`에게 위임할 수 있게 된다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/visitor)

`Square`, `Circle`, `Rectangle` 세 종류의 도형이 있을 때, 넓이를 계산하거나 무게중심을 찾는 과정을 Visitor 패턴을 통해 처리하는 예제이다.

```go
type Shape interface {
	GetType() string
	Accept(Visitor)
}
```

```go
type Visitor interface {
	VisitForSquare(*Square)
	VisitForCircle(*Circle)
	VisitForRectangle(*Rectangle)
}
```

- Go언어는 오버로딩을 지원하지 않기 때문에 메서드 명을 모두 다르게 해야한다.

```go
type Square struct {
	side int
}

func NewSquare(side int) *Square {
	return &Square{
		side: side,
	}
}

func (s *Square) Accept(v Visitor) {
	v.VisitForSquare(s)
}

func (s *Square) GetType() string {
	return "Square"
}
```

```go
type Circle struct {
	radius int
}

func NewCircle(r int) *Circle {
	return &Circle{
		radius: r,
	}
}

func (c *Circle) Accept(v Visitor) {
	v.VisitForCircle(c)
}

func (c *Circle) GetType() string {
	return "Circle"
}
```

```go
type Rectangle struct {
	height int
	width  int
}

func NewRectangle(h, w int) *Rectangle {
	return &Rectangle{
		height: h,
		width:  w,
	}
}

func (r *Rectangle) Accept(v Visitor) {
	v.VisitForRectangle(r)
}

func (r *Rectangle) GetType() string {
	return "Rectangle"
}
```

```go
type AreaCalculator struct {
	area float64
}

func NewAreaCalculator() *AreaCalculator {
	return &AreaCalculator{}
}

func (c *AreaCalculator) VisitForSquare(s *Square) {
	c.area = float64(s.side * s.side)
	fmt.Println("Calculating area for square")
}

func (c *AreaCalculator) VisitForCircle(s *Circle) {
	c.area = float64(s.radius*s.radius) * math.Pi
	fmt.Println("Calculating area for circle")
}

func (c *AreaCalculator) VisitForRectangle(s *Rectangle) {
	c.area = float64(s.height * s.width)
	fmt.Println("Calculating area for rectangle")
}
```

```go
type MiddleCoordinates struct {
	x float64
	y float64
}

func NewMiddleCoordinates() *MiddleCoordinates {
	return &MiddleCoordinates{}
}

func (c *MiddleCoordinates) VisitForSquare(s *Square) {
	fmt.Println("Calculating middle point coordinates for square")
}

func (c *MiddleCoordinates) VisitForCircle(s *Circle) {
	fmt.Println("Calculating middle point coordinates for circle")
}

func (c *MiddleCoordinates) VisitForRectangle(s *Rectangle) {
	fmt.Println("Calculating middle point coordinates for rectangle")
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	square := NewSquare(2)
	circle := NewCircle(3)
	rectangle := NewRectangle(2, 3)

	areaCalculator := NewAreaCalculator()

	square.Accept(areaCalculator)
	circle.Accept(areaCalculator)
	rectangle.Accept(areaCalculator)

	fmt.Println()

	middleCoordinates := NewMiddleCoordinates()

	square.Accept(middleCoordinates)
	circle.Accept(middleCoordinates)
	rectangle.Accept(middleCoordinates)
}
```

### Plus

간단히 의존성의 한 특성을 간략히 설명하고 Double Dispatch에 대해 설명하려 한다. 클래스 `A`와 `B`가 있고,  `B` 인스턴스에서 `A`의 메서드를 사용한다고 하자. 그러던 중, `A` 클래스의 메서드를 제거하면 `B`는 오류를 뱉는다. 이럴 때 `B`는 `A`를 의존하고 있다고 한다.

**Dispatch**는 어떤 메서드를 호출할 것인가를 결정하는 과정을 의미한다. 즉 메서드의 의존성을 결정하는 과정이라고 할 수 있다. 그 과정의 종류로 동적 디스패치와 정적 디스패치 두 종류가 있다.

- Static Dispatch

  ```java
  public class Person {
  	public void print() {
  		System.out.println("Hello");
  	}
  
  	public void print(String greeting) {
  		System.out.println(greeting);
  	}
  }
  ```

  ```java
  public class Main {
  	public static void main(String[] args) {
  		Person p = new Person();
  		p.print();
  		p.print("hi");
  	}
  }
  ```

  이 경우 컴파일 시점에 컴파일러가 어떤 메서드가 (바이트 코드가) 실행될 지 알고 있다.

- Dynamic Dispatch

  ```java
  public abstract class Job {
  	abstract void printJob();
  }
  
  public class Student extends Job {
  	@Override
  	public void printJob() {
  		System.out.println("Student");
  	}
  }
  
  public class Teacher extends Job {
  	@Override
  	public void printJob() {
  		System.out.println("Teacher");
  	}
  }
  ```

  ```java
  public class Main {
  	public static void main(String[] args) {
  		Job student = new Student();
  		Job teacher = new Teacher();
  
  		student.printJob();
  		teacher.printJob();
  	}
  }
  ```

  이 경우 바이트 코드를 보면 `Job` 클래스의 `printJob()`을 실행한다는 것만을 알 뿐, 어떤 구현체의 메서드인지는 모른다. 결국 어느 메서드를 실행시키는 지가 런타임 시점에 결정된다.

Visitor 패턴은 **Double Dispatch** 방법을 사용하는데, 먼저 이 방법을 적용하지 않은 단순 Dynamic Dispatch 예제 코드다.

```java
interface Post {
	void postOn(SNS sns);
}

static class Text implements Post {
	public void postOn(SNS sns) {
		if (sns instanceof Facebook) {
			System.out.println("facebook - text");
		}
		if (sns instanceof Twitter) {
			System.out.println("twitter - text");
		}
	}
}

static class Picture implements Post {
	public void postOn(SNS sns) {
		if (sns instanceof Facebook) {
			System.out.println("facebook - picture");
		}
		if (sns instanceof Twitter) {
			System.out.println("twitter - picture");
		}
	}
}
```

```java
interface SNS {
}

static class Facebook implements SNS {
}

static class Twitter implements SNS {
}
```

```java
public static void main(String[] args) {
	List<Post> postList = Arrays.asList(new Text(), new Picture());
	List<SNS> snsList = Arrays.asList(new Twitter(), new Facebook());
    
	postList.forEach(post -> snsList.forEach(sns -> post.postOn(sns)));
}
```

`Post` 객체의 `postOn()` 메서드에 `SNS` 객체를 파라미터로 전달하면 Dynamic Dispatch가 적용되어 `Text`인지 `Picture`인지 판단하고 알맞은 클래스의 `postOn()` 메서드를 실행하는 과정이 런타임 시점에 이뤄진다. 이후 `postOn()` 메서드에서는 `SNS` 객체 타입을 `instanceOf()`를 이용해 분기한다. 이는 곧 `Post`가 `SNS`에 대한 의존성이 있다는 것이고, 코드 변경에 취약하다.

그래서 나온 해결책이 Double Dispatch이다.

```java
interface Post { 
	void postOn(SNS sns);
}

static class Text implements Post { 
	public void postOn(SNS sns) {
		sns.post(this);
	}
}

static class Picture implements Post {
	public void postOn(SNS sns) {
		sns.post(this);
	}
}
```

```java
interface SNS {
	void post(Text text);
	void post(Picture picture);
}

static class Facebook implements SNS {
	@Override
	public void post(Text text) {
		System.out.println("facebook - text");
	}

	@Override
	public void post(Picture picture) {
		System.out.println("facebook - picture");
	}
}

static class Twitter implements SNS {
	@Override
	public void post(Text text) {
		System.out.println("twitter - text");
	}

	@Override
    public void post(Picture picture) {
		System.out.println("twitter - picture");
	}
}
```

```java
public static void main(String[] args) {
	List<Post> postList = Arrays.asList(new Text(), new Picture());
	List<SNS> snsList = Arrays.asList(new Twitter(), new Facebook());

	postList.forEach(post -> snsList.forEach(sns -> post.postOn(sns)));
}
```

위의 예제에서 `SNS`인터페이스에 `post()` 메서드가 추가되어, 실 로직을 `SNS` 객체에서 책임진다. 첫번째 `postOn()` 메서드가 호출될 때 `Text`인지 `Picture`인지 결정되며 첫번째 Dynamic Dispatch가 이뤄지고, 메서드 내에서 `sns.post()` 메서드가 호출되면서 두번째 Dynamic Dispatch가 일어난다. Double Dispatch를 사용하면 향후 `SNS` 구현체가 하나 추가되어도 `Post` 객체에 코드 변경이 일어나지 않는다. 즉 `Post`와 `SNS` 사이의 의존성이 제거된 상태이다.

### Note

- 복잡한 객체 구조에서 특정 작업을 처리해야 하는 경우 사용

> Element 수정에 따라 모든 Visitor들을 수정해야 하는 소요가 있다.

[^1]: [Visitor Origin](https://refactoring.guru/design-patterns/visitor)
[^2]: [alkhwa-113 Tistory Post](https://alkhwa-113.tistory.com/entry/%EB%94%94%EC%8A%A4%ED%8C%A8%EC%B9%98-%EB%8B%A4%EC%9D%B4%EB%82%98%EB%AF%B9-%EB%94%94%EC%8A%A4%ED%8C%A8%EC%B9%98-%EB%8D%94%EB%B8%94-%EB%94%94%EC%8A%A4%ED%8C%A8%EC%B9%98)