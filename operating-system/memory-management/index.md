---
title: "[CS] 운영체제 메모리 관리"
date: 2023-09-07
pin: false
tags:
- Computer Science
- Operating System
- Memory
---

## Background

### Memory Type

- Processor Register
- Cache Memory
- Main Memory
- Auxiliary Storage

![Storage Hierarchy](images/storage-hierarchy.png)

- Data Transfer Unit

  - Word

    CPU와 Primary Memory 사이의 Data Transfer Unit

  - Block

    Primary Memory와 Secondary Storage 사이의 Data Transfer Unit

- Access Time

  - Register

    0~1 CPU Clock Cycle

  - Memory

    50~200 CPU Clock Cycle

### Address Binding

![Address Binding Diagram](images/address-binding-diagram.png)

- Compile-time Binding

  ![Compile-time Binding Example](images/compile-time-binding-example.png)

  - Absolute Code 생성
  - Recompilation 경우, Memory 시작점을 변경해야 함

- Load-time Binding

  ![Load-time Binding Example](images/load-time-binding-example.png)

  - Compiler는 Relocatable Code를 생성해야 함
  - Memory 시작점 변경을 위해서 Reloading과 Relocation을 거쳐야 함

- Run-time Binding

  ![Run-time Binding Example](images/run-time-binding-example.png)

  - Logical Address에서 Physical Address 맵핑을 위해 MMU, Memory Management Unit 필요

### Dynamic Linking

- Stub Concept 활용
- Shared Library와 같은 Code Share 가능

### Overlay Structure

![Overlay Structure Example](images/overlay-structure-example.png)

- 해당 시점에 필요한 이미지만을 Memory에 상주시키는 구조

### Swapping

![Swapping Diagram](images/swapping-diagram.png)

- 효율적인 CPU Utilization을 위해 Time Quantum > Swap Time 이어야 함
- Swap Device
  - Disk의 일부
  - Process Image를 Contiguous Allocation



## Contiguous Allocation

### Policy for Organization

- Number of Processes in Memory
- Amount of Allocated Memory Space for Each Process
- Memory Partition Method

### Uniprogramming

![Uniprogramming Memory Diagram](images/uniprogramming-memory-diagram.png)

- If Program Size > Memory Size

  Overlay Structure 필요

  이를 위해Compiler, Linker, Loader Support 필요

- Kernel Protection

  ![Uniprogramming Memory Kernel Protection Diagram](images/uniprogarmming-memory-kernel-protection.png)

- Low System Resource Utilization

### FPM, Fixed Partition Multiprogramming

- Concept

  ![FPM Example](images/fpm-example.png)

  - Memory를 Fixed-Size Partition으로 나눔
  - Process와 Partition은 1대1 맵핑
  - Low Memory Management Overhead

- Partition Bound Protection

  ![FPM Protection Method 1](images/fpm-protection-method-1.png)

  ![FPM Protection Method 2](images/fpm-protection-method-2.png)

  - During Context Switching

- Waste of Storage Space
  - Internal Fragmentation
  - External Fragmentation

### VPM, Variable Partition Multiprogramming

- Concept

  ![VPM Example](images/vpm-example.png)

  - 동적으로 Memory Partition State 변화
  - No Internal Fragmentation

- Placement Strategy

  - First-Fit

    - Simple & Low Overhead

  - Best-Fit

    - Long Search Time

    - External Fragmentation 가속화

  - Worst-Fit

  - Next-Fit

    - Circular Search
    - Low Overhead

- Coalescing Hole

  ![VPM Coalescing Hole Example](images/vpm-coalescing-hole-example.png)

  - Merge Adjacent Free Partition

- Storage Compaction

  ![VPM Storage Compaction Example](images/vpm-storage-compaction-exapmle.png)

  - Place All Free Memory Together
  - Consume Long CPU Time



## Discontiguous Allocation

### Paging

- Concept

  ![Paging Example](images/paging-example.png)

  - Page

    Logical Memory into Fixed-Size Blocks

  - Frame

    Physical Memory into Fixed-Size Blocks

  - Logical Address

    $v=(p,d)$

    - Page Number $p$
    - Page Displacement $d$

- Address Mapping

  ![Paging Address Mapping Diagram](images/paging-address-mapping-diagram.png)

  - From Logical Address To Physical Address
  - Use Page Table for Each Process

### Segmentation

- Concept

  ![Segmentation](images/segmentation-example.png)

  - Memory를 Variable-Sized Segment Collection으로 취급

  - Program을 Various Object Collection으로 취급

    Code, Global Variables, Heap, Stack, ...

  - Logical Address

    $v=(s,d)$

    - Segment Number $s$
    - Displacement $d$

- Address Mapping

  ![Segmentation Address Mapping Diagram](images/segmentation-address-mapping-diagram.png)



## Virtual Memory