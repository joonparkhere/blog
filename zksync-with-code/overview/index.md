---
title: zkSync 구조 Overview
date: 2022-09-06
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

유저가 블록체인 트랜잭션을 서명하고 제출했다고 해서, 그 요청이 즉각적으로 처리 및 반영되지 않는다. P2P 네트워크 특성으로 인해 확정짓기까지의 시간이 필요한데, 여기서 등장하는 두 개념이 Confirmation과 Finality다.

Confirmation은 블록체인 노드 중 하나가 제출한 트랜잭션을 블록에 담아서 체인에 연결했음을 의미한다. 이더리움 L1에서 이 의미는 해당 트랜잭션이 담긴 블록을 PoW로 채굴했다는 의미가 될 것이고, Rollup L2에서 이 의미는 단순 노드가 해당 트랜잭션이 포함된 블록을 생성하여 체인에 이었다는 의미가 된다.

Finality는 Confirmation 상태의 트랜잭션이 향후 변경될 가능성이 현격히 적어져, 확정되었다고 얘기할 수 있음을 의미한다. 블록체인은 단일한 체인로 합의되어 이어지는 것이 아니라, 특정 합의 방식을 이용해 여러 개의 체인들 중 하나를 선택한다. 그래서 유저가 제출한 트랜잭션이 확정되기 위해서는 Confirmation 이후에도 해당 블록 뒤에 몇 개의 블록이 이어져있어야 한다. 보통 이더리움 L1에서는 6개 이상의 블록이 이어졌다면 확정되었다고 여긴다. Rollup L2에서의 Finality는 L1으로 롤업한 트랜잭션이 Finality 상태가 되어야 L2 트랜잭션이 Finality 상태가 되었다고 할 수 있다. 따라서 중요한 포인트는 L2에서 어떻게, 그리고 어느 정도의 텀을 두어 L1으로 롤업할 것인가 이다.

비트코인, 이더리움 네트워크의 Finality는 확률에 의존한다. 더 많은 블록이 쌓일수록 Revert될 확률이 줄어들어, Finality 상태 여부를 결정한다. 그러나 ZK Rollup에서 Finality는 기존 L1과는 조금 다르다. L2 블록이 생성되고 해당 State가 L1에 커밋되면, 증명 과정이 시작된다. 블록에 담긴 트랜잭션들에 대한 SNARK 검증 증거가 생성되고, L1 스마트 컨트랙트에 제출된다. 그리고 L1 스마트 컨트랙트가 증거 검증을 마치면, 그리고 L1 블록이 Finality 상태가 되면 L2 블록도 Finality 상태가 된다.

그렇다면 ZK Rollup은 어느정도의 주기마다 L1에 트랜잭션과 State를 커밋할까?
SNARK Proof를 생성하는 과정이 꽤나 시간이 소요되기 때문에, 현 시점에서 일반적인 서버 하드웨어 성능 기준으로1000개의 트랜잭션이 담긴 블록에 대해 약 10분이 소요된다. 이는 zk-SNARK에 대한 연구와 전반적인 하드웨어 성능에 따라 향후 1분 내로 이뤄질 것으로 예상된다.

## Decentralization

보통 블록체인에서 탈중앙화를 이야기 할 때는 아래와 같이 정도에 따라 구분된다.

1. Centralized custody (fully trusted): Coinbase
2. Collective custody (trust in the honest majority): sidechains
3. Non-custodial via fraud proofs (trust in the honest minority): optimistic rollups
4. Non-custodial, centrally operated (trustless): zkSync
5. Multi-operator (trustless, weak censorship-resistance): Cosmos
6. Peer-to-peer (trustless, strong censorship-resistance): Ethereum, Bitcoin

현 시점에서의 zkSync는 Level 4에 해당한다.

## zkSync High-Level Overview

- zkSync Smart Contract
	- Manage Users' Balances and Verify Correctness of Operations
- Prover Application
	- Poll for Available Jobs and Get a Witness from Server
	- Create a Proof for an Executed Block
	- Report a Proof to Server
- Server Application
	- Running Node
	- Monitor On-Chain Operations
	- Accept Transactions
	- Generate L2 Blocks
	- Request Proofs for Executed Blocks
	- Publish Data to Smart Contract

## Universal CRS Setup

ZKP로써, PLONK Proof 시스템을 사용한다.
이는 Common Reference String (CRS) 라고 하는 Trusted Setup을 필요로 한다.
zkSync 개발진인 Matter Labs은 [AZTEC](https://aztec.network/) 프로토콜의 PLONK 시스템에 참여 중이다.
맨처음 한 번만 Setup을 진행해 놓으면, 향후 어느 애플리케이션에도 모두 재사용 가능하다.

이 글 작성일 기준, AZTEC 프로토콜에는 202개의 참여자가 Setup 과정에 기여 중이다.
자세한 목록은 [여기](https://ignition.aztecprotocol.com/)에서 확인할 수 있다.

## Reference

- [zkSync Official Document](https://docs.zksync.io/)
- [zkSync 2.0 Official Document](https://v2-docs.zksync.io)
- [Ethereum Official Document, "Finality"](https://ethereum.org/en/developers/docs/consensus-mechanisms/pow/#finality)

