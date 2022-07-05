## Strategy

### Intent

이 패턴은 한 부류의 알고리즘을 각각 별도의 클래스로 분리하고 서로 상호전환이 되도록 만든다.

### Problem

네비게이션 서비스를 만들 때, 처음에는 도로 상의 경로만을 고려해 길을 찾아준다고 하자. 그러나 요구 사항이 늘어, 도보를 통한 경로와 자전거를 타고 갈 수 있는 경로에 대한 길 찾기도 해야한다. 이렇게 구현해야 하는 로직이 늘어나게 되면 해당 클래스의 책임이 비대해지고, 작업이 어려워진다.

### Solution Structure

![Example Solution[^1]](images/strategy-solution.png)

Strategy 패턴은 같은 부류에 속하지만 구현 방법이 다른 로직들을 별도의 클래스로 분리해낸다. Original 클래스인 `Context`는 이런 `Strategy`를 참조할 수 있는 필드를 가지며, 실 행동은 참조 중인 객체에 위임한다. 더불어 `Context` 객체는 어떤 `Strategy`를 선택해야 하는 지에 대한 책임을 지니지 않고 단지 외부로부터 주입받는다. 이렇게 함으로써 `Context`는 `Strategy`에 대한 의존성이 제거되고 캡슐화된다.

![Solution Structure[^1]](images/strategy-structure.png)

1. `Context`는 `Concrete Strategy`에 대한 참조 필드를 지니며, 이를 통해 소통한다.
2. `Strategy` 인터페이스는 `Context`가 `Strategy`의 세부 구현에 대해 모르도록 캡슐화 및 디커플링을 한다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/strategy)

캐시의 Replacement 알고리즘 선택에 대한 간략 예제이다.

```go
type Cache struct {
	storage     map[string]string
	algo        EvictionAlgo
	capacity    int
	maxCapacity int
}

func InitCache(a EvictionAlgo) *Cache {
	return &Cache{
		storage:     make(map[string]string),
		algo:        a,
		capacity:    0,
		maxCapacity: 2,
	}
}

func (c *Cache) SetAlgo(a EvictionAlgo) {
	c.algo = a
}

func (c *Cache) Add(key, value string) {
	if c.capacity == c.maxCapacity {
		c.evict()
	}

	c.capacity += 1
	c.storage[key] = value
}

func (c *Cache) Get(key string) string {
	value := c.storage[key]
	delete(c.storage, key)
	return value
}

func (c *Cache) evict() {
	c.algo.Evict(c)
	c.capacity -= 1
}
```

```go
type EvictionAlgo interface {
	Evict(c *Cache)
}
```

```go
type fifo struct {
}

func NewFIFO() *fifo {
	return &fifo{}
}

func (l *fifo) Evict(c *Cache) {
	fmt.Println("Evicting by fifo strategy")
}
```

```go
type lru struct {
}

func NewLRU() *lru {
	return &lru{}
}

func (l *lru) Evict(c *Cache) {
	fmt.Println("Evicting by lru strategy")
}
```

```go
type lfu struct {
}

func NewLFU() *lfu {
	return &lfu{}
}

func (l *lfu) Evict(c *Cache) {
	fmt.Println("Evicting by lfu strategy")
}
```

다음은 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	lfu := NewLFU()
	cache := InitCache(lfu)

	cache.Add("a", "1")
	cache.Add("b", "2")
	cache.Add("c", "3")

	lru := NewLRU()
	cache.SetAlgo(lru)

	cache.Add("d", "4")

	fifo := NewFIFO()
	cache.SetAlgo(fifo)

	cache.Add("e", "5")
}
```

### Real Example

자바의 `Comparator` 클래스는 주로 Collection 내의 요소들을 정렬할 때 아래처럼 로직을 주입받아서 동작하곤 한다.

```java
@FunctionalInterface
public interface Comparator<T> {
    int compare(T o1, T o2);
}
```

```java
Collections.sort(list, new Comparator() {
    @Override
    public int compare(Object o1, Object o2) {
        // compare logic for specific object
    }
})
```

### Note

- 여러 종류의 로직이 존재하고 런타임 상에 로직을 변경해야할 때 사용
- 비즈니스 로직과 세부 구현의 분리가 필요할 때 사용

> Client는 각 Strategy의 차이점을 알아야만 Context에 적절한 객체를 주입할 수 있다.

[^1]: [Strategy Origin](https://refactoring.guru/design-patterns/strategy)