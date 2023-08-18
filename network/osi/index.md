---
title: "[CS] 네트워크 OSI 7 계층"
date: 2023-08-15
pin: false
tags:
- Computer Science
- Network
- OSI
---

## Intro

### Concept

- Open Systems Interconnection Reference Model

  개방형 시스템 상호 연결 모델의 표준

- 실제 인터넷에서 사용되는 TCP/IP는 OSI 참조 모델을 기반으로 상업적이고 실무적으로 이용될 수 있도록 단순화한 것

### Background

- 초기 여러 정보 통신 업체 장비는 자신의 것들끼리만 연결이 되어 호환성이 떨어짐
- 모든 시스템의 상호 연결에 있어 문제가 없도록 표준을 정한 것이 OSI 7 Layer Model
- 각 Layer는 서로 다른 기능과 역할을 수행함으로써, 네트워크 통신의 효율적인 처리 및 Layer 별 문제 분리 후 해결하기 위한 구조
- 표준과 학습 도구로의 의미로 제작됨



## Operation

![OSI 7 Model](images/osi-model.jpg)

1. OSI 7 계층은 Application, Representation, Session, Transport, Network, Data-Link, Physical 계층으로 나뉨

2. 전송 시 Layer 7에서 Layer 1으로 각각의 계층마다 인식할 수 있는 헤더를 추가함

   출발지에서 데이터가 전송될 때 헤더가 추가되는데, Layer 2에서만 오류 제어를 위해 꼬리 부분에 추가됨

3. Physical Layer에서 0 또는 1의 신호가 되어 전송 매체를 통해 전송됨

4. 수신 시 Layer 1에서 Layer 7으로 헤더를 떼어냄



## Layers

![OSI 7 Layers with Examples](images/osi-model-with-example.png)

### Physical

- 데이터의 물리적 전송을 담당

  Bit를 전기 신호로 변환하거나, 전기 신호를 Bit로 디코딩하는 등 물리적인 신호 처리를 수행

- 네트워크에서 데이터가 실제로 전송되는 물리적인 측면을 다루기 위해 분리된 Layer

  다양한 물리 매체와 전송 방식에 대한 처리를 추상화하여 상위 Layer들에게 일관된 인터페이스 제공

- 전송 이외에 별도의 알고리즘, 오류 제어 기능이 없음

- 데이터 단위는 Bit

- 관련 장비로는 Cable, Repeater, Hub 등이 있음

### Data-Link

- 물리적인 연결을 통해 신뢰성있는 Frame 전송을 보장하고, 오류 검출 및 수정, 흐름 제어 등을 수행

  Point-To-Point 전송

- Physical Layer에서 전송된 데이터의 오류와 손실을 관리하고 안정적인 전송을 보장하기 위해, 그리고 다양한 물리 매체에 대한 추상화를 제공하기 위해 필요한 Layer

- 각 Frame에 주소를 할당하여 안전하게 데이터를 전송하는 기능 존재

- MAC Address를 통해서 통신

- 데이터 단위는 Frame

- 관련 장비로는 Bridge, Switch 등이 있음

### Network

- 데이터의 경로 선택과 라우팅을 관리하며, 데이터 전송 경로를 결정하여 최적화함

- 여러 개의 네트워크가 연결되어 있는 상황에서 데이터의 효율적인 라우팅과 전송 경로 결정을 위해 필요한 Layer

  또한 여러 네트워크 간의 상호 연결성을 제공

- 전송할 IP Address가 필요하므로, 이 계층의 헤더에 주소가 포함됨

- 데이터 단위는 Packet

- 관련 장비로는 Router, L3 Switch 등이 있음

### Transport

- 데이터의 신뢰성과 흐름 제어를 담당하며, 데이터의 분할 및 재조립, 오류 복구 등을 수행
- 종단 간의 통신을 관리하고, 데이터의 정확한 전달 및 순서 유지, 오류 복구 등의 위해 데이터를 Segment로 나누거나 합치는 기능을 제공
- 데이터 전송을 위해 Port 번호를 사용
- 데이터 단위는 Segment

### Session

- 데이터 교환을 위한 세션 관리와 동기화를 수행

  연결 설정, 유지, 종료 등을 관리

- 통신 세션의 설정과 유지를 관리하여 데이터 흐름을 조율하고, 필요한 시간 동안 안정적인 통신을 위해 필요한 Layer

### Representation

- 데이터 형식 변환, 암호화, 압축 등의 데이터 변환을 처리
- 서로 다른 시스템 간의 데이터 형식을 변환하고, 데이터 보안 및 압축 등의 처리를 위해 필요한 Layer
- 주로 세 가지의 기능
  1. 송신자에서 온 데이터를 해석하기 위한 Application Layer 데이터 부호화 및 변화
  2. 수신자가 데이터의 압축을 풀 수 있는 방식으로 데이터를 압축
  3. 데이터의 암호화와 복호화

### Application

- 최종 사용자나 응용 프로그램이 실제로 상호작용하는 Layer로, 다양한 프로그램을 지원하는 서비스와 프로토콜을 제공
- 사용자와 프로그램 간의 효율적인 데이터 교환을 위해 서비스를 제공하며, 상위 Layer들의 작업을 지원
- 예를 들어 전자메일, 인터넷, 동영상 플레이어 등이 있음



## Reference

- ["[네트워크] OSI 7 계층 (OSI 7 LAYER) 기본 개념, 각 계층 설명", cgotijh](https://velog.io/@cgotjh/%EB%84%A4%ED%8A%B8%EC%9B%8C%ED%81%AC-OSI-7-%EA%B3%84%EC%B8%B5-OSI-7-LAYER-%EA%B8%B0%EB%B3%B8-%EA%B0%9C%EB%85%90-%EA%B0%81-%EA%B3%84%EC%B8%B5-%EC%84%A4%EB%AA%85)