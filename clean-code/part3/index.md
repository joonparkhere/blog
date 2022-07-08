---
title: Clean Code 10~13장
date: 2021-05-14
pin: false
tags:
- Software Enginerring
- Clean Code
- Java
---

## 10장 클래스

### 클래스 체계

클래스를 정의하는 표준 자바 관례에 따르면, 가장 면서 `static public` 변수가 나오고, 다음으로 `private` 변수가 나오며, 이어서 `private` 인스턴스 변수가 나옵니다. `public` 변수가 필요한 경우는 거의 없습니다. 변수 목록 다음에는 `public` 함수가 나옵니다. `private` 함수는 자신을 호출하는 `public` 함수 직후에 넣습니다. 죽, 추상화 단계가 순차적으로 내려갑니다.

### 클래스는 작아야 한다!

클래스를 만들 때 **첫 번째 규칙은 크기**입니다. **두 번째 규칙도 크기**입니다. 여기서 크기는 실제 클래스 파일 크기를 가리키기도 하지만, 본질적으로 **책임**을 의미합니다.

클래스 이름은 해당 클래스 책임을 기술해야 합니다. 따라서 작명은 클래스 크기를 줄이는 첫 번째 관문인 셈입니다. 간결한 이름이 떠오르지 않는다면 대부분 클래스 크기가 너무 커서 그럴 것 입니다. 또한 클래스 설명은 `if`, `and`, `or`, `but`을 사용하지 않고서 대략 25단어 내외로 가능해야 합니다.

### 단일 책임 원칙 SRP

SRP(SIngle Responsibility Principle)은 클래스나 모듈을 변경할 이유가 단 하나뿐이어야 한다는 원칙입니다. 책임, 즉 변경할 이유를 파악햐려 애쓰다 보면 코드를 추상화하기도 쉬워집니다. SRP는 클래스 설계할 때 가장 무시되는 규칙 중 하나 입니다. 프로그래머 대다수가 프로그램이 돌아가면 일이 끝났다고 여기기 때문입니다. `깨끗하고 체계적인 소프트웨어`라는 다음 관심사로 전환하지 않습니다. 시스템의 규모가 어느 수준에 이르면 시스템은 논리가 많고도 복잡해 집니다. 이런 복잡성을 다루려면 체계적이 정리가 필수입니다.

### 응집도 유지, 작은 클래스 여럿

클래스는 인스턴스 변수의 수가 작아야 합니다. 각 클래스 메서드는 클래스 인스턴스 변수를 하나 이상 사용해야 합니다. 일반적으로 메서드가 변수를 더 많이 사용할수록 메서드와 클래스는 응집도가 더 높습니다. 일반적으로 응집도가 가장 높은 클래스는 불가능하지만, 응집도가 높다는 것은 서로 의존하며 논리적인 단위로 묶인다는 의미이기에 프로그래머들은 응집도가 높은 클래스를 선호합니다.

변수가 아주 많은 큰 함수 하나가 있다면, 큰 함수를 작은 함수 여럿으로 나눠서 크기를 작게, 책임을 작게 해야 합니다. 이때 함수가 사용하는 몇몇 변수를 묶어서 독자적인 클래스로 분리하면 응집도를 높이면서 크기도 작게 만들 수 있습니다.

```java
public class PrintPrimes {
    public static void main(String[] args) {
        final int M = 1000; 
        final int RR = 50;
        final int CC = 4;
        final int WW = 10;
        final int ORDMAX = 30; 
        int P[] = new int[M + 1]; 
        int PAGENUMBER;
        int PAGEOFFSET; 
        int ROWOFFSET; 
        int C;
        int J;
        int K;
        boolean JPRIME;
        int ORD;
        int SQUARE;
        int N;
        int MULT[] = new int[ORDMAX + 1];

        J = 1;
        K = 1; 
        P[1] = 2; 
        ORD = 2; 
        SQUARE = 9;

        while (K < M) { 
            do {
                J = J + 2;
                if (J == SQUARE) {
                    ORD = ORD + 1;
                    SQUARE = P[ORD] * P[ORD]; 
                    MULT[ORD - 1] = J;
                }
                N = 2;
                JPRIME = true;
                while (N < ORD && JPRIME) {
                    while (MULT[N] < J)
                        MULT[N] = MULT[N] + P[N] + P[N];
                    if (MULT[N] == J) 
                        JPRIME = false;
                    N = N + 1; 
                }
            } while (!JPRIME); 
            K = K + 1;
            P[K] = J;
        } 
        {
            PAGENUMBER = 1; 
            PAGEOFFSET = 1;
            while (PAGEOFFSET <= M) {
                System.out.println("The First " + M + " Prime Numbers --- Page " + PAGENUMBER);
                System.out.println("");
                for (ROWOFFSET = PAGEOFFSET; ROWOFFSET < PAGEOFFSET + RR; ROWOFFSET++) {
                    for (C = 0; C < CC;C++)
                        if (ROWOFFSET + C * RR <= M)
                            System.out.format("%10d", P[ROWOFFSET + C * RR]); 
                    System.out.println("");
                }
                System.out.println("\f"); PAGENUMBER = PAGENUMBER + 1; PAGEOFFSET = PAGEOFFSET + RR * CC;
            }
        }
    }
}
```

위의 어지러운 코드를 아래로 리펙토링 할 수 있습니다.

```java
public class PrimePrinter {
    public static void main(String[] args) {
        final int NUMBER_OF_PRIMES = 1000;
        int[] primes = PrimeGenerator.generate(NUMBER_OF_PRIMES);

        final int ROWS_PER_PAGE = 50; 
        final int COLUMNS_PER_PAGE = 4; 
        RowColumnPagePrinter tablePrinter = 
            new RowColumnPagePrinter(ROWS_PER_PAGE, 
                        COLUMNS_PER_PAGE, 
                        "The First " + NUMBER_OF_PRIMES + " Prime Numbers");
        tablePrinter.print(primes); 
    }
}
```

```java
public class RowColumnPagePrinter { 
    private int rowsPerPage;
    private int columnsPerPage; 
    private int numbersPerPage; 
    private String pageHeader; 
    private PrintStream printStream;

    public RowColumnPagePrinter(int rowsPerPage, int columnsPerPage, String pageHeader) { 
        this.rowsPerPage = rowsPerPage;
        this.columnsPerPage = columnsPerPage; 
        this.pageHeader = pageHeader;
        numbersPerPage = rowsPerPage * columnsPerPage; 
        printStream = System.out;
    }

    public void print(int data[]) { 
        int pageNumber = 1;
        for (int firstIndexOnPage = 0 ; 
            firstIndexOnPage < data.length ; 
            firstIndexOnPage += numbersPerPage) { 
            int lastIndexOnPage =  Math.min(firstIndexOnPage + numbersPerPage - 1, data.length - 1);
            printPageHeader(pageHeader, pageNumber); 
            printPage(firstIndexOnPage, lastIndexOnPage, data); 
            printStream.println("\f");
            pageNumber++;
        } 
    }

    private void printPage(int firstIndexOnPage, int lastIndexOnPage, int[] data) { 
        int firstIndexOfLastRowOnPage =
        firstIndexOnPage + rowsPerPage - 1;
        for (int firstIndexInRow = firstIndexOnPage ; 
            firstIndexInRow <= firstIndexOfLastRowOnPage ;
            firstIndexInRow++) { 
            printRow(firstIndexInRow, lastIndexOnPage, data); 
            printStream.println("");
        } 
    }

    private void printRow(int firstIndexInRow, int lastIndexOnPage, int[] data) {
        for (int column = 0; column < columnsPerPage; column++) {
            int index = firstIndexInRow + column * rowsPerPage; 
            if (index <= lastIndexOnPage)
                printStream.format("%10d", data[index]); 
        }
    }

    private void printPageHeader(String pageHeader, int pageNumber) {
        printStream.println(pageHeader + " --- Page " + pageNumber);
        printStream.println(""); 
    }

    public void setOutput(PrintStream printStream) { 
        this.printStream = printStream;
    } 
}
```

```java
public class PrimeGenerator {
    private static int[] primes;
    private static ArrayList<Integer> multiplesOfPrimeFactors;

    protected static int[] generate(int n) {
        primes = new int[n];
        multiplesOfPrimeFactors = new ArrayList<Integer>(); 
        set2AsFirstPrime(); 
        checkOddNumbersForSubsequentPrimes();
        return primes; 
    }

    private static void set2AsFirstPrime() { 
        primes[0] = 2; 
        multiplesOfPrimeFactors.add(2);
    }

    private static void checkOddNumbersForSubsequentPrimes() { 
        int primeIndex = 1;
        for (int candidate = 3 ; primeIndex < primes.length ; candidate += 2) { 
            if (isPrime(candidate))
                primes[primeIndex++] = candidate; 
        }
    }

    private static boolean isPrime(int candidate) {
        if (isLeastRelevantMultipleOfNextLargerPrimeFactor(candidate)) {
            multiplesOfPrimeFactors.add(candidate);
            return false; 
        }
        return isNotMultipleOfAnyPreviousPrimeFactor(candidate); 
    }

    private static boolean isLeastRelevantMultipleOfNextLargerPrimeFactor(int candidate) {
        int nextLargerPrimeFactor = primes[multiplesOfPrimeFactors.size()];
        int leastRelevantMultiple = nextLargerPrimeFactor * nextLargerPrimeFactor; 
        return candidate == leastRelevantMultiple;
    }

    private static boolean isNotMultipleOfAnyPreviousPrimeFactor(int candidate) {
        for (int n = 1; n < multiplesOfPrimeFactors.size(); n++) {
            if (isMultipleOfNthPrimeFactor(candidate, n)) 
                return false;
        }
        return true; 
    }

    private static boolean isMultipleOfNthPrimeFactor(int candidate, int n) {
        return candidate == smallestOddNthMultipleNotLessThanCandidate(candidate, n);
    }

    private static int smallestOddNthMultipleNotLessThanCandidate(int candidate, int n) {
        int multiple = multiplesOfPrimeFactors.get(n); 
        while (multiple < candidate)
            multiple += 2 * primes[n]; 
        multiplesOfPrimeFactors.set(n, multiple); 
        return multiple;
    } 
}
```

### 변경하기 쉬운 클래스

대다수 시스템은 지속적인 변경이 필요합니다. 그리고 뭔가를 변경할 때마다 시스템이 의도대로 동작하지 않을 위험이 따릅니다. 깨끗한 시스템은 클래스를 체계적으로 정리해 변경에 수반하는 위험을 낮춥니다.

```java
public class Sql {
    public Sql(String table, Column[] columns)
    public String create()
    public String insert(Object[] fields)
    public String selectAll()
    public String findByKey(String keyColumn, String keyValue)
    public String select(Column column, String pattern)
    public String select(Criteria criteria)
    public String preparedInsert()
    private String columnList(Column[] columns)
    private String valuesList(Object[] fields, final Column[] columns) 
	private String selectWithCriteria(String criteria)
    private String placeholderList(Column[] columns)
}
```

위의 클래스에서 새로운 SQL 문을 지원하려면 많은 부분에 손을 대야 합니다. 보다 변경하기 쉽게 하기 위해서, 클래스를 더 작게 쪼개는 편이 좋습니다.

```java
abstract public class Sql {
	public Sql(String table, Column[] columns) 
	abstract public String generate();
}
public class CreateSql extends Sql {
	public CreateSql(String table, Column[] columns) 
	@Override public String generate()
}

public class SelectSql extends Sql {
	public SelectSql(String table, Column[] columns) 
	@Override public String generate()
}

public class InsertSql extends Sql {
	public InsertSql(String table, Column[] columns, Object[] fields) 
	@Override public String generate()
	private String valuesList(Object[] fields, final Column[] columns)
}

public class SelectWithCriteriaSql extends Sql { 
	public SelectWithCriteriaSql(
	String table, Column[] columns, Criteria criteria) 
	@Override public String generate()
}

public class SelectWithMatchSql extends Sql { 
	public SelectWithMatchSql(String table, Column[] columns, Column column, String pattern) 
	@Override public String generate()
}

public class FindByKeySql extends Sql public FindByKeySql(
	String table, Column[] columns, String keyColumn, String keyValue) 
	@Override public String generate()
}

public class PreparedInsertSql extends Sql {
	public PreparedInsertSql(String table, Column[] columns) 
	@Override public String generate() {
	private String placeholderList(Column[] columns)
}

public class Where {
	public Where(String criteria) public String generate()
	public String generate() {
}

public class ColumnList {
	public ColumnList(Column[] columns) public String generate()
	public String generate() {
}
```

이렇게 재구성한 SQL 클래스는 SRP와 OCP도 지원합니다.

### 변경으로부터 격리

객체 지향 프로그래밍에는 구체적인 `concrete` 클래스와 추상 `abstract` 클래스가 있습니다. 구체적인 클래스는 상세한 구현을 포함하며 추상 클래스는 개념만 포함합니다. 따라서 인터페이스와 추상 클래스를 사용해 구현이 미치는 영향을 격리합니다.

```java
public insterface StockExchange {
	Money currentPrice(String symbol);
}
```

```java
public Portfolio {
	private StockExchange exchange;
    
	public Portfolio(StockExchange exchange) {
		this.exchange = exchange;
	}
}
```

```java
public class PortfolioTest {
	private FixedStockExchangeStub exchange;
	private Portfolio portfolio;

	@Before
	protected void setUp() throws Exception {
		exchange = new FixedStockExchangeStub();
		exchange.fix("MSFT", 100);
		portfolio = new Portfolio(exchange);
	}

	@Test
	public void GivenFiveMSFTTotalShouldBe500() throws Exception {
		portfolio.add(5, "MSFT");
		Assert.assertEquals(500, portfolio.value());
	}
}
```

이처럼 시스템의 결합도를 낮추면 유연성과 재사용성도 더욱 높아집니다. 결합도가 낮다는 것은 각 시스템 요소가 다른 요소로부터 그리고 변경으로부터 잘 격리되어 있다는 의미입니다. 이렇게 결합도를 최소로 줄이면 DIP(Dependency Inversion Principle)을 따르는 클래스가 나옵니다. DIP는 본질적으로 클래스가 상세한 구현이 아니라 추상화에 의존해야 한다는 원칙입니다.



## 11장 시스템

### 제작과 사용을 분리하라

소프트웨어 시스템은 **애플리케이션 객체를 제작하고 의존성을 서로 '연결'하는 준비**과정과 준비 과정 이후에 이어지는 **런타임 로직**을 분리해야 합니다.시작 단계는 모든 애플리케이션이 풀어야 할 관심사(concern)입니다. 관심사 분리는 가장 중요한 설계 기법 중 하나입니다.

**main 분리**
시스템 생성과 시스템 사용을 분리하는 한 가지 방법으로, 생성과 관련한 코드는 모두 main이나 main이 호출하는 모듈로 옮기고, 나머지 시스템은 모든 객체가 생성되었고 모든 의존성이 연결되었다고 가정합니다.

![](images/11-separate-main.png)

main 함수에서 시스템에 필요한 객체를 생성한 후 이를 애플리케이션에 넘기는 제어 흐름입니다. 애플리케이션은 그저 객체를 사용할 뿐입니다. 모든 화살표가 main 쪽에서 애플리케이션 쪽을 향합니다. 즉, 애플리케이션은 main이나 객체가 생성되는 과정을 전혀 모른다는 뜻입니다.

**의존성 주입**
사용과 제작을 분리하는 강력한 메커니즘 하나가 의존성 주입(Dependency Injection, DI)입니다. 의존성 주입은 제어의 역전(Inversion of Control, IoC)기법을 의존성 관리에 적용한 메커니즘입니다. 제어 역전에서는 한 객체가 맡은 보조 책임을 새로운 객체에게 전적으로 떠넘깁니다. 새로운 객체는 넘겨받은 책임만 맡으므로 SRP를 지키게 됩니다. 의존성 관리 맥락에서 객체는 의존성 자체를 인스턴스로 만드는 책임은 지지 않습니다. 대신에 이런 책임을 다른 전담 메커니즘에 넘겨서 제어를 역전합니다. 초기 설정은 시스템 전체에서 필요하므로 대개 책임질 메커니즘으로 main 루틴이나 특수 컨테이너를 사용합니다.

### 확장

처음부터 올바르게 시스템을 만들 수 있다는 믿음은 미신입니다. 대신 프로그래머는 주어진 사용자 스토리에 맞춰 시스템을 구현해야 합니다. 내일은 새로운 스토리에 맞춰 시스템을 조정하고 확장해야 합니다. 반복적이고 점진적인 애자일 방식입니다. 이런 경우 깨끗한 코드는 테스트 주도 개발(Test-Driven Development, TDD), 리팩터링으로 쉽게 만들 수 있습니다.

### 자바 프록시

개별 객체나 클래스에서 매서드 호출을 감싸는 경우처럼 단순한 상황에 적합합니다.

```java
// 은행 추상화
public interface Bank {
    Collection<Account> getAccounts();
    void setAccounts(Collection<Account> accounts);
}
```

```java
// 추상화를 위한 POJO(Plain Old Java Object) 구현
public class BankImpl implements Bank {
    private List<Account> accounts;

    public Collection<Account> getAccounts() {
        return accounts;
    }
    
    public void setAccounts(Collection<Account> accounts) {
        this.accounts = new ArrayList<Account>();
        for (Account account: accounts) {
            this.accounts.add(account);
        }
    }
}
```

```java
// 프록시 API가 필요한 InvocationHandler
public class BankProxyHandler implements InvocationHandler {
    private Bank bank;
    
    public BankHandler (Bank bank) {
        this.bank = bank;
    }
    
    // InvocationHandler에 정의된 메서드
    public Object invoke(Object proxy, Method method, Object[] args) throws Throwable {
        String methodName = method.getName();
        if (methodName.equals("getAccounts")) {
            bank.setAccounts(getAccountsFromDatabase());
            
            return bank.getAccounts();
        } else if (methodName.equals("setAccounts")) {
            bank.setAccounts((Collection<Account>) args[0]);
            setAccountsToDatabase(bank.getAccounts());
            
            return null;
        } else {
            ...
        }
    }
    
    // 세부사항은 여기에 이어짐
    protected Collection<Account> getAccountsFromDatabase() { ... }
    protected void setAccountsToDatabase(Collection<Account> accounts) { ... }
}
```

```java
Bank bank = (Bank) Proxy.newProxyInstance(
    Bank.class.getClassLoader(),
    new Class[] { Bank.class },
    new BankProxyHandler(new BankImpl())
);
```

위 코드에서는 프록시로 감쌀 `Bank` 인터페이스와 비즈니스 논리를 구현하는 `BankImpl` POJO(Plain Old Java Object)을 정의했습니다. 프록시 API에 `InvocationHandler`를 넘겨줘서 호출되는 `Bank` 메서드를 구현하는 데 사용됩니다.

### 순수 자바 AOP 프레임워크

대부분의 프록시 코드는 판박이라 도구로 자동화할 수 있습니다. 순수 자바 관점을 구현하는 스프링 AOP 등과 같은 프레임워크는 내부적으로 자바 프록시를 사용합니다. 스프링은 비즈니스 논리를 POJO로 구현하며, POJO는 순수하게 도메인에 초점을 맞춥니다. 따라서 테스트가 개념적으로 쉽고 간단하며, 사용자 스토리에 맞게 구현하기 쉬워 코드를 보수하고 개선하기 편합니다.

### 테스트 주도 시스템 아키텍처 구축

AOP처럼 관점으로 관심사를 분리하는 방식은 그 위력이 막강합니다. 애플리케이션 도메인 논리를 POJO로 작성할 수 있다면, 즉 코드 수준에서 아키텍처 관심사를 분리할 수 있다면 진정한 테스트 주도 아키텍처 구축이 가능해집니다.

다시 말해, 아주 단순하면서도 멋지게 분리된 아키텍처로 소프트웨어 프로젝트를 진행해 결과물을 재빨리 출시한 후, 기반 구조를 추가하며 조금씩 확장해나가도 괜찮습니다. 그렇다고 아무 방향 없이 프로젝트에 뛰어들어도 좋다는 의미는 아닙니다. 프로젝트를 시작할 때는 일반적인 범위, 목표, 일정 그리고 결과로 내놓을 시스템의 일반적인 구조까지 생각해야 합니다. 하지만 변하는 환경에 대처해 진로를 변경할 능력도 반드시 유지해야 합니다.

### 의사 결정을 최적화하라

모듈을 나누고 관심사를 분리하면 지엽적인 관리와 결정이 가능해집니다. 아주 큰 시스템에서는 한 사람이 모든 결정을 내리기 어렵기 때문에, 가장 적합한 사람에게 책임을 맡기는 것이 가장 좋습니다. 최대한 정보를 모아 최선의 결정을 내리기 위해서, 때때로 **가능한 마지막 순간까지 결정을 미루는 방법이 최선**이라는 사실을 까먹곤 합니다.

### 명백한 가치가 있을 때 표준을 현명하게 사용하라

표준을 사용하면 아이디어와 컴포넌트를 재사용하기 쉽고, 적절한 경험을 가진 사람을 구하기 쉬우며, 좋은 아이디어를 캡슐화하기 쉽고, 컴포넌트를 엮기 쉽습니다. 하지만 때로는 표준을 만드는 시간이 너무 오래 걸려 업계가 기다리지 못하기도 하고, 어떤 표준은 원래 표준을 제정한 목적을 잊어버리기도 합니다. 따라서 아주 과장되게 포장된 표준에 집착하는 경우들을 경계해야합니다.

### 시스템은 도메인 특화 언어가 필요하다

DSL(Domain-Specific Language)은 간단한 스크립트 언어나 표준 언어로 구현한 API를 가리킵니다. 좋은 DSL은 도메인 개념과 그 개념을 구현한 코드 사이에 존재하는 의사소통 간극을 줄여 줍니다. 효과적으로 사용한다면 DSL은 추상화 수준을 코드 관용구나 디자인 패턴 이상으로 끌어올릴 수 있습니다. 그래서 개발자가 적절한 추상화 수준에서 코드 의도를 표현할 수 있습니다.

### 결론

시스템 역시 깨끗해야 합니다. 깨끗하지 못한 아키텍처는 도메인 논리를 흐리며 기민성을 떨어뜨립니다. 도메인 논리가 흐려지면 제품 품질이 떨어집니다 버그가 숨어들기 쉬어지고, 스토리를 구현하기 어려워지는 탓입니다. 기민성이 떨어지면 생산성이 낮아져 TDD가 제공하는 장점이 사라집니다.

모든 추상화 단계에서 의도는 명확히 표현해야 합니다. 그러려면 POJO를 작성하고 관점 혹은 과점과 유사한 메커니즘을 사용해 각 구현 관심사를 분리해야 합니다.

시스템을 설계하든 개별 모듈을 설계하든, 실제로 돌아가는 가장 단순한 수단을 사용해야 한다는 사실을 명심해야 합니다.



## 12장 창발성

코드 구조와 설계를 파악하기 쉬워지는, 단순한 설계 규칙 네 가지가 있습니다. 켄트 벡은 앞으로 얘기할 규칙들을 따르면 설계는 '단순하다'고 말합니다.

### 설계 규칙 1: 모든 테스트를 실행하라

무엇보다 먼저, 설계는 의도한 대로 돌아가는 시스템을 내놓아야 합니다. 테스트를 철저히 거쳐 모든 테스트 케이스를 항상 통과하는 시스템을 만들려 애쓰면 크기가 작고 목적 하나만 수행하는 클래스가 나옵니다. 테스트 케이스가 많을수록 개발자는 테스트가 쉽게 코드를 작성합니다.

결합도가 높으면 테스트 케이스를 작성하기 어렵습니다. 그래서 테스트 케이스를 많이 작성할수록 개발자는 DIP과 같은 원칙과 DI, 인터페이스, 추상화 등과 같은 도구를 사용해 결합도를 낮춰, 설계 품질을 높입니다.

놀랍게도 **테스트 케이스를 만들고 계속 돌려라**라는 간단하고 단순한 규칙을 따르면 시스템은 낮은 결합도와 높은 응집력이라는, 객체 지향 방법론이 지향하는 목표를 저절로 달성합니다.

### 설계 규칙 2: 중복을 없애라

테스트 케이스를 모두 작성했다면 이제 코드와 클래스를 정리할 차례입니다. 구체적으로는 코드를 점진적으로 리팩터링 해나갑니다.

```java
int size() { /* ... */ }
boolean isEmpty() { /* ... */ }
```

위의 메서드도 일종의 중복입니다. 깔끔한 시스템을 만들려면 단 몇 줄이라도 중복을 제거하겠다는 의지가 필요합니다.

```java
public void scaleToOneDimension(float desiredDimension, float imageDimension) {
    if (Math.abs(desiredDimension - imageDimension) < errorThreshold)
        return;
    float scalingFactor = desiredDimension / imageDimension;
    scalingFactor = (float)(Math.floor(scalingFactor * 100) * 0.01f);
  
    RenderedOpnewImage = ImageUtilities.getScaledImage(image, scalingFactor, scalingFactor);
    image.dispose();
    System.gc();
    image = newImage;
}

public synchronized void rotate(int degrees) {
    RenderedOpnewImage = ImageUtilities.getRotatedImage(image, degrees);
    image.dispose();
    System.gc();
    image = newImage;
}
```

다음은 위의 코드에서 동일한 부분을 제거해 리팩터링한 코드입니다.

```java
public void scaleToOneDimension(float desiredDimension, float imageDimension) {
    if (Math.abs(desiredDimension - imageDimension) < errorThreshold)
        return;
    float scalingFactor = desiredDimension / imageDimension;
    scalingFactor = (float) Math.floor(scalingFactor * 10) * 0.01f);
    replaceImage(ImageUtilities.getScaledImage(image, scalingFactor, scalingFactor));
}

public synchronized void rotate(int degrees) {
    replaceImage(ImageUtilities.getRotatedImage(image, degrees));
}

private void replaceImage(RenderedOp newImage) {
    image.dispose();
    System.gc();
    image = newImage;
}
```

이렇게 놔두면 클래스가 SRP를 위반하므로 새로 만든 메서드를 다른 클래스로 옮겨서 추상화 하는 게 좋습니다. 이런 식으로 소규모 재사용은 시스템 복잡도를 극적으로 줄여줍니다.

고차원 중복 제거를 위해서는 TEMPLATE METHOD 패턴을 자주 사용합니다.

```java
public class VacationPolicy {
    public void accrueUSDDivisionVacation() {
        // 지금까지 근무한 시간을 바탕으로 휴가 일수를 계산하는 코드
        // ...
        // 휴가 일수가 미국 최소 법정 일수를 만족하는지 확인하는 코드
        // ...
        // 휴가 일수를 급여 대장에 적용하는 코드
        // ...
    }
  
    public void accrueEUDivisionVacation() {
        // 지금까지 근무한 시간을 바탕으로 휴가 일수를 계산하는 코드
        // ...
        // 휴가 일수가 유럽연합 최소 법정 일수를 만족하는지 확인하는 코드
        // ...
        // 휴가 일수를 급여 대장에 적용하는 코드
        // ...
    }
}
```

이 경우 최소 법정 일수를 계산하는 코드만 제외하면 두 메서드는 거의 동일하므로 여기에 TEMPLATE METHOD 패턴을 적용해 중복을 제거합니다.

```java
abstract public class VacationPolicy {
    public void accrueVacation() {
        caculateBseVacationHours();
        alterForLegalMinimums();
        applyToPayroll();
    }

    private void calculateBaseVacationHours() { /* ... */ };
    abstract protected void alterForLegalMinimums();
    private void applyToPayroll() { /* ... */ };
}

public class USVacationPolicy extends VacationPolicy {
    @Override protected void alterForLegalMinimums() {
        // 미국 최소 법정 일수를 사용한다.
    }
}

public class EUVacationPolicy extends VacationPolicy {
    @Override protected void alterForLegalMinimums() {
        // 유럽연합 최소 법정 일수를 사용한다.
    }
}
```

하위 클래스는 중복되지 않는 정보만 제공해 빠진 구멍을 메우는 느낌입니다.

### 설계 규칙 3: 프로그래머 의도를 표현해라

코드를 짜는 동안에는 스스로가 문제에 푹 빠져 코드를 구석구석 이해하므로 자신이 이해하는 코드를 짜기는 쉽습니다. 하지만 나중에 코드를 유지보수할 사람이 코드를 짜는 사람만큼이나 문제를 깊이 이해할 가능성은 희박합니다.

소프트웨어 프로젝트 비용 중 대다수는 장기적인 유지보수에 들어갑니다. 코드를 변경하면서 버그의 싹을 심지 않으려면 유지보수 개발자가 시스템을 제대로 이해해야 합니다.

우선, 좋은 이름을 선택합니다. 이름과 기능이 완전히 딴판인 클래스나 함수로 유지보수 담당자를 놀라게 해서는 안 됩니다.

둘째, 함수와 클래스 크기를 가능한 줄입니다. 작은 클래스와 작은 함수는 이름 짓기도 쉽고, 구현하기도 쉽고, 이해하기도 쉽습니다.

셋째, 표준 명칭을 사용합니다. 예를 들어, 디자인 패턴은 의사소통과 표현력 강화가 주요 목적입니다. 클래스가 COMMAND나 VISITOR과 같은 표준 패턴을 사용해 구현된다면 클래스 이름에 패턴 이름을 넣어줍니다.

넷째, 단위 테스트 케이스를 꼼꼼히 작성합니다. 테스트 케이스는 소위 '예제로 보여주는 문서' 입니다. 다시 말해, 잘 만든 테스트 케이스를 읽어보면 클래스 기능이 한눈에 들어옵니다.

가장 중요한 방법은 노력입니다. 코드만 돌린 후 다음 문제로 직행하는 사례가 너무나도 흔합니다. 큰 함수를 작은 함수 여럿으로 나누고, 자신의 코드에 조금만 더 주의를 기울여야 합니다.

### 설계 규칙 4: 클래스와 메서드 수를 최소로 줄여라

중복을 제거하고, 의도를 표현하고, SRP를 준수한다는 개념도 극단으로 치달으면 득보다 실이 많이집니다. 때로는 무의미하고 독단적인 정책 탓에 클래스 수와 메서드 수가 늘어나기도 합니다. 가능한 독단적인 견해는 멀리하고 실용적인 방식을 택하는 편이 좋습니다. 목표는 함수와 클래스 크기를 작게 유지하면서 동시에 시스템 크기도 작게 유지하는 데 있습니다.



## 13장 동시성

### 동시성이 필요한 이유?

동시성은 결합(coupling)을 없애는 전략입니다. 즉, 무엇(what)과 언제(when)를 분리하는 전략입니다. 이 둘을 분리하면 애플리케이션의 구조와 효율이 극적으로 나아집니다. 구조적인 과넞ㅁ에서 프로그램은 작은 협력 프로그램 여럿으로 보이게 됩니다.

하지만 구조적 개선만을 위해 동시성을 채택하는 건 아닙니다. 어떤 시스템은 응답 시간과 작업 처리량(throughput) 개선을 위해 직접적인 동시성 구현이 불가피합니다. 예를 들어 한 번에 한 사용자를 1초동안 처리하는 시스템이 있다고 가정할 때, 사용자가 소수라면 시스템이 아주 빨리 반응할 수 있지만 수가 늘어날수록 시스템이 응답하는 속도도 늦어집니다. 수백명 뒤에 줄 서려는 사용자는 아마 없을 것 입니다. 많은 사용자들을 동시에 처리ㅏ면 시스템 응답 시간을 높일 수 있습니다.

### 미신과 오해

다음은 동시성과 관련한 일반적인 미신과 오해입니다.

- 동시성은 항상 성능을 높여준다.

  대기 시간이 아주 길어 여러 스레드가 프로세서를 공유할 수 있거나, 여러 프로세서가 동시에 처리할 독립적인 계산이 충분히 많은 경우에만 성능이 높아집니다. 그러나 어느 쪽도 일상적으로 발생하는 상황은 아닙니다.

- 동시성을 구현해도 설계는 변하지 않는다.

  단일 스레드 시스템과 다중 스레드 시스템은 설계가 판이하게 다릅니다. 일반적으로 무엇과 언제를 분리하면 시스템 구조가 크게 달라집니다.

- 웹 또는 EJB 컨테이너를 사용하면 동시성을 이해할 필요가 없다.

  실제로는 컨테이너가 어떻게 동작하는지, 어떻게 동시 수정, 데드락 등과 같은 문제를 피할 수 있는지를 알아야만 합니다.

다음은 동시성과 관련된 타당한 생각 몇 가지입니다.

- 동시성은 다소 부하를 유발한다.

  성능 측면에서 부하가 걸리며, 코드도 더 짜야 합니다.

- 동시성은 복잡하다.

- 일반적으로 동시성 버그는 재현하기 어렵다.

  그래서 종종 진짜 결함으로 간주되지 않고 일회성 문제로 여겨 무시하기 쉽습니다.

- 동시성을 구현하려면 흔히 근본적인 설계 전략을 제고해야 한다.

### 단일 책임 원칙(SRP) - 동시성 방어 원칙 1

SRP는 주어진 메서드/클래스/컴포넌트를 변경할 이유가 하나여야 한다는 원칙입니다. 동시성은 복잡성 하나만으로도 따로 분리할 이유가 충분합니다. 따라서 동시성을 구현할 때는 다음 몇 가지를 고려합니다.

- 동시성 코드는 독자적인 개발, 변경, 조율 주기가 있다.
- 동시성 코드에는 독자적인 난관이 있다.
- 잘못 구현한 동시성 코드는 별의별 방식으로 실패한다.

### 따름(corollary) 정리 - 동시성 방어 원칙 2

객체 하나를 공유한 두 스레드가 각각 동일 필드를 수정하면 서로 간섭이 일어나므로 예상치 못한 결과가 나옵니다. 이를 해결하는 방안으로 공유 객체를 사용하는 코드 내 임계 영역을 `synchronized` 키워드를 사용함으로써 보호합니다. 그렇다고 임계 영역을 마구 늘리면 안됩니다. 보호할 영역을 빼먹을 수 있고, 올바르게 보호했는지 등 확인해야할 것들이 많아지기 때문에 공유 자료를 최대한 줄이는 것이 좋습니다.

공유 자료를 줄이려면 처음부터 공유하지 않는 방법이 제일 좋습니다. 때때로는 객체를 복사해 읽기 전용으로 사용하는 방법이 가능합니다. 또 어떤 경우에는 각 스레드가 객체를 복사해 사용한 후 한 스레드가 해당 사본에서 결과를 가져오는 방법도 가능합니다.

이외에도 자시만의 세상에 존재하는 스레드를 구현하는 것도 방법입니다. 즉, 다른 스레드와 자료를 공유하지 않도록 만듭니다. 각 스레드는 클라이언트 요청 하나를 처리하여 모든 정보를 비공유하며 로컬 변수에 저장하는 방식입니다.

### 라이브러리를 이해하라

자바 5는 스레드 코드를 위한 기능을 지원합니다.

- 스레드 환경에 안전한 컬렉션을 사용한다.

  일례로, `ConcurrentHashMap`은 거의 모든 상황에서 `HashMap`보다 빠릅니다. 동시 읽기/쓰기를 지원하며, 자주 사용하는 복합 연산을 다중 스레드 상에서 안전하게 만든 메서드로 제공합니다.

- 서로 무관한 작업을 수행할 때는 `executor` 프레임워크를 사용한다.

- 가능하다면 스레드가 차단(blocking)되지 않는 방법을 사용한다.

- 일부 클래스 라이브러리는 스레드에 안전하지 못하다.

### 실행 모델을 이해하라

먼저 스레드 관련 기본 용어를 설명하겠습니다.

| 용어                         | 설명                                                         |
| ---------------------------- | ------------------------------------------------------------ |
| 한정된 자원 (Bound Resource) | 다중 스레드 환경에서 사용하는 자원으로, 크기나 숫자가 제한적입니다. 데이터베이스 연결, 길이가 일정한 읽기/쓰기 버퍼 등이 예입니다. |
| 상호 배제 (Mutual Exclusion) | 한 번에 한 스레드만 공유 자료나 공유 자원을 사용할 수 있는 경우를 가리킵니다. |
| 기아 (Starvation)            | 한 스레드나 여러 스레드가 굉장히 오랫동안 혹은 영원히 자원을 기다립니다. 예를 들어, 항상 짧은 스레드에게 우선선위를 준다면, 짧은 스레드가 지속적으로 이어질 경우, 긴 스레드가 기아 상태에 빠집니다. |
| 데드락 (Deadlock)            | 여러 스레드가 서로가 끝나기를 기다립니다. 모든 스레드가 각기 필요한 자원을 다른 스레드가 점유하는 바람에 어느 쪽도 더 이상 진행하지 못합니다. |
| 라이브락 (Livelock)          | 락을 거는 단계에서 각 스레드가 서로를 방해합니다. 스레드는 계속해서 진행하려 하지만, 공명(resonance)으로 인해, 굉장히 오랫동안 혹은 영원히 진행하지 못합니다. |

그리고 대표적으로 다중 스레드 프로그래밍에서 사용하는 실행 모델이 몇 가지 있습니다.

- 생산자 - 소비자
- 읽기 - 쓰기
- 식사하는 철학자들

### 동기화하는 메서드 사이에 존재하는 의존성을 이해하라

자바 언어는 개별 메서드를 보호하는 `synchronzied`라는 개념을 지원하지만, 공유 클래스 하나에 동기화된 메서드가 여럿이라면 구현이 올바른지 다시 한 번 확인하는 게 좋습니다.

공유 객체 하나에 여러 메서드가 필요한 상황도 생깁니다. 그럴 때는 다음 세 가지 방법을 고려합니다.

- 클라이언트에서 잠금

  클라이언트에서 첫 번째 메서드를 호출하기 전에 서버를 잠급니다. 마지막 메서드를 호출할 때까지 잠금을 유지합니다.

- 서버에서 잠금

  서버에다 `서버를 잠그고 모든 메서드를 호출한 후 잠금을 해제하는` 메서드를 구현합니다. 클라이언트는 이 메서드를 호출합니다.

- 연결(Adapted) 서버

  잠금을 수행하는 중간 단계를 생성합니다. '서버에서 잠금' 방식과 유사하지만 원래 서버는 변경하지 않습니다.

### 동기화하는 부분을 작게 만들어라

락은 스레드를 지연시키고 부하를 가중시킵니다. 그러므로 `synchronized`문을 여기저기 남발하지 않고, 임계 영역은 확실히 보호하도록 그 수를 최대한 줄여야 합니다.

### 올바른 종료 코드는 구현하기 어렵다

종료 코드를 개발 초기부터 고민하고 동작하게 초기부터 구현해야 합니다. 생각보다 오래 걸리고 어려우므로, 이미 나온 알고리즘을 검토하는 편이 좋습니다.

### 스레드 코드 테스트하기

스레드가 하나인 프로그램은 충분한 테스트로 위험을 낮출 수 있지만, 스레드가 둘 이상으로 늘어나면 상황은 급격하게 복잡해집니다. 되도록이면 문제를 노출하는 테스트 케이스를 작성하도록 애써야 합니다. 그리고 프로그램 설정과 시스템 설정과 부하를 바꿔가며 자주 돌려봐야 합니다.

고려할 사항이 아주 많은데, 아래에 몇 가지 구체적인 지침을 살펴보겠습니다.

- 말이 안 되는 실패는 잠정적인 스레드 문제로 취급하라

  대다수 개발자는 스레드가 다른 코드와 교류하는 방식을 직관적으로 이해하지 못합니다. 스레드 코드에 잠입한 버그로 인한 실패는 재현하기 아주 어렵습니다. 그래서 대부분 단순한 일회성 문제로 치부하고 무시합니다. 일회성 문제를 계속 무시한다면 잘못된 코드 위에 코드가 계속 쌓이게 됩니다.

- 다중 스레드를 고려하지 않은 순차 코드부터 제대로 돌게 만들자.

  당연한 얘기이지만, 스레드 환경 밖에서 코드가 제대로 도는지 반드시 확인합니다.일반적인 방법으로는, 스레드가 호출하는 POJO(Plain Old Java Object)를 만듭니다. POJO는 스레드를 모르기 때문에 스레드 환경 밖에서 테스트가 가능합니다. 더불어 스레드 환경 밖에서 생기는 버그와 스레드 환경에서 생기는 버그를 동시에 디버깅하면 안됩니다. 먼저 스레드 환경 밖에서 코드를 올바로 돌리는 것이 우선 입니다.

- 다중 스레드를 쓰는 코드 부분을 다양한 환경에 쉽게 끼워 넣을 수 있도록 스레드 코드를 구현하라.

  - 한 스레드로 실행하거나, 여러 스레드로 실행하거나, 실행 중 스레드 수를 바꿔봅니다.
  - 스레드 코드를 실제 환경이나 테스트 환경에서 돌려봅니다.
  - 테스트 코드를 빨리, 천천히, 다양한 속도로 돌려봅니다.
  - 반복 테스트가 가능하도록 테스트 케이스를 작성합니다.

- 다중 스레드를 쓰는 코드 부분을 상황에 맞춰 조정할 수 있게 작성하라.

  적절한 스레드 개수를 파악하려면 상당한 시행착오가 필요합니다. 스레드 개수를 조율하기 쉽도록 코드를 구현합니다. 프로그램이 돌아가는 도중에 스레드 개수를 변경하는 방법도 고려합니다. 혹은 프로그램 처리율과 효율에 따라 스스로 스레드 개수를 조율하는 코드도 고민합니다.

- 프로세서 수보다 많은 스레드를 돌려보라.

  시스템이 스레드를 스와핑할 때도 문제가 발생합니다. 스와핑이 잦을수록 임계 영역을 빼먹은 코드나 데드락을 일으키는 코드를 찾기 쉬워집니다.

- 다른 플랫폼에서 돌려보라.

  다중 스레드 코드는 플랫폼에 따라 다르게 돌아갑니다. 따라서 코드가 돌아갈 가능성이 있는 플랫폼 전부에서 테스트를 수행해야 마땅합니다.

- 코드에 보조 코드를 넣어 돌려라. 강제로 실패를 일으키게 해보라.

  스레드 버그가 산발적이고 우발적이고 재현이 어려운 이유는 코드가 실행되는 수천 가지 경로 중에 아주 소수만 실패하기 때문입니다. 이렇듯 드물게 발생하는 오류를 좀 더 자주 일으키려면 보조 코드를 추가해 코드가 실행되는 순서를 바꿔줍니다. 예를 들어, `Object.wati()`, `Object.sleep()`, `Object.yield()`, `Object.priority()`등과 같은 메서드를 추가해 코드를 다양한 순서로 실행합니다. 코드에 보조 코드를 추가하는 방법은 직접 구현하거나, 자동화하는 방법이 있습니다.

