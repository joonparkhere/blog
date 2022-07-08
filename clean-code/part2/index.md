---
title: Clean Code 4~9장
date: 2021-05-07
pin: false
tags:
- Software Enginerring
- Clean Code
- Java
---

# 클린 코드 4~9장 요약 정리



## 4장 **주석**

**잘 달린** 주석은 그 어떤 정보보다 유용합니다. 그러나 그 중요성만큼, 근거 없는 주석은 코드를 이해하기 어렵게 만듭니다. 오래되고 조잡한 주석은 거짓과 잘못된 정보를 퍼뜨릴 수 있습니다. 본문에서, `주석은 `순수하게 선하지` 못하다. 사실상 주석은 기껏해야 필요악이다.`라고 말할 만큼 되도록이면 주석은 쓰지 말라고 강조합니다. 주석은 고의가 아닐지는 모르지만 대부분 옳은 설명을 덧붙이는 게 아니고, 오래될수록 완전히 그릇될 가능성이 크므로 저자는 주석을 **실패**라고 까지 생각합니다. 물론, 주석을 엄격하게 관리하여 복구성, 관련성, 정확성이 언제나 높도록 유지하면 좋지만, 그렇게 에너지를 쏟을 바에는 애초에 주석이 필요 없는 방향으로 노력하는 게 더 좋습니다.

### 주석은 나쁜 코드를 보완하지 못한다

코드에 주석을 추가하는 일반적인 이유는 코드 품질 자체가 나쁘기 때문입니다. 모듈을 짜고 보니 짜임새가 엉망이고 알아먹기 어려워서 주석이나 달아야겠다고 생각합니다. 하지만, 대부분의 경우에는, 코드를 정리하는 것이 맞습니다.

### 코드로 의도를 표현하라

```java
if ((employee.flags & HOURLY_FLAG) && employee.age > 65)
```

```java
if (employee.isEligibleForFullBenefites())
```

코드만으로 의도를 설명하기 어려운 경우가 존재할지 모르지만 예시처럼, 대다수의 경우에는 코들 의도를 표현할 수 있습니다.

### 좋은 주석

1. 법적인 주석

   때로는 회사가 정립한 구현 표준에 맞춰 법적인 이유로 특정 주석을 넣으라고 명시합니다. 저작권 정보와 소유권 정보 등이 들어갈 수 있습니다.

2. 정보를 제공하는 주석

   때로는 기본적인 정보를 주석으로 제공하면 편리합니다.

   ```java
   // 테스트 중인 Responder 인스턴스를 반환한다.
   protected abstract Responder reponderInstance();
   ```

   ```java
   protected abstract Responder reponderBeingTested();
   ```

   하지만 언제나 최대한, 코드로 의도 표현하는 것이 좋습니다.

3. 의도를 설명하는 주석

   때로는 주석으로 구현 이해를 도와주는 걸 넘어서 결정에 깔린 의도까지 설명하기도 합니다.

   ```java
   public int compareTo(Object o) {
       if (o instanceof WikiPagePath) {
           WikiPagePath p = (WikiPagePath) o;
           String compressName = StringUtil.join(names, "");
           String compressedArgumentName = StringUtil.join(p.names, "");
           return compressedName.compareTo(compressedArgumentName);
       }
       return 1;  // 오른쪽 유형이므로 정렬 순위가 더 높다.
   }
   ```

4. 의미를 명료하게 밝히는 주석

   때때로 모호한 인수나 반환값은 그 의미를 읽기 좋게 표현하면 이해하기 쉬워집니다.

   ```java
   public void testCompareTo() throws Exception {
       WikiPagePath a = PathParser.parse("PageA");
       WikiPagePath ab = PathParser.parse("PageA.PageB");
       WikiPagePath b = PathParser.parse("PageB");
       
       assertTrue(a.compareTo(a) == 0);    // a == a
       asserTrue(a.compareTo(b) != 0);     // a != b
       assertTrue(ab.compareTo(ab) == 0);  // ab == ab
   }
   ```

5. 결과를 경고하는 주석

   때로 다른 프로그래머에게 결과를 경고할 목적으로 주석을 사용합니다.

   ```java
   // 여유 시간이 충분하지 않다면 실행하지 마십시오.
   public void _testWithReallyBigFile() {
       writeLinesToFile(10000000);
       
       response.setBody(testFile);
       response.readToSent(this);
       String responseString = output.toString();
       assertSubString("Content-Loength: 100000000", responseString);
       assertTrue(bytesSent > 100000000);
   }
   ```

   물론 최근에는 `@Ignore("실행이 너무 오래 걸린다.")` 애노테이션을 이용해서 표현할 수도 있습니다.

6. TODO 주석

   앞으로 할 일을 주석으로 남겨두면 편합니다.

   ```java
   // TODO-MdM 현재 필요하지 않다.
   // 체크아웃 모델을 도입하면 함수가 필요 없다.
   protected VersionInfo makeVersion() throws Exception {
       return null;
   }
   ```

   필요하다 여기지만, 당장 구현하기 어려운 업무를 기술하곤 합니다. 하지만 어떤 용도로 사용하든 시스템에 나쁜 코드를 남겨 놓는 핑계가 되어서는 안 됩니다.

7. 중요성을 강조하는 주석

   자칫 대수롭지 않다고 여겨질 뭔가의  중요성을 강조하기 위해서도 주석을 사용합니다.

   ```java
   String listItemContent = match.group(3).trim();
   // 여기서 trim은 정말 중요하다. trim 함수는 문자열에서 시작 공백을 제거한다.
   // 문자열에 시작 공백이 있으면 다른 문자열로 인식되기 때문이다.
   new ListItemWidget(this, listItemContent, this.level + 1);
   return buildList(text.substring(match.end()));
   ```

8. 공개 API에서의 Javadocs

   설명이 잘 된 공개 API는 참으로 유용하기에, 공개 API를 구현한다면 훌륭한 Javadocs를 작성해야 합니다.

### 나쁜 주석

대다수의 주석이 이 범주에 속합니다. 허술한 코드를 지탱하거나, 엉성한 코드를 변명하거나, 미숙한 결정을 합리화하는 등 프로그래머가 주절거리는 독백에 불과합니다.

1. 주절거리는 주석

   주석을 달기로 결정했다면 충분한 시간을 들여 최고의 주석을 달도록 노력해야 합니다.

   ```java
   public void loadProperties() {
       try {
           String propertiesPath = propertiesLocation + "/" + PROPERTIES_FILE;
           FileInputStream propertiesStream = new FileInputStream(propertiesPath);
           loadedProperties.load(propertiesStream);
       } catch(IOException e) {
           // 속성 파일이 없다면 기본값을 모두 메모리로 읽어 들였다는 의미다.
       }
   }
   ```

   주석으로 주절거리는 부분은 개발자에게야 의미가 있겠지만, 다른 사람에게는 전해지지 않습니다. 무엇이, 언제, 읽어들여서 어떻게 되는 지 이해하기 힘듭니다. 결국 답을 알아내려면 다른 코드를 뒤져보는 수밖에 없습니다. 이런 주석은 바이트만 낭비할 뿐입니다.

2. 같은 이야기를 중복하는 주석

   ```java
   // this.closed가 true일 때 반환되는 유틸리티 메서드다.
   // 타임아웃에 도달하면 예외를 던진다.
   public synchronized void waitForClose(final long timeoutMillis) throws Exception {
       if (!closed) {
           wait(timeoutMillis);
           if (!closed)
               throws new Exception("MockResponseSend could not be close");
       }
   }
   ```

   이 경우 주석이 코드보다 더 많은 정보를 제공하지 않습니다. 코드를 정당화하는 것도 아니고, 의도나 근거를 설명하는 주석도 아닙니다. 오히려 코드보다 읽기가 쉽지도 않습니다.

   ```java
   public abstract class ContainerBase implements Container, Lifecycle, PipeLine, MBeanRegistration, Serializable {
       /**
        * 이 컴포넌트의 프로세서 지연값
        */
       protected int backgroundProcessorDelay = -1;
       
       /* ... */
   }
   ```

   이 경우도 쓸모없고 중복된 Javadocs가 매우 많습니다. 코드만 지저분하고 정신 없게 만들 뿐입니다.

3. 오해할 여지가 있는 주석

   ```java
   // this.closed가 true일 때 반환되는 유틸리티 메서드다.
   // 타임아웃에 도달하면 예외를 던진다.
   public synchronized void waitForClose(final long timeoutMillis) throws Exception {
       if (!closed) {
           wait(timeoutMillis);
           if (!closed)
               throws new Exception("MockResponseSend could not be close");
       }
   }
   ```

   이 주석은 중복이 상당히 많으면서도 오해할 여지가 살짝 있습니다. `this.closed`가 `true`로 변하는 순간에 메서드는 반환되는 것이 아니라, `this.closed`가 `true`여야 메서드는 반환됩니다. 이처럼 살짝 잘못된 정보로 인해, 이 코드의 주석을 보고 메서드를 사용한 어는 프로그래머는 자기 코드가 굼벵이 기어가듯 돌아가는 이유를 찾느라 골머리르 앓을 수 있습니다.

4. 의무적으로 다는 주석

   ```java
   /**
    *
    * @param title CD 제목
    * @param author CD 저자
    */
   public void addCD(String title, String author) {
       CD cd = new CD();
       cd.title = title;
       cd.author = author;
       cdList.add(cd);
   }
   ```

   모든 함수에 Javadocs를 달거나, 모든 변수에 주석을 달아야 한다는 규칙을 효율적이지 않습니다. 오히려 코드를 복잡하게 만들며, 그릇된 정보를 퍼뜨리고, 혼동과 무질서를 초래할 수 있습니다.

5. 이력을 기록하는 주석

   ```java
   /**
    * 11-Oct-2001: 클래스를 다시 정리하고 새로운 패키지인 com.jrefinery.date로 옮겼다 (DG);
    *
    * 05-Nov-2001: getDescription() 메서드를 추가했으며 NotableDate class를 제거했다 (DG);
    *
    */
   ```

   소스 코드 관리 시스템이 없던 시절에는 모든 모듈 첫머리에 변경 이력을 기록하고 관리하는 관례가 바람직 했지만, 이제는 혼란만 가중할 뿐입니다.

6. 있으나 마나 한 주석

   ```java
   /**
    * 기본 생성자
    */
   protected AnnualDateRule() {
   }
   ```

   이 경우의 주석은 너무 당연한 사실을 언급하며 새로운 정보를 제공하지 못합니다.

7. 함수나 변수로 표현할 수 있다면 주석을 달지 마라

   ```java
   // 전역 목록 <smodule>에 속하는 모듈이 우리가 속한 하위 시스템에 의존하는가?
   if (smodule.getDependSubsystems().contains(subSysMod.getSubSystem()))
   ```

   ```java
   ArrayList moduleDependees = smodule.getDependSubsystems();
   String ourSubSystem = subSysMod.getSubSystem();
   if (moduleDependees.contains(ourSubSystem))
   ```

   위와 같이 주석이 필요하지 않도록 코드를 개선하는 편이 더 좋습니다.

8. 공로를 돌리거나 저자를 표시하는 주석

   소스 코드 관리 시스템은 누가 언제 무엇을 추가했는지 모두 제공합니다. 저자 이름으로 코드를 오염시킬 필요가 없습니다. 이런 주석은 그냥 오랫동안 코드에 방치되어 점차 부정확하고 쓸모없는 정보로 변하기 쉽습니다.

9. 주석으로 처리한 코드

   주석으로 처리한 코드는 절대 작성하지 않아야 합니다.

   ```java
   InputStreamResponse response = new InputStreamResponse();
   response.setBody(formatter.getResultStream(), formatter.getByteCount());
   // InputStream resultsStream = formatter.getResultStream();
   // StreamReader reader = new StreamRead(resultsStream);
   // response.setContent(reader.read(formatter.getByteCount()));
   ```

   주석으로 처리된 코드는 혹여나 하는 마음에 다른 사람들이 지우기를 주저합니다. 그래서 쓸모 없는 코드가 점차 쌓여 갑니다.

10. 전역 정보를 나타내는 주석

    주석을 달아야 한다면 근처에 있는 코드만 기술해야 합니다. 코드 일부에 주석을 달면서 시스템의 전반적인 정보를 기술하면 안 됩니다.

    ```java
    /**
     * 적합성 테스트가 동작하는 포트: 기본값은 <b>8082</b>.
     *
     * @param fitnessePort
     */
    public void setFitnessePort(int fitnessePort) {
        this.fitnessePort = fitnessePort;
    }
    ```

11. 비공개 코드에서 Javadocs

    공개 API는 Javadocs가 유용하지만 공개하지 않을 코드라면 쓸모가 없습니다. 유용하지 않을 뿐만 아니라 Javadocs 주석이 요구하는 형식으로 인해 코드만 보기 싫고 산만해질 뿐입니다.

### 예제

```java
/**
 * 이 클래스는 사용자가 지정한 최대 값까지 소수를 생성한다. 사용한 알고리즘은 에라스토테네스의 체다.
 * <p>
 * 에라스토테네스: 기원전 276년 ...(후략)
 * </p>
 * 알고리즘은 상당히 단순하다. 2에서 시작하는 정수 배열을 대상으로 2의 배수를 모두 제거한다.
 * 다음으로 남은 정수를 찾아 이 정수의 배수를 모두 지운다. 최대 값의 제곱근이 될 때까지 이를 반복한다.
 *
 * @author Alphonse
 * @version 13 Feb 2002 atp
 */
import java.util.*;

public class GeneratePrimes {
    /*
     * @param maxValue는 소수르 찾아낼 최대 값
     */
     public static int[] generatePrimes(int maxValue) {
         if (maxValue >= 2) {  // 유일하게 유요한 경우
             // 선언
             int s = maxValue + 1;  // 배열 크기
             boolean[] f = new booleans[s];
             int i;
             
             // 배열을 참으로 초기화
             for (i = 0; i < s; i++)
                 f[i] = true;
             
             // 소수가 아닌 알려진 숫자를 제거
             f[0] = f[1] = false;
             
             // 체
             int j;
             for (i = 2; i < Math.sqrt(s) + 1; i++) {
                 if (f[i]) {  // i가 남아 있는 숫자라면 이 숫자의 배수를 구한다.
                     for (j = 2 * i; j < s; j += i)
                         f[j] = false;  // 배수는 소수가 아니다.
                 }
             }
             
             // 소수 개수는?
             int count = 0;
             for (i = 0; i < s; i++) {
                 if (f[i])
                     count++;  // 카운트 증가
             }
             
             int[] primes = new int[count];
             
             // 소수를 결과 배열로 이동한다.
             for (i = 0, j = 0; i < s; i++) {
                 if (f[i])  // 소수일 경우에
                     primes[j++] = i;
             }
             
             return primes;  // 소수를 반환한다.
         }
         else // maxValue < 2
                 return new int[0];  // 입력이 잘못되면 비어 있는 배열을 반환한다.
     }
}
```

위 코드를 리팩터링한 결과가 아래 코드 입니다.

```java
/**
 * 이 클래스는 사용자가 지정한 최대 값까지 소수를 구한다.
 * 알고리즘은 에라스토테네스의 체다.
 * 2에서 시작하는 정수 배열을 대상으로 작업한다.
 * 처음으로 남아 있는 정수를 찾아 배수를 모두 제거한다.
 * 배열에 더 이상 배수가 없을 때까지 반복한다.
 */
public class PrimeGenerator {
    private static boolean[] crossedOut;
    private static int[] result;
    
    public static int[] generatePrimes(int maxValue) {
        if (maaxValue < 2)
            return new int[0];
        else {
            uncrossIntegersUpTo(maxValue);
            crossOutMultiples();
            putUncrossedIntegerIntoResult();
            return result;
        }
    }
    
    private static void uncrossIntegersUpTo(int maxValue) {
        crossedOut = new boolean[maxValue + 1];
        for (int i = 2; i < crossedOut.length; i++)
            crossedOut[i] = false;
    }
    
    private static void crossOutMultiples() {
        int limit = determineIterationLimit();
        for (int i = 2; i <= limit; i++)
            if (notCrossed(i))
                crossOutMultiplesOf(i);
    }
    
    private static int determineIterationLimit() {
        // 배열에 있는 모든 배수는 배열 크기의 제곱근보다 작은 소수의 인수다.
        // 따라서 이 제곱근보다 더 큰 숫자의 배수는 제거할 필요가 없다
        dobule iterationLimit = Math.sqrt(crossedOut.length);
        return (int) iterationLimit;
    }
    
    private static void crossOutMultiplesOf(int i) {
        for (int multiple = 2 * i; multiple < crossedOut.length; multiple += i)
            crossedOut[multiple] = true;
    }
    
    private static boolean notCrossed(int i) {
        return crossedOut[i] == false;
    }
    
    private static void putUncrossedIntegersIntoResult() {
        result = new int[numberOfUncrossedIntegers()];
        for (int j = 0, i = 2; i < crossedOut.length; i++)
            if (notCrossed(i))
                result[j++] = i;
    }
    
    private static int nubmerOfUncrossedIntegers() {
        int count = 0;
        for (int i = 2; i < crossedOut.length; i++)
            if (notCrossed(i))
                count++;
        
        return count;
    }
}
```



## 5장 형식 맞추기



### 형식을 맞추는 목적

코드 형식은 매우 중요해서, 무시하기 어려운 경우가 발생할 수도 있습니다. 융퉁성 없이 맹목적으로 따르면 안 됩니다. 코드 형식은 **의사소통의 일환**입니다. 어쩌면 '돌아가는 코드'가 개발자의 1차적인 의무라 여길지도 모르겠습니다. 오늘 구현한 기능이 다음 버전에서 바뀔 확률이 아주 높다는 걸 고려하면, 구현 해놓은 코드의 가독성은 앞으로 바뀔 코드의 품질에 지대한 영향을 미칩니다.

### 적절한 행 길이를 유지하라

세로 길이 기준으로, 소스 코드의 적당한 길이를 알아보기 위해 JUnit과 같은 대형 프로젝트 7개 통계를 내본 결과, 어느 프로젝트는 평균 파일 크기가 약 65줄, 다른 프로젝트는 평균이 약 200줄 정도로 편차가 꽤 심했습니다. 이것이 의미하는 바는 단순합니다. 짧은 파일로도 충분히 대형 프로젝트 규모의 시스템을 구축할 수 있다는 것.

### 신문 기사처럼 작성하라

신문처럼, 좋은 코드는 위에서 아래로, 상단 부분에는 고차원 개념과 알고리즘을, 변수 혹은 함수 이름만 보고도 찾고 있는 모듈이 맞는지 확인할 수 있게끔, 아래로 내려갈수록 의도를 세세하게 묘사하도록 짜여야 합니다. 신문이 사실, 날짜, 이름 등을 무작위로 뒤섞은 긴 기사 하나만 싣는다면 아무도 읽지 않을 것입니다.

### 개념은 빈 행으로 분리하라

거의 모든 코드는 왼쪽에서 오른쪽으로, 그리고 위에서 아래로 읽힙니다. 각 행은 수식이나 절을 나타내고, 일련의 행 묶음은 완결된 생각 하나를 표현합니다. 각 생각 사이는 빈 행을 넣어 분리해야 마땅합니다.

### 세로 밀집도

줄바꿈이 개념을 분리한다면 세로 밀집도는 연관성을 의미합니다. 즉, 서로 밀접한 코드 행은 세로로 가까이 놓여야 한다는 뜻입니다.

```java
public class ReporterConfig {
    /**
     * 리포터 리스너의 클래스 이름
     */
    private String m_className;
    
    /**
     * 리포터 리스너의 속성
     */
    private List<Property> m_properties = new ArrayList<>();
    public void addProperty(Property property) {
        m_properties.add(property);
    }
}
```

```java
public class ReporterConfig {
    private String m_className;
    private List<Property> m_properties = new ArrayList<>();
    
    public void addProperty(Property property) {
        m_properties.add(property);
    }
}
```

### 수직 거리

종종 함수 연관 관계와 동작 방식을 이해하려고 이 함수에서 저 함수로 오가며 소스 파일을 위아래로 뒤지는 등 뺑뺑이를 돌았던 경험이 있습니다. 서로 밀접한 개념은 세로로 가까이 둬야 하고, 한 파일에 속해야 마땅합니다.

변수는 사용하는 위치에 최대한 가까이 선언합니다. 함수는 짧게 구현해야 한다는 점을 고려하면, 지연 변수는 각 함수 맨 처음에 선언합니다.

```java
public int countTestCases() {
    int count = 0;
    for (Test each : tests)
        count += each.countTestCases();
    return count;
}
```

인스턴스 변수는 클래스 맨 처음에 선언합니다. 변수 간에 세로로 거리를 두지 않습니다.

한 함수가 다른 함수를 호출하는, 종속 함수는 서로 세로로 가까이 배치해야 합니다. 또한 가능하다면 호출하는 함수를 호출되는 함수보다 먼저 배치합니다. 위에서 아래로 자연스럽게 읽힐 수 있도록. 

개념적인 친화도가 높은 어떤 코드가 있다면, 코드를 가까이 배치하는 편이 좋습니다. 한 함수가 다른 함수를 호출해 생기는 직접적인 종속성, 변수와 그 변수를 사용하는 함수 등이 그 예시입니다.

```java
public class Assert {
    static public void assertTime(String message, boolean condition) {
        if (!condition)
            fail(message);
    }
    
    static public void assertTrue(boolean condition) {
        assertTrue(null, condition);
    }
    
    static public void assertFalse(String message, boolean condition) {
        assertTrue(message, !condition);
    }
    
    static public void assertFalse(boolean condition) {
        assertFalse(null, condition);
    }
    
    /* ... */
}
```

### 세로 순서

일반적으로 함수 호출 종속성은 아래 방향으로 유지합니다. 그러면 소스 코드 모듈이 고차원에서 저차원으로 자연스럽게 내려갑니다. 신문 기사와 마찬가지로 가장 중요한 개념을 가장 먼저 표현하고, 세세한 사항은 가장 마지막에 표현합니다.

### 가로 형식 맞추기

한 행의 가로 길이 또한 대형 프로젝트 7개를 조사해본 결과, 60자 이하인 파일이 전체의 약 70%에 달하고, 80자 이후부터 파일의 수는 급격하게 감소합니다. 요즘의 시대 상황을 고려하면, 120자 정도로 행 길이를 제한하는 걸 권장합니다.

### 가로 공백과 밀집도

가로로는 공백을 사용해 밀접한 개념과 느슨한 개념을 표현합니다.

```java
private void measureLine(String line) {
    lineCount++;
    int lineSize = line.length();
    totalChars += lineSize;
    lineWidthHistogram.addLine(lineSize, lineCount);
    recordWidestLine(lineSize);
}
```

위의 코드는 할당 연산자를 강조하기 위해 앞뒤에 공백을 주었고, 함수 이름과 이어지는 괄호 사이에는 밀접하기 떄문에 공백을 넣지 않았습니다.

```java
public class Quadratic {
    public static double root1(double a, double b, double c) {
        double determinant = determinant(a, b, c);
        return (-b + Math.sqrt(determinant) / (2*a));
    }
    private static double determinant(double a, double b, double c) {
        return b*b - 4*a*c;
    }
}
```

연산자 우선순위를 강조하기 위해서도 공백을 사용합니다. 곱셈의 우선순위가 가장 높기 때문에 승수 사이에는 공백이 없고, 항 사이에는 공백이 들어갑니다.

### 들여쓰기

코드의 범위로 이뤄진 계층을 표현하기 위해 코드를 들여씁니다. 들여쓰는 정도는 계층에서 코드가 자리잡은 수준에 비례합니다.

```java
public class CommentWidget extends TextWidget {
    public static final String REGEXP = "^#[^\r\n]*(?:(?:\r\n)|\n|\r)?";
    
    public CommentWidget(ParentWidget parent, String text) { super(parent, text); }
    public String render() throws Exception { return ""; }
}
```

위의 코드처럼 간혹 간단한 if문 혹은 짧은 함수에서 들여쓰기 규칙을 무시하곤 합니다. 하지만 항상 원점으로 돌아가 들여쓰기를 넣어야 합니다.

### 팀 규칙

팀은 한 가지 규칙에 합의해야 합니다. 그리고 모든 팀원은 그 규칙을 따라야 합니다. 그래야 소프트웨어가 일관적인 스타일을 보입니다. 스타일은 일관적이고 매끄러워야 읽기 쉬운 문서가 될 수 있습니다.



## 6장 객체와 자료 구조



### 자료 추상화

```java
public class Point {
    public double x;
    public double y;
}
```

- 구체적인 `Point`클래스로, 구현을 외부로 노출합니다.

```java
public interface Point {
    double getX();
    double getY();
    void setCartesian(double x, double y);
    double getB();
    double getTheta();
    void setPolar(double r, double theta);
}
```

- 구현을 외부로 노출하지 않으면서도 클래스 메서드가 접근 정책을 강제합니다.

구현을 숨기겠다고 무작정 변수 사이에 함수라는 계층을 넣는다고 감춰지지 않습니다. 단순히 인터페이스나 조회/설정 함수만으로는 추상화가 이뤄지지 않습니다. 개념의 추상화가 필요합니다. 추상 인터페이스를 제공해 사용자가 구현을 모른 채 자료의 핵심을 조작할 수 있어야 합니다.

### 자료/객체 비대칭

위의 예제는 객체와 자료 구조 사이에 벌어진 차이를 보여줍니다. 객체는 추상화 뒤로 자료를 숨긴 채 자료를 다루는 함수만 공개합니다. 자료 구조는 자료를 그대로 공개하며 별다른 함수는 제공하지 않습니다. 이 두가지는 본질적으로 상반됩니다.

```java
public class Square {
    public Point topLeft;
    public double side;
}

public class Rectangle {
    public Point topLeft;
    public double height;
    public double sidth;
}

public class Circle {
    public Point center;
    public double radius;
}

public class Geometry {
    public final double PI = 3.14;
    
    public double area(Object shape) throws NoSuchShapeException {
        if (shape instanceof Square) {
            /* ... */
        }
        else if (shape instanceof Square) {
            /* ... */
        }
        else if (shape instanceof Square) {
            /* ... */
        }
    }
}
```

- 만약 `Geometry` 클래스에 둘레 길이를 구하는 함수를 추가하고 싶은 경우, 도형 클래스에는 아무 영향을 끼치지 않고, 함수를 추가하면 됩니다. 반대로 새 도형을 추가하고 싶은 경우에는 `Geometry`에 속한 함수를 모두 고쳐야 합니다.

```java
public class Square implemnets Shape {
    private Point topLeft;
    private double side;
    
    public double area() {
        return side*side;
    }
}

public class Rectangle implements Shape {
    private Point topLeft;
    private double height;
    private double width;
    
    public double area() {
        return height * width;
    }
}

public class Circle implements Shape {
    private Point center;
    private double radius;
    public final double PI = 3.14;
    
    public double area() {
        return PI * radius * radius;
    }
}
```

- 여기서 `area()` 메서드는 다형 메서드입니다. `Geometry`와 같은 클래스는 필요 없습니다. 그러므로 새 도형을 추가해도 기존 함수에 아무런 영향을 미치지 않습니다. 반면 새 함수를 추가하고 싶다면 도 형 클래스를 전부 고쳐야 합니다.

(자료 구조를 사용하는) 절차적인 코드는 기존 자료 구조를 변경하지 않으면서 새 함수를 추가하기 쉽습니다. 반면, 객체 지향 코드는 기존 함수를 변경하지 않으면서 새 클래스를 추가하기 쉽습니다.

위의 역도 참 입니다. 절차적인 코드는 새로운 자료 구조를 추가하기 어렵습니다. 그러려면 모든 함수를 고쳐야 합니다. 객체 지향 코드는 새로운 함수를 추가하기 어렵습니다. 그러려면 모든 클래스를 고쳐야 합니다.

### 디미터 법칙

디미터 법칙은 잘 알려진 휴리스킥(heuristic)으로, 모듈은 자신이 조작하는 객체의 속사정을 몰라야 한다는 법칙입니다. 앞서 보았듯이, 객체는 자료를 숨기고 함수를 공개합니다. 좀 더 정확히 표현하자면, 디미터 법칙은 '클래스 C의 메서드 f는 다음과 같은 객체의 메서드만 호출해야 한다'라고 주장합니다.

- 클래스 C
- f가 생성한 객체
- f 인수로 넘어온 객체
- C 인스턴스 변수에 저장된 객체

하지만 위 객체에서 허용된 메서드가 반환하는 객체의 메서드는 호출하면 안 됩니다. 아래 코드는 디미터 법칙을 어기는 듯이 보입니다.

```java
final String outtputDir = ctxt.getOptions().getScratchDir().getAbsolutePath();
```

### 기차 충돌

위와 같은 코드를 기차 충돌이라고 부릅니다. 아래와 같이 나누는 편이 좋습니다.

```java
Options opts = ctxt.getOptions();
File scratchDir = opts.getScratchDir();
final String outputDir = scratchDir.getAbsoulutePath();
```

이렇게 짜인 코드의 경우, 함수 하나가 아는 지식이 굉장히 많은 상태입니다. 즉, 많은 객체를 탐색할 줄 아는 것입니다. 이 예제가 디미터 법칙을 위반하는지 여부는 각 클래스가 객체인지 아니면 자료 구조인지에 따라 달라집니다. 객체라면 내부 구조를 숨겨야 하므로 확실히 디미터 법칙을 위반합니다.

### 잡종 구조

때때로 절반은 객체, 절반은 자료 구조인 잡종 구조가 나옵니다. 공개 변수나 공개 조회/설정 함수가 있고, 공개 조회/설정 함수는 비공개 변수를 그대로 노출합니다. 이런 구조는 새로운 함수는 물론이고 새로운 자료 구조도 추가하기 어렵습니다. 따라서 되도록이면 피하는 편이 좋습니다.

### 구조체 감추기

위의 예제에서 정말 각 클래스들이 객체라면, 줄줄이 사탕으로 엮으면 안 됩니다.

```java
ctxt.getAbsolutePathOfScratchDirectoryOption();
```

- 이 방법은 `ctxt` 객체에 공개해야 하는 메서드가 너무 많아집니다.

```java
ctxt.getScratchDirectoryOption().getAbsolutePath();
```

- 이 방법은 `getScratchDirectoryOption()`이 객체가 아니라 자료 구조를 반환한다고 가정합니다.

두 방법 모두 깔끔하지 않습니다. `ctxt`가 객체라면 뭔가를 하라고 말해야지, 속을 드러내라고 말하면 안 됩니다. 임시 디렉터리의 절대 경로가 왜 필요한지를 알아내서 수정해야 합니다. 해당 모듈의 추가 코드를 살펴보면, 임시 디렉터리의 절대 경로를 얻으려는 이유가 임시 파일을 생성하기 위함입니다.

```java
BufferedOutputStream bos = ctxt.createScratchFileStream(classFileName);
```

따라서 객체에게 이 임무를 맡기면 적당합니다.

### 자료 전달 객체

자료 구조체의 전형적인 형태는 공개 변수만 있고 함수가 없는 클래스입니다. 이런 구조를 DTO(Data Transfer Object)라고 합니다. DTO는 굉장히 유용한 구조체로, DB에 저장된 가공되지 않은 정보를 애플리케이션 코드에서 사용할 객체로 변환하는 일련의 단계에서 가장 처음으로 사용하는 구조체입니다.



## 7장  오류 처리

### 오류 코드보다 예외를 사용하라

```java
public class Device Controller {
    /* ... */
    public void sendShutDown() {
        DeviceHandle handle = getHandle(DEV1);
        if (handle != DeviceHandle.INVALID) {
            retrieveDeviceRecord(handle);
            if (recode.getStatus() != DEVICE_SUSPENDED) {
                pauceDevice(handle);
                clearDeviceWorkQueue(handle);
                closeDevce(handle);
            }
            else {
                logger.lod("Device suspended.");
            }
        }
        else {
            logger.log("Invalid handle for: " + DEV1.toString());
        }
    }
}
```

위처럼 오류 코드를 사용하면 호출자 코드가 복잡해 집니다. 함수를 호출한 즉시 오류를 확인해야 하기 때문입니다. 그래서 오류가 발생하면 예외를 던지는 편이 낫습니다.

```java
public class DeviceController {
    /* ... */
    public void sendShutDonw() {
        try {
            tryToShutDown();
        } catch (DeviceShutDownError e) {
            logger.log(e);
        }
    }
    
    private void tryToShutDown() thros DeviceShutDownError {
        DeviceHandle handle = getHandle(DEV1);
        DeviceRecord record = retrieveDeviceRecord(handle);
        
        pauseDevice(handle);
        clearDeviceWorkQueue(handle);
        closeDevice(handle);
    }
    
    private DeviceHandle getHandle(DeviceID id) {
        /* ... */
        throw new DeviceShutDownError("Invalid handle for: " + id.toString());
    }
}
```

### Try-Catch-Finally 문부터 작성하라

어떤 면에 있어서는 try 블록은 트랜잭션과 비슷합니다. 블록에서 무슨 일이 생기든지 catch 블록은 프로그램 상태를 일관성 있게 유지해야 합니다. 그러므로 예외가 발생할 코드를 짤 때는 try-catch-finally 문으로 시작하는 편이 낫습니다.

```java
public List<RecordedGrip> retrieveSection(String sectionName) {
    try {
        FileInputStream stream = new FileInputStream(sectionName);
        stream.close();
    } catch (FileNotFoundException e) {
        throw new StorageException("retrieval error", e);
    }
    return new ArrayList<RecordedGrip>();
}
```

try-catch 구조로 범위를 정의했으므로 TDD를 사용해 필요한 나머지 논리를 추가합니다. 먼저 강제로 예외를 일으키는 테스트 케이스를 작성한 후 테스트를 통과하게 코드를 작성하는 방법을 권장합니다. 그러면 자연스럽게 try 블록의 트랜잭션 범위부터 구현하게 되므로 범위 내에서 트랜잭션 본질을 유지하기 쉬워집니다.

### 예외에 의미를 제공하라

오류가 발생한 원인과 위치를 찾기 쉽도록 예외를 던질 때는 전후 상황을 충분히 덧붙여야 합니다. 오류 메시지에 정보를 담아 예외와 함께 던지는 것이 좋습니다. 실패한 연산 이름과 실패 유형도 같이 언급합니다.

### 호출자롤 고려해 예외 클래스를 정의하라

아래 코드는 오류를 형편없이 분류한 사례로, 외부 라이브러리가 던질 예외를 모두 잡아냅니다.

```java
ACMEPort port = new ACMEPort(12);

try {
    port.open();
} catch (DeviceResponseException e) {
    reportPortError(e);
    logger.log("Device response exception", e);
} catch (ATM1212UnlockedException e) {
    reportPortError(e);
    logger.log("Unlock exception", e);
} catch (GMXError e) {
    reportPortError(e);
    logger.log("Device response exception");
} finally {
    /* ... */
}
```

예외에 대응하는 방식이 예외 유형과 무관하게 거의 동일합니다. 따라서 중복을 제거하면서 예외 클래스를 정의하면 코드를 간결하게 고칠 수 있습니다.

```java
LocalPort port = new LocalPort(12);
try {
    port.open();
} catch (PortDeviceFailure e) {
    reportError(e);
    logger.log(e.getMessage(), e);
} finally {
    /* ... */
}
```

```java
public class LocalPort {
    private ACMEPort innerPort;
    
    public LocalProt(int portNumber) {
        innerPort = new ACMEPort(portNumber);
    }
    
    public void open() {
        try {
            innerPort.open();
        } catch (DeviceResponseException e) {
            throw new PortDeviceFailure(e);
        } catch (ATM1212UnlockedException e) {
            throw new PortDeviceFailure(e);
        } catch (GMXError e) {
            throw new PortDeviceFailure(e);
        }
    }
}
```

`LocalPort` 클래스처럼 외부 API를 감싸는 클래스는 매우 유용합니다. 외부 API를 감싸면 외부 라이브러리와 프로그램 사이에서 의존성이 크게 줄어듭니다. 나중에 다른 라이브러리로 갈아타더라도 비용이 적습니다.

### null을 반환하지 마라

null을 반환하는 코드는 일거리를 늘릴 뿐만 아니라 호출자에게 문제를 떠넘기는 꼴입니다. 중간 하나라도 null 확인을 빼먹는다면 애플리케이션이 통제 불능에 빠질지도 모릅니다. 메서드에서 null을 반환하고 싶다면, 그 대신 예외를 던지거나 특수 사례 객체를 반환하는 것이 좋습니다.

```java
List<Employee> employees = getEmployees();
for (Employee e : employees) {
    totalPay += e.getPay();
}
```

```java
public List<Employee> getEmployees() {
    if (/* 직원이 없다면 */) {
        return Collections.emptyList();
    }
}
```

위처럼 null을 반환하는 대신 빈 컬렉션을 반환다면 null 체크할 필요없이 코드가 훨씬 깔끔해집니다.

### null을 전달하지 마라

정상적인 인수로 null을 기대하는 API가 아니라면 메서드로 null을 전달하는 코드는 최대한 피해야 합니다.

```java
public class MetricsCalculator {
    public double xProjection(Point p1, Point p2) {
        assert p1 != null : "p1 should not be null";
        assert p2 != null : "p2 should not be null";
        return (p2.x - p1.x) * 1.5;
    }
}
```

assert 문을 사용해서 일부는 방지할 수는 있지만, 여전히 문제를 완전히 해결하지는 못합니다. 대다수 프로그래밍 언어는 호출자가 실수로 넘기는 null을 적절히 처리하는 방법이 없습니다. 애초에 null을 넘기지 못하도록 금지하는 정책을 따르는 것이 보다 합리적입니다.



## 8장 경계

시스템에 들어가는 모든 소프트웨어를 직접 개발하는 경우는 드뭅니다. 때로는 외부 소스를 사용해서 구현하는데, 이 소프트웨어 경계를 깔끔하게 처리하는 방법을 알아보겠습니다.

### 외부 코드 사용하기

인터페이스 제공자와 언터페이스 사용자 사이에는 특유의 밀당이 존재합니다. 패키지 제공자는 적용성을 최대한 넓히려 애쓰지만, 사용자는 자신의 요구에 집중하는 인터페이스를 바라곤 합니다.

```java
Map sensors = new HashMap();
Sensor s = (Sensor) sensors.get(sensorId);
```

`Map`은 범용성을 위해 Object를 반환하므로 이를 올바른 유형으로 변환할 책임은 라이브러를 사용하는 클라이언트에게 있습니다. 위처럼 코드를 짜도 동작은 하지만, 깨끗한 코드라고 보기는 어렵습니다. 이때 경계에 위치한 클래스를 생성해주면 한결 나아집니다.

```java
public class Sensors {
    private Map sensors = new HashMap();
    
    public Sensor getById(String id) {
        return (Sensor) sensors.get(id);
    }
}
```

경계 인터페이스인 `Map`을 `Sensors` 안으로 숨깁니다. 혹여나 `Map` 인터페이스가 변하더라도 나머지 프로그램에는 영향을 미치지 않습니다. `Sensors` 클래스 안에서 객체 유형을 관리하고 변환하기 대문입니다. 더불어 `Sensors` 클래스는 프로그램에 필요한 인터페이스를 제공해서, 코드를 이해하기는 쉽지만 오용하기는 어렵게 강제합니다. 물론 `Map`과 같은 클래스를 사용할 때마다 위와 같이 캡슐화하라는 뜻은 아닙니다. `Map`과 유사한 경계 인터페이스를 여기저기 넘기지 말아야 하는 것이 핵심입니다.

### 경계 살피고 익히기

타사 라이브러리를 가져왔으나 사용법이 분명치 않은 경우가 종종 있습니다. 외부 코드는 익히기 어렵고, 통합하기도 어렵습니다. 따라서 그 대신 우리쪽 코드를 작성해 외부 코드를 호출하는 대신 먼저 간단한 테스트 케이스를 작성해 외부 코드를 익히는 방향이 좋습니다.



## 9장 단위 테스트

### TDD 법칙 세 가지

TDD는 실제 코드를 짜기 전에 단위 테스트부터 짜라고 요구합니다. 이를 위한 세부 법칙을 살펴보겠습니다.

- **첫째 법칙**: 실패하는 단위 테스트를 작성할 때까지 실제 코드를 작성하지 않습니다.
- **둘째 법칙**: 컴파일은 실패하지 않으면서 실행이 실패하는 정도로만 단위 테스트를 작성합니다.
- **셋째 법칙**: 현재 실패하는 테스트를 통과할 정도로만 실제 코드를 작성합니다.

위 세 가지 규칙을 따르면 개발과 테스트가 대략 30초 주기로 묶입니다. 이렇게 개발을 진행하면 실제 코드와 맞먹을 정도로 방대한 테스트 코드가 나오고, 이는 심각한 관리 문제를 유발하기도 합니다.

### 깨끗한 테스트 코드 유지하기

지저분한 테스트 코드를 내놓으나 테스트를 안 하나 오십보 백보입니다. 실제 코드가 진화하면 테스트 코드도 변해야 합니다. 테스트 코드가 지저분할수록 변경하기 어려워지고, 점점 실제 코드를 짜는 시간보다 테스트 케이스를 추가하는 시간이 더 걸리기 십상입니다. **테스트 코드는 실제 코드 못지 않게 중요**합니다.

### 테스트는 유연성, 유지보수성, 재사용성을 제공한다

테스트 케이스는 실제 코드를 유연하게 만드는 버팀목입니다. 테스트 케이스가 없다면 모든 변경이 잠정적인 버그이지만, 있으면 코드 변경이 두렵지 않습니다.  즉 코드에 유연성, 유지보수성, 재사용성을 제공하는 버팀목이 바로 단위 테스트입니다. 따라서 테스트 코드가 지저분하면 코드를 변경하는 능력이 떨어지면 코드 구조를 개선하는 능력도 떨어집니다. 테스트 코드가 지저분할수록 실제 코드도 지저분해집니다. 결국 테스트 코드를 잃어버리고 실제 코드도 망가집니다.

### 깨끗한 테스트 코드

깨끗한 테스트 코드를 만들려면 **가독성**이 제일 우선입니다. 어쩌면 실제 코드보다 테스트 코드에 더더욱 중요합니다. BUILD-OPERATE-CHECK 패턴처럼 각 테스트를 명확히 세 부분으로 나눠서 작성하는 구조가 테스트 구조에 적합합니다. 더불어 잡다하고 세세한 코드를 거의 다 없애야 합니다. 본론에 바로 돌입해 진짜 필요한 자료 유형과 함수만 사용해야 합니다.

### 테스트 당 개념 하나

assert 문을 테스트 당 하나만 사용하라는 규칙도 훌륭한 지침입니다. 그러나 규칙의 핵심은 오로지 하나의 개념만을 테스트해야 하는 것입니다. 이것저것 잡다한 개념을 연속적으로 테스트하는 긴 함수는 피합니다.

### F.I.R.S.T

깨끗한 테스트는 다음 다섯 가지 규칙을 따릅니다.

- **Fast**: 테스트는 자주 돌리면서 문제를 수시로 찾을 수 있도록 빨리 돌아야 합니다. 
- **Independent**: 각 테스트는 서로 의존하면 안 됩니다. 한 테스트가 다음 테스트가 실행될 환경을 준비해서는 안 되며, 각 테스트는 어떤 순서로 실행해도 괜찮아야 합니다.
- **Repeatable**: 테스트는 어떤 환경에서도 반복 가능해야 합니다.
- **Self-Validating**: 테스트는 성공 혹은 실패로, boolean 값으로 결과를 내야 합니다. 통과 여부를 알려고 로그 파일을 읽게 만들어서는 안 됩니다.
- **Timely**: 테스트는 적시에 작성해야 합니다. 단위 테스트는 테스트하려는 실제 코드를 구현하기 직전에 구현합니다. 실제 코드를 구현한 다음에 테스트 코드를 만들면 실제 코드가 테스트하기 어려울 수 있습니다.