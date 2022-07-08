---
title: 스프링 Data JPA 정리
date: 2021-03-13
pin: false
tags:
- Spring
- JPA
---

# 스프링 데이터 JPA 정리

## 프로젝트 환경 설정

### 프로젝트 생성

[스프링 부트 스타터](https://start.spring.io/)에서 프로젝트 초기 설정을 합니다. `Spring Web`, `Spring Data JPA`, `H2 Database`, `Lombok` 의존성을 추가하여 다운 받은 zip 파일을 IDE로 열어서 동작 확인을 합니다.

최근 InteliJ 버전은 Gradle로 실행을 하는 것이 기본 설정이어서 실행 속도가 느립니다. `Preferences -> Build, Execution, Deployment -> Build Tools -> Gradle` 목록에서 `Build and run using`과 `Run tests using`을 `InteliJ IDEA`로 변경합니다.

그리고 롬복을 적용하기 위해, `Preferences -> plugin -> lombok 검색 실행`을 한 후, `Preferences -> Annotation Processors 검색 -> Enable annotation processing 체크`후 IDE를 재시작합니다.

### 스프링 데이터 JPA와 DB 설정

`application.yml` 파일을 만들어서 구동 방식을 설정합니다.

```yaml
spring:
  datasource:
    url: jdbc:h2:tcp://localhost/~/datajpa
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

- `ddl-auto: create`: 애플리케이션 실행 시점에 테이블을 drop하고 다시 생성합니다.
- `org.hibernate.SQL: debug`: logger를 통해 하이버네이트 실행 SQL을 남깁니다.

추가로 DB에 날리는 쿼리 파라미터 로그를 남기기 위해 외부 라이브러리를 사용합니다.

```gradle
implementation 'com.github.gavlyukovskiy:p6spy-spring-boot-starter:1.5.7'
```

`build.gradle`에 의존성을 추가하면 사용할 수 있습니다. 이 외부 라이브러리는 시스템 자원을 사용하므로, 개발 단계에서는 편하게 사용해도 괜찮습니다. 하지만 운영 시스템에 적용하려면 꼭 성능 테스트를 하고 사용하는 것이 좋습니다.



## 예제 도메인 모델

![](./images/domain-member-model.png) ![](./images/domain-member-erd.png)

도메인 모델과 ERD(Entity Relationship Diagram) 입니다. 이를 토대로 도메인 구현을 하겠습니다.

```java
@Entity
@Getter @Setter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@ToString(of = {"id", "username", "age"})
public class Member {

    @Id @GeneratedValue
    @Column(name = "member_id")
    private Long id;
    private String username;
    private int age;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "team_id")
    private Team team;

//    protected Member() {}   // @Entity는 디폴트 생성자가 있어야 함 && excess 레벨은 private은 불가

    public Member(String username) {
        this.username = username;
    }

    public Member(String username, int age, Team team) {
        this.username = username;
        this.age = age;
        if (team != null) {
            changeTeam(team);
        }
    }

    public void changeTeam(Team team) {
        this.team = team;
        team.getMembers().add(this);
    }

}
```

- 롬복을 활용에 `@Getter`, `@Setter` 설정을 합니다. 물론 실제 프로젝트에서는 수정자 접근은 재고려해야 합니다.
- 엔티티 클래스의 경우 access 수준이 `protected` 이상인 디폴트 생성자가 있어야 합니다. 직접 만들어줘도 되고, `@NoArgsConstructor(access = AccessLevel.PROTECTED)`으로 자동 생성해도 됩니다.
- 향후 해당 객체 정보 출력을 위해 `@ToString`으로 설정합니다. 이때 가급적이면 연관 관계가 없는 내부 필드만 적는 것이 좋습니다.
- `Member`와 `Team`은 `N:1` 관계이므로, `Member` 클래스에 FK가 있어야 하므로, `@JoinColumn(name = "team_id")` 설정을 해주어야 하고, `@ManyToOne` fetch 전략도 `@ManyToOne(fetch = FetchType.LAZY)`을 명시해서 지연 전략으로 설정합니다.
- `changeTeam()`과 같은 메서드를 통해, 즉 연관 관계 편의 메서드를 통해, 양방향 연관 관계 객체를 한번에 처리합니다.

```java
@Entity
@Getter @Setter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@ToString(of = {"id", "name"})
public class Team {

    @Id @GeneratedValue
    @Column(name = "team_id")
    private Long id;
    private String name;

    @OneToMany(mappedBy = "team")
    private List<Member> members = new ArrayList<>();

    public Team(String name) {
        this.name = name;
    }
}
```

- `N:1`관계에서 `1`에 해당하므로 `@OneToMany(mappedBy = "team")`으로 맵핑 설정을 명시합니다. 즉, `Member.team`이 연관 관계의 주인(FK를 소유)이며, `Team.members`는 연관 관계의 주인이 아닙니다. 따라서 `Member.team`이 DB의 FK 값을 변경할 수 있고, 반대편은 읽기만 가능합니다.

우선 설명은 뒤에서 자세히 하고, 스프링 데이터 JPA를 활용해서 동작하도록 Repository를 만들겠습니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
}
```

도메인 구현 및 Repository 구현을 완료했다면 테스트 코드를 작성해서 제대로 구성 및 동작하는지 확인해보겠습니다,

```java
@SpringBootTest    // Junit5부터는 @RunWith(SpringRunner.class)를 사용하지 않아도 됨
@Transactional
@Rollback(false)
class MemberTest {

    @PersistenceContext
    EntityManager em;

    @Test
    public void testEntity() {
        Team teamA = new Team("teamA");
        Team teamB = new Team("teamB");

        em.persist(teamA);
        em.persist(teamB);

        Member member1 = new Member("member1", 10, teamA);
        Member member2 = new Member("member2", 10, teamA);
        Member member3 = new Member("member3", 10, teamB);
        Member member4 = new Member("member4", 10, teamB);

        em.persist(member1);
        em.persist(member2);
        em.persist(member3);
        em.persist(member4);

        // 초기화
        em.flush();
        em.clear();

        // 확인
        List<Member> members = em.createQuery("select m from Member m", Member.class)
                .getResultList();

        for (Member member : members) {
            System.out.println("member = " + member);
            System.out.println("team = " + member.getTeam());
        }
    }

}
```

```java
@SpringBootTest
@Transactional
@Rollback(false)    // 메서드 실행 후 DB에 데이터 롤백을 할지 말지 설정
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;

    @Test
    public void testMember() {
        Member member = new Member("memberA");
        Member savedMember = memberRepository.save(member);

        Optional<Member> byId = memberRepository.findById(savedMember.getId());
        Member findMember = byId.get();

        assertThat(findMember.getId()).isEqualTo(member.getId());
        assertThat(findMember.getUsername()).isEqualTo(member.getUsername());
        assertThat(findMember).isEqualTo(member);
    }

}
```

- `@Transactional`: 각 테스트 코드에서 DB로부터 데이터를 가져오고, 테스트가 끝나면 DB를 원래대로 돌려놓습니다.
- `@Rollback(false)`: 테스트 코드 실행 중에 쿼리를 보낸 것들을 테스트 완료 후에도 롤백하지 않습니다.
- `MemberRepository`는 구현체가 없는 인터페이스 상태임에도 정상적으로 DB에 데이터를 저장하거나 조회하는 기능이 잘 동작함을 알 수 있습니다.



## 공통 인터페이스 기능

### 순수 JPA 기반 Repository 예제

```java
@Repository
public class MemberJpaRepository {

    @PersistenceContext
    private EntityManager em;

    public Member save(Member member) {
        em.persist(member);
        return member;
    }

    public void delete(Member member) {
        em.remove(member);
    }

    public long count() {
        return em.createQuery("select count(m) from Member m", Long.class)
                .getSingleResult();
    }

    public Member find(Long id) {
        return em.find(Member.class, id);
    }

    public List<Member> findAll() {
        return em.createQuery("select m from Member m", Member.class)
                .getResultList();
    }

    public Optional<Member> findById(Long id) {
        Member member = em.find(Member.class, id);
        return Optional.ofNullable(member);
    }

}
```

```java
@Repository
public class TeamJpaRepository {

    @PersistenceContext
    private EntityManager em;

    public Team save(Team team) {
        em.persist(team);
        return team;
    }

    public void delete(Team team) {
        em.remove(team);
    }

    public long count() {
        return em.createQuery("select count(t) from Team m", Long.class)
                .getSingleResult();
    }

    public List<Team> findAll() {
        return em.createQuery("select t from Team t", Team.class)
                .getResultList();
    }

    public Optional<Team> findById(Long id) {
        Team team = em.find(Team.class, id);
        return Optional.ofNullable(team);
    }

}
```

- JPA에서의 수정은 변경 감지 기능을 사용하면 됩니다. 트랜잭션 안에서 엔티티를 조회한 다음에 데이터를 변경하면, 트랜잭션 종료 시점에 변경 감지 기능이 작동해서 변경된 엔티티를 감지하고 UPDATE SQL을 실행합니다.

위의 Repository를 테스트하는 코드를 작성해보겠습니다.

```java
@SpringBootTest
@Transactional
class MemberJpaRepositoryTest {

    @Autowired
    MemberJpaRepository memberJpaRepository;

    @Test
    public void testMember() {
        Member member = new Member("memberA");
        Member savedMember = memberJpaRepository.save(member);

        Member findMember = memberJpaRepository.find(savedMember.getId());

        assertThat(findMember.getId()).isEqualTo(member.getId());
        assertThat(findMember.getUsername()).isEqualTo(member.getUsername());
        assertThat(findMember).isEqualTo(member);
    }

    @Test
    public void basicCRUD() {
        Member member1 = new Member("member1");
        Member member2 = new Member("member2");

        memberJpaRepository.save(member1);
        memberJpaRepository.save(member2);

        Member findMember1 = memberJpaRepository.findById(member1.getId()).get();
        assertThat(findMember1).isEqualTo(member1);

        Member findMember2 = memberJpaRepository.findById(member2.getId()).get();
        assertThat(findMember2).isEqualTo(member2);

        List<Member> all = memberJpaRepository.findAll();
        assertThat(all.size()).isEqualTo(2);

        long count = memberJpaRepository.count();
        assertThat(count).isEqualTo(2);

        memberJpaRepository.delete(member1);
        memberJpaRepository.delete(member2);

        long deletedCount = memberJpaRepository.count();
        assertThat(deletedCount).isEqualTo(0);
    }

}
```

### 공통 인터페이스 설정

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
    List<Member> findByUsername(String username);
}
```

`org.springframework.data.repository.Repository`를 구현한 클래스는 스캔 대상으로, 스프링 데이터 JPA가 구현 클래스 대신 생성합니다. 즉, 구현 클래스는 프록시 기술로 스프링에서 만들어 줍니다. 더불어, `@Repository` 애노테이션 없어도 스프링 데이터 JPA가 컴포넌트 스캔을 자동으로 처리합니다.

![](./images/spring-data-jpa-common-interface.png)

- 제네릭 타입

  `T`: 엔터티 / `ID`: 엔터티의 식별자 타입 / `S`: 엔티티와 그 자식 타입

- 주요 메서드

  `save(S)`: 새로운 엔티티는 저장하고 이미 있는 엔티티는 병합합니다.

  `delete(T)`: 엔티티 하나를 삭제합니다. 내부에서는 `EntityManager.remove()`를 호출합니다.

  `getOne(ID)`: 엔티티를 프록시로 조회합니다. 내부에서는 `EntityManager.getReference()`를 호출합니다.

  `findAll(...)`: 모든 엔티티를 조회합니다. 정렬이나 페이징 조건을 파라미터로 제공할 수 있습니다.



## 쿼리 메서드 기능

### 메서드 이름으로 쿼리 생성

먼저 순수한 JPA Repository에서 일므과 나이를 기준으로 회원 조회하는 기능을 만들어 보겠습니다.

```java
@Repository
public class MemberJpaRepository {

    @PersistenceContext
    private EntityManager em;

    public List<Member> findByUsernameAndAgeGreaterThan(String username, int age) {
        return em.createQuery("select m from Member m where m.username = :username and m.age > :age", Member.class)
                .setParameter("username", username)
                .setParameter("age", age)
                .getResultList();
    }

}
```

```java
@SpringBootTest
@Transactional
class MemberJpaRepositoryTest {

    @Autowired
    MemberJpaRepository memberJpaRepository;

    @Test
    public void findByUsernameAndAgeGreaterThan() {
        Member m1 = new Member("AAA", 10);
        Member m2 = new Member("AAAA", 20);

        memberJpaRepository.save(m1);
        memberJpaRepository.save(m2);

        List<Member> result = memberJpaRepository.findByUsernameAndAgeGreaterThan("AAAA", 15);

        assertThat(result.get(0).getUsername()).isEqualTo("AAAA");
        assertThat(result.get(0).getAge()).isEqualTo(20);
        assertThat(result.size()).isEqualTo(1);
    }

}
```

스프링 데이터 JPA는 메서드 이름을 분석해서 JPQL 쿼리를 만들어 실행해 줍니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
    List<Member> findByUsernameAndAgeGreaterThan(String Username, int age);
}
```

```java
@SpringBootTest
@Transactional
@Rollback(false)
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;

    @Test
    public void findByUsernameAndAgeGreaterThan() {
        Member m1 = new Member("AAA", 10);
        Member m2 = new Member("AAAA", 20);

        memberRepository.save(m1);
        memberRepository.save(m2);

        List<Member> result = memberRepository.findByUsernameAndAgeGreaterThan("AAAA", 15);

        assertThat(result.get(0).getUsername()).isEqualTo("AAAA");
        assertThat(result.get(0).getAge()).isEqualTo(20);
        assertThat(result.size()).isEqualTo(1);
    }
    
}
```

쿼리 메서드 필터 조건은 [스프링 데이터 JPA 공식 문서](https://docs.spring.io/spring-data/jpa/docs/current/reference/html/#jpa.query-methods.query-creation)를 참고해서 작성하면 됩니다.

추가로 제공하는 쿼리 메서드 기능들이 있습니다. 이 기능들은 엔티티의 필드명을 참고해서 바인딩 하는 것이기 때문에, 필드명이 변경되면 인터ㅔ이스에 정의한 메서드 이름도 꼭 함께 변경해야 합니다. 제대로 매칭이 안되는 경우에는 애플리케이션 로딩 시점에 오류를 뱉어냅니다.

- 조회: `find...By`, `read...By`, `query...By`, `get...By` ([참고](https://docs.spring.io/spring-data/jpa/docs/current/reference/html/#repositories.query-methods.query-creation))

  `findHelloBy`처럼 `...`에 식별하기 위한 내용 및 설명이 들어가도 됩니다.

- COUNT: `count...By` 반환 타입 `long`

- EXISTS: `exists...By` 반환 타입 `boolean`

- 삭제: `delete...By`, `remove...By` 반환 타입 `long`

- DISTINCT: `findDistinct`, `findMemberDistinctBy`

- LIMIT: `findFirst3`, `findFirst`, `findTop`, `findTop3` ([참고](https://docs.spring.io/spring-data/jpa/docs/current/reference/html/#repositories.limit-query-result))

### JPA NamedQuery

```java
@Entity
@Getter @Setter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@ToString(of = {"id", "username", "age"})
@NamedQuery(
        name = "Member.findByUsername",
        query = "select m from Member m where m.username = :username"   // 문법 오류가 있으면 컴파일 에러를 뱉어냄
)
public class Member {

    @Id @GeneratedValue
    @Column(name = "member_id")
    private Long id;
    private String username;
    private int age;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "team_id")
    private Team team;

    /* ... */

}
```

- `@NamedQuery` 애노테이션으로 NamedQuery를 정의합니다.

  `query`에 적힌 문자열에서 오타가 있거나 문법이 맞지 않는 경우, 애플리케이션 로딩 시점에 오류를 뱉습니다.

먼저 순수 JPA를 직접 사용해서 NamedQuery를 호출해 보겠습니다.

```java
@Repository
public class MemberJpaRepository {

    @PersistenceContext
    private EntityManager em;

     public List<Member> findByUsername(String username) {
        return em.createNamedQuery("Member.findByUsername", Member.class)
                .setParameter("username", username)
                .getResultList();
    }

}
```

이어서 스프링 데이터 JPA로 NamedQuery를 사용해 보겠습니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
//    @Query(name = "Member.findByUsername")
    List<Member> findByUsername(@Param("username") String username);
}
```

- 기본적으로 `@Query` 애노테이션을 사용해서 NamedQuery를 지정해줘야 하지만, 이를 생략하고 메서드 이름만으로도 호출할 수 있습니다.

  선언한 '도메인 클래스 + .(점) + 메서드 이름'으로 NamedQuery를 찾아서 실행합니다. 만약 실행할 NamedQuery가 없으면 메서드 이름으로 쿼리 생성 전략을 사용합니다.

### Repository 메서드에 쿼리 정의

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
    @Query("select m from Member m where m.username = :username and m.age = :age")  // 오타가 나면 애플리케이션 로딩 시점에 오류를 뱉어냄
    List<Member> findMember(@Param("username") String username, @Param("age") int age);
}
```

위의 경우처럼 NamedQuery를 직접 등록해서 사용하는 일은 드뭅니다. 대신 `@Query`를 사용해서 Repository 메서드에 쿼리를 직접 정읩합니다.

### 특정 값 혹은 DTO 조회

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
    @Query("select m.username from Member m")
    List<String> findUsernameList();			// username을 조회

    @Query("select new study.datajpa.dto.MemberDto(m.id, m.username, t.name) from Member m join m.team t")
    List<MemberDto> findMemberDto();			// DTO 클래스 조회
}
```

- DTO로 직접 조회하려면 JPA의 `new` 명령어를 사용해야 합니다. 더불어 알맞는 DTO 생성자가 필요합니다.

### 파라미터 바인딩

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
    @Query("select m from Member m where m.username in :names")
    List<Member> findByNames(@Param("names") Collection<String> names);
}
```

JPQL에 `:names`처럼 필요한 파라미터를 바인딩할 때는 `@Param` 애노테이션을 사용합니다.

### 반환 타입

스프링 데이터 JPA는 유연한 반환 타입을 지원합니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {
    List<Member> findByUsernameAndAgeGreaterThan(String Username, int age);  // 컬렉션
    Member findMemberByUsername(String username);  // 단건
    Optional<Member> findOptionalByUsername(String username);  // 단건 Optional
}
```

컬렉션의 경우, 조회 결과가 없으면 `null`이 아닌 빈 컬렉션을 반환합니다. 그리고 단건 조회 메서드에서 조회 결과가 없으면 `null`을 반환하며, 결과가 여러 개인 경우 `NonUniqueResultException` 예외가 발생합니다.

### 순수 JPA의 페이징과 정렬

만약 아래의 조건으로 페이징과 정렬을 사용할 때, 순수한 JPA에서의 예제 코드를 살펴보겠습니다.

- 검색 조건: 나이 10살
- 정렬 조건: 이름으로 내림차순
- 페이징 조건: 첫 번째 페이지, 페이지당 보여 줄 데이터는 3건

```java
@Repository
public class MemberJpaRepository {

    @PersistenceContext
    private EntityManager em;

    public List<Member> findByPage(int age, int offset, int limit) {
        return em.createQuery("select m from Member m where m.age = :age order by m.username desc", Member.class)
                .setParameter("age", age)
                .setFirstResult(offset)
                .setMaxResults(limit)
                .getResultList();
    }

    public long totalCount(int age) {
        return em.createQuery("select count(m) from Member m where m.age = :age", Long.class)
                .setParameter("age", age)
                .getSingleResult();
    

}
```

```java
@SpringBootTest
@Transactional
class MemberJpaRepositoryTest {

    @Autowired
    MemberJpaRepository memberJpaRepository;

    @Test
    public void paging() {
        memberJpaRepository.save(new Member("member1", 10));
        memberJpaRepository.save(new Member("member2", 10));
        memberJpaRepository.save(new Member("member3", 10));
        memberJpaRepository.save(new Member("member4", 10));
        memberJpaRepository.save(new Member("member5", 10));

        int age = 10;
        int offset = 0;
        int limit = 3;

        // when
        List<Member> members = memberJpaRepository.findByPage(age, offset, limit);
        long totalCount = memberJpaRepository.totalCount(age);

        // then
        assertThat(members.size()).isEqualTo(3);
        assertThat(totalCount).isEqualTo(5);
    }

}
```

### 스프링 데이터 JPA 페이징과 정렬

스프링 데이터 JPA는 페이징과 정렬을 위해 파라미터로 `Pageable` 인터페이스를 받습니다. 해당 인터페이스는 기본적으로 페이징 기능 메서드를 갖고 있고, 더불어 내부에 `Sort` 인터페이스도 포함하고 있습니다.

이외에도 `Page`, `Slice`와 같이 특별한 반환 타입도 지원합니다. `Page`는 추가 count 쿼리 결과를 포함하는 페이징을 수행하고, `Slice`는 추가 count 쿼리 없이 다음 페이지만 확인 가능하도록 수행합니다. (내부적으로 `limit + 1`을 조회함으로써 동작) 그리고 반환 타입이 `List`라면 추가 count 쿼리 없이 결과만 반환합니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {

    Page<Member> findByAge(int age, Pageable pageable);
    
    Slice<Member> findSliceByAge(int age, Pageable pageable);

    @Query(
            value = "select m from Member m",
            countQuery = "select count(m.username) from Member m"
    )
    Page<Member> findMemberAllCountBy(Pageable pageable);

    List<Member> findTop3By();

}
```

- 두 번째 파라미터로 받은 `Pagable`은 인터페이스입니다. 따라서 실제 사용할 때는 구현체인 `PageRequest` 객체를 사용합니다.
- `findMemberAllCountBy`처럼 `count 쿼리를 분리해서 사용할 수도 있습니다.

```java
@SpringBootTest
@Transactional
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;

    @Test
    public void paging() {
        memberRepository.save(new Member("member1", 10));
        memberRepository.save(new Member("member2", 10));
        memberRepository.save(new Member("member3", 10));
        memberRepository.save(new Member("member4", 10));
        memberRepository.save(new Member("member5", 10));

        int age = 10;
        PageRequest pageRequest = PageRequest.of(0, 3, Sort.by(Sort.Direction.DESC, "username"));

        // when
        Page<Member> page = memberRepository.findByAge(age, pageRequest);
        List<Member> content = page.getContent();

        // then
        assertThat(content.size()).isEqualTo(3);
        assertThat(page.getTotalElements()).isEqualTo(5);
        assertThat(page.getNumber()).isEqualTo(0);
        assertThat(page.getTotalPages()).isEqualTo(2);
        assertThat(page.isFirst()).isTrue();
        assertThat(page.hasNext()).isTrue();
    }

    @Test
    public void slicing() {
        memberRepository.save(new Member("member1", 10));
        memberRepository.save(new Member("member2", 10));
        memberRepository.save(new Member("member3", 10));
        memberRepository.save(new Member("member4", 10));
        memberRepository.save(new Member("member5", 10));

        int age = 10;
        PageRequest pageRequest = PageRequest.of(0, 3, Sort.by(Sort.Direction.DESC, "username"));

        // when
        Slice<Member> slice = memberRepository.findSliceByAge(age, pageRequest);
        List<Member> content = slice.getContent();

        // then
        assertThat(content.size()).isEqualTo(3);
        assertThat(slice.getNumber()).isEqualTo(0);
        assertThat(slice.isFirst()).isTrue();
        assertThat(slice.hasNext()).isTrue();
    }

    @Test
    public void pagingDto() {
        memberRepository.save(new Member("member1", 10));
        memberRepository.save(new Member("member2", 10));
        memberRepository.save(new Member("member3", 10));
        memberRepository.save(new Member("member4", 10));
        memberRepository.save(new Member("member5", 10));

        int age = 10;
        PageRequest pageRequest = PageRequest.of(0, 3, Sort.by(Sort.Direction.DESC, "username"));

        // when
        Page<Member> page = memberRepository.findByAge(age, pageRequest);
        List<Member> content = page.getContent();

        Page<MemberDto> toMap = page.map(member -> new MemberDto(member.getId(), member.getUsername(), null));

        // then
        assertThat(content.size()).isEqualTo(3);
        assertThat(page.getTotalElements()).isEqualTo(5);
        assertThat(page.getNumber()).isEqualTo(0);
        assertThat(page.getTotalPages()).isEqualTo(2);
        assertThat(page.isFirst()).isTrue();
        assertThat(page.hasNext()).isTrue();
    }

}
```

- `PageRequest` 생성자의 첫 번째 파라미터에는 현재 페이지를, 두 번째 파라미터에는 조회할 데이터 수를 입력합니다. 여기에 추가로 정렬 정보도 파라미터로 사용할 수 있습니다. 참고로 페이지는 0부터 시작합니다.
- `pagingDto` 테스트의 경우, 반환 받은 페이징을 DTO로 변환하는 과정을 수행합니다. DB로 부터 객체를 가져와서 애플리케이션 뷰 단에 해당 객체를 그대로 전달하면 엔티티 변경 위험이 있으므로, 일반적으로 DTO로 변환 후 전달합니다. 이를 위해 `Page`의 `map` 메서드를 사용해서 구현합니다.

### 벌크성 수정 쿼리

벌크 쿼리는 DB상 데이터 중에 특정 조건이 맞는 데이터들 전부 수정하는 쿼리를 의미합니다.
먼저 순수 JPA를 사용한 벌크성 수정 쿼리를 살펴보겠습니다.

```java
@Repository
public class MemberJpaRepository {

    @PersistenceContext
    private EntityManager em;

    public int bulkAgePlus(int age) {
        return em.createQuery("update Member m set m.age = m.age + 1 where m.age >= :age")
                .setParameter("age", age)
                .executeUpdate();
    }

}
```

```java
@SpringBootTest
@Transactional
class MemberJpaRepositoryTest {

    @Autowired
    MemberJpaRepository memberJpaRepository;

    @Test
    public void bulkUpdate() {
        memberJpaRepository.save(new Member("member1", 10));
        memberJpaRepository.save(new Member("member2", 19));
        memberJpaRepository.save(new Member("member3", 20));
        memberJpaRepository.save(new Member("member4", 21));
        memberJpaRepository.save(new Member("member5", 40));

        // when
        int resultCount = memberJpaRepository.bulkAgePlus(20);

        // then
        assertThat(resultCount).isEqualTo(3);
    }

}
```

이번에는 스프링 데이터 JPA를 사용한 벌크성 수정 쿼리를 살펴보겠습니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {

    @Modifying(clearAutomatically = true)
    @Query("update Member m set m.age = m.age + 1 where m.age >= :age")
    int bulkAgePlus(@Param("age") int age);

}
```

- 벌크성 수정 및 삭제 쿼리는 `@Modifying` 애노테이션을 사용해야 합니다.

  더불어 `clearAutomatically` 값을 `true`로 설정하면 해당 쿼리 수행 후, 영속성 컨텍스트를 초기화해 줍니다. 이 옵션 없이 회원을 `findById`로 다시 조회하면 영속성 컨텍스트에 과거 값이 남아서 문제가 될 수 있습니다.

```java
@SpringBootTest
@Transactional
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;
    
    @Test
    public void bulkUpdate() {
        memberRepository.save(new Member("member1", 10));
        memberRepository.save(new Member("member2", 19));
        memberRepository.save(new Member("member3", 20));
        memberRepository.save(new Member("member4", 21));
        memberRepository.save(new Member("member5", 40));

        // when
        int resultCount = memberRepository.bulkAgePlus(20);

        Member member5 = memberRepository.findByUsername("member5").get(0);
        assertThat(member5.getAge()).isEqualTo(41);

        // then
        assertThat(resultCount).isEqualTo(3);
    }

}
```

### @EntityGraph

`@EntityGraph`는 Fetch Join처럼 연관된 엔티티들을 SQL 한번에 조회하는 방법입니다.
기본적으로 `member`=>`team`은 지연로딩 관계이어서, 실제 `team`의 데이터를 조회할 때마다 쿼리가 실행됩니다. 이는 결국 N+1 문제가 발생됩니다.

```java
@SpringBootTest
@Transactional
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;

    @Autowired
    TeamRepository teamRepository;

    @Autowired
    EntityManager em;

    @Test
    public void findMemberLazy() {
        // given
        Team teamA = new Team("teamA");
        Team teamB = new Team("teamB");
        teamRepository.save(teamA);
        teamRepository.save(teamB);

        Member member1 = new Member("member1", 10, teamA);
        Member member2 = new Member("member2", 15, teamB);
        memberRepository.save(member1);
        memberRepository.save(member2);

        em.flush();
        em.clear();

        // when N + 1 problem
        List<Member> members = memberRepository.findAll();
        for (Member member : members) {
            System.out.println("member = " + member.getUsername());
            System.out.println("member.getTeam() = " + member.getTeam());
        }
    }

}
```

따라서 연관된 엔티티를 한번에 조회하려면 Fetch Join이 필요합니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {

    @Query("select m from Member m left join fetch m.team")
    List<Member> findMemberFetchJoin();

}
```

스프링 데이터 JPA는 JPA가 제공하는 엔티티 그래프 기능을 편리하게 사용하도록 도와줍니다. 이 기능을 사용하면 JPQL 없이 Fetch Join을 할 수 있습니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long> {

    @Override
    @EntityGraph(attributePaths = {"team"})
    List<Member> findAll();

    @EntityGraph(attributePaths = {"team"})
    @Query("select m from Member m")
    List<Member> findMemberEntityGraph();

    @EntityGraph(attributePaths = {"team"})
    List<Member> findEntityGraphByUsername(@Param("username") String username);

}
```

- 사실상 Fetch Join의 간편 버전이라고 생각하면 됩니다.
- Join할 때는 Left Outer Join을 사용합니다.

```java
@SpringBootTest
@Transactional
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;

    @Autowired
    TeamRepository teamRepository;

    @Autowired
    EntityManager em;

    @Test
    public void findMemberEntityGraph() {
        // given
        Team teamA = new Team("teamA");
        Team teamB = new Team("teamB");
        teamRepository.save(teamA);
        teamRepository.save(teamB);

        Member member1 = new Member("member1", 10, teamA);
        Member member2 = new Member("member1", 10, teamB);
        Member member3 = new Member("member2", 15, teamB);
        memberRepository.save(member1);
        memberRepository.save(member2);
        memberRepository.save(member3);

        em.flush();
        em.clear();

        // when
        List<Member> members = memberRepository.findAll();
        for (Member member : members) {
            System.out.println("member = " + member.getUsername());
            System.out.println("member.getTeam() = " + member.getTeam());
        }

        List<Member> memberEntityGraph = memberRepository.findMemberEntityGraph();
        for (Member member : memberEntityGraph) {
            System.out.println("member = " + member);
        }

        List<Member> member1EntityGraph = memberRepository.findEntityGraphByUsername("member1");
        for (Member member : member1EntityGraph) {
            System.out.println("member = " + member);
        }
    }

}
```

`@NamedEntityGraph`도 지원하는데, 엔티티에 미리 명시를 하고 Repository에서 사용할 수 있습니다.

```java
@Entity
@Getter @Setter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@ToString(of = {"id", "username", "age"})
@NamedQuery(
        name = "Member.findByUsername",
        query = "select m from Member m where m.username = :username"
)
@NamedEntityGraph(
        name = "Member.all",
        attributeNodes = @NamedAttributeNode("team")
)
public class Member {
   /* ... */
}
```

```java
public interface MemberRepository extends JpaRepository<Member, Long> {

    @EntityGraph("Member.all")
    List<Member> findEntityGraphByUsername(@Param("username") String username);

}
```

### JPA Hint

JPA 쿼리 힌트를 사용해서 JPA 구현체에게 정보를 제공할 수 있습니다. (SQL 힌트가 아님)

```java
public interface MemberRepository extends JpaRepository<Member, Long> {

    @QueryHints(value = @QueryHint(name = "org.hibernate.readOnly", value = "true"))
    Member findReadOnlyByUsername(String username);

}
```

```java
@SpringBootTest
@Transactional
class MemberRepositoryTest {

    @Autowired
    MemberRepository memberRepository;

    @Autowired
    EntityManager em;

    @Test
    public void queryHint() {
        // given
        Member member1 = new Member("member1", 10);
        memberRepository.save(member1);

        em.flush();
        em.clear();

        // when
        Member findMember = memberRepository.findReadOnlyByUsername(member1.getUsername());
        findMember.setUsername("memberA");

        em.flush(); // update query 실행 x (snapshot 생성 x)
    }

}
```



## 확장 기능

### 사용자 정의 Repository 구현

스프링 데이터 JPA Repository는 인터페이스만 정의하고 구현체는 스프링이 자동 생성하는 방식입니다. 만약 이외의 메서드를 정의하려면, 스프링 데이터 JPA가 제공하는 인터페이스 내의 기능들을 모두 직접 구현해야 하는데, 너무 많아서 하기 힘듭니다. 이럴 경우에 사용자 정의 Repository를 구현해서 사용합니다.

```java
public interface MemberRepositoryCustom {
    List<Member> findMemberCustom();
}
```

```java
@RequiredArgsConstructor
public class MemberRepositoryCustomImpl implements MemberRepositoryCustom {

    private final EntityManager em;

    @Override
    public List<Member> findMemberCustom() {
        return em.createQuery("select m from Member m", Member.class)
                .getResultList();
    }

}
```

```java
public interface MemberRepository extends JpaRepository<Member, Long>, MemberRepositoryCustom {
    /* ... */
}
```

사용자 정의 Repository 인터페이스의 구현 클래스명은 `Repository 인터페이스명 + Impl` 혹은 `사용자 정의 Repository 인터페이스명 + Impl` 규칙을 지켜서 만들어야 합니다. 이를 스프링 데이터 JPA가 인식해서 빈으로 등록합니다.

> **참고**
>
> 항상 사용자 정의 Repository가 필요한 것은 아닙니다. 그냥 임의의 Repository를 만들어서 JPA와는 별개로 직접 `@Repository` 애노테이션을 붙여 사용해도 됩니다.

### Auditing

엔티티를 생성 및 변경할 때 변경한 시각과 사람을 추적하고 싶을 경우 사용합니다.

먼저 순수 JPA를 사용하는 경우의 코드를 살펴보겠습니다.

```java
@MappedSuperclass
@Getter
public class JpaBaseEntity {

    @Column(updatable = false)
    private LocalDateTime createdDate;
    private LocalDateTime updatedDate;

    @PrePersist
    public void prePersist() {
        LocalDateTime now = LocalDateTime.now();
        createdDate = now;
        updatedDate = now;
    }

    @PreUpdate
    public void preUpdate() {
        updatedDate = LocalDateTime.now();
    }

}
```

```java
public class Member extends JpaBaseEntity {
    /* ... */
}
```

JPA에서는 `@PrePersist`, `@PostPersist`, `@PreUpdate`, `@PostUpdate` 주요 이벤트 애노테이션을 지원합니다.

이번에는 스프링 데이터 JPA를 사용한 코드를 살펴보겠습니다.

```java
@EnableJpaAuditing
@SpringBootApplication
public class DataJpaApplication {
    @Bean
	public AuditorAware<String> auditorProvider() {
		return () -> Optional.of(UUID.randomUUID().toString());
	}
}
```

- 스프링 부트 설정 클래스에 `@EnableJpaAuditing`을 적용해야 합니다.

- 등록자 및 수정자를 처리해주는 `AuditorAware`을 스프링 빈으로 등록합니다.

  실무에서는 세션 정보나, 스프링 시큐리티 로그인 정보에서 ID를 받곤 합니다.

```java
@EntityListeners(AuditingEntityListener.class)
@MappedSuperclass
@Getter
public class BaseTimeEntity {

    @CreatedDate
    @Column(updatable = false)
    private LocalDateTime createdDate;

    @LastModifiedDate
    private LocalDateTime lastModifiedDate;

}
```

```java
@EntityListeners(AuditingEntityListener.class)
@MappedSuperclass
@Getter
public class BaseEntity extends BaseTimeEntity {

    @CreatedBy
    @Column(updatable = false)
    private String createdBy;

    @LastModifiedBy
    private String lastModifiedBy;

}
```

- 엔티티에 `@EntityListeners(AuditingEntityListener.class)`를 적용해야 합니다.
- `@CreatedDate`, `@CreatedBy`, `@LastModifiedDate`, ` @LastModifiedBy` 애노테이션을 지원합니다.

> **참고**
>
> 저장 시점에 등록일, 등록자는 물론이고 수정일, 수정자도 같은 데이터가 저장됩니다. 데이터가 중복 저장되는 것 같지만, 이렇게 해두면 보다 유지보수 관점에서 편리합니다. 이렇게 하지 않으면 `null`값이 들어갈 수 있기 때문에 쿼리 날릴 때 더 복잡해 질 수 있습니다.

### Web 확장 - 도메인 클래스 컨버터

```java
@RestController
@RequiredArgsConstructor
public class MemberController {

    private final MemberRepository memberRepository;

    @GetMapping("/members/{id}")
    public String findMember(@PathVariable("id") Long id) {
        Member member = memberRepository.findById(id).get();
        return member.getUsername();
    }

    @GetMapping("/members2/{id}")
    public String findMember2(@PathVariable("id") Member member) {
        return member.getUsername();
    }

    @PostConstruct
    public void init() {
        for (int i = 0; i < 30; i++)
            memberRepository.save(new Member("member" + i, i + 10));
    }

}
```

- 이전에는 `findMember()` 메서드와 같이, PK로 객체를 조회해서 사용했습니다.

- 도메인 클래스 컨버터를 사용하면 `findMember2()` 메서드처럼 스프링에서 중간 과정을 처리해서 엔티티 객체를 바로 받아올 수 있습니다.

  주의해야할 점은 받아온 객체는 트랜잭션이 없는 범위에서 조회한 것이므로, 단순 조회용으로만 사용해야 한다는 점입니다.

### Web 확장 - 페이징과 정렬

스프링 데이터가 제공하는 페이징과 정렬 기능을 스프링 MVC에서 편리하게 사용할 수 있습니다.

```java
@RestController
@RequiredArgsConstructor
public class MemberController {

    private final MemberRepository memberRepository;

    @GetMapping("/members")
    public Page<MemberDto> list(Pageable pageable) {
        return memberRepository.findAll(pageable)
                .map(member -> new MemberDto(member.getId(), member.getUsername(), null));
    }

    @PostConstruct
    public void init() {
        for (int i = 0; i < 30; i++)
            memberRepository.save(new Member("member" + i, i + 10));
    }

}
```

- 파라미터롤 `Pageable`을 받아서 사용합니다.

  `Pageable`은 인터페이스로, 실제 객체는 `PageRequest`가 생성됩니다.

- 요청 URL이 `/members?page=0&size=3&sort=id,desc&sort=username,desc`인 경우

  - page: 현재 페이지(0부터 시작)
  - size: 한 페이지에 노출할 데이터 건 수
  - sort: 정렬 조건

  스프링 부트의 기본값은 `application.yml`에서 수정할 수 있습니다.

  ```yaml
  spring:
    data:
      web:
        pageable:
          default-page-size: 10
          max-page-size: 2000
  ```

위처럼 URL로 설정 요청을 받지 않고, 개별 설정을 하려면 `@PageableDefault` 애노테이션을 사용합니다.

```java
@RestController
@RequiredArgsConstructor
public class MemberController {

    private final MemberRepository memberRepository;

    @GetMapping("/members")
    public Page<MemberDto> list(
            @PageableDefault(
                    size = 12, sort = {"username"}, direction = Sort.Direction.DESC
            ) Pageable pageable) {
        return memberRepository.findAll(pageable)
                .map(member -> new MemberDto(member.getId(), member.getUsername(), null));
    }

}
```

이전에 언급한 것처럼, 엔티티 객체를 뷰 단에 그대로 전달하면 안되므로 DTO로 변환해서 전달을 해줘야 합니다. 위의 코드보다 간단히 하려면 아래 코드처럼 약간 수정해주면 됩니다.

```java
@Data
public class MemberDto {

    private Long id;
    private String username;
    private String teamName;

    public MemberDto(Long id, String username, String teamName) {
        this.id = id;
        this.username = username;
        this.teamName = teamName;
    }

    public MemberDto(Member member) {
        this.id = member.getId();
        this.username = member.getUsername();
        this.teamName = null;
    }

}
```

```java
@RestController
@RequiredArgsConstructor
public class MemberController {

    private final MemberRepository memberRepository;

    @GetMapping("/members")
    public Page<MemberDto> list(
            @PageableDefault(
                    size = 12, sort = {"username"}, direction = Sort.Direction.DESC
            ) Pageable pageable) {
        return memberRepository.findAll(pageable)
                .map(MemberDto::new);
    }

}
```



## 스프링 데이터 JPA 분석

### 스프링 데이터 JPA 구현체 분석

공통 인터페이스의 구현체는 `org.springframework.data.jpa.repository.support.SimpleJpaRepository`입니다.

```java
@Repository
@Transactional(readOnly = true)
public class SimpleJpaRepository<T, ID> implements JpaRepositoryImplementation<T, ID> {
    
    /* ... */
    
    @Transactional
    public <S extends T> S save(S entity) {
        if (entityInformation.isNew(entity)) {
            em.persist(entity);
            return entity;
        } else {
            return em.merge(entity);
        }
    }
    
}
```

- `@Repository` 적용해서 컴포넌트 스캔 대상으로 등록하고, JPA 예외를 스프링이 추상화한 예외로 전환합니다.

- `@Transactional` 적용

  기본적으로 JPA의 모든 변경은 트랜잭션 안에서 동작하게 됩니다.

  - 단순 조회 용 기능의 경우, `readonly = true` 속성이 적용되어 영속성 컨텍스트 플러시를 생략함으로써 약간의 성능 향상을 얻을 수 있습니다.
  - 변경(등록, 수정, 삭제) 기능의 경우는 트랜잭션 처리합니다.
  - 서비스 계층에서 트랜잭션을 시작하지 않은 상태라면, Repository에서 트랜잭션을 시작합니다. (시작한 상태라면, 해당 트랜잭션을 전파 받아서 사용)

- `save()` 메서드

  기본 동작 로직은 새로운 엔티티면 저장(`persist`)을 하고, 그렇지 아니면 병합(`merge`)를 합니다.

### 새로운 엔티티를 구별하는 방법

Repository에서 `isNew()` 메서드처럼 새로운 엔티티인지 확인할 때 사용하는 기본 전략은 다음과 같습니다,

- 식별자가 객체인 경우, `null`을 기준으로 판단합니다.
- 식별자가 자바 primitive 타입인 경우, `0`을 기준으로 판단합니다.

JPA 식별자 생성 전략이 `@GeneratedValue`면 `save()` 호출 시점에 식별자가 없으므로 새로운 엔티티로 인식해서 정상 동작합니다. 그런데 JPA 식별자 생성 전략이 `@Id`만 사용해서 직접 할당하는 경우라면 이미 식별자 값을 받은 상태로 `save()`를 호출하게 됩니다. 따라서 이 경우에는 `merge()`가 호출되는데, `merge()`는 우선 DB에 `select` 쿼리를 날려서 값을 확인하고, DB에 값이 없으면 새로운 엔티티로 인지하므로 비효율 적입니다.

위와 같은 경우를 피하기 위해, `Persistable` 인터페이스를 구현해서 판단 로직을 변경할 수 있습니다. 먼저 `Persistable` 인터페이스는 아래와 같은 구조입니다.

```java
package org.springframework.data.domain;

public interface Persistable<ID> {
    ID getId();
    boolean isNew();
}
```

```java
@Entity
@EntityListeners(AuditingEntityListener.class)
@Getter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class Item implements Persistable<String> {

    @Id
    private String id;

    @CreatedDate
    private LocalDateTime createdDate;

    public Item(String id) {
        this.id = id;
    }

    @Override
    public boolean isNew() {
        return createdDate == null;
    }

}
```

- `Item` 엔티티의 경우 식별자 생성 전략이 따로 없으므로, 식별자를 가진 상태에서 `save()`가 호출됩니다.

- `Persistable` 인터페이스의 `getId()`와 `isNew()` 메서드를 오버라이딩해서 새로운 엔티티 판단 기준을 수정할 수 있습니다.

  위의 경우, 오로지 `id` 필드만으로 새로운 엔티티인지 아닌지 구분이 어려우므로, `@CreatedDate`를 사용해서, 해당 힐드 값이 없으면 JPA 상에서 아직 생성한 것이 아니므로 새로운 엔티티 임을 간접적으로 판단할 수 있습니다.



## 이외의 기능들

### Specifications (명세)

스프링 데이터 JPA는 JPA Criteria를 활용해서 DDD(Domain Drive Design)에서 소개하는 SPECIFICATION 개념을 사용하도록 지원합니다. 술어(predicate)로는, 참 또는 거짓으로 평가하며 `AND`, `OR` 같은 연산자로 조합해서 다양한 검색 조건을 쉽게 생성합니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long>, MemberRepositoryCustom, JpaSpecificationExecutor<Member> {
    /* ... */
}
```

- `JpaSpecificationExecutor` 인터페이스를 상속해서 사용합니다.
- 해당 인터페이스 내 메서드들은 `Specification`을 파라미터로 받아서 검색 조건으로 사용합니다.

```java
public class MemberSpec {

    public static Specification<Member> teamName(final String teamName) {
        return (Specification<Member>) (root, query, builder) -> {
            if (StringUtils.isEmpty(teamName)) {
                return null;
            }
            Join<Member, Team> t = root.join("team", JoinType.INNER);

            return builder.equal(t.get("name"), teamName);
        };
    }

    public static Specification<Member> username(final String username) {
        return (Specification<Member>) (root, query, builder) ->
            builder.equal(root.get("username"), username);
    }

}
```

- 명세를 정의하기 위해 `Specification` 인터페이스를 구현합니다.
- `toPredicate()` 메서드만 구현하면 되는데, 이때 JPA Criteria의 `Root`, `CriteriaQuery`, `CriteriaBuilder` 클래스를 파라미터로 넘겨 줍니다. (예제 코드에서는 편의 상 람다를 사용)

```java
@SpringBootTest
@Transactional
class MemberRepositroyTest {
    
    @Autowired
    MemberRepository memberRepository;
    
    @Autowired
    EntityManager em;
    
    @Test
    public void specBasic() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        Specification<Member> spec = MemberSpec.username("member1").and(MemberSpec.teamName("teamA"));
        List<Member> result = memberRepository.findAll(spec);

        assertThat(result.size()).isEqualTo(1);
    }
    
}
```

- 구현한 `Specification`을 사용하면 명세들을 `where()`, `and()`, `or()`, `not()` 등으로 조립하여 사용할 수 있습니다.
- 예제에서 사용한 `findAll()`은 회원 이름 명세(`username`)와 팀 이름 명세(`teamName`)를 `and`로 조합해서 검색 조건으로 사용하였습니다.

그렇지만 실무에서는 JPA Criteria를 거의 쓰지 않습니다. 이를 대신해 QueryDSL을 사용하는 편이 좋습니다.

### Query By Example

```java
public interface MemberRepository extends JpaRepository<Member, Long>, MemberRepositoryCustom, JpaSpecificationExecutor<Member> {
    /* ... */
}
```

- `JpaRepository` 인터페이스는 `QueryByExample` 인터페이스를 상속 받기 때문에, 별도의 인터페이스를 설정하지 않아도 사용할 수 있습니다.

```java
@SpringBootTest
@Transactional
class MemberRepositroyTest {
    
    @Autowired
    MemberRepository memberRepository;
    
    @Autowired
    EntityManager em;
    
    @Test
    public void queryByExample() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        // Probe 생성
        Member member = new Member("member1");
        Team team = new Team("teamA");
        member.setTeam(team);

        ExampleMatcher matcher = ExampleMatcher.matching()
                .withIgnorePaths("age");

        Example<Member> example = Example.of(member, matcher);

        List<Member> result = memberRepository.findAll(example);

        assertThat(result.get(0).getUsername()).isEqualTo("member1");
    }
    
}
```

- Probe: 필드에 데이터가 있는 실제 도메인 객체를 뜻합니다.
- ExampleMatcher: 특정 필드를 일치시키는 상세한 정보를 제공합니다. (재사용 가능)
- Example: Probe와 ExampleMatcher로 구성되며, 쿼리를 생성하는데 사용됩니다.

**장점**

- 동적 쿼리를 편리하기 처리 가능합니다.
- 도메인 객체를 그대로 사용할 수 있습니다.
- 데이터 저장소를 RDB에서 NoSQL로 변경해도 코드 변경 없도록 추상화 되어 있습니다.

**단점**

- 내부 조인(INNER JOIN)만 가능하고, 외부 조인(LEFT JOIN)은 불가능 합니다.
- 중첩 제약 조건을 사용할 수 없습니다.
- 매칭 조건이 매우 단순합니다.

`QueryByExample`도 실무에서 사용하기에는 매칭 조건이 너무 단순하고, LEFT JOIN이 안됩니다. 이 대신에 QueryDSL을 사용하는 편이 좋습니다.

### Projections

엔티티 대신에 DTO를 편리하게 조회할 때 사용합니다.

```java
public interface UsernameOnly {
//    @Value("#{target.username + ' ' + target.age}")
    String getUsername();
}
```

- 조회할 엔티티의 필드를 `getter` 형식으로 지정하면 해당 필드만 선택해서 조회합니다. (Projection)
- 위처럼 특정 필드만 가져오는 것을 **Closed Projections**라고 합니다.
- 주석처리된 애노테이션을 풀면, 스프링의 SpEL 문법을 사용하게 됩니다. 이 경우 DB에서 엔티티 필드를 다 조회해온 다음에 계산하여 JPQL SELECT절 최적화가 안됩니다. 이를 **Open Projections**라고 합니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long>, MemberRepositoryCustom, JpaSpecificationExecutor<Member> {
    
    List<UsernameOnly> findClosedProjectionsByUsername(@Param("username") String username);

    List<UsernameOnlyDto> findClassProjectionsByUsername(@Param("username") String username);
    
}
```

- 반환 타입으로 인지하기 때문에, 메서드 이름은 자유롭게 정해도 됩니다.

```java
@SpringBootTest
@Transactional
class MemberRepositroyTest {
    
    @Autowired
    MemberRepository memberRepository;
    
    @Autowired
    EntityManager em;
    
    @Test
    public void closedProjections() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        List<UsernameOnly> result = memberRepository.findClosedProjectionsByUsername("member1");
        for (UsernameOnly usernameOnly : result) {
            System.out.println("usernameOnly = " + usernameOnly);
        }
    }
    
}
```

- 콘솔 로그의 날라간 SQL문을 보면

  ```sql
  select m.username from member m where m.username='member1'
  ```

  으로 `username`만 조회(Projection)하는 것을 확인할 수 있습니다.

더불어, 쿼리 형식은 똑같고 반환 타입만 달라지는 경우 제네릭을 사용해서 확장할 수 있습니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long>, MemberRepositoryCustom, JpaSpecificationExecutor<Member> {
    
    <T> List<T> findProjectionsByUsername(@Param("username") String username, Class<T> type);
    
}
```

인터페이스가 아닌 구체적인 DTO 형식으로 조회(Projection)하는 것도 가능합니다.

```java
@Getter
public class UsernameOnlyDto {

    private final String username;

    public UsernameOnlyDto(String username) {
        this.username = username;
    }

}
```

- 생성자의 파라미터 이름으로 매칭합니다.

```java
@SpringBootTest
@Transactional
class MemberRepositroyTest {
    
    @Autowired
    MemberRepository memberRepository;
    
    @Autowired
    EntityManager em;
    
    @Test
    public void classProjections() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        List<UsernameOnlyDto> result = memberRepository.findClassProjectionsByUsername("member1");
        for (UsernameOnlyDto usernameOnlyDto : result) {
            System.out.println("usernameOnlyDto = " + usernameOnlyDto.getUsername());
        }
    }
    
}
```

추가로 중첩 구조 처리하는 예제 코드를 살펴보겠습니다.

```java
public interface NestedClosedProjections {

    String getUsername();
    TeamInfo getTeam();

    interface TeamInfo {
        String getName();
    }

}
```

```java
@SpringBootTest
@Transactional
class MemberRepositroyTest {
    
    @Autowired
    MemberRepository memberRepository;
    
    @Autowired
    EntityManager em;
    
    @Test
    public void projections() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        List<NestedClosedProjections> result = memberRepository.findProjectionsByUsername("member1", NestedClosedProjections.class);
        for (NestedClosedProjections nestedClosedProjections : result) {
            String username = nestedClosedProjections.getUsername();
            System.out.println("username = " + username);
            String name = nestedClosedProjections.getTeam().getName();
            System.out.println("name = " + name);
        }
    }
    
}
```

- 콘솔 로그이 찍힌 SQL문을 보면 SELECT 절이 root 엔티티만 최적화된 것을 확인할 수 있습니다. (`member`의 `username` 가져오는 것은 최적화가 되어있으나, `team` 엔티티는 모두 가져온 후 계산)
- 즉 조회(Projection) 대상이 root 엔티티면, JPQL SELECT 절 최적화가 가능합니다.
- 그러나 대상이 root 엔티티가 아니면 LEFT OUTER JOIN로 모든 필드를 SELECT해서 엔티티로 가져온 다음에 계산합니다.

따라서 Projection은 대상이 root 엔티티면 유용합니다. 그러나 이는 실무의 복잡한 쿼리를 해결하기에는 한계가 있습니다. 따라서 복잡해지면 QueryDSL을 사용하는 편이 좋습니다.

### Native Query

가급적 네이티브 쿼리는 사용하지 않는 편이 좋기 때문에, 참고 용으로 예제 코드만 작성하겠습니다.

```java
public interface MemberRepository extends JpaRepository<Member, Long>, MemberRepositoryCustom, JpaSpecificationExecutor<Member> {
    
    @Query(value = "select * from member where username = ?", nativeQuery = true)
    Member findByNativeQuery(String username);

    @Query(value = "select m.member_id as id, m.username, t.name as teamName " +
            "from member m left join team t",
            countQuery = "select count(*) from member",
            nativeQuery = true)
    Page<MemberProjection> findByNativeProjection(Pageable pageable);
    
}
```

```java
@SpringBootTest
@Transactional
class MemberRepositroyTest {
    
    @Autowired
    MemberRepository memberRepository;
    
    @Autowired
    EntityManager em;
    
    @Test
    public void nativeQuery() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        Member result = memberRepository.findByNativeQuery("member1");
        System.out.println("result = " + result);
    }

    @Test
    public void nativeQueryProjection() {
        Team teamA = new Team("teamA");
        em.persist(teamA);

        Member member1 = new Member("member1", 0, teamA);
        Member member2 = new Member("member2", 0, teamA);
        em.persist(member1);
        em.persist(member2);

        em.flush();
        em.clear();

        Page<MemberProjection> result = memberRepository.findByNativeProjection(PageRequest.of(0, 10));
        List<MemberProjection> content = result.getContent();
        for (MemberProjection memberProjection : content) {
            System.out.println("memberProjection = " + memberProjection.getUsername());
            System.out.println("memberProjection = " + memberProjection.getTeamName());
        }
    }
    
}
```

