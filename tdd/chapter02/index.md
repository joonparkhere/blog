---
title: "TDD: By Example 2부"
date: 2021-04-12
pin: false
tags:
- TDD
- Python
---

# TDD 2부: xUnit 예시

이번 장에서는 테스트 주도 개발을 위한 도구의 구현에 대해 얘기해보려 한다. 당연히 테스트 주도로.



## xUnit으로 가는 첫걸음

우선 테스트 케이스를 만들고 테스트 메서드를 실행할 수 있어야 한다. 이때 테스트 케이스를 작성하기 위해 사용할 프레임워크를 테스트하기 위한 테스트 케이스를 작성해야 하는... 문제가 있다. 아직 프레임워크가 없기 때문에 첫 번째 작은 단계는 수동으로 검증을 할 것이다.

> - [ ] 테스트 메서드 호출하기
> - [ ] 먼저 setUp 호출하기
> - [ ] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [ ] 수집된 결과를 출력하기

첫 번째 원시 테스트에는 테스트 메서드가 호출되면 true, 그렇지 않으면 false를 반환할 작은 프로그램이 필요하다. 테스트 메서드 안에 플래그를 설정하는 테스트 케이스가 있다면, 그 테스트 케이스가 실행된 이후에 플래그를 인쇄해 볼 수 있고, 그러면 그게 맞는지 아닌지 확인해 볼 수 있다.

메서드가 실행되었는지를 알려주는 테스트 케이스이므로 클래스 이름을 WasRun으로 하고, 플래그 역시 wasRun으로 하자.

```python
test = WasRun("testMethod")
print(test.wasRun)
test.testMethod()
print(test.wasRun)
```

```python
class WasRun:
    def __init__(self, name):
        self.wasRun = None

    def testMethod(self):
        self.wasRun = 1
```

다음으로 필요한 것은 테스트 메서드를 직접 호출하는 대신 진짜 인터페이스인 run() 메서드를 사용하는 것이다.

```python
test = WasRun("testMethod")
print(test.wasRun)
test.run()
print(test.wasRun)
```

```python
class WasRun:
    # ...
    def run(self):
        self.testMethod()
```

이번에는 testMethod()를 동적으로 호출하도록 바꿔보자. 파이썬은 클래스의 이름이나 메서드의 이름을 함수처럼 다룰 수 있으므로 이를 이용해서 객체를 얻어낼 수 있다.

```python
class WasRun:
    def __init__(self, name):
        self.wasRun = None
        self.name = name

    def testMethod(self):
        self.wasRun = 1

    def run(self):
        method = getattr(self, self.name)
        method()
```

이제 WasRun 클래스는 독립된 두 가지 일을 수행한다. 하나는 메서드가 호출되었는지 그렇지 않은지를 기억하는 일이고, 다른 하나는 메서드를 동적으로 호출하는 일이다. 이제 본격적으로 테스트 케이스를 만들어보자.

```python
class TestCase:
    def __init__(self, name):
        self.name = name

    def run(self):
        method = getattr(self, self.name)
        method()
```

```python
class WasRun(TestCase):
    def __init__(self, name):
        self.wasRun = None
        TestCase.__init__(self, name)

    def testMethod(self):
        self.wasRun = 1
```

매번 None이랑 1이 나오는지 확인하는 것도 자동화하자.

```python
class TestCaseTest(TestCase):
    def testRunning(self):
        test = WasRun("testMethod")
        assert not test.wasRun
        test.run()
        assert test.wasRun
```

```python
TestCaseTest("testRunning").run()
```

> - [x] 테스트 메서드 호출하기
> - [ ] 먼저 setUp 호출하기
> - [ ] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [ ] 수집된 결과를 출력하기



## 테이블 차리기

테스트를 작성하다 보면 공통된 패턴을 발견하게 된다.

1. Arrange (준비) - 객체를 생성한다.
2. Act (행동) - 어떤 자극을 준다.
3. Assert (확인) - 결과를 검사한다.

두 번째와 세 번째 단계인 행동과 확인 단계는 항상 다르지만, 처음 단계인 준비 단계는 여러 테스트에 걸쳐 동일한 경우가 종종 있다. 이 부분을 테스트로 표현해보자. TestCaseTest에서는 setUp이 되었는지 확인하는 코드가, WasRun에서는 실제로 wasSetUp을 1로 할당하는 코드가, TestCase에서는 setUp을 호출하는 코드가 필요할 것이다.

```python
class TestCaseTest(TestCase):
    # ...
    def testSetUp(self):
        test = WasRun("testMethod")
        test.run()
        assert test.wasSetUp
```

```python
class WasRun(TestCase):
    # ...
    def __init__(self, name):
        TestCase.__init__(self, name)

    def setUp(self):
        self.wasRun = None
        self.wasSetUp = 1
```

```python
class TestCase:
    # ...
    def setUp(self):
        pass

    def run(self):
        self.setUp()
        method = getattr(self, self.name)
        method()
```

이렇게 바꾸면, 테스트를 실행하기 전에 플래그를 검사하지 않도록 testRunning을 단순화해야 한다. 더불어 테스트 자체도 단순화할 수 있다. testSetUp과 testRunning 두 경우 모두 WasRun의 인스턴스를 생성하는데, 그 대신 WasRun을 setUp에서 생성하고 테스트 메서드에서 그걸 사용하게 할 수 있다.

```python
class TestCaseTest(TestCase):
    def setUp(self):
        self.test = WasRun("testMethod")
        
    def testSetUp(self):
        self.test.run()
        assert self.test.wasSetUp

    def testRunning(self):
        self.test.run()
        assert self.test.wasRun
```

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [ ] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [ ] 수집된 결과를 출력하기



## 뒷정리 하기

언제까지 계속 플래그를 통해 실행 여부를 확인할 수는 없으므로 로그를 간단히 남기는 방식으로 전략을 바꾸자. 항상 로그의 끝부분에만 기록을 추가하면 메서드 호출 순서를 알 수 있을 것이다.

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [ ] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [ ] 수집된 결과를 출력하기
> - [ ] WasRun에 로그 문자열 남기기

```python
class WasRun(TestCase):
    # ...
    def setUp(self):
        self.log = "setUp "

    def testMethod(self):
        self.log = self.log + "testMethod "
```

```python
class TestCaseTest(TestCase):
    # ...
    def testSetUp(self):
        self.test.run()
        assert "setUp testMethod" == self.test.log
```

이제 testSetUp 테스트는 두 개의 테스트가 할 일을 모두 수행하므로, testRunning을 지우고 testSetUp의 이름을 알맞게 바꿔주자.

```python
class TestCaseTest(TestCase):
    def setUp(self):
        pass

    def testTemplateMethod(self):
        test = WasRun("testMethod")
        test.run()
        assert "setUp testMethod" == test.log
```

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [ ] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [ ] 수집된 결과를 출력하기
> - [x] WasRun에 로그 문자열 남기기

이제 tearDown 구현을 위한 테스트를 작성한 후 구현해보자.

```python
class TestCaseTest(TestCase):
    # ...
    def testTemplateMethod(self):
        test = WasRun("testMethod")
        test.run()
        assert "setUp testMethod tearDown " == test.log
```

```python
class TestCase:
    # ...
    def run(self):
        self.setUp()
        method = getattr(self, self.name)
        method()
        self.tearDown()

    def tearDown(self):
        pass
```

```python
class WasRun(TestCase):
    # ...
    def tearDown(self):
        self.log = self.log + "tearDown "
```

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [x] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [ ] 수집된 결과를 출력하기
> - [x] WasRun에 로그 문자열 남기기



## 셈하기

tearDown()은 테스트 메서드에서 예외가 발생하건 말건 호출되도록 보장되어야 한다. 하지만 테스트가 작동하도록 하려면 예외를 잡아야 한다. 즉, 여러 테스트를 실행했을 때 "5개 테스트가 실행됨. 2개 실패."과 같은 결과를 보길 원한다. 이를 위해 TestCase.run()이 테스트 하나의 실행 결과를 기록하는 TestResult 객체를 반환하게 만들자.

```python
class TestCaseTest(TestCase):
    # ...
    def testResult(self):
        test = WasRun("testMethod")
        result = test.run()
        assert "1 run, 0 failed" == result.summary()
```

우선 가짜 구현으로 테스트를 통과하게끔 작성하자.

```python
class TestResult:
    def summary(self):
        return "1 run, 0 failed"
```

```python
class TestCase:
    # ...
    def run(self):
        self.setUp()
        method = getattr(self, self.name)
        method()
        self.tearDown()
        return TestResult
```

이제 summary() 구현을 조금씩 실체화하자.

```python
class TestResult:
    def __init__(self):
        self.runCount = 0
        
    def testStarted(self):
        self.runCount = self.runCount + 1
        
    def summary(self):
        return "%d run, 0 failed" % self.runCount
```

```python
class TestCase:
    # ...
    def run(self):
        result = TestResult()
        result.testStarted()
        self.setUp()
        method = getattr(self, self.name)
        method()
        self.tearDown()
        return result
```

실패하는 테스트의 수도 비슷한 흐름으로 작성하자. 먼저 테스트 케이스부터.

```python
class TestCaseTest(TestCase):
    # ...
    def testFailedResult(self):
        test = WasRun("testBrokenMethod")
        result = test.run()
        assert "1 run, 1 failed" == result.summary()
```

```python
class WasRun(TestCase):
    # ...
    def testBrokenMethod(self):
        raise Exception
```

지금은 WasRun.testBrokenMethod에서 던진 예외를 처리하지 않지만, 여기까지만 구현하고 할일 목록을 체크해보자.

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [x] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [x] 수집된 결과를 출력하기
> - [x] WasRun에 로그 문자열 남기기
> - [ ] 실패한 테스트 보고하기



## 실패 처리하기

실패한 테스트를 발견하면 좀 더 세밀한 단위의 테스트를 작성해서 올바른 결과를 출력하는 걸 확인해보자.

```python
class TestCaseTest(TestCase):
    # ...
    def testRailedResultFormatting(self):
        result = TestResult()
        result.testStarted()
        result.testFailed()
        assert "1 run, 1 failed" == result.summary()
```

```python
class TestResult:
    def __init__(self):
        self.runCount = 0
        self.failureCount = 0

    def testStarted(self):
        self.runCount = self.runCount + 1

    def testFailed(self):
        self.failureCount = self.failureCount + 1

    def summary(self):
        return "%d run, %d failed" % (self.runCount, self.failureCount)
```

```python
class TestCase:
    # ...
    def run(self):
        result = TestResult()
        result.testStarted()
        self.setUp()
        try:
            method = getattr(self, self.name)
            method()
        except:
            result.testFailed()
        self.tearDown()
        return result
```

물론 위 run() 메서드에는 setUp()에서 발생한 예외에 대한 처리를 하지 못한다는 문제가 있다. 이는 추후 연습 과제로 남겨두자.

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [x] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [ ] 여러 개의 테스트 실행하기
> - [x] 수집된 결과를 출력하기
> - [x] WasRun에 로그 문자열 남기기
> - [x] 실패한 테스트 보고하기



## 얼마나 달콤한지

이번에는 여러 테스트가 같이 실행될 수 있도록 만드는 일을 해보자. 현재 코드 파일의 끝 부분에는 모든 테스트들을 호출하는 코드가 있는데, 놓친 디자인 요소를 찾기 위해 일부러 만드는 중복이 아니라면 제거하는 편이 좋다.

```python
TestCaseTest("testTemplateMethod").run().summary()
TestCaseTest("testResult").run().summary()
TestCaseTest("testFailedResult").run().summary()
TestCaseTest("testRailedResultFormatting").run().summary()
```

이들을 모아서 한 번에 실행할 수 있는 TestSuite를 만들고, 거기에 테스트를 몇 개 넣은 다음 모두 실행하여 결과를 얻어내 보자.

```python
class TestCaseTest(TestCase):
    # ...
    def testSuite(self):
        suite = TestSuite()
        suite.add(WasRun("testMethod"))
        suite.add(WasRun("testBrokenMethod"))
        result = suite.run()
        assert "2 run, 1 failed" == result.summary()
```

```python
class TestSuite:
    def __init__(self):
        self.tests = []

    def add(self, test):
        self.tests.append(test)

    def run(self):
        result = TestResult()
        for test in self.tests:
            test.run(result)
        return result
```

test.run()에 TestResult를 매개 변수로 넘겨주었기 때문에 호출하는 곳에서 TestResult를 할당하자.

```python
class TestCaseTest(TestCase):
    # ...
    def testSuite(self):
        suite = TestSuite()
        suite.add(WasRun("testMethod"))
        suite.add(WasRun("testBrokenMethod"))
        result = TestResult()
        suite.run(result)
        assert "2 run, 1 failed" == result.summary()
```

```python
class TestSuite:
    # ...
    def run(self, result):
        for test in self.tests:
            test.run(result)
        return result
```

```python
class TestCase:
    # ...
    def run(self, result):
        result.testStarted()
        self.setUp()
        try:
            method = getattr(self, self.name)
            method()
        except:
            result.testFailed()
        self.tearDown()
```

이제 파일 뒷부분에 있는 테스트 호출 코드를 정리할 수 있다.

```python
suite = TestSuite()
suite.add(TestCaseTest("testTemplateMethod"))
suite.add(TestCaseTest("testResult"))
suite.add(TestCaseTest("testFailedResult"))
suite.add(TestCaseTest("testRailedResultFormatting"))
suite.add(TestCaseTest("testSuite"))
result = TestResult()
suite.run(result)
print(result.summary())
```

그리고 이전에 작성했던 테스트 코드들은 run() 메서드에 TestResult를 넘겨주지 않았으므로 이를 알맞게 수정하자.

```python
class TestCaseTest(TestCase):
    def setUp(self):
        self.result = TestResult()

    def testTemplateMethod(self):
        test = WasRun("testMethod")
        test.run(self.result)
        assert "setUp testMethod tearDown " == test.log

    def testResult(self):
        test = WasRun("testMethod")
        test.run(self.result)
        assert "1 run, 0 failed" == result.summary()

    def testFailedResult(self):
        test = WasRun("testBrokenMethod")
        test.run(self.result)
        assert "1 run, 1 failed" == result.summary()

    def testRailedResultFormatting(self):
        self.result.testStarted()
        self.result.testFailed()
        assert "1 run, 1 failed" == result.summary()

    def testSuite(self):
        suite = TestSuite()
        suite.add(WasRun("testMethod"))
        suite.add(WasRun("testBrokenMethod"))
        suite.run(self.result)
        assert "2 run, 1 failed" == result.summary()
```

> - [x] 테스트 메서드 호출하기
> - [x] 먼저 setUp 호출하기
> - [x] 나중에 tearDown 호출하기
> - [ ] 테스트 메서드가 실패하더라도 tearDown 호출하기
> - [x] 여러 개의 테스트 실행하기
> - [x] 수집된 결과를 출력하기
> - [x] WasRun에 로그 문자열 남기기
> - [x] 실패한 테스트 보고하기
> - [ ] TestCase 클래스에서 TestSuite 생성하기

