---
title: TDD로 배우는 Go언어
date: 2021-06-16
pin: false
tags:
- Go
- TDD
---

# TDD로 배우는 Golang 

[Learn Go with tests Gitbook](https://quii.gitbook.io/learn-go-with-tests/) 을 토대로 기초적인 문법을 TDD (Test-Drived Development) 로 배울 생각이다. (우선은 문법을 익히는 용도이므로 일부 챕터만 정리할 예정)



## Hello, world

### 구성

- 먼저 새로운 디렉토리를 만들어서, `hello.go` 파일 안에 아래처럼 코드를 작성하자.


```go
package main

import "fmt"

func Hello() string {
	return "Hello, world"
}

func main() {
	fmt.Println(Hello())
}
```

```go
package main

import "testing"

func TestHello(t *testing.T) {
	got := Hello()
	want := "Hello, world"

	if got != want {
		t.Errorf("got %q want %q", got, want)
	}
}
```

- 같은 디렉토리 상에 `hello_test.go` 파일을 만들어서 테스트 코드를 작성하자.
- `t.Errorf()`에서 사용하는 포맷팅 변수는 [Golang Docs](https://golang.org/pkg/fmt/#hdr-Printing)에서 확인할 수 있다.
- JetBrain사의 Goland IDE를 사용하면 조금 더 편리하게 작성할 수 있다.

### Go modules

그리고 위 파일이 위치한 디렉토리에서 터미널에 `go test`를 실행시켜보면, 오류가 날 수 있다. 그럴 경우, `go mod init [module-name]`을 입력해서 `go.mod` 파일을 생성하고 다시 `go test`를 실행시키면 테스트 코드가 실행되며 이상없이 통과한다.

### 테스트 작성법

테스트 코드를 작성할 때 다음과 같은 규칙이 있다.

- `xxx_test.go` 와 같은 네이밍 규칙을 지켜야 한다.
- 코드 내의 테스트 함수는 `Test`으로 시작해야 한다.
- 테스트  함수는 오직 하나의 인자로 `t *testing.T`를 받아온다.
- `*testing.T`를 사용하기 위해서는 `"test"` 패키지를 import 해야 한다.

추가로, 터미널에 `godoc -http :8000`을 실행시키면 `localhost:8000/pkg`에서 로컬 PC에 설치된 패키지들의 목록과 Docs들을 읽어볼 수 있다.

### Hello, YOU

위에서 작성한 코드를 조금 더 동적으로 동작하도록 약간씩 바꿔보자. 이제부터는 TDD 방식 그대로, 먼저 테스트 코드를 작성하고 컴파일 에러를 해치우고, 리팩토링하는 순서로 진행하자.

```go
package main

import "testing"

func TestHello(t *testing.T) {
	got := Hello("Joon")
	want := "Hello, Joon"

	if got != want {
		t.Errorf("got %q want %q", got, want)
	}
}
```

- `Hello()` 메서드에 인자를 추가해서 컴파일 되도록 바꾸자.

```go
package main

import "fmt"

const englishHelloPrefix = "Hello, "

func Hello(name string) string {
	return englishHelloPrefix + name
}

func main() {
	fmt.Println(Hello("world"))
}
```

### Hello, world... again

```go
package main

import "testing"

func TestHello(t *testing.T) {
    
	t.Run("saying hello to people", func(t *testing.T) {
		got := Hello("Joon")
		want := "Hello, Joon"
		
		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})
	
	t.Run("say 'Hello, world' when an empty string is supplied", func(t *testing.T) {
		got := Hello("")
		want := "Hello, world"
		
		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})
    
}
```

- 위처럼 테스트 함수 내부에 서브 테스트들을 실행시킬 수 있다.

  이는 한 메서드에서 다른 시나리오들을 테스트할 때 종종 사용되는 방식이다. 더불어서, 이렇게 작성하면 반복되는 코드들을 메서드로 추출하는 등의 방법을 통해 중복을 제거할 수 있다.

```go
package main

import "testing"

func TestHello(t *testing.T) {

	assertCorrectMessage := func(t testing.TB, got string, want string) {
		t.Helper()
		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	}

	t.Run("saying hello to people", func(t *testing.T) {
		got := Hello("Joon")
		want := "Hello, Joon"
		assertCorrectMessage(t, got, want)
	})

	t.Run("say 'Hello, world' when an empty string is supplied", func(t *testing.T) {
		got := Hello("")
		want := "Hello, world"
		assertCorrectMessage(t, got, want)
	})

}
```

- 에러 처리하는 `if`문 부분을 `assert` 메서드로 추출해서 중복을 제거했다.

```go
package main

import "fmt"

const englishHelloPrefix = "Hello, "

func Hello(name string) string {
	if name == "" {
		name = "world"
	}
	return englishHelloPrefix + name
}

func main() {
	fmt.Println(Hello("world"))
}
```

### More requirements

```go
package main

import "fmt"

const englishHelloPrefix = "Hello, "
const spanishHelloPrefix = "Hola, "
const frenchHelloPrefix = "Bonjour, "

func Hello(name string, language string) string {
	if name == "" {
		name = "world"
	}

	return greetingPrefix(language) + name
}

func greetingPrefix(language string) (prefix string) {
	switch language {
	case "spanish":
		prefix = spanishHelloPrefix
	case "french":
		prefix = frenchHelloPrefix
	default:
		prefix = englishHelloPrefix
	}
	return
}

func main() {
	fmt.Println(Hello("world", ""))
}
```

- 받아오는 인자를 하나 더 추가해서 사용 언어에 따라 다른 문장이 반환되도록 수정했다.
- `greetingPrefix()`는 함수 선언부에서 `prefix string`라고 명시함으로써 해당 함수의 반환값은 `string`형의 `prefix`임을 알 수 있다. 따라서 해당 함수에서 `return prefix`할 필요 없이 그냥 `return`이라고 쓰면 알아서 `prefix` 변수가 반환된다. `prefix` 변수의 초기값은 자료형에 따라 다른데, `int`인 경우 `0` 이고 `string`인 경우 `""`이다.



## Integers

### 테스트 코드부터 작성

```go
package integers

import (
	"testing"
)

func TestAdder(t *testing.T) {
	sum := Add(2, 2)
	expected := 4

	if sum != expected {
		t.Errorf("expected '%d' but got '%d'", expected, sum)
	}
}
```

- `adder_test.go` 파일이 컴파일 에러가 나지 않도록 `adder.go`파일 작성

```go
package integers

func Add(x, y int) int {
	return x + y
}
```

### 예제

```go
package integers

import (
	"fmt"
	"testing"
)

func TestAdder(t *testing.T) {
	sum := Add(2, 2)
	expected := 4

	if sum != expected {
		t.Errorf("expected '%d' but got '%d'", expected, sum)
	}
}

func ExampleAdd() {
	sum := Add(1, 5)
	fmt.Println(sum)
	// Output: 6
}
```

- `xxx_test.go`에 `Example`로 시작하는 함수를 작성해서 해당 테스트 코드에 대한 예제를 적을 수 있다. 마지막에 주석으로 `// Output`을 적어야 예제라고 인식된다. 테스트 코드와 예제 코드를 한 번에 실행시키려면 터미널에 `go test -v`를 실행하면 된다.



## Iteration

Go언어에는 반복문 문법으로 오로지 `for`문만 존재한다.

### 테스트 코드 작성

```go
package iteration

import "testing"

func TestRepeat(t *testing.T) {
	repeated := Repeat("a")
	expected := "aaaaa"

	if repeated != expected {
		t.Errorf("expected %q but got %q", expected, repeated)
	}
}
```

```go
package iteration

const repeatCount = 5

func Repeat(target string) string {
	var repeated string
	for i := 0; i < repeatCount; i++ {
		repeated += target
	}
	return repeated
}
```

### Benchmarking

테스트 코드에 다음과 같이 함수를 작성하면 테스트 대상 함수의 성능 측정을 할 수 있다.

```go
package iteration

import "testing"

func BenchmarkRepeat(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Repeat("a")
	}
}
```

```
[결과]
goos: windows
goarch: amd64
pkg: iteration
cpu: Intel(R) Core(TM) i5-10210U CPU @ 1.60GHz
BenchmarkRepeat
BenchmarkRepeat-8   	 3867487	       358.9 ns/op
PASS
```

- `3867487`번 실행되고 평균 소요 시간이 `358.9` 나노세컨즈임을 확인할 수 있다.



## Arrays and Slices

### 테스트 코드 - Arrays

```go
package sum

import "testing"

func TestSum(t *testing.T) {
	numbers := [5]int{1, 2, 3, 4, 5}

	got := Sum(numbers)
	want := 15

	if got != want {
		t.Errorf("got %d want %d given, %v", got, want, numbers)
	}
}
```

- 배열은 길이가 정해져 있고, 다음과 같이 두 가지 방법으로 선언할 수 있다.
  - `[N]type{value1, value2, ... , valueN}` e.g. `[5]int{1, 2, 3, 4, 5}`
  - `[...]type{value1, value2, ... , valueN}` e.g. `[...]int{1, 2, 3, 4, 5}`

```go
package sum

func Sum(numbers [5]int) int {
	sum := 0
	//for i := 0; i < 5; i++ {
	//	sum += numbers[i]
	//}
	for _, number := range numbers {
		sum += number
	}
	return sum
}
```

- 앞서 배운 `for`문을 돌면서 동작하도록 구현하면 되는데, 여기서 `range`는 해당 배열의 인덱스와 값을 반환한다.
- 변수를 `_`으로 선언하면 해당 값은 무시하겠다는 의미로, 위 코드에서는 배열의 인덱스를 무시하고 있다.
- `Sum()`은 인자로 크기가 5인 `int`형 배열을 받아오는데, 다른 자료형 배열을 넘기면 당연히 컴파일 오류가 뜨고 더불어 길이만 다른 배열을 넘기더라도 컴파일 오류가 뜬다. (다른 타입으로 인식)

### 테스트 코드 - Slices

```go
package sum

import "testing"

func TestSum(t *testing.T) {

	t.Run("collection of 5 numbers", func(t *testing.T) {
		numbers := []int{1, 2, 3, 4, 5}

		got := Sum(numbers)
		want := 15

		if got != want {
			t.Errorf("got %d want %d given, %v", got, want, numbers)
		}
	})

	t.Run("collection of any size", func(t *testing.T) {
		numbers := []int{1, 2, 3}

		got := Sum(numbers)
		want := 6

		if got != want {
			t.Errorf("got %d want %d given, %v", got, want, numbers)
		}
	})

}
```

- 슬라이스는 크기 제한 없이 컬렉션을 사용할 수 있다.

```go
package sum

func Sum(numbers []int) int {
	sum := 0
	for _, number := range numbers {
		sum += number
	}
	return sum
}
```

- 배열과 슬라이스는 엄연히 구분되는 타입이므로, `Sum()`이 인자로 슬라이스를 받아오도록 수정한다.

### 테스트 커버리지

테스트 케이스가 코드를 얼마나 커버하고 있는지 확인하려면 터미널에 `go test -cover` 명령을 입력하면 된다. 이를 통해 중복된 테스트 케이스를 제거할 수 있다.

### 테스트 코드 - Slices 활용 1

```go
func TestSumAll(t *testing.T) {
	got := SumAll([]int{1, 2}, []int{0, 9})
	want := []int{3, 9}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v want %v", got, want)
	}
}
```

- 여러 슬라이스들을 인자로 넘겨 각각의 총합을 반환하는 함수를 구현하고자 한다.
- 슬라이스간의 비교는 `==`로 불가능하므로, 반복을 해서 값들을 비교하거나 `reflect.DeepEqual()`과 같은 라이브러리를 이용하자.

```go
func SumAll(numbersToSum ...[]int) (sums []int) {
	lengthOfNumbers := len(numbersToSum)
	sums = make([]int, lengthOfNumbers)

	for i, numbers := range numbersToSum {
		sums[i] = Sum(numbers)
	}

	return
}
```

- 인자로 `...`를 사용하면, 여러 인자를 받아올 수 있다.

```go
func SumAll(numbersToSum ...[]int) (sums []int) {
	for _, numbers := range numbersToSum {
		sums = append(sums, Sum(numbers))
	}
	return
}
```

- `SumAll()`를 조금 더 리팩터링하면, `append()`를 사용해서 슬라이스들을 이을 수 있다.

### 테스트 코드 - Slices 활용 2

```go
func TestSumAllTails(t *testing.T) {
	got := SumAllTails([]int{1, 2}, []int{0, 9})
	want := []int{2, 9}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v want %v", got, want)
	}
}
```

- 인자로 받아오는 각 슬라이스 중에서 뒷부분 일부만을 더한 값들을 반환하는 함수를 구현하고자 한다.

```go
func SumAllTails(numbersToSum ...[]int) []int {
	var sums []int
	for _, numbers := range numbersToSum {
		tail := numbers[1:]
		sums = append(sums, Sum(tail))
	}
	return sums
}
```

- `numbers[1:]`처럼 슬라이스 중 일부를 조각 내어 슬라이스를 만들 수 있다. 지금의 경우, 인덱스 값 `1`부터 마지막까지를 조각내어 슬라이스를 만들고 있다.

```go
func TestSumAllTails(t *testing.T) {

	checkSums := func(t testing.TB, got, want []int) {
		t.Helper()
		if !reflect.DeepEqual(got, want) {
			t.Errorf("got %v want %v", got, want)
		}
	}

	t.Run("make the sums of some slices", func(t *testing.T) {
		got := SumAllTails([]int{1, 2}, []int{0, 9})
		want := []int{2, 9}
		checkSums(t, got, want)
	})

	t.Run("safely sum empty slices", func(t *testing.T) {
		got := SumAllTails([]int{}, []int{3, 4, 5})
		want := []int{0, 9}
		checkSums(t, got, want)
	})

}
```

- 발생할 수 있는 예외 케이스 (슬라이스가 비어있는 경우) 를 추가로 작성하기 위해 테스트 코드를 리팩터링하자.

```go
func SumAllTails(numbersToSum ...[]int) []int {
	var sums []int
	for _, numbers := range numbersToSum {
		if len(numbers) == 0 {
			sums = append(sums, 0)
			continue
		}
		tail := numbers[1:]
		sums = append(sums, Sum(tail))
	}
	return sums
}
```

- 슬라이스가 빈 경우에 대한 예외 처리 로직을 추가했다.



## Struts Methods, Interfaces

### 단순 테스트 코드

```go
func TestPerimeter(t *testing.T) {
	got := Perimeter(10.0, 10.0)
	want := 40.0

	if got != want {
		t.Errorf("got %.2f want %.2f", got, want)
	}
}

func TestArea(t *testing.T) {
	got := Area(12.0, 6.0)
	want := 72.0
	
	if got != want {
		t.Errorf("got %.2f want %.2f", got, want)
	}
}
```

- 둘레와 넓이를 계산하는 함수를 구현하고자 한다.

```go
func Perimeter(width, height float64) float64 {
	return (width + height) * 2
}

func Area(width, height float64) float64 {
	return width * height
}
```

- 위처럼 간단히 구현할 수 있지만, 대상이 사각형임을 명시하기 위해 구조체를 도입한다.

### 구조체

```go
type Rectangle struct {
	Width float64
	Height float64
}
```

```go
func TestPerimeter(t *testing.T) {
	rectangle := Rectangle{10.0, 10.0}
	got := Perimeter(rectangle)
	want := 40.0

	if got != want {
		t.Errorf("got %.2f want %.2f", got, want)
	}
}

func TestArea(t *testing.T) {
	rectangle := Rectangle{12.0, 6.0}
	got := Area(rectangle)
	want := 72.0

	if got != want {
		t.Errorf("got %.2f want %.2f", got, want)
	}
}
```

- `Rectangle{10.0, 10.0}`과 같이 구조체 타입을 초기화할 수 있다.

```go
func Perimeter(rectangle Rectangle) float64 {
	return (rectangle.Width + rectangle.Height) * 2
}

func Area(rectangle Rectangle) float64 {
	return rectangle.Width * rectangle.Height
}
```

- 테스트 케이스가 통과하도록 함수 선언부도 수정해준다.

그리고 다른 형태의 도형을 추가해보자.

```go
func TestArea(t *testing.T) {

	t.Run("rectangles", func(t *testing.T) {
		rectangle := Rectangle{12, 6}
		got := Area(rectangle)
		want := 72.0

		if got != want {
			t.Errorf("got %.2f want %.2f", got, want)
		}
	})

	t.Run("circles", func(t *testing.T) {
		circle := Circle{10}
		got := Area(circle)
		want := 314.1592653589793

		if got != want {
			t.Errorf("got %g want %g", got, want)
		}
	})

}
```

- 원을 표현하는 구조체 구현이 필요하다.

```go
type Circle struct {
	Radius float64
}
```

여기서 문제가 발생한다. `Area()`라는 함수는 도형의 종류에 따라 동작하는 로직이 달라서 구현체가 다르다. 그러나 Go언어에서는 다음과 같은 오버로딩을 지원하지 않는다.

```go
func Area(rectangle Rectangle) float64 { /* ... */ }
func Area(circle Circle) float64 { /* ... */ }
```

따라서 가능한 선택지는 두 가지다.

- 구현해야 하는 `Area()` 함수를 각각 다른 패키지에 위치하도록 한다. 그러나 이 경우 패키지를 추가로 구성하는 것은 과도한 방안이다.
- 각 타입별로 메서드를 정의한다.

### 메서드

메서드는 대상이 있는 함수이다. 바로 테스트 케이스를 작성해서 살펴보자. 

```go
func TestArea(t *testing.T) {

	t.Run("rectangles", func(t *testing.T) {
		rectangle := Rectangle{12, 6}
		got := rectangle.Area()
		want := 72.0

		if got != want {
			t.Errorf("got %.2f want %.2f", got, want)
		}
	})

	t.Run("circles", func(t *testing.T) {
		circle := Circle{10}
		got := circle.Area()
		want := 314.1592653589793

		if got != want {
			t.Errorf("got %g want %g", got, want)
		}
	})

}
```

- 각 타입별 메서드를 선언해주자.

```go
type Rectangle struct {
	Width float64
	Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}
```

- 메서드 선언와 함수 선언의 차이점은 대상의 유무이다. 메서드의 경우 `func (receiverName ReceiverType) MethodName(args)` 형태로, 앞부분에 추가로 대상을 명시한다.
- 더불어 일종의 약속으로, `receiverName`은 `ReceiverType`의 맨 앞글자를 따와서 네이밍을 한다. (e.g. `r Rectangle`)

### 인터페이스

위의 테스트 케이스에서 `Area()` 를 호출하는 부분에 중복이 발생한다. 이를 하나로 묶어서 추상화하기 위해 인터페이스를 도입한다. 먼저 테스트 케이스를 작성해서 살펴보자.

```go
func TestArea(t *testing.T) {

	checkArea := func(t testing.TB, shape Shape, want float64) {
		t.Helper()
		got := shape.Area()
		if got != want {
			t.Errorf("got %g want %g", got, want)
		}
	}

	t.Run("rectangles", func(t *testing.T) {
		rectangle := Rectangle{12, 6}
		checkArea(t, rectangle, 72.0)
	})

	t.Run("circles", func(t *testing.T) {
		circle := Circle{10}
		checkArea(t, circle, 314.1592653589793)
	})

}
```

- 테스트 케이스가 통과하도록 `Shape` 인터페이스를 선언해주자.

```go
type Shape interface {
	 Area() float64
}

type Rectangle struct {
	Width float64
	Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}
```

- 위처럼 `Shape` 인터페이스를 선언만 하더라도, 테스트 케이스가 통과한다. 이는 다른 언어의 인터페이스와는 구별되는 부분이다. 보통 `Rectangle`이 `Shape`의 구현체라고 명시를 하는 부분이 필요한데, Go언어의 경우 그렇지 않다. 단지 `Shape` 인터페이스의 `Area()` 메서드와 이름과 반환 타입이 같은 메서드를 가진다면 해당 구조체는 인터페이스의 구현체가 된다.

### 익명 구조체를 통한 테스트 리팩터링

```go
func TestArea(t *testing.T) {

	areaTests := []struct {
		shape Shape
		want float64
	} {
		{Rectangle{12, 6}, 72.0},
		{Circle{10}, 314.1592653589793},
	}

	for _, tt := range areaTests {
		got := tt.shape.Area()
		if got != tt.want {
			t.Errorf("got %g want %g", got, tt.want)
		}
	}

}
```

- 익명 구조체를 선언해서 테스트 케이스를 보다 깔끔하게 만들 수 있다.
- 더불어, 다른 도형 (구조체 타입) 이 추가되더라도 코드 중복 없이 테스트 케이스를 작성할 수 있게 된다.

삼각형을 추가해보자.

```go
func TestArea(t *testing.T) {

	areaTests := []struct {
		shape Shape
		want float64
	} {
		{Rectangle{12, 6}, 72.0},
		{Circle{10}, 314.1592653589793},
		{Triangle{12, 6}, 36.0},
	}

	for _, tt := range areaTests {
		got := tt.shape.Area()
		if got != tt.want {
			t.Errorf("got %g want %g", got, tt.want)
		}
	}

}
```

```go
type Triangle struct {
	Base float64
	Height float64
}

func (t Triangle) Area() float64 {
	return (t.Base * t.Height) * 0.5
}
```

더불어 테스트 케이스를 다음과 같이 보다 명시적으로 표현할 수 있다.

```go
func TestArea(t *testing.T) {

	areaTests := []struct {
		shape Shape
		want float64
	} {
		{shape: Rectangle{12, 6}, want: 72.0},
		{shape: Circle{10}, want: 314.1592653589793},
		{shape: Triangle{12, 6}, want: 36.0},
	}

	for _, tt := range areaTests {
		got := tt.shape.Area()
		if got != tt.want {
			t.Errorf("%#v got %g want %g", tt.shape, got, tt.want)
		}
	}

}
```

- 구조체 선언 부분에서 해당 값이 어떤 값을 의미하는지 명시할 수 있다. (e.g. `{shape: Rectangle{12, 6}, want: 72.0}`)
- 에러를 처리하는 부분에서 `%#v`를 사용해서 어떤 구조체에서 에러가 발생했는지 정확하게 명시할 수 있다.



## Pointer, Errors

구조체를 이용하여 지갑 예제를 만들어 보자. 먼저 테스트 코드부터 작성하자.

### 예제 기본 구현 - Deposit

```go
func TestWallet(t *testing.T) {
	wallet := Wallet{}
	wallet.Deposit(10)

	got := wallet.Balance()
	want := 10

	if got != want {
		t.Errorf("got %d want %d", got, want)
	}
}
```

- 컴파일 에러에서 벗어나기 위해 지갑 구조체와 메서드들을 구현하자.

```go
type Wallet struct {
	balance int
}

func (w Wallet) Deposit(amount int) {
	w.balance += amount
}

func (w Wallet) Balance() int {
	return w.balance
}
```

- 이렇게 구현하고 테스트 코드를 실행하면, 컴파일 에러는 해결했지만 테스트를 통과하지 못한다.
- 보다 정확한 확인을 위해 직접 메모리 주소값을 찍어보자.

```go
func TestWallet(t *testing.T) {
	wallet := Wallet{}
	wallet.Deposit(10)

	got := wallet.Balance()

	fmt.Printf("address of balance in test is %v \n", &wallet.balance)

	want := 10

	if got != want {
		t.Errorf("got %d want %d", got, want)
	}
}
```

```go
func (w Wallet) Deposit(amount int) {
	fmt.Printf("address of balance in Deposit in %v \n", &w.balance)
	w.balance += amount
}
```

```
[결과]
address of balance in Deposit in 0xc00011a2a0 
address of balance in test is 0xc00011a298 
```

- 위처럼 서로 다른 메모리 주소에 접근해서 잔액 조회가 제대로 이뤄지지 않고 있다.

- Golang에서 함수를 호출할 때는 인자 값이 복사되어 전달되기 때문이다.

  즉 `Deposit()` 메서드에 전달되는 `w Wallet`은 값이 복사된 또 다른 지갑 구조체가 생성된 것이다.

- 지금 예제에서는 동일한 지갑에 대해 동작해야 하므로 포인터를 이용해서 같은 구조체에 접근하도록 할 수 있다.

### 포인터 활용

```go
func (w *Wallet) Deposit(amount int) {
	w.balance += amount
}

func (w *Wallet) Balance() int {
	return w.balance
}
```

- 이처럼 타입명 앞에 `*`를 붙이면 메모리 주소값이 전달된다.

- 다만 다른 언어와 다른 점은, `*w.balance`와 같이 포인터가 가르키는 값에 접근하기 위해 `*`를 사용하지 않아도 된다는 점이다.

  즉 명시적으로 표현하지 않아도 자동으로 Dereference가 일어난다.

- 더불어서 사실 `Balance()` 메서드는 굳이 포인터로 구조체를 전달받지 않아도 괜찮다. 다만 코드의 일관성을 위해 위 예제에서는 포인터를 이용하였다.

### 커스텀 타입

지갑에 들어갈 화폐를 `Bitcoin`이라고 할 때 해당 타입을 선언해보자. 먼저 테스트 코드부터 작성한다.

```go
func TestWallet(t *testing.T) {
	wallet := Wallet{}
	wallet.Deposit(Bitcoin(10))

	got := wallet.Balance()
	want := Bitcoin(10)

	if got != want {
		t.Errorf("got %s want %s", got, want)
	}
}
```

- 위처럼 지갑에 들어갈 것들을 `Bitcoin`으로 바꾼 후, `Bitcoin` 이라는 타입을 만들어주자.
- 커스텀 타입을 초기화 하려면 `Bitcoin(999)` 꼴로 사용하면 된다.

```go
type Bitcoin int

type Stringer interface {
	String() string
}

func (b Bitcoin) String() string {
	return fmt.Sprintf("%d BTC", b)
}

type Wallet struct {
	balance Bitcoin
}

func (w *Wallet) Deposit(amount Bitcoin) {
	w.balance += amount
}

func (w *Wallet) Balance() Bitcoin {
	return w.balance
}
```

- 구조체 필드와 메서드 인자들을 커스텀 타입으로 변경하자.
- 커스텀 타입의 `String()` 메서드를 재정의하려면 위처럼 `Stringer` 인터페이스 내의 `String()` 메서드 네이밍과 반환 타입을 지켜서 구현하면 된다.

### 예제 추가 구현 - Withdraw

출금을 위한 메서드 구현도 해보자. 먼저 테스트 코드를 조금 리팩터링 및 작성하고 난 후, 구현하자.

```go
func TestWallet(t *testing.T) {

	assertBalance := func(t testing.TB, wallet Wallet, want Bitcoin) {
		t.Helper()
		got := wallet.Balance()

		if got != want {
			t.Errorf("got %s want %s", got, want)
		}
	}

	t.Run("Deposit", func(t *testing.T) {
		wallet := Wallet{}
		wallet.Deposit(Bitcoin(10))
		assertBalance(t, wallet, Bitcoin(10))
	})

	t.Run("Withdraw", func(t *testing.T) {
		wallet := Wallet{balance: Bitcoin(20)}
		wallet.Withdraw(Bitcoin(10))
		assertBalance(t, wallet, Bitcoin(10))
	})

}
```

```go
func (w *Wallet) Withdraw(amount Bitcoin) {
	w.balance -= amount
}
```

- 만약 잔고보다 많은 금액을 출금하려한다면, 에러를 반환해야 한다. 이어서 이를 구현해보자.

### 에러

```go
func TestWallet(t *testing.T) {

	/* ... */
	
	assertError := func(t testing.TB, err error) {
		t.Helper()
		if err == nil {
			t.Error("wanted an error but did not get one")
		}
	}

	t.Run("Withdraw insufficient funds", func(t *testing.T) {
		startingBalance := Bitcoin(20)
		wallet := Wallet{startingBalance}
		err := wallet.Withdraw(Bitcoin(100))

		assertBalance(t, wallet, startingBalance)
		assertError(t, err)
	})

}
```

- `Withdraw()` 메서드가 제대로 동작하지 않으면 에러를 반환하도록 수정하고, 반환받은 에러를 검증하자.

```go
func (w *Wallet) Withdraw(amount Bitcoin) error {
	if amount > w.balance {
		return errors.New("oh no")
	}

	w.balance -= amount
	return nil
}
```

- 메서드에 반환값으로 `error`가 추가되었고, 에러를 반환할 경우 `errors.New()`를 사용하면 되며 정상적인 경우 `nil`을 반환하면 된다.
- `nil`은 다른 언어의 `null`가 유사한 개념이다.

에러에 대한 정보를 확실히 알 수 있도록 테스트 코드를 리팩터링 해보자.

```go
func TestWallet(t *testing.T) {

	/* ... */
	
	assertError := func(t testing.TB, got error, want string) {
		t.Helper()
		if got == nil {
			t.Fatal("did not get an error but wanted one")
		}
		if got.Error() != want {
			t.Errorf("got %q, want %q", got, want)
		}
	}

	t.Run("Withdraw insufficient funds", func(t *testing.T) {
		startingBalance := Bitcoin(20)
		wallet := Wallet{startingBalance}
		err := wallet.Withdraw(Bitcoin(100))

		assertBalance(t, wallet, startingBalance)
		assertError(t, err, "cannot withdraw, insufficient funds")
	})

}
```

- `t.Fatal()`이 실행되면 해당 테스트 케이스는 실행 중단된다.

```go
var ErrInsufficientFunds = errors.New("cannot withdraw, insufficient funds")

func (w *Wallet) Withdraw(amount Bitcoin) error {
	if amount > w.balance {
		return ErrInsufficientFunds
	}

	w.balance -= amount
	return nil
}
```

- 메서드에서 새로운 에러를 정의하고 반환하는 것보다는 코드 재사용성을 높이기 위해 외부에서 정의 후 가져다 쓰는 편이 좋은 패턴이다.

이제 전반적으로 테스트 코드를 깔끔하게 정리하고 다음 챕터로 넘어가자.

```go
func TestWallet(t *testing.T) {

	t.Run("Deposit", func(t *testing.T) {
		wallet := Wallet{}
		wallet.Deposit(Bitcoin(10))

		assertBalance(t, wallet, Bitcoin(10))
	})

	t.Run("Withdraw", func(t *testing.T) {
		wallet := Wallet{balance: Bitcoin(20)}
		err := wallet.Withdraw(Bitcoin(10))

		assertBalance(t, wallet, Bitcoin(10))
		assertNoError(t, err)
	})

	t.Run("Withdraw insufficient funds", func(t *testing.T) {
		wallet := Wallet{balance: Bitcoin(20)}
		err := wallet.Withdraw(Bitcoin(100))

		assertBalance(t, wallet, Bitcoin(20))
		assertError(t, err, "cannot withdraw, insufficient funds")
	})

}

func assertBalance(t testing.TB, wallet Wallet, want Bitcoin) {
	t.Helper()
	got := wallet.Balance()

	if got != want {
		t.Errorf("got %s want %s", got, want)
	}
}

func assertNoError(t testing.TB, got error) {
	t.Helper()
	if got != nil {
		t.Fatal("got an error but did not want one")
	}
}

func assertError(t testing.TB, got error, want string) {
	t.Helper()
	if got == nil {
		t.Fatal("did not get an error but wanted one")
	}
	if got.Error() != want {
		t.Errorf("got %q, want %q", got, want)
	}
}
```



## Map

배열과 슬라이스는 순서대로 요소가 보관된 구조라면, key-value 짝으로 보관되는 구조 Map에 대해 살펴보자.

### Serach 구현

```go
func TestDictionary(t *testing.T) {
	dictionary := map[string]string{"test": "this is just a test"}

	got := Search(dictionary, "test")
	want := "this is just a test"

	assertStrings(t, got, want)
}

func assertStrings(t testing.TB, got, want string) {
	t.Helper()
	
	if got != want {
		t.Errorf("got %q want %q given", got, want)
	} 
}
```

- `map`을 선언할 때 `[ ]`안에는 Key의 타입을, 밖에는 Value의 타입을 명시하면 된다.

  이때 Key 타입은 비교가능한 타입이어야 한다. Map에서 값을 찾거나 업데이트하는 동작을 하기 위해서 비교는 필수적이기 때문이다. Value 타입은 아무거나 상관없다. 또 다른 `map`이 될 수도 있다.

```go
func Search(dictionary map[string]string, word string) string {
	return dictionary[word]
}
```

커스텀 타입으로 `map`을 사용해보자.

```go
func TestDictionary(t *testing.T) {
	dictionary := Dictionary{"test": "this is just a test"}

	got := dictionary.Search("test")
	want := "this is just a test"

	assertStrings(t, got, want)
}

func assertStrings(t testing.TB, got, want string) {
	t.Helper()

	if got != want {
		t.Errorf("got %q want %q given", got, want)
	}
}
```

```go
type Dictionary map[string]string

func (d Dictionary) Search(word string) string {
	return d[word]
}
```

위에서는 `map`에 값이 들어있다는 전제하에 테스트 코드를 작성했다. 그렇지 않은 경우에 대한 처리도 해주어야 한다. 경우들을 판별할 수 있도록 `Search()` 메서드가 추가로 에러를 반환하도록 수정해야 한다.

```go
func TestDictionary(t *testing.T) {
	dictionary := Dictionary{"test": "this is just a test"}
	
	t.Run("known word", func(t *testing.T) {
		got, _ := dictionary.Search("test")
		want := "this is just a test"

		assertStrings(t, got, want)
	})
	
	t.Run("unknown word", func(t *testing.T) {
		_, err := dictionary.Search("unknown")
		want := "could not find the word you were looking for"
		
		assertError(t, err, ErrNotFound)		
		assertStrings(t, err.Error(), want)
	})
	
}

func assertStrings(t testing.TB, got, want string) {
	t.Helper()

	if got != want {
		t.Errorf("got %q want %q given", got, want)
	}
}

func assertError(t testing.TB, got, want error) {
	t.Helper()
	
	if got != want {
		t.Errorf("got error %q want %q", got, want)
	}
}
```

```go
var ErrNotFound = errors.New("could not find the word you were looking for")

type Dictionary map[string]string

func (d Dictionary) Search(word string) (string, error) {
	definition, ok := d[word]
	if !ok {
		return "", ErrNotFound
	}

	return definition, nil
}
```

- `d[word]`에서 반환하는 두 번째 값은 `boolean` 타입으로 성공 유무를 알려준다.
- 발생할 수 있는 에러의 재사용을 위해 외부에서 선언하였다.

### Add 구현

이번에는 `map`에 새로운 쌍을 넣는 메서드를 구현해보자.

```go
func TestAdd(t *testing.T) {
	dictionary := Dictionary{}
	dictionary.Add("test", "this is just a test")

	assertDefinition(t, dictionary, "test", "this is just a test")
}

func assertDefinition(t testing.TB, dictionary Dictionary, word, definition string) {
	t.Helper()

	got, err := dictionary.Search(word)
	if err != nil {
		t.Fatal("should find added word: ", err)
	}

	if definition != got {
		t.Errorf("got %q want %q", got, definition)
	}
}
```

```go
func (d Dictionary) Add(word, definition string) {
	d[word] = definition
}
```

- 메서드들을 구현하다보면 하나 의문점이 생긴다. 메서드에서 받아올 때, `d *Dictionary`가 아닌데도 key-value 쌍이 추가가 잘 반영된다.

  이는 `map` 타입의 속성 중 하나로, 마치 Reference 타입처럼 느껴지지만 사실은 그렇지 않다. 따라서 메서드에 `map`을 넘겨줄 때는 실제로 `map`을 복사하는 것이지만 오로지 포인터 부분만 복사하는 것이고 실제 자료 구조는 보존된다.

- 더불어서 `map`이 가르키는 실제 자료 구조는 `nil`일 수 있다. `nil map`은 읽기 작업을 마치 비어있는 `map`처럼 수행하지만, 쓰기 작업을 할 경우 런타임 에러가 발생한다.

  따라서 다음처럼 비어있는 `map`을 선언 및 초기화하면 안된다.

  ```go
  var m map[string]string
  ```

  이보다는 아래처럼 하는 것이 안전하다.

  ```go
  var m = map[string]string{}
  // 또는
  var m = make(map[string]string)
  ```

  위의 두 경우 모두 비어있는 `hash map`을 만들어주며 런타임 에러가 발생하지 않도록 도와준다.

이어서 이미 존재하는 키에 대해 `Add()` 메서드 동작을 구현해보자.

```go
func TestAdd(t *testing.T) {

	t.Run("new word", func(t *testing.T) {
		dictionary := Dictionary{}
		err := dictionary.Add("test", "this is just a test")

		assertError(t, err, nil)
		assertDefinition(t, dictionary, "test", "this is just a test")
	})

	t.Run("existing word", func(t *testing.T) {
		dictionary := Dictionary{"test": "this is just a test"}
		err := dictionary.Add("test", "new test")

		assertError(t, err, ErrWordExists)
		assertDefinition(t, dictionary, "test", "this is just a test")
	})

}

func assertError(t testing.TB, got, want error) {
	t.Helper()

	if got != want {
		t.Errorf("got error %q want %q", got, want)
	}
}

func assertDefinition(t testing.TB, dictionary Dictionary, word, definition string) {
	t.Helper()

	got, err := dictionary.Search(word)
	if err != nil {
		t.Fatal("should find added word: ", err)
	}

	if definition != got {
		t.Errorf("got %q want %q", got, definition)
	}
}
```

```go
var (
	ErrNotFound = errors.New("could not find the word you were looking for")
	ErrWordExists = errors.New("cannot add word because it already exists")
)

func (d Dictionary) Add(word, definition string) error {
	_, err := d.Search(word)

	switch err {
	case ErrNotFound:
		d[word] = definition
	case nil:
		return ErrWordExists
	default:
		return err
	}

	return nil
}
```

- `Search()` 메서드로 이미 존재하는 값인지 확인을 한다. 이때 기준이 되는 것은 반환받은 `err`이다.
- 추가로 발생할 수 있는 에러를 정의했다.

```go
const (
	ErrNotFound = DictionaryErr("could not find the word you were looking for")
	ErrWordExists = DictionaryErr("cannot add word because it already exists")
)

type DictionaryErr string

func (e DictionaryErr) Error() string {
	return string(e)
}
```

- 추가로 에러를 `const`로 만들기 위해, 커스텀 타입을 정의하고 `error` 인터페이스의 `Error()` 메서드를 구현했다.

### Update 구현

위에서 구현한 로직과 유사한 방법으로 `Update()` 메서드를 짜보자. 반복되는 내용이 많으므로 빠르게 진행하였다.

```go
func TestUpdate(t *testing.T) {
	
	t.Run("existing word", func(t *testing.T) {
		dictionary := Dictionary{"test": "this is just a test"}
		err := dictionary.Update("test", "new definition")
		
		assertError(t, err, nil)
		assertDefinition(t, dictionary, "test", "new definition")
	})
	
	t.Run("new word", func(t *testing.T) {
		dictionary := Dictionary{}
		err := dictionary.Update("test", "this is just a test")
		
		assertError(t, err, ErrWordDoesNotExists)
	})
	
}
```

```go
const (
	ErrWordDoesNotExists = DictionaryErr("cannot update word because it does not exist")
)

func (d Dictionary) Update(word, definition string) error {
	_, err := d.Search(word)

	switch err {
	case ErrNotFound:
		return ErrWordDoesNotExists
	case nil:
		d[word] = definition
	default:
		return err
	}

	return nil
}
```

### Delete 구현

간단한 로직만 구현하고 넘어도록 하자.

```go
func TestDelete(t *testing.T) {
	dictionary := Dictionary{"test": "test definition"}
	dictionary.Delete("test")

	_, err := dictionary.Search("test")
	if err != ErrNotFound {
		t.Errorf("Expected %q to be deleted", "test")
	}
}
```

```go
func (d Dictionary) Delete(word string) {
	delete(d, word)
}
```



## Concurrency

웹사이트에 접근 요청을 보내서 상태값을 받아오는 과정을 반복하는 코드를 통해 비동기 실행을 익혀보자.

```go
type WebsiteChecker func(string) bool

func CheckWebsites(wc WebsiteChecker, urls []string) map[string]bool {
	results := make(map[string]bool)

	for _, url := range urls {
		results[url] = wc(url)
	}

	return results
}
```

- `CheckWebsites()`는 인자로 넘어온 `WebsiteChecker`를 통해 `urls`에 담겨있는 웹사이트들에 접근해서 이상이 없다면 `true`, 그렇지 않으면 `false`를 받아온 결과를 `map`에 기록 후 반환한다.

```go
func mockWebsiteChecker(url string) bool {
	if url == "https://www.acmicpc.net/" {
		return false
	}
	return true
}

func TestCheckWebsites(t *testing.T) {
	websites := []string{
		"https://www.google.co.kr/",
		"https://www.naver.com/",
		"https://www.acmicpc.net/",
	}

	want := map[string]bool{
		"https://www.google.co.kr/": true,
		"https://www.naver.com/": true,
		"https://www.acmicpc.net/": false,
	}

	got := CheckWebsites(mockWebsiteChecker, websites)

	if !reflect.DeepEqual(want, got) {
		t.Fatalf("Wanted %v, got %v", want, got)
	}
}
```

- 실제로 HTTP로 웹사이트에 접근하지 않고 mocking 구현으로 테스트 코드를 짠다.
- `map`을 비교할 때는 `reflect.DeepEqual()`을 사용해야 한다.

```go
func slowStubWebsiteChecker(_ string) bool {
	time.Sleep(20 * time.Millisecond)
	return true
}

func BenchmarkCheckWebsites(b *testing.B) {
	urls := make([]string, 100)
	for i := 0; i < len(urls); i++ {
		urls[i] = "a url"
	}

	for i := 0; i < b.N; i++ {
		CheckWebsites(slowStubWebsiteChecker, urls)
	}
}
```

- 이 테스트 코드는 성능 측정을 위한 벤치마크 코드로,  `slowStubWebsiteChcker` 구현체를 사용할 경우 평균적으로 얼마나 시간이 걸리는지 측정한다.

```
goos: windows
goarch: amd64
pkg: go-with-tests/concurrency
cpu: Intel(R) Core(TM) i5-10210U CPU @ 1.60GHz
BenchmarkCheckWebsites
BenchmarkCheckWebsites-8   	       1	2678779000 ns/op
PASS
```

- 평균적으로 각 작업 당 대략 2.68 초가 소요되었다.

### 고루틴

이제 성능 개선을 위해 고루틴을 적용해서 비동기 처리를 해보자.

```go
func CheckWebsites(wc WebsiteChecker, urls []string) map[string]bool {
	results := make(map[string]bool)

	for _, url := range urls {
		go func() {
			results[url] = wc(url)
		}()
	}

	return results
}
```

- `go` 키워드를 사용하면 비동기 처리를 할 수 있다.

  `go`는 함수 호출 앞에서만 사용할 수 있으므로 익명 함수를 만들어서 `map`에 기록하도록 수정했다.

- 위 코드에서 익명 함수를 사용함으로써 `results`를 인자로 넘기지 않고서도 접근이 가능하고, 더불어 지금은 비어있는 상태이지만 `()`를 사용해서 일반 함수를 사용하듯이 인자를 전달할 수도 있다.

이제 다시 테스트 코드를 실행해보자.

```
Wanted map[https://www.acmicpc.net/:false https://www.google.co.kr/:true https://www.naver.com/:true], got map[https://www.acmicpc.net/:false]
```

- 테스트가 실패하면서 출력된 에러 메세지이다. 마지막 웹사이트에 대한 결과만 반환받았다.
- 이는 `for`문에서 `url`변수가 재사용되기 때문이다. 반복문이 돌면서 동일한 메모리 주소를 가진 `url` 변수에 값만 바뀌게 된다. 이때 익명 함수는 인자로 `url` 변수를 받아온 것이 아니므로 `url` 변수에 대한 래퍼런스를 하게 된다. 따라서 `urls` 배열에 담겨있는 마지막 값에 대해서만 동작하게 된 것이다.

이러한 상황을 피하려면 `url`을 익명 함수에 인자로 전달해주면 된다.

```go
func CheckWebsites(wc WebsiteChecker, urls []string) map[string]bool {
	results := make(map[string]bool)
	resultChannel := make(chan result)

	for _, url := range urls {
		go func(u string) {
			resultChannel <- result{u, wc(u)}
		}(url)
	}
    
    time.Sleep(2 * time.Second)

	return results
}
```

- 익명 함수에서 앞에 위치한 `()`에 인자로 받아올 변수명과 타입을 명시하고, 뒤에 위치한 `()`에 인자로 넘길 변수를 쓰면 된다.
- 추라고 `CheckWebsites()` 호출이 종료되어버리면 고루틴도 강제 종료되기 때문에 `time.Sleep()`으로 일단 종료를 늦춰서 결과를 확인하자.

더불어서 테스트 케이스를 실행하면 때때로 아래처럼 오류 메세지가 뜨기도 한다.

```go
fatal error: concurrent map writes

goroutine 8 [running]:
runtime.throw(0xd02029, 0x15)
	C:/Program Files/Go/src/runtime/panic.go:1117 +0x79 fp=0xc000047f18 sp=0xc000047ee8 pc=0xbdab39
runtime.mapassign_faststr(0xcda340, 0xc00006e5d0, 0xd02566, 0x16, 0x0)
	C:/Program Files/Go/src/runtime/map_faststr.go:211 +0x411 fp=0xc000047f80 sp=0xc000047f18 pc=0xbb47f1
go-with-tests/concurrency.CheckWebsites.func1(0xd09648, 0xc00006e5d0, 0xd02566, 0x16)
	C:/Users/tmdgh/algorithm-study/week00/go-with-tests/concurrency/check_websites.go:10 +0x79 fp=0xc000047fc0 sp=0xc000047f80 pc=0xcc1fb9
runtime.goexit()
	C:/Program Files/Go/src/runtime/asm_amd64.s:1371 +0x1 fp=0xc000047fc8 sp=0xc000047fc0 pc=0xc0faa1
created by go-with-tests/concurrency.CheckWebsites
	C:/Users/tmdgh/algorithm-study/week00/go-with-tests/concurrency/check_websites.go:9 +0xa5
```

- 이는 비동기 처리 중인 여러 고루틴에서 같은 메모리 주소에 쓰기 동작을 수행하려다 충돌이 나서 에러가 발생한 것이다. 일종의 Race Condition 이다.

이제 대부분의 경우 테스트가 통과한다. 하지만 코드 실행 중에 쓰레드를 잠시 중단시키는 점과 위와 같은 에러가 발생하고 있으므로 이를 고치기 위해 채널을 도입해야 한다.

### 채널

채널은 값들을 보내기도 하고 받기도 하는 Golang 자료 구조이다. 개인적으로는 시스템 프로그래밍에서 `C`언어로 구현할 때 사용한 `pipe`와 느낌이 유사했는데, 바로 코드를 통해 사용하는 방법을 살펴보자.

```go
type result struct {
	string
	bool
}

func CheckWebsites(wc WebsiteChecker, urls []string) map[string]bool {
	results := make(map[string]bool)
	resultChannel := make(chan result)

	for _, url := range urls {
		go func(u string) {
			resultChannel <- result{u, wc(u)}
		}(url)
	}

	for i := 0; i < len(urls); i++ {
		r := <- resultChannel
		results[r.string] = r.bool
	}

	return results
}
```

- `result`라는 커스텀 구조체 타입을 추가로 만들었다. 해당 구조체 내의 필드는 익명이다.
- 배열, 슬라이스, 맵을 초기화하듯이 채널도 `make(chan result)`로 생성할 수 있다.
- `채널 <- 값` 형태로 사용하면 `값`을 `채널`에 보내도록 하는 것이고, `값 <- 채널` 형태로 사용하면 `채널`에 있던 데이터를 `값`으로 받는 것이다.
- `값 <- 채널`처럼 채널을 통해 값을 받으면 고루틴이 호출되어 `채널 <- 값`으로 데이터를 보낸만큼, 데이터를 받도록 기다린다. 따라서 `time.Sleep()`가 필요없게 된다.

이제 다시 벤치마크 테스트 코드를 실행해서 성능을 비교해보자.

```
goos: windows
goarch: amd64
pkg: go-with-tests/concurrency
cpu: Intel(R) Core(TM) i5-10210U CPU @ 1.60GHz
BenchmarkCheckWebsites
BenchmarkCheckWebsites-8   	      36	  30588797 ns/op
PASS
```

- 평균적으로 각 동작 당 대략 0.03초가 소요되었다.
- 고루틴을 사용하기 전보다 훨씬 빨리진 것을 알 수 있다.


