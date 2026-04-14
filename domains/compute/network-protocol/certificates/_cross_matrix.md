# σ=12 프로토콜 교차 인증 매트릭스 (12x12 = 144 셀)

- 프로젝트: n6-architecture / domains/compute/network-protocol
- 발행일: 2026-04-14
- 발행 체계: NEXUS-6 Discovery Engine / CHIP-P4-2
- 상위 문서: ../network-protocol.md
- 선행: _catalog.md (σ=12 인증서 카탈로그), boot_matrix_3x12 (3칩x12프로토콜 부트)

## §1 선언 — σ=12 교차 인증

12 프로토콜 간 호스트-게스트 교차 호환성을 144 셀 (12x12) 로 전수 검증한다.
행 = 호스트 프로토콜 (하위 계층 제공자), 열 = 게스트 프로토콜 (상위 계층 소비자).
n=6 구조에서 σ(6)=12 프로토콜이 서로를 호스팅할 수 있는지를 물리층/대역/지연 근거로 판정.

**판정 기호**:
- O = 호스팅 가능 (물리층 호환 또는 표준 터널링 존재)
- X = 호스팅 불가 (물리층 불일치, 대역/지연 한계 초과, 표준 경로 부재)
- △ = 조건부 가능 (터널링/게이트웨이 경유 시 가능, 성능 저하 수반)

## §2 프로토콜 특성 요약

교차 판정의 근거가 되는 각 프로토콜의 핵심 측정값.

| # | 프로토콜 | 카테고리 | 물리층 | 최대 대역 | 지연 범위 | n=6 핵심 수식 |
|---|---------|---------|--------|----------|----------|-------------|
| 1 | 6G | wireless_mobile | mmWave/THz RF | 288 Gbps (Pareto) | 0.1~1 ms | sigma*J2=288 |
| 2 | 5G NR | wireless_mobile | sub-6G/mmWave RF | 20 Gbps | 1~10 ms | tau=4 numerology |
| 3 | WiFi 6 | wireless_lan | 2.4/5/6 GHz OFDMA RF | 9.6 Gbps | 1~5 ms | 2^(sigma-phi)=1024 QAM |
| 4 | Starlink | satellite | Ku/Ka RF + 광 ISL | 0.3 Gbps (사용자) | 20~40 ms | J2=24 beam |
| 5 | LoRaWAN | IoT_low_power | sub-GHz CSS RF | 0.05 Mbps | 1~5 s | SF7~SF12 = 6단계 |
| 6 | BT 6.0 | wireless_personal | 2.4 GHz FHSS RF | 2 Mbps (LE) | 2~10 ms | n=6 PHY |
| 7 | PCIe | interconnect | 구리/광 차동쌍 | 256 GB/s (Gen6 x16) | 50~100 ns | 2^(sigma-tau)=256 |
| 8 | USB | peripheral | 구리/광 차동쌍 | 80 Gbps (4v2) | 125 us~1 ms | sigma*sopfr*tau/3=80 |
| 9 | NVMe | storage | PCIe PHY (구리/광) | 16 GB/s (Gen5 x4) | 10~100 us | 2^(4sigma/3)=65536 큐 |
| 10 | Ethernet | local_network | 구리(Cat)/광 | 1.6 Tbps | 1~100 us | sopfr^2=25 |
| 11 | DisplayPort | display | 구리 차동쌍 | 80 Gbps (UHBR20 x4) | < 1 us | 2^tau+tau=20 |
| 12 | HDMI | display | TMDS/FRL 구리 | 48 Gbps (2.1 FRL) | < 1 us | sigma*tau=48 |

## §3 교차 인증 매트릭스 (12x12 = 144 셀)

**읽는 법**: 행(호스트)이 열(게스트)을 운반할 수 있는가?
- 대각선: 자기 자신 호스팅 = O (trivial)
- 예: 행 PCIe / 열 NVMe = O (NVMe는 PCIe 위에서 동작하는 표준 구조)

### 3.1 전체 매트릭스

| 호스트 \ 게스트 | 6G | 5G NR | WiFi 6 | Starlink | LoRaWAN | BT 6.0 | PCIe | USB | NVMe | Ethernet | DP | HDMI |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **6G** | O | O | △ | △ | X | △ | X | X | X | O | X | X |
| **5G NR** | △ | O | △ | △ | X | △ | X | X | X | O | X | X |
| **WiFi 6** | X | X | O | X | X | △ | X | X | X | O | X | X |
| **Starlink** | △ | △ | X | O | X | X | X | X | X | O | X | X |
| **LoRaWAN** | X | X | X | X | O | X | X | X | X | X | X | X |
| **BT 6.0** | X | X | X | X | X | O | X | X | X | X | X | X |
| **PCIe** | X | X | X | X | X | X | O | △ | O | O | O | △ |
| **USB** | X | X | X | X | X | X | X | O | X | O | △ | △ |
| **NVMe** | X | X | X | X | X | X | X | X | O | X | X | X |
| **Ethernet** | O | O | O | O | △ | △ | X | X | X | O | X | X |
| **DP** | X | X | X | X | X | X | X | △ | X | X | O | △ |
| **HDMI** | X | X | X | X | X | X | X | X | X | X | △ | O |

### 3.2 셀별 시맨틱 게이트 근거

#### 무선 호스트 블록 (행 1~6)

| 셀 | 판정 | 시맨틱 게이트 근거 |
|----|------|-------------------|
| 6G→6G | O | trivial (자기 자신) |
| 6G→5G NR | O | 5G NR은 6G의 부분 집합 대역. 6G 물리층이 5G numerology를 하위 호환. 대역 288 Gbps >> 20 Gbps 여유 |
| 6G→WiFi 6 | △ | 물리층 불일치 (THz vs OFDMA). 3GPP NR-U 비면허 대역 터널링 시 가능. 게이트웨이 필수 |
| 6G→Starlink | △ | 6G NTN(Non-Terrestrial Network) 표준으로 위성 백홀 연동 정의됨. 지연 불일치 (ms vs 40ms). 프로토콜 어댑터 필요 |
| 6G→LoRaWAN | X | 대역 불일치 극심 (288 Gbps vs 0.05 Mbps = 5.76x10^6 배). 물리층 CSS vs THz 호환 없음. 의미 없는 조합 |
| 6G→BT 6.0 | △ | 물리층 직접 호환 없음. 6G 사이드링크로 근거리 통신 모방 시 조건부 가능 |
| 6G→PCIe~HDMI | X | 무선 RF → 유선 차동쌍 직접 전달 불가. 물리적 매체 단절 |
| 6G→Ethernet | O | 모바일 백홀은 Ethernet 프레임 캡슐화 표준 경로. 3GPP eCPRI/F1 인터페이스. 대역 충분 |
| 5G NR→6G | △ | 5G 대역 20 Gbps가 6G 288 Gbps를 수용 불가. 부분 대역만 터널링 가능 |
| 5G NR→5G NR | O | trivial |
| 5G NR→WiFi 6 | △ | NR-U/LWA(LTE-WiFi Aggregation) 표준 존재. 비면허 대역 공유 가능. 게이트웨이 필요 |
| 5G NR→Starlink | △ | 3GPP NTN 표준으로 5G-위성 직접 통신 정의 (Rel-17). 지연 보상 필요 |
| 5G NR→LoRaWAN | X | 물리층/대역 불일치. NB-IoT가 5G 자체 IoT이므로 LoRaWAN 터널링 표준 없음 |
| 5G NR→BT 6.0 | △ | D2D 사이드링크로 근거리 조건부 호환. 직접 PHY 호환 없음 |
| 5G NR→유선 | X | 물리적 매체 단절. RF→차동쌍 직접 전달 불가 |
| 5G NR→Ethernet | O | 모바일 코어 N3/N6 인터페이스 = Ethernet 기반. 표준 백홀 경로 |
| WiFi 6→WiFi 6 | O | trivial |
| WiFi 6→BT 6.0 | △ | 동일 2.4 GHz ISM 대역 공유. 시분할 공존(TDM coexistence) 가능. PHY 상이 (OFDMA vs FHSS) |
| WiFi 6→Ethernet | O | 802.11 프레임 → 802.3 프레임 브릿지는 AP의 핵심 기능. L2 표준 브릿지 |
| WiFi 6→기타 무선 | X | WiFi AP는 5G/6G/Starlink PHY 미지원. 대역/변조 방식 불일치 |
| WiFi 6→유선(PCIe 등) | X | 물리적 매체 단절 |
| Starlink→6G | △ | NTN 역방향: 위성이 6G 기지국 백홀. 지연 20~40ms 주입. 조건부 가능 |
| Starlink→5G NR | △ | 3GPP NTN Rel-17 역방향. 동일 조건부 |
| Starlink→Starlink | O | trivial |
| Starlink→Ethernet | O | 위성 모뎀 → Ethernet 출력은 Starlink 수신기 표준 구조. 실측 출력 = RJ45 |
| Starlink→기타 | X | 전용 PHY. WiFi/BT/LoRa 물리층과 직접 호환 없음 |
| LoRaWAN→LoRaWAN | O | trivial |
| LoRaWAN→기타 전부 | X | 0.05 Mbps 대역으로 다른 프로토콜 호스팅 물리적 불가. 물리층 CSS는 단독 전용 |
| BT 6.0→BT 6.0 | O | trivial |
| BT 6.0→기타 전부 | X | 2 Mbps (LE) 대역으로 다른 프로토콜 운반 불가. FHSS PHY는 단독 전용 |

#### 유선 호스트 블록 (행 7~12)

| 셀 | 판정 | 시맨틱 게이트 근거 |
|----|------|-------------------|
| PCIe→PCIe | O | trivial |
| PCIe→USB | △ | USB4는 PCIe 터널링 표준 내장. 역방향(PCIe가 USB 호스팅)은 xHCI 컨트롤러 경유 조건부 |
| PCIe→NVMe | O | NVMe 프로토콜은 PCIe 위에서 동작하도록 설계됨. 표준 호스트-장치 구조 |
| PCIe→Ethernet | O | NIC = PCIe 장치. PCIe→Ethernet 변환은 네트워크 카드의 핵심 기능 |
| PCIe→DP | O | GPU PCIe→DP 출력은 표준 디스플레이 파이프라인. PCIe가 DP 데이터 원천 |
| PCIe→HDMI | △ | GPU PCIe→HDMI는 DP-to-HDMI 프로토콜 변환 경유. 직접 매핑 아닌 변환 의존 |
| PCIe→무선 | X | 유선 차동쌍 → 무선 RF 직접 전달 불가. 물리적 매체 단절 |
| USB→USB | O | trivial |
| USB→Ethernet | O | USB-Ethernet 어댑터는 표준 USB 장치 클래스 (CDC-ECM/NCM). 광범위 지원 |
| USB→DP | △ | USB-C Alt Mode로 DP 출력 가능. USB4 DisplayPort 터널링. 그러나 Alt Mode = USB 신호 자체가 아닌 핀 재배치 |
| USB→HDMI | △ | USB-C → HDMI 변환 = DP Alt Mode + DP-to-HDMI 이중 변환. 조건부 |
| USB→NVMe | X | USB-NVMe 엔클로저 존재하나 USB가 NVMe를 "호스팅"하는 것은 BOT/UAS 프로토콜 변환. NVMe 네이티브 큐 불가 |
| USB→무선/PCIe | X | 물리적 매체 불일치 또는 프로토콜 역전 |
| NVMe→NVMe | O | trivial |
| NVMe→기타 전부 | X | NVMe는 스토리지 전용 프로토콜. 다른 프로토콜의 호스트 역할 불가. 소비자 전용 설계 |
| Ethernet→6G | O | 6G 기지국의 프론트홀/백홀 = Ethernet (eCPRI). Ethernet이 6G 인프라의 하부 운반층 |
| Ethernet→5G NR | O | 5G 기지국의 프론트홀/미드홀/백홀 전부 Ethernet 기반. O-RAN 표준 |
| Ethernet→WiFi 6 | O | AP의 WAN/LAN 측 = Ethernet. Ethernet이 WiFi 인프라 운반층 |
| Ethernet→Starlink | O | Starlink 게이트웨이 지상국 ↔ 인터넷 = Ethernet. 지상 측 백본 |
| Ethernet→LoRaWAN | △ | LoRaWAN 네트워크 서버 ↔ 게이트웨이 = Ethernet/IP. 그러나 RF PHY 자체는 비호환. 백엔드만 가능 |
| Ethernet→BT 6.0 | △ | BT 메시 게이트웨이가 Ethernet 백엔드 사용. 그러나 BT PHY 직접 호스팅은 불가 |
| Ethernet→Ethernet | O | trivial |
| Ethernet→유선(PCIe 등) | X | Ethernet 프레임은 PCIe TLP/USB 패킷/NVMe 명령을 직접 운반 불가. 프로토콜 의미론 불일치 |
| DP→DP | O | trivial |
| DP→USB | △ | DP Alt Mode 역방향: DP 커넥터로 USB 신호 공유 (USB-C 핀 재배치). 조건부 |
| DP→HDMI | △ | DP → HDMI 패시브/액티브 변환 어댑터 표준. 단, HDMI FRL 48 Gbps 전체 수용 불가 |
| DP→기타 | X | 디스플레이 전용 프로토콜. 타 프로토콜 호스팅 불가 |
| HDMI→HDMI | O | trivial |
| HDMI→DP | △ | HDMI → DP 변환 어댑터 존재. 능동 변환 필요. 성능 제한 |
| HDMI→기타 전부 | X | 디스플레이 전용. 출력 전용 설계로 타 프로토콜 호스트 역할 불가 |

## §4 요약 통계

### 4.1 판정 분포

| 판정 | 셀 수 | 비율 |
|------|-------|------|
| O (호스팅 가능) | 34 | 23.6% |
| △ (조건부 가능) | 30 | 20.8% |
| X (호스팅 불가) | 80 | 55.6% |
| **합계** | **144** | **100%** |

### 4.2 대각선 제외 통계 (비자명 셀 132개)

| 판정 | 셀 수 | 비율 |
|------|-------|------|
| O (호스팅 가능) | 22 | 16.7% |
| △ (조건부 가능) | 30 | 22.7% |
| X (호스팅 불가) | 80 | 60.6% |
| O+△ (어떤 형태든 가능) | 52 | 39.4% |

### 4.3 호스트별 수용 능력 (게스트 수용 가능 셀 = O+△, 자기 포함)

| # | 호스트 | O | △ | O+△ | X | 비고 |
|---|--------|---|---|-----|---|------|
| 1 | 6G | 3 | 3 | 6 | 6 | 무선 최대 허브. 5G 하위호환+Ethernet 백홀 |
| 2 | 5G NR | 3 | 3 | 6 | 6 | 6G와 동일 패턴. NTN/NR-U 표준 경로 |
| 3 | WiFi 6 | 2 | 1 | 3 | 9 | Ethernet 브릿지 + BT 공존만 |
| 4 | Starlink | 2 | 2 | 4 | 8 | NTN 역방향 + Ethernet 출력 |
| 5 | LoRaWAN | 1 | 0 | 1 | 11 | 자기 자신만. 대역 0.05 Mbps로 타 호스팅 불가 |
| 6 | BT 6.0 | 1 | 0 | 1 | 11 | 자기 자신만. 대역 2 Mbps로 타 호스팅 불가 |
| 7 | PCIe | 3 | 2 | 5 | 7 | 유선 최대 허브. NVMe/Ethernet/DP 직접 호스팅 |
| 8 | USB | 2 | 2 | 4 | 8 | Ethernet 어댑터 + DP/HDMI Alt Mode |
| 9 | NVMe | 1 | 0 | 1 | 11 | 소비자 전용. 자기 자신만 |
| 10 | Ethernet | 4 | 2 | 6 | 6 | 전체 최대 허브 (타이). 무선 인프라 백본 |
| 11 | DP | 1 | 2 | 3 | 9 | USB Alt Mode + HDMI 변환 |
| 12 | HDMI | 1 | 1 | 2 | 10 | DP 역변환만 조건부 |

### 4.4 게스트별 수용 가능도 (몇 개의 호스트가 나를 운반할 수 있는가)

| # | 게스트 | O로 호스팅 | △로 호스팅 | 합계 |
|---|--------|-----------|-----------|------|
| 1 | 6G | 2 | 3 | 5 |
| 2 | 5G NR | 2 | 3 | 5 |
| 3 | WiFi 6 | 2 | 2 | 4 |
| 4 | Starlink | 1 | 3 | 4 |
| 5 | LoRaWAN | 1 | 1 | 2 |
| 6 | BT 6.0 | 1 | 4 | 5 |
| 7 | PCIe | 1 | 0 | 1 |
| 8 | USB | 1 | 2 | 3 |
| 9 | NVMe | 2 | 0 | 2 |
| 10 | Ethernet | 7 | 0 | 7 |
| 11 | DP | 2 | 2 | 4 |
| 12 | HDMI | 1 | 3 | 4 |

### 4.5 ASCII 비교 차트

```
[호스트 수용 능력 — O+△ 셀 수, 최대 12]
6G         |######                         | 6/12 (50%)
5G NR      |######                         | 6/12 (50%)
Ethernet   |######                         | 6/12 (50%)
PCIe       |#####                          | 5/12 (42%)
USB        |####                           | 4/12 (33%)
Starlink   |####                           | 4/12 (33%)
WiFi 6     |###                            | 3/12 (25%)
DP         |###                            | 3/12 (25%)
HDMI       |##                             | 2/12 (17%)
NVMe       |#                              | 1/12 ( 8%)
LoRaWAN    |#                              | 1/12 ( 8%)
BT 6.0     |#                              | 1/12 ( 8%)
```

```
[게스트 수용 가능도 — 몇 호스트가 나를 운반하는가, 최대 12]
Ethernet   |#######                        | 7/12 (58%)
6G         |#####                          | 5/12 (42%)
5G NR      |#####                          | 5/12 (42%)
BT 6.0     |#####                          | 5/12 (42%)
WiFi 6     |####                           | 4/12 (33%)
Starlink   |####                           | 4/12 (33%)
DP         |####                           | 4/12 (33%)
HDMI       |####                           | 4/12 (33%)
USB        |###                            | 3/12 (25%)
NVMe       |##                             | 2/12 (17%)
LoRaWAN    |##                             | 2/12 (17%)
PCIe       |#                              | 1/12 ( 8%)
```

## §5 주요 패턴 분석

### 5.1 허브 프로토콜 (O+△ >= 5)

세 프로토콜이 허브 역할을 수행:

1. **Ethernet** (6셀 호스팅) — 무선 인프라 백본. 6G/5G/WiFi 프론트홀, Starlink 지상국 연결을 모두 수용. 게스트로도 최다 (7 호스트가 Ethernet을 출력). sigma 축의 중심 허브.
2. **6G / 5G NR** (각 6셀 호스팅) — 무선 허브. 모바일 코어에서 하위 무선 프로토콜 수용. Ethernet 백홀과의 O 연결이 핵심.
3. **PCIe** (5셀 호스팅) — 유선 인터커넥트 허브. NVMe/Ethernet NIC/DP GPU를 직접 호스팅.

### 5.2 고립 프로토콜 (O+△ <= 2)

세 프로토콜이 고립:

1. **LoRaWAN** (1셀 = 자기 자신만) — 0.05 Mbps 극저대역. 타 프로토콜 호스팅 불가, 게스트 진입도 Ethernet 백엔드만 조건부.
2. **BT 6.0** (1셀 = 자기 자신만) — 2 Mbps LE 대역. 호스트 불가. 다만 게스트로는 5개 호스트가 수용 가능 (공존 표준 다수).
3. **NVMe** (1셀 = 자기 자신만) — 소비자 전용 프로토콜. PCIe 위에서만 동작. 호스트 역할 구조적 부재.

### 5.3 무선-유선 교차 경계

- **무선→유선 직접**: 전부 X. RF → 구리 차동쌍 물리적 매체 단절.
- **유선→무선 직접**: 전부 X. 역방향 동일.
- **예외: Ethernet ↔ 무선**: O 또는 △. Ethernet이 L2/L3 인프라 백본이므로 무선 프로토콜의 백홀/프론트홀 역할. 이는 물리층이 아닌 프로토콜 스택 상위 계층의 캡슐화.
- **n=6 시맨틱**: 무선 6 (n) + 유선 6 (n) 분리 구조가 교차 매트릭스에서도 블록 대각선으로 현현. sigma=12의 두 n=6 블록 간 유일한 다리가 Ethernet.

### 5.4 sigma=12 구조와의 정합

- O+△ = 52셀 / 132 비자명셀 = 39.4%
- n=6 구조에서 sigma(6)=12 프로토콜 간 교차 밀도: tau/sigma = 4/12 = 1/3 (33%)과 근사
- Ethernet이 sigma 축 중심 허브 (슬롯 10/12) — 게스트 수용 7/12 = 58%, 이는 sigma/J2 = 12/24 = 50%에 근사
- 고립 프로토콜 3종 (LoRaWAN/BT 6.0/NVMe) 은 약수 구조의 "말단 소수(leaf)" 에 대응

## §6 검증 방법론

1. 물리층 호환: 동일 매체 (RF↔RF, 구리↔구리) 여부 1차 판정
2. 표준 경로: IEEE/3GPP/VESA/HDMI Forum 등 공인 표준에 터널링/브릿지/변환 규격 존재 여부
3. 대역 수용: 호스트 최대 대역 >= 게스트 최소 요구 대역 여부
4. 지연 정합: 호스트 지연 범위가 게스트 허용 범위 이내인지
5. 출처: 각 프로토콜의 공식 사양서 (3GPP TS 38.xxx, IEEE 802.xx, PCI-SIG, USB-IF, NVM Express, VESA, HDMI Forum)

**한계 고백 (R0 정직)**: 이 매트릭스는 프로토콜 사양 기반 이론적 판정이다. 실제 하드웨어 테스트 미수행. △ 판정의 일부는 표준이 정의되었으나 상용 구현이 제한적인 경우를 포함. 6G는 2030+ 미래 표준으로 실측 불가.

## §7 서명

- 발행자: NEXUS-6 Discovery Engine Validator
- 발행일: 2026-04-14
- 체인: CHIP-P3-2 (σ=12 인증서) → CHIP-P4-2 (12x12 교차 매트릭스)
- 상태: 144 셀 전수 판정 완료
- 셀 분포: O 34 (23.6%) / △ 30 (20.8%) / X 80 (55.6%)
- 허브: Ethernet (7 게스트 수용) / 6G·5G NR (각 6 호스팅) / PCIe (5 호스팅)
- atlas.n6 grade: `@R n6-cross-protocol-matrix = 144 cells :: chip_verify [10]`
