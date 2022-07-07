---
title: "Go언어에서의 에러 핸들링"
date: 2022-02-22
pin: false
tags:
- Go
- Code Style
---

# Go언어 에러 핸들링

이 글은 [Dave Cheney Blog](https://dave.cheney.net/2016/04/27/dont-just-check-errors-handle-them-gracefully) 글을 토대로 구글링 한 것들을 종합한 내용이다. 타 언어와 달리 Exception이 없는 구조의 Go언어에서 에러를 핸들링하는 방법을 알아보고자 한다.

## Error는 그냥 값일 뿐이다

본문은 Go언어에서 에러 핸들링하는 더 나은 전략이 무엇일까 하는 고민으로부터 출발한 글이다. 하지만 스탠다드한 방법 (일종의 정답) 은 존재하지 않는다는 결론을 내렸다. 그 대신 에러 처리가 대략 세 가지 정도의 카테고리로 분류될 수 있다고 생각한다.

---

## Sentinel Errors

첫번째 카테고리는 Sentinel (파수꾼) Errors 라고 불린다.

```go
if err == ErrSomething { ... }
```

Sentinel이라는 이름은, 컴퓨터 프로그래밍에서 더 이상의 처리가 불가능할 때 이를 나타내는 특정 값을 사용한다는 것에서 유래된 것이다. Go에서도 종종 에러를 나타내는 특정 값들을 사용한다. (e.g., `io.EOF`, `syscall.ENOENT`) 때로 실제로는 발생하지 않는 에러를 나타내는 Sentinel 에러도 있다. (e.g., `go/build.NoGoError`, `'path/filepath.Walk`')

이렇게 Sentinel Value를 사용하면, 호출하는 쪽에서 `==` 연산자를 이용해 미리 정의된 값과 비교를 해야하므로 유연하지 못하다. 세가지 에러 핸들링 카테고리 중 가장 유연하지 못한 방법이다. 초기 개발 이후 더 많은 로직을 제공하고자 할 때는 또 다른 에러를 리턴하게 될 수도 있는데, 이런 상황이 되면 기존의 `==`를 통한 처리가 불완전해질 수 있다.

### 절대 `error.Error()`로 Error Message를 검사하지 말자

`error.Error()` 메서드는 해당 에러의 에러 메시지를 반환해준다. 간혹 이 메서드를 통해 로직 처리를 하는 경우가 있는데, 절대 이 방식으로 검사하지 말아야 한다. Go언에서 Error 인터페이스 내의 `Error()` 메서드는 Code로의 처리가 아니라 개발자를 위해 존재하는 기능이다. 이 메서드에서 내뱉는 메시지는 로그를 통해, 또는 Stdout을 통해 보여질 것일 뿐, 이를 이용해 프로그램 행동이 변하는 식의 코드는 하지 말아야 한다. 테스트 코드 작성에서는 허용해도 괜찮다는 의견도 있으나, 그럼에도 불구하고 에러가 담고 있는 메시지를 비교하는 것은 악취나는 코드를 만드는 일이고, 되도록이면 피해야하는 일이라고 생각한다.

### Sentinel Error도 개발한 API의 한 부분이다

개발한 Public 함수/메서드가 특정 값의 에러를 리턴한다면, 그 값도 Public이어야 한다. 그리고 당연히 문서화되어야 한다. API의 노출 부분에 추가되는 것이다. 만약 API가 특정 에러를 리턴하는 인터페이스를 정의한다면, 그 인터페이스의 모든 구현들은 오로지 그 에러만을 리턴하도록 제약되는 것이다. 설령 그 구현체들이 더 상세한 에러를 제공할 수 있다고 하더라도 말이다.

이러한 예시를 `io.Reader`에서 찾아볼 수 있다. `io.Copy()`와 같은 함수들은 `io.EOF`를 리턴하는 `reader` 구현을 요구한다. 이는 호출하는 측에 "no more data, but that isn't an error" 상황을 아려주기 위함이다.

### Sentinel Error는 두 패키지간 의존성을 유발한다

Sentinel Error Value를 사용할 때 가장 큰 문제점은 호출하는 측과 홀출되는 측, 두 패키지 간의 코드 의존성이 생긴다는 것이다. 진행 중인 프로젝트에 수많은 패키지가 존재하고, 프로젝트 내 다른 패키지들에서 특정 에러 조건을 검사하기 위해 Import 해야하는 복잡한 커플링 상황은 분명 좋지 않다. 프로젝트가 규모가 크다면, 이런 패턴의 작업은 Import Loop의 공포를 제대로 느끼게끔 해줄 것이다.

### 결론, Sentinel Error 사용을 피해라

결론적으로 하고 싶은 말은, 코드에서 되도록 Sentinel Erorr 사용을 피하라는 것이다. 스탠다드 라이브러리에서 사용되고 있는 몇몇 케이스가 있다만, 흉내내야하는 패턴은 아니다.

---

## Error Types

Error Types 는 Go언어에서 에러 처리의 두 번째 유형이다.

```go
if err, ok := err.(SomeType); ok { ... }
```

Error Type 은 직접 정의한 타입 중, Error 인터페이스를 구현한 것이다. 아래의 예에서 `MyError` 타입은 무슨일 이 발생했는 지를 설명하는 메시지 뿐만 아니라, `File`과 `Line`을 트래킹한다.

```go
type MyError struct {
    Msg string
    File string
    Line int
}

func (e *MyError) Error() string {
    return fmt.Sprintf("%s:%d: %s", e.File, e.Line, e.Msg)
}
```

```go
return &MyError{"Something happened", "server.go", 42}
```

`MyError`는 타입이기 때문에 호출하는 쪽에서는 에러에 대한 추가 정보를 뽑아내기 위해 Type Assertion (`x.(T)` 형식의 표기법) 을 사용할 수 있다.

```go
err := something()
switch err := err.(type) {
case nil:
    // call succeede, nothing to do
case *MyError:
    fmt.Println("error occurred on line: ", err.Ling)
default:
    // unknown error
}
```

첫번째 유형에서 보았던 Error Value보다 Erro Type을 이용함으로써 개선된 점이라면, 더 많은 Context 정보를 제공할 수 있도록 원래 에러를 Wrapping할 수 있다는 점이다. 이에 대한 예는 `os.PathError` 타입에서 찾아볼 수 있다. 사용하려고 했던 파일에 대한 정보나, 수행하려고 했던 작업에 대한 정보를 기본 에러에 추가해주는 역할이다.

```go
// PathError records an error and the operation
// and file path that caused it.
type PathError struct {
    Op string
    Path string
    Err error // the cause
}

func (e *PathError) Error() string { ... }
```

### 발생할 수 있는 문제점들

위의 예시들을 통해 알 수 있듯이, 호출하는 쪽에서 Type Assertiong을 사용하거나 Type Switch 구문을 사용할 수 있기 때문에 Error Type은 Public으로 작성되어야 한다. 만약 작성한 코드가 특정 에러 타입을 요구하는 인터페이스를 구현한다면, 그 인터페이스의 모든 구현체들은 Error Type을 정의한 패키지에 의존성을 가지게 된다. 이는 호출자와의 강한 결합 관계를 만들고, 결국은 불안정한 API를 양산하게 된다.

### 결론, Error Type 사용을 피해라

무엇이 문제인 지에 대한 정보 등, 조금 더 많은 Context 정보를 포함할 수 있는 Error Type이 Sentinel Error Value보다는 그래도 낫지만, Error Type은 Error Value의 많은 문제점을 그대로 가진다. 그래서 말하고자 하는 바는, Error Type 사용을 피하라는 것이다. 또는 적어도 Error Type을 Public API의 일부로 포함시키지 말라는 것이다.

---

## Opaque Errors

이제 에러 처리의 세번째 카테고리다. 이 방법이 세 가지 카테고리 중 가장 유연한 전략이다. 왜냐면 개발한 코드와 호출자 사이의 결합도가 가장 적은 이유 때문이다. 이런 스타일을 Opaque (불투명) 에러 처리라고 부른다. 내용물에 대해서는 어떤 추측이나 가정도 없이 그냥 에러를 리턴하는 것이다. 이렇게 한다면 에러 핸들링은 디버깅할 때 굉장히 유용해진다.

```go
func fn() error {
    x, err := bar.Foo()
    if err != nil {
        return err
    }
    // ...
}
```

### Type이 아니라 Behavior에 대해 Error Assertion을 해야한다

어떤 경우에는 에러 처리에 대해 이분법적 접근만으로는 충분하지 않을 수 있다. 예를 들어, 네트워크 I/O 같은 외부 호출 경우에서는, 호출자가 작업의 재수행 여부를 판단하기 위해 에러를 검사해야할 수 있다. 이런 경우 에러가 특정 Type이거나 Value인 것을 보고 Assertion을 하는 것이 아니라, 에러가 특정 행위를 구현하고 있는 지를 보고 Assertion할 수 있다.

```go
type temporary interface {
    Temporary() bool
}

// IsTemporary returns true if err is temporary.
func IsTemporary(err error) bool {
    te, ok := err.(temporary)
    return ok && te.Temporary()
}
```

코드를 보면 에러가 Retry될 수 있는 지 판단하기 위해, `IsTemporary()` 함수에 어떤 에러든지 전달할 수 있다. 만약 에러가 `temporary` 인터페이스를 구현하고 있지 않다면, (즉 `Temporary()` 메서드를 가지고 있지 않다면) 이를 통해 Retry하면 안된다는 것을 알 수 있다. 반면에 만약 에러가 `Temporary()` 메서드를 구현하고 있다면 아마도 호출자는 `Temporary()`가 `true`를 리턴하는 경우 Retry할 수 있을 것이다.

여기에서 중요한 점은 이러한 로직이 에러를 정의하는 패키지를 Import할 필요없이 구현될 수 있다는 것이다. 에러의 기초가 되는 타입이 뭔지 몰라도 된다는 점이다. 단지 개발할 때는 그것의 행위에만 관심을 두면 되는 것이다.

---

## Error 검사만으로 그치지 말고, 우아하게 처리해라

Go언에에서 에러 핸들링할 때 강조하고 싶은 두번째 격언이다. "단지 에러를 검사만 하지 말고, 우아하게 처리하세요". 아래 코드에서 문제점을 찾아보자

```go
func AuthenticateRequest(r *Request) error {
    err := authenticate(r.User)
    if err != nil {
        return err
    }
    return nil
}
```

확실히 말할 수 잇는 점은 함수 구현의 5줄이 아래 1줄로 대체될 수 있다는 점이다. 하지만 이는 코드 리뷰에서 누구나 잡아낼 수 있는 간단한 것이다.

```go
return authenticate(r.User)
```

이보다 근본적인 문제점은 최초의 에러가 어디서부터 비롯된 것인지를 알 수 없다는 점이다. 만약 `authenticate()` 함수가 에러를 리턴한다면, `AuthenticateRequest()` 함수는 그 에러를 호출자에게 그대로 리턴할 것이다. 그리고 호출자도 아마 똑같이 그냥 리턴할 수도 있다. 프로그램의 꼭대기, `main()` 에서는 그 에르를 화면에 출력하거나 로그 파일에 기록할 것이다. 그 출력이나 기록 내용은 아마도 "No such file or directory" 가 전부일 수 있다.

최초 에러가 어디에서 생성된 것인지 File이나 Line에 대한 아무 정보가 없다. 에러 호출 Path를 알 수 있는 Call Stack의 Stack Trace 정보도 없다. 이러한 경우 코드 작성자는 에러 발생의 근원을 찾기 위해, 코드를 분해하는 기니긴 여정으로 내몰리게 될 것이다.

Donovan과 Kernighan의 책 "The Go Programming Language" 에서는 에러 Path 상에서 `fmt.Errorf()`를 이용해 Context 정보를 추가하라고 권장하고 있다.

```go
func AuthenticateRequest(r *Request) error {
    err := authenticate(r.User)
    if err != nil {
        return fmt.Errorf("authenticate failed: %v", err)
    }
    return nil
}
```

하지만 이렇게 처리하는 것은 앞에서 언급했던 얘기와 모순된다. `fmt.Errorf()`를 이용해 원래 Error Value를 변형하는 것은 Equality를 깨뜨리고, 원래의 에러 Context 정보를 파괴할 수 있기 때문이다.

### Error에 내용 추가하기

에러에 Context 정보를 추가하는 방법으로 적절한 기능을 제공하는 [Errors](https://pkg.go.dev/github.com/pkg/errors) 패키지에는 두 개의 주요 함수가 있다.

```go
// Wrap annotates cause with a message.
func Wrap(cause error, message string) error

// Cause unwraps an annotated error.
func Cause(err error) error
```

`Wrap()` 함수는 기존의 에러를 메시지로 감싸서 Wraping된 에러를 만들어내고, `Cause()` 함수는 그 반대다. 이 두 함수를 이용해서, 어떤 에러이든 Context 정보를 추가할 수 있고, 에러의 내부를 조사하거나 복원할 수도 있다.

```go
func ReadFile(path string) ([]byte, error) {
    f, err := os.Open(path)
    if err != nil {
        return nil, errors.Wrap(err, "open failed")
    }
    defer f.Close()
    
    buf, err := ioutil.ReadAll(f)
    if err != nil {
        return nil, errors.Wrap(err, "read failed")
    }
    return buf, nil
}

func ReadConfig() ([]byte, error) {
    home := os.Getenv("HOME")
    config, err := ReadFile(filepath.Join(home, ".settings.xml"))
    return config, errors.Wrap(err, "could not read config")
}
```

예제 코드의 `ReadConfig` 함수에서 에러가 발생하면 Wrapping한 덕분에 아래처럼 나이스한 추가 정보를 볼 수 있다.

```
could not read config: open failed: open /Users/tmp/.settings.xml:
no such file or directory
```

이처럼 에러를 Wrapping하면 에러 스택을 만들어내기 때문에 추가적인 디버깅 정보를 얻을 수 있다.

---

## Error를 딱 한번만 처리해라

여기서 에러를 처리한다 함은, 에러 값을 조사하고 어떻게 처리할 지 결정을 내린다는 의미이다. 발생한 에러에 대해 무언가를 결정할 일이 없다면 단순히 에러를 무시하기도 하며, 하나의 에러에 대해 두 번 이상의 로직을 처리되기도 한다.

```go
func Write(w io.Writer, buf []byte) {
    w. Write(buf) // ignore error
}
```

```go
func Write(w io.Write, buf []byte) error {
    _, err := w.Write(buf)
    if err != nil {
        log.Println("unable to write: ", err)
        return err
    }
    return nil
}
```

`Write()` 함수에서 에러가 발생하면 로그 기록을 남기고 호출자에게 리턴된다. 그리고 그 호출자 또한 로그를 남기고 리턴을 하며, 프로그램 최상단까지 올라갈 것이다. 즉, 로그 파일에 중복된 정보가 계속 쌓여가게 된다. 그러면서도 어떤 추가적인 Context 정보 없이 처음 발생한 에러에 대한 정보만을 받게 된다. 위에서 사용한 `Wrap()` 함수를 사용하여 Context 정보를 넣어주고, 로그 기록 등의 에러 처리는 한 번만 하도록 해야한다.

---

## 결론

결록적으로, 에러는 패키지의 Public API 중의 한 부분이다. 그러니까 에러도 다른 파트들 다루듯이 다루어야 한다.

최대한의 유연함을 위해 모든 에러를 Opaque하게 처리하려는 노력을 하는 게 바람직하다. 그렇게 처리할 수 없는 상황에서는 Type이나 Value말고 Behavior에 대해 Assertion 해야한다.

또한 Sentinel 에러 사용 횟수를 최소한으로 줄이고, 되도록 에러들을 감싸서 Opaque 에러로 바꾸어야 한다.

마지막으로 에러를 점검할 필요가 있을 경우에는 `Cause()` 함수를 이용해 원래의 에러를 복원하면 된다.

---

## 참고 자료

- [Dave Cheney Blog](https://dave.cheney.net/2016/04/27/dont-just-check-errors-handle-them-gracefully)
- [Rain.i Blog](http://cloudrain21.com/golang-graceful-error-handling#handle-errors-gracefully)

