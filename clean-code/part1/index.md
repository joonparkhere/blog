---
title: Clean Code 1~3장
date: 2021-05-06
pin: false
tags:
- Software Enginerring
- Clean Code
- Java
---


# 클린 코드



## 1장 깨끗한 코드

> 모든 프로그래머가 기한을 맞추려면 나쁜 코드를 양산할 수밖에 없다고 느낀다. 그들은 빨리 가려고 시간을 들이지 않는다. 진짜 전문가는 틀렸다는 사실을 잘 안다. 나쁜 코드를 양산하면 기한을 맞추지 못한다. 오히려 엉망진창인 상태로 인해 속도가 곧바로 늦어지고, 결국 기한을 놓친다. 기한을 맞추는 유일한 방법은, 그러니까 **빨리 가는 유일한 방법은, 언제나 코드를 최대한 깨끗하게 유지하는 습관**이다.

### 우리들 생각

깨끗한 코드가 무엇인가에 대한 노련한 프로그래머들의 답변입니다.

- C++ 창시자, 비야네 스트롭스트룹

  > 우아하고 효율적인 코드. 논리가 간단해야 버그가 숨어들지 못한다. **깨끗한 코드는 한 가지를 제대로** 한다.

- Object Oriented Analysis and Design with Application 저자, 그래디 부치

  > 깨끗한 코드는 단순하고 직접적이다. **결코 설계자의 의도를 숨기지 않는다**. 명쾌한 추상화와 단순한 제어문으로 가득하다.

- 이클립스 전략의 대부, 데이브 토마스

  >  깨끗한 코드는 **작성자가 아닌 사람도 읽기 쉽고 고치기도 쉽다**. 특정 목적을 달성하는 방법은 하나만 제공한다.

- Extreme Programming Adventure in C# 저자, 론 제프리스

  > 중복이 없다. 클래스, 메서드, 함수 등을 최대한 줄인다. 같은 작업을 여러 차례 반복한다면 코드가 아이디어를 제대로 표현하지 못한다는 증거이며, 의미 있는 이름으로 설정 / 단일 기능을 수행하도록 객체와 메서드 쪼개기 등으로 표현력을 강화햔다.

### 우리는 저자다

프로그래머들은 저자이며 독자입니다. 코드를 짤 때는 자신이 저자라는 사실을, 그리고 이 노력을 보고 판단을 내릴 독자가 있다는 사실을 기억해야 한다고 강조합니다. 예를 들어 기존 코드의 일부분을 개선 및 추가하는 경우, 아래 과정처럼 한 경험이 많습니다.

> 변경할 함수로 스크롤.
>잠시 멈추고 생각.
> 모듈 상단으로 스크롤해 변수 초기화를 확인.
>다시 내려와 입력.
> 지금 바꾸려는 함수를 호출하는 함수로 스크롤한 후 함수가 호출되는 방식을 살피기.
>다시 돌아와 코드 입력.
> 반복 ...

코드를 읽는 시간 대 코드를 짜는 시간 비율이 10 대 1을 훌쩍 넘기도 합니다. 기존 코드를 읽어야 새 코드를 짜므로 읽기 쉽게 만들면 사실은 짜기도 쉬워 집니다.



## 2장 의미있는 이름

### 분명한 의도 표현

변수, 함수, 클래스 이름은 **존재 이유**, **수행 기능**, **사용 방법**이 명확히 드러나 있어야 합니다.

```java
public List<int[]> getThem() {
    List<int[]> list1 = new ArrayList<int[]>();
    for (int[] x: theList)
    if (x[0] == 4)
        list1.add(x);
    return list1;
}
```

- 공백과 들여쓰기는 적당하고, 변수와 상수의 개수도 많지 않은 편입니다.
- 그러나 코드 맥락이 코드 자체에 드러나지 않습니다.
  1. `theList`에 무엇이 들어있는가
  2. 0번째 값이 어째서 중요한가
  3. 값 4는 무슨 의미인가
  4. 반환하는 리스트를 어떻게 사용하는가

```java
public List<int[]> getFlaggedCells() {
    List<int[]> flaggedCells = new ArrayList<int[]>();
    for (int[] cell: gameBoard)
        if (cell.isFlagged())
            flaggedCells.add(cell);
    return flaggedCells;
}
```

- 이름을 명료하게 설정하면 보다 코드의 목적을 쉽게 이해할 수 있습니다.

### 그릇된 정보와 의미없는 정보

여러 계정을 그룹으로 묶을 때, 실제 `List`가 아니라면 accountList라 명명하지 않는 것이 좋습니다. 프로그래머에게 `List`는 특수한 의미이기 때문에, accountGroup, Accounts 등으로 명명합니다.

서로 비슷한 이름을 사용하지 않아야 합니다. 한 모듈에서 `XYZControllerForEfficientHandlingOfStrings`라는 이름과 `XYZControllerForEfficientStorageOfStrings`라는 이름을 사용한다면, 아무래도 많이 헷갈릴 수 밖에 없습니다.

컴파일러를 통과할지라도 연속된 숫자를 덧붙이거나 불용어를 추가하는 방식은 적절하지 않습니다. 이름이 달라야 한다면 의미도 달라져야 합니다.

연속적인 숫자를 덧붙인 이름(`a1`,`a2`, ... ,`aN`)은 그릇된 정보를 제공하는 이름은 아니지만, 아무런 정보를 제공하지 못하는 이름일 뿐입니다. 저자 의도가 전혀 드러나지 않습니다.

불용어를 추가한 경우 역시 아무런 정보도 제공하지 못합니다. `Product`라는 클래스가 있을 때, 다른 클래스를 `ProduectInfo` 혹은 `ProductData`라 부른다면 개념을 구분하지 않은 채 이름만 달리한 경우입니다. 마치 a, an, the처럼 의미가 불분명한 불용어를 사용한 것과 같습니다. 불용어의 쓰임은 중복을 낳을 수 밖에 없습니다.`Name`과 `NameString`, `Customer`와 `CustomerObject` 등과 같이 차이가 전혀 와닿지 않습니다.

### 발음과 검색이 쉬운 이름

발음하기 어려운 이름은 토론하기도 어렵습니다.

```java
class DtaRcrd102 {
    private Date genymdhms;
    private Date modymdhms;
    private final String pszqint = "102";
}
```

```java
class Customer {
    private Date generationTimestamp;
    private Date modificationTimestamp;
    private final String recordId = "102";
}
```

아래의 코드는 보다 지적인 대화가 가능하고, 더욱 의미가 와닿게 됩니다.

문자 하나를 사용하는 이름과 상수는 텍스트 코드에서 쉽게 눈에 띄지 않는다는 문제가 있습니다. 만약 `d`라는 이름의 변수가 있고 이를 찾기 위해 d를 검색한다면, d가 들어가는 파일 이름이나 수식이 모두 검색되어서 아마도 찾고자 하는 걸 찾기 힘들 것 입니다. 대신 `WORK_DAYS_PER_WEEK`라는 이름이었다면, 훨씬 찾기 쉬웠을 것 입니다. **이름 길이는 범위 크기에 비례**해야 합니다. 변수나 상수를 코드 여러 곳에서 사용한다면 검색하기 쉬운 이름이 바람직합니다. 

### 인코딩을 피해라

유형이나 범위 정보처럼 이름에 인코딩할 정보는 아주 많습니다. 여기에 추가로 의미 구분을 위한 인코딩 규칙까지 추가하게 되면 그만큼 이름을 해독하기 어려워집니다. 멤버 변수를 선언할 때 `m_` 접두어를 붙이는 경우 등이 이에 해당합니다. 전반적인 코드에 `m_`이 붙어있다면 아무래도 가독성이 떨어지고, 심지어 사람들은 접두어를 무시하고 이름을 해독하는 방법을 재빨리 익힙니다.

인터페이스 클래스와 구현 클래스 구분을 위해 인코딩이 필요하기도 합니다. 흔히 `IShapeFactory`와 `ShapeFactory`처럼 인터페이스에는 `I` 접두어를 붙이기도 하는데, 이는 주의를 흐트리고 과도한 정보를 제공할 수 있습니다. 이보다는 구현 클래스에 인코딩을 해서 `ShapeFactoryImp` 등으로 사용하는 편이 좋습니다.

### 한 개념에 한 단어

추상적인 개념 하나에 단어 하나를 선택해 이를 고수해야 합니다. 같은 메스드를 클래스마다 `fetch`, `retrieve`, `get` 혹은 `controller`, `manager`, `driver` 등의 경우처럼 제각각 부르면 혼란스럽습니다.

더불어 한 단어를 두 가지 목적으로 사용하면 안됩니다. `add`라는 기존 메서드가 값 두 개를 더하거나 이어서 새로운 값을 만든다고 할 때, 집합에 값 하나를 추가하는 메서드를 `add`라 명명하는 것은 좋지 않습니다. 두 메서드는 맥락이 다르므로 `insert`나 `append`라는 이름이 적당합니다.

### 의미있는 맥락 추가

예를 들어, `firstName`, `lastName`, `street`, `city`, `state`, `zipcode`라는 변수가 있다면 이들이 주소를 표현함을 금방 알아챌 수 있습니다. 하지만 어느 메서드가 `state` 변수 하나만 사용한다면 주소 일부라는 걸 쉽게 알아차리지 못합니다. 따라서 `addr`라는 접두어를 추가해 `addrState`라 쓰거나, `Address`라는 클래스에 속하도록 수정 해주는 것이 좋습니다.

### 적합한 이름

클래스 혹은 객체 이름에는 명사나 명사구를 사용하는 것이 좋습니다. `Customer`, `WikiPage`, `Account`, `AddressParser` 등이 좋은 예시입니다. `Manager`, `Processor`, `Data`, `Info` 등과 같은 단어는 피하고, 동사는 사용하지 않는 것이 좋습니다.

메서드 이름은 동사나 동사구가 적합합니다. `postPayment`, `deletePage`, `save` 등이 좋은 예입니다. 접근자, 변경자, 조건자에는 javabean 표준에 따라 값 앞에 `get`, `set`, `is`를 붙입니다.



## 3장 함수

### 작게 만들어라

함수를 만드는 **첫째 규칙은 '작게'** 입니다. **두 번째 규칙은 '더 작게'** 입니다. 함수는 100줄을 넘어서는 절대로 안 되고, 20줄도 긴 편입니다. 각 함수가 명백하게 이야기 하나를 표현하도록 작게 작게 줄여야 합니다. 이는 곧 if / else 문, while 문 등에 들어가는 블록은 한 줄이어야 한다는 의미입니다. 대개 블록에서 함수를 호출하고, 그렇게 되면 바깥을 감싸는 함수가 작아져서 코드 이해가 쉬워 집니다.

```java
public static String renderPageWithSetupsAndTeardowns (PageData pageData, bollean isSuite) throws Exception {
    if (isTestPage(pageData))
        includeSetupAndTeardownPages(pageData, isSuite);
    return pageData.getHtml();
}
```

### 한 가지만 해라

함수는 **한 가지만을 해야 하고, 그 한가지를 잘 해야 합니다.** 문제는 그 '한 가지'가 무엇인지 알기가 어렵다는 점입니다. 일반적으로, 지정된 함수 이름 아래에서 추상화 수준이 하나인 단계만 수행한다면 그 함수는 한 가지 작업만 하는 것 입니다.  함수 내에서 의미 있는 이름으로 다른 함수를 추출 할 수 있다면 그 함수는 여러 작업을 하는 셈입니다.

즉, **함수 당 추상화 수준은 하나**로 해야 합니다. 한 함수 내에 추상화 수준을 섞으면, 특정 표현이 근본 개념인지 아니면 세부 사항인지 구분하기 어려워져 헷갈립니다. 그래서 **내려가기** 규칙이라는 게 있습니다. 코드는 위에서 아래로 이야기처럼 읽혀야 좋습니다. 한 함수 다음에는 추상화 수준이 한 단계 낮은 함수가 위치하도록 합니다. 즉, 위에서 아래로 프로그램을 읽으면 함수 추상화 수준이 한 번에 한 단계씩 낮아집니다.

### Switch 문

`switch`문은 본질적으로 N가지를 처리하기 때문에 작게 만들기 어렵습니다. 그럼에도 다형성을 활용하여 조금은 개선할 수 있습니다.

```java
public Money calculatePay(Employee e) throws InvalidExployeeType {
    switch (e.type) {
        case COMMISSIONED:
            return calculateCommissionedPay(e);
        case HOURLY:
            return calculateHourlyPay(e);
        case SALARIED:
            return calculateSalariedPay(e);
        default:
            throw new InvalidExployeeType(e.type);
    }
}
```

위 코드는 몇 가지 문제가 있습니다.

1. 함수가 깁니다. 새로운 직원 유형을 추가하면 더욱 길어집니다.
2. '한 가지' 작업만 수행하지 않습니다.
3. SRP(Single Responsibility Principle)을 위반합니다.
4. OCP(Open Closed Prinicple)을 위반합니다.

무엇보다 가장 심각한 문제는 위 함수와 구조가 동일한 함수가 무한정 존재할 수 있다는 사실입니다.

```java
public abstract class Employee {
    public abstract boolean isPayday();
    public abstract Money calculatePay();
    public abstract void deliverPay(Money pay);
}

public interface EmployeeFactory {
    public Employee makeEmployee(EmployeedRecord r) throws InvalidEmployeeType;
}

public class EmployeeFactoryImpl implements EmployeeFactory {
    public Employee makeEmployee(EmployeeRecord r) throws InvalidEmployeeType {
        switch (r.type) {
            case COMMISSIONED:
                return new CommissionedEmployyee(r);
            case HOURLY:
                return new HourlyEmployee(r);
            case SALARIED:
                return new SalariedEmployee(r);
            default:
                thorw new InvalidEmployeeType(r.type);
        }
    }
}
```

이렇게 `switch`문을 상속 관계로 추상 팩토리에 숨겨서 노출되지 않도록 하고, 다형성을 이용해 적절한 `Employee` 파생 클래스의 인스턴스를 생성하도록 수정합니다.

### 서술적인 이름을 사용하라

`setupTeardownIncluder`, `isTestable`, `includeSetupAndTeardownPages` 등 서술적인 이름을 사용하는 것이 좋습니다. 서술적인 이름을 사용하면 개발자 머릿속에서도 설계가 뚜렷해지므로 코드를 개선하기 쉬워 집니다. 이름이 길어도 괜찮습니다. 길고 서술적인 이름이 길고 서술적인 주석보다 좋습니다. 함수 이름을 정할 때는 여러 단어가 쉽게 읽히는 명명법을 사용해서, 그리고 여러 단어를 사용해 함수 기능을 잘 표현하도록 이름을 선택해야 합니다.

그리고 이름을 붙일 때는 일관성이 있어야 합니다. 모듈 내에서 함수 이름은 같은 문구, 명사, 동사를 사용합니다. `includeSetupAndTeardownPages`, `includeSetupPages`, `includeSuiteSetupPage`, `includeSetupPage` 등이 좋은 예입니다. 문체가 비슷하면 이야기를 순차적으로 풀어가기도 쉬워 집니다.

### 함수 인수

함수에서 이상적인 인수 개수는 0개 입니다. 다음은 1개이고, 그 다음은 2개 입니다. 3개는 가능한 피하는 편이 좋고, 4개 이상은 특별한 이유가 필요합니다. 기본적으로 인수는 개념을 이해하기 어렵게 만듭니다. 테스트 관점에서 보면 인수는 더 어렵습니다. 인수가 3개를 넘어가면 인수마다 유효한 값으로 모든 조합을 구성해 테스트해야 하므로 상당히 부담스러워집니다. 그리고 흔히 함수에다 인수로 입력을 넘기고 반환값으로 출력을 받는다는 개념에 익숙하기 때문에 출력 인수는 입력 인수보다 이해하기 어렵습니다.

함수에 인수 1개를 넘기는 단항 형식에는 크게 두 가지 경우가 있습니다. 하나는 인수에 질문을 던지는 경우로, `boolean fileExists("MyFile")`이 예시입니다. 다른 하나는 인수를 뭔가로 변환해 결과를 반환하는 경우로, `InputStream fileOpen("MyFile")`이 예시입니다. 드물게 `passwordAttemptFailedNtimes(attempts)`처럼 이벤트를 처리하는 함수도 있습니다. 위의 경우들이 아니라면 단항 함수는 가급적 피하는 편이 좋습니다.

인수가 2개인 함수는 인수가 1개인 함수보다 이해하기 어렵습니다. `writeField(name)`은 `writeField(outputStream, name)`보다 이해하기 쉽듯이, 전자의 형식이 더 쉽게 읽히고 빨리 이해됩니다. 후자는 잠시 주춤하며 첫 인수를 무시해야 한다는 사실을 깨닫는 시간이 필요합니다. 따라서 가급적 이항 함수는 단항 형식으로 바꾸도록 애써야 합니다. `writeField` 메서드를 `outputStream` 클래스 구성원으로 만들어 `outputStream.sriteField(name)`으로 호출하게끔 만들어야 합니다.

인수가 2~3개 필요하다면 일부를 독자적인 클래스 변수로 선언할 가능성을 짚어보는 것이 좋습니다. `Circle makeCircle(doube x, double y, double radius)`를 `Circle makeCircle(Point point, double radius)`로 바꾸는 것처럼 인수를 줄임과 더불어, 묶인 `x`와 `y`는 클래스 내에서 이름을 붙여야 하므로 개념을 명확히 표현할 수도 있게 됩니다.

함수의 의도나 인수의 순서와 의도를 제대로 표현하려면 좋은 함수 이름이 필요합니다. `write(name)`보다는 `wirteField(name)`이 조금 더 낫고, `assertEquals(expected, actual)`보다 `assertExpectedEqualsActual(expected, acutal)`이 더 낫습니다.

### 부수 효과를 일으키지 마라

함수에서 한 가지를 하겠다고 약속하고선 예상치 못하게 클래스 변수를 수정하거나, 함수로 넘어온 인수나 시스템 전역 변수를 수정하거나 하는 등의 부수 효과를 일으키는 경우가 있다.

```java
public boolean checkPassword(String userName, String password) {
    User user = UserGateway.findByName(userName);
    if (user != User.NULL) {
        String codedPhrase = user.getPhraseEncodedByPassword();
        String phrase = crptographer.decrpt(codedPhrase, password);
        if ("Valid Password".equals(phrase)) {
            Session.initialize();
            return true;
        }
    }
    return false;
}
```

여기서 함수가 일으키는 부수 효과는 `Session.initialize()` 호출입니다. `checkPassword` 함수의 이름만 봐서는 세션을 초기화한다는 사실이 드러나지 않습니다. 이런 부수 효과는 시간적인 결합을 초래합니다. 즉, 위 함수는 세션을 초기화해도 괜찮은 경우에나 호출이 가능합니다. 자칫 잘못 호출하면 의도하지 않게 세션 정보가 날아가게 되는 것입니다 이러한 시간적인 결합은 혼란을 일으킵니다.

### 명령과 조회를 분리하라

함수는 뭔가를 수행하거나 뭔가에 답하거나 둘 중 하나만 해야 합니다. `if (set("username", "unclebob"))` 코드를 독자 입장에서 읽어보면, "username"이 "unclebob"으로 설정되어 있는지 확인하는 코드인지, 아니면 "username"을 'unclebob'으로 설정하는 코드인지 함수를 호출하는 코드만 봐서는 의미가 모호합니다. "set"이라는 단어가 동사인지 형용사인지 분간하기 어려운 탓입니다. 이러한 혼란을 애초에 뿌리 뽑으려면 객체 상태를 변경하는 부분과 객체 정보를 반환하는 부분을 분리하는 것이 좋습니다.

### 오류 코드보다 예외를 사용하라

```java
if (deletePage(page) == E_OK) {
    if (registry.deleteReference(page.name) == E_OK) {
        if (configKeys.deleteKey(page.name.makeKey()) == E_OK) {
            logger.log("page deleted");
        } else {
            logger.log("configKey not deleted");
        }
    } else {
        logger.log("deleteReference from registry failed");
    }
} else {
    logger.log("deleted failed");
    return E_ERROR;
}
```

오류 코드를 반환하면 호출자는 오류 코드를 곧바로 처리해야 하는 문제에 부딪힙니다. 반면 오류 코드 대신 예외를 사용하면 오류 처리 코드가 원래 코드에서 분리되므로 코드가 깔끔해 집니다.

```java
try {
    deletPage(page);
    registry.deleteReference(page.name);
    configKeys.deleteKey(pagename.makeKey());
}
catch (Exception e) {
    logger.log(e.getMessage());
}
```

사실 try / catch 불록은 코드 구조에 혼란을 일으키며, 정상 동작과 오류 처리 동작을 뒤섞습니다. 그러므로 해당 블록을 별도 함수로 뽑아내는 편이 좋습니다.

```java
public void delete(Page page) {
    try {
        deletePageAndAllReferences(page);
    }
    catch (Exception e) {
        logError(e);
    }
}

private void deletePageAndAllReferences(Page page) throws Exception {
    deletePage(page);
    registry.deleteReference(page.name);
    configKeys.deletKey(page.name.makeKey());
}

private void logError(Exception e) {
    logger.log(e.getMessage());
}
```

오류 처리도 한 가지 작업입니다. 함수는 '한 가지' 작업만 해야 하므로 오류를 처리하는 함수는 오류만 처리해야 마땅합니다.

### 반복하지 마라

중복은 소프트웨어에서 모든 악의 근원입니다. 많은 원칙과 기법이 중복을 없애거나 제어할 목적으로 나왔습니다. 구조적 프로그래밍, AOP, COP 모두 어떤 면에서 중복 제거 전략입니다. 

### 함수를 짜는 흐름

소프트웨어를 짜는 행위는 여느 글짓기와 비슷합니다. 논문이나 기사를 작성할 때는 먼저 생각을 기록한 후 읽기 좋게 다듬는 과정을 거칩니다. 초안은 대게 서투르고 어수선하므로 원하는 대로 읽힐 때까지 말을 다듬고 문장을 고치고 문단을 정리합니다.

함수를 짤 때도 마찬가지 입니다. 처음에는 들여쓰기 단계도 많고 중복된 루프도 많으며, 인수 목록도 아주 깁니다. 하지만 그 서투른 코드를 빠짐없이 테스트하는 단위 테스트 케이스를 만들어서 코드를 다듬고, 함수를 만들고, 이름을 바꾸고, 중복을 제거하고, 메서드를 줄여야 합니다. 최종적으로는 지금껏 설명한 규칙을 따르는 함수가 얻어 집니다. 처음부터 탁 짜낼 수 있는 사람은 없습니다.

