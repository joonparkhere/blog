---
title: 영지식 증명 감 잡기
date: 2022-07-20
pin: false
tags:
- ZKP
---

## Do Dive

블록체인 혹은 크립토 분야에서는 익명성을 중요 가치로 여긴다. 이에 따라 Web2 인프라에서의 인증서 기반의 Authentication 등을 적용하기 어려워지고, 신원 노출로 인해 선호하지도 않는다. 그래서 주목받고 있는 암호학 중 하나가 영지식 증명 (Zero-Knowledge Proof) 이다. Tornado Cash처럼 Application 수준의 서비스로, zkRollup처럼 Network Throughput을 위한 프로토콜 수준으로, Soul Bound Token처럼 신원 인증을 위한 수단으로, ... 등등 많은 곳에서 도입하고 있다.

그럼에도 영지식 증명, 암호학은 접근하기 어렵고 이해하기에는 더 어렵다. 영지식 증명하면 빠지지 않는 그림이 있다. 알리바바의 동굴이라는 예제인데, 이 예제의 수준을 넘어 더 깊이 이해하지 않고 (~~못하고~~) 넘어간 경우들이 대부분이었다. 이번에는 조금 더 Dive를 해보려한다.

## Idea

우선 영지식 증명의 주 목표는 **Secret을 갖고 있는 Prover는 이를 그 누구에게도 공개하지 않고, Verifier에게 자신이 Secret을 알고 있다는 것만을 증명**하는 것이다. 비교적 친근한 알리바바의 동굴 예제로 포문을 열어보자.

![](images/ali-baba-cave.png "알리바바의 동굴")

- Prover (증명자) - 분홍색 옷을 입은 사람
- Verifier (검증자) - 초록색 옷을 입은 사람
- Secret (비밀값) - 동굴의 문을 열 수 있는 주문
- Challenge (과정) - Verifier가 Prover에게 나올 방향을 요구하는 과정

동굴에는 A와 B 방향의 두 갈래길이 있고, 가운데는 문으로 막혀있으며 이 문은 주문을 통해서만 열 수 있다. Prover는 동굴의 문을 열 수 있는 주문을 알고 있고, 이를 Verifier에게 알리지 않은 채 자신이 주문을 알고 있다는 것을 증명하려 한다.

1. 먼저 Verifier는 동굴 밖에서 기다리고, Prover는 두 갈래길에서 가고 싶은 곳으로 먼저 들어간다.
2. Verifier는 Prover에게 A (또는 B) 로 나오라고 한다.
3. Prover는 Verifier가 요구한 A (또는 B) 로 나온다.
4. 위의 과정을 반복한다.

Prover가 주문을 모르더라도 Verifier가 우연히 같은 방향으로 나오라고 할 경우, 주문을 알고 있는 것처럼 보일 수 있다. 이런 불상사를 막기위해 과정을 여러 번 반복한다. 우연히 방향이 일치할 확률은 $1/2$ 이다. 과정을 20번 정도 반복하면 $(1/2)^{20}$ 확률로 속일 수 있게 된다. 즉, 영지식 증명은 시행 횟수에 따라 **확률적**으로 Prover를 확신할 수 있게 된다.

## Formal Definition

영지식 증명 (ZKP; Zero-Knowledge Proof) 은 암호학에서 누군가가 상대방에게 어떤 상태가 참이라는 것을 증명할 때, 그 문장의 참 거짓 여부를 제외한 어떤 것도 노출되지 않도록 하는 절차이다. 영지식 증명을 활용한 프로토콜의 가장 큰 특징은 정보를 공개하지 않고 정보의 유효성을 증명할 수 있는 방법이라는 것이다.

영지식 증명에는 어떤 상태의 유효성 (참 혹은 거짓) 을 증명하고자 하는 증명자 (Prover) 와 이를 검증하고자 하는 검증자 (Verifier) 가 참여한다.

- Prover

  자신이 가지고 있는 정보가 무엇인지 공개하지 않고, Verifier에게 "정보를 알고 있다"는 것을 증명하고 싶은 참여자

- Verifier

  Prover가 해당 정보를 가지고 있음을 검증하고 싶은 참여자

- Secret

  Prover가 가지고 있음을 증명하고 싶은 정보, 더불어 모두에게 숨기고자 하는 정보

- Challenge

  Prover가 Secret을 가지고 있는지 확인하기 위해 Verifier가 문제를 내는 과정

- Statment is true

  Prover가 Secret을 가지고 있음을 Verifier가 검증한 상태

ZKP의 수학적 정의는 다음과 같다.
$$
\forall x \in L, z \in \{0, 1\}^{*}, View_{\hat{V}}[P(x) \leftrightarrow \hat{V}(x, z)]=S(x, z)
$$

- $P, V, S$ : Turing Machines (튜링 머신; 여러 가지 기호들을 일정한 규칙에 따라 바꾸는 기계)
- $P(x)$ : Prover, $V(x, z)$: Verifier, $S(x, z)$: Simulator
- $L$ : Language
- $z$ : Verifier의 Challenge Value
- $\leftarrow, \rightarrow$ : Prover와 Verifier의 Challenge 과정 (Interactive Proof System)
- $View$ : Interactive Proof 과정을 관찰하여 기록한 것

정의를 간략히 살펴보면, Verifier의 Challenge Value인 0, 1에 대한 Prover와 Verifier의 Challenge 증명 과정을 다른 튜링 머신에서 똑같이 시뮬레이션 할 수 있어야 한다는 것이다. 앞서 설명한 알리바바 동굴에 대입하면 다음과 같다.

- Prover가 A 혹은 B로 들어간다.
- Verifier는 Prover가 A로 나오게 할 지, B로 나오게 할 지를 Challenge Value인 $z$ 값으로 결정한다.
- Prover는 Verifier의 $z$ 값에 따라 A 혹은 B로 나온다.
- $z$ 값을 바꿔가며 이 과정을 반복하고, 기록한다.
- 그리고 과정들을 튜링 머신에서 똑같이 실행하면, Prover는 사전에 시행한 시뮬레이션과 똑같은 방향으로 나와야 한다.

Prover가 Challenge 과정에서 Verifier에게 Secret을 알고 있음을 확신시킬 수 있는 올바른 답을 계속 제공했다면, Verifier는 확률적으로 Prover가 Secret을 갖고 있음을 확신할 수 있다.

여기서 주의할 점은 Verifier가 $z$ 값을 통해 Prover가 Secret을 알고 있는지 확률적으로 확신할 수 있지만, Secret이 무엇인 지에 대한 그 어떠한 정보도 알아내는 것은 불가능해야 한다는 점이다. 또한 시뮬레이션 할 때도 Prover의 Secret에 대한 정보는 알 수 없어야 한다.

또 한가지 주의할 점은 Verifier를 제외한 다른 사람들은 Prover가 Secret을 알고 있는 지에 대해 확신하는 것도 불가능하도록 만들어야 한다는 것이다.

## Prior Agreement

위에서 언급한 주의해야할 조건들을 가능하게 하는 것이 Prior Agreement (사전 공모) 의 가능성이다.

알리바바 동굴 예제에서 Verifier는 Prover가 주문을 알고 있는지 확인하기 위해 A 혹은 B로 나오라고 요구하는 과정을 20번 반복한다. 그런데 만약 사전에 Verifier가 Prover에게 20번의 순서를 공유했다고 가정해보자. 이렇게 되면 Prover는 주문을 모르더라도 Verifier가 어디로 나오라고 할 지 알기 때문에, 20번의 Challenge를 모두 성공할 수 있다. 실제로 영지식 증명에서는 이런 사전 공모가 가능하다.

Prior Agreement 가능성이 존재하기에, Prover와 Verifier를 제외한 제 3자는 영지식 증명 과정에서 서로 사전에 정보를 공유했는 지, 혹은 공유하지 않았는지 확신할 수 없다. 이에 따라, 제 3자는 Prover와 Verifier가 서로 짜고 Prover가 Secret을 알고 있다고 거짓으로 증명할 수 있다는 의심을 하게 된다. 이어서 Prover가 진짜 Secret을 알고 있을까하는 의심으로 이어지게 된다.

이를 조금 다른 관점에서 보면, Prover가 Secret을 알고 있다는 정보 조차 제 3자에게 노출되지 않았음을 의미한다. 그리고 Verifier 본인만이 Prover와 사전에 공모를 했는지 안했는지 알고 있기 때문에, Prover는 Verifier에게만 Secret을 알고 있다고 증명할 수 있다.

여기서 중요한 조건은 Verifier가 Challenge Value인 $z$ 값을 공개하면 안된다는 점이다. 만약 Verifier가 $z$ 값을 도출하는 과정과 그 값을 제 3자에게 제공한다면, 제 3자는 Prover와 Verifier가 사전에 공모했다고 의심할 수 없게 된다. 이는 Prover의 Challenge 성공에 따라 Secret을 알고 있다는 것을 Verifier 외에도 제 3자가 확신할 수 있다는 것이고, 영지식을 잃는 것을 의미한다.

## Number Theory

ZKP를 조금 더 깊게 살펴보기 전에, 미리 리마인드하고 넘아가면 좋을 만한 부분들을 소개하려 한다. 대체적으로 암호학과 수학 관련 내용들이다.

### Modulo Operation

나눗셈 연산에서 나머지 연산을 하는, 코드에서는 주로 `%`로 표현하는 연산 과정을 말한다. 예를 들어, $46 \ \bmod \ 12 \equiv 10 \ \bmod \ 12$ 이다. 관련 특성들을 살펴보자.

- $a \equiv b \pmod N$ 이면, 정수 $k$ 에 대해 $a+k \equiv b+k \pmod N$
- $a \equiv b \pmod N$ 이면, 정수 $k$ 에 대해 $ka \equiv kb \pmod N$
- $a \equiv b \pmod N$ 이면, 0보다 큰 정수 $k$ 에 대해 $a^k \equiv b^k \pmod N$
- $ka \equiv kb \pmod N$ 이고 $k$ 가 $N$ 과 서로소이면, $a \equiv b \pmod N$

### Modulo Multiplication Inverse

어떤 수를 그 수의 역수와 곱하면 1이 된다. $A$ 의 역수는 $\frac{1}{A} \ (=A^{-1})$ 이다. 모듈러 연산에서 곱셈의 역원, 즉 모듈러 역수는 아래의 규칙으로 정해진다.

- $N$ 과 서로소인 수인 $a$ 에 대하여 $aa^{-1} \equiv 1 \pmod N$

모듈러 역수를 찾는 방법은 $Z_n = \{0, \ 1, \ 2, \ \ldots, \ (N-1)\}$ 에 속하는 수와 $a$ 를 곱한 값과 $\bmod N$ 연산 결과 값이 $1$ 인 경우를 찾는 것이다. 예를 들어 $5 \bmod 8$ 에 대한 역수를 찾는다면, 아래 표처럼 $5$ 가 답이 된다.

|   $Z_8$    |  0   |  1   |  2   |  3   |  4   |  5   |  6   |  7   |
| :--------: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\times 5$ |  0   |  5   |  10  |  15  |  20  |  25  |  30  |  35  |
| $\bmod 8$  |  0   |  5   |  2   |  7   |  4   |  1   |  6   |  3   |

### Fermat's Theorem

페르마의 소정리는 아래와 같다.
$$
\begin{matrix}
a^{p-1} \equiv 1 \pmod p \\
(단, \ p: prime, \ a: coprime \ with \ p)
\end{matrix}
$$

### Euler's Totient Function

오일러의 피 함수는 $n$ 보다 작고, $n$ 과 서로소인 정수의 수를 나타내며, $\varnothing(n)$ 로 표기한다.

소수 $p$ 에 대해서는 $\varnothing(p)=p-1$ 를 만족한다. 그리고 서로 다른 두 소수 $p, \ q$ 에 대해서 아래의 식을 만족한다.
$$
\varnothing(pq)=\varnothing(p) \times \varnothing(q)=(p-1) \times (q-1)
$$

### Euler's Theorem

오일러 정리는 페르마의 소정리를 일반화한 정리 중 하나이다.
$$
\begin{matrix}
a^{\varnothing(n)} \equiv 1 \pmod n \\
(단, \ a와 \ n은 \ 서로소)
\end{matrix}
$$

### An Order

보다 일반화된 정리로, 규칙을 나타낸다.
$$
a^m \equiv 1 \pmod n
$$
가장 작은 양의 정수 $m$ 은 $a \pmod n$ 의 Order (순서, 내지는 규칙) 를 나타내며, 규칙의 주기 또한 알 수 있다. 예를 들어 $7 \pmod {19}$ 는 아래와 같이 연산 결과의 순서 (7, 11, 1) 와 주기 (3) 를 알 수 있다.
$$
\begin{matrix}
7^1 \equiv 7 \pmod {19} \\
7^2 \equiv 49 \equiv 11 \pmod {19} \\
7^3 \equiv 343 \equiv 1 \pmod {19} \\
7^4 \equiv 7 \pmod {19} \\
7^5 \equiv 11 \pmod {19} \\
7^6 \equiv 1 \pmod {19} \\
\ldots
\end{matrix}
$$

### Primitive Root

Order of $a \pmod n$ 와 $\varnothing(n)$ 이 같은 경우, $a$ 를 $n$ 의 원시근이라고 부른다. 예를 들어 소수 19의 원시근은 2, 3, 10, 13, 14, 15 이다.

![](images/modulo-19.png "Modulo 19")

### Discrete Logarithm

이산 로그는 정수 $b$ 에 대하여 $b \equiv r \pmod p$ 를 만족하는 $r$ (단, $0 \le r \le p-1$ ) 이 존재할 때, $b \equiv a^i \pmod p$ 를 만족하는 $i$ (단, $0 \le i \le p-1$) 를 찾는 것을 의미한다.

예를 들어 $p=9, \ a=2$ 인 경우를 생각해보자. $\varnothing(p)=6 \ (\{1, \ 2, \ 4, \ 5, \ 7, \ 8\})$ 이다. 이에 따라 $i=6$ 이라고 주어질 때, $b$ 를 찾는 것은 $2^6 \equiv 1 \pmod 9$ 으로, 매우 쉽다. 하지만 반대로 $b=1$ 이 주어지고 $i$ 를 찾아야 할 경우, $2^i \equiv 1 \pmod 9$ 를 계산하는 것은 어렵다. 특히, 암호학에서는 매우 큰 소수 $p$ 를 이용하기 때문에 상당히 버거운 작업이 된다.

이러한 Trapdoor 성질로 인해 여러 암호 알고리즘 (Diffie-Hellman Key Sharing, Digital Signature Algorithm 등) 에서 이산 로그가 활용된다.

## ZKP Under the Hood

Discrete Logarithm (이산 로그) 를 이용하여 구현된 ZKP 과정을 알아보려 한다. 이산 로그의 역함수인 이산 거듭제곱 ($a^{x}=b$) 은 효율적으로 계산할 수 있지만, 이산 로그 계산 ($x=log_{a}b$) 은 어려운 것으로 알려진 군이 존재하는데, ZKP도 이를 이용해 응용된 암호학 중 하나다.

- $x$ : Secret
- $y$ : Given Value by Prover
- $p$ : Large Prime
- $g$ : Generator

위의 기호를 참고하여, 아래의 과정을 통해 Verifier에게 Secret의 정보를 알리지 않고, Secret을 가지고 있음을 증명할 수 있다.

0. Prover는 Secret인 $x$ 값을 알고 있다.

1. Prover와 Verifier는 Prime $p$ 와 Generator $g$ 에 대해 합의한다.

2. Prover는 $y=g^{x}\ mod\ p$ 를 계산하여 $y$ 값을 Verifier에게 전달한다.

3. 아래의 과정을 계속 반복하여 확률적으로 확신할 수 있게 된다.

   1. Prover는 $r\in U[0,\ p-2]$ 에 해당하는 랜덤한 $r$ 값을 골라서, $C=g^{r} \ mod \ p$ 를 계산하여 $C$ 값을 Verifier에게 전달한다.

   2. Verifier는 Prover에게 $(x+r) \ mod \ (p-1)$ 또는 $r$ 값을 요청하여 전달 받는다.

      - $(x+r) \ mod \ (p-1)$ 경우

        Verifier는 $C$ 값과 $y$ 값을 알고 있으므로, 수식 $(C \cdot y) \ mod \ p \equiv g^{(x+r) \ mod \ (p-1)} \ mod \ p$ 을 통해 Prover가 $x$ 값을 알고 있는지 검증한다.

      - $r$  경우

        Verifier는 $C$ 값을 알고 있으므로, 수식 $C \equiv g^{r} \ mod \ p$ 을 통해 Prover가 $x$ 값을 알고 있는지 검증한다.

      여기서 $(x+r) \ mod \ (p-1)$ 값은 $x \ mod \ (p-1)$ 값을 암호화하여 Prover가 가진 $x$ 값을 유추하기 굉장히 어렵게 만든다. $r$ 값이 $[0, \ p-2]$ 범위에서 실제로 랜덤하게 선택되었다면, $x$ 값에 대한 어떠한 정보도 노출하지 않게 된다.

## Reference

- [Wikipedia ZKP](https://en.wikipedia.org/wiki/Zero-knowledge_proof)
- [Wikipedia Turing machine](https://en.wikipedia.org/wiki/Turing_machine)
- [WikiHash Discrete Logarithms](http://wiki.hash.kr/index.php/%EC%9D%B4%EC%82%B0%EB%A1%9C%EA%B7%B8)
- [Hyun Jeong, "영지식 증명 이해하기"](https://hyun-jeong.medium.com/h-3c3d45861ced)

