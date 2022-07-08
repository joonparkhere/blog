---
title: KAS SDK 간략 소개
date: 2020-11-14
pin: false
tags:
- Blockchain
- Klaytn
---

Klaytn에서 활용할 수 있는 여러가지 툴이 있습니다.

* 웹 기반 지갑 Kaikas
* 카카오톡 내 지갑 Klip
* 개발 SDK caver
* API 지원 Klaytn API Service
* 트랜잭션 조회 Klaytnscope

이전에 Bapp 혹은 Dapp 개발을 할 경우에, Klaytn을 사용하기 위해서는 두 가지 방법이 있었습니다.

1. caver SDK를 사용하는 방법
2. axios 모듈 등을 활용해 KAS를 사용하는 방법

두 가지 툴이 제공하는 기능들이 꽤나 유사하지만 구현시의 방법이 너무나도 다르기 때문에 대부분의 경우 둘 중 하나만 선택해서 개발을 하곤 했습니다. 그러나 Klaytn측에 caver SDK 확장 버전, caver-js/java-ext-kas v1.0.0을 공개했습니다.

이 확장버전은 기존 caver-js/java의 확장 라이브러리롤 Klaytn의 JavaScript/Java SDK인 caver-js/java 기능과 KAS JavaScript/Java SDK 기능을 함께 제공합니다. 즉, 기존의 caver SDK 기능은 유지하면서도 확장된 SDK를 통해 API call을 할 수 있게 되었습니다. 먼저 KAS에서 어떤 기능을 제공해주는 간략하게 설명드리겠습니다.

KAS는 보다 간편하게 개발할 수 있도록 기존 SDK에는 없거나 보다 확장된 4가지 기능을 제공합니다.

* Klaytn Node
* Token History
* Wallet
* Anchor

첫번째 Node API는 Klaytn의 기본적인 기능들을 활용할 수 있게 해주는 API입니다. 원래 Klaytn을 사용하려면 실제 Klaytn 네트워크의 노드가 되어 해당 노드를 운영해야 사용가능하지만, Node API로 Klaytn에 접속해 블록체인 플랫폼을 바로 활용할 수 있습니다. 예를 들어, JSON RPC 호출, FT/NFT 컨트랙트 조회 등의 기능이 가능합니다.

두번째 Token History API을 사용하면 Klaytn 계정이 Klay, FT (KIP-7, ERC-20), NFT (KIP-17, ERC-721)을 보내고 받은 기록, 토큰 컨트랙트 정보, NFT 정보, NFT 소유권 변동 기록을 조회할 수 있습니다. 이전에는 Klaytnscope에서 일일이 확인해야 했던 기능이 API call을 통해 데이터를 가져올 수 있습니다.

세번째 Wallet API는 Klaytn 계정을 만들어 관리하고 트랜잭션을 전송해주는 API입니다. 이 API로 계정을 만들면 개인키 관리를 별도의 지갑이 해주기 때문에 따로 관리할 필요 없습니다. Wallet API는 크게 계정을 만들고 관리하는 Account 파트와 트랜잭션을 전송하는 Transaction 파트로 나뉩니다. 기존 caver SDK에서는 제공하지 않았던, KAS console로 Bapp에서 사용 중인 Account Pool 혹은 Fee-payer Pool을 확인할 수 있는 기능을 제공합니다.

마지막 Anchor API는 서비스 체인 데이터의 신뢰성을 보장하기 위해 데이터 신뢰성을 증명할 수 있는 메타데이터를 Klaytn 메인 체인에 전송하는 기능을 제공합니다.

caver 확장버전이 나오기 전, 위의 4가지 기능들을 사용하기 위해서는 매번 HTTP call을 위해 API Url, API Key, Params 등을 세팅해주고, response를 파싱해줘야 했습니다. 하지만 이제는 caver-js/java-ext-kas 툴을 이용해 더욱 간편하게 구현할 수 있습니다. 위의 4가지  기능 외에도 약간의 추가 기능들을 제공합니다. 그에 대한 설명은 아래와 같습니다.

* caver.initKASAPI
* caver.rpc
* caver.kas.tokenHistory
* caver.kas.wallet
* caver.kas.anchor

</br>
1.Set auth

본격적으로 caver 확장버전을 개발 시 사용하기 앞서, caver instance를 개발자의 API Key에 맞게 할당하는 과정이 필요합니다.
```
const caver = new CaverExtKAS()
caver.initKASAPI(chainId, accessKeyId, secretAccessKey)
```
위와 같이 초기에만 caver instance를 초기화 해주면 이후로는 API Key를 매번 params로 넣어줄 필요가 없습니다. 이외에도 각 KAS API 서비스에 대한 초기화 함수도 제공합니다.

* caver.initNodeAPI
* caver.initTokenHistoryAPI
* caver.initWalletAPI
* caver.initAnchorAPI

</br>
2. Use rpc

기존 caver SDK에도 rpc 모듈은 존재합니다. 다만 caver 확장버전에서의 rpc모듈은 KAS Node API 기능을 추가로 담고있습니다.

* caver.rpc.klay
* caver.rpc.net

</br>
3. Use tokenHistory

`caver.kas.tokenHistory`는 KAS의 tokenHistory API가 제공해주는 기능들을 caver SDK로 사용할 수 있습니다. 대표적인 기능은 다음과 같습니다.

* `GET /v2/contract/ft`로 FT 컨트랙트 리스트 가져오기
* `GET /v2/contract/ft/{ft-address}`로 FT 컨트랙트 주소 가져오기
* `Get /v2/contract/nft/{nft-address}/token`로 NFT 리스트 가져오기
* `Get /v2/contract/nft/{nft-address}/token/{token-id}`로 NFT의 소유자 변경 기록 가져오기

이외에도 다양한 추가기능들을 지원합니다.

</br>
4. Use wallet

`caver.kas.wallet`은 계정 관리와 여러 종류의 트랜잭션을 생성하고 보내는 기능을 제공합니다. 대표적인 기능은 다음과 같습니다.

* `POST /v2/account`로 새로운 계정 만들기
```js
const result = await caver.kas.wallet.createAccount()
```

* `PUT /v2/account/{address}/disable`로 해당 계정 비활성화하기
```js
const result = await caver.kas.wallet.disableAccount(address)
```

* `POST /v2/account/{address}/tx/{transaction-id}/sign`로 트랜잭션에 서명하기
```js
const result = await caver.kas.wallet.signTransaction(address, transactionId)
```

* `POST /v2/tx/contract/deploy`로 새로운 컨트랙트 배포하기
```js
const tx = {
  from: '0x...',
  value: 0,
  input: '0x...',
  gas: 1000000,
  submit: true
}
const result = await caver.kas.wallet.requestSmartContractDeploy(tx)
```

* `PUT /v2/tx/account`로 계정 정보 갱신하기
```js
const tx = {
  from: '0x...',
  accountKey: {
    keyType: 4,
    key: {
      threshold: 2,
      weightedKeys: [
        {
          weight: 1,
          publicKey: '0x...PubKey01'
        },
        {
          weight: 1,
          publicKey: '0x...PubKey02'
        }
      ]
    }
  },
  gas: 1000000,
  submit: true
}
const result = await caver.kas.wallet.requestAccountUpdate(tx)
```

* `POST /v2/tx/fd/contract/deploy`로 새로운 컨트랙트를 글로벌 수수료 담당자(KAS)가 배포 시의 수수료를 일정 부분 감당하여 배포하기
```js
const tx = {
  from: '0x...',
  value: 0,
  input: '0x...',
  gas: 1000000,
  submit: true,
  feeRatio: 99
}
const result = await caver.kas.wallet.requestFDSmartContractDeployPaidByGlobalFeePayer(tx)
```


이외에도 정말 다양한 계정 및 트랜잭션 관련 API call을 지원합니다. 개발 시 가장 많이 쓰일 부분이므로 KAS Docs 등을 참고하여 자세히 살펴보면 좋을 것 같습니다.