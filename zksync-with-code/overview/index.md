---
title: zkSync 구조 Overview
date: 2022-08-30
pin: false
tags:
- Blockchain
- Ethereum
- Layer 2
---

## ZK Rollup Architecture

대략적으로, zkRollup은 아래와 같이 동작한다.

1. 유저가 트랜잭션을 서명하고, 검증자에게 제출한다. (in L2)

2. 검증자는 수천개의 트랜잭션들을 묶어 하나의 블록에 담고, 이들의 상태 변화 값에 대한 루트 해시값 (Cryptographic Commitment) 을 L1 스마트 컨트랙트에 제출한다.

   더불어 해당 상태의 루트 값이 실제 결과와 동일하다는 SNARK (Cryptographic Proof) 도 같이 제출한다.

3. 증거 뿐만 아니라, 상태값은 L1에서 `calldata` 동작을 통해 받아올 수 있다. 이로 인해 누구나 아무때나 상태 값들을 확인해볼 수 있다.

4. 증거와 상태값은 L1 스마트 컨트랙트에 의해 검증된다. 모든 트랜잭션들의 타당성과 데이터 접근 가능성을 보장해준다.

그리고 zkRollup이 다른 L2에 비해 갖는 이점은 아래와 같다.

- 검증자들은 절대 상태값을 훼손하거나 조작할 수 없다. (Unlike Sidechains)
- 검증자들이 제 역할을 못하더라도, 유저들은 자신의 자금을 L2 스마트 컨트랙트에서 빼낼 수 있다. (Unlike Plasma)
- Rollup한 결과가 조작 되었는 지 검증하기 위해 단일 파티가 존재하지 않아도 된다. (Unlike Optimistic Rollup)

## Transaction Confirmation & Finality



## Decentralization

보통 블록체인에서 탈중앙화를 이야기 할 때는 아래와 같이 정도에 따라 구분된다.

1. Centralized custody (fully trusted): Coinbase
2. Collective custody (trust in the honest majority): sidechains
3. Non-custodial via fraud proofs (trust in the honest minority): optimistic rollups
4. Non-custodial, centrally operated (trustless): zkSync
5. Multi-operator (trustless, weak censorship-resistance): Cosmos
6. Peer-to-peer (trustless, strong censorship-resistance): Ethereum, Bitcoin

현 시점에서의 zkSync는 Level 4에 해당한다.



## Reference

- [zkSync Official Document](https://docs.zksync.io/)

