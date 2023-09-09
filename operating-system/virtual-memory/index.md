---
title: "[CS] 운영체제 가상 메모리"
date: 2023-09-08
pin: false
tags:
- Computer Science
- Operating System
- Memory
---

## About Virtual Memory

### Concept

- Execution Process를 온전히 Memory에 Load하지 않고 일부만 하는 것

  일부 Partition은 Execution 중 필요한 것만 Load

### Benefit

- Easier Programming

- Higher Multiprogramming Degree

  Increase CPU Utilization and Throughput

- Less I/O for Loading & Swapping

### Drawback

- Overhead of Address Mapping
- Overhead of Page Fault Handling



## Address Mapping

### Contiguous Allocation

![Contiguous Allocation Address Mapping Example](images/address-mapping-contiguous-allocation.png)

- Relative Address
- Relocation

### Noncontiguous Allocation

![Noncontiguous Allocation Address Mapping Example](images/address-mapping-noncontiguous-allocation.png)

- Transformation At Run-time from Virtual Address to Read Address

### Block Mapping

- Partition Program into Blocks

- Virtual Address $v=(b, d)$

  ![Block Mapping Virtual Address Example](images/block-mapping-virtual-address.png)

  - Block Number $b$
  - Displacement in a Block $d$

- Need BMT, Block Map Table

  ![Block Mapping BMT Example](images/block-mapping-bmt.png)

  - One BMT for Each Process

- Procedure Example

  ![Block Mapping Procedure Example](images/block-mapping-procedure.png)

  1. Access BMT of Process

  2. Find Entry for block $b$ in BMT

  3. Check Residence Bit of Entry

     1. Residence Bit == 0

        - Page Fault

        - Context Switching

          Load Block from Swap Device

     2. Residence Bit == 1

        - Extract Read Address from Entry

  4. Compose Real Address $r$

  5. Access Memory with Read Address $r$



## Demand Paging

### Concept

![Paging System Diagram](images/paging-system-diagram.png)

- Partition Program into Same Size Blocks, Pages
- Pre-Partitioned Memory, Page Frames
- During Execution, Load Demanded Pages
- No Logical Partition
- Complex for Sharing & Protection

### PMT, Page Map Table

![Demand Paging PMT Example](images/demand-paging-pmt.png)

- PMT for Each Process

### Direct Mapping

- Double Memory Access

  Memory Access를 위해 PMT에 Access

![Demand Paging Direct Mapping Procedure 1Example](images/demand-paging-direct-mapping.png)

1. Access PTBR, Page Table Base Register of Process

2. Find Entry for Page $p$ in PMT

3. Check Residence Bit of Entry

   1. Residence Bit == 0

      - Page Fault

      - Context Switching

        Load Page into Memory from Swap Device

      - Update PMT

   2. Residence Bit == 1

      - Extract Page Frame Number $p'$ from Entry

4. Compose Read Address $r$ with Page Frame Number $p'$ and Displacement $d$

5. Access Memory with Read Address $r$

### Associative Mapping

- TLB, Translation Lock-aside Buffer
- Low Overhead, but Expensive HW

![Demand Paging Associative Mapping Procedure Example](images/demand-paging-associative-mapping.png)

### Hybrid Direct / Associative Mapping

- Small TLB
  - Full PMT in Memory Kernel Space
  - Subset of PMT in TLB

![Demand Paging Hybrid Mapping Procedure Example](images/demand-paging-hybrid-mapping.png)

### Issue

- Sharable Data Page

  ![Demand Paging Data Page Sharing](images/demand-paging-sharable-data-page.png)

- Sharable Procedure Page

  ![Demand Paging Procedure Page Sharing Problem](images/demand-paging-procedure-page-sharing-problem.png)

  ![Demand Paging Procedure Page Sharing Solution](images/demand-paging-procedure-page-sharing-solution.png)

- Page Fault Handling

  Machine Instruction은 Atomic하기 때문에 중간에 Block될 경우 다음 Instruction을 실행하게 됨

  그러나 Page Fault의 경우 로직에 실질적인 문제가 아니므로, 이후 Instruction을 재실행시켜야 함



## Segmentation

### Concept

![Segmentation System Diagram](images/segmentation-system-diagram.png)

- Partition Program into Logical Blocks with Different Size
- Easy Segment Sharing & Protection
- Larger Overhead

### SMT, Segment Map Table

![Segmentation SMT Example](images/segmentation-smt.png)

- Segment Length

  Block Size가 다를 수 있기 때문에 필요

- Protection Bits

  Logical Partition이기 때문에 필요

### Direct Mapping

![Segmentation Direct Mapping Procedure Example](images/segmentation-direct-mapping.png)

1. Access SMT Base Address of Process
2. Find Entry for Segment $s$ in SMT
3. Check Followings
   1. Residence Bit == 0
      - Segment Fault
      - Load Segment from Swap Device
      - Update SMT
   2. $d>l_s$
      - Segment Overflow Exception
   3. Violate Protection Policy
      - Segment Protection Exception
4. Extract Real Address $r$ of Segment $a_s$ and Displacement $d$
5. Access Memory with Read Address $r$



## Hybrid Paging / Segmentation

### Concept

![Hybrid System Program Partition Diagram](images/hybrid-system-program-partition-diagram.png)

- Partition Program into Logical Segments
- Partition Segment into Pages

### SMT and PMT

- SMT

  ![Hybrid Mechanism SMT Example](images/hybrid-smt.png)

  - No Residence Bit Field

- PMT

  ![Hybrid Mechanism PMT Example](images/hybrid-pmt.png)

- Tables

  ![Hybrid SMT and PMT Example](images/hybrid-smt-pmt.png)

  - One SMT for Each Process
  - One PMT for Each Segment

### Direct Mapping

![Hybrid Mechanism Direct Mapping Procedure Example](images/hybrid-direct-mapping.png)

