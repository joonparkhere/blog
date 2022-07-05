## Flyweight

### Intent

이 패턴은 다수의 객체들이 동일한 값을 갖는 데이터를 공유하도록 만들어 리소스를 절약할 수 있는 방법이다.

### Problem

![Example Problem[^1]](images/flyweight-problem-en.png)

간단한 슈팅 게임으로, 자그마한 총알들이 날아가는 게임을 만들고자 한다. 이때 총알에는 위치, 이동 방향, 속도, 색상, 형태 등의 정보들이 필요하다. 여기서 슈팅 게임이니만큼 많은 수의 총알들이 게임 내에서 필요할텐데, 메모리 용량이 적은 모바일 기기나 임베디드 기기 등은 게임에서 요구하는 메모리를 충족하지 못하고 중단될 것이다.

### Solution Structure

![Example Solution[^1]](images/flyweight-solution1-en.png)

총알 객체를 유심히 살펴보면 각 객체마다 다른 값을 갖는 필드 - 위치, 이동 방향, 속도 - 가 있고 그렇지 않는, 즉 공통의 값을 갖는 필드가 있다. 예를 들어 총알의 색상이나 형태를 결정하는 `color`,` sprite` 필드가 될 수 있는데, 특히나 `sprite` 필드는 큰 용량을 차지한다. 이러한 필드들을 묶어서 공유하도록 객체 구조를 수정하면 총알 객체들의 요구 메모리 용량이 줄어든다.

![Example Solution Structure[^1]](images/flyweight-solution3-en.png)

객체가 갖는 값 중, 주로 정적인 데이터들을 **Intrinsic State**라고 칭한다. 이들은 객체 내부에 위치해서 외부에서는 읽기만 가능하고, 변경할 수는 없다. 이와 대비되는 개념으로, 외부로부터 자주 변경되는 데이터들은 **Extrinsic State**라고 한다.

Flyweight 패턴은 Extrinsic State를 객체 내부에 보관하지 않도록 한다. 오롯이 Inttrinsic State만 객체 내에 존재하며, 이를 다른 Context에서도 공유하도록 한다. 따라서 Flyweight 패턴의 객체 내의 값은 수정되면 안된다. 처음 생성될 때 초기화된 이후로는 읽기만 가능해야 한다.

![Structure[^1]](images/flyweight-structure.png)

1. `Flyweight` 클래스는 원래 객체가 갖는 필드 중 여러 객체들에게 공유되는 값 (Intrinsic State) 을 갖는다. 메서드의 파라미터로 받아오는 값들은 Extrinsic State 이다.
2. `Context` 클래스는 각 객체마다 고유한 값을 갖는 Extrinsic State를 갖고, `Flyweight` 클래스를 참조한다. 이 두가지를 이용하여 원래 객체의 Full State를 나타낼 수 있다.
3. 객체가 해야하는 동작은 주로 `Flyweight` 클래스에 위치한다. 따라서 `operation()` 메서드를 호출할 때 Extrinsic State를 파라미터로 전달하는 형태를 취한다.
4. `Flyweight Factory`는 이미 존재하는 `Flyweight` Pool을 관리한다. `Client`가 직접 `Flyweight`를 생성하지 않고, 이 클래스를 통한다. `Flyweight`를 식별할 수 있는 약간의 `Instrinsic State`를 파라미터로 전달하여 이미 존재하는 객체이면 그걸 바로 반환하고, 존재하지 않는다면 새롭게 생성해서 반환한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/flyweight)

게임에서 `Terrorist`와 `CounterTerrorist` 역할이 있고, 각 플레이어는 복장, 역할, 게임 내에서의 위치 등의 필드를 갖는다. 이 중 복장 (`dress`) 은 플레이어 객체 사이에서 공유될 수 있는 값이기 때문에 Flyweight 패턴을 적용한다. 더불어 `dress` 객체 접근시 Context에서 공유될 수 있도록 Factory 패턴을 적용했다.

```go
type playerStatus string

const (
	terrorist      = playerStatus("T")
	countTerrorist = playerStatus("CT")
)

type player struct {
	dress     dress
	status    playerStatus
	latitude  int
	longitude int
}

func newPlayer(status playerStatus, dressType dressType) *player {
	factory := getDressFactoryInstance()
	dress, _ := factory.getDressByType(dressType)
	return &player{
		dress:  dress,
		status: status,
	}
}

func (p *player) newLocation(lat, long int) {
	p.latitude = lat
	p.longitude = long
}
```

```go
type dress interface {
	getColor() string
}
```

```go
type terroristDress struct {
	color string
}

func newTerroristDress() *terroristDress {
	time.Sleep(time.Second)
	return &terroristDress{
		color: "red",
	}
}

func (t *terroristDress) getColor() string {
	return t.color
}
```

```go
type counterTerroristDress struct {
	color string
}

func newCounterTerroristDress() *counterTerroristDress {
	time.Sleep(time.Second)
	return &counterTerroristDress{
		color: "green",
	}
}

func (c *counterTerroristDress) getColor() string {
	return c.color
}
```

- `dress`객체의 리소스를 표현하기 위해 `terroristDress`와 `counterTerroristDress`가 생성될 때 1초 동안 Sleep 하도록 설정했다.

```go
type dressType string

const (
	terroristDressType        = dressType("c")
	counterTerroristDressType = dressType("ct")
)

var (
	dressFactorySingleInstance = &dressFactory{
		dressMap: make(map[dressType]dress),
	}
)

type dressFactory struct {
	dressMap map[dressType]dress
}

func getDressFactoryInstance() *dressFactory {
	return dressFactorySingleInstance
}

func (d *dressFactory) getDressByType(dressType dressType) (dress, error) {
	if d.dressMap[dressType] != nil {
		return d.dressMap[dressType], nil
	}

	if dressType == terroristDressType {
		d.dressMap[dressType] = newTerroristDress()
		return d.dressMap[dressType], nil
	}
	if dressType == counterTerroristDressType {
		d.dressMap[dressType] = newCounterTerroristDress()
		return d.dressMap[dressType], nil
	}

	return nil, fmt.Errorf("Wrong dress type passed")
}
```

- `dressType`을 갖는 `dress` 객체가 만들어진 적이 없는 경우에만 생성하고, 이미 존재한다면 존재하는 객체를 반환하여 리소스를 절약한다.

```go
type game struct {
	terrorists        []*player
	counterTerrorists []*player
}

func newGame() *game {
	return &game{
		terrorists:        make([]*player, 0),
		counterTerrorists: make([]*player, 0),
	}
}

func (g *game) addTerrorist(dressType dressType) {
	player := newPlayer(terrorist, dressType)
	g.terrorists = append(g.terrorists, player)
}

func (g *game) addCounterTerrorist(dressType dressType) {
	player := newPlayer(countTerrorist, dressType)
	g.counterTerrorists = append(g.counterTerrorists, player)
}
```

```go
func TestBefore(t *testing.T) {
	now := time.Now()

	game := newGame()
	for i := 0; i < 10; i++ {
		game.terrorists = append(game.terrorists, &player{
			dress:  newTerroristDress(),
			status: terrorist,
		})
	}
	for i := 0; i < 3; i++ {
		game.counterTerrorists = append(game.counterTerrorists, &player{
			dress:  newCounterTerroristDress(),
			status: countTerrorist,
		})
	}

	duration := time.Since(now)
	fmt.Printf("Duration: %s\n", duration) // Duration: 13.1093886s
}

func TestAfter(t *testing.T) {
	now := time.Now()

	game := newGame()
	for i := 0; i < 10; i++ {
		game.addTerrorist(terroristDressType)
	}
	for i := 0; i < 3; i++ {
		game.addCounterTerrorist(counterTerroristDressType)
	}

	duration := time.Since(now)
	fmt.Printf("Duration: %s\n", duration) // Duration: 2.0095587s
}
```

- Factory 패턴과 Flyweight 패턴을 적용해 리소스 낭비를 막고 공유하여 게임 실행에 필요한 리소스가 줄어듬을 확인할 수 있다.

### Real Example

아래는 Java의 `BigDecimal` 클래스와 `valueOf()` 메서드 일부이다.

```java
// Cache of common small BigDecimal values.
private static final BigDecimal ZERO_THROUGH_TEN[] = {
    new BigDecimal(BigInteger.ZERO,       0,  0, 1),
    new BigDecimal(BigInteger.ONE,        1,  0, 1),
    new BigDecimal(BigInteger.TWO,        2,  0, 1),
    new BigDecimal(BigInteger.valueOf(3), 3,  0, 1),
    new BigDecimal(BigInteger.valueOf(4), 4,  0, 1),
    new BigDecimal(BigInteger.valueOf(5), 5,  0, 1),
    new BigDecimal(BigInteger.valueOf(6), 6,  0, 1),
    new BigDecimal(BigInteger.valueOf(7), 7,  0, 1),
    new BigDecimal(BigInteger.valueOf(8), 8,  0, 1),
    new BigDecimal(BigInteger.valueOf(9), 9,  0, 1),
    new BigDecimal(BigInteger.TEN,        10, 0, 2),
};

// Cache of zero scaled by 0 - 15
private static final BigDecimal[] ZERO_SCALED_BY = {
    ZERO_THROUGH_TEN[0],
    new BigDecimal(BigInteger.ZERO, 0, 1, 1),
    new BigDecimal(BigInteger.ZERO, 0, 2, 1),
    new BigDecimal(BigInteger.ZERO, 0, 3, 1),
    new BigDecimal(BigInteger.ZERO, 0, 4, 1),
    new BigDecimal(BigInteger.ZERO, 0, 5, 1),
    new BigDecimal(BigInteger.ZERO, 0, 6, 1),
    new BigDecimal(BigInteger.ZERO, 0, 7, 1),
    new BigDecimal(BigInteger.ZERO, 0, 8, 1),
    new BigDecimal(BigInteger.ZERO, 0, 9, 1),
    new BigDecimal(BigInteger.ZERO, 0, 10, 1),
    new BigDecimal(BigInteger.ZERO, 0, 11, 1),
    new BigDecimal(BigInteger.ZERO, 0, 12, 1),
    new BigDecimal(BigInteger.ZERO, 0, 13, 1),
    new BigDecimal(BigInteger.ZERO, 0, 14, 1),
    new BigDecimal(BigInteger.ZERO, 0, 15, 1),
};

public static BigDecimal valueOf(long val) {
    if (val >= 0 && val < ZERO_THROUGH_TEN.length)
        return ZERO_THROUGH_TEN[(int)val];
    else if (val != INFLATED)
        return new BigDecimal(null, val, 0, 0);
    return new BigDecimal(INFLATED_BIGINT, val, 0, 0);
}

static BigDecimal zeroValueOf(int scale) {
    if (scale >= 0 && scale < ZERO_SCALED_BY.length)
        return ZERO_SCALED_BY[scale];
    else
        return new BigDecimal(BigInteger.ZERO, 0, scale, 1);
}
```

- 자주 쓰이는 객체는 `static`으로 먼저 만들어 놓고, 클라이언트 요청이 미리 만들어놓은 수의 범위에 있을 때 새로 객체를 만들지않고 바로 반환한다.

### Note

- 애플리케이션에 의해 생성되는 객체의 수가 많으며, 객체들이 공통된 값을 갖는 경우일 때 사용

[^1]: [Flyweight](https://refactoring.guru/design-patterns/flyweight)
