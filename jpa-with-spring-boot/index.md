---
title: 스프링 부트와 JPA 활용 1편 정리
date: 2021-02-22
pin: false
tags:
- Spring
- JPA
---

저는 이 강의를 JPA에 대한 깊은 이해 없이 야생형 학습 방식으로, 선제적으로 수강했습니다.



## 프로젝트 환경설정

### 프로젝트 생성

[Spring Boot Starter](https://start.spring.io/)로 프로젝트 기본 세팅을 할 수 있습니다. 최근 추세가 Maven에서 Gradle로 넘어가는 성향이어서 프로젝트 종류는 Gradle로 진행했습니다. 사용하는 모듈은 아래와 같습니다.

- Spring Web: 기본적인 Web에 필요한 dependency를 가져옵니다.
- Spring Data JPA: 주로 Spring Data와 Hibernate를 이용하여 JPA Data를 처리합니다.
- Thymeleaf: Spring에서 밀고있는 server-side template으로, 해당 형식의 문서를 별도의 수정 작업없이 HTML로 바로 볼 수 있다는 점이 강점입니다.
- Validation: `@NotNull`, `@Min`과 같이 도메인에 대한 검증을 하는 `Bean Validation`의 구현체로써 Hibernate를 사용합니다.
- Lombok: 다양한 annotation library로, 주로 `Getter`와 `Setter`를 편리하게 세팅할 때 사용합니다.

### DB 설치

실습용 application 제작을 하기 때문에 비교적 간단히 사용할 수 있는 in-memory 방식의 H2 DB를 사용합니다. [H2](https://h2database.com/)에서 다운로드 및 설치 후 `./bin/` 경로의 OS에 맞게 `bat` 혹은 `sh` 파일을 실행해서 구동합니다. 처음에는 `jdbc:h2:~/{Table Name}`로 접속해서 table을 생성 후, 이후에는 `jdbc:h2:tcp://localhost/~/{Table Name}`으로 접속할 수 있습니다. 그리고 Spring Boot 프로젝트에서 DB 설정에 맞게 `application.properties` 혹은 `application.yml`에 프로젝트 설정을 해줍니다. 설정 항목이 많다면 `yml` 형식이 유용하므로 해당 형식을 사용했습니다.

```yaml
spring:
  datasource:
    url: jdbc:h2:tcp://localhost/~/{Table Name}
    username: sa
    password:
    driver-class-name: org.h2.Driver
  jpa:
    hibernate:
      ddl-auto: create
    properties:
      hibernate:
        # show_sql: true
        format_sql: true
logging.level:
  org.hibernate.SQL: debug
  # org.hibernate.type: trace
```



## 도메인 분석 및 설계

### 도메인 모델

우선 application 개발에 필요한 도메인을 정의합니다. 예를 들어 간단한 쇼핑몰을 만든다면, `회원`, `상품`, `주문`, `배송`등이 있을 수 있습니다. 그리고 각 도메인 간의 관계를 설정합니다. `회원`과 `주문`의 관계는 `1:N`일 것이고,  `주문`과 `배송`의 관계는 `1:1`일 것입니다. 더불어 `상품`을 구현할 때 필요하다면 상속 관계도 사용합니다.

<center><img src="./images/domain_example.png" style="zoom:50%;" />          <img src="./images/entity_example.png" style="zoom:37%;" /></center>

### 테이블 설계

도메인이 어느정도 확정되었다면 그걸 바탕으로 테이블을 구성합니다. 어떤 필드가 PK(Primary Key)가 되어야 하며, 도메인 간의 관계에서는 어느 도메인이 FK(Foreign Key)를 갖도록 설정할 지에 대한 것들을 설정합니다. 주로 FK가 있는 도메인을 해당 연관관계의 주인으로 정하는 것이 권장됩니다. 예를 들어 `회원`과 `주문` 도메인은 `1:N` 관계인데, `1:N` 관계에서  FK는 항상 `N`쪽에 FK가 위치하므로`주문` 도메인에 위치하여 이 연관관계의 주인은 `주문`이 됩니다. 여기서 연관관계의 주인이란, 단순히 FK를 누가 관리하냐의 문제이지 비즈니스상 우위에 있다는 의미가 아닙니다. 예를 들어, `자동차`와 `바퀴` 도메인이 있는데 연관관계의 주인을 `자동차`로 정하면 `자동차`가 관리하지 않는 `바퀴` 테이블의 FK 값이 업데이트되므로 유지/보수가 어려워집니다.

<img src="./images/table_example.png" style="zoom:50%;" />

### Entity 클래스 개발

설정한 도메인 모델과 테이블 설계에 맞게 프로젝트에 entity 클래스를 만듭니다. `@Entity` annotation으로 해당 클래스가 영속적인 도메인 오브젝트임을 표현합니다. 해당 entity에 `@Id`, `@GeneratedValue`, `@Column(name = "{Column Name}")` 등의 annotation으로 필드를 표현합니다. 그리고 entity간의 연관관계는 `@OneToOne`, `@OneToMany`, `@ManyToOne`, `@ManyToMany`로 표현하고, 연관관계의 주인 표현은 주인 entity 쪽에 `@JoinColumn(name = "{Column Name}")`과 그렇지 않은 entity 쪽에 `@OneToMany(mappedBy = "{주인 Entity의 Field Name}")`으로 합니다.

연관관계를 구현할 때는 `EAGER`(즉시 로딩)보다는 `LAZY`(지연 로딩) 방식으로 설정해야 합니다. `EAGER`일 경우 예측이 어렵고, 어떤 SQL이 실행될 지 추적하기 여렵습니다. 특히 N+1 문제 등이 자주 발생할 수 있습니다. `@XToMany`의 경우 기본값이 `LAZY`이지만, `@XToOne`의 경우 기본값이 `EAGER`이기 때문에 `@ManyToOne(fetch = FetchType.LAZY)`처럼 설정해야 합니다. 만약 연관된 entity를 함께 조회해야하는 경우라면, fetch join 혹은 entity graph 기능을 사용하여 해결합니다.

컬렉션은 필드에서 초기화하는 것이 안전합니다. `1:N` 관계의 경우 `List<>()`를 사용해서 컬렉션을 표현하는데 필드에서 초기화할 수도 있고, 필드에서는 선언만 하고 생성자에서 초기할 수도 있습니다. 선언과 초기화를 분리한다면 `NullPointerException` 문제가 발생할 수 있기 때문에 약간의 메모리를 더 소비하더라도 필드에서 초기화하는 것이 안전합니다.

예를 들어 `회원`과 `주문` entity는 아래처럼 구현됩니다.

```java
@Entity
@Getter @Setter
public class Member {
    @Id @GeneratedValue
    @Column(name = "member_id")
    private Long id;
    
    private String name;
	
    @Embedded
    private Address address;

    @OneToMany(mappedBy = "member")
    private List<Order> orders = new ArrayList<>();
}
```

```java
@Entity
@Table(name = "orders")
@Getter @Setter
public class Order {
    @Id @GeneratedValue
    @Column(name = "order_id")
    private Long id;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "member_id")
    private Member member;

    @OneToMany(mappedBy = "order", cascade = CascadeType.ALL)
    private List<OrderItem> orderItems = new ArrayList<>();

    @OneToOne(cascade = CascadeType.ALL, fetch = FetchType.LAZY)
    @JoinColumn(name = "delivery_id")
    private Delivery delivery;

    private LocalDateTime orderDate;

    @Enumerated(EnumType.STRING)
    private OrderStatus status;
}
```

> 실무에서는 `@ManyToMany`를 사용하지 않는 게 좋습니다.
>
> 편리할 것 같지만 중간 테이블에 column을 추가할 수 없고, 세밀하게 query를 실행하기 어렵기 때문에 실무에서 사용하기에는 한계가 있습니다. 중간 entity를 만들고 `@ManyToOne`, `@OneToMany`로 mapping해서 사용하는 게 좋습니다.



## Application 개발

### Application Architecture

Spring을 활용한 application의 구조 큰 틀은 비슷한 것 같습니다. 주로 계층형 구조를 사용합니다.

- controller, web: 웹 계층
- service: 비즈니스 로직, 트랜잭션 처리
- repository: JPA를 직접 사용하는 계층, entity manager 사용
- domain: entity가 모여 있는 계층, 모든 계층에서 사용

개발은 도메인 설계 및 개발 후, service와 repository 계층을 개발하고, test case로 검증을 마치면 웹 계층을 개발하는 순서로 진행됩니다. 계층 간의 구조도는 아래와 같습니다.

<img src="./images/application_architecture.png" style="zoom:80%;" />

### Repositoy & Service 계층 개발

`주문` 도메인에 대한 repository와 service로 예를 들어 설명하겠습니다.

```java
@Repository
@RequiredArgsConstructor
public class OrderRepository {
    
    private final EntityManager em;
    
    /* persist order logic */
    public void save(Order order) {}
    
    /* find order logic */
    public Order findOne(Long id) {}
    
    
}
```

- `@Repository` annotation은 스프링 빈으로 등록되도록 해주고, JPA Exception을 Spring 기반 Exception으로 변환해줍니다.
- `Entity Manager`은 entity와 관련된 작업을 하기 위해 DB에 액세스하는 역할을 수행합니다. 원래는 `@PersistenceContext` annotation으로 주입을 해야하지만, Spring Boot에서는 entity manage를 `final`처리하고 `lombok`의 `@RequiredArgsConstructor` annotation으로 대체할 수 있다.

```java
@Service
@Transactional(readOnly = true)
@RequiredArgsConstructor
public class OrderService {
    
    private final OrderRepository orderRepository;
    
    /* order logic */
    @Transactional
    public Long order(Long memberId, Long itemId, int count) {}
    
    /* order cancel logic */
    @Transcational
    public void cancelOrder(Long orderId) {}
    
}
```

- `@Service` annotation은 비즈니스 로직을 처리하는 객체를 스프링 빈으로 등록되도록 합니다.
- `@Transactional` annotation은 트랜잭션을 보내고, 트랜잭션의 성질을 기준으로 commit할 지, rollback할 지 판단해줍니다.
  - `readOnly = true` 옵션은 데이터 변경이 없는 메서드에 사용합니다.영속성 context를 flush하지 않으므로 약간의 성능 향상을 기대할 수 있습니다. 기본값은 `false`이므로 메서드의 용도에 맞게 annotation을 정해야 합니다.
- `@RequiredArgsConstructor` annotation은 Spring boot에서 `final` 처리된 filed를 파라미터로 갖는 생성자를 만들어줍니다. 따라서 `@Autowired` annotation을 쓰지 않고, 생성자로 주입하는 코드 없이도 그 역할을 수행할 수 있습니다. 대부분 권장하는 방법 또한 `@Autowired`를 통한 field injection보다는 Constructor injection을 권장하므로 이 방법을 사용하였습니다.

>  참고로, 이 강의에서는 비즈니스 로직 대부분이 entity에 있고 service 계층은 단순히 entity에 필요한 요청을 위함하는 역할을 수행하도록 짜여있습니다. 이처럼 entity가 비즈니스 로직을 가지고 객체 지향의 특성을 적극 활용하는 것을 [도메인 모델 패턴](https://martinfowler.com/eaaCatalog/domainModel.html)이라고 합니다. 반대로 entity에는 비즈니스 로직이 거의 없고 service 계층에서 대부분의 비즈니스 로직을 처리하는 것을 [트랜잭션 스크립트 패턴](https://martinfowler.com/eaaCatalog/transactionScript.html)이라고 합니다.

Spring 강의를 여러개 들으면서 두 패턴 방식으로 개발해보았는데 `도메인 모델 패턴`은 조금 더 객체 지향 목적을 충실히 지키고, 그리고 `트랜잭션 스크립트 패턴`은 코드를 이해하는데 더 직관적이다라고 생각합니다.

### Test Code

작성한 repository와 service 계층이 제대로 동작하는 지 확인하려면 test code를 작성해야 합니다. 지금의 test code는 Spring과 결합하여 작성했지만, 더 좋은 test code는 각 메서드마다 제 역할을 잘 수행하는 지 단위 테스트하는 것입니다.

```java
@RunWith(SpringRunner.class)
@SpringBootTest
@Transactional
public class OrderServiceTest {
    
    @PersistenceContext
    EntityManager em;
    
    @Autowired
    OrderService orderService;
    
    /* test code */
    @Test
    public void 상품주문() throws Exception {}
    
}
```

- `@RunWith(SpringRunner.class)` annotation은 Spring과 테스트를 통합하겠다는 의미입니다.
- `@SpringBootTest` annotation은 테스트 코드를 실행하기 전에 Spring Boot를 띄우고 하겠다는 의미입니다. 이게 없다면 `@Autowired`와 같은 annotation들은 모두 실패합니다.
- `@Transactional` annotation은 각각의 테스트를 실행할 때마다 트랜잭션을 시작하고 테스트가 끝나면 강제로 rollback하여 반복 가능한 테스트를 지원합니다.



## 웹 계층 개발

### 레이아웃

해당 강의에서는 `thymeleaf` server-side template과 `Bootstrap`을 사용해 프론트 부분을 구현합니다.

먼저 `thymeleaf` 적용을 위해 `application.yml`에 추가 설정 정보를 작성해야합니다.

```yaml
spring:
  thymeleaf:
    prefix: classpath:/templates/
    suffix: .html
```

위의 정보를 토대로 Spring에서 렌더링할 View를 매핑합니다.

그리고 `Bootstrap`은 [공식 웹사이트](https://getbootstrap.com/)에서 `css`와 `js`를 다운받을 수 있습니다. 파일 전체를 `resources/static/`경로로 옮기고, 필요한 경우 추가 `css` 파일을 만들어줍니다.

### Controller

`회원`관련 기능과 `주문`관련 기능을 예시로 설명하겠습니다. 먼저 `회원` 등록 및 조회 기능을 위한 Controller 입니다.

```java
@Controller
@RequiredArgsConstructor
public class MemberController {

    private final MemberService memberService;

    @GetMapping(value = "/members")
    public String list(Model model) {
		/* logic for find all members */
        
        model.addAttribute("members", members);
        return "members/memberList";
    }

    @GetMapping(value = "/members/new")
    public String createForm(Model model) {
        model.addAttribute("memberForm", new MemberForm());
        return "members/createMemberForm";
    }

    @PostMapping(value = "/members/new")
    public String create(@Valid MemberForm form, BindingResult result) {
        if (result.hasErrors()) {
            return "members/createMemberForm";
        }
        /* logic for input new member info */
        
        memberService.join(member);
        return "redirect:/";
    }
}
```

- `@Controller`는 Web MVC(Model View Controller)에서 Controller 역할을 수행하도록 Spring에 알려주고 빈으로 등록합니다. Controller는 주로 사용자의 요청을 처리한 후 지정된 View에 Model 객체를 넘겨주는 동작을 수행합니다.
- `@GetMapping`, `@PostMapping`는 GET/POST 요청을 처리하는 메서드에 사용하며, `value` 옵션으로 경로를 지정합니다.

Controller에서 `create` 부분을 살펴보면 entity를 직접 사용하지 않고 form 객체를 통해 동작을 수행합니다. 이는 화면 요구사항이 복잡해지면 해당 기능들이 점점 늘어나서 화면을 위한 로직을 처리하는 객체를 추가로 만든 것입니다. Entity는 핵심 비즈니스 로직만 가지고 있고, 다른 로직은 없어야 합니다. 따라서 주로 form 객체나 DTO(Data Transfer Object)를 사용하여 구현됩니다. 아래의 예시는 `회원` Entity에 대한 Form 객체입니다.

```java
@Getter @Setter
public class MemberForm {
    @NotEmpty(message = "회원 이름은 필수 입니다")
    private String name;

    private String city;
    private String street;
    private String zipcode;
}
```

다음은 `주문` 관련 기능을 위한 Controller입니다.

```java
@Controller
@RequiredArgsConstructor
public class OrderController {

    private final OrderService orderService;
    private final MemberService memberService;
    private final ItemService itemService;

    @GetMapping(value = "/orders")
    public String orderList(@ModelAttribute("orderSearch") OrderSearch orderSearch, Model model) {
		/* logic for find all orders */
        
        model.addAttribute("orders", orders);
        return "order/orderList";
    }

    @GetMapping(value = "/order")
    public String createForm(Model model) {
        /* logic for find all members and items */

        model.addAttribute("members", members);
        model.addAttribute("items", items);
        return "order/orderForm";
    }

    @PostMapping(value = "/order")
    public String order(@RequestParam("memberId") Long memberId,
                        @RequestParam("itemId") Long itemId,
                        @RequestParam("count") int count) {
		/* logic for order */
        
        return "redirect:/orders";
    }

    @PostMapping(value = "/orders/{orderId}/cancel")
    public String cancelOrder(@PathVariable("orderId") Long orderId) {
		/* logic for cancel order */
        
        return "redirect:/orders";
    }
}
```

- `@RequestParam`은 사용자가 요청시 전달하는 값을 매개변수와 1:1 매핑합니다. 다른 옵션없이 annotation 기본 형태만 사용된다면 매개변수명으로 매핑하고 `( )`안에 별도로 이름을 정해줄 수 있습니다.
- `@ModelAttribute`는 사용자가 요청시 전달하는 값을 Object 형태로 매핑합니다. `회원` 예시처럼 Form 객체가 존재하거나 매개할 객체가 존재한다면 이 annotation을 사용해서 한번에 매핑할 수 있습니다.
- `@PathVariable`은 URI의 특정 부분에 접근하여 매개변수로 할당합니다. URI 상에서 `{ }`로 감싸여있는 부분에 접근할 수 있는데, 별도의 annotation 옵션이 없다면 매개변수명과 동일한 URI 특정 부분을 찾고 `( )`안에 이름을 정해서 찾을 수도 있습니다.

### 변경 감지와 병합

이 부분은 웹 계층이라기 보다는 JPA에 해당하는 부분입니다. 웹 계층관련 기능을 개발하다보면 Entity를 업데이트하는 기능을 구현하곤 합니다. 예를 들어 `주문`객체는 처음에 만들어지고 사용자의 요청에 의해 배송지와 같은 정보가 수정될 수 있습니다. 이때 `변경 감지` 혹은 `병합` 방식으로 업데이트가 수행됩니다. 각 방식을 설명하기 전, `준영속 Entity`를 설명하겠습니다.

`준영속 Entity`는 영속성 Context가 더는 관리하지 않는 Entity를 말합니다. 예를 들어 주문 정보를 업데이트하는 `updateOrder`라는 메서드가 있다면, 해당 메서드에는 DB에 정보를 업데이트하는 요청을 위임하는 부분이 있을 것입니다. 이때 전달되는 객체가 `준영속 Entity`입니다. 해당 객체는 이미 DB에 한번 저장되어서 식별자를 가지고 있어서 `준영속 Entity`로 볼 수 있습니다. 이 Entity를 수정하는 방식이 바로 `변경 감지`와 `병합`인 것입니다.

`변경 감지` 기능은 영속성 Context에서 Entity를 다시 조회한 후에 데이터를 수정하는 방법입니다. 트랜잭션 안에서 Entity를 다시 조회해서 변경할 값을 선택하고, commit 시점에 변경 감지(Dirty Checking)이 동작해서 DB에 UPDATE SQL이 실행되는 방식입니다. 아래는 간단한 예시입니다.

```java
@Transactional
void update(Order orderParam) {	// orderParam: 파라미터로 넘어온 준영속 상태의 Entity
    Order findOrder = em.find(Order.class, orderParam.getId());	// 동일한 Entitiy를 조회
    findOrder.setAddress(orderParam.getAddress());	// 데이터를 수정
}
```

`병합` 기능은 준영속 상태의 Entity를 영속 상태로 변경합니다. 준영속 Entity의 식별자 값으로 영속 Entity를 조회해서 영속 Entity의 값을 준영속 Entity의 값으로 모두 교체합니다. 트랜잭션 commit 시점에 변경 감지 기능이 동작해서 DB에 UPDATE SQL이 실행되는 방식입니다. 아래는 간단한 예시입니다.

```java
@Transactional
void update(Order orderParam) {	// orderParam: 파라미터로 넘어온 준영속 상태의 Entity
    Order mergeOrder = em.merge(orderParam);
}
```

1. `merge()`를 실행
2. 파라미터로 넘어온 준영속 Entity의 식별자 값으로 1차 캐시, 없다면 DB에서 Entity를 조회
3. 조회한 영속 Entity의 값을 준영속 Entity의 값으로 교체
4. 영속 상태인 Entity를 반환

주의할 점은, `병합`을 사용할 경우 모든 속성이 변경되어 변경할 값이 없다면 `null`로 업데이트될 수 있다는 점입니다. 반면 `변경 감지`는 원하는 속성만 선택해서 변경할 수 있습니다. 실무에서는 보통 업데이트를 위한 기능은 매우 제한적입니다. 그런데 `병합`은 모든 Field를 변경해버리고 데이터가 없으면 `null`로 업데이트 해버립니다. `병합`을 사용하면서 이 문제를 해결하려면, 변경 Form 화면에서 모든 데이터를 항상 유지해야 합니다. 실무에서는 보통 변경 가능한 데이터만 노출하기 때문에, 이는 오히려 번거로울 수 있습니다. 띠리사 Entity를 변경할 때는 항상 `변경 감지`를 사용하는 게 권장됩니다.



이것으로 Spring Boot와 JPA로 Web application을 만들어보는 첫번째 과정이 끝났습니다. 저는 JPA에 대한 탄탄한 이해없이 선제적으로 이 강의를 수강한 것이므로 다음 과정으로 넘어가기 전, JPA에 대한 강의를 듣고 API와 성능 향상을 다루는 과정으로 진행할 계획입니다. 다음은 JPA 강의 포스트로 찾아 뵙겠습니다.