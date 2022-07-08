---
title: JUnit5 살펴보기
date: 2021-03-15
pin: false
tags:
- Java
- TDD
---

# TDD를 위한 JUnit 5 살펴보기

## JUnit 5

JUnit은 Java용 Unit Test Framework이다. TDD(Test-Driven Development) 면에서 중요하며 `XUnit`이라는 이름의 Unit Test Framework 계열의 하나이다.

여기서 `XUnit`이란, 각각 프로그래밍 언어에서도 단위 테스트를 위한 프레임워크가 존재하며 대부분 이름을 `xUnit`이라 칭한다. Python의 Unit Test Framework는 `PyUnit`, C++의 Unit Test Framework는 `CppUnit`인 것 처럼 자바의 Unit Test Framwork가 JUnit인 것이다.

JUnit 5는 이전 버전과 다르게 세 가지 하위 프로젝트의 여러 모듈로 구성되어 있다.

![](./images/junit5_architecture.png)

- **JUnit Platform**

  테스트를 발견하고 테스트 계획을 생성하는 `TestEngine Interface`를 가지고 있다. 해당 `TestEngine`을 통해서 테스트를 발견하고, 실행하여 결과를 보고한다.

- **JUnit Jupiter**

  `TestEngine`의 실제 구현체는 별도의 모듈들인데, 그 모듈 중 하나가 `jupiter-engine`이다. 이 모듈은 `jupiter-api`를 사용해서 작성한 테스트 코드를 발견하고 실행한다. `Jupiter API`는 `JUnit 5`에 새롭게 추가된 테스트 코드용 API로서, 이를 사용해서 테스트 코드를 작성할 수 있다.

- **JUnit Vintage**

  이전 `JUnit 4` 버전으로 작성한 테스트 코드를 실행할 때에는 `vintage-engine` 모듈을 사용한다.



## Assertion Method

JUnit Jupiter Assertion은 모두 `static` 메서드이다.

### assertEquals

```java
@Test
void equalsAssertions() {
    assertEquals(2, 1+1);
}
```

### assertSame

```java
@Test
void sameAssertions() {
    assertSame("abc", "abc");
}
```

### assertNotNull

```java
@Test
void nullCheckAssertions() {
    String input = "a";
    assertNotNull(input);
}
```

### assertThorws

```java
@Test
void exceptionAssertions() {
    String input = null;
    assertThrows(NullPointerException.class, () -> {
        int length = input.length();
        System.out.println("length = " + length);
    });
}
```

### assertTimeout

```java
@Test
void timeoutAssertions() {
    assertTimeout(Duration.ofMillis(100),
            () -> Thread.sleep(50)
    );
}
```

### assertAll

```java
@Test
void groupedAssertions() {
    assertAll(
            () -> assertEquals(2, 1+1),
            () -> assertEquals(2, 4/2)
    );
}
```



## Life Cycle Annotations

### @BeforeAll

이전 버전의 `@BeforeCalss`와 동일하게, 현재 클래스의 모든 테스트 메서드보다 먼저 실행한다. 해당 메서드는 당연하게도 `static`이어야 한다.

### @BeforeEach

이전 버전의 `@Before`와 동일하게, 해당 메서드를 각 테스트 메서드 전에 실행한다.

### @AfterAll

이전 버전의 `@AfterClass`와 동일하게, 현재 클래스의 모든 테스트 메서드 이후에 실행한다. 적용 메서드는 `static`이어야 한다.

### @AfterEach

이전 버전의 `@After`와 동일하게, 각 테스트 메서드 실행 이후에 실행한다.

```java
public class LifeCycleTest {

    @RepeatedTest(3)
    void doSomething() {
        System.out.println("hi");
    }

    @BeforeAll
    static void beforeStart() {
        System.out.println("before start");
    }

    @BeforeEach
    void init() {
        System.out.println("init");
    }

    @AfterAll
    static void afterStart() {
        System.out.println("after start");
    }

    @AfterEach
    void done() {
        System.out.println("done");
    }

}
```



## Base Annotations

### @DisplayName

테스트 클래스 또는 메서드의 이름을 정의한다.

```java
@DisplayName("Test Class for DisplayName")
public class DisplayNameTest {

    @Test
    @DisplayName("Test Method for DisplayName")
    void doSomething() {
        System.out.println("hi");
    }

}
```

### @Tag

테스트 필터링을 위한 태그를 선언한다. 태그는 `null`일 수 없고, 공백과 `&`, `(`, `!` 등과 같은 예약 문자를 포함할 수 없다.

```java
@Tags({
    @Tag("fast"),
    @Tag("model")
})
public class TagTest {

    @Test
    @Tag("exact")
    void doSomething() {
        System.out.println("hi");
    }

}
```

### @Disabled

이전 버전의 `@Ignore`과 동일하게, 테스트 클래스 또는 메서드를 비활성화할 수 있다.

```java
public class IgnoreTest {

    @Test
    @Disabled("Not implemented yet")
    void notYet() {
        String input = null;
        int length = input.length();
    }

}
```

### @Timeout

실행에 주어진 시간을 초과하는 경우 테스트, 테스트 팩토리, 테스트 템플릿 또는 생명 주기 방법이 실패하도록 설정할 수 있다.

```java
public class TimeoutTest {

    @Test
    @Timeout(value = 500, unit = TimeUnit.MILLISECONDS)
    void timeoutTest() throws InterruptedException {
        delay(400);
    }


    void delay(int millisecond) throws InterruptedException {
        TimeUnit.MILLISECONDS.sleep(millisecond);
    }

}
```

### @ExtendWith

사용자 정의 확장명을 동록하는데 사용한다. 단위 테스트 간에 공통적으로 사용할 기능을 구현하여 해당 애노테이션으로 적용할 수 있다.

```java
@Target({ ElementType.TYPE, ElementType.METHOD })
@Retention(RetentionPolicy.RUNTIME)
@Documented
@ExtendWith(DisabledOnOsCondition.class)
@API(status = STABLE, since = "5.1")
public @interface DisabledOnOs {
    /* ... */
}
```



## Test Class Annotations

### @Nested

해당 주석이 달린 클래스는 정적이 아닌 중첩 테스트 클래스임을 나타낸다. 주석을 달지 않으면 해당 클래스 내의 테스트 코드들은 실행되지 않지만, `@Nested`를 붙이면 정상적으로 실행된다.

```java
public class NestedTest {
    
    @Test
    void outerTest() {
        System.out.println("Outer Class Test");
    }

    @Nested
    class InnerClass {

        @Test
        void innerTest() {
            System.out.println("Inner Class Test");
        }

    }

}
```

### @TestInstance

테스트 인스턴스의 생명 주기를 설정할 때 사용한다.

- `PER_METHOD`: 테스트 메서드당 인스턴스가 생성된다.
- `PER_CLASS`: 테스트 클래스당 인스턴스가 생성된다.

클래스 단위 생명 주기를 가지는 클래스는 테스트 실행 중 단 하나의 인스턴스만을 생성한다. 그러므로 `@BeforeAll`이나 `@AfterAll` 적용 메서드가 `static`일 필요가 없고, `@Nested` 적용 클래스에서 생명 주기 애노테이션을 사용할 수 있게 된다.

```java
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
public class InstanceTest {

    @Test
    void doSomeThing() {
        System.out.println("hi");
    }

    @BeforeAll
    void init() {
        System.out.println("TestInstance: before");
    }

    @AfterAll
    void done() {
        System.out.println("TestInstance: after");
    }

}
```



## Test Method Annotations

### @Test

해당 메서드는 테스트 대상 메서드임을 의미한다.

### @ParameterizedTest

`@ValueSource`와 같이 사용되며, 해당 메서드를 여러 개의 파라미터에 대해서 테스트할 수 있다.

```java
@ParameterizedTest
@ValueSource(ints = {1, 3, 5, -3, 15, Integer.MAX_VALUE})
void isOdd(int num) {
    Assertions.assertTrue(num % 2 != 0);
}
```

### @ValueSource

해당 애노테이션에 지정한 배열을 파라미터 값으로 순서대로 넘겨준다. 테스트 메서드 당 하나의 파라미터만을 전달할 때 사용할 수 있다. 전달할 수 있는 값은 리터럴 값의 배열이다.

**literal value 종류**: `short`, `byte`, `int`, `long`, `float`, `double`, `char`, `java.lang.String`, `java.lang.Class`

```java
@ParameterizedTest
@ValueSource(strings = {"AAA", "ABC", "AVXZE", "EFSCZ_EFDFA"})
void isHavingA(String input) {
    assertTrue(input.contains("A"));
}
```

### @RepeatedTest

동일 테스트를 반복할 때 사용한다.

```java
@RepeatedTest(4)
void repeated() {
    String input = null;

    assertThrows(NullPointerException.class, () -> {
        int length = input.length();
        System.out.println("length = " + length);
    });
}
```

### @TestFactory

동적으로 테스트를 작성할 수 있게 도와준다. `@ParameterizedTest`와 유사하지만, 보다 유연하게 테스트를 작성할 수 있다. (사실 잘 쓰일지는 의문이다,,,)

```java
public class FactoryTest {

    @TestFactory
    Collection<DynamicTest> dynamicTestsFromCollection() {
        return Arrays.asList(
                dynamicTest("1st dynamic test", () -> assertTrue(true)),
                dynamicTest("2nd dynamic test", () -> assertEquals(4, 2 * 2))
        );
    }

    @TestFactory
    Stream<DynamicTest> generateRandomNumberOfTests() {

        Iterator<Integer> inputGenerator = new Iterator<Integer>() {

            final Random random = new Random();
            int current;

            @Override
            public boolean hasNext() {
                current = random.nextInt(100);
                return current % 7 != 0;
            }

            @Override
            public Integer next() {
                return current;
            }
        };

        Function<Integer, String> displayNameGenerator = (input) -> "input:" + input;

        ThrowingConsumer<Integer> testExecutor = (input) -> assertTrue(input % 7 != 0);

        return DynamicTest.stream(inputGenerator, displayNameGenerator, testExecutor);
    }

    @TestFactory
    Stream<DynamicTest> test() {
        class TestTemplate {
            final String name;
            final int age;

            public TestTemplate(String name, int age) {
                this.name = name;
                this.age = age;
            }
        }

        return Stream.of(
                new TestTemplate("Seung", 19),
                new TestTemplate("Ho", 20)
        ).map(e -> dynamicTest("test" + e.name, () -> {
            assertTrue(e.age > 18, e.name + "'s age");
        }));
    }

}
```

### @TestTemplate

여러 번 호출되도록 설계된 테스트 케이스의 템플릿임을 나타낸다. (이 애노테이션도 잘 쓰일지 의문이다,,,)

```java
public class TemplateTest {

    @TestTemplate
    @ExtendWith(MyTestTemplateInvocationContextProvider.class)
    void templateTest(String parameter) {
        assertEquals(3, parameter.length());
    }

    public static class MyTestTemplateInvocationContextProvider implements TestTemplateInvocationContextProvider {
        @Override
        public boolean supportsTestTemplate(ExtensionContext context) {
            return true;
        }

        @Override
        public Stream<TestTemplateInvocationContext> provideTestTemplateInvocationContexts(ExtensionContext context) {
            return Stream.of(invocationContext("foo"), invocationContext("bar"));
        }

        private TestTemplateInvocationContext invocationContext(String parameter) {
            return new TestTemplateInvocationContext() {
                @Override
                public String getDisplayName(int invocationIndex) {
                    return parameter;
                }

                @Override
                public List<Extension> getAdditionalExtensions() {
                    return Collections.singletonList(new ParameterResolver() {
                        @Override
                        public boolean supportsParameter(ParameterContext parameterContext, ExtensionContext extensionContext) throws ParameterResolutionException {
                            return parameterContext.getParameter().getType().equals(String.class);
                        }

                        @Override
                        public Object resolveParameter(ParameterContext parameterContext, ExtensionContext extensionContext) throws ParameterResolutionException {
                            return parameter;
                        }
                    });
                }
            };
        }
    }

}
```

### @TestMethodOrder

테스트의 순서를 지정할 수 있는 기능이다. 일반적으로 테스트는 순서에 의존하지 않도록 작성해야 한다. 그럼에도 로직 흐름을 순서대로 테스트할 경우가 있을 수 있으므로 간혹 사용한다.

```java
@TestMethodOrder(MethodOrderer.OrderAnnotation.class)
public class MethodOrderTest {
    
    @Test
    @Order(1)
    void test01() {
        System.out.println("test01");
    }

    @Test
    @Order(4)
    void test04() {
        System.out.println("test04");
    }

    @Test
    @Order(2)
    void test02() {
        System.out.println("test02");
    }

    @Test
    @Order(3)
    void test03() {
        System.out.println("test03");
    }

}
```

그리고 `@TestMethodOrder`는 알파벳 순서 이외에도 애노테이션으로 직접 순서를 지정하거나 랜덤한 순서로 실행하는 기능을 제공한다.



## 커스텀 Annotation

JUnit의 Jupiter Annotation은 Meta Annotation으로도 사용할 수 있다. 즉, 기존의 Annotation을 상속하는 사용자 정의 커스텀 Annotation을 정의할 수 있다.

만약 코드 베이스 전체에 `@Tag("fast")` Annotation이 필요한 경우, 이를 대신할 수 있는 커스텀 Annotation `@Fast`를 아래와 같이 정의해서 사용할 수 있다.

```java
@Target({ElementType.METHOD, ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Tag("fast")
public @interface Fast {
}
```

- `@Target`: 해당 애노테이션을 사용할 수 있는 대상의 종류를 지정
- `@Retention`: 해당 애노테이션이 컴파일된 클래스 파일에 저장되는지 여부와 런타임시 표시되는지 여부를 지정

```java
@Test
@Fast
void myFast() {
    System.out.println("fast!");
}
```


