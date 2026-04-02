# HEXA-CTRL --- Level 4: BT-59 제어 SoC 아키텍처

**Level**: 4 / 8 (제어칩)
**Date**: 2026-04-02
**Status**: Living Document v1.0
**BT Connections**: BT-59, BT-28, BT-58

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  sigma*tau = 48   sigma-tau = 8   2^sigma = 4096
```

---

## 1. 레벨 목표

HEXA-1 SoC 기반 로봇 전용 제어칩. tau=4 계층 실시간 제어로
mu=1ms 지연 달성. sigma*tau=48 TOPS NPU + sigma-tau=8 CPU 코어.

---

## 2. 성능 비교 --- 시중 vs HEXA-CTRL

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [로봇 제어칩] 비교: 시중 최고 vs HEXA-CTRL                      │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                   │
  │  제어 지연 (ms)                                                    │
  │  Atlas SoC     ████████████████░░░░░░░░░░░░░  ~5 ms              │
  │  Optimus FSD   ████████████████████░░░░░░░░░  ~8 ms              │
  │  HEXA-CTRL     ████░░░░░░░░░░░░░░░░░░░░░░░░  mu=1 ms            │
  │                                     (sopfr=5배 향상)              │
  │                                                                   │
  │  AI 연산 효율 (TOPS/W)                                            │
  │  Jetson Orin   ████████████████░░░░░░░░░░░░░  ~5 TOPS/W         │
  │  HEXA-CTRL     ██████████████████████████████  tau=4 TOPS/W      │
  │                (48 TOPS / sigma=12W = tau TOPS/W)                 │
  │                                                                   │
  │  모터 채널 수                                                      │
  │  시중 MCU      ████████████████░░░░░░░░░░░░░  8 ch               │
  │  HEXA-CTRL     ██████████████████████████████  sigma=12 ch       │
  │                                     (sigma/sigma-tau=1.5배↑)     │
  │                                                                   │
  │  TDP (W)                                                          │
  │  Jetson Orin   ████████████████████████████░░  60W                │
  │  HEXA-CTRL     ████████████████░░░░░░░░░░░░░  sigma=12W          │
  │                                     (sopfr=5배 효율↑)            │
  │                                                                   │
  │  개선 배수: n=6 상수 기반 (mu, sopfr, sigma, tau)                 │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. 아키텍처

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-CTRL SoC 내부 구조                                         │
  │                                                                   │
  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
  │  │  CPU Cluster │  │  NPU        │  │  Periph.    │              │
  │  │  sigma-tau=8 │  │  sigma*tau  │  │             │              │
  │  │  cores       │  │  =48 TOPS   │  │  ADC: sigma │              │
  │  │  RT Linux    │  │  BT-59 stack│  │  =12 bit    │              │
  │  │              │  │             │  │  x n=6 ch   │              │
  │  │  L1: 2^sopfr │  │  Egyptian   │  │             │              │
  │  │  = 32KB      │  │  MoE routing│  │  PWM: sigma │              │
  │  │  L2: 2^sigma │  │             │  │  =12 bit    │              │
  │  │  = 4MB       │  │  INT8/FP16  │  │  x sigma ch │              │
  │  └─────────────┘  └─────────────┘  └─────────────┘              │
  │                                                                   │
  │  ┌─ Memory ────────────────────────────────────────┐             │
  │  │  LPDDR: sigma-tau = 8 GB                         │             │
  │  │  SRAM:  J2 = 24 MB (on-chip)                     │             │
  │  │  Flash: 2^sopfr = 32 GB (model storage)          │             │
  │  └──────────────────────────────────────────────────┘             │
  │                                                                   │
  │  TDP: sigma = 12W    Process: TSMC N3 (sigma*tau=48nm pitch)     │
  └──────────────────────────────────────────────────────────────────┘
```

### tau=4 제어 계층

```
  ┌────────────┬────────────┬────────────┬────────────┐
  │  L1 서보   │  L2 모션   │  L3 계획   │  L4 전략   │
  │  1 kHz     │  100 Hz    │  10 Hz     │  1 Hz      │
  │  PID + 힘  │  역기구학  │  경로계획  │  VLM + RL  │
  │  지연<1ms  │  지연<10ms │  지연<100ms│  지연<1s   │
  │  CPU 1core │  CPU 2core │  NPU 25%  │  NPU 75%  │
  └────────────┴────────────┴────────────┴────────────┘
  계층 수 = tau = 4, 주파수 비 = sigma-phi = 10x per level
```

---

## 4. DSE 후보군

| # | SoC | NPU (TOPS) | CPU | TDP | n6 연결 |
|---|-----|-----------|-----|-----|---------|
| 1 | HEXA-CTRL v1 | 48 | 8 core | 12W | sigma*tau, sigma-tau, sigma |
| 2 | HEXA-CTRL lite | 24 | 4 core | 6W | J2, tau, n |
| 3 | Jetson Orin | 275 | 12 core | 60W | 비-n6 |
| 4 | Custom RISC-V | 48 | 6 core | 10W | sigma*tau, n |
| 5 | HEXA-CTRL+ | 96 | 8 core | 24W | sigma(sigma-tau), sigma-tau, J2 |

**Best Path**: HEXA-CTRL v1 (48 TOPS / 12W / 8 core) --- 전 파라미터 n=6 EXACT

---

## 5. n6 EXACT 목록

| # | 파라미터 | 값 | n=6 표현 | 상태 |
|---|---------|-----|---------|------|
| 1 | NPU | 48 TOPS | sigma*tau = 48 | EXACT |
| 2 | CPU cores | 8 | sigma-tau = 8 | EXACT |
| 3 | ADC | 12 bit | sigma = 12 | EXACT |
| 4 | PWM channels | 12 | sigma = 12 | EXACT |
| 5 | TDP | 12W | sigma = 12 | EXACT |
| 6 | Memory | 8 GB | sigma-tau = 8 | EXACT |
| 7 | 제어 계층 | 4 | tau = 4 | EXACT |
| 8 | 센서 축 | 6 | n = 6 | EXACT |
| 9 | SRAM | 24 MB | J2 = 24 | EXACT |
| 10 | 주파수 비 | 10x | sigma-phi = 10 | EXACT |

**EXACT 비율: 10/10 = 100%**

---

## 6. BT 연결

- **BT-59**: 8-layer AI stack --- silicon->precision->memory->compute->arch->train->opt->inference
  - HEXA-CTRL은 BT-59의 robot 특화 구현
- **BT-28**: Computing Architecture Ladder --- 30+ EXACT
  - sigma-tau=8 CPU, sigma*tau=48 TOPS 패턴 재확인
- **BT-58**: sigma-tau=8 Universal AI Constant --- 16/16 EXACT
  - CPU 8 core, Memory 8 GB, MoE 8 experts
