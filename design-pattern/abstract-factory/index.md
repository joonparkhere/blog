---
title: "[디자인 패턴 GURU] Abstract Factory 패턴"
date: 2021-12-30
pin: false
tags:
- Software Enginerring
- Design Pattern
- Go
- Java
---

## Abstract Factory

### Intent

구현 클래스에 커플링되지 않고 비슷한 특성을 갖는 객체들을 생성하는 방법이다.

### Problem

![예시[^1]](images/abstract-factory-problem-en.png)

예를 들어, `Chair`, `Sofa`, `CoffeeTable` 세 개의 제품이 있고, 각 제품마다 `Modern`, `Victorian`, `ArtDeco` 세 개의 디자인을 갖는다면, `Chair`의 Factory Method에 세 디자인에 대한 구현 로직이 있어야 한다. 여기서 디자인이 추가된다면 점점 코드가 복잡해지고, 다른 가구와의 코드 중복이 발생한다. 더불어 구현체의 코드 수정이 불가피하다.

### Solution Structure

![Chair 클래스 예제 코드[^1]](images/abstract-factory-solution1.png)

Abstract Factory 패턴은 먼저 각 특성을 갖는 구현체의 역할을 명시할 인터페이스를 선언하고, 각 구현체들을 만든다. 예를 들어, 모든 `Chair` 구현체는 `Chair` 인터페이스를 구현해야 한다.

![Abstract Factory 예제 코드[^1]](images/abstract-factory-solution2.png)

그리고 생성 메서드를 포함한 Abstract Factory를 선언한다. 구현체와의 디커플링을 위해 생성 메서드는 추상 객체를 반환해야 한다. 이제 각 특성을 갖는 객체를 생성하는 Factory 클래스를, `AbstractFactory` 인터페이스를 구현하여 만든다. 예를 들어, `ModernFurnitureFactory`는 `ModernChair`, `ModernSofa`, `ModernCoffeeTable` 객체 생성을 담당한다.

![Abstract Factory 구조[^1]](images/abstract-factory-structure.png)

0. 우선 각각의 구현체가 갖는 특성들을 행렬 형태로 정리해 본다.
1. `Abstract Product`들은 공통된 역할을 명시하는 인터페이스다.
2. `Concrete Product`들은 추상 객체의 구현체이며 각 특성에 의해 그룹핑된 클래스이다.
3. `Abstract Factory`는 추상 객체들을 생성하는 메서드들을 갖는 인터페이스다.
4. `Concrete Factory`들은 Factory의 구현체이며, 각 특성에 맞는 구현체를 생성한다.

### Applicability

- 다양한 특성을 갖는 여러 객체들이 있고, 이후 특성을 추가할 때 구현체의 코드를 건들지 않기를 원하거나 사전에 어느 정도의 확장성을 고려해야 할 지 감이 오지 않을 때 사용

> Factory Method 방법과 마찬가지로, 점점 더 많은 특성을 갖는 구현체가 늘어날 수록 코드가 복잡해질 수 있다.

### Code Example

- Practice with Go

  [Github Repository](https://github.com/joonparkhere/records/tree/main/design-pattern/project/hello-creational-pattern/abstract-factory)

- Spring framework

  ```java
  public abstract class AbstractFactoryBean<T>
  		implements FactoryBean<T>, BeanClassLoaderAware, BeanFactoryAware, InitializingBean, DisposableBean {
      protected abstract T createInstance() throws Exception;
      // ...
  }
  ```
  
  ```java
  public class ListFactoryBean extends AbstractFactoryBean<List<Object>> {
      @Override
  	protected List<Object> createInstance() {
  		if (this.sourceList == null) throw new IllegalArgumentException("'sourceList' is required");
          
  		List<Object> result = null;
  		if (this.targetListClass != null) 
              result = BeanUtils.instantiateClass(this.targetListClass);
  		else 
  			result = new ArrayList<>(this.sourceList.size());
          
  		Class<?> valueType = null;
  		if (this.targetListClass != null)
  			valueType = ResolvableType.forClass(this.targetListClass).asCollection().resolveGeneric();
  		if (valueType != null) {
  			TypeConverter converter = getBeanTypeConverter();
  			for (Object elem : this.sourceList)
  				result.add(converter.convertIfNecessary(elem, valueType));
  		}
  		else {
  			result.addAll(this.sourceList);
  		}
          
  		return result;
  	}
      // ...
  }
  ```
  
  ```java
  public class MapFactoryBean extends AbstractFactoryBean<Map<Object, Object>> {
      @Override
  	protected Map<Object, Object> createInstance() {
  		if (this.sourceMap == null) throw new IllegalArgumentException("'sourceMap' is required");
  
  		Map<Object, Object> result = null;
  		if (this.targetMapClass != null)
  			result = BeanUtils.instantiateClass(this.targetMapClass);
  		else
  			result = CollectionUtils.newLinkedHashMap(this.sourceMap.size());
          
  		Class<?> keyType = null;
  		Class<?> valueType = null;
  		if (this.targetMapClass != null) {
  			ResolvableType mapType = ResolvableType.forClass(this.targetMapClass).asMap();
  			keyType = mapType.resolveGeneric(0);
  			valueType = mapType.resolveGeneric(1);
  		}
  		if (keyType != null || valueType != null) {
  			TypeConverter converter = getBeanTypeConverter();
  			for (Map.Entry<?, ?> entry : this.sourceMap.entrySet()) {
  				Object convertedKey = converter.convertIfNecessary(entry.getKey(), keyType);
  				Object convertedValue = converter.convertIfNecessary(entry.getValue(), valueType);
  				result.put(convertedKey, convertedValue);
  			}
  		}
  		else {
  			result.putAll(this.sourceMap);
  		}
          
  		return result;
  	}
      // ...
  }
  ```
  
  ```java
  public class SetFactoryBean extends AbstractFactoryBean<Set<Object>> {
      @Override
  	protected Set<Object> createInstance() {
  		if (this.sourceSet == null) throw new IllegalArgumentException("'sourceSet' is required");
          
  		Set<Object> result = null;
  		if (this.targetSetClass != null) 
              result = BeanUtils.instantiateClass(this.targetSetClass);
  		else 
              result = new LinkedHashSet<>(this.sourceSet.size())
  
  		Class<?> valueType = null;
  		if (this.targetSetClass != null)
  			valueType = ResolvableType.forClass(this.targetSetClass).asCollection().resolveGeneric();
  		if (valueType != null) {
  			TypeConverter converter = getBeanTypeConverter();
  			for (Object elem : this.sourceSet)
  				result.add(converter.convertIfNecessary(elem, valueType));
  		}
  		else {
  			result.addAll(this.sourceSet);
  		}
          
  		return result;
  	}
      // ...
  }
  ```

[^1]: [Abstarct Factory Origin](https://refactoring.guru/design-patterns/abstract-factory)
