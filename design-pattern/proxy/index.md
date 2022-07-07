---
title: "[디자인 패턴 GURU] Proxy 패턴"
date: 2022-01-14
pin: false
tags:
- Software Engineering
- Design Pattern
- Go
- Java
---

## Proxy

### Intent

Proxy 패턴은 어느 한 객체를 대리해서 접근 제어를 한다. 실 객체에 요청이 들어와서 동작을 수행하기 전이나 후에 무언가 진행되어야 하는 것들을 설정할 수 있다.

### Problem

DB Connnection과 같이 많은 리소스를 요구하는 거대한 객체가 있다고 하자. 보통 DB에 접근할 일은 때때로 있어서 상시 접근할 필요는 없다. Lazy Initialization 방식을 적용해서 발생할 수 있는 문제를 일부 해결할 수 있지만, 객체마다 코드 중복이 발생하게 된다.

### Solution Structure

Proxy 패턴은 제어하고자 하는 개체와 동일한 인터페이스를 구현해야 하고, 클라이언트는 Proxy 객체를 통해 실 객체에 요청을 보낸다. 클라이언트로부터 요청을 발생할 때까지 Proxy 객체는 실 객체의 껍데기만을 갖고 있다가, 요청이 들어오면 실 객체를 생성한다.

![Structure[^1]](images/proxy-structure.png)

1. `ServiceInterface`는 실 객체인 `Service`의 인터페이스이다. `Proxy` 객체 또한 이 언테페이스를 따라야 한다.
2. `Proxy` 객체는 실 객체를 참조하는 필드를 가져야 한다. Proxy 과정을 마치면 실 객체의 메서드를 호출해 `Client`로부터의 요청을 전달한다. 주로 `Proxy` 객체가 실 객체의 생애주기를 관리한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/proxy)

실 서비스를 담당하는 `application` 객체가 있고, 이를 Proxy하는 `nginx` 객체가 있다. Proxy 객체는 다수 요청을 방지하기 위한 접근 제어를 선 처리 후 실 동작을 `sever`에게 위임한다.

```go
type server interface {
	handleRequest(string, string) (int, string)
}
```

```go
type application struct {
}

func newApplication() *application {
	return &application{}
}

func (a *application) handleRequest(url, method string) (int, string) {
	if url == "/app/status" && method == "GET" {
		return 200, "OK"
	}
	if url == "/create/user" && method == "POST" {
		return 201, "User Created"
	}
	return 404, "Not Ok"
}
```

```go
type nginx struct {
	application       *application
	maxAllowedRequest int
	rateLimiter       map[string]int
}

func newNginxServer() *nginx {
	return &nginx{
		application:       newApplication(),
		maxAllowedRequest: 2,
		rateLimiter:       make(map[string]int),
	}
}

func (n *nginx) handleRequest(url, method string) (int, string) {
	allowed := n.checkRateLimiting(url)
	if !allowed {
		return 403, "Not Allowed"
	}
	return n.application.handleRequest(url, method)
}

func (n *nginx) checkRateLimiting(url string) bool {
	if _, ok := n.rateLimiter[url]; ok {
		n.rateLimiter[url] = 1
	}
	if n.rateLimiter[url] > n.maxAllowedRequest {
		return false
	}
	n.rateLimiter[url] += 1
	return true
}
```

- 이외에도 로깅을 담당하거나 리소스 관리 등의 동작을 추가할 수 있다.

```go
func TestAfter(t *testing.T) {
	nginxServer := newNginxServer()
	appStatusURL := "/app/status"
	createUserURL := "/create/user"

	httpCode, body := nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(createUserURL, "POST")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 201, Body: User Created

	httpCode, body = nginxServer.handleRequest(createUserURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 404, Body: Not Ok
}
```

### Real Example

![Spring Security FilterChainProxy[^2]](images/proxy-filterchainproxy.png)

Spring Security의 `FilterChainProxy`를 살펴보려 한다. 이는 일종의 특수 필터로 `SecurityFilterChain`을 통해 클라이언트 요청을 다수의 Filter 객체에 위임한다.

```java
@Configuration
public class SecurityConfig extends WebSecurityConfigurerAdapter {
    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http.mvcMatcher("/foo/**");
    }
}
```

![FilterChainProxy Operation[^2]](images/proxy-security-filters-dispatch.png)

![Debug Example[^2]](images/proxy-securityFilterChain_foo.png)

### Note

- **Lazy Initialization (Virtual Proxy)**

  무거운 작업을 담당하는 객체가 리소스를 낭비하는 경우 사용

- **Access Control (Protection Proxy)**

  특정 클라이언트만이 객체를 접근하도록 제어하고자 하는 경우 사용

- **Local Execution of a Remote Service (Remote Proxy)**

- **Logging Requests (Logging Proxy)**

- **Cacching Request Results (Caching Proxy)**

[^1]: [Proxy Origin](https://refactoring.guru/design-patterns/proxy)
[^2]: [yaho1024 Velog Post](https://velog.io/@yaho1024/Spring-Security-FilterChainProxy)

