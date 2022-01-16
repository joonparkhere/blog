---
title: "[디자인 패턴 GURU] Factory 패턴"
summary: Refactoring Guru 서적을 기반으로한 디자인 패턴 학습 Factory 패턴
date: 2021-12-30
pin: false
image: images/factory-method-en.png
tags:
- Software Enginerring
- Design Pattern
- Go
- Java
---

## Factory Method

### Intent

Super-class 형태로 인터페이스를 이용해 객체를 생성하면서도 Sub-class에서는 생성될 객체의 세부 타입을 변경할 수 있는 방법이다.

### Problem

애플리케이션에서 군수 관리를 위한 객체 생성을 구현한다고 하자. 처음에는 오직 트럭을 통한 방안만 구현하여 구현 코드는 `Truck` 클래스 내에 위치한다.

![문제점[^1]](images/factory-method-problem1-en.png)

그러나 서비스 규모가 커지면서 배를 통한 운송도 필요하게 되어, `Ship` 클래스를 구현해야 한다. 하지만 관련 구현이 모두 `Truck` 클래스에 의존적이므로 전반적인 코드 수정이 필연적이다. 이 상태가 유지된다면 다른 운송 수단이 추가될 때마다 이 문제는 점점 거대해지며, 구현 코드는 수많은 조건문의 분기들로 이루어져 코드의 악취가 나게 된다.

### Solution Structure

![Truck과 Ship 클래스 예제](images/factory-method-solution2-en.png)

직접 구현 객체를 생성하는 부분을 Factory Method를 호출하여 생성하도록 수정한다. 이때 Factory Method의 반환 객체는 Super-class가 된다. Sub-class에서는 Factory Method를 오버라이딩하여 각 역할에 맞는 객체를 반환하도록 구현하면 된다. 이때 Sub-class에서는 Super-class를 상속하는 객체만을 반환해야 하는 약간의 한계점이 발생한다. 이로써 실제 서비스 동작은 Sub-class들을 통해 이루어지지만 Factory Method를 이용해 확장에 용이하도록 구조를 변경할 수 있다. 더불어 클라이언트는 모든 구현 객체의 상세 로직을 알 필요 없이, 오로지 그 객체의 역할만을 알면 된다.

![코드 구조](images/fatcory-method-structure.png)

1. `Product`는 구현하고자 하는 객체의 역할에 집중한다.

2. `Concrete Prodocuts`들은 `Product` 인터페이스의 각 구현체들이다.

3. `Creator`는 인터페이스인 `Product`를 반환하는 Factory Method가 정의된 클래스이다.

   Factory Method가 알맞은 `Concrete Creator`를 호출할 수 있도록 구현체를 식별할 수 있는 파라미터가 전달되어야 한다.

4. `Concrete Creator`들은 Base Factory Method를 오버라이딩하여 각 구현체를 반환한다.

### Applicability

- 사전에 정확히 어떠한 구현체 혹은 의존성이 필요한 지 알 수 없을 때 사용
- 클라이언트들에게 현재 개발한 코드의 향후 확장성을 제공하고자 할 때 사용
- 매번 새로운 객체를 생성하지 않고 존재하는 객체를 재사용하여 리소스를 절약
  1. 기존에 생성된 객체들에 대한 정보를 기록
  2. 객체 요청이 올 때, 기록된 정보 중에 사용되지 않고 있는 (Free) 객체를 탐색
  3. 그러한 객체가 있다면 반환하고, 없다면 새로운 객체를 생성하여 반환

> 너무 많은 Sub-class가 있는 경우 지나치게 코드가 복잡해질 수 있다. 이를 방지하기 위해 Sub-class 끼리 계층적인 구조를 갖도록 하는 것을 권장한다.

### Code Example

- Practice with Go

  [Github Repository](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-creational-pattern/factory-method)

- Spring Framework

  ```java
  public interface BeanFactory {
      Object getBean(String name) throws BeansException;
      <T> T getBean(String name, Class<T> requiredType) throws BeansException;
      // ...
  }
  ```
  
  ```java
  public class SimpleJndiBeanFactory extends JndiLocatorSupport implements BeanFactory {
      @Override
  	public Object getBean(String name) throws BeansException {
  		return getBean(name, Object.class);
  	}
      
      @Override
  	public <T> T getBean(String name, Class<T> requiredType) throws BeansException {
  		try {
  			if (isSingleton(name))
  				return doGetSingleton(name, requiredType);
  			else
  				return lookup(name, requiredType);
  		}
  		catch (Exception ex) { ... }
  	}
      // ...
  }
  ```

[^1]: [Factory Method Origin](https://refactoring.guru/design-patterns/factory-method)

