---
title: "TDD: By Example 1부"
date: 2021-04-01
pin: false
tags:
- TDD
- Java
---

# TDD 1부: 화폐 예제

1부에서는 테스트 주도 개발의 리듬을 볼 수 있도록 전형적인 모델 코드를 개발한다. 그 리듬은 다음과 같이 요약할 수 있다.

- 재빨리 테스트를 하나 추가한다.
- 모든 테스트를 실행하고 새로 추가한 것이 실패하는지 확인한다.
- 코드를 조금 바꾼다.
- 모든 테스트를 실행하고 전부 성공하는지 확인한다.
- 리팩토링을 통해 중복을 제거한다.



## 다중 통화를 지원하는 Money 객체

| 종목 |  주  | 가격 | 합계  |
| :--: | :--: | :--: | :---: |
| IBM  | 1000 |  25  | 2500  |
|  GE  | 400  | 100  | 40000 |
|      |      | 합계 | 65000 |

만약 위와 같은 보고서가 있을 때 다중 통화를 지원하는 보고서를 만들려면 통화 단위와 환율을 추가해야 한다.

|   종목   |  주  |  가격   |   합계    |
| :------: | :--: | :-----: | :-------: |
|   IBM    | 1000 | 25 USD  | 2500 USD  |
| Novartis | 400  | 100 CHF | 40000 CHF |
|          |      |  합계   | 65000 USD |

| 기준 | 변환 | 환율 |
| :--: | :--: | :--: |
| CHF  | USD  | 1.5  |

이를 구현하기 위해 할일 목록을 꾸준히 업데이트하면서 진행하자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [ ] 5 USD * 2 = 10 USD

첫 번째 작업보다는 두 번째 것이 수월해 보이므로 이를 위한 테스트 코드부터 작성하자.

```java
public void testMultiplication() {
    Dollar five = new Dollar(5);
    five.times(2);
    assertEquals(10, five.amount);
}
```

이렇게 코드를 짜고 나면 빨간 줄 (컴파일 에러) 투성이다. 그리고 객체 자체에도 문제가 있다. 바로 할일 목록에 추가하자.



## 타락한 객체

> - [ ] 5 USD + 10 CHF = 10 USD
> - [ ] 5 USD * 2 = 10 USD
> - [ ] amount를 private로 만들기
> - [ ] Dollar의 side effect?
> - [ ] Money 반올림?

그리고 이제 Dollar 객체와 메서드를 만들어서 테스트를 통과하게 하자.

```java
public class Dollar {

    public int amount;

    public Dollar(int amount) {
        this.amount = amount;
    }

    public void times(int multiplier) {
        this.amount *= multiplier;
    }
}
```

```java
@Test
public void testMultiplication() {
    Dollar five = new Dollar(5);
    five.times(2);
    assertEquals(10, five.amount);
}
```

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [ ] amount를 private로 만들기
> - [ ] Dollar의 side effect?
> - [ ] Money 반올림?

이제 첫 번째 테스트에 완료 표시를 할 수 있게 됐다. 일반적으로 TDD 주기는 아래처럼 진행된다.

1. 테스트를 작성한다. 마음 속에 있는 오퍼레이션이 코드에 어떤 식으로 나타나길 원하는지 생각해보자. 원하는 인터페이스를 개발하라.
2. 실행 가능하게 만든다. 다른 무엇보다도 중요한 것은 빨리 초록 막대를 보는 것이다. 깔끔하고 단순한 해법이 보인다면 그것을 입력하고, 구현하는데 몇 분 정도 걸릴 것 같으면 할일 목록에 추가해두고 원래 문제로 돌아와 스텁 구현으로 통과시키자.
3. 올바르게 만든다. 이제 시스템이 동작하므로 이전에 저질렀던 죄악들을 수습하자. 우리 목적은 **작동하는 깔끔한 코드를 얻는 것**이다.

위 사항을 상기하면서, Dollar 부작용을 처리해보자. 지금 상황에서는 Dollar에 대한 연산을 수행하면 해당 객체의 값이 바뀐다. 이를 바꿔보자. (항상 테스트 코드를 먼저 작성한다.)

```java
@Test
public void testMultiplication() {
    Dollar five = new Dollar(5);
    Dollar product = five.times(2);
    assertEquals(10, product.amount);
    product = five.times(3);
    assertEquals(15, product.amount);
}
```

```java
public class Dollar {

    private int amount;

    public Dollar(int amount) {
        this.amount = amount;
    }

    public Dollar times(int multiplier) {
        return new Dollar(amount * multiplier);
    }

}
```

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [ ] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?

느낌 (부작용에 대한 혐오감) 을 테스트 (하나의 Dollar 객체에 곱하기를 두 번 수행하는 것) 로 변환하는 것은 TDD의 일반적인 주제다. 이런 작업을 오래 할수록 미적 판단을 테스트로 담아내는 것에 점점 익숙해지게 된다.



## 모두를 위한 평등

어떤 정수에 1을 더했을 때, 우리는 원래 정수가 변할 거라고 예상하기보다는 원래 정수에 1이 더해진 새로운 값을 갖게 될 것을 예상한다. 하지만 일반적으로 객체는 우리 예상대로 작동하지 않는다. 어떤 계약에 새로운 보상 항목을 추가하면 그 계약 자체가 변하게 되는 것이다. 이런 형태를 값 객체 패턴이라고 한다. 값 객체에 대한 제약 사항 중 하나는 객체의 인스턴스 변수가 생성자를 통해서 일단 설정된 후에는 결코 변하지 않는다는 것이다.

값 객체가 암시하는 것 중 하나는 이전에 했듯이, 모든 연산에 새 객체를 반환해야 한다는 것이다. 또 다른 암시는 값 객체는 equals()를 구현해야 한다는 것인데, $5 라는 것은 항상 다른 $5 만큼이나 똑같이 좋은 것이기 때문이다.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [ ] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [ ] equals()
> - [ ] hashCode()

```java
@Test
public void testEquality() {
    assertEquals(new Dollar(5), new Dollar(5));
    assertNotEquals(new Dollar(5), new Dollar(6));
}
```

```java
@Getter
public class Dollar {
    // ...
    @Override
    public boolean equals(Object obj) {
        Dollar dollar = (Dollar) obj;
        return this.amount == dollar.amount;
    }
}
```

동일성 문제를 부분적으로 해결할 수 있게 됐다. 아직 null 값이나 객체 자체에 대한 처리는 하지 않았으므로 목록에 추가해주자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [ ] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object



## 프라이버시

개념적으로 Dollar.time() 연산은 호출을 받은 객체의 값에 인자로 받은 곱수만큼 곱한 값을 갖는 Dollar를 반환해야 하지만 테스트가 정확히 그것을 말하지 않으므로 수정하자.

```java
@Test
public void testMultiplication() {
    Dollar five = new Dollar(5);
    assertEquals(new Dollar(10), five.times(2));
    assertEquals(new Dollar(15), five.times(3));
}
```

이 테스트는 일련의 오퍼레이션이 아니라 참인 명제에 대한 단언들이므로 우리의 의도를 더 명확하게 이야기해준다. 테스트를 고치고 나니 이제 Dollar의 amount 인스턴스 변수를 사용하는 코드는 Dollar 자신 뿐이다. 따라서 변수를 private로 변경할 수 있다.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object



## 솔직히 말하자면 (Franc-ly Speaking)

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [ ] 5 CHF * 2 = 10 CHF

다음으로 Dollar 객체와 비슷하지만 달러 대신 프랑(Franc)을 표현할 수 있는 객체를 구현하기 위한 테스트 코드를 작성하자.

```java
@Test
public void testFrancMultiplication() {
    Franc five = new Franc(5);
    assertEquals(new Franc(10), five.times(2));
    assertEquals(new Franc(15), five.times(3));
}
```

처음 Dollar를 구현해낸 것처럼, 지금 당장은 컴파일 에러가 뜨므로 필요한 클래스와 메서드를 만들어주자.

```java
public class Franc {

    private int amount;

    public Franc(int amount) {
        this.amount = amount;
    }

    public Franc times(int multiplier) {
        return new Franc(amount * multiplier);
    }

    @Override
    public boolean equals(Object obj) {
        Franc franc = (Franc) obj;
        return this.amount == franc.amount;
    }

}
```

다시 한 번 TDD 과정을 상기하고 가자.

1. 테스트 작성
2. 컴파일되게 하기
3. 실패하는지 확인하기 위해 실행
4. 실행하게 만듦
5. 중복 제거

각 단계에는 서로 다른 목적이 있다. 처음 네 단계는 빨리 진행해야 한다. 주의해야 할 점은 마지막 단계 없이는 앞의 네 단계도 제대로 되지 않는다. 적절한 시기에 적절한 설계를 돌아가게 만들고, 올바르게 만들어라.

Franc 객체를 만들어보고 나니 Dollar 객체와 많은 부분이 중복임을 알 수 있다. 바로 할일 목록에 추가하자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [ ] Dollar/Franc 중복
> - [ ] 공용 equals
> - [ ] 공용 times



## 돌아온 '모두를 위한 평등'

이제 테스트를 빨리 통과하기 위해 위에서 저질러 놓은 중복을 청소할 시간이다. Dollar 클래스와 Franc 클래스의 공통 상위 클래스 Money를 만들어 중복되는 부분을 없앨 생각이다. 그리고 Money 클래스가 공통의 equals 코드를 갖게하면 더욱 중복을 제거할 수 있을 것 같다.

먼저 위 과정에서 빠뜨려버린 Franc 객체 비교 테스트 코드를 추가 작성하자.

```java
@Test
public void testEquality() {
    assertEquals(new Dollar(5), new Dollar(5));
    assertNotEquals(new Dollar(5), new Dollar(6));
    assertEquals(new Franc(5), new Franc(5));
    assertNotEquals(new Franc(5), new Franc(6));
}
```

그리고 Dollar와 Franc 클래스의 중복되는 부분을 Money 클래스로 옮기자.

```java
public class Money {

    protected int amount;

    @Override
    public boolean equals(Object obj) {
        Money money = (Money) obj;
        return this.amount == money.amount;
    }

}
```

```java
public class Dollar extends Money {

    public Dollar(int amount) {
        this.amount = amount;
    }

    public Dollar times(int multiplier) {
        return new Dollar(amount * multiplier);
    }

}
```

```java
public class Franc extends Money {

    public Franc(int amount) {
        this.amount = amount;
    }

    public Franc times(int multiplier) {
        return new Franc(amount * multiplier);
    }

}
```

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [ ] Dollar/Franc 중복
> - [x] 공용 equals
> - [ ] 공용 times
> - [ ] Dollar와 Franc 비교하기

이어서 Dollar와 Franc를 비교하는 걸 해보자.



## 사과와 오렌지 (* "You can`t compare apples and oranges.")

먼저 기대하는 대로 테스트 코드를 작성해보자.

```java
@Test
public void testEquality() {
    assertEquals(new Dollar(5), new Dollar(5));
    assertNotEquals(new Dollar(5), new Dollar(6));
    assertEquals(new Franc(5), new Franc(5));
    assertNotEquals(new Franc(5), new Franc(6));
    assertNotEquals(new Dollar(5), new Franc(5));
}
```

마지막 assert에 걸려서 테스트가 실패한다. 현재 테스트 결과는 Dollar와 Franc가 같다고 얘기하고 있다. Money의 equals 코드에서 Dollar와 Franc 클래스를 비교함으로써 한 번 거를 수 있다.

```java
public class Money {
    // ...
    @Override
    public boolean equals(Object obj) {
        Money money = (Money) obj;
        return this.amount == money.amount
                && this.getClass().equals(money.getClass());
    }

}
```

물론 클래스 자체를 비교하기 보다는 구현 객체의 통화를 비교하면 좋지만, 현재로써는 바로 구현하기 어려우므로 목록에 추가해두자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [ ] Dollar/Franc 중복
> - [x] 공용 equals
> - [ ] 공용 times
> - [x] Dollar와 Franc 비교하기
> - [ ] 통화?

이제는 공통 times 코드를 처리할 차례다.



## 객체 만들기

현재 Dollar와 Franc 클래스 내의 time() 코드는 사실상 별 다른 게 없다. 먼저 반환 타입을 Money로 바꾸자.

```java
public class Dollar extends Money {
    // ...
    public Money times(int multiplier) {
        return new Dollar(amount * multiplier);
    }

}
```

```java
public class Franc extends Money {
    // ...
    public Money times(int multiplier) {
        return new Franc(amount * multiplier);
    }

}
```

두 times 코드의 중복을 바로 제거할 수 있지만, TDD의 단계를 확실히 밟아가기 위해 먼저 하위 클래스에 대한 직접적인 참조를 제거해보자. 기존 테스트 코드 수정부터 하자.

```java
@Test
public void testMultiplication() {
    Money five = Money.dollar(5);
    assertEquals(new Dollar(10), five.times(2));
    assertEquals(new Dollar(15), five.times(3));
}
```

이렇게 바꾸고 나면 Money 클래스에 dollar 메서드와 times 메서드가 없음을 알려준다. 먼저 Dollar 객체를 새로 만들어 반환해주는 팩토리 메서드부터 만들어보자. Franc 객체에 대한 팩토리 메서드도 간단히 예상 가능하므로 만들자.

```java
public class Money {
    // ...
    public static Dollar dollar(int amount) {
        return new Dollar(amount);
    }
    
    public static Franc franc(int amount) {
        return new Franc(amount);
    }

}
```

그리고 times 메서드는 Money 클래스를 추상 클래스로 변경한 후 선언해두자. 더불어 팩토리 메서드의 선언을 바꿀 수 있다.

```java
public abstract class Money {

    protected int amount;

    public abstract Money times(int multiplier);

    public static Money dollar(int amount) {
        return new Dollar(amount);
    }
    
    public static Money franc(int amount) {
        return new Franc(amount);
    }

    @Override
    public boolean equals(Object obj) {
        Money money = (Money) obj;
        return this.amount == money.amount
                && this.getClass().equals(money.getClass());
    }

}
```

이제 기존의 테스트 코드에 팩토리 메서드를 적용하자.

```java
public class StudyTDD {

    @Test
    public void testMultiplication() {
        Money five = Money.dollar(5);
        assertEquals(Money.dollar(10), five.times(2));
        assertEquals(Money.dollar(15), five.times(3));
    }

    @Test
    public void testEquality() {
        assertEquals(Money.dollar(5), Money.dollar(5));
        assertNotEquals(Money.dollar(5), Money.dollar(6));
        assertEquals(Money.franc(5), Money.franc(5));
        assertNotEquals(Money.franc(5), Money.franc(6));
        assertNotEquals(Money.dollar(5), Money.franc(5));
    }

    @Test
    public void testFrancMultiplication() {
        Franc five = new Franc(5);
        assertEquals(new Franc(10), five.times(2));
        assertEquals(new Franc(15), five.times(3));
    }

}
```

그리고 테스트 코드 중복 제거를 위해, Franc를 위한 별도의 times 테스트는 testMultiplication에서도 충분히 검증하는지 확인해야 함을 목록에 추가하자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [ ] Dollar/Franc 중복
> - [x] 공용 equals
> - [ ] 공용 times
> - [x] Dollar와 Franc 비교하기
> - [ ] 통화?
> - [ ] testFrancMultiplication을 지워야 할까?



## 우리가 사는 시간

통화 개념을 구현하기 위해 테스트 코드를 작성해보자. 이를 위해 복잡한 객체들이 필요할 수도 있지만 당분간은 그런 것들 대신 문자열을 쓰도록 하자.

```java
@Test
public void testCurrency() {
    assertEquals("USD", Money.dollar(1).currency());
    assertEquals("USD", Money.franc(1).currency());
}
```

이어서 테스트 코드 실행을 위해 currency 부분을 구현하자. 결과적으로 두 클래스를 모두 포함할 수 있는 동일한 구현을 만들고자 한다. 통화를 인스턴스 변수에 저장하고, 메서드에서는 그냥 그걸 반환하게 만드는 편이 나을 것 같다.

```java
public abstract class Money {
    // ...
    public abstract String currency();

}
```

```java
public class Dollar extends Money {
    // ...
    private String currency;

    @Override
    public String currency() {
        return currency;
    }

}
```

```java
public class Franc extends Money {
    // ...
    private String currency;

    @Override
    public String currency() {
        return currency;
    }

}
```

이제 두 currency 메서드가 동일하므로 변수 선언과 구현을 모두 Money 클래스로 올릴 수 있다. 그리고 통화 문자열을 정적 팩토리 메서드로 옮긴다면 두 생성자가 동일해져서 공통 구현을 만들 수 있게 된다. 그에 맞춰 생성자를 호출하는 부분들을 모두 수정해주자.

```java
public abstract class Money {

    protected int amount;
    protected String currency;

    public String currency() {
        return currency;
    };

    public abstract Money times(int multiplier);

    public static Money dollar(int amount) {
        return new Dollar(amount, "USD");
    }

    public static Money franc(int amount) {
        return new Franc(amount, "CHF");
    }

    @Override
    public boolean equals(Object obj) {
        Money money = (Money) obj;
        return this.amount == money.amount
                && this.getClass().equals(money.getClass());
    }

}
```

```java
public class Dollar extends Money {

    public Dollar(int amount, String currency) {
        this.amount = amount;
        this.currency = currency;
    }

    public Money times(int multiplier) {
        return Money.dollar(amount * multiplier);
    }

}
```

```java
public class Franc extends Money {

    public Franc(int amount, String currency) {
        this.amount = amount;
        this.currency = currency;
    }

    public Money times(int multiplier) {
        return Money.franc(amount * multiplier);
    }

}
```

이제 하위 클래스의 두 생성자가 동일해졌다. 구현을 상위 클래스로 올리자.

```java
public abstract class Money {
    // ...
    public Money(int amount, String currency) {
        this.amount = amount;
        this.currency = currency;
    }

}
```

```java
public class Dollar extends Money {
    // ...
    public Dollar(int amount, String currency) {
        super(amount, currency);
    }

}
```

```java
public class Franc extends Money {
    // ...
    public Franc(int amount, String currency) {
        super(amount, currency);
    }

}
```

이제 times 메서드를 상위 클래스로 올리고 하위 클래스들을 제거할 준비가 거의 다 됐다.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [ ] Dollar/Franc 중복
> - [x] 공용 equals
> - [ ] 공용 times
> - [x] Dollar와 Franc 비교하기
> - [x] 통화?
> - [ ] testFrancMultiplication 제거



## 흥미로운 시간

이번 장에서는 Money를 나타내기 위한 단 하나의 클래스만을 갖도록 바꿔나갈 것이다. Dollar와 Franc 클래스 내의 times 메서드를 동일한 구조로 만들기 위해 팩토리 메서드를 인라인시켜보자.

```java
public class Dollar extends Money {
    // ...
    public Money times(int multiplier) {
        return new Money(amount * multiplier, currency);
    }

}
```

```java
public class Franc extends Money {
    // ...
    public Money times(int multiplier) {
        return new Money(amount * multiplier, currency);
    }

}
```

이렇게 하려면 Money 클래스가 콘크리트 클래스이어야 하므로 수정해주자. 이때 추상 메서드였던 times() 는 쉽게 구현체를 생각해낼 수 있으므로 바로 구현을 해놓자.

```java
public class Money {
    // ...
    public Money times(int multiplier) {
        return new Money(amount * multiplier, currency);
    }

}
```

현재 equals 메서드가 클래스 자체를 비교하고 있으므로 Dollar(10, "USD")와 Money(10, "USD")가 서로 다르다고 판단하고 있다. 이를 고치기 위해 테스트 코드를 작성하고, 클래스 자체 비교를 통화 문자열 비교로 바꾸어주자.

```java
@Test
public void testDifferentClassEquality() {
    assertEquals(new Money(10, "USD"), new Dollar(10, "USD"));
}
```

```java
public class Money {
    // ...
    @Override
    public boolean equals(Object obj) {
        Money money = (Money) obj;
        return this.amount == money.amount
                && this.currency().equals(money.currency());
    }

}
```

이제 곱하기도 구현했으니 아무 역할도 없는 하위 클래스들을 제거할 수 있게 되었다.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [ ] Dollar/Franc 중복
> - [x] 공용 equals
> - [x] 공용 times
> - [x] Dollar와 Franc 비교하기
> - [x] 통화?
> - [ ] testFrancMultiplication 제거



## 모든 악의 근원

현재 두 하위 클래스 Dollar와 Franc에는 달랑 생성자 밖에 없는 상태다. 단지 생성자 때문에 하위 클래스가 있을 필요는 없기 때문에 하위 클래스를 제거해보자. 물론, 하위 클래스의 생성자를 호출하는 부분도 고쳐야 한다.

```java
public class Money {
    // ...
    public static Money dollar(int amount) {
        return new Money(amount, "USD");
    }

    public static Money franc(int amount) {
        return new Money(amount, "CHF");
    }

}
```

그리고 testFrancMultiplication() 테스트는 testMultiplication() 테스트로 대체 가능하므로 제거하자. 더불어 testDifferentClassEquality()에서 하는 테스트는 사실 testEquality() 테스트에서 충분히, 실은 과하게 검증하고 있다. 앞의 테스트는 지우고 뒤 테스트는 수정해주자.

```java
@Test
public void testEquality() {
    assertEquals(Money.dollar(5), Money.dollar(5));
    assertNotEquals(Money.dollar(5), Money.dollar(6));
    assertNotEquals(Money.dollar(5), Money.franc(5));
}
```

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD * 2 = 10 USD
> - [x] amount를 private로 만들기
> - [x] Dollar의 side effect?
> - [ ] Money 반올림?
> - [x] equals()
> - [ ] hashCode()
> - [ ] Equal null
> - [ ] Equal object
> - [x] 5 CHF * 2 = 10 CHF
> - [x] Dollar/Franc 중복
> - [x] 공용 equals
> - [x] 공용 times
> - [x] Dollar와 Franc 비교하기
> - [x] 통화?
> - [x] testFrancMultiplication 제거



## 드디어, 더하기

이제 클래스가 Money 하나뿐이다. 덧셈을 다룰 준비가 됐다. 할일 목록이 조금 지저분하니 새 목록으로 옮겨 적어보자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [ ] 5 USD + 5 USD = 10 USD

바로 환율을 고려한 덧셈으로 직행하기 보다 단계를 하나하나 밟아나가자. 먼저 동일 화폐에 대한 덧셈을 위한 테스트 코드를 작성해보자.

```java
@Test
public void testSimpleAddition() {
    Money five = Money.dollar(5);
    Expression sum = five.plus(five);
    Bank bank = new Bank();
    Money reduced = bank.reduce(sum, "USD");
    assertEquals(Money.dollar(10), reduced);
}
```

Expression은 연산의 결과를 표현하는 객체다. 위 테스트의 경우 plus() 메서드에 의한 결과를 갖게 된다. 그리고 연산 중 화폐가 다른 경우가 있을 수 있으므로 reduce() 메서드를 통해 동일 화폐로 축약시킨다. 보통 축약 작업을 은행에서 담당하므로 Bank 객체도 만들어주었다.

먼저 Expression을 클래스보다는 더 가벼운 인터페이스로 구현하자.

```java
public interface Expression {
}
```

그리고 plus() 메서드는 Expression을 반환하도록 구현하자. 더불어 Money를 Expression의 구현체로 설정하자.

```java
public class Money implements Expression {
    // ...
    public Expression plus(Money addend) {
        return new Money(amount + addend.amount, currency);
    }

}
```

이번에는 Bank와 reduce() 메서드를 만들자. 세부 로직까지 구현은 힘드므로 스텁 구현으로 진행하자.

```java
public class Bank {

    public Money reduce(Expression source, String to) {
        return Money.dollar(10);
    }

}
```



## 진짜로 만들기

> - [ ] 5 USD + 10 CHF = 10 USD
> - [ ] 5 USD + 5 USD = 10 USD

우선 Money.plus()는 그냥 Money가 아닌 Expression (Sum) 을 반환해야 한다. 이를 위한 테스트 코드를 작성해보자.

```java
@Test
public void testPlusReturnsSum() {
    Money five = Money.dollar(5);
    Expression result = five.plus(five);
    Sum sum = (Sum) result;
    assertEquals(five, sum.augend);
    assertEquals(five, sum.addend);
}
```

이어서 Sum을 구현해보자.

```java
public class Sum {
    Money augend;
    Money addend;
}
```

현재 Money.plus()는 Sum이 아닌 Money를 반환하게 되어 있기 때문에, ClassCastException이 발생한다. 이를 위해 plus() 메서드를 수정하고 필요한 부분들을 추가하자.

```java
public class Money implements Expression {
    // ...
    public Expression plus(Money addend) {
        return new Sum(this, addend);
    }

}
```

```java
public class Sum implements Expression {

    Money augend;
    Money addend;

    public Sum(Money augend, Money addend) {
        this.augend = augend;
        this.addend = addend;
    }
    
}
```

이제 Bank.reduce()는 Sum을 전달 받는다. 만약 Sum이 가지고 있는 Money의 통화가 모두 동일하고, reduce를 통해 얻어내고자 하는 Money의 통화 역시 같다면, 결과는 Sum 내에 있는 Money들의 amount를 합친 값을 갖는 Money 객체여야 한다. 이를 테스트 코드로 짜보자.

```java
@Test
public void testReduceSum() {
    Expression sum = new Sum(Money.dollar(3), Money.dollar(4));
    Bank bank = new Bank();
    Money result = bank.reduce(sum, "USD");
    assertEquals(Money.dollar(7), result);
}
```

그리고 reduce()를 알맞게 구현해보자.

```java
public class Bank {

    public Money reduce(Expression source, String to) {
        Sum sum = (Sum) source;
        int amount = sum.augend.amount + sum.addend.amount;
        return new Money(amount, to);
    }

}
```

하지만 이 코드는 다음 두 가지 이유로 지저분하다.

- 캐스팅 (형변환). 이 코드는 모든 Expression에 대해 작동해야 한다.
- 공용 (public) 필드와 그 필드들에 대한 두 단계에 걸친 레퍼런스.

우선, 외부에서 접근 가능한 필드 몇 개를 들어내기 위해 메서드 본문을 Sum으로 옮길 수 있다.

```java
public class Bank {

    public Money reduce(Expression source, String to) {
        Sum sum = (Sum) source;
        return sum.reduce(to);
    }

}
```

```java
public class Sum implements Expression {
    // ...
    public Money reduce(String to) {
        int amount = augend.amount + addend.amount;
        return new Money(amount, to);
    }
    
}
```

그리고 Bank.reduce()의 인자로 Money를 넘겼을 경우를 어떻게 테스트할 것인지 상기시키기 위해 목록을 하나 추가하자.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [ ] 5 USD + 5 USD = 10 USD
> - [ ] Bank.reduce(Money)

```java
@Test
public void testReduceMoney() {
    Bank bank = new Bank();
    Money result = bank.reduce(Money.dollar(1), "USD");
    assertEquals(Money.dollar(1), result);
}
```

```java
public class Bank {

    public Money reduce(Expression source, String to) {
        if (source instanceof Money)
            return (Money) source;
        Sum sum = (Sum) source;
        return sum.reduce(to);
    }

}
```

클래스를 명시적으로 검사하는 코드가 있을 때에는 항상 다형성을 사용하도록 바꾸는 것이 좋다. Sum은 reduce(String)을 구현하므로, Money도 그것을 구현하도록 만든다면 reduce()를 Expression 인터페이스에도 추가할 수 있게 된다.

```java
public class Money implements Expression {
    // ...
    public Money reduce(String to) {
        return this;
    }

}
```

```java
public interface Expression {
    
    Money reduce(String to);
    
}
```

```java
public class Bank {

    public Money reduce(Expression source, String to) {
        return source.reduce(to);
    }

}
```

이렇게 함으로써 지저분한 캐스팅과 클래스 검사 코드를 제거할 수 있다.

> - [ ] 5 USD + 10 CHF = 10 USD
> - [ ] 5 USD + 5 USD = 10 USD
> - [x] Bank.reduce(Money)
> - [ ] Money에 대한 통화 변환을 수행하는 Reduce
> - [ ] Reduce(Bank, String)



## 바꾸기

이어서 통화를 실제로 전환하는 기능을 구현하기 위한 테스트 코드를 작성해보자.

```java
@Test
public void testReduceMoneyDifferentCurrency() {
    Bank bank = new Bank();
    bank.addRate("CHF", "USD", 2);
    Money result = bank.reduce(Money.franc(2), "USD");
    assertEquals(Money.dollar(1), result);
}
```

프랑과 달러의 환율이 2:1 이라고 가정할 때, 나누기 2을 하면 된다. (수치 상의 모든 귀찮은 문제를 외면..)

```java
public class Money implements Expression {
    // ...
    public Money reduce(String to) {
        int rate = (currency.equals("CHF") && to.equals("USD")) ? 2 : 1;
        return new Money(amount / rate, to);
    }

}
```

위 코드로 인해서 갑자기 Money가 환율에 대해 알게 돼 버렸다. 환율에 대한 모든 일은 Bank가 처리해야 한다. 따라서 Expression.reduce()의 인자로 Bank를 넘겨야 한다.

```java
public class Bank {

    public Money reduce(Expression source, String to) {
        return source.reduce(this, to);
    }

}
```

```java
public interface Expression {

    Money reduce(Bank bank, String to);

}
```

```java
public class Sum implements Expression {
    // ...
    public Money reduce(Bank bank, String to) {
        int amount = augend.amount + addend.amount;
        return new Money(amount, to);
    }

}
```

```java
public class Money implements Expression {
    // ...
    public Money reduce(Bank bank, String to) {
        int rate = (currency.equals("CHF") && to.equals("USD")) ? 2 : 1;
        return new Money(amount / rate, to);
    }

}
```

이제 환율을 Bank에서 계산할 수 있게 되었다.

```java
public class Bank {
    // ...
    int rate(String from, String to) {
        return (from.equals("CHF") && to.equals("USD"))
                ? 2
                : 1;
    }

}
```

```java
public class Money implements Expression {
    // ...
    public Money reduce(Bank bank, String to) {
        int rate = bank.rate(currency, to);
        return new Money(amount / rate, to);
    }

}
```

그리고 테스트와 코드에서 환율에 해당하는 숫자 2가 모두 나오고 있다. 이걸 없애려면 Bank에서 환율표를 가지고 있다가 필요할 때 찾아볼 수 있게 해야한다. 이를 위한 객체를 따로 만들자. 더불어 이 객체를 테이블의 키로 쓸 것이므로 equals()와 hashCode()를 구현해야 한다. (빠르게 진행할 수 있게 간단한 구현으로 하자.)

```java
public class Pair {

    private String from;
    private String to;

    public Pair(String from, String to) {
        this.from = from;
        this.to = to;
    }

    @Override
    public int hashCode() {
        return 0;
    }

    @Override
    public boolean equals(Object obj) {
        Pair pair = (Pair) obj;
        return from.equals(pair.from) && to.equals(pair.to);
    }
    
}
```

이제 Bank에 환율을 저장할 수 있도록 테이블을 추가하고 테이블에서 환율을 가져올 수 있도록 수정하자.

```java
public class Bank {

    private Hashtable<Pair, Integer> rates = new Hashtable();
    
    void addRate(String from, String to, int rate) {
        rates.put(new Pair(from, to), rate);
    }

    int rate(String from, String to) {
        if (from.equals(to))
            return 1;
        return rates.get(new Pair(from, to));
    }

    public Money reduce(Expression source, String to) {
        return source.reduce(this, to);
    }

}
```

추가로 같은 화폐일 때도 환율 보장이 되도록 테스트 코드를 작성하자.

```java
@Test
public void testIdentityRate() {
    assertEquals(1, new Bank().rate("USD", "USD"));
}
```

> - [ ] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD + 5 USD = 10 USD
> - [x] Bank.reduce(Money)
> - [x] Money에 대한 통화 변환을 수행하는 Reduce
> - [x] Reduce(Bank, String)



## 서로 다른 통화 더하기

이번에는 가장 큰 테스트인 '5 USD + 10 CHF'를 구현할 것이다.

```java
@Test
public void testMixedAddition() {
    Expression fiveDollars = Money.dollar(5);
    Expression tenFrancs = Money.franc(10);
    Bank bank = new Bank();
    bank.addRate("CHF", "USD", 2);
    Money result = bank.reduce(fiveDollars.plus(tenFrancs), "USD");
    assertEquals(Money.dollar(10), result);
}
```

먼저 plus() 메서드를 인터페이스에 추가하고 구현체도 작성하자.

```java
public interface Expression {
    // ...
    Expression plus(Expression addend);
    
}
```

```java
public class Money implements Expression {
    // ...
    public Expression plus(Expression addend) {
        return new Sum(this, addend);
    }

}
```

```java
public class Sum implements Expression {
    // ...
    @Override
    public Expression plus(Expression addend) {
        return null;
    }

}
```

이제 테스트 코드 실행은 되지만 실패한다. 실질적인 reduce 동작을 안하는 것 같으므로 수정해주자.

```java
public class Sum implements Expression {

    Expression augend;
    Expression addend;

    public Sum(Expression augend, Expression addend) {
        this.augend = augend;
        this.addend = addend;
    }

    @Override
    public Expression plus(Expression addend) {
        return null;
    }

    public Money reduce(Bank bank, String to) {
        int amount = augend.reduce(bank, to).amount + addend.reduce(bank, to).amount;
        return new Money(amount, to);
    }

}
```

```java
public class Money implements Expression {
   // ...
    public Expression plus(Expression addend) {
        return new Sum(this, addend);
    }

    public Expression times(int multiplier) {
        return new Money(amount * multiplier, currency);
    }

}
```

이제 모든 테스트가 통과한다. 다만 스텁 구현이 아직 남아있으므로 할일 목록에 추가하고 넘어가자.

> - [x] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD + 5 USD = 10 USD
> - [x] Bank.reduce(Money)
> - [x] Money에 대한 통화 변환을 수행하는 Reduce
> - [x] Reduce(Bank, String)
> - [ ] Sum.plus
> - [ ] Expression.times



## 드디어, 추상화

먼저 Sum.plus()에 대한 테스트부터 작성해보자.

```java
@Test
public void testSumPlusMoney() {
    Expression fiveDollars = Money.dollar(5);
    Expression tenFrancs = Money.franc(10);
    Bank bank = new Bank();
    bank.addRate("CHF", "USD", 2);
    Expression sum = new Sum(fiveDollars, tenFrancs).plus(fiveDollars);
    Money result = bank.reduce(sum, "USD");
    assertEquals(Money.dollar(15), result);
}
```

```java
public class Sum implements Expression {
    // ...
    @Override
    public Expression plus(Expression addend) {
        return new Sum(this, addend);
    }

}
```

> - [x] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD + 5 USD = 10 USD
> - [x] Bank.reduce(Money)
> - [x] Money에 대한 통화 변환을 수행하는 Reduce
> - [x] Reduce(Bank, String)
> - [x] Sum.plus
> - [ ] Expression.times

이이서 Expression.times를 위한 테스트 코드를 작성하자.

```java
@Test
public void testSumTimes() {
    Expression fiveDollars = Money.dollar(5);
    Expression tenFrancs = Money.franc(10);
    Bank bank = new Bank();
    bank.addRate("CHF", "USD", 2);
    Expression sum = new Sum(fiveDollars, tenFrancs).times(2);
    Money result = bank.reduce(sum, "USD");
    assertEquals(Money.dollar(20), result);
}
```

```java
public interface Expression {
    // ...
    Expression times(int multiplier);

}
```

```java
public class Sum implements Expression {
    // ...
    @Override
    public Expression times(int multiplier) {
        return new Sum(augend.times(multiplier), addend.times(multiplier));
    }

}
```

이제 모든 테스트가 통과한다.

> - [x] 5 USD + 10 CHF = 10 USD
> - [x] 5 USD + 5 USD = 10 USD
> - [x] Bank.reduce(Money)
> - [x] Money에 대한 통화 변환을 수행하는 Reduce
> - [x] Reduce(Bank, String)
> - [x] Sum.plus
> - [x] Expression.times



## Money 회고

### 다음 할 일은?

이제 코딩은 끝난 걸까? 아직 Sum.plus()와 Money.plus() 사이에 지저분한 중복이 남았다. Expression을 인터페이스 대신 클래스로 바꾼다면 공통되는 코드를 담아낼 적절한 곳이 될 것이다.

TDD를 완벽을 위한 노력의 일환으로 사용할 수도 있겠지만 그건 TDD의 가장 효과적인 용법이 아니다. 만약 시스템이 크다면, 당신이 늘 건드리는 부분들을 절대적으로 견고해야 한다. 그래야 나날이 수정할 때 안심할 수 있다.

'다음 할 일은 무엇인가?'에 관련된 또 다른 질문은 '어떤 테스트들이 추가로 더 필요할까?'다. 때로는 실패해야 하는 테스트가 성공하는 경우가 있는데, 그럴 땐 그 이유를 찾아내야 한다. 또는 실패해야 하는 테스트가 실제로 실패하기도 하는데, 이때는 이를 이미 알려진 제한 사항 또는 앞으로 해야 할 작업 등의 의미로 그 사실을 기록해야 한다.

마지막으로, 할일 목록이 빌 때가 그때까지 설계한 것을 검토하기에 적절한 시기다.

### 프로세스

TDD의 주기는 다음과 같다.

- 작은 테스트를 추가한다.
- 모든 테스트를 실행하고, 실패하는 것을 확인한다.
- 코드에 변화를 준다.
- 모든 테스트를 실행하고, 성공하는 것을 확인한다.
- 중복을 제거하기 위해 리팩토링한다.

### 최종 검토

TDD를 배울 때 명심해야 하는 세 가지는

- 테스트를 확실히 돌아가게 만드는 세 가지 접근법

  가짜로 구현하기, 삼각측량법, 명백하게 구현하기.

- 설계를 주도하기 위한 방법으로 테스트 코드와 실제 코드 사이의 중복을 제거하기.

- 길이 미끄러우면 속도를 줄이고 상황이 좋으면 속도를 높이는 식으로 테스트 사이의 간격을 조절할 수 있는 능력.

