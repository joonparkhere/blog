---
title: "[CS] 운영체제 개요"
date: 2023-08-28
pin: false
tags:
- Computer Science
- Operating System
---

---

## 전체 목차

1. About OS
   - OS Functions
   - OS Classification
   - OS Structure
   - System Calls, Interrupts, Traps, Exceptions
   - Virtualization

2. Process Management

   - Process (Resource) Concept

   - Process vs Thread

   - Process State Transition

   - Process Control Block

   - Scheduling Queue

   - Interrupts

   - Context Switching

   - Inter Process Communication

3. Process Scheduling

   - Scheduling Criteria and Level

   - Scheduling Policies (Preemptive, Non-Preemptive)

   - Schemes

1. Process Synchronization
   - Process In Multiprogramming Systems
   - Race Condition
   - SW Solution
   - HW Solution
   - Semaphore, Mutex, Spinlock, Event Count, Sequencer
2. Deadlock Handling
   - Deadlock Concept
   - Deadlock Model
   - Deadlock Resolution Methodologies
3. Memory Management
   - Address Binding
   - Static Linking, Dynamic Linking
   - Uni-Programming, Fixed Partition Multiprogramming, Variable Partition Multiprogramming
   - Paging, Segmentation
4. Virtual Memory
   - Organization Concept
   - Segmentation
   - Hybrid Paging & Segmentation
   - Management Concept
   - Cost Model
   - HW & SW Components
   - Algorithms
5. File System
   - Disk System
   - File Concept
   - File Access Methods
   - Directory Structure
   - File System Mounting
   - File Sharing
   - File Protection Model
   - Allocation Methods
   - Free Space Management




## 선정 이유

### 집단지성

선택과 집중을 위해 유명한 [면접 준비를 위해 Gyoogle Repo](https://github.com/gyoogle/tech-interview-for-developer/tree/master)를 참고해 중요 포인트를 잡았다.

1. 운영체제란
2. 프로세스 vs 스레드
3. 프로세스 주소 공간
4. Interrupt
5. System Call
6. PCB와 Context Switching
7. Inter Process Communication
8. CPU 스케줄링
9. Dead Lock
10. Race Condition
11. Semaphore & Mutex
12. Paging & Segmentation
13. 페이지 교체 알고리즘
14. Memory
15. File System



### 서적 참고

![Reference Book Cover](C:/Users/tmdgh/Workspace/records/network/intro/images/reference-book-cover.jpg)

위 책을 참고하여 목차와 내용을 작성했다.

1. 운영체제와 컴퓨터
   1. 운영체제의 역할과 구조
   2. 컴퓨터의 요소
2. 메모리
   1. 메모리 계층
   2. 메모리 관리
3. 프로세스와 스레드
   1. 프로세스와 컴파일 과정
   2. 프로세스의 상태
   3. 프로세스의 메모리 구조
   4. PCB
   5. 멀티프로세싱
   6. 스레드와 멀티스레딩
   7. 공유 자원과 임계 영역
   8. 교착 상태
4. CPU 스케줄링 알고리즘
   1. 비선점형 방식
   2. 선점형 방식



### ChatGPT와 함께

그리고 ChatGPT 이용해서 가이드라인을 뽑아왔다. "운영체제 관련해서 채용 면접 때 나올 수 있는 질문 20가지를 중요한 순서대로 알려줘" 라고 물어봤을 때 작성일 기준 아래와 같은 대답을 받았다.

1. 운영체제란 무엇인가요?

2. 프로세스와 스레드의 차이는 무엇인가요?

3. 프로세스 스케줄링이란 무엇인가요?

   다양한 스케줄링 알고리즘을 설명해주세요.

4. 교착상태란 무엇이며, 그것을 해결하기 위한 방법은 무엇인가요?

5. 가상 메모리란 무엇이며, 페이징과 세그멘테이션의 차이점은 무엇인가요?

6. 페이지 교체 알고리즘에는 어떤 것들이 있으며, 각각의 특징을 설명해주세요.

7. 캐시 메모리의 목적과 동작 방식에 대해 설명해주세요.

8. 파일 시스템이란 무엇이며, 파일 할당 방식과 관련된 내용을 설명해주세요.

9. RAIㅇ 시스템이 무엇인지와 다양한 RAID 레벨에 대해 설명해주세요.

10. 프로세스 동기화와 관련된 문제가 무엇이며, 상호 배제, 동기화, 교착상태를 설명해주세요.

11. 세마포어와 뮤텍스의 차이점은 무엇인가요?

12. 스레드 동기화를 위해 사용되는 도구들과 그 동작 원리를 설명해주세요.

13. 스레드 풀이란 무엇이며, 어떤 상황에서 사용하나요?

14. 인터럽트와 트랩의 차이점은 무엇인가요?

15. 페이지 테이블이란 무엇이며, 가상 주소에서 물리 주소로의 변환 과정을 설명해주세요.

16. 동적 연결과 정적 연결의 차이는 무엇인가요?

17. 메모리 단편화에 대해 설명하고, 내부 단편화와 외부 단편화의 차이를 설명해주세요.

18. 인터페이스와 API의 차이를 설명해주세요.

    운영체제에서 어떻게 사용되나요?

19. 페이지 교체 알고리즘의 최적해와 최악해를 설명하고, 각각 어떤 상황에서 발생하나요?

20. 멀티프로세싱과 멀티스레딩의 장단점을 비교해보세요.
