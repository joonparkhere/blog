---
title: Blockchain Performance Measurement
date: 2022-09-04
pin: false
tags:
- Blockchain
- Ethereum
- Layer 2
---

블록체인 분야에서 성능은 중요한 지표이다. 관련하여 Layer 1 프로젝트들 뿐만 아니라 Layer 2 솔루션들에 대한 성능 분석 및 평가가 다양하게 존재한다. 그러나 스탠다드하지 않고 그다지 신뢰할 수 없는 결과들이 주를 이룬다. 블록체인 네트워크 특성상, 정확한 평가가 어렵다는 이유가 가장 큰 장애물이다.

## Scalability vs. Performance

- Scalability
  시스템의 성능 개선 가능성 및 능력을 의미
- Performance
  현재 달성할 수 있는 성능을 의미

흔히 블록체인 성능을 이야기할 때, 두 단어를 혼용하여 쓰곤 한다. 이더리움 Layer 1과 Rollup Layer 2 프로젝트들은 비교적 Performance는 떨어지지만, 향후 Scalability를 높여 더 높은 성능을 달성하고자 한다.

## Latency vs. Throughput

- Latency
  각각의 트랜잭션이 얼마나 빠르게 컨펌될 수 있는가
- Throughput
  단위 시간동안 처리될 수 있는 트랜잭션의 양

## Challenges Measuring Latency

- 언제 시간 측정을 시작할 것인가
  트랜잭션을 로컬에서 제출할 때? 아니면 트랜잭션이 Mempool에 전송되었을 때?
- 언제 시간 측정을 마칠 것인가
  트랜잭션이 블록체 담겼을 때? 아니면 Follow-Up 블록이 5~6개 쌓였을 때?

> 조금 더 명확히 하자면, Latency에는 두 종류가 있다.
> - Confirmation Latency
> - Finality Latency

종종 검증자의 관점에서 측정하곤 한다.
- 클라이언트가 트랜잭션을 Broadcast할 때 측정을 시작
- 트랜잭션이 합리적으로 컨펌되었을 때 측정을 종료

하지만 이 경우 다음의 사항을 무시할 수 있다.
- P2P 네트워크 상의 지연 정도를 무시
- Client-Side 지연 정도를 무시
- 트랜잭션의 Priority와 Write Weight를 무시

**Latency is a Distribution, not a sing number**

Latency는 아래와 같은 이유로 상황에 따라 달라질 수 있다.
- Batching
  Batch의 정도에 따라, 일부 트랜잭션들은 배치가 꽉 찰 때까지 기다리는 경우가 있다.
- Variable Congestion
  시스템이 즉각적으로 처리할 수 있는 것보다 더 많은 트랜잭션이 Mempool에 있는 등
- Consensus-Layer Variance
  합의 알고리즘 종류에 따라 발생할 수 있는 딜레이

**Claims about latency should present a distribution (or histogram) of confirmation times, rather than a single number like the mean of median**

## Challenges Measuring Throughput

**Not all transactions are equal**

이더리움의 경우 트랜잭션은 Account의 State를 변경하며, 해당 Job의 정도에 따라 일종의 비용인 Gas를 측정한다. 이 Gas에 따라서 트랜잭션의 Light 혹은 Heavy 정도가 많이 달라진다.

또 하나 주의할 점은 특정 실험으로 측정한 결과 값을 토대로, 해당 네트워크의 전체 성능을 운운하면 안된다는 것이다. 네트워크의 상황은 시시각각 다르기 때문에, 실험을 진행한 순간으로 전체를 얘기할 수 없다.

**In the absence of any clear standard, historic workloads from a popular network like Ethereum suffice**

## Transaction Fees

## Layer 2 Fake Performance

Optimistic Rollup 혹은 ZK Rollup 프로젝트들은 부족한 이더리움의 성능을 개선하고자 등장한 솔루션이다. 이들이 주장하는 것의 핵심은 탈중앙과 보안의 정도를 이더리움 네트워크로부터 상속받고, 성능을 늘리겠다는 것이다. 조금 찾아보면 이더리움보다 훨씬 높은 성능을 제공할 수 있는 것처럼 나온다.

- Ethereum: 20 TPS
- Optimistic Rollups: 2k TPS
- ZK Rollups: 2k TPS

그러나 이는 Layer 2 내에서의 트랜잭션 Confirmation 기준으로 측정할 때의 이야기다. Rollup Layer 2는 결국 주요 상태 변화 값을 Layer 1 (이더리움) 에 주기적으로 제출해야한다. 이렇게 L1으로 제출되어야 L2 트랜잭션들을 신뢰할 수 있게 된다. 즉, 단순 Confirmation이 아닌 Finality는 여전히 좋지 못하다.

## Reference
- [Joseph Bonneau, "Why blockchain performance is hard to measure"](https://a16zcrypto.com/why-blockchain-performance-is-hard-to-measure/)
- [msfew, "(Almost) Everything about Rollup"](https://www.foresightventures.com/wap/focusdetail-31.html)
