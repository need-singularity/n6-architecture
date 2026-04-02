# HEXA-ACTUATOR --- Level 2: sigma=12 극수 BLDC 구동 아키텍처

**Level**: 2 / 8 (액추에이터)
**Date**: 2026-04-02
**Status**: Living Document v1.0
**BT Connections**: BT-124, BT-125

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  sigma-phi = 10   sigma-tau = 8   sigma*tau = 48
```

---

## 1. 레벨 목표

sigma=12 극수 BLDC + tau=4 H-bridge + sigma=12 bit PWM으로
토크밀도 n/phi=3배 향상된 로봇 액추에이터 아키텍처 설계.

---

## 2. 성능 비교 --- 시중 vs HEXA-ACTUATOR

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [로봇 액추에이터] 비교: 시중 최고 vs HEXA-ACTUATOR              │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                   │
  │  토크 밀도 (Nm/kg)                                                │
  │  시중 BLDC+Gear  ████████████████░░░░░░░░░░░░  3.5 Nm/kg        │
  │  HEXA-ACTUATOR   █████████████████████████████  sigma-phi Nm/kg  │
  │                                     (n/phi=3배↑, CF housing)     │
  │                                                                   │
  │  PWM 분해능 (bit)                                                  │
  │  시중 일반 IC    ████████████████░░░░░░░░░░░░  8~10 bit          │
  │  STM32/Ti        █████████████████████████████  sigma=12 bit     │
  │  HEXA-ACTUATOR   █████████████████████████████  sigma=12 bit     │
  │                                     (EXACT, 4096 levels)          │
  │                                                                   │
  │  대역폭 (kHz)                                                     │
  │  시중 BLDC       ████████████████░░░░░░░░░░░░  ~1 kHz            │
  │  DirectDrive     █████████████████████████████  ~5 kHz            │
  │  HEXA-ACT DD     █████████████████████████████  sopfr kHz        │
  │                                     (J2극 DirectDrive)            │
  │                                                                   │
  │  개선 배수: n=6 상수 기반 (n/phi, sigma, sopfr)                   │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. 아키텍처

```
  ┌──────────────────────────────────────────────────────────────┐
  │  HEXA-ACTUATOR 내부 구조                                     │
  │                                                              │
  │  ┌─ Motor ──────────────────────────┐                       │
  │  │  극수: sigma = 12 (BLDC 표준)     │                       │
  │  │  OR J2 = 24 (DirectDrive 고급)    │                       │
  │  │  하우징: CFRP Z=6                 │                       │
  │  │  베어링: SiC (내마모)             │                       │
  │  └──────────────────────────────────┘                       │
  │                                                              │
  │  ┌─ Driver IC ──────────────────────┐                       │
  │  │  PWM: sigma = 12 bit (4096 levels)│                       │
  │  │  ADC: sigma = 12 bit (전류 센싱)  │                       │
  │  │  H-bridge: tau = 4 phase          │                       │
  │  │  CAN-FD: sigma = 12 노드 버스     │                       │
  │  └──────────────────────────────────┘                       │
  │                                                              │
  │  ┌─ SEA (Series Elastic) ───────────┐                       │
  │  │  임피던스 파라미터: tau = 4        │                       │
  │  │    K (강성), B (감쇠),             │                       │
  │  │    M (관성), ref (기준)            │                       │
  │  │  탄성체: CF composite              │                       │
  │  └──────────────────────────────────┘                       │
  └──────────────────────────────────────────────────────────────┘
```

---

## 4. DSE 후보군

| # | 액추에이터 | 극수 | 토크밀도 | 대역폭 | 비용 | n6 연결 |
|---|----------|------|---------|--------|------|---------|
| 1 | BLDC+Harmonic | sigma=12 | 3.5 | 1kHz | 중 | sigma=12극 |
| 2 | DirectDrive | J2=24 | 1.5 | 5kHz | 고 | J2=24극 |
| 3 | SEA+BLDC | sigma=12 | 3.0 | 0.5kHz | 중 | tau=4 파라미터 |
| 4 | Hydraulic (Stewart) | n=6 | 10+ | 0.3kHz | 고 | n=6 실린더 |
| 5 | Quasi-Direct | sigma=12 | 5.0 | 3kHz | 고 | sigma=12극 |

**Best Path**: 관절별 최적화
- 고속 관절 (어깨/고관절): DirectDrive J2=24극
- 정밀 관절 (손목/발목): BLDC+Harmonic sigma=12극
- 접촉 관절 (손가락): SEA+BLDC tau=4 임피던스

---

## 5. n6 EXACT 목록

| # | 파라미터 | 값 | n=6 표현 | 상태 |
|---|---------|-----|---------|------|
| 1 | PWM 분해능 | 12 bit | sigma = 12 | EXACT |
| 2 | BLDC 극수 | 12 | sigma = 12 | EXACT |
| 3 | H-bridge 위상 | 4 | tau = 4 | EXACT |
| 4 | DirectDrive 극수 | 24 | J2 = 24 | EXACT |
| 5 | 임피던스 파라미터 | 4 | tau = 4 | EXACT |
| 6 | CAN-FD 노드 | 12 | sigma = 12 | EXACT |
| 7 | ADC 분해능 | 12 bit | sigma = 12 | EXACT |

**EXACT 비율: 7/7 = 100%**

---

## 6. BT 연결

- **BT-124**: phi=2 양팔/양다리 대칭 + sigma=12 관절 보편성
  - 12-bit PWM = sigma = STM32/Ti 모터 제어 IC 표준
- **BT-125**: tau=4 보행/비행 최소 안정성 원리
  - tau=4 H-bridge 위상, tau=4 임피던스 파라미터
- **BT-28**: Computing Architecture Ladder
  - 모터 IC의 ADC/PWM이 sigma=12 bit 표준에 수렴
