---
title: Clean Code 14~17장
date: 2021-05-21
pin: false
tags:
- Software Enginerring
- Clean Code
- Java
---

## 14장 점진적인 개선

다음은 `Args` 생성자에 (입력으로 돌어온) 인수 문자열과 형식 문자열을 넘겨 `Args` 인스턴스를 생성한 후 `Args` 인스턴스에다 인수 값을 질의하는 예제이다.

```java
public static void main(String[] args) {
  try {
    Args arg = new Args("l,p#,d*", args);
    boolean logging = arg.getBoolean('l');
    int port = arg.getInt('p');
    String directory = arg.getString('d');
    executeApplication(logging, port, directory);
  } catch (ArgsException e) {
    System.out.print("Argument error: %s\n", e.errorMessage());
  }
}
```

### Args 구현

```java
public class Args {
  private Map<Character, ArgumentMarshaler> marshalers;
  private Set<Character> argsFound;
  private ListIterator<String> currentArgument;
  
  public Args(String schema, String[] args) throws ArgsException { 
    marshalers = new HashMap<Character, ArgumentMarshaler>(); 
    argsFound = new HashSet<Character>();
    
    parseSchema(schema);
    parseArgumentStrings(Arrays.asList(args)); 
  }
  
  private void parseSchema(String schema) throws ArgsException { 
    for (String element : schema.split(","))
      if (element.length() > 0) 
        parseSchemaElement(element.trim());
  }
  
  private void parseSchemaElement(String element) throws ArgsException { 
    char elementId = element.charAt(0);
    String elementTail = element.substring(1); validateSchemaElementId(elementId);
    if (elementTail.length() == 0)
      marshalers.put(elementId, new BooleanArgumentMarshaler());
    else if (elementTail.equals("*")) 
      marshalers.put(elementId, new StringArgumentMarshaler());
    else if (elementTail.equals("#"))
      marshalers.put(elementId, new IntegerArgumentMarshaler());
    else if (elementTail.equals("##")) 
      marshalers.put(elementId, new DoubleArgumentMarshaler());
    else if (elementTail.equals("[*]"))
      marshalers.put(elementId, new StringArrayArgumentMarshaler());
    else
      throw new ArgsException(INVALID_ARGUMENT_FORMAT, elementId, elementTail);
  }
  
  private void validateSchemaElementId(char elementId) throws ArgsException { 
    if (!Character.isLetter(elementId))
      throw new ArgsException(INVALID_ARGUMENT_NAME, elementId, null); 
  }
  
  private void parseArgumentStrings(List<String> argsList) throws ArgsException {
    for (currentArgument = argsList.listIterator(); currentArgument.hasNext();) {
      String argString = currentArgument.next(); 
      if (argString.startsWith("-")) {
        parseArgumentCharacters(argString.substring(1)); 
      } else {
        currentArgument.previous();
        break; 
      }
    } 
  }
  
  private void parseArgumentCharacters(String argChars) throws ArgsException { 
    for (int i = 0; i < argChars.length(); i++)
      parseArgumentCharacter(argChars.charAt(i)); 
  }
  
  private void parseArgumentCharacter(char argChar) throws ArgsException { 
    ArgumentMarshaler m = marshalers.get(argChar);
    if (m == null) {
      throw new ArgsException(UNEXPECTED_ARGUMENT, argChar, null); 
    } else {
      argsFound.add(argChar); 
      try {
        m.set(currentArgument); 
      } catch (ArgsException e) {
        e.setErrorArgumentId(argChar);
        throw e; 
      }
    } 
  }
  
  public boolean has(char arg) { 
    return argsFound.contains(arg);
  }
  
  public int nextArgument() {
    return currentArgument.nextIndex();
  }
  
  public boolean getBoolean(char arg) {
    return BooleanArgumentMarshaler.getValue(marshalers.get(arg));
  }
  
  public String getString(char arg) {
    return StringArgumentMarshaler.getValue(marshalers.get(arg));
  }
  
  public int getInt(char arg) {
    return IntegerArgumentMarshaler.getValue(marshalers.get(arg));
  }
  
  public double getDouble(char arg) {
    return DoubleArgumentMarshaler.getValue(marshalers.get(arg));
  }
  
  public String[] getStringArray(char arg) {
    return StringArrayArgumentMarshaler.getValue(marshalers.get(arg));
  } 
}
```

- 스타일과 구조를 주의 깊게 읽어보자
- 여기저기 뒤적일 필요 없이 위에서 아래로 코드가 읽힌다.

아래는 일부러 빼놓은 `ArgumentMarshaler` 정의 및 파생 클래스이다.

```java
public interface ArgumentMarshaler {
  void set(Iterator<String> currentArgument) throws ArgsException;
}
```

```java
public class BooleanArgumentMarshaler implements ArgumentMarshaler { 
  private boolean booleanValue = false;
  
  public void set(Iterator<String> currentArgument) throws ArgsException { 
    booleanValue = true;
  }
  
  public static boolean getValue(ArgumentMarshaler am) {
    if (am != null && am instanceof BooleanArgumentMarshaler)
      return ((BooleanArgumentMarshaler) am).booleanValue; 
    else
      return false; 
  }
}
```

```java
public class StringArgumentMarshaler implements ArgumentMarshaler { 
  private String stringValue = "";
  
  public void set(Iterator<String> currentArgument) throws ArgsException { 
    try {
      stringValue = currentArgument.next(); 
    } catch (NoSuchElementException e) {
      throw new ArgsException(MISSING_STRING); 
    }
  }
  
  public static String getValue(ArgumentMarshaler am) {
    if (am != null && am instanceof StringArgumentMarshaler)
      return ((StringArgumentMarshaler) am).stringValue; 
    else
      return ""; 
  }
}
```

```java
public class IntegerArgumentMarshaler implements ArgumentMarshaler { 
  private int intValue = 0;
  
  public void set(Iterator<String> currentArgument) throws ArgsException { 
    String parameter = null;
    try {
      parameter = currentArgument.next();
      intValue = Integer.parseInt(parameter);
    } catch (NoSuchElementException e) {
      throw new ArgsException(MISSING_INTEGER);
    } catch (NumberFormatException e) {
      throw new ArgsException(INVALID_INTEGER, parameter); 
    }
  }
  
  public static int getValue(ArgumentMarshaler am) {
    if (am != null && am instanceof IntegerArgumentMarshaler)
      return ((IntegerArgumentMarshaler) am).intValue; 
    else
    return 0; 
  }
}
```

- 나머지 `DoubleArgumentMarshaler`와 `StringArrayArgumentMarshaler`는 타 파생 클래스와 똑같은 패턴이므로 코드를 생략한다.

다음은 오류 코드를 정의하는 부분이다.

```java
public class ArgsException extends Exception { 
  private char errorArgumentId = '\0'; 
  private String errorParameter = null; 
  private ErrorCode errorCode = OK;
  
  public ArgsException() {}
  
  public ArgsException(String message) {super(message);}
  
  public ArgsException(ErrorCode errorCode) { 
    this.errorCode = errorCode;
  }
  
  public ArgsException(ErrorCode errorCode, String errorParameter) { 
    this.errorCode = errorCode;
    this.errorParameter = errorParameter;
  }
  
  public ArgsException(ErrorCode errorCode, char errorArgumentId, String errorParameter) {
    this.errorCode = errorCode; 
    this.errorParameter = errorParameter; 
    this.errorArgumentId = errorArgumentId;
  }
  
  public char getErrorArgumentId() { 
    return errorArgumentId;
  }
  
  public void setErrorArgumentId(char errorArgumentId) { 
    this.errorArgumentId = errorArgumentId;
  }
  
  public String getErrorParameter() { 
    return errorParameter;
  }
  
  public void setErrorParameter(String errorParameter) { 
    this.errorParameter = errorParameter;
  }
  
  public ErrorCode getErrorCode() { 
    return errorCode;
  }
  
  public void setErrorCode(ErrorCode errorCode) { 
    this.errorCode = errorCode;
  }
  
  public String errorMessage() { 
    switch (errorCode) {
      case OK:
        return "TILT: Should not get here.";
      case UNEXPECTED_ARGUMENT:
        return String.format("Argument -%c unexpected.", errorArgumentId);
      case MISSING_STRING:
        return String.format("Could not find string parameter for -%c.", errorArgumentId);
      case INVALID_INTEGER:
        return String.format("Argument -%c expects an integer but was '%s'.", errorArgumentId, errorParameter);
      case MISSING_INTEGER:
        return String.format("Could not find integer parameter for -%c.", errorArgumentId);
      case INVALID_DOUBLE:
        return String.format("Argument -%c expects a double but was '%s'.", errorArgumentId, errorParameter);
      case MISSING_DOUBLE:
        return String.format("Could not find double parameter for -%c.", errorArgumentId); 
      case INVALID_ARGUMENT_NAME:
        return String.format("'%c' is not a valid argument name.", errorArgumentId);
      case INVALID_ARGUMENT_FORMAT:
        return String.format("'%s' is not a valid argument format.", errorParameter);
    }
    return ""; 
  }
  
  public enum ErrorCode {
    OK, INVALID_ARGUMENT_FORMAT, UNEXPECTED_ARGUMENT, INVALID_ARGUMENT_NAME, 
    MISSING_STRING, MISSING_INTEGER, INVALID_INTEGER, MISSING_DOUBLE, INVALID_DOUBLE
  }
}
```

처음부터 코드를 이름을 붙인 방법, 함수 크기, 코드 형식에 각별히 주목해서 한 번 더 세세히 읽어보자.

### 깔끔한 코드는 어떻게?

프로그래밍은 과학보다 공예에 가깝다. 깨끗한 코드를 짜려면 먼저 지저분한 코드를 짠 뒤에 정리해야 한다. 대다수 신입 프로그래머는 이 충고를 충실히 따르지 않는다. 일단 프로그램이 돌아가는 프로그램을 목표로 잡는다. 그리고 프로그램이 돌아가면 다음 업무로 넘어간다. 경험이 풍부한 프로그래머라면 이런 행동이 자살 행위라는 사실을 잘 알 것이다.

### 점진적으로 개선하다

프로그램을 망치는 가장 좋은 방법 중 하나는 개선이라는 이름 아래 구조를 크게 뒤집는 행위다. 어떤 프로그램은 그저 그런 '개선'에서 결코 회복하지 못한다. 그래서 주로 TDD 기법을 사용한다. TDD(Test-Drive Development)는 언제 어느 때라도 시스템이 돌아가야 한다는 원칙을 따른다.

그저 돌아가는 코드만으로는 부족하며, 돌아가는 코드가 심하게 망가지는 사례는 흔하다. 설계와 구조를 개선할 시간이 없다고 변명할지 모르지만 이는 나쁜 코드보다 더 오랫동안 더 심각하게 개발 프로젝트에 악영향을 미칠 것이다. 나중에 코드를 개선하려면 비용이 엄청나게 많이 들것이다. 코드가 썩어가며 모듈은 서로서로 얽히고설켜 뒤엉키고 숨겨진 의존성이 수도 없이 생긴다. 반면 처음부터 코드를 깨끗하게 유지하기란 상대적으로 쉽다. 아침에 엉망으로 만든 코드를 오후에 정리하기는 어렵지 않다. 그러므로 언제나 최대한 깔끔하고 단순하게 정리하자. 절대로 썩어가게 방치하면 안 된다.



## 15장 JUnit 들여다보기

### JUnit 프레임워크

JUnit은 수많은 저자가 작성한 프레임워크이지만, 시작은 컨트 벡과 에릭 감마 두 사람이다. 이를 사용한 예시로, 문자열 비교 오류를 파악할 때 유용한 모듈 `ComparisonCompactor`과 그에 대한 테스트 코드를 살펴보겠다.

```java
public class ComparisonCompactorTest extends TestCase {

    public void testMessage() {
        String failure = new ComparisonCompactor(0, "b", "c").compact("a");
        assertTrue("a expected:<[b]> but was:<[c]>".equals(failure));
    }

    public void testStartSame() {
        String failure = new ComparisonCompactor(1, "ba", "bc").compact(null);
        assertEquals("expected:<b[a]> but was:<b[c]>", failure);
    }

    public void testEndSame() {
        String failure = new ComparisonCompactor(1, "ab", "cb").compact(null);
        assertEquals("expected:<[a]b> but was:<[c]b>", failure);
    }

    public void testSame() {
        String failure = new ComparisonCompactor(1, "ab", "ab").compact(null);
        assertEquals("expected:<ab> but was:<ab>", failure);
    }

    public void testNoContextStartAndEndSame() {
        String failure = new ComparisonCompactor(0, "abc", "adc").compact(null);
        assertEquals("expected:<...[b]...> but was:<...[d]...>", failure);
    }

    public void testStartAndEndContext() {
        String failure = new ComparisonCompactor(1, "abc", "adc").compact(null);
        assertEquals("expected:<a[b]c> but was:<a[d]c>", failure);
    }

    public void testStartAndEndContextWithEllipses() {
        String failure = new ComparisonCompactor(1, "abcde", "abfde").compact(null);
        assertEquals("expected:<...b[c]d...> but was:<...b[f]d...>", failure);
    }

    public void testComparisonErrorStartSameComplete() {
        String failure = new ComparisonCompactor(2, "ab", "abc").compact(null);
        assertEquals("expected:<ab[]> but was:<ab[c]>", failure);
    }

    public void testComparisonErrorEndSameComplete() {
        String failure = new ComparisonCompactor(0, "bc", "abc").compact(null);
        assertEquals("expected:<[]...> but was:<[a]...>", failure);
    }

    public void testComparisonErrorEndSameCompleteContext() {
        String failure = new ComparisonCompactor(2, "bc", "abc").compact(null);
        assertEquals("expected:<[]bc> but was:<[a]bc>", failure);
    }

    public void testComparisonErrorOverlappingMatches() {
        String failure = new ComparisonCompactor(0, "abc", "abbc").compact(null);
        assertEquals("expected:<...[]...> but was:<...[b]...>", failure);
    }

    public void testComparisonErrorOverlappingMatchesContext() {
        String failure = new ComparisonCompactor(2, "abc", "abbc").compact(null);
        assertEquals("expected:<ab[]c> but was:<ab[b]c>", failure);
    }

    public void testComparisonErrorOverlappingMatches2() {
        String failure = new ComparisonCompactor(0, "abcdde", "abcde").compact(null);
        assertEquals("expected:<...[d]...> but was:<...[]...>", failure);
    }

    public void testComparisonErrorOverlappingMatches2Context() {
        String failure = new ComparisonCompactor(2, "abcdde", "abcde").compact(null);
        assertEquals("expected:<...cd[d]e> but was:<...cd[]e>", failure);
    }

    public void testComparisonErrorWithActualNull() {
        String failure = new ComparisonCompactor(0, "a", null).compact(null);
        assertEquals("expected:<a> but was:<null>", failure);
    }

    public void testComparisonErrorWithActualNullContext() {
        String failure = new ComparisonCompactor(2, "a", null).compact(null);
        assertEquals("expected:<a> but was:<null>", failure);
    }

    public void testComparisonErrorWithExpectedNull() {
        String failure = new ComparisonCompactor(0, null, "a").compact(null);
        assertEquals("expected:<null> but was:<a>", failure);
    }

    public void testComparisonErrorWithExpectedNullContext() {
        String failure = new ComparisonCompactor(2, null, "a").compact(null);
        assertEquals("expected:<null> but was:<a>", failure);
    }

    public void testBug609972() {
        String failure = new ComparisonCompactor(10, "S&P500", "0").compact(null);
        assertEquals("expected:<[S&P50]0> but was:<[]0>", failure);
    }
}
```

- 위 테스트 코드로 `ComparisonCompactor` 모듈에 대한 코드 커버리지 분석을 수행하면 100%가 나온다. 즉, 테스트 케이스가 모든 행, 모든 `if`문, 모든 `for` 문을 실행한다는 의미다.

다음 코드는 `ComparisionCompactor` 모듈이다. 코드는 잘 분리되어있고, 표현력이 적절하며, 구조가 단순하므로 코드를 잘 살펴보자.

```java
public class ComparisonCompactor {

    private static final String ELLIPSIS = "...";
    private static final String DELTA_END = "]";
    private static final String DELTA_START = "[";

    private int fContextLength;
    private String fExpected;
    private String fActual;
    private int fPrefix;
    private int fSuffix;

    public ComparisonCompactor(int contextLength, String expected, String actual) {
        fContextLength = contextLength;
        fExpected = expected;
        fActual = actual;
    }

    public String compact(String message) {
        if (fExpected == null || fActual == null || areStringsEqual()) {
            return Assert.format(message, fExpected, fActual);
        }

        findCommonPrefix();
        findCommonSuffix();
        String expected = compactString(fExpected);
        String actual = compactString(fActual);
        return Assert.format(message, expected, actual);
    }

    private String compactString(String source) {
        String result = DELTA_START + source.substring(fPrefix, source.length() - fSuffix + 1) + DELTA_END;
        if (fPrefix > 0) {
            result = computeCommonPrefix() + result;
        }
        if (fSuffix > 0) {
            result = result + computeCommonSuffix();
        }
        return result;
    }

    private void findCommonPrefix() {
        fPrefix = 0;
        int end = Math.min(fExpected.length(), fActual.length());
        for (; fPrefix < end; fPrefix++) {
            if (fExpected.charAt(fPrefix) != fActual.charAt(fPrefix)) {
                break;
            }
        }
    }

    private void findCommonSuffix() {
        int expectedSuffix = fExpected.length() - 1;
        int actualSuffix = fActual.length() - 1;
        for (; actualSuffix >= fPrefix && expectedSuffix >= fPrefix; actualSuffix--, expectedSuffix--) {
            if (fExpected.charAt(expectedSuffix) != fActual.charAt(actualSuffix)) {
                break;
            }
        }
        fSuffix = fExpected.length() - expectedSuffix;
    }

    private String computeCommonPrefix() {
        return (fPrefix > fContextLength ? ELLIPSIS : "") + fExpected.substring(Math.max(0, fPrefix - fContextLength), fPrefix);
    }

    private String computeCommonSuffix() {
        int end = Math.min(fExpected.length() - fSuffix + 1 + fContextLength, fExpected.length());
        return fExpected.substring(fExpected.length() - fSuffix + 1, end) + (fExpected.length() - fSuffix + 1 < fExpected.length() - fContextLength ? ELLIPSIS : "");
    }

    private boolean areStringsEqual() {
        return fExpected.equals(fActual);
    }
}
```

위 코드는 대부분이 잘 짜여 있지만 몇몇 부분은 개선할만한 하다.

가장 먼저 멤버 변수 앞에 붙인 접두어는 중복되는 정보로, 모두 제거하는 것이 좋다.

```java
private int contextLength;
private String expected;
private String actual;
private int prefix;
private int suffix;
```

다음으로 `compact()` 함수 시작부에 캡슐화되지 않은 조건문이 보인다. 의도를 명확히 표현하려면 조건문을 캡슐화하는 것이 좋다. 이때 부정문은 긍정문보다 이해하기 약간 더 어려우므로 조건을 긍정으로 만드는 편이 좋다.

```java
public String compact(String message) {
    if (canBeCompacted()) {
        /* ... */
    } else {
        /* ... */
    }       
}

private boolean canBeCompacted() {
    return expected != null && actual != null && !areStringsEqual();
}
```

그리고 `compact()` 함수에서 사용하는 `this.*`도 거슬린다. 멤버 변수와 이름이 똑같은 변수를 사용하면 혼란을 줄 수 있으므로 보다 명확한 이름으로 수정하자.

```java
public String compact(String message) {
    if (canBeCompacted()) {
        findCommonPrefix();
        findCommonSuffix();
        String compactExpected = compactString(expected);
        String compactActual = compactString(actual);
        return Assert.format(message, compactExpected, compactActual);
    } else {
        return Assert.format(message, expected, actual);
    }       
}
```

이번에는 `compact()` 함수 이름을 고쳐보자. 내부의 조건문에 따라 압축하지 않을 수도 있기 때문에 오류 점검이라는 부가 단계가 숨겨져 있다. 게다가 함수는 단순히 압축된 문자열이 아니라 형식이 갖춰진 문자열을 반환한다. 따라서 역할에 맞게 `formatCompactedComparison`으로 바꾸자.

그리고 `if`문 안에서는 예상 문자열과 실제 문자열을 진짜로 압축하는 부분이 있다. 이 부분을 빼내 `compactExpectedAndActual`이라는 메서드로 만들자. 이때 중간 과정에 필요한 두 부분을 멤버 변수로 승격시켰다.

```java
/* ... */

private String compactExpected;
private String compactActual;

/* ... */

public String formatCompactedComparison(String message) {
    if (canBeCompacted()) {
        compactExpectedAndActual();
        return Assert.format(message, compactExpected, compactActual);
    } else {
        return Assert.format(message, expected, actual);
    }
}

private compactExpectedAndActual() {
    findCommonPrefix();
    findCommonSuffix();
    compactExpected = compactString(expected);
    compactActual = compactString(actual);
}
```

이제 `compactExpectedAndActual()` 함수를 살펴보자. 아래 두 줄은 변수를 반환하지만, 위 두 줄은 반환값이 없다. 즉 함수 사용 방식이 일관적이지 않다. 이를 수정하자.

```java
private compactExpectedAndActual() {
    prefixIndex = findCommonPrefix();
    suffixIndex = findCommonSuffix();
    String compactExpected = compactString(expected);
    String compactActual = compactString(actual);
}

private int findCommonPrefix() {
    int prefixIndex = 0;
    int end = Math.min(expected.length(), actual.length());
    for (; prefixIndex < end; prefixIndex++) {
        if (expected.charAt(prefixIndex) != actual.charAt(prefixIndex)) {
            break;
        }
    }
    return prefixIndex;
}

private int findCommonSuffix() {
    int expectedSuffix = expected.length() - 1;
    int actualSuffix = actual.length() - 1;
    for (; actualSuffix >= prefixIndex && expectedSuffix >= prefix; actualSuffix--, expectedSuffix--) {
        if (expected.charAt(expectedSuffix) != actual.charAt(actualSuffix)) {
            break;
        }
    }
    return expected.length() - expectedSuffix;
}
```

`findCommonSuffix()`를 주의 깊게 살펴보면 숨겨진 시간적인 결합이 존재한다. 다시 말해, `findCommonSuffix`는 `findCommonPrefix`가 `prefixIndex`를 계산한다는 사실에 의존한다. 따라서 시간 결합을 외부에 노출하고자 `prefixIndex`를 인수로 넘기도록 수정하자.

```java
private compactExpectedAndActual() {
    prefixIndex = findCommonPrefix();
    suffixIndex = findCommonSuffix(prefixIndex);
    String compactExpected = compactString(expected);
    String compactActual = compactString(actual);
}

private int findCommonSuffix(int prefixIndex) {
    int expectedSuffix = expected.length() - 1;
    int actualSuffix = actual.length() - 1;
    for (; actualSuffix >= prefixIndex && expectedSuffix >= prefix; actualSuffix--, expectedSuffix--) {
        if (expected.charAt(expectedSuffix) != actual.charAt(actualSuffix)) {
            break;
        }
    }
    return expected.length() - expectedSuffix;
}
```

그러나 이 방식은 다소 자의적이다. 함수 호출 순서는 확실해 지지만 `prefixIndex`가 필요한 이유는 설명하지 못한다. 그러므로 다른 방법을 고안해보자.

```java
private compactExpectedAndActual() {
    findCommonPrefixAndSuffix();
    String compactExpected = compactString(expected);
    String compactActual = compactString(actual);
}

private void findCommonPrefixAndSuffix() {
    findCommonPrefix();
    int suffixLength = 1;
    for (; suffixOverlapsPrefix(suffixLength); suffixLength++) {
        if (charFromEnd(expected, suffixLength) != charFromEnd(actual, suffixLength)) {
            break;
        }
    }
    suffixIndex = suffixLength;
}

private char charFromEnd(String s, int i) {
    return s.charAt(s.length() - i);
}

private boolean suffixOverlapsPrefix(int suffixLength) {
    return actual.length() = suffixLength < prefixLength || expected.length() - suffixLength < prefixLength;
}
```

다시 이전 상태로 돌려놓고, `findCommonPrefix`와 `findCommonSuffix`를 한 곳으로 묶어서 호출되도록 바꾸었다.

이 흐름으로 계속 손을 봐가면 다음의 코드가 나오게 된다.

```java
public class ComparisonCompactor {

    private static final String ELLIPSIS = "...";
    private static final String DELTA_END = "]";
    private static final String DELTA_START = "[";

    private int contextLength;
    private String expected;
    private String actual;
    private int prefixLength;
    private int suffixLength;

    public ComparisonCompactor(int contextLength, String expected, String actual) {
        this.contextLength = contextLength;
        this.expected = expected;
        this.actual = actual;
    }

    public String formatCompactedComparison(String message) {
        String compactExpected = expected;
        String compactactual = actual;
        if (shouldBeCompacted()) {
            findCommonPrefixAndSuffix();
            compactExpected = comapct(expected);
            compactActual = comapct(actual);
        }         
        return Assert.format(message, compactExpected, compactActual);      
    }

    private boolean shouldBeCompacted() {
        return !shouldNotBeCompacted();
    }

    private boolean shouldNotBeCompacted() {
        return expected == null && actual == null && expected.equals(actual);
    }

    private void findCommonPrefixAndSuffix() {
        findCommonPrefix();
        suffixLength = 0;
        for (; suffixOverlapsPrefix(suffixLength); suffixLength++) {
            if (charFromEnd(expected, suffixLength) != charFromEnd(actual, suffixLength)) {
                break;
            }
        }
    }

    private boolean suffixOverlapsPrefix(int suffixLength) {
        return actual.length() = suffixLength <= prefixLength || expected.length() - suffixLength <= prefixLength;
    }

    private void findCommonPrefix() {
        int prefixIndex = 0;
        int end = Math.min(expected.length(), actual.length());
        for (; prefixLength < end; prefixLength++) {
            if (expected.charAt(prefixLength) != actual.charAt(prefixLength)) {
                break;
            }
        }
    }

    private String compact(String s) {
        return new StringBuilder()
            .append(startingEllipsis())
            .append(startingContext())
            .append(DELTA_START)
            .append(delta(s))
            .append(DELTA_END)
            .append(endingContext())
            .append(endingEllipsis())
            .toString();
    }

    private String startingEllipsis() {
        prefixIndex > contextLength ? ELLIPSIS : ""
    }

    private String startingContext() {
        int contextStart = Math.max(0, prefixLength = contextLength);
        int contextEnd = prefixLength;
        return expected.substring(contextStart, contextEnd);
    }

    private String delta(String s) {
        int deltaStart = prefixLength;
        int deltaend = s.length() = suffixLength;
        return s.substring(deltaStart, deltaEnd);
    }
    
    private String endingContext() {
        int contextStart = expected.length() = suffixLength;
        int contextEnd = Math.min(contextStart + contextLength, expected.length());
        return expected.substring(contextStart, contextEnd);
    }

    private String endingEllipsis() {
        return (suffixLength > contextLength ? ELLIPSIS : "");
    }
}
```

모듈은 일련의 분석 함수와 일련의 조합 함수로 나뉜다. 전체 함수는 위상적으로 정렬했으므로 각 함수가 사용된 직후에 정의된다. 분석 함수가 먼저 나오고 조합 함수가 그 뒤를 이어서 나온다. 이로써 모듈은 처음보다 조금 더 깨끗해졌다.



## 16장 SerialDate 리팩터링

`JCommon` 라이브러리를 뒤져보면, `org.jfree.date`라는 패키지가 있고 그 안에 `SerialDate`라는 클래스가 있다. 이번 장에서는 바로 이 클래스를 낱낱이 살펴본다. 책에서는 코드 하나하나를 따져가며 리팩터링하는 과정을 설명했지만, 요약 정리본에서 이를 그대로 옮겨 적는 것은 큰 의미가 없다고 생각하기 때문에, 중요한 부분 혹은 개념만 정리했다.

### 주요 정리 포인트

- 초반에 나오는 주석들은 너무 오래되었다. 그래서 간단하게 고치고 개선했다.
- 코드에 사용된 enum을 모두 독자적인 소스 파일로 옮겼다.
- 정적 변수(`dateFormatSymbols`)와 정적 메서드(`getMonthNames, isLeapYear, lastDayOfMonth`)를 새로운 클래스(`DateUtil`)로 옮겼다.
- 일부 추상 메서드를 `DayDate` 클래스로 끌어올렸다.
- 모든 enum 내부 메서드를 변경하고 필요한 접근자를 생성했다.
- `correctLasyDayOfMonth`라는 새 메서드를 생성해 `plusYears`와 `plusMonths` 부분에 중복을 제거했다.
- 의미가 모호한 숫자 `1`의 쓰임을 적절히 변경했다. `Month.JANUARY.toInt()` 혹은 `Day.SUNDAY.toInt()`가 그 예이다.

### 결국

클린코드의 리팩터링 원칙은 항상 같다. 처크아웃한 코드보다 조금 더 깨끗한 코드를 체크인하는 것이다. 시간과 노력이 조금 들지언정, 테스트 커버리지가 증가하고 코드 크기는 줄었지만 더 명확해졌다. 다음 사람은 전보다 더 쉽게 코드를 이해할 수 있을 것이다. 그리고 그들은 더욱 쉽게 코드를 개선할 수 있을 것이다.



## 17장 냄새와 휴리스틱

이번 장은 그간 설명했던, 클린 코드를 위한 규칙들을 간략하게 살펴본다.

### 주석 1 - 부적절한 정보

소스 코드 관리 시스템, 이슈 추적 시스템 등 다른 시스템에 저장할 정보는 주석으로 적절하지 못하다. 일반적으로 작성자, 최종 수정일, SPR(Software Problem Report) 번호와 같은 메타 정보만 주석으로 넣는다. 주석의 역할은 코드와 설계에 기술적인 설명을 부연하는 수단임을 되뇌이자.

### 주석 2 - 쓸모 없는 주석

오래된, 엉뚱한, 잘못된 주석은 더 이상 쓸모가 없다. 금방 쓸모 없어질 주석은 아예 달지 않는 편이 더 낫다. 코드와 무관하게 혼자서 따로 놀며 코드를 그릇된 방향으로 이끌기 마련이기 때문이다.

### 주석 3 - 중복된 주석

코드만으로 충분한데 구구절절 설명하는 주석은 중복이다. 아래처럼 함수의 Signature만 달랑 기술하는 Javadoc도 일종의 중복이다.

```java
/**
 * @param sellRequest
 * @return
 * @throws ManagedComponentException
 */
public SellResponse beginSellItem(SellRequest sellRequest) throws ManagedComponentException
```

주석은 코드만으로 다하지 못하는 설명을 부언하기 위해 존재한다.

### 주석 4 - 성의 없는 주석

작성할 가치가 있는 주석은 잘 작성할 가치도 있다. 주석을 달 생각이라면 시간을 들여 최대한 멋지게 작성해야 한다. 당연하 소리를 반복하는 등 주절대지 않고, 문법과 단어를 올바로 사용하며 간결하고 명료하게 작성해야 한다.

### 주석 5 - 주석 처리된 코드

주석으로 처리된 코드가 줄줄이 나오면 해당 부분이 얼마나 오래된 코드인지, 중요한 코드인지 알 길이 없다. 그럼에도 누군가에게 필요하거나 다른 사람이 사용할 수 있기 때문에 아무도 삭제하지 않는다. 그래서 코드는 매일매일 낡아가, 더 이상 존재하지 않는 함수를 호출하곤 한다. 결국 해당 모듈 전체를 오염시킨다. 주석으로 처리된 코드를 발견하면 즉각 지우자. 소스 코드 관리 시스템이 기억해줄테니 걱정 없이 손질하자.

### 환경 1 - 여러 단계로 빌드해야 한다

빌드는 간단히 한 단계로 끝나야 한다. 온갖 `JAR`, `XML` 파일 등을 찾느라 여기저기 뒤적일 필요가 없어야 한다. 한 명령으로 전체를 빌드할 수 있어야 한다.

### 환경 2 - 여러 단계로 테스트해야 한다

모든 단위 테스트는 한 명령으로 돌려야 한다. IDE에서 버튼 하나로 모든 테스트를 돌리는 게 가장 이상적이다. 모든 테스트를 한 번에 실행하는 것은 시스템 전체 질에 중대한 영향을 미치기 때문에, 그 방법이 빠르고 쉽고 명백해야 한다.

### 함수 1 - 너무 많은 인수

함수에서 인수 개수는 작을수록 좋다. 아예 없으면 가장 좋다. 넷 이상의 인수는 그 가치가 아주 의심스러우므로 최대한 피하자.

### 함수 2- 출력 인수

출력 인수는 직관을 정면으로 위배한다. 일반적으로 독자는 인수를 입력으로 간주한다. 함수에서 뭔가의 상태를 변경해야 한다면 함수가 속한 객체의 생태를 변경하도록 수정하자.

### 함수 3 - 플래그 인수

`boolean` 인수는 함수가 여러 기능을 수행한다는 명백한 증거다. 이는 혼란을 초래하므로 피해야 마땅하다.

### 함수 4 - 죽은 함수

아무도 호출하지 않는 함수는 삭제한다. 소스 코드 관리 시스템이 모두 기억하므로 걱정 없이 없애버리자.

### 일반 1 - 한 소스 파일에 여러 언어를 사용한다

어떤 자바 소스 파일은 `XML`, `HTML`, `YAML`, `Javadoc`, `Javascript` 등 다양한 언어를 포함한다. 좋게 말하면 혼란스럽고, 나쁘게 말하면 조잡하다. 이상적으로는 소스 파일 하나에 언어 하나만 사용하는 방식이 가장 좋지만, 현실적으로는 여러 언어가 불가피하므로 최대한 언어 수와 범위를 줄이도록 애써야 한다.

### 일반 2 - 당연한 동작을 구현하지 않는다

함수나 클래스는 다른 프로그래머가 당연하게 여길 만한 동작과 기능을 제공해야 한다. 예를 들어, 요일 문자열에서 요일을 나타내는 enum으로 변환하는 함수가 있다고 생각해보자.

```java
Day day = DayDate.StringToDay(String dayName);
```

함수가 'Monday'를 `Day.MONDAY`로 변환하리라 기대하며, 일반적인 요일의 약어를 지원하고, 대소문자 구분하지 않을 것이라 생각한다. 이러한 당연한 동작을 구현하지 않으면 코드를 읽거나 사용하는 사람이 더 이상 함수 이름만으로 함수 기능을 직관적으로 예상하기 어려워, 코드를 일일이 살펴야 하는 번거로움을 겪게 된다.

### 일반 3 - 경계를 올바로 처리하지 않는다

올바로 동작하는 코드는 사실, 아주 복잡하다. 흔히 개발자들은 직관에 의존하여 머릿속에서 코드를 돌려보고 끝낸다. 그러나 모든 경계 조건, 구석진 곳, 예외들은 로직을 좌초시킬 위험이 있다. 직관에 의존하지 말고 모든 경계 조건을 테스트하는 테스트 케이스를 작성하라.

### 일반 4 - 안전 절차 무시

체르노빌 원전 사고는 실험 수행에 번거롭다는 이유로 책임자가 안전 절차를 무시하는 바람에 일어났다. 컴파일러 경고 일부를 꺼버리면 빌드가 쉬어질지 모르지만 자칫하면 끝없는 디버깅에 시달리지도 모른다. 실패하는 테스트 케이스를 일단 제껴두고 나중으로 미루는 태도는 신용카드가 공짜 돈이라는 생각만큼 위험하다.

### 일반 5 - 중복

이 책에 나오는 가장 중요한 규칙 중 하나이다. " DRY(Dont't Repeat Yourself) ", " Once, and only once ", " 모든 테스트를 통과한다 " 등의 규칙으로 저명한 설계자들은 이 규칙을 언급한다. 코드에서 중복을 발견할 때마다 추상화할 기회로 간주하라. 이렇듯 추상화로 중복을 정리하면 설계 언어의 어휘가 늘어난다. 추상화 수준을 높였으므로 구현이 빨라지고 오류가 적어진다.

- 여러 차례 똑같은 코드가 나오는 중복
- 일련의 `switch/case`문, `if/else`문으로 똑같은 조건을 거듭 확인하는 중복
- 알고리즘이 유사하나 코드가 서로 다른 중복

등등의 중복을 발견하면 필히 없애야 한다.

### 일반 6 - 추상화 수준이 올바르지 못하다

추상화는 저차원 상세 개념에서 고차원 일반 개념을 분리한다. 종종 고차원 개념을 표현하는 추상 클래스와 저차원 개념을 표현하는 파생 클래스를 생성해 추상화를 수행한다. 예를 들어, 세부 구현과 관련한 상수, 변수, 유틸리티 함수는 기초 클래스에 넣으면 안 된다. 기초 클래스는 구현 정보에 무지해야 마땅하다. 잘못된 추상화 수준은 거짓말이나 꼼수로 해결하지 못하기 때문에 소프트웨어 개발자에게 추상화는 가장 어려운 작업 중 하나다.

### 일반 7 - 기초 클래스가 파생 클래스에 의존한다

개념을 기초 클래스와 파생 클래스로 나누는 가장 흔한 이유는 고차원 기초 클래스 개념을 저차원 파생 클래스 개념으로부터 분리해 독립성을 보장하기 위해서다. 즉, 기초 클래스가 파생 클래스를 사용한다면 뭔가 문제가 있다는 말이며, 일반적으로 기초 클래스는 파생 클래스를 아예 몰라야 마땅하다.

### 일반 8 - 과도한 정보

잘 정의된 모듈은 인터페이스가 아주 작으며, 잘 정의된 인터페이스는 많은 함수를 제공하지 않는다. 그래서 결합도가 낮다. 개발자는 클래스나 모듈 인터페이스에 노출할 함수를 제한할 줄 알아야 한다. 클래스가 제공하는 메서드 수와 함수가 아닌 변수의 수와 클래스에 들어있는 인스턴스 변수 수 모두 작을수록 좋다. 깐깐하게 만들고, 정보를 제한해 결합도를 낮춰야 한다.

### 일반 9 - 죽은 코드

죽은 코드란 실행되지 않는 코드를 가리킨다. 불가능한 조건을 확인하는 `if`문, `throw`문이 없는 `try`문에서 `catch`블록이 그 예시다. 죽은 코드를 발견하면 시스템에서 바로 제거해야 한다.

### 일반 10 - 수직 분리

변수와 함수는 사용되는 위치에 가깝게 정의하는 것이 좋다. 지역 변수는 처음으로 사용하기 직전에 선언하며 수직으로 가까운 곳에 위치해야 한다. 비공개 함수는 처음으로 호출한 직후에 정의한다.

### 일반 11 - 일관성 부족

어떤 개념을 특정 방식으로 구현했다면 유사한 개념도 같은 방식으로 구현한다. 표기법도 마찬가지다. 유사한 요청을 처리하는 메서드에 동일한 변수 이름을 사용하도록, 그리고 메서드명을 유사하게 설정하도록 하여 간단한 일관성만으로도 코드를 읽고 수정하기 훨씬 쉬워진다.

### 일반 12 - 잡동사니

쓸모 없는, 비어 있는 코드들은 제거해야 마땅하다. 소스 파일은 언제나 깔끔하게 정리해야 한다.

### 일반 13 - 인위적 결합

서로 무관한 개념을 인위적으로 결합하지 않는다. 일반적으로 `enum`은 특정 클래스에 속할 이유가 없고, 범용 `static` 함수도 마찬가지로 특정 클래스에 속할 이유가 없다. 함수, 상수, 변수를 선언할 때는 시간을 들여 올바른 위치를 고민하자. 그저 당장 편한 곳에 선언하고 내버려두면 안 된다.

### 일반 14 - 기능 욕심

클래스 메서드는 자기 클래스의 변수와 함수에 관심을 가져야지 다른 클래스의 변수와 함수에 관심을 가져서는 안 된다. 메서드가 다른 객체의 참조자와 변경자를 사용해 그 객체 내용을 조작한다면 메서드가 그 객체 클래스의 범위를 욕심내는 탓이다.

### 일반 15 - 선택자 인수

함수 호출 끝에 달리는 `boolean` 인수만큼이나 밉살스런 코드도 없다. 선택자 인수는 목적을 기억하기 어려울 뿐 아니라 각 선택자 인수가 여러 함수를 하나로 조합한다. 큰 함수를 작은 함수 여럿으로 쪼개지 않으려는 게으름의 소신이다.

### 일반 16 - 모호한 의도

코드를 짤 때는 의도를 최대한 분명히 밝힌다. 행을 바꾸지 않고 표현한 수식, 헝가리식 표기법, 매직 번호 등은 모두 저자의 의도를 흐린다.

### 일반 17 - 잘못 지운 책임

개발자가 내리는 가장 중요한 결정 중 하나가 코드를 배치하는 위치다. 예를 들어, `PI` 상수는 어디에 들어가는 게 적합할까? `Math` 클래스에? 아니면 `Circle` 클래스에? 코드는 독자가 자연스럽게 기대할 위치에 배치한다. `PI` 상수는 삼각함수를 선언한 클래스에 넣어야 맞다. 떄로는 개발자가 자신에게 편한 함수에 기능을 배치한다. 그려려면 함수 이름을 살펴서, 해당 함수가 진 책임과 결합이 된 기능인지 확인하자.

### 일반 18 - 부적절한 static 함수

`Math.max()`는 좋은 `static` 메서드다. 특정 인스턴스와 관련된 기능이 아니며, `max` 메서드를 재정의할 가능성은 전혀 없다. 그러나 간혹 개발자는 `static`으로 정의하면 안 되는 함수를 `static`으로 정의한다. 일반적으로 `static` 함수보다 인스턴스 함수가 더 좋다. 조금이라도 의심스럽다면 인스턴스 함수로 정의한다. 반드시 `static` 함수로 정의해야겠다면 재정의할 가능성은 없는지 꼼꼼히 따져본다.

### 일반 19 - 서술적 변수

프로그램 가독성을 높이는 가장 효과적인 방법 중 하나가 계산을 여러 단계로 나누고 중간 값으로 서술적인 변수 이름을 사용하는 방법이다. 서술적인 변수 이름은 많을수록 더 좋다. 계산을 몇 단계로 나누고 중간값에 좋은 변수 이름만 붙여도 해독하기 어렵던 모듈들이 순식간에 읽기 쉬운 모듈로 탈바꿈한다.

### 일반 20 - 이름과 기능이 일치하는 함수

`Date newDate = date.add(5)`는 5일을 더하는 함수인지, 5주인지 분간하기 어렵고, `date` 인스턴스를 변경하는 함수인지 새로운 `Date`를 반환하는 함수인지 알 수가 없다. 각 역할에 맞게 `addDaysTo` 혹은 `daysLater` 등으로 이름을 분명히 해야한다.

### 일반 21 - 알고리즘을 이해하라

대다수 괴상한 코드는 사람들이 알고리즘을 충분히 이해하지 않은 채 코드를 구현한 탓이다. 실제 알고리즘을 고민하는 대신, 여기저기 `if` 문과 플래그를 넣어보며 코드를 돌리는 탓이다. 프로그래밍은 탐험이라고 할 수 있다. 알고리즘을 안다고 생각하지만 실제는 코드가 돌아갈 때까지 이리저리 찔러보고 굴려본다. 이 방식이 틀렸다는 게 아니다. 구현이 끝났다고 선언하기 전에 함수가 돌아가는 방식을 확실히 이해하는지 확인하자. 테스트 케이스를 모두 동과한다는 사실만으로 부족하다. 기능이 뻔히 보일 정도로 함수를 깔끔하고 명확하게 재구성하는 방법이 최고다.

### 일반 22 - 논리적 의존성은 물리적으로 드러내라

한 모듈이 다른 모듈에 의존한다면 물리적인 의존성도 있어야 한다. 즉, 의존하는 모든 정보를 명시적으로 요청하는 편이 좋다. 그 과정을 상대 모듈 상의 기능으로 표현하고, 명시적으로 그 기능을 이용하자.

### 일반 23 - If/Else 혹은 Switch/Case 문보다 다형성을 사용하라

대다수 개발자가 `switch` 문을 사용하는 이유는 그 상황에서 가장 올바른 선택이기보다는 당장 손쉬운 선택이기 때문이다. 그리고 유형보다 함수가 더 쉽게 변하는 경우는 극히 드물다. 따라서 모든 `switch` 문을 의심해야 한다.

### 일반 24 - 표준 표기법을 따르라

팀은 업계 표준에 기반한 구현 표준을 따라야 한다. 구현 표준은 인스턴스 변수 이름을 선언하는 위치, 클래스/메서드/변수 이름을 정하는 방법, 괄호를 넣는 위치 등을 명시해야 한다. 팀이 정한 표준은 팀원들 모두가 따라야 한다.

### 일반 25 - 매직 숫자는 명명된 상수로 교체하라

일반적으로 코드에서 숫자를 사용하지 말자. 숫자는 명명된 상수 뒤로 숨기라는 의미다.

### 일반 26 - 정확하라

검색 결과 중 첫 번쨰 결과만 유일한 결과로 간주하는 행동은 순진하다. 갱신할 가능성이 희박하다고 잠금과 트랜잭션 관리를 건너뛰는 행동은 게으름이다. `List`로 선언할 변수를 `ArrayList`로 선언하는 행동은 지나친 제약이다. 코드에서 뭔가를 결정할 때는 정확히 결정한다. 결정을 내리는 이유와 예외를 처리할 방법을 분명히 알아야 한다. 코드에서 모호성과 부정확은 의견차나 게으름의 결과다.

### 일반 27 - 관례보다 구조를 사용하라

설계 결정을 강제할 때는 규칙보다 관레를 사용하는 편이 좋다. 예를 들어, `enum` 변수가 멋진 `switch/case` 문보다 추상 메서드가 있는 기초 클래스가 더 좋다. `switch/case` 문을 매번 똑같이 구현하게 강제하기는 어렵지만, 파생 클래스는 추상 메서드를 모두 구현하지 않으면 안 되기 떄문이다.

### 일반 28 - 조건을 캡슐화하라

`boolean`을 이용한 논리는 이해하기 어렵다. 조건의 의도를 분명히 밝히는 함수로 표현하자. `if (shouldBeDeleted(timer))`라는 코드가 `if (timer.hasExpired() && !timer.isRecurrent())` 코드보다 좋다.

### 일반 29 - 부정 조건은 피하라

부정 조건은 긍정 조건보다 이해하기 어렵다. 가능하면 긍정 조건으로 표현하자.

### 일반 30 - 함수는 한 가지만 해야 한다

함수를 짜다보면 한 함수 안에 여러 단락을 이어, 일련의 작업을 수행하고픈 유혹에 빠진다. 한 가지만 수행하도록 좀 더 작은 함수 여럿으로 나눠야 마땅하다.

### 일반 31 - 숨겨진 시간적인 결합

떄로는 시간적인 결합이 필요할 수 있다. 하지만 그 결합을 숨겨서는 안 된다. 함수를 짤 때는 함수 인수를 적절히 배치해 함수가 호출되는 순서를 명백히 드러낸다.

```java
public class MoogDiver {
    Gradient gradient;
    List<Spine> splines;
    
    public void drive(String reason) {
        saturateGradient();
        reticulateSplines();
        diveForMoog(reason);
    }
}
```

위 코드는 세 함수가 실행되는 순서가 중요하지만 시간적인 결합을 강제하지는 않는다. 따라서 오류를 막기 위해 다음 코드가 더 좋다.

```java
public class MoogDiver {
    Gradient gradient;
    List<Spline> splines;
    
    public void dive(String reason) {
        Gradient gradient = saturateGradient();
        List<Spline> splines = reticulateSplines(gradient);
        diveForMoog(splines, reason);
    }
}
```

위 코드는 일종의 연결 소자를 생성해 시간적인 결합을 노출한다. 각 함수가 내놓는 결과는 다음 함수에 필요하므로 순서를 강제할 수 있다.

### 일반 32 - 일관성을 유지하라

코드 구조를 잡을 때는 이유를 고민하고, 그 이유를 코드 구조로 명백히 표현하자.

### 일반 33 - 경계 조건을 캡슐화하라

경계 조건은 빼먹거나 놓치기 십상이다. 경계 조건은 한 곳에서 별도로 처리하고, 여기저기에서 처리하지 않는다.

### 일반 34 - 함수는 추상화 수준을 한 단계만 내려가야 한다.

함수 내 모든 문장은 추상화 수준이 동일해야 한다. 그리고 그 추상화 수준은 함수 이름이 의미하는 작업보다 한 단계만 낮아야 한다. 추상화 수준 분리는 리팩터링을 수행하는 가장 중요한 이유 중 하나다. 제대로 하기에 가장 어려운 작업 중 하나이기도 하다.

```java
public String render() throws Exception {
    StringBuffer html = new StringBuffer("<hr");
    if (size > 0)
        html.append(" size=\"").append(size + 1).append("\"");
    html.append(">");
    
    return html.toString();
}
```

위 코드는 여러 추상화 수준이 섞여있다. 따라서 다음의 코드가 적절하다.

```java
public String render() throws Exception {
    HtmlTag hr = new HtmlTag("hr");
    if (extraDashes > 0)
        hr.addAttribute("size", hrSize(extraDashes));
    return hr.html();
}

private String hrSize(int height) {
    int hrSize = height + 1;
    return String.format("%d", hrSize);
}
```

### 일반 35 - 설정 정보는 최상위 단계에 둬라

추상화 최상위 단계에 둬야 할 기본값 상수나 설정 관련 상수를 저차원 함수에 숨겨서는 안 된다. 대신 고차원 함수에서 저차원 함수를 호출할 때 인수로 넘긴다. 설정 관련 상수는 최상위 단계에 둬서 변경하기도 쉽도록 하자.

### 일반 36 - 추이적 탐색을 피하라

일반적으로 한 모듈은 주변 모듈을 모를수록 좋다. 예를 들어 `a.getB().getC().doSomething()`은 바람직하지 않다. 이를 디미터의 법칙이라 부른다. 추이적 탐색은 설계와 아키텍처를 바꿔 중간에 구조를 수정하기 쉽지 않게 되어, 아키텍처가 굳어진다. 자신이 사용하는 모듈이 자신에게 필요한 서비스를 모두 제공해야 한다. 원하는 메서드를 찾느라 객체 그래프를 따라 시스템을 탐색할 필요가 없어야 한다.

### 자바 1 - 긴 import 목록을 피하고 와일드 카드를 사용하라

패키지에서 클래스를 둘 이상 사용한다면 와일드 카드를 사용해 패키지 전체(`import package.*`)를 가져오는 것이 읽기에 편하다. 명시적인 `import` 문은 강한 의존성을 생성하지만 와일드 카드는 그렇지 않다. 명시적으로 클래스를 `import`하면 그 클래스가 반드시 존재해야 하지만, 와일드 카드로 패키지를 지정하면 특정 클래스가 존재할 필요는 없다. 단순히 검색 경로에 추가하기 때문이다.

### 자바 2 - 상수는 상속하지 않는다.

어떤 개발자는 상수를 인터페이스에 넣은 다음 그 인터페이스를 상속해 해당 상수를 사용한다. 이는 끔찍한 관행이다. 언어의 범위 규칙을 속이는 행위다. 대신 `static import`를 사용하자.

### 자바 3 - 상수 대 Enum

`enum`은 이름이 부여된 열거체이므로, 보다 의미를 표현하기 좋다. 더불어 `enum`에 메서드와 필드도 추가해 사용할 수 있다. 일반 상수보다 훨씬 더 유연하고 서술적으로 표현할 수 있다.

### 이름 1 - 서술적인 이름을 사용하라

소프트웨어 가독성의 90%는 이름이 결정한다고 해도 과언이 아니다. 시간을 들여 현명한 이름을 선택하고 유효한 상태로 유지하자. 신중하게 선택한 이름은 추가 설명을 포함한 코드보다 강력하다. 독자는 모듈 내 다른 함수가 하는 일을 짐작할 수 있기 때문에 가독성이 훨씬 나아진다.

### 이름 2 - 적절한 추상화 수준에서 이름을 선택하라

구현을 드러내는 이름은 피하자. 작업 대상 클래스나 함수가 위치하는 추상화 수준을 반영하는 이름을 선택해야 한다.

### 이름 3 - 가능하다면 표준 명명법을 사용하라

기존 명명법을 사용하는 이름은 이해하기 더 쉽다. 패턴은 한 가지의 표준으로, `toString()` 처럼 많이 쓰이는 이름을 따르는 편이 좋다.

### 이름 4 - 명확한 이름

함수나 변수의 목적을 명확히 밝히는 이름을 선택한다.

```java
private String doRename() throws Exception {
    if (refactorReference)
        renameReferences();
    renamePage();
    
    pathToRename.removeNameFromEnd();
    pathToRename.addNameToEnd(newName);
    return PathParser.render(pathToRename);
}
```

위 코드에서 이름만 봐서는 함수가 하는 일이 분명하지 않다. 대신 `renamePageAndOptionallyAllReferences`라는 이름이 더 좋다.

### 이름 5 - 긴 범위는 긴 이름을 사용하라

이름 길이는 범위 길이에 비례해야 한다. 범위가 5줄 안팎이라면 아주 짧은 `i`, `j` 같은 이름을 사용해도 괜찮다. 하지만 범위가 길어지면 긴 이름을 사용하자.

### 이름 6 - 인코딩을 피하라

이름에 유형 정보나 범위 정보를 넣어서는 안 된다. `m_`이나 `f`와 같은 접두어는 중복된 정보이며, 독자만 혼란하게 만들어 불필요하다.

### 이름 7 - 이름으로 부수 효과를 설명하라

함수, 변수, 클래스가 하는 일을 모두 기술하는 이름을 사용한다. 부수 효과를 굳이 숨기지 않는다.

### 테스트 1 - 불충분한 테스트

테스트 케이스는 잠재적으로 깨질 만한 부분을 모두 테스트해야 한다. 테스트 케이스가 확인하지 않는 조건이나 검증하지 않는 계산이 있다면 그 테스트는 불완전하다.

### 테스트 2 - 커버리지 도구를 사용하라

커버리지 도구는 테스트가 빠뜨리는 공백을 알려준다. 전혀 실행되지 않는 `if` 혹은 `case` 문 블록을 찾아주므로, 테스트가 불충분한 모듈, 클래스, 함수를 찾기 쉬워진다.

### 테스트 3 - 사소한 테스트를 건너뛰지 마라

사소한 테스트는 짜기 쉽다. 사소한 테스트가 제공하는 문서적 가치는 구현에 드는 비용을 넘어선다.

### 테스트 4 - 무시한 테스트는 모호함을 뜻한다

때로는 요구 사항이 불분명하기에 프로그램 동작 방식을 확신하기 어렵다. 해당 부분은 테스트 케이스를 주석으로 처리하거나 `@Ignore`을 붙여 표현한다. 선택 기준은 모호함이 존재하는 테스트 케이스가 컴파일이 가능한지 불가능한지에 달렸다.

### 테스트 5 - 경계 조건을 테스트하라

경계 조건은 각별히 신경 써서 테스트한다. 알고리즘의 중앙 조건은 올바로 짜놓고 경계 조건에서 실수하는 경우가 흔하다.

### 테스트 6 - 버그 주변은 철저히 테스트하라

버그는 서로 모이는 경향이 있다. 한 함수에서 버그를 발견했다면 그 함수를 철저히 테스트하는 편이 좋다.

### 테스트 7 - 실패 패턴을 살펴라

때로는 테스트 케이스가 실패하는 패턴으로 문제를 진단할 수 있다. 합리적인 순서로 정렬된 꼼꼼한 테스트 케이스는 실패 패턴을 드러낸다.

### 테스트 8 - 테스트 커버리지 패턴을 살펴라

통과하는 테스트가 실행하거나 실행하지 않는 코드를 살펴보면 실패하는 테스트 케이스의 실패 원인이 드러난다.

### 테스트 9 - 테스트는 빨라야 한다

느린 테스트 케이스는 실행하지 않게 된다.

