---
title: "[디자인 패턴 GURU] Composite 패턴"
summary: Refactoring Guru 서적을 기반으로 한 디자인 패턴 학습 Composite 패턴
date: 2022-01-07
pin: false
image: images/composite.png
tags:
- Software Engineering
- Design Pattern
- Go
---

## Composite

### Intent

Composite 패턴은 트리 구조로 객체들을 구성하고, 각 객체가 독립적으로 동작하도록 한다.

### Problem

만약 `Product`와 `Box` 객체가 있고, `Box`안에는 `Product` 혹은 또 하나의 `Box`가 있을 수 있다고 하자. 이러한 경우 아래 그림처럼 Recursive한 트리 구조 형태가 된다.

![Composite 문제 예제[^1]](images/composite-problem-en.png)

상자에 담긴 물건들의 가격의 총합을 알고 싶은 경우, 각각의 `Product` 구현 로직을 알고 있어야 함은 물론이고 내부 `Box`가 `Product` 혹은 `Box`를 포함하는 지에 대한 처리도 해야한다. 트리 구조의 규모가 거대해지면 사실상 불가능한 방안이다.

### Solution Structure

![Composite 구조[^1]](images/composite-structure-en.png)

1. `Component` 인터페이스는 트리 내에 위치한 요소들에 대한 공통 역할이다.

2. `Leaf`는 자식 요소가 존재하지 않는 객체이다. 실질적인 작업을 처리하게 된다.

3. `Container` (aka Composite) 는 자식 요소를 갖는 객체이다. 자식 요소는 보통 배열 형식의 필드값으로 갖는다.

   자식 요소의 구현 로직에 대해서는 의존하지 않되, `Component` 인터페이스를 통해 필요한 작업을 처리 혹은 위임할 수 있다.

### Code Example- [Go](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-structural-pattern/composite)

`file`과 `folder` 객체가 있고, `folder`는 또 다른 `folder` 혹은 `file`를 포함할 수 있다. `folder`에 포함된 모든 `file`을 검색하기 위해 공통 인터페이스 `component`가 필요하다.

```go
type component interface {
	search(string)
}
```

```go
type file struct {
	name string
}

func (f *file) search(keyword string) {
	fmt.Printf("Something or keyword %s in file %s\n", keyword, f.name)
}

func (f *file) getName() string {
	return f.name
}
```

```go
type folder struct {
	components []component
	name       string
}

func (f *folder) search(keyword string) {
	fmt.Printf("Searching recursively for keyword %s in folder %s\n", keyword, f.name)
	for _, composite := range f.components {
		composite.search(keyword)
	}
}

func (f *folder) add(c component) {
	f.components = append(f.components, c)
}
```

이를 이용한 테스트 케이스다.

```go
func TestAfter(t *testing.T) {
	file1 := &file{"file1"}
	file2 := &file{"file2"}
	file3 := &file{"file3"}

	folder1 := &folder{name: "folder1"}
	folder1.add(file1)

	folder2 := &folder{name: "folder2"}
	folder2.add(file2)
	folder2.add(file3)
	folder2.add(folder1)

	folder2.search("rose")
}
```

### Note

- 트리 구조의 객체들이 있고 간단하거나 복잡한 요소들을 동시에, 그리고 일관되게 처리하고자 할 때 사용

> 트리에서의 최하단 요소는 Component 인터페이스의 역할을 지니진 않지만 해당 인터페이스의 구현체이어야 하므로, ISP (Interface Segregation Principle) 을 위반한다.

[^1]: [Composite Origin](https://refactoring.guru/design-patterns/composite)
