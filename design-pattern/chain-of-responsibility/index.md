## Chain of Responsibility

### Intent

이 패턴은 클라이언트가 보낸 요청을 처리할 때, 핸들러들이 묶여있는 체인을 따라서 처리하는 방법이다. 각 핸들러는 요청에 따라 직접 처리할 지, 아니면 다음 핸들러로 넘길지 결정한다.

### Problem

사용자 인증 요청을 담당하는 서버가 있다고 하자. 초기에는 유저가 입력한 Credential이 올바른지 확인해서 결과를 반환하면 되지 않나라고 생각할 수 있다. 하지만 Credential이 올바르지 않은 경우 예외 처리, 반복적인 요청을 탐지해서 블록하는 동작, 리소스의 효율적인 활용을 위한 Cache 활용 동작 등등 사용자 인증 요청 처리 과정에서 해야하는 동작들이 잇따라 존재할 수 있다.

### Solution Structure

![Brief Structure[^1]](images/cor-solution1-en.png)

다른 Behavioral Pattern과 마찬가지로, Chain of Responsibility 패턴 또한 Handler라고 불리는 각 객체로 분리해 작업을 처리하도록 만든다. 각 Handler들을 연결하는 방식으로는 체인 형태를 띤다. 클라이언트가 보낸 요청은 결과를 반환받을 때까지 체인을 돌아다니며 처리된다. 이때 중요한 점은 각 Handler가 전달받은 요청을 직접 처리해서 결과를 반환할지, 다음으로 연결된 Handler에게 전달할지를 결정한다는 것이다.

![Solution Structure[^1]](images/cor-structure.png)

1. `Handler` 인터페이스는 각 구현체가 처리해야 하는 메서드를 정의한다. 더불어 다음 `Handler`를 참조할 수 있는 메서드도 있다.
2. `Base Handler`는 각 구현체가 공통적으로 처리하는 부분 (Boilerplate Code) 을 묶은 클래스다.
3. `Concrete Handler`들은 실제 요청을 처리한다. 전달받은 요청에 따라 직접 처리할지 다음 `Handler`에게 넘길지 결정한다. 주로 필요한 값들은 생성될 때 주입받고, 객체 생성 이후에는 Immutable하게 동작한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/content/post/design-pattern/project/hello-behavioral-pattern/CoR)

`patient`가 병원에 방문해 거쳐야 하는 일련의 과정 (`reception` - `docotor` - `medical` - `cashier`) 을 CoR 패턴을 적용하는 예제이다.

```go
type patient struct {
	name              string
	registrationDone  bool
	doctorCheckUpDone bool
	medicineDone      bool
	paymentDone       bool
}
```

```go
type department interface {
	execute(*patient)
	setNext(department)
}
```

```go
type reception struct {
	next department
}

func (r *reception) execute(p *patient) {
	if p.registrationDone {
		fmt.Println("Registration already done")
	} else {
		fmt.Println("Reception registering patient")
		p.registrationDone = true
	}
	r.next.execute(p)
}

func (r *reception) setNext(next department) {
	r.next = next
}
```

```go
type doctor struct {
	next department
}

func (d *doctor) execute(p *patient) {
	if p.doctorCheckUpDone {
		fmt.Println("Doctor checkup already done")
	} else {
		fmt.Println("Doctor checking patient")
		p.doctorCheckUpDone = true
	}
	d.next.execute(p)
}

func (d *doctor) setNext(next department) {
	d.next = next
}
```

```go
type medical struct {
	next department
}

func (m *medical) execute(p *patient) {
	if p.medicineDone {
		fmt.Println("Medicine already given to patient")
	} else {
		fmt.Println("Medical giving medicine to patient")
		p.medicineDone = true
	}
	m.next.execute(p)
}

func (m *medical) setNext(next department) {
	m.next = next
}
```

```go
type cashier struct {
	next department
}

func (c *cashier) execute(p *patient) {
	if p.paymentDone {
		fmt.Println("Payment Done")
	} else {
		fmt.Println("Cashier getting money from patient")
	}
}

func (c *cashier) setNext(next department) {
	c.next = next
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	cashier := &cashier{}

	medical := &medical{}
	medical.setNext(cashier)

	doctor := &doctor{}
	doctor.setNext(medical)

	reception := &reception{}
	reception.setNext(doctor)

	patient := &patient{name: "joon"}
	reception.execute(patient)
}
```

### Real Example

Spring Security의 Filter Chain을 살펴보려 한다. 아래는 대략적인 Filter들의 종류와 각 Filter들이 수행하는 동작이다.

![Kinds of Filter[^2]](images/cor-security-filters.png)

![Operation of Filters[^2]](images/cor-security-filter-invocation.png)

```java
public interface Filter {
       public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException;
}
```

```java
public abstract class AbstractAuthenticationProcessingFilter extends GenericFilterBean implements ApplicationEventPublisherAware, MessageSourceAware {
    @Override
	public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
		doFilter((HttpServletRequest) request, (HttpServletResponse) response, chain);
	}

	private void doFilter(HttpServletRequest request, HttpServletResponse response, FilterChain chain) throws IOException, ServletException {
		if (!requiresAuthentication(request, response)) {
			chain.doFilter(request, response);
			return;
		}
		try {
			Authentication authenticationResult = attemptAuthentication(request, response);
			if (authenticationResult == null) {
				// return immediately as subclass has indicated that it hasn't completed
				return;
			}
			this.sessionStrategy.onAuthentication(authenticationResult, request, response);
			// Authentication success
			if (this.continueChainBeforeSuccessfulAuthentication) {
				chain.doFilter(request, response);
			}
			successfulAuthentication(request, response, chain, authenticationResult);
		}
		catch (InternalAuthenticationServiceException failed) { /* ... */ }
		catch (AuthenticationException ex) { /* ... */ }
	}
}
```

```java
public class UsernamePasswordAuthenticationFilter extends AbstractAuthenticationProcessingFilter {
    @Override
	public Authentication attemptAuthentication(HttpServletRequest request, HttpServletResponse response) throws AuthenticationException {
		if (this.postOnly && !request.getMethod().equals("POST")) {
			throw new AuthenticationServiceException("Authentication method not supported: " + request.getMethod());
		}
		String username = obtainUsername(request);
		username = (username != null) ? username : "";
		username = username.trim();
		String password = obtainPassword(request);
		password = (password != null) ? password : "";
		UsernamePasswordAuthenticationToken authRequest = new UsernamePasswordAuthenticationToken(username, password);
		// Allow subclasses to set the "details" property
		setDetails(request, authRequest);
		return this.getAuthenticationManager().authenticate(authRequest);
	}
}
```

### Note

- 클라이언트가 보내는 요청이 다양한 방식으로 처리되어야 할 때 사용
- 특정 순서에 따라 요청이 처리되어야 할 때 사용

> Handler Chain에서 처리되지 않는 종류의 요청에 대한 예외 처리가 필요하다.

[^1]: [Chain of Responsibility Origin](https://refactoring.guru/design-patterns/chain-of-responsibility)
[^2]: [tmdgh0221 Velog Post](https://velog.io/@tmdgh0221/Spring-Security-%EC%99%80-OAuth-2.0-%EC%99%80-JWT-%EC%9D%98-%EC%BD%9C%EB%9D%BC%EB%B3%B4#spring-security)
