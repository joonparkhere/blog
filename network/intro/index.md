---
title: "[CS] 네트워크 개요"
date: 2023-08-10
pin: false
tags:
- Computer Science
- Network
---

## 전체 목차

1. OSI Model
   - LAN, WAN
2. Address
   - Sub-net, Super-net
   - Classful, CIDR
   - DHCP, NAT
   - IPv6
   - ARP, ICMP
   - Forwarding, Routing
   - DNS
   - VPN, Proxy
3. TCP & UDP
   - Handshakes
   - Flow Control
   - Error Control
   - Congestion Control
4. HTTP & HTTPS
   - HTTP/1.1
   - HTTP/2
   - TLS/SSL




## 선정 이유

### 집단지성

선택과 집중을 위해 유명한 [면접 준비를 위해 Gyoogle Repo](https://github.com/gyoogle/tech-interview-for-developer/tree/master)를 참고해 중요 포인트를 잡았다.

1. OSI 7계층
2. TCP Flow Control & Congestion Control
3. TCP Handshakes
4. UDP
5. TLS/SSL
6. HTTP & HTTPS
7. Load Balancing
8. Blocking, Non-Blocking & Synchronous, Asynchronous



### 서적 참고

![Reference Book Cover](images/reference-book-cover.jpg)

위 책을 참고하여 목차와 내용을 작성했다.

1. 네트워크 기초
   1. 처리량과 지연 시간
   2. 네트워크 토폴로지와 병목 현상
   3. 네트워크 분류
   4. 네트워크 성능 분석 명령어
   5. 네트워크 프로토콜 표준화
2. TCP/IP 4계층 모델
   1. 계층 구조
   2. PDU
3. 네트워크 기기
   1. 네트워크 기기의 처리 범위
   2. 애플리케이션 계층을 처리하는 기기
   3. 인터넷 계층을 처리하는 기기
   4. 데이터 링크 계층을 처리하는 기기
   5. 물리 계층을 처리하는 기기
4. IP 주소
   1. ARP
   2. Hop-By-Hop 통신
   3. IP 주소 체계
   4. IP 주소를 이용한 위치 정보
5. HTTP
   1. HTTP/1.0
   2. HTTP/1.1
   3. HTTP/2
   4. HTTPS
   5. HTTP/3



### ChatGPT와 함께

그리고 ChatGPT 이용해서 가이드라인을 뽑아왔다. "컴퓨터 네트워크 관련해서 채용 면접 때 나올 수 있는 질문 20가지를 중요한 순서대로 알려줘" 라고 물어봤을 때 작성일 기준 아래와 같은 대답을 받았다.

1. OSI 7계층과 각 계층의 역할에 대해 설명해보세요.
2. TCP와 UDP의 차이점은 무엇인가요? 어떤 상황에서 어떤 프로토콜을 사용해야 하나요?
3. IP 주소와 MAC 주소의 역할과 차이점을 설명해주세요.
4. 네트워크에서 DNS(Domain Name System)이 무엇인지 설명하고 어떻게 작동하는지 알려주세요.
5. HTTP와 HTTPS의 차이점과 HTTPS가 왜 중요한지 설명해보세요.
6. 랜(Local Area Network)과 완(Wide Area Network)의 차이를 설명하세요.
7. 라우터와 스위치의 역할과 차이점을 설명해주세요.
8. 서브넷 마스크란 무엇이며 왜 사용되는지 설명해보세요.
9. ARP(Address Resolution Protocol)이 무엇이며 어떤 목적으로 사용되는지 설명해주세요.
10. NAT(Network Address Translation)의 역할과 장점에 대해 설명해보세요.
11. Firewall이 무엇이며 왜 네트워크 보안에 중요한지 설명해주세요.
12. DDoS 공격이란 무엇이며 어떻게 방어할 수 있는지 알려주세요.
13. VPN(Virtual Private Network)이 무엇인지 설명하고 어떤 상황에서 사용되는지 알려주세요.
14. CIDR(Classless Inter-Domain Routing)이 무엇이며 IP 주소 체계에서 어떻게 활용되는지 설명해보세요.
15. 네트워크 지연과 대역폭의 차이를 이해하고, 어떻게 네트워크 성능을 개선할 수 있는지 설명해보세요.
16. 이더넷(Ethernet) 기술에 대해 설명하고, 각 속도별로 어떤 상황에 사용되는지 알려주세요.
17. IPv4와 IPv6의 차이점과 IPv6의 도입 배경에 대해 설명해보세요.
18. 서버와 클라이언트 모델이 무엇인지 설명하고, 이 모델이 어떤 형태로 활용되는지 알려주세요.
19. Proxy 서버의 역할과 이점에 대해 설명해보세요.
20. 네트워크 문제 해결 절차를 설명하고, 실제 문제를 해결하는 예시를 들어보세요.

