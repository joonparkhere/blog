---
title: 라즈베리파이 4B OMV 설치하기
date: 2020-02-13
pin: false
tags:
- Raspberry Pi
- NAS
---

라즈베리파이 4로 NAS(Network Attached Storage)를 구축해보려 합니다. 이전 라즈베리파이보다 유선랜 속도, USB 3.0 지원 등 HW 스펙이 많이 좋아져서 개인용 NAS로는 충분할 것 같다는 생각이 들어, 덜컥 라즈베리파이 4B와 WD 2TB HDD를 질러버렸습니다.

1차적인 목표는 단순 개인용 스토리지를 구축하는 것이지만, 더 나아가 Plex 혹은 Jellyfin 플랫폼을 이용한 미디어 서버로 구동시키거나 하나의 웹 서버로도 확장 가능합니다.

오늘은 기본적인 OS 개요와 설치, 그리고 초기 세팅을 진행하려 합니다.

# OS 설치

NAS에 최적화된 OS인 OMV 5를 설치하기 위해 먼저 라즈베리파이 OS를 설치 해야합니다.



### Raspberry Pi OS 설치

`Raspberry Pi OS`는 라즈베리파이 재단에서 개발한 공식 전용 OS로, 데비안(Debian) 리눅스를 기반으로 만들어진 배포판이며 이전에는 라즈비안(Raspbian)이라고도 불렸습니다.

1. OS 파일 다운로드

   우선, [공식 웹사이트](https://www.raspberrypi.org/software/operating-systems/)에서 목적에 맞게 OS를 다운로드 받습니다. 지금은 NAS 구축용이기 때문에 GUI 없이 가장 가벼운 버전의 Lite를 다운로드 하겠습니다.

   

2. OS 설치용 microSD 카드 세팅

   다운받은 zip 파일을 플래싱할 microSD 카드가 필요합니다. (넉넉히 16GB 이상을 추천드립니다.)

   [SD Card Formatter ](https://www.sdcard.org/downloads/formatter/)와 같은 formatter로 SD카드를 말끔히 포맷하여 플래싱 준비를 마칩니다.

   

3. OS 파일 Flash 및 SSH 세팅

   [Etcher](https://www.balena.io/etcher/) 툴을 이용해 microSD 카드에 OS 파일을 flash 합니다. (대략 5~10분정도 시간이 소요됩니다.)

   정상적으로 플래싱을 마쳤다면 다시 microSD 카드를 PC에 꽂아서 초기 SSH 접근을 위한 세팅을 합니다. (설치한 OS가 Lite 버전이므로 Cli 환경에서 작업을 해야하므로 SSH가 필수입니다.)

   boot 디스크(microSD 카드)의 최상단 경로에 확장자가 없는 `ssh`파일을 생성해줍니다. (추후 OS 설치가 진행될 때 해당 파일의 유무로 초기 SSH 접속 허용을 결정하기 때문에 반드시 있어야합니다.)

   

4. OS 설치 및 네트워크 세팅

   boot 디스크(microSD 카드)를 라즈베리파이에 꽂아서 부팅해줍니다.

   그동안 간단한 네트워크 세팅을 하기 위해 라즈베리파이에 유선랜을 꽂고 사용중인 공유기 관리 페이지에 접속합니다. 향후에 라즈베리파이에 할당된 IP로 원격 접속을 할 것이기 떄문에 제멋대로 IP가 바뀌지 않도록 static IP 설정을 통해 고정해줍니다.

   

5. SSH 접속 및 초기 세팅

   대략 15~20분이 지났다면 같은 네트워크 환경의 PC에서 라즈베리파이에 SSH 접속을 시도해봅니다.

   ```bash
   ssh pi@[Host IP] -p [Port]
   ```

   위의 명령어를 `cmd`에 입력하여 접속하거나 [Putty](https://www.putty.org/)로 접속합니다. 초기 port는 `ssh` default인 22번입니다. (만약 `connection refused`가 된다면 라즈베리파이를 재부팅해보시기 바랍니다.)

   SSH 접속이 되었다면 초기 PW인 `raspberry`를 입력해서 `pi`계정에 접근하고 아래의 명령어로 PW를 변경합니다.

   ```bash
   sudo passwd pi
   ```

   이후 `OMV` OS 설치 후 SSH 접속을 허용하기 위해 아래의 명령어로 현재 계정에 권한을 부여합니다.

   ```bash
   sudo adduser pi ssh
   ```

   그리고 아래의 명령어로 OS 초기 세팅을 합니다.

   ```bash
   sudo raspi-config
   ```

   * microSD 카드의 용량 전체를 활용하기 위한 설정

     `Advanced Options` 의 `Expand Filesystem` 항목 선택

   * Locale 설정

     `Localisation Options`의 `Change Timezone` 항목 선택 후 `Asia/Seoul` 선택

   * SSH 설정

     `Interfacing Options`의 `SSH` 항목 선택

   

6. su 설정과 OS 업데이트

   OS를 최신으로 업데이트하기 위해 `root` 계정에 접근을 해야합니다.

   초기에는 `root` 계정 PW 설정이 되어있지 않으므로 아래의 명령어로 PW를 설정 및 로그인 후 업데이트 합니다. 

   ```bash
   sudo passwd root
   su
   ```

   ```bash
   apt-get update
   apt-get upgrade
   ```

이것으로 `Raspberry Pi OS` 세팅을 마쳤습니다. 이제 간단한 명령어 한 줄로 `OMV 5`를 설치할 수 있습니다.



### OpenMediaVault (OMV) 설치

`OMV`는 리눅스 기반의 Debian을 수정해서 만든 NAS 제작에 최적화된 OS 중 하나입니다. 이를 활용하여 SSH, FTP, SMB, BitTorrent 클라이언트 등의 서비스를 손쉽게 구성할 수 있습니다. 더불어, 최신 버전의 `Raspberry Pi OS`에서는 `OMV 5`를 설치해야 합니다.

1. OS 다운로드 및 설치

   ```bash
   wget -O - https://github.com/OpenMediaVault-Plugin-Developers/installScript/raw/master/install | sudo bash
   ```

   위의 명령어로 OS 설치를 위한 파일을 다운로드 받고 자동으로 설치까지 진행합니다. (꽤나 긴 시간이 소요됩니다.)

   정상적으로 과정을 마쳤다면 아래의 명령어로 라즈베리파이를 재부팅합니다.

   ```bash
   sudo reboot
   ```

   

2. OMV Web 접속

   `OMV 5` 설치를 잘 마쳤다면 다른 기기에서 OMV Web에 접근할 수 있습니다.

   동일 네트워크 환경의 PC에서 주소창에 [라즈베리파이 IP]를 입력하면 OMV 페이지가 로딩됩니다.

   초기 ID는 `admin`, PW는 `openmediavault`로 로그인할 수 있습니다.

   <img src="./images/omv_login.png" style="zoom:80%;" />



여기까지 NAS 구축을 위한 라즈베리파이 기본적인 OS 설치를 마쳤습니다.

이후에는 HDD를 마운트 해보도록 하겠습니다.

