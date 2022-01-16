---
title: "[디자인 패턴 GURU] Builder 패턴"
summary: Refactoring Guru 서적을 기반으로한 디자인 패턴 학습 Builder 패턴
date: 2021-12-31
pin: false
image: images/builder-en.png
tags:
- Software Enginerring
- Design Pattern
- Go
- Java
---

## Builder

### Intent

Builder 패턴은 복잡한 객체를 체계적으로 생성할 수 있도록 돕는 방안이다.

### Problem

만약 아래와 같이 다양하면서 반복적인 객체들이 복잡하게 얽혀있다고 할 때, 생성자는 객체가 복잡한 만큼 거대해지게 된다.

![House 예제[^1]](images/builder-problem1.png)

가장 간단한 해결책은 `House` 클래스를 상속한 Sub-class들을 만들어서 각각의 객체들을 Sub-class에서 생성하도록 하는 것이다. 그러나 Sub-class 수가 많아질수록, 심지어는 하나의 필드가 추가되더라도, 점점 계층 구조가 깊어지게 된다.

다른 해결책은 Sub-class를 만들지 않고, 하나의 클래스에서 거대한 생성자로 모든 필드들을 처리하는 것이다. 하지만 이렇게 되면 필드값을 세팅하기 위한 생성자 파라미터에 의미없는 값들이 너무나도 많아지게 된다.

![House Giant Constructor[^1]](images/builder-problem2.png)

### Solution Structure

![Extract Builder Class[^1]](images/builder-solution1.png)

Builder 패턴은 객체 생성하는 코드를 별도의 클래스로 추출하여 진행하도록 한다. 해당 클래스는 `buildWalls`, `buildDoor` 등과 같은 생성 로직을 담고 있다. 여기서 중요한 점은 어떠한 객체를 생성하기 위해서 모든 로직들을 호출하지 않아도 된다는 점이다. 객체 생성에 필요한 필드값을 세팅하는 메서드들만 호출하여 만든다.

![Builder Structure[^1]](images/builder-structure.png)

1. `Builder` 인터페이스는 객체 생성에 공통적으로 필요한 필드를 세팅한다.

2. `Concrete Builder`들은 각 객체마다 다른 로직을 가진 필드를 세팅한다. (e.g., `buildStepA()`, `buildStepB()`)

   더불어 세팅한 결과를 조회할 수 있는 메서드를 구현해야 한다. (e.g., `getResult()`)

3. `Product`들은 생성된 객체다. 이 객체들은 동일한 클래스 계층 구조나 인터페이스에 속하지 않을 수 있다.

4. `Director`는 반복되는 객체 생성 과정을 별도의 메서드로 구현해, 재사용할 수 있도록 한다.

   이 클래스는 필수로 있어야 하는 것은 아니다.

### Applicability

- 아래와 같이 망원경처럼 점점 길이지는 생성자들을 제거 가능

  ```java
  class Pizza {
      Pizza(int size) { ... }
      Pizza(int size, boolean cheese) { ... }
      Pizza(int size, boolean cheese, boolean pepperoni) { ... }
      // ...
  }
  ```

- Composite Tree 패턴 (복잡한 객체의 성질) 을 가진 클래스를 생성

> 객체를 생성하기 위해 `Builder`, `Director` 클래스가 필요하는 등, 이전보다 코드 복잡성이 높아질 수 있다.

### Code Example

- Practice with Go

  [Github Repository](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-creational-pattern/builder)

- Spring Security

  ```java
  public interface WebSecurityConfigurer<T extends SecurityBuilder<Filter>> extends SecurityConfigurer<Filter, T> {
  }
  ```

  ```java
  public abstract class WebSecurityConfigurerAdapter implements WebSecurityConfigurer<WebSecurity> {
      protected WebSecurityConfigurerAdapter() {
  		this(false);
  	}
  
  	protected WebSecurityConfigurerAdapter(boolean disableDefaults) {
  		this.disableDefaults = disableDefaults;
  	}
      
      @Autowired
  	public void setApplicationContext(ApplicationContext context) {
  		/* ... */
  	}
      
      @Autowired(required = false)
  	public void setTrustResolver(AuthenticationTrustResolver trustResolver) {
  		this.trustResolver = trustResolver;
  	}
  
  	@Autowired(required = false)
  	public void setContentNegotationStrategy(ContentNegotiationStrategy contentNegotiationStrategy) {
  		this.contentNegotiationStrategy = contentNegotiationStrategy;
  	}
  
  	@Autowired
  	public void setObjectPostProcessor(ObjectPostProcessor<Object> objectPostProcessor) {
  		this.objectPostProcessor = objectPostProcessor;
  	}
  
  	@Autowired
  	public void setAuthenticationConfiguration(AuthenticationConfiguration authenticationConfiguration) {
  		this.authenticationConfiguration = authenticationConfiguration;
  	}
      // ...
  }
  ```

[^1]: [Builder Origin](https://refactoring.guru/design-patterns/builder)

