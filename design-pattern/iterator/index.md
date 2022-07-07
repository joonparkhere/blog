---
title: "[디자인 패턴 GURU] Iterator 패턴"
date: 2022-01-16
pin: false
tags:
- Software Engineering
- Design Pattern
- Go
- Java
---

## Iterator

### Intent

이 패턴은 Collection에 대한 내부 정보없이 안에 속한 Element들을 훑기 위해 사용하는 방법이다.

### Problem

단순한 구조의 Collection을 넘어서, Tree 구조처럼 일련의 순서가 모호한 Collection인 경우 내부 Element를 훑는 순서는 BFS, DFS 등 다양할 수 있다. Collection 내 Element들을 접근하는 다양한 알고리즘들을 구현하다보면 Collection 클래스는 그 자체의 역할이 모호해지며, 각 알고리즘들은 Collectino마다 구현 방법에 따라 달라지기 때문에 해당 알고리즘들을 Genric하게 뽑아내기도 쉽지 않다.

### Solution Structure

![Iterator Brief Structure[^1]](images/iterator-solution1.png)

주요 포인트는 Element들을 훑는 로직을 Collection 클래스와 분리하고자 하는 점이다.

![Iterator Structure[^1]](images/iterator-structure.png)

1. `Iterator` 인터페이스는 내부 Element들을 훑기 위한 동작의 정의가 되어있다.
2. `Concrete Iterator`들은 특정 알고리즘을 구현한다. 다른 `Iterator`와는 독립적으로 동작하여 모든 Element들을 훑도록 보장되어야 한다.
3. `Collection` 인터페이스는 `Iterator`에 접근할 수 있는 메서드를 포함한다.
4. `Concrete Collection`은 유저가 전달한 요청에 맞게 `Concrete Iterator`를 참조하는 `Collection`이다.

### Code Example - [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-behavioral-pattern/iterator)

어느 식당에서 요리 메뉴 (Element) 가 있고 점심 메뉴 (Collection) 를 판매한다고 할 때, 판매중인 메뉴들을 모두 훑는 (Iterator) 예제이다.

```go
type dish struct {
	name  string
	price int
}

func newDish(name string, price int) *dish {
	return &dish{
		name:  name,
		price: price,
	}
}
```

```go
type iterator interface {
	hasNext() bool
	getNext() *dish
}
```

```go
type dishIterator struct {
	index  int
	dishes []*dish
}

func newDishIterator(dishes []*dish) *dishIterator {
	return &dishIterator{
		index:  0,
		dishes: dishes,
	}
}

func (i *dishIterator) hasNext() bool {
	if i.index < len(i.dishes) {
		return true
	}
	return false
}

func (i *dishIterator) getNext() *dish {
	if i.hasNext() {
		dish := i.dishes[i.index]
		i.index += 1
		return dish
	}
	return nil
}
```

```go
type menu interface {
	createIterator() iterator
}
```

```go
type lunchMenu struct {
	dishes []*dish
}

func newLunchMenu() *lunchMenu {
	return &lunchMenu{}
}

func (l *lunchMenu) addDish(dish *dish) *lunchMenu {
	l.dishes = append(l.dishes, dish)
	return l
}

func (l *lunchMenu) createIterator() iterator {
	return newDishIterator(l.dishes)
}
```

아래는 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	soup := newDish("soup", 3000)
	beef := newDish("beef", 10000)
	fish := newDish("fish", 8000)
	cake := newDish("cake", 5000)

	lunchMenu := newLunchMenu()
	lunchMenu.addDish(soup).
		addDish(beef).
		addDish(fish).
		addDish(cake)

	iterator := lunchMenu.createIterator()
	for iterator.hasNext() {
		dish := iterator.getNext()
		fmt.Printf("Dish is %s and price is %d\n", dish.name, dish.price)
	}
}
```

### Real Example

실 적용 예제로, Java의 `Enumeration` 인터페이스가 있다.

```java
public interface Enumeration<E> {
    boolean hasMoreElements();

    E nextElement();
    
    default Iterator<E> asIterator() {
        return new Iterator<>() {
            @Override public boolean hasNext() {
                return hasMoreElements();
            }
            @Override public E next() {
                return nextElement();
            }
        };
    }
}
```

구현체로 Spring에서 제공하는 `Enumerator`가 있으며, Spring Security에서 Request Header 이름들을 훑을 때 사용한다.

```java
public class Enumerator<T> implements Enumeration<T> {
    private Iterator<T> iterator = null;
    
	public Enumerator(Collection<T> collection) {
		this(collection.iterator());
	}
    
    // ...
    
    @Override
	public boolean hasMoreElements() {
		return (this.iterator.hasNext());
	}
    
    @Override
	public T nextElement() throws NoSuchElementException {
		return (this.iterator.next());
	}
}
```

```java
class SavedRequestAwareWrapper extends HttpServletRequestWrapper {
    // ...
    
    @Override
	public Enumeration getHeaderNames() {
		return new Enumerator<>(this.savedRequest.getHeaderNames());
	}
}
```

### Note

- 복잡한 자료구조 내부를 추상화하며 활용할 수 있도록 하고 싶을 때 사용

[^1]: [Iteratort Origin](https://refactoring.guru/design-patterns/iterator)
