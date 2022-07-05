## Prototype

### Intent

Prototype 패턴은 코드의 의존성 없이 특정 객체와 동일한 객체를 복사하고 싶을 때 사용한다.

### Problem

단순히 객체를 복사하기 위해서는 동일한 클래스의 빈 객체를 생성하고, 복사하고자 하는 객체가 가진 필드 값들과 동일하게 전부 세팅해야 한다. 그러나 만약 private 필드나 외부에서 접근할 수 없는 것이 있는 경우에는 문제가 발생할 수 있다. 더불어 이처럼 구현한다면, 객체 복사과정이 객체의 구현에 의존하게 된다.

### Solution Structure

Prototype 패턴은 객체 복사 과정을 해당 객체가 하도록 위임하는 형태이다.

![Prototype Structure[^1]](images/prototype-structure.png)

1. `Prototype` 인터페이스는 복사하는 역할을 갖는 메서드를 갖는다.
2. `Concrete Prototype`은 복사 역할의 메서드를 구현해야 한다.
3. `Client`는 인터페이스의 복사 역할의 메서드를 호출하여 객체 복사를 한다.

### Applicability

- 복사하고자 하는 객체에 의존하여 진행하고 싶지 않은 경우 이용

> 때때로 객체를 복사하는 과정에서 순환 참조가 발생할 수 있다.

### Code Example

- Practice with Go

  [Github Repository](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-creational-pattern/prototype)

- java.lang.Object

  ```java
  public class Object {
      public boolean equals(Object obj) {
          return (this == obj);
      }
      
      @IntrinsicCandidate
      protected native Object clone() throws CloneNotSupportedException;
      
      public String toString() {
          return getClass().getName() + "@" + Integer.toHexString(hashCode());
      }
      // ...
  }
  ```

  ```java
  public class ArrayList<E> extends AbstractList<E>
          implements List<E>, RandomAccess, Cloneable, java.io.Serializable
  {
      public Object clone() {
          try {
              ArrayList<?> v = (ArrayList<?>) super.clone();
              v.elementData = Arrays.copyOf(elementData, size);
              v.modCount = 0;
              return v;
          } catch (CloneNotSupportedException e) {
              // this shouldn't happen, since we are Cloneable
              throw new InternalError(e);
          }
      }
      // ...
  }
  ```

[^1]: [Prototype Origin](https://refactoring.guru/design-patterns/prototype)

