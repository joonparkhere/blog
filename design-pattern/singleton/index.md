---
title: "[디자인 패턴 GURU] Singleton 패턴"
date: 2022-01-02
pin: false
tags:
- Software Enginerring
- Design Pattern
- Go
- Java
---

## Singleton

### Intent

Singleton 패턴은 클래스가 오직 하나의 인스턴스만을 갖고 전역 접근 가능하도록 하는 방법이다.

### Problem

아래의 두 가지 문제를 해결하고자 한다. 이때 Singleton 패턴은 SRP 원칙을 위배한다.

1. 클래스가 오직 하나의 인스턴스만을 갖도록 한다. 이때 일반적인 클래스 생성자는 항상 새로운 인스턴스를 반환하므로, 생성자 접근을 제어하고 다른 방식으로 인스턴스를 반환하는 로직 구현이 필요하다.
2. 그렇게 만들어진 인스턴스를 전역에서 접근 가능하도록 한다. 더불어 그 인스턴스가 오버라이팅되지 않도록 보호한다.

### Solution Structure

클래스의 기본 생성자는 외부에서 접근하지 못하도록 private 제어를 건다. 대신 private 생성자를 호출하거나 이미 생성된 인스턴스를 반환하는 정적 메서드를 제공한다.

![Singleton Structure[^1]](images/singleton-structure-en.png)

1. `Singleton` 클래스는 항상 동일한 인스턴스를 반환하도록 `getInstance`와 같은 정적 메서드를 포함한다.

   정적 메서드은 **lazy initialization**을 지원해야 한다. 메서드가 처음 호출될 때만 새 인스턴스를 생성한 후 정적 필드 값으로 넣어준다. 이후 메서드를 호출할 때는 동일한 인스턴스를 반환한다.

### Applicability

- 어느 클래스가 오직 하나의 인스턴스만을 갖도록 해야할 때 사용

> SRP (Single Responsibilty Principle) 을 위배한다. 이 패턴은 동시에 두 가지 문제를 해결하려 한다.
>
> 패턴 이용을 위해 추가적인 (어쩌면 지저분한) 코드가 필요하다.
>
> Singleton 패턴으로 인해 좋지 않는 구조 설계가 가려질 수 있다.
>
> 멀티 스레드 환경에서는 별도의 조치가 추가적으로 필요하다.
>
> 유닛 테스트 환경에서 제대로 된 테스트가 어렵다. 별도로 모킹하는 과정이 필요하다.

### Code Example

- Practice with Go

  [Github Repository](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-creational-pattern/singleton)

- Spring Container Singleton

  ```java
  public class GenericApplicationContext extends AbstractApplicationContext implements BeanDefinitionRegistry {
  	private final DefaultListableBeanFactory beanFactory;
      // ...
  }
  ```

  ```java
  public class DefaultListableBeanFactory extends AbstractAutowireCapableBeanFactory
  		implements ConfigurableListableBeanFactory, BeanDefinitionRegistry, Serializable {
      @Override
  	public <T> T getBean(Class<T> requiredType, @Nullable Object... args) throws BeansException {
  		Assert.notNull(requiredType, "Required type must not be null");
  		Object resolved = resolveBean(ResolvableType.forRawClass(requiredType), args, false);
  		if (resolved == null) throw new NoSuchBeanDefinitionException(requiredType);
  		return (T) resolved;
  	}
      
      @Nullable
  	private <T> T resolveBean(ResolvableType requiredType, @Nullable Object[] args, boolean nonUniqueAsNull) {
  		NamedBeanHolder<T> namedBean = resolveNamedBean(requiredType, args, nonUniqueAsNull);
  		if (namedBean != null)
  			return namedBean.getBeanInstance();
          
  		BeanFactory parent = getParentBeanFactory();
  		if (parent instanceof DefaultListableBeanFactory)
  			return ((DefaultListableBeanFactory) parent).resolveBean(requiredType, args, nonUniqueAsNull);
  		else if (parent != null) {
  			ObjectProvider<T> parentProvider = parent.getBeanProvider(requiredType);
  			if (args != null)
  				return parentProvider.getObject(args);
  			else
  				return (nonUniqueAsNull ? parentProvider.getIfUnique() : parentProvider.getIfAvailable());
  		}
          
  		return null;
  	}
      // ...
  }
  ```

  스프링은 서버 환경에서 Singleton이 만들어져 사용하는 것을 적극 지원한다. 하지만 자바의 기본적인 Singleton 패턴의 구현 방식은 여러 단점이 있기 때문에, 스프링은 직접 Singleton Container 역할을 하는 Singleton Registry를 만들어 관리한다. **Singleton Registry**는 IoC 방식의 컨테이너를 이용해 기존 Singleton 방식의 단점을 해결한다.

  스프링의 빈들은 `Bean Factory`에 의해 관리되며 기본적으로 빈의 생명주기 Scope는 Singleton이다. 별도의 설정이 없다면 `DefaultListableBeanFactory`를 스프링 부트에서 기본으로 사용한다. 위의 `resolveBean` 메서드 내에서는 `private`, `static`와 같은 접근 제어자를 통한 Singleton 패턴이 없다.

[^1]: [Singleton Origin](https://refactoring.guru/design-patterns/singleton)

