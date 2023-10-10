---
title: "[Ethereum] Gas"
date: 2023-10-10
pin: false
tags:
- Blockchain
- Ethereum
- Gas
---

## About a Gas

![EVM Execution Model](images/evm-execution-model.png)

### What

- Measure Amount of Computational Effort
- Protect against Spam or Infinite Loops
- $Gas Fee = Gas * Gas Price$

![Gas Supply and Refund Flow](images/gas-flow.png)

- Refund $Max \ Fee - (Base \ Fee + Priority \ Fee)$





## Gas Fee Calculation

- Next Block에 포함되기 위해서는 Gas Fee 입찰에 성공해야 함

### Base Fee

| Block Number | Included Gas | Fee Increase | Current Base Fee |
| :----------- | -----------: | -----------: | ---------------: |
| 1            |          15M |           0% |         100 gwei |
| 2            |          30M |           0% |         100 gwei |
| 3            |          30M |        12.5% |       112.5 gwei |
| 4            |          30M |        12.5% |       126.6 gwei |

- Previous Block 상태에 따라 결정됨
- Base Fee를 명시한 Transaction이 담긴 Block이 생성될 경우 Base Fee는 소각됨

### Priority Fee

- Validator Incentive Tip

- $Total \ Fee=(Units \ of \ Gas) * (Base \ Fee + Priority \ Fee)$

- Example: Alice Send 1ETH to BOB

  - Need Gas: 21,000
  - Set Base Fee: 10 gwei
  - Set Priority Fee: 2 gwei

  $Total \ Fee=21,000 * (10+2)=252,000 \ gwei=0.000252 \ ETH$



## References

- [Ethereum Gas](https://ethereum.org/en/developers/docs/gas/)