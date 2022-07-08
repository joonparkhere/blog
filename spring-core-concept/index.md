---
title: 스프링 핵심 원리 기본편 정리
date: 2021-02-23
pin: false
tags:
- Spring
---

# 스프링 핵심원리 기본편

## 큰 흐름 잡기

### 스프링 부트란?

스프링을 편리하게 사용할 수 있도록 지원하는 프레임워크입니다. 이 툴을 사용하면서 제가 와닿는 강점은 아래와 같습니다. 그렇다면 **스프링**의 핵심 개념은 무엇일까요? 본질적으로 스프링은 자바 언어 기반의 프레임워크입니다. 따라서 객체 지향이라는 강력한 특징을 살려낼 수 있는, 즉 좋은 객체 지향 애플리케이션을 개발할 수 있게 도와주는 역할을 해줍니다.

### 객체 지향의 특징

이어서 객체 지향이 어떤 특징을 갖고 있는지 알아보면서, 이를 스프링에서는 어떻게 지원하는지 확인해보겠습니다.

객체 지향 프로그래밍의 의미는 컴퓨터 프로그램을 명령어의 목록으로 보는 시각에서 벗어나 여러 개의 독립된 단위인 **객체**들의 **모임**으로 파악하고자 하는 것입니다. 각각의 **객체**는 **메세지**를 주고받고, 데이터를 처리할 수 있습니다. (**협력**) 더불어 이는 프로그램을 마치 컴퓨터 부품을 갈아 끼우듯이, **유연**하고 **변경**이 용이하게 만들어주기 때문에 대규모 소프트웨어 개발에 많이 사용됩니다.

이를 위한 중요한 특징으로, 네 가지를 꼽을 수 있습니다. 추상화, 캡슐화, 상속, 다형성. 이 중에서 가장 중요한 **다형성**에 대해 더 알아보도록 하겠습니다.

다형성을 설명하기 위해서, **역할**과 **구현**으로 세상을 구분하여 예시를 들어보겠습니다.

- 자동차라는 역할이 있다면, 이것의 구현체로 K3, 아반떼, 테슬라 모델3 등이 있습니다.
- 로미오와 줄리엣의 역할이 있다면, 구현체로 장동건과 원빈, 그리고 김태희와 송혜교 등이 있습니다.

이렇게 **역할**과 **구현**으로 구분하면 세상이 **단순**해지고 **유연**해지며 **변경**도 편리해집니다.

- 클라이언트는 대상의 역할(인터페이스)만 알면 됩니다.
- 클라이언트는 구현 대상의 내부 구조를 몰라도 됩니다.
- 클라이언트는 구현 대상의 내부 구조가 변경되어도 영향을 받지 않습니다.
- 클라이언트는 구현 대상 자체를 변경해도 영향을 받지 않습니다.

자바 언어에 대입을 하면 역할은 인터페이스, 구현은 클래스라고 할 수 있습니다. 즉, 객체 설계시 역할(인터페이스)를 먼저 부여하고, 그 역할을 수행하는 구현체(클래스)를 만들어야합니다.

그 과정에서 중요한 자바의 기본 문법이 바로 **오버라이딩**입니다. 실제 동작하는 로직은 구현체에 맞게 오버라이딩된 메서드이며, 클래스는 인터페이스를 구현한 것이므로 유연하게 필요에 따라 객체를 변경할 수 있습니다.

<img src="./images/polymorphism.png" style="zoom:80%;" />

```java
public class MemberService {
//	private MemberRepository memberRepository = new MemoryMemberRepository();
    private MemberRepository memberRepository = new JdbcMemberRepository();
}
```

즉 다형성의 본질은, 인터페이스를 구현한 객체 인스턴스를 **실행 시점**에 **유연**하게 **변경**할 수 있다는 점입니다. 이를 이해하려면 **협력**이라는 객체사이의 관계에서 시작해야합니다. 한 문장으로 정리하면, **클라이언트를 변경하지 않고, 서버의 구현 기능을 유연하게 변경 가능**하다는 점입니다.

### 좋은 객체 지향 설계의 원칙: SOLID

**SOLID**는 클린코드로 유명한 로버트 마틴이 정리한 5가지 원칙입니다.

- SRP (Single Responsibility Principle): 단일 책임 원칙

  한 클래스는 하나의 책임만을 가져야 합니다. 이때 하나의 책임을 구분하는 기준은 **변경**입니다. 변경이 있을 때 파급 효과가 적으면 단일 책임 원칙을 잘 따른 것으로 볼 수 있습니다.

- OCP (Open / Closed Principle): 개방 / 폐쇄 원칙

  소프트웨어 요소는 확장에는 열려있으나 변경에는 닫혀있어야 합니다. 확장을 하려면 당연히 기존 코드를 변경해야 한다고 생각할 수 있지만, **다형성**을 활용한다면, **역할과 구현의 분리**를 생각해보면 가능합니다.

  그러나 순수 자바 언어로는 위의 자바 예시처럼, 다형성을 사용했지만 OCP 원칙을 지킬 수는 없습니다. 이를 해결하기 위해서, 객체를 생성하고 연관관계를 맺어주는 별도의 조립 및 설정자가 필요합니다. 이 부분을 스프링에서 도맡아 줍니다.

- LSP (Liskov Substitution Principle): 리스코프 치환 원칙

  프로그램의 객체는 프로그램의 정확성을 깨뜨리지 않으면서 하위 타입의 인스턴스로 바꿀 수 있어야 합니다. 이는 인터페이스를 구현한 구현체를 믿고 사용하기 위한 원칙으로, 하위 클래스는 인터페이스 규약을 다 지켜야 함을 뜻합니다. 단순히 컴파일에 성공하는 것을 넘어서, 자동차 인터페이스의 엑셀은 앞으로 가라는 기능인데 이를 뒤로 가게 구현하면 LSP에 위반되는 것처럼, 인터페이스 규약을 따라야 합니다.

- ISP (Interface Segregation Principle): 인터페이스 분리 원칙

  클라이언트를 위한 인터페이스 여러 개가 하나의 범용 인터페이스보다 제 역할을 다합니다. 자동차 인터페이스는 운전 인터페이스와 정비 인터페이스로 분리하는 것과 같이, 인터페이스가 명확해지고 대체 가능성이 높아집니다. 동시에 하나의 인터페이스가 변하더라도 운전자 클라이언트에 영향을 주지 않게 됩니다.

- DIP (Dependnecy Inversion Principle): 의존관계 역전 원칙

  프로그래머는 구체화에 의존하지 않고, 추상화에 의존해야 합니다. 즉, **역할**에 의존해야 하는 것입니다.

  그러나 위의 자바 예시처럼 순수 자바 언어로는, 클라이언트가 구현 클래스를 직접 선택하여 인터페이스와 구현 클래스를 동시에 의존합니다. 이 문제를 스프링에서 해결해줍니다.

### 객체 지향 설계와 스프링

 자바 언어로는 OCP와 DIP를 지키기 힘듭니다. 이를 위해 스프링이 등장했고, 이외에도 객체 지향을 위해 추가적인 강점을 제공합니다.

- DI (Dependency Injection) 개념과 DI 컨테이너를 제공하여 다형성와 OCP, DIP를 가능하게 지원합니다.
- 클라이언트 코드의 변경없이 기능을 확장하도록 도와줍니다.



## 예제를 통한 스프링 핵심 원리 이해

### 주문 도메인 설계

구현하고자 하는 모델은 아래의 다이어그램입니다.

- 주문 도메인 협력, 역할, 책임

  ![](./images/order-domain-info.png)

- 주문 도메인 전체

  ![](./images/order-domain.png)

- 주문 클래스 다이어그램

  ![](./images/order-class-diagram.png)

- 주문 객체 다이어그램

  ![](./images/order-object-diagram.png)

### 주문 도메인 개발

```java
public class OrderServiceImpl implements OrderService {
//  private final DiscountPolicy discountPolicy = new FixDiscountPolicy();
    private final DiscountPolicy discountPolicy = new RateDiscountPolicy();
}
```

위처럼 자바 코드를 작성하면, OCP와 DIP 원칙에 어긋납니다. 기능을 확장해려면 클라이언트 코드에 영향을 주기 때문에 OCP를 위반하며, 주문 서비스 클라이언트(`OrderServiceImpl`)은 `DiscountPolicy` 인터페이스를 의존하는 것과 동시에 구현 클래스인 `FixDiscountPolicy` 혹은 `RateDiscountPolicy`에도 의존하고 있기 때문에 DIP를 위반합니다.

구현하고자 했던 모델이 아래와 같다면,

![](./images/order-expect-dependecy.png)

현재 구현한 모델의 실제 모습은 아래와 같습니다.

![](./images/order-wrong-dependecy.png)

즉, 위 코드를 인터페이스에만 의존하도록 설계를 변경해야 합니다.

```java
public class OrderServiceImpl implements OrderService {
    private final DiscountPolicy discountPolicy;
}
```

그러나 이 경우에는 구현체가 없기 때문에 `null pointer exception`이 발생하며 제대로 동작하지 않습니다. 따라서 누군가가 클라이언트인 `OrdeServiceImpl`에 `DiscountPolicy`의 구현 객체를 대신 생성하고 주입해주어야 합니다.

### AppConfig의 등장

애플리케이션의 전체 동작 방식을 구성하기 위해 **구현 객체를 생성**하고, **연결**하는 책임을 가지는 별도의 설정 클래스입니다. 이를 클라이언트인 `OrderServiceImpl`에 구현하지 않은 이유는, SRP 원칙을 지키기 위해서입니다. `OrderServiceImpl`은 `DiscountPolicy`의 구현 객체를 가지고 주문 로직을 수행하는 역할을 수행해야 합니다. 여기에 추가로 각 인터페이스에 어떤 구현 객체가 들어와야 하는지 정하는, 즉 또 다른 역할(책임)을 추가한다면 클라이언트는 점점 복잡해집니다. 따라서 각각의 책임을 확실히 분리하기 위해 `AppConfig`를 별도로 만드는 것입니다.

```java
public class AppConfig {
    public OrderService orderService() {
        return new OrderServiceImpl(memberRepository(), discountPolicy());
    }
    
    public MemberRepository memberRepository() {
        return new MemoryMemberRepository();
    }
    
    public DiscountPolicy discountPolicy() {
        return new RateDiscountPolicy();
    }
}
```

```java
public class OrderServiceImpl implements OrderService {
    private final MemberRepository memberRepository;
    private final DiscountPolicy discountPolicy;
    
    public OrderServiceImpl(MemberRepository memberRepository, DiscountPolicy discountPolicy) {
        this.memberRepository = memberRepository;
        ths.discountPolicy = discountPolicy;
    }
}
```

```java
public class OrderApp {

    public static void main(String[] args) {
        AppConfig appConfig = new AppConfig();
        OrderService orderService = appConfig.orderService();

        Long memberId = 1L;
        Member member = new Member(memberId, "memberA", Grade.VIP
        Order order = orderService.createOrder(memberId, "itemA", 20000);

        System.out.println("order = " + order);
    }
}
```

- 더이상 `OrderServiceImpl`은 구현 클래스를 의존하지 않습니다.
- `OrderServiceImpl` 입장에서 생성자를 통해 어떤 구현 객체가 주입될지 알 수 없습니다. 이는 오직 외부(`AppConfig`)에서 결정합니다.

![](./images/order-correc-dependecy.png)

별도의 설정 클래스 `AppConfig`를 사용함으로써, OCP와 DIP 원칙을 지키며 기존에 하고자 했던 설계를 했습니다. 비즈니스 로직상 `DiscountPolicy` 인터페이스의 구현 객체로 다른 클래스가 추가되어도, 구성 영역인 `AppConfig`에서 수정하면 사용 영역의 어떠한 코드 변경없이 확장할 수 있습니다.

### 의존 관계 주입

#### IoC (Inversion of Control) 제어의 역전

- 기존 프로그램은 클라이언트 구현 객체가 스스로 필요한 서버 구현 객체를 생성하고, 연결하고, 실행했습니다. 한마디로 구현 객체가 프로그램의 제어 흐름을 스스로 조종했습니다.
- 반면에 `AppConfig`가 등장한 이후에 구현 객체는 자신의 로직을 실행하는 역할만 담당합니다. 프로그램의 제어 흐름은 이제 `AppConfig`가 가져갑니다.
- 이렇듯 프로그램의 제어 흐름을 직접 제어하는 것이 아니라 외부에서 관리하는 것을 제어의 역전(IoC)이라고 합니다.

#### DI (Dependency Injection) 의존 관계 주입

`OrderServiceImpl`은 `DiscountPolicy` 인터페이스에만 의존합니다. 실제 어떤 구현 객체가 사용될지는 모릅니다. 이러한 의존관계는 **정적인 클래스 의존 관계**와 실행 시점에 결정되는 **동적인 객체(인스턴스) 의존 관계**로 분리해서 생각해야 합니다.

- 정적인 클래스 의존 관계

  클래스가 사용하는 import 코드만 보고 의존 관계를 쉽게 판단할 수 있습니다. 정적인 의존 관계는 애플리케이션을 실행하지 않아도 분석할 수 있습니다. `OrderServiceImpl`은 `MemberRepository`와 `DiscountPolicy`에 의존함을 알 수 있는 것처럼 말입니다.

  ![](./images/order-class-diagram.png)

- 동적인 객체(인스턴스) 의존 관계

  애플리케이션 실행 시점에 실제 생성된 객체 인스턴스의 참조가 연결된 의존 관계입니다.

  ![](./images/order-object-diagram.png)

DI 개념을 정리하면 아래와 같습니다.

- 애플리케이션 **실행 시점(런타임)**에 외부에서 실제 구현 객체를 생성하고 클라이언트에 전달해서 클라이언트와 서버의 실제 의존 관계가 연결되는 것을 **의존 관계 주입**이라 합니다.
- 의존 관계 주입을 사용하면 클라이언트 코드를 변경하지 않고, 클라이언트가 호출하는 대상의 타입 인스턴스를 변경할 수 있습니다.
- 의존 관계 주입을 사용하면 클래스 의존 관계를 변경하지 않고, 동적인 객체 인스턴스 의존 관계를 쉽게 변경할 수 있습니다.

#### DI 컨테이너

`AppConfig`처럼 객체를 생성하고 관리하면서 의존 관계를 연결해주는 것을 IoC 컨테이너 혹은 DI 컨테이너라고 합니다.

### 스프링으로 전환

```java
@Configuration
public class AppConfig {
    @Bean
    public OrderService orderService() {
        return new OrderServiceImpl(memberRepository(), discountPolicy());
    }
    
    @Bean
    public MemberRepository memberRepository() {
        return new MemoryMemberRepository();
    }
    
    @Bean
    public DiscountPolicy discountPolicy() {
        return new RateDiscountPolicy();
    }
}
```

```java
public class OrderApp {

    public static void main(String[] args) {
//        AppConfig appConfig = new AppConfig();
//        OrderService orderService = appConfig.orderService();

        ApplicationContext applicationContext = new AnnotationConfigApplicationContext(AppConfig.class);
        
        OrderService orderService = applicationContext.getBean("orderService", OrderService.class);

        Long memberId = 1L;
        Order order = orderService.createOrder(memberId, "itemA", 20000);

        System.out.println("order = " + order);
    }
}
```

- `ApplicationContext`를 스프링 컨테이너라고 합니다.
- 기존에는 개발자 `AppConfig`를 사용해서 직접 객체를 생성하고 DI를 했지만, 이제부터는 스프링 컨테이너를 통해서 사용합니다.
- 스프링 컨테이너는 `@Configuration`이 붙은 `AppConfig`를 설정 정보로 사용합니다. 여기서 `@Bean`이라 적힌 메서드를 모두 호출해서 반환된 객체를 스프링 컨테이너에 등록합니다. 이렇게 스프링 컨테이너에 등록된 객체를 스프링빈이라고 합니다.
- 이전에는 개발자가 필요한 객체를 `AppConfig`를 사용해서 직접 조회했지만, 이제부터는 스프링 컨테이너를 통해서 필요한 스프링 빈(객체)를 찾아야 합니다.
- 기존에는 개발자가 직접 자바 코드로 모든 것을 했다면 이제부터는 스프링 컨테이너에 객체를 스프링 빈으로 등록하고, 스프링 컨테이너에서 스프링 빈을 찾아서 사용하도록 변경되었습니다.

## 스프링 컨테이너와 스프링 빈

### 스프링 컨테이너 생성 과정

1. 스프링 컨테이너 생성

   ```java
   ApplicationContext applicationContext = new AnnotationConfigApplicationContext(AppConfig.class);
   ```

   - `ApplicationContext`를 스프링 컨테이너라 합니다.
   - XML 기반 혹은 애노테이션 기반의 자바 설정 클래스, 두 방법으로 만들 수 있습니다.
   - `AnnotationConfigApplicationContext`는 인터페이스 `ApplicationContext`의 구현체입니다.

2. 스프링 빈 등록
   - 스프링 컨테이너는 파라미터로 넘어온 설정 클래스 정보(`AppConfig`)를 사용해서 스프링 빈을 등록합니다.
   - 빈 이름의 디폴트 값은 메서드 이름입니다. (`@Bean(name="orderService")` 으로 직접 설정 가능)
3. 스프링 빈 의존 관계 설정
   - 설정 정보를 참고해서 의존 관계를 주입(DI)합니다.
   - 싱글톤 컨테이너로, 단순히 자바 코드를 호출하는 것과 차이가 있습니다.

### 스프링 빈 조회

- 기본

  ```java
  AnnotationConfigApplicationContext ac = new AnnotationConfigApplicationContext(AppConfig.class);
  
  @Test
  @DisplayName("빈 이름으로 조회")
  void findBeanByName() {
      OrderService orderService = ac.getBean("orderService", OrderService.class);
      Assertions.assertThat(orderService).isInstanceOf(OrderServiceImpl.class);
  }
  
  @Test
  @DisplayName("이름 없이 타입으로 조회")
  void findBeanByType() {
      OrderService orderService = ac.getBean(OrderService.class);
      Assertions.assertThat(orderService).isInstanceOf(OrderServiceImpl.class);
  }
  
  @Test
  @DisplayName("구체 타입으로 조회")
  void findBeanByName2() {
      OrderService orderService = ac.getBean("orderService", OrderServiceImpl.class);
      Assertions.assertThat(orderService).isInstanceOf(OrderServiceImpl.class);
  }
  
  @Test
  @DisplayName("빈 이름으로 조회X")
  void findBeanByNameX() {
  //    Object xxxxxx = ac.getBean("XXXXXX");
      org.junit.jupiter.api.Assertions.assertThrows(NoSuchBeanDefinitionException.class,
              () -> ac.getBean("XXXXXX"));
  }
  ```

  스프링 컨테이너에서 스프링 빈을 찾는 가장 기본적인 조회 방법

  1. `ac.getBean(빈이름, 타입)`
  2. `ac.getBean(타입)`

- 동일한 타입이 둘 이상인 경우

  ```java
  AnnotationConfigApplicationContext ac = new AnnotationConfigApplicationContext(SameBeanConfig.class);
  
  @Test
  @DisplayName("타입으로 조회시 같은 타입이 둘 이상 있으면, 중복 오류가 발생한다")
  void findBeanByTypeDuplicate() {
      Assertions.assertThrows(NoUniqueBeanDefinitionException.class,
              () -> ac.getBean(MemberRepository.class));
  }
  
  @Test
  @DisplayName("타입으로 조회시 같은 타입이 둘 이상 있으면, 빈 이름을 지정하면 된다")
  void findBeanByName() {
      MemberRepository memberRepository1 = ac.getBean("memberRepository1", MemberRepository.class);
      org.assertj.core.api.Assertions.assertThat(memberRepository1).isInstanceOf(MemberRepository.class);
  }
  
  @Test
  @DisplayName("특정 타입을 모두 조회하기")
  void findAllBeanByType() {
      Map<String, MemberRepository> beansOfType = ac.getBeansOfType(MemberRepository.class);
      for (String key : beansOfType.keySet()) {
          System.out.println("key = " + key + " value = " + beansOfType.get(key));
      }
      org.assertj.core.api.Assertions.assertThat(beansOfType.size()).isEqualTo(2);
  }
  
  
  @Configuration
  static class SameBeanConfig {
      @Bean
      public MemberRepository memberRepository1() {
          return new MemoryMemberRepository();
      }
      @Bean
      public MemberRepository memberRepository2() {
          return new MemoryMemberRepository();
      }
  }
  ```

  타입으로 조회시 같은 타입의 스프링 빈이 둘 이상이면 오류가 발생합니다. 이때는 빈 이름을 지정해야 합니다.

- 상속 관계

  ```java
  AnnotationConfigApplicationContext ac = new AnnotationConfigApplicationContext(TestConfig.class);
  
  @Test
  @DisplayName("부모 타입으로 조회시, 자식이 둘 이상 있으면, 중복 오류가 발생한다")
  void findBeanByParentTypeDuplicate() {
      assertThrows(NoUniqueBeanDefinitionException.class,
              () -> ac.getBean(DiscountPolicy.class));
  }
  
  @Test
  @DisplayName("부모 타입으로 조회시, 자식이 둘 이상 있으면, 빈 이름을 지정하면 된다")
  void findBeanByParentTypeBeanName() {
      DiscountPolicy rateDiscountPolicy = ac.getBean("rateDiscountPolicy", DiscountPolicy.class);
      assertThat(rateDiscountPolicy).isInstanceOf(RateDiscountPolicy.class);
  }
  
  @Test
  @DisplayName("부모 타입으로 모두 조회하기")
  void findAllBeanByParentType() {
      Map<String, DiscountPolicy> beansOfType = ac.getBeansOfType(DiscountPolicy.class);
      assertThat(beansOfType.size()).isEqualTo(2);
  }
  
  @Configuration
  static class TestConfig {
      @Bean
      public DiscountPolicy rateDiscountPolicy() {
          return new RateDiscountPolicy();
      }
      @Bean
      public DiscountPolicy fixDiscountPolicy() {
          return new FixDiscountPolicy();
      }
  }
  ```

  부모 타입으로 빈을 조회하면, 자식 타입들도 함께 조회됩니다. (`Object` 타입으로 조회하면, 모든 스프링 빈을 조회하게 됩니다.)

### BeanFactory와 ApplicationContext

![](./images/spring-container-structure.png)

#### BeanFactory

- 스프링 컨테이너의 최상위 인터페이스
- 스프링 빈을 관리하고 조회하는 역할을 담당
- `getBean()`을 제공
- 위 테스트 코드에서 사용한 대부분 기능을 BeanFactory가 제공

#### ApplicationContext

- BeanFactory 기능을 모두 상속받아서 제공

- 빈 관리 및 조회 기능 뿐만이 아닌, 여러 부가 기능을 제공

  ![](./images/application-context-function.png)

  - **메세지 소스를 활용한 국제화 기능** (한국에서 들어오면 한국어로, 영어권에서 들어오면 영어로 출력)
  - **환경 변수**: 로컬 / 개발 / 운영 등을 구분해서 처리
  - **애플리케이션 이벤트**: 이벤트를 발행하고 구독하는 모델을 편리하게 지원
  - **편리한 리소스 조회**: 파일, 클래스 패스, 외부 등에서 리소스를 편리하게 조회

### 다양한 설정 형식 지원

스프링 컨테이너는 다양한 형식의 설정 정보를 받아드릴 수 있게 유연하게 설계가 되어있습니다. (자바 코드, XML, Groovy 등등)

![](./images/spring-container-type.png)

## 싱글톤 컨테이너

### 웹 애플리케이션과 싱글톤

```java
@Test
@DisplayName("스프링 없는 순수한 DI 컨테이너")
void pureContainer() {
    AppConfig appConfig = new AppConfig();
    MemberService memberService1 = appConfig.memberService();
    MemberService memberService2 = appConfig.memberService();

    System.out.println("memberService1 = " + memberService1);
    System.out.println("memberService2 = " + memberService2);

    Assertions.assertThat(memberService1).isNotSameAs(memberService2);
}
```

이전에 만든 스프링 없는 순수한 DI 컨테이너인 `AppConfig`는 요청을 할 때마다 객체를 새로 생성합니다. 문제는 보통의 웹 애플리케이션은 여러 고객이 동시에 요청을 합니다.  고객 트래픽 초당 100이 나오면, 초당 100개 객체가 생성되고 소멸되는 꼴입니다. 이 문제를 해결하기 위해 해당 객체가 딱 1개만 생성되고 공유하도록 설계한 **싱글톤 패턴**을 이용합니다.

### 싱글톤 패턴

```java
public class SingletonService {
    private static final SingletonService instance = new SingletonService();

    public static SingletonService getInstance() {
        return instance;
    }

    private SingletonService() {
    }
}
```

1. `static` 영역에 객체를 미리 하나 생성해서 올려둡니다.
2. 이 객체 인스턴스가 필요하면 오직 `getInstance` 메서드를 통해서만 조회할 수 있습니다. (항상 같은 객체를 반환)
3. 오직 1개의 객체 인스턴스만 존재해야 하므로, 생성자를 `private`로 막아서 외부에서 생성되는 것을 막도록 합니다.

하지만 싱글톤 패턴에는 여러 문제점이 있습니다.

1. 싱글톤 패턴을 구현하는 코드 자체가 많이 들어갑니다.
2. 의존 관계상 클라이언트가 구체 클래스에 의존합니다. 결국 DIP를 위반합니다.
3. 클라이언트가 구체 클래스에 의존해서 OCP 원칡을 위반할 가능성이 높습니다.
4. 테스크 코드 작성이 어렵습니다.
5. 내부 속성을 변경하거나 초기화하기 어렵습니다.
6. `private` 생성자로 자식 클래스를 만들기 어렵습니다.

결론적으로, 유연성이 떨어져서 싱글톤 패턴은 안티패턴으로 불리기도 합니다.

### 싱글톤 컨테이너

스프링 컨테이너는 싱글톤 패턴의 문제점을 해결하며 객체 인스턴스를 싱글톤으로 관리합니다. (싱글톤 컨테이너 역할을 하는 기능을 싱글톤 레지스트리라 합니다.)

- 싱글톤 패턴을 적용하지 않아도, 객체 인스턴스를 싱글톤으로 관리합니다.
- DIP, OCP, 테스트 코드 작성, `private` 생성자로 부터 자유롭게 싱글톤을 사용할 수 있습니다.

### 싱글톤 방식의 주의점

이 방식을 사용하면 여러 클라이언트가 하나의 같은 객체 인스턴스를 공유하기 때문에 싱글톤 객체는 상태를 유지(stateful)하게 설계하면 안되고, **무상태(stateless)**로 설계해야 합니다.

- 특정 클라이언트에 의존적인 필드가 있으면 안됩니다.
- 특정 클라이언트가 값을 변경할 수 있는 필드가 있으면 안됩니다.
- 가급적 읽기만 가능해야 합니다.
- 필드 대신 자바에서 공유되지 않는 지역변수, 파라미터, ThreadLocal 등을 사용해야 합니다.

### `@Configuration`과 싱글톤

```java
@Configuration
public class AppConfig {
    @Bean
    public MemberService memberService() {
        return new MemberServiceImpl(memberRepository());
    }

    @Bean
    public MemberRepository memberRepository() {
        return new MemoryMemberRepository();
    }

    @Bean
    public OrderService orderService() {
        return new OrderServiceImpl(memberRepository(), discountPolicy());
    }

    @Bean
    public DiscountPolicy discountPolicy() {
        return new RateDiscountPolicy();
    }
}
```

`memberService`와 `orderService` 빈을 만드는 코드를 보면 각각 `memberRepository()`를 호출해서 `new MemoryMemberRepository()`가 호출됩니다. 결과적으로 서로 다른 2개의 객체 인스턴스가 생성되면서 싱글톤이 깨지는 것처럼 보이지만, 스프링 컨테이너는 이를 하나의 객체로 유지시킵니다.

```java
@Test
void configurationTest() {
    AnnotationConfigApplicationContext ac = new AnnotationConfigApplicationContext(AppConfig.class);

    MemberServiceImpl memberService = ac.getBean("memberService", MemberServiceImpl.class);
    OrderServiceImpl orderService = ac.getBean("orderService", OrderServiceImpl.class);
    MemberRepository memberRepository = ac.getBean("memberRepository", MemberRepository.class);

    MemberRepository memberRepository1 = memberService.getMemberRepository();
    MemberRepository memberRepository2 = orderService.getMemberRepository();

    System.out.println("memberRepository1 = " + memberRepository1);
    System.out.println("memberRepository2 = " + memberRepository2);
    System.out.println("memberRepository = " + memberRepository);

    Assertions.assertThat(memberService.getMemberRepository()).isSameAs(memberRepository);
    Assertions.assertThat(orderService.getMemberRepository()).isSameAs(memberRepository);
}
```

직접 테스트 코드를 작성해서 확인해봐도, `memberRepository1`과 `memberRepository2`는 같은 객체로 조횝니다.

### 바이트코드 조작

스프링 컨테이너는 싱글톤 레지스트리로, 스프링 빈이 싱글톤이 되도록 보장해야 합니다. 그러나 자바 코드까지 조작하기는 어려우므로, 클래스의 바이트 코드를 조작하는 리이브러리를 사용합니다.

```java
@Test
void configurationDeep() {
    AnnotationConfigApplicationContext ac = new AnnotationConfigApplicationContext(AppConfig.class);
    AppConfig bean = ac.getBean(AppConfig.class);

    System.out.println("bean = " + bean);
	//출력: bean = class hello.core.AppConfig$$EnhancerBySpringCGLIB$$bd479d70
}
```

순수한 클래스라면 `class hello.core.AppConfig`가 출력되어야 하겠지만, `@Configuration`을 적용한 `AppConfig`는 CGLIB라는 바이트 코드 조작 라이브러리를 사용해서 `AppConfig` 클래스를 상속받은 임의의 다른 클래스를 만들고, 그 다른 클래스를 스프링 빈으로 등록합니다.

![](./images/bytecode-modify.png)

해당 임의의 다른 클래스가 싱글톤이 보장되도록 해줍니다.

- `@Bean`이 붙은 메서드마다 이미 스프링 빈이 존재하면 존재하는 빈을 반환하고, 스프링 빈이 없으면 생성해서 빈으로 등록하고 반환하는 코드가 동적으로 만들어집니다.

## 컴포넌트 스캔

### 기본 개념

이전까지 스프링 빈을 등록할 때는 자바 코드에 `@Bean`을 통해서 설정 정보에 직접 등록할 빈을 나열했습니다. 하지만 이렇게 등록해야하는 빈의 수가 커지면 단순 반복, 설정 정보의 증가, 누락 등의 문제가 발생할 수 있습니다. 그래서 스프링은 **설정 정보가 없어도 자동으로 스프링 빈을 등록**하는 컴포넌트 스캔 기능을 제공합니다. 더불어서, **의존 관계를 자동으로 주입**하는 `@Autowired` 기능도 제공합니다.

- 컴포넌트 스캔을 사용하려면 `@ComponentScan`을 설정 정보에 붙여주면 됩니다. (기존의 `AppConfig`와는 다르게 `@Bean`으로 등록한 클래스가 하나도 없습니다.)
- 컴포넌트 스캔은 `@Component` 애노테이션이 붙은 클래스를 스캔해서 스프링 빈으로 등록합니다. 따라서 각 클래스가 스캔의 대상이 되도록 `@Component` 애노테이션을 붙여주어야 합니다.
- 스프링 빈과의 의존 관계 주입은 각 클래스 안에서 해결해야 합니다. 이때 `@Autowired`를 사용합니다.

```java
@Configuration
@ComponentScan(
//        basePackages = {"hello.core"},
        excludeFilters = @ComponentScan.Filter(type = FilterType.ANNOTATION, classes = Configuration.class)
)
public class AutoAppConfig {
}
```

```java
@Component
public class OrderServiceImpl implements OrderService {

    private final MemberRepository memberRepository;
    private final DiscountPolicy discountPolicy;

    @Autowired
    public OrderServiceImpl(MemberRepository memberRepository, DiscountPolicy discountPolicy) {
        this.memberRepository = memberRepository;
        this.discountPolicy = discountPolicy;
    }
}
```

>컴포넌트 스캔을 사용하면 `@Configuration`이 붙은 설정 정보도 자동으로 등록되기 때문에, `AppConfig`, `TestConfig`등 앞서 만들어두었던 설정 정보도 함께 등록되고 실행되어 버립니다. 그래서 `excludeFilters`를 사용해서 설정 정보는 컴포넌트 스캔 대상에서 제외했습니다. 보통은 설정 정보를 컴포넌트 스캔 대상에서 제외하지 않지만, 기존 예제 코드를 최대한 남기기 위해서 이 방법을 선택했습니다.

1. `@ComponenetScan`
   - `@Component`가 붙은 모든 클래스를 스프링 빈으로 등록합니다.
   - 스프링 빈의 기본 이름은 클래스명을 사용하되 맨 앞 글자만 소문자를 사용합니다.
2. `@Autowired` 의존 관계 자동 주입
   - 생성자에 `@Autowired`를 지정하면, 스프링 컨테이너가 자동으로 해당 스프링 빈을 찾아서 주입합니다.
   - 기본 조회 전략은 타입이 같은 빈을 찾는 것입니다.

### 탐색 위치

모든 클래스를 스캔하면 시간이 오래 걸릴 수 있기 때문에 특정 위치부터 탐색하도록 시작 위치를 지정할 수 있습니다.

```java
@ComponentScan(
	basePackages = {"hello.core"}
)
```

- `basePackages`로 시작 위치를 지정해서, 해당 패키지를 포함한 하위 패키지를 모두 탐색합니다.
- 만약 지정하지 않으면 `@ComponentScan`이 붙은 설정 정보 클래스의 패키지가 시작 위치가 됩니다.

권장하는 방법은 패키지 위치를 별도로 지정하지 않고, 설정 정보 클래스의 위치를 프로젝트 최상단에 두는 것입니다. 이렇게 하면 하위 패키지가 모두 스캔 대상이 됩니다.

### 컴포넌트 스캔 기본 대상

컴포넌트 스캔은 `@Component` 뿐만 아니라 여러 대상을 추가로 포함합니다.

- `@Component`

  컴포넌트 스캔에서 사용합니다.

- `@Controller`

  스프링 MVC 컨트롤러로 인식합니다.

- `@Service`

  특별한 처리를 하지 않지만, 보통 개발자들이 핵심 비즈니스 로직을 여기에 위치시켜서 비즈니스 계층을 인식하는데 도움이 됩니다.

- `@Repository`

  스프링 데이터 접근 계층으로 인식하고, 데이터 계층의 예외를 스프링 예외로 변환해줍니다.

- `@Configuration`

  스프링 설정 정보로 인식하고, 스프링 빈이 싱글톤을 유지하도록 추가 처리를 합니다.

### 필터

```java
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface MyIncludeComponent {
}
```

```java
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface MyExcludeComponent {
}
```

```java
@MyIncludeComponent
public class BeanA {}
```

```java
@MyExcludeComponent
public class BeanB {}
```

```java
public class ComponentFilterAppConfigTest {
    @Test
    void filterScan() {
        AnnotationConfigApplicationContext ac = new AnnotationConfigApplicationContext(ComponentFilterAppConfig.class);
        BeanA beanA = ac.getBean("beanA", BeanA.class);
        assertThat(beanA).isNotNull();
        assertThrows(
                NoSuchBeanDefinitionException.class,
                () -> ac.getBean("beanB", BeanB.class)
        );
    }

    @Configuration
    @ComponentScan(
            includeFilters = @ComponentScan.Filter(type = FilterType.ANNOTATION, classes = MyIncludeComponent.class),
            excludeFilters = @ComponentScan.Filter(type = FilterType.ANNOTATION, classes = MyExcludeComponent.class)
    )
    static class ComponentFilterAppConfig {
    }
}
```

- `includeFilters`: 컴포넌트 스캔 대상을 추가로 지정합니다.
- `excludeFilters`: 컴포넌트 스캔에서 제외할 대상을 지정합니다.

### 중복 등록과 충돌

1. 자동 빈 등록 vs 자동 빈 등록

   컴포넌트 스캔에 의해 자동으로 스프링 빈이 등록되는데, 이름이 같은 경우 스프링은 오류를 발생시킵니다.

2. 수동 빈 등록 vs 자동 빈 등록

   수동 빈이 자동 빈을 오버라이딩 해버려서, 수동 빈 등록이 우선권을 가집니다.

## 의존 관계 자동 주입

### 주입 방법

DI에는 크게 4가지 방법이 있습니다.

- 생성자 주입

  ```java
  @Component
  public class OrderServiceImpl implements OrderService {
      private final MemberRepository memberRepository;
      private final DiscountPolicy discountPolicy;
  
      @Autowired
      public OrderServiceImpl(MemberRepository memberRepository, DiscountPolicy discountPolicy) {
          this.memberRepository = memberRepository;
          this.discountPolicy = discountPolicy;
      }
  }
  ```

  생성자 호출 시점에 딱 1번만 호출되는 것이 보장되는 것이 특징으로, **불변**, **필수** 의존 관계에 사용합니다.

  그리고 생성자가 1개인 스프링 빈이라면, `@Autowired`를 생략해도 자동 주입이 됩니다.

- setter 주입

  ```java
  @Component
  public class OrderServiceImpl implements OrderService {
      private MemberRepository memberRepository;
      private DiscountPolicy discountPolicy;
      
      @Autowired
      public void setMemberRepository(MemberRepository memberRepository) {
      	this.memberRepository = memberRepository;
      }
      
      @Autowired
      public void setDiscountPolicy(DiscountPolicy discountPolicy) {
      	this.discountPolicy = discountPolicy;
      }
  }
  ```

  필드 값을 변경하는 수정자 메서드를 통해서 , **선택**, **변경** 가능성이 있는 의존 관계에 사용합니다.

- 필드 주입

  ```java
  @Component
  public class OrderServiceImpl implements OrderService {
  	@Autowired
  	private MemberRepository memberRepository;
  	@Autowired
  	private DiscountPolicy discountPolicy;
  }
  ```

  필드에 바로 주입하는 방법으로, 코드가 간결하지만 DI 프레임워크가 없으면 아무것도 할 수 없게 되므로 실제 코드에서는 사용하지 않은 것이 좋습니다. 테스트 코드 혹은 스프링 설정을 목적으로 하는 `@Configuration` 같은 곳에서만 특별한 용도로 사용합니다.

- 일반 메서드 주입

  ```java
  @Component
  public class OrderServiceImpl implements OrderService {
  	private MemberRepository memberRepository;
  	private DiscountPolicy discountPolicy;
  	
      @Autowired
  	public void init(MemberRepository memberRepository, DiscountPolicy discountPolicy) {
  		this.memberRepository = memberRepository;
  		this.discountPolicy = discountPolicy;
  	}
  }
  ```

  한번에 여러 필드를 주입 받을 수 있으나 일반적으로 잘 사용하지 않습니다.

### 생성자 주입

최근에는 DI 프레임워크 대부분이 다음과 같은 이유로 생성자 주입을 권장합니다.

- 대부분의 의존 관계는 애플리케이션 종료 전까지 변경할 일이 없습니다.
- 수성자 주입을 사용하려면 메서드를 public으로 열어두어야 하는데, 이는 좋은 설계법이 아닙니다.
- 필요한 의존 관계가 누락되었을 때, 컴파일 오류로 쉽게 고칠 수 있습니다. (추가로, `final` 키워드를 사용할 수도 있습니다.)

따라서 개발을 할 때 대부분의 경우 생성자에 `final` 키워드를 사용해서 만드는데, 이를 간편하게 해주는 라이브러리 **롬복**이 있습니다.

```java
@Component
@RequiredArgsConstructor
public class OrderServiceImpl implements OrderService {
    private final MemberRepository memberRepository;
    private final DiscountPolicy discountPolicy;
}
```

롬복 라이브러리가 제공하는 `@RequiredArgsConstructor` 기능을 사용하면 `final`이 붙은 필드를 모아서 생성자를 자동으로 만들어줍니다.

### 의존 관계의 조회되는 빈이 2개 이상인 경우

```java
@Autowired
private DiscountPolicy discountPolicy
```

```java
@Component
public class FixDiscountPolicy implements DiscountPolicy {}
```

```java
@Component
public class RateDiscountPolicy implements DiscountPolicy {}
```

`@Autowired`는 타입으로 조회하기 때문에 선택된 빈이 2개 이상일 때 `NoUniqueBeanDefinitionException` 오류가 발생합니다. 이때 하위 타입으로 지정할 수도 있지만, DIP를 위배하고 유연성이 떨어지기 때문에 `@Autowried`에 필드명을 적용해서 해결합니다.

- `@Autowired` 필드명 매칭

  ```java
  @Autowired
  private DiscountPolicy rateDiscountPolicy
  ```

  `@Autowired`는 타입 매칭을 시도하고 ,이때 여러 빈이 있으면 필드 이름, 파라미터 이름으로 빈 이름을 추가 매칭합니다. 필드명이 `rateDiscountPolicy`이므로 정상 주입됩니다.

- `@Qualifier` 사용

  ```java
  @Component
  @Qualifier("mainDiscountPolicy")
  public class RateDiscountPolicy implements DiscountPolicy {}
  ```

  ```java
  @Component
  @Qualifier("fixDiscountPolicy")
  public class FixDiscountPolicy implements DiscountPolicy {}
  ```

  ```java
  @Autowired
  public OrderServiceImpl(MemberRepository memberRepository, @Qualifier("mainDiscountPolicy") DiscountPolicy discountPolicy) {
  	this.memberRepository = memberRepository;
  	this.discountPolicy = discountPolicy;
  }
  ```

  빈 등록시 `@Qualifier`를 붙여서 등록하고, 의존 관계 주입시에 `@Qualifier`로 등록한 이름을 적어줍니다. 만약 주입할 때 `@Qualifier`로 등록한 이름이 없다면, 빈 이름을 추가로 찾습니다.

  추가로, `@Qualifier("mainDiscountPolicy")` 이렇게 문자를 적으면 컴파일시 타입 체크가 안됩니다. 다음과 같은 애노테이션을 만들어서 문제를 해결할 수 있습니다.

  ```java
  @Target({ElementType.FIELD, ElementType.METHOD, ElementType.PARAMETER, ElementType.TYPE, ElementType.ANNOTATION_TYPE})
  @Retention(RetentionPolicy.RUNTIME)
  @Inherited
  @Documented
  @Qualifier("mainDiscountPolicy")
  public @interface MainDiscountPolicy {
  }
  ```

  ```java
  @Component
  @MainDiscountPolicy
  public class RateDiscountPolicy implements DiscountPolicy {}
  ```

  ```java
  @Autowired
  public OrderServiceImpl(MemberRepository memberRepository, @MainDiscountPolicy DiscountPolicy discountPolicy) {
  	this.memberRepository = memberRepository;
  	this.discountPolicy = discountPolicy;
  }
  ```

  애노테이션에는 상속이라는 개념은 없습니다. 다만 스프링은 여러 애노테이션을 모아서 사용하는 기능을 제공합니다.

- `@Primary` 사용

  ```java
  @Component
  @Primary
  public class RateDiscountPolicy implements DiscountPolicy {}
  ```

  ```java
  @Component
  public class FixDiscountPolicy implements DiscountPolicy {}
  ```

  ```java
  @Autowired
  public OrderServiceImpl(MemberRepository memberRepository, DiscountPolicy discountPolicy) {
  	this.memberRepository = memberRepository;
  	this.discountPolicy = discountPolicy;
  }
  ```

  `@Primary`로 우선 순위를 정해서 의존 관계를 주입할 수 있습니다.

`@Primary`는 마치 기본값처럼 동작하는 것이고, `@Qualifier`는 매우 상세하게 동작합니다. 두 경우를 모두 사용한 경우 자동보다는 수동이, 넓은 범위의 선택권보다는 좁은 범위의 선택권이 우선 순위가 높아서 `@Qualifier`의 우선권이 높습니다.

### 조회한 빈이 모두 필요한 경우

```java
public class AllBeanTest {
    @Test
    void findAllBean() {
        ApplicationContext ac = new AnnotationConfigApplicationContext(AutoAppConfig.class, DiscountService.class);
        DiscountService discountService = ac.getBean(DiscountService.class);
        assertThat(discountService).isInstanceOf(DiscountService.class);
        
        Member member = new Member(1L, "userA", Grade.VIP);
        int discountPrice = discountService.discount(member, 10000, "fixDiscountPolicy");
        assertThat(discountPrice).isEqualTo(1000);
    }
    
    static class DiscountService {
        private final Map<String, DiscountPolicy> policyMap;
        private final List<DiscountPolicy> policies;
        
        public DiscountService(Map<String, DiscountPolicy> policyMap, List<DiscountPolicy> policies) {
            this.policyMap = policyMap;
            this.policies = policies;
        }
        
        public int discount(Member member, int price, String discountCode) {
            DiscountPolicy discountPolicy = policyMap.get(discountCode);
            return discountPolicy.discount(member, price);
        }
    }
}
```

- `Map<String, DiscountPolicy>`: 맵의 키에 스프링 빈의 이름을 넣어주고, 그 값으로 `DiscountPolicy` 타입으로 조회한 모든 스프링 빈을 담아줍니다.
- `List<DiscountPolicy>`: `DiscountPolicy` 타입으로 조회한 모든 스프링 빈을 담아줍니다. (만약 해당하는 타입의 스프링 빈이 없으면, 빈 컬렉션이나 맵을 주입합니다.)

### 자동, 수동의 올바른 실무 운영 기준

스프링은 `@Component` 뿐만 아니라 `@Controller`, `@Service`, `@Repository`처럼 계층에 맞추어 애플리케이션 로직을 자동으로 스캔할 수 있도록 지원합니다. 사실 설정 정보를 기반으로 애플리케이션을 구성하는 부분과 동작하는 부분을 명확하게 나누는 것이 이상적이자만, 개발자 입장에서 스프링 빈을 하나 등록할 때 `@Component`만 넣어주면 끝나는 일을 설정 정보를 위해 여러 과정을 번거롭게 해야합니다. 그래서 점점 자동을 선호하는 추세입니다.

애플리케이션은 크게 업무 로직과 기술 지원 로직으로 나눌 수 있습니다.

- **업무 로직 빈**: 웹을 지원하는 컨트롤러, 핵심 비즈니스 로직이 있는 서비스, 데이터 계층의 로직을 처리하는
  리포지토리등이 모두 업무 로직에 하당합니다. 보통 비즈니스 요구사항을 개발할 때 추가되거나 변경됩니다.
- **기술 지원 빈**: 기술적인 문제나 공통 관심사(AOP)를 처리할 때 주로 사용됩니다. 데이터베이스 연결이나, 공
  통 로그 처리처럼 업무 로직을 지원하기 위한 하부 기술이나 공통 기술들입니다.

기술 지원 로직은 업무 로직과 비교해서 그 수가 매우 적고, 보통 애플리케이션 전반에 걸쳐서 광범위하게 영향을 미칩니다. 그리고 적용이 잘 되고 있는지 아닌지 조차 파악하기 어려운 경우가 많습니다. 그래서 이런 기술 지원 로직들은 가급적 수동 빈 등록을 사용해서 명확하게 들어내는 것이 좋습니다.

## 빈 생명 주기 콜백

데이터베이스 커넥션 풀처럼 애플리케이션 시작 시점에 필요한 연결을 미리 해두고, 애플리케이션 종료 시점에 연결을 모두 종료하는 작업을 진행하려면, 객체의 초기화와 종료 작업이 필요합니다.

스프링 빈은 객체 생성 후, 의존 관계를 주입하는 순서로 라이프사이클이 동작합니다. 따라서 초기화 작업은 의존 관계 주입이 완료되고 난 다음에 호출해야 합니다. 스프링 빈에게 콜백 메스드를 통해서 이 시점을 알려주는 다양한 기능이 있습니다. 또한 스프링은 스프링 컨테이너가 종료되기 직전에 소멸 콜백을 줍니다.

### 스프링 빈의 이벤트 라이프사이클

1. 스프링 컨테이너 생성
2. 스프링 빈 생성
3. 의존 관계 주입
4. 초기화 콜백: 빈이 생성되고 빈의 의존 관계 주입이 완료된 후 호출
5. 로직
6. 소멸 전 콜백: 빈이 소멸되기 직전에 호출
7. 스프링 종료

> **객체의 생성과 초기화를 분리하는 편이 좋습니다.**
>
> 생성자는 필수 정보를 받고, 메모리를 할당해서 객체를 생성하는 책임을 가집니다. 반면에 초기화는 이렇게 생성된 값들을 활용해서 외부 커넥션을 연결하는 등 무거운 동작을 수행합니다. 따라서 이 두 부분을 명확하게 나누는 것이 유지보수 관점에서 좋습니다.

### 빈 생명 주기 콜백 방법

- 인터페이스 `InitializingBean`, `DisposableBean`

  ```java
  public class NetworkClient implements InitializingBean, DisposableBean {
  	private String url;
  
      public NetworkClient() {
  		System.out.println("생성자 호출, url = " + url);
  	}
  
  	public void connect() {
  		System.out.println("connect: " + url);
  	}
  
      public void call(String message) {
  		System.out.println("call: " + url + " message = " + message);
  	}
  
  	public void disConnect() {
  		System.out.println("close + " + url);
  	}
  
      @Override
  	public void afterPropertiesSet() throws Exception {
  		connect();
          call("초기화 연결 메시지");
  	}
  
      @Override
  	public void destroy() throws Exception {
  		disConnect();
  	}
  }
  ```

  - `InitializingBean`은 `afterPropertiesSet()` 메서드로 초기화를 지원합니다.
  - `DisposableBean`은 `destory()` 메서드로 소멸을 지원합니다.

  이 인터페이스는 스프링 전용 인터페이스로, 해당 코드가 스프링에 의존하게 됩니다. 그리고 초기화, 소멸 메서드의 이름을 변경할 수 없고, 더불어 외부 라이브러리에 적용할 수 없습니다. 이 방법은 최근에는 거의 사용하지 않습니다.

- 빈 등록 초기화, 소멸 메서드 지정

  ```java
  @Configuration
  static class LifeCycleConfig {
      @Bean(initMethod = "init", destroyMethod = "close")
      public NetworkClient networkClient() {
          NetworkClient networkClient = new NetworkClient();
          networkClient.setUrl("http://hello-spring.dev");
          return networkClient;
      }
  }
  ```

  이 방식은 스프링 코드에 의존하지 않으면서 설정 정보에 메서드를 자유롭게 지정할 수 있습니다. 더불어 외부 라이브러리에도 적용할 수 있습니다.

- 애노테이션 `@PostConstruct`, `@PreDestory`

  ```java
  public class NetworkClient {
  	private String url;
  
      public NetworkClient() {
  		System.out.println("생성자 호출, url = " + url);
  	}
  
  	public void connect() {
  		System.out.println("connect: " + url);
  	}
  
      public void call(String message) {
  		System.out.println("call: " + url + " message = " + message);
  	}
  
  	public void disConnect() {
  		System.out.println("close + " + url);
  	}
  
      @PostConstruct
      public void init() {
          connect();
          call("초기화 연결 메세지");
      }
  
      @PreDestroy
      public void close() {
          disconnect();
      }
  }
  ```

  최신 스프링에서 가장 권장하는 방법으로, 스프링에 종속적이지 않은 자바 표준입니다. 다만 외부 라이브러리에는 적용하지 못한다는 단점이 있습니다.

## 빈 스코프

스프링 빈은 기본적으로 싱글톤 스코프로 생성되어 스프링 컨테이너의 시작과 함께 생성되어서 스프링 컨테이너가 종료될 때까지 유지됩니다. 하지만 이외에도 다양한 스코프를 지원합니다.

- **싱글톤**: 디폴트 스코프로, 스프링 컨테이너의 시작과 종료까지 유지되는 가장 넓은 범위의 스코프입니다.
- **프로토타입**: 스프링 컨테이너는 프로토타입 빈의 생성과 의존 관계 주입까지만 관여하고 더는 관리하지 않는 매우 짧은 범위의 스코프입니다.
- **웹 관련 스코프**
  - **request**: 웹 요청이 들어오고 나갈 때까지 유지되는 스코프입니다.
  - **session**: 웹 세션이 생성되고 종료될 때까지 유지되는 스코프입니다.
  - **application**: 웹의 서블릿 컨텍스와 같은 범위로 유지되는 스코프입니다.

### 프로토타입 스코프

해당 스코프를 스프링 컨테이너에 조회하면 스프링 컨테이너는 항상 새로운 인스턴스를 생성해서 반환합니다. 여기서 핵심은 **스프링 컨테이너는 프로토타입 빈을 생성하고, 의존 관계 주입과 초기화까지만 처리**한다는 것입니다. 클라이언트에 빈을 반환한 후 스프링 컨테이너는 생성된 프로토타입 빈을 관리하지 않습니다. 그래서 `@PreDestory` 같은 종료 메서드가 호출되지 않습니다.

#### 싱글톤 빈에서 프로토타입 빈 사용시 문제점

![](./images/singleton-prototype-bean-problem.png)

싱글톤 빈은 보통 스프링 컨테이너 생성 시점에 함께 생성되고, 의존 관계 주입도 발생합니다. 따라서 주입 시점에 스프링 컨테이너에 프로토타입 빈을 요청해서 내부 필드에 보관합니다. 그런데 싱글톤 빈은 생성 시점에만 의존 관계 주입을 받기 때문에, 프로토타입 빈이 싱글톤 빈과 함께 계속 유지되는 문제가 생깁니다.

#### Provider로 문제 해결

싱글톤 빈과 프로토타입 빈을 함께 사용할 때마다 항상 새로운 프로토타입 빈을 생성하기 위해서는, 사용할 때마다 스프링 컨테이너에 새로 요청하는 것이 가장 간단한 방법입니다. 이처럼 지정한 비을 컨테이너에서 대신 찾아주는 DL 서비스를 제공하는 것이 `ObjectProvider`입니다.

```java
public class SingletonBean {
	@Autowired
	private ObjectProvider<PrototypeBean> prototypeBeanProvider;

    public int logic() {
		PrototypeBean prototypeBean = prototypeBeanProvider.getObject();
		prototypeBean.addCount();
		return prototypeBean.getCount();
	}   
}
```

- `ObjectProvider`의 `getObject()`를 호출하면 내부에서는 스프링 컨테이너를 통해 해당 빈을 찾아서 반환합니다. (DL)
- 이외의 편의 기능을 많이 제공하고, 스프링에 의존적인 점이 특징입니다.

자바 표준의 Provider를 사용하는 방법도 있습니다.

```java
public class SingletonBean {
    @Autowired
    private Provider<PrototypeBean> prototypeBeanObjectProvider;

    public int logic() {
	    PrototypeBean object = prototypeBeanObjectProvider.get();
        object.addCount();
        return object.getCount();
	}   
}
```

- `provider`의 `get()`을 호출하면 내부에서는 스프링 컨테이너를 통해 해당 빈을 찾아서 반환합니다. (DL)
- 별도의 라이브러리가 필요하지만, 자바 표준으로 다른 컨테이너에서도 사용할 수 있는 점이 특징입니다.

### 웹 스코프

웹 환경에서만 동작하는 스코프로, 스프링이 해당 스코프의 종료 시점까지 관리하여 종료 메서드가 호출됩니다.

#### 웹 소코프 종류

- **request**: HTTP 요청 하나가 들어오고 나갈 때 까지 유지되는 스코프로, 각각의 HTTP 요청마다 별도의 빈
  인스턴스가 생성되고, 관리됩니다.
- **session**: HTTP Session과 동일한 생명주기를 가지는 스코프입니다.
- **application**: ServletContext와 동일한 생명주기를 가지는 스코프입니다.
- **websocket**: 웹 소켓과 동일한 생명주기를 가지는 스코프입니다.

#### requset 스코프 예제

동시에 여러 HTTP 요청이 올 때 정확히 어떤 요청이 남긴 로그인지 구분하기 위해 request 스코프를 사용해보겠습니다.

```java
@Component
@Scope(value = "request")
public class MyLogger {
    private String uuid;
    private String requestURL;

    public void log(String message) {
        System.out.println("[" + this.uuid + "]" + "[" + this.requestURL + "] " + message);
    }
    
    public void setRequestURL(String requestURL) {
        this.requestURL = requestURL;
    }

    @PostConstruct
    public void init() {
        this.uuid = UUID.randomUUID().toString();
        System.out.println("[" + this.uuid + "] request scope bean create: " + this);
    }

    @PreDestroy
    public void destroy() {
        System.out.println("[" + this.uuid + "] request scope bean close: " + this);
    }
}
```

- 로그를 출력하기 위한 클래스로, request 스코프로 지정하여 이 빈은 HTTP 요청 당 하나씩 생성되고, HTTP 요청이 끝나는 시점에 소멸됩니다.
- 이 빈이 생성되는 시점에 자동으로 초기화 메서드를 사용해서 uuid를 저장합니다. 이 빈은 HTTP 요청 당 하나씩 생성되므로, 다른 HTTP 요청과 구분할 때 uuid를 사용합니다.
- 이 빈이 소멸되는 시점에 소멸 전 메서드로 종료 메시지를 남깁니다.

```java
@Controller
@RequiredArgsConstructor
public class LogDemoController {
    private final LogDemoService logDemoService;
    private final ObjectProvider<MyLogger> myLoggerObjectProvider;
//    private final MyLogger myLogger;

    @RequestMapping("log-demo")
    @ResponseBody
    public String logDemo(HttpServletRequest request) {
        MyLogger myLogger = myLoggerObjectProvider.getObject();
        String requestURL = request.getRequestURL().toString();
        myLogger.setRequestURL(requestURL);

        myLogger.log("controller test");
        logDemoService.logic("test id");
        return "OK";
    }
}
```

```java
@Service
@RequiredArgsConstructor
public class LogDemoService {
    private final ObjectProvider<MyLogger> myLoggerObjectProvider;
//    private final MyLogger myLogger;

    public void logic(String id) {
        MyLogger myLogger = myLoggerObjectProvider.getObject();
        myLogger.log("service id = " + id);
    }
}
```

- 스프링 애플리케이션을 실행하는 시점에 싱글톤 빈은 생성해서 주입이 가능하지만, request 스코프 빈은 아직 생성되지 않기 때문에 `MyLogger` 빈이 아직 만들어지기 전입니다.
- 이를 해결하기 위해 Provider를 사용해서 request 스코프 빈의 생성을 지연할 수 있습니다.

### 스코프와 프록시

```java
@Component
@Scope(value = "request", proxyMode = ScopedProxyMode.TARGET_CLASS)
public class MyLogger {
}
```

- 프록시 방식으로, `MyLogger`의 가짜 프록시 클래스를 만들어두고, HTTP request와 상관 없이 가짜 프록시 클래스를 다른 빈에 미리 주입해둘 수 있습니다.

```java
@Controller
@RequiredArgsConstructor
public class LogDemoController {
    private final LogDemoService logDemoService;
    private final MyLogger myLogger;

    @RequestMapping("log-demo")
    @ResponseBody
    public String logDemo(HttpServletRequest request) {
        String requestURL = request.getRequestURL().toString();
        myLogger.setRequestURL(requestURL);

        myLogger.log("controller test");
        logDemoService.logic("test id");
        return "OK";
    }
}
```

```java
@Service
@RequiredArgsConstructor
public class LogDemoService {
    private final MyLogger myLogger;

    public void logic(String id) {
        myLogger.log("service id = " + id);
    }
}
```

- CGLIB라는 라이브러리로 클래스를 상속 받은 가짜 프록시 객체를 만들어서 주입합니다.
- 가짜 프록시 객체는 요청이 오면 그때 내부에서 진짜 빈을 요청하는 위임 로직이 들어 있습니다.
- 가짜 프록시 객체는 원본 클래스를 상속 받아서 만들어졌기 때문에 이 객체를 사용하는 클라이언트 입장에서는 사실 원본인지 아닌지도 모르게, 동일하게 사용할 수 있습니다.(다형성)



















