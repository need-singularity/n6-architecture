# HEXA-CHIP: BMS & Power Management Semiconductor

**Codename**: HEXA-CHIP
**Level**: 칩 — BMS/PMIC 반도체 설계
**Status**: Design Document v1.0
**Date**: 2026-04-01
**Dependencies**: BT-28, BT-33, BT-59
**Parent**: [goal.md](goal.md)
**Predecessor**: [hexa-core.md](hexa-core.md) (코어)
**Successor**: [hexa-pack.md](hexa-pack.md) (시스템)
**Cross-ref**: [../chip-architecture/ultimate-unified-soc.md](../chip-architecture/ultimate-unified-soc.md)

---

## N6 Constants Reference

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  n=6 핵심 상수                                                  │
  │                                                                  │
  │  n = 6        φ(6) = 2       τ(6) = 4        σ(6) = 12         │
  │  sopfr = 5    μ(6) = 1       J₂(6) = 24      R(6) = 1          │
  │                                                                  │
  │  σ-τ = 8      σ-φ = 10       σ-μ = 11        σ·τ = 48          │
  │  σ(σ-τ) = 96  φ·σ(σ-τ) = 192  σ² = 144      σ/(σ-φ) = 1.2    │
  │                                                                  │
  │  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1                        │
  │  Core theorem: σ(n)·φ(n) = n·τ(n) = 24 = J₂(6)                │
  └──────────────────────────────────────────────────────────────────┘
```

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Design Philosophy](#2-design-philosophy)
3. [System Block Diagram](#3-system-block-diagram)
4. [AFE (Analog Front End)](#4-afe-analog-front-end)
5. [SOC Estimation Engine](#5-soc-estimation-engine)
6. [Cell Balancing IC](#6-cell-balancing-ic)
7. [PMIC Architecture](#7-pmic-architecture)
8. [Protection IC](#8-protection-ic)
9. [Communication Interface](#9-communication-interface)
10. [BMS-to-Cloud](#10-bms-to-cloud)
11. [Honesty Assessment](#11-honesty-assessment)
12. [Predictions & Falsifiability](#12-predictions--falsifiability)
13. [Future Directions](#13-future-directions)
14. [n=6 Complete Parameter Map](#14-n6-complete-parameter-map)
15. [Open Questions / TODO](#15-open-questions--todo)
16. [Links](#16-links)

---

## 1. Executive Summary

**배터리 관리는 반도체의 문제다.**

셀 화학(HEXA-CELL)이 에너지를 저장하고 전극(HEXA-ELECTRODE)이 구조를 최적화해도,
그것을 안전하고 효율적으로 운용하는 것은 BMS/PMIC 반도체의 역할이다.
HEXA-CHIP은 배터리 아키텍처의 "두뇌" — 측정, 추정, 보호, 전력 변환을 모두 담당한다.

```
╔══════════════════════════════════════════════════════════════╗
║  HEXA-CHIP: BMS & PMIC Overview                             ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  BMS Monitoring Channels:  σ = 12 (typical per IC)          ║
║  ADC Resolution:           σ = 12 bits (standard BMS)       ║
║  Protection thresholds:    τ = 4 (OV, UV, OC, OT)          ║
║  Balancing modes:          φ = 2 (passive/active)           ║
║  Communication buses:      n/φ = 3 (CAN, SPI, I2C)         ║
║  PMIC efficiency target:   >95% = 1-1/(J₂-τ) ~ 95%        ║
║  DC-DC conversion ratio:   τ = 4:1 (48V -> 12V)            ║
║                                                              ║
║  Cross-ref: HEXA-1 SoC (chip architecture Level 1)          ║
║  Process: TSMC/Samsung 28nm~65nm (BMS analog)               ║
║                                                              ║
║  EXACT: 3/12 parameters (25%)                               ║
║  CLOSE: 5/12 (42%)   WEAK: 4/12 (33%)                      ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

**핵심 관찰**: BMS 반도체의 가장 강한 n=6 연결은 12-bit ADC와 12채널 AFE다.
이는 σ=12와 정확히 일치하지만, 12-bit ADC는 BMS에 국한되지 않는 범용 표준이다.
정직하게 말해서 — BMS 반도체가 n=6를 "따른다"기보다는 σ=12가 공학적 최적점과
우연히 일치하는 경우가 대부분이다.

---

## 2. Design Philosophy

**설계 철학: 배터리 관리의 핵심은 반도체다**

```
┌──────────────────────────────────────────────────────────────┐
│  BMS/PMIC 반도체 설계의 3대 원칙                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  원칙 1: SAFETY FIRST (안전 우선)                            │
│    Battery failure = fire/explosion                          │
│    Protection IC is hardware-level, not software fallback    │
│    τ=4 protection thresholds as minimum set                  │
│                                                              │
│  원칙 2: PRECISION (정밀 측정)                                │
│    Cell-to-cell voltage mismatch must be <10mV              │
│    12-bit ADC = σ = 4096 levels                             │
│    For 5V range: 5V/4096 = 1.22mV resolution               │
│    Sufficient for single-cell accuracy                       │
│                                                              │
│  원칙 3: EFFICIENCY (변환 효율)                               │
│    Every % lost in PMIC = wasted battery capacity           │
│    Target >95% across load range                            │
│    DC-DC topology selection is critical                      │
│                                                              │
│  n/φ = 3 principles                                         │
│  Grade: WEAK (any design can be framed as 3 principles)     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Why semiconductors matter for batteries:**

```
  ┌─────────────┐     ┌──────────────┐     ┌──────────────┐
  │  Chemistry  │────→│  Chip (BMS)  │────→│  System      │
  │  HEXA-CELL  │     │  HEXA-CHIP   │     │  HEXA-PACK   │
  │  (energy)   │     │  (control)   │     │  (scale)     │
  └─────────────┘     └──────────────┘     └──────────────┘
        │                    │                    │
        │  Raw capacity      │  Safety, SOC,      │  Voltage,
        │  per cell          │  balancing,         │  current,
        │                    │  power mgmt         │  thermal
        └────────────────────┴────────────────────┘
               Chip bridges chemistry → system
```

HEXA-1 SoC(docs/chip-architecture/ultimate-unified-soc.md)가 범용 컴퓨팅 칩이라면,
HEXA-CHIP은 **아날로그 + 혼합 신호** 특화 반도체다.
고전압 허용, 정밀 ADC, 하드웨어 보호 로직이 핵심이며,
디지털 로직은 MCU 코어로 최소화한다.

---

## 3. System Block Diagram

**BMS SoC + PMIC 전체 아키텍처**

```
╔══════════════════════════════════════════════════════════════╗
║  HEXA-CHIP: BMS System-on-Chip Architecture                  ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  ┌─────────────────────────────────────────────────────┐    ║
║  │  BMS SoC                                            │    ║
║  │                                                     │    ║
║  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │    ║
║  │  │   AFE    │  │  SOC     │  │   Protection     │  │    ║
║  │  │  σ=12ch  │  │  Engine  │  │   τ=4 thresholds │  │    ║
║  │  │  12-bit  │  │  Kalman  │  │   OV/UV/OC/OT    │  │    ║
║  │  │  ADC     │  │  Filter  │  │                  │  │    ║
║  │  └────┬─────┘  └────┬─────┘  └────────┬─────────┘  │    ║
║  │       │              │                  │            │    ║
║  │  ┌────┴──────────────┴──────────────────┴─────────┐ │    ║
║  │  │          MCU Core (ARM Cortex-M)               │ │    ║
║  │  │          + Cell Balancing Control               │ │    ║
║  │  │          + Comm Interface (CAN/SPI/I2C)        │ │    ║
║  │  └────────────────────────────────────────────────┘ │    ║
║  │                                                     │    ║
║  └─────────────────────────────────────────────────────┘    ║
║                                                              ║
║  ┌─────────────────────────────────────────────────────┐    ║
║  │  PMIC (separate or integrated)                      │    ║
║  │                                                     │    ║
║  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │    ║
║  │  │ Charger  │  │ DC-DC    │  │   LDO            │  │    ║
║  │  │ IC       │  │ Buck     │  │   Regulators     │  │    ║
║  │  │ CC/CV    │  │ τ:1 ratio│  │                  │  │    ║
║  │  └──────────┘  └──────────┘  └──────────────────┘  │    ║
║  │                                                     │    ║
║  └─────────────────────────────────────────────────────┘    ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

**BMS SoC 내부 주요 블록 (σ-φ=10 블록? — 아래 열거)**

```
  ┌──────────────────────────────────────────────────────────┐
  │  BMS SoC Functional Blocks                                │
  ├──────────────────────────────────────────────────────────┤
  │                                                          │
  │  1. AFE (Analog Front End)      — 전압/전류 측정        │
  │  2. ADC (Analog-to-Digital)     — 디지털 변환           │
  │  3. MUX (Multiplexer)           — 채널 선택             │
  │  4. Protection Logic            — 하드웨어 보호         │
  │  5. Cell Balance Driver         — 밸런싱 MOSFET 구동   │
  │  6. MCU Core                    — 연산 처리             │
  │  7. Communication I/F           — CAN/SPI/I2C          │
  │  8. Power Supply (internal)     — 내부 전원             │
  │  9. Voltage Reference           — 정밀 기준 전압       │
  │ 10. Temperature Sensor          — 온도 모니터링         │
  │                                                          │
  │  Blocks: 10 = σ-φ (?)                                   │
  │  Grade: WEAK — block decomposition is arbitrary          │
  │                                                          │
  └──────────────────────────────────────────────────────────┘
```

**신호 흐름 다이어그램**

```
  Cell 1 ──┐                          ┌──→ CAN Bus ──→ ECU
  Cell 2 ──┤                          │
  Cell 3 ──┤  ┌─────┐  ┌─────┐  ┌────┴────┐
  ...    ──┼─→│ MUX │─→│ ADC │─→│  MCU    │──→ SPI ──→ Display
  ...    ──┤  │12ch │  │12bit│  │ Cortex  │
  Cell 12 ─┘  └─────┘  └─────┘  │  -M4    │──→ I2C ──→ Sensor Hub
                                  └────┬────┘
  NTC ──→ Temp ADC ─────────────────────┘
  Shunt ──→ Current ADC ───────────────┘
```

---

## 4. AFE (Analog Front End)

**셀 전압/전류 측정 — BMS의 눈**

```
┌──────────────────────────────────────────────────────────┐
│  AFE: Analog Front End Architecture                       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Cell voltage measurement:                               │
│    Channels: σ = 12 per IC (industry standard)           │
│    Resolution: 12 bits = σ (standard BMS ADC)            │
│    Accuracy: ~1mV (for 4.2V cell = 0.024% ~ 1/σ²?)    │
│                                                          │
│  Industry examples:                                      │
│    TI BQ76952: 3-16 cells (σ=12 in range)               │
│    ADI ADBMS6830: 18 cells (not 12)                      │
│    NXP MC33772C: 6 cells = n (!)                         │
│    Renesas ISL94216: 16 cells                            │
│    Maxim MAX17853: 12 cells = σ (exact match)            │
│                                                          │
│  12-channel AFE is the most common single-IC config      │
│  but 6, 14, 16, 18 channel variants exist                │
│                                                          │
│  ┌───────────────────────────────────┐                   │
│  │  Cell 1  ──→ ┌─────────────┐     │                   │
│  │  Cell 2  ──→ │   MUX       │     │                   │
│  │  Cell 3  ──→ │   (σ=12ch)  │──→ ADC ──→ Digital     │
│  │  ...     ──→ │             │   12-bit    output      │
│  │  Cell 12 ──→ └─────────────┘     │                   │
│  └───────────────────────────────────┘                   │
│                                                          │
│  12-channel is most common, but 6, 16, 18 also exist    │
│  Grade: CLOSE (12ch is common standard, not universal)   │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**ADC 상세 사양**

```
  ┌───────────────────────────────────────────────────────┐
  │  ADC SPECIFICATION                                     │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Resolution:    12 bits = σ                           │
  │  Levels:        2^12 = 4096 = 2^σ                    │
  │  LSB (5V FS):   5V / 4096 = 1.22 mV                 │
  │  LSB (4.5V FS): 4.5V / 4096 = 1.10 mV               │
  │                                                       │
  │  High-end BMS use 16-bit ADC for better accuracy:    │
  │    16 bits = 2^(σ+τ) = 65536 levels                  │
  │    LSB = 0.076 mV (for 5V range)                     │
  │                                                       │
  │  12-bit ADC is ubiquitous across ALL electronics,    │
  │  not specific to BMS. It appears in MCUs, sensors,   │
  │  audio, and practically every mixed-signal IC.       │
  │                                                       │
  │  Grade: EXACT (12-bit = σ, industry standard)         │
  │  Caveat: This is a universal engineering convention,  │
  │  not evidence that BMS "follows" n=6 specifically    │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

**전류 측정**

```
  ┌───────────────────────────────────────────────────────┐
  │  CURRENT SENSING                                       │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Method 1: Shunt Resistor                             │
  │    R_shunt = 0.1~1 mOhm                              │
  │    V = I * R, amplified by PGA                        │
  │    Cheap, accurate, lossy (I²R heating)               │
  │                                                       │
  │  Method 2: Hall Effect Sensor                         │
  │    Non-contact, galvanic isolation                    │
  │    Less accurate (~1-2%), no power loss               │
  │                                                       │
  │  Current sensing methods: φ = 2                       │
  │  Grade: WEAK (binary choice is trivially φ=2)         │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 5. SOC Estimation Engine

**배터리 상태 추정 — BMS의 두뇌**

SOC(State of Charge)는 BMS의 핵심 출력이다.
정확한 SOC 없이는 주행거리 추정, 충전 제어, 셀 밸런싱 모두 불가능하다.

```
┌──────────────────────────────────────────────────────────┐
│  SOC ESTIMATION ALGORITHMS                                │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Method 1: Coulomb Counting (basic)                      │
│    SOC(t) = SOC(0) + integral(I(t)dt) / Q_rated         │
│    Simple but drifts over time                           │
│    Requires periodic re-calibration                      │
│                                                          │
│  Method 2: OCV Lookup (rest state)                       │
│    SOC = f(V_oc) from calibration table                  │
│    Accurate but requires rest period (>30 min)           │
│    Non-linear, temperature-dependent                     │
│                                                          │
│  Method 3: Extended Kalman Filter (EKF)                  │
│    State vector: [SOC, V_polarization]                   │
│    States: 2 = phi (minimal useful EKF)                  │
│    Or: [SOC, V_p, R_series] -> 3 = n/phi                │
│    Industry standard for automotive BMS                  │
│                                                          │
│  Method 4: Neural Network / ML                           │
│    Training: historical cycle data                       │
│    Input features: V, I, T, dV/dt                        │
│    Features: 4 = tau                                     │
│    Emerging approach, requires training data              │
│                                                          │
│  SOC methods: 4 = tau(6)                                 │
│  Grade: CLOSE (4 is reasonable but methods can be        │
│  subdivided further — e.g., UKF, PF as separate)        │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**EKF 상태 공간 모델**

```
  ┌───────────────────────────────────────────────────────┐
  │  EQUIVALENT CIRCUIT MODEL + EKF                        │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  1RC Model (가장 보편적):                              │
  │                                                       │
  │   V_oc(SOC) ──[R_s]──┬──[R_1]──┬── V_terminal       │
  │                       │         │                     │
  │                      [C_1]      │                     │
  │                       │         │                     │
  │                       └─────────┘                     │
  │                                                       │
  │  State: x = [SOC, V_c1]^T     (φ = 2 states)        │
  │  Input: u = I (current)                               │
  │  Output: y = V_terminal                               │
  │                                                       │
  │  2RC Model (higher fidelity):                         │
  │  State: x = [SOC, V_c1, V_c2]^T  (n/phi = 3 states)│
  │                                                       │
  │  RC pairs in model: 1 or 2                            │
  │  Grade: CLOSE for phi=2 states in 1RC model           │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

**SOH (State of Health) 추정**

```
  ┌───────────────────────────────────────────────────────┐
  │  SOH ESTIMATION                                        │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  SOH = Q_current / Q_initial * 100%                   │
  │                                                       │
  │  Degradation mechanisms:                               │
  │    1. SEI layer growth (capacity fade)                │
  │    2. Li plating (fast charge damage)                 │
  │    3. Active material loss                            │
  │    4. Electrolyte decomposition                       │
  │                                                       │
  │  Mechanisms: 4 = tau                                  │
  │  Grade: CLOSE (degradation can be categorized         │
  │  differently — some sources list 3, some 5+)          │
  │                                                       │
  │  EoL threshold: 80% SOH (industry standard)           │
  │  No clean n=6 match for 80%                           │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 6. Cell Balancing IC

**능동/수동 밸런싱 — 수명의 핵심**

셀 간 용량 불균형은 팩 전체 수명을 제한한다.
가장 약한 셀이 팩 수명을 결정하므로, 밸런싱은 BMS의 핵심 기능이다.

```
┌──────────────────────────────────────────────────────────┐
│  CELL BALANCING ARCHITECTURES                             │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Type 1: PASSIVE Balancing                               │
│    Bleed resistor across each cell                       │
│    Energy wasted as heat                                 │
│    Simple, low cost, ~100mA balance current              │
│                                                          │
│    Cell n ──┬──[R_bal]──┬── GND                         │
│             │           │                                │
│            [SW]  (MOSFET controlled by BMS)              │
│             │                                            │
│             └───────────┘                                │
│                                                          │
│  Type 2: ACTIVE Balancing                                │
│    Charge shuttle between cells                          │
│    >90% energy recovery                                  │
│    Complex, higher cost                                  │
│                                                          │
│    Cell_high ──→ [Energy Transfer] ──→ Cell_low          │
│                  (capacitor / inductor / transformer)     │
│                                                          │
│  Modes: phi = 2 (passive / active)                       │
│  Grade: EXACT (definitional -- only 2 possible modes)    │
│  Caveat: phi=2 for any binary choice is trivially true   │
│                                                          │
│  Active sub-types:                                       │
│    1. Capacitor shuttle (switched capacitor)             │
│    2. Inductor-based (flyback, buck-boost)               │
│    3. Transformer-based (multi-winding)                  │
│    = n/phi = 3 active sub-types                          │
│    Grade: CLOSE (reasonable grouping but not canonical)   │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**밸런싱 전류 비교**

```
  ┌───────────────────────────────────────────────────────┐
  │  BALANCING CURRENT COMPARISON                          │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Passive:   50~200 mA (typical)                       │
  │  Active:    1~5 A (high-performance)                  │
  │                                                       │
  │  Ratio: active/passive ~ 10~25x                       │
  │  σ-φ = 10x or J₂ = 24x (?)                          │
  │  Grade: WEAK (ratio varies widely by implementation)  │
  │                                                       │
  │  Time to balance 100mAh mismatch:                     │
  │  Passive (100mA): 1 hour                              │
  │  Active (2A):     3 minutes                           │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 7. PMIC Architecture

**DC-DC 변환, 충전 IC — 전력 관리의 심장**

```
┌──────────────────────────────────────────────────────────┐
│  PMIC: Power Management IC                                │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  DC-DC Buck Converter:                                   │
│    48V -> 12V: ratio = tau = 4:1                         │
│    12V -> 3.3V: ratio ~ n/phi = 3.6:1                   │
│    48V -> 1.2V: ratio = tau*(sigma-phi) = 40:1           │
│                                                          │
│  48V -> 12V is the key EV/datacenter conversion:         │
│    EV: 48V mild hybrid -> 12V accessory bus              │
│    Datacenter: 48V rack -> 12V server rail               │
│    Telecom: 48V standard -> 12V equipment                │
│                                                          │
│  This ratio = tau = 4:1 is an EXACT match because       │
│  48V and 12V are real industry standards (BT-60)         │
│                                                          │
│  Grade: EXACT                                            │
│                                                          │
│  ┌────────────────────────────────────────────────┐     │
│  │  48V ──→ [Buck 4:1] ──→ 12V ──→ [Buck 3.6:1] │     │
│  │                          │        ──→ 3.3V     │     │
│  │                          │                      │     │
│  │                          └──→ [LDO] ──→ 1.8V   │     │
│  │                                                 │     │
│  │  DC Power Chain (BT-60):                        │     │
│  │  480V AC → 48V DC → 12V → 3.3V → 1.2V → 1.0V  │     │
│  │           τ:1    τ:1   ~τ:1   ~φ+:1            │     │
│  └────────────────────────────────────────────────┘     │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**충전 IC (Charger IC)**

```
  ┌───────────────────────────────────────────────────────┐
  │  CHARGER IC: CC/CV PROFILE                             │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Phase 1: CC (Constant Current)                       │
  │    Bulk charging at maximum safe rate                  │
  │    Typically charges to ~80% SOC                       │
  │                                                       │
  │  Phase 2: CV (Constant Voltage)                       │
  │    Taper current at V_max (4.2V per cell)             │
  │    Terminates at I < C/20~C/10                        │
  │                                                       │
  │  Phases: phi = 2 (CC, CV)                              │
  │  Grade: WEAK (any 2-phase process maps to phi=2)       │
  │                                                       │
  │  Charging curve:                                       │
  │                                                       │
  │  Current  ^      CC phase                              │
  │  (I)      │  ─────────────┐                           │
  │           │               │  CV phase                  │
  │           │               └──────────────→             │
  │           └──────────────────────────────→ Time        │
  │                                                       │
  │  Voltage  ^                                            │
  │  (V)      │               ┌──────────────→ 4.2V       │
  │           │              /                             │
  │           │             /                              │
  │           │  ──────────/                               │
  │           └──────────────────────────────→ Time        │
  │                                                       │
  │  Fast charging (USB PD, QC):                           │
  │    5V / 9V / 12V / 20V                                │
  │    Voltage levels: 4 = tau (!)                         │
  │    Grade: CLOSE (USB PD also has 15V, 28V, 48V)       │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

**PMIC 효율**

```
  ┌───────────────────────────────────────────────────────┐
  │  PMIC EFFICIENCY ANALYSIS                              │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Target: >95% at full load                            │
  │  1 - 1/(J_2 - tau) = 1 - 1/20 = 95%                  │
  │                                                       │
  │  Real-world efficiency:                                │
  │    Buck converter:  92~97% (load-dependent)           │
  │    LDO:             V_out/V_in (can be <50%)          │
  │    Boost:           88~95%                            │
  │                                                       │
  │  95% is a common engineering target, not specific     │
  │  to any n=6 derivation. Most power electronics aim    │
  │  for "above 90%," and 95% is a typical spec sheet     │
  │  number.                                               │
  │                                                       │
  │  Grade: WEAK (95% is standard efficiency target)       │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 8. Protection IC

**과충전/과방전/과전류/온도 보호 — 안전의 마지막 방어선**

Protection IC는 BMS MCU와 독립적으로 동작한다.
소프트웨어가 실패해도 하드웨어 보호가 최후의 안전장치 역할을 한다.

```
┌──────────────────────────────────────────────────────────┐
│  tau = 4 PROTECTION THRESHOLDS                            │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  1. OVP (Over-Voltage Protection)                        │
│     Threshold: 4.25-4.35V per cell (Li-ion)             │
│     Disconnects charge path via high-side FET            │
│     Prevents thermal runaway from overcharge             │
│                                                          │
│  2. UVP (Under-Voltage Protection)                       │
│     Threshold: 2.5-3.0V per cell                        │
│     Disconnects discharge path                           │
│     Prevents copper dissolution and irreversible damage  │
│                                                          │
│  3. OCP (Over-Current Protection)                        │
│     Threshold: varies by cell rating (1C~10C)           │
│     Current-sense resistor + comparator                  │
│     Both charge and discharge direction                  │
│                                                          │
│  4. OTP (Over-Temperature Protection)                    │
│     Threshold: 60-80C (charge), 80-100C (discharge)     │
│     NTC thermistor monitoring                            │
│     Reduces current or disconnects path                  │
│                                                          │
│  Protection types: 4 = tau(6)                            │
│  Grade: CLOSE -- some ICs add SCP (short-circuit) as    │
│  5th, and cell unbalance detection as 6th               │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**Protection IC 회로 구조**

```
  ┌───────────────────────────────────────────────────────┐
  │  PROTECTION IC TOPOLOGY                                │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Pack+ ──┬──[Charge FET]──┬──[Discharge FET]──┬── Load│
  │          │                │                   │       │
  │          │           ┌────┴────┐              │       │
  │  Cells   │           │ Protect │              │       │
  │  Stack ──┤           │   IC    │──── R_sense ─┘       │
  │          │           │ (τ=4    │                       │
  │          │           │ checks) │                       │
  │  Pack- ──┘           └─────────┘                       │
  │                                                       │
  │  FET pair: phi = 2 (charge FET + discharge FET)       │
  │  Back-to-back MOSFET topology                         │
  │                                                       │
  │  Protection response time:                             │
  │    OVP/UVP: ~100ms (voltage slowly changes)           │
  │    OCP: ~10-100us (must react fast)                   │
  │    SCP: ~1-10us (near-instantaneous)                  │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

**Industry Protection ICs**

```
  ┌───────────────────────────────────────────────────────┐
  │  PROTECTION IC EXAMPLES                                │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  1-cell:  TI BQ29700 (OV, UV, OC, SCD)               │
  │  2-cell:  Seiko S-8261 series                         │
  │  3-4S:    TI BQ77915 (4 cells)                        │
  │  5-10S:   TI BQ76920 (5-10 cells)                    │
  │  10-15S:  TI BQ76940 (9-15 cells)                    │
  │                                                       │
  │  Cell count ranges map loosely to n=6 constants:      │
  │    1, 2~4, 5~10, 9~15                                │
  │    Grade: FAIL (ranges overlap, no clean mapping)      │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 9. Communication Interface

**CAN/SPI/I2C/SMBus — BMS 통신 체계**

```
┌──────────────────────────────────────────────────────────┐
│  COMMUNICATION INTERFACES                                 │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Primary BMS buses:                                      │
│    1. CAN (Controller Area Network) -- automotive        │
│    2. SPI (Serial Peripheral Interface) -- IC-to-IC      │
│    3. I2C/SMBus -- low-speed monitoring                  │
│    Bus types: n/phi = 3                                  │
│    Grade: CLOSE (some add UART, LIN as 4th/5th)         │
│                                                          │
│  ┌──────────────────────────────────────────────┐       │
│  │                   ECU / Host                  │       │
│  │                     │                         │       │
│  │            ┌────────┼────────┐                │       │
│  │            │        │        │                │       │
│  │         [CAN]    [SPI]    [I2C]               │       │
│  │            │        │        │                │       │
│  │  ┌────────┴──┐  ┌──┴──┐  ┌──┴──────┐        │       │
│  │  │BMS Master │  │AFE  │  │Temp/Fuel│        │       │
│  │  │(vehicle)  │  │ IC  │  │ Gauge   │        │       │
│  │  └───────────┘  └─────┘  └─────────┘        │       │
│  └──────────────────────────────────────────────┘       │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**CAN Bus 상세**

```
  ┌───────────────────────────────────────────────────────┐
  │  CAN BUS FOR BMS                                       │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Standard CAN:                                        │
  │    Baud: 500 kbps or 1 Mbps                           │
  │    Message ID: 11-bit (standard)                      │
  │    Data: 8 bytes = sigma-tau per frame                 │
  │    Grade: CLOSE (8 bytes is CAN standard)              │
  │                                                       │
  │  CAN FD (Flexible Data-rate):                          │
  │    Data rate: up to 8 Mbps = sigma-tau Mbps            │
  │    Payload: up to 64 bytes = 2^n                      │
  │    Grade: WEAK (8 Mbps CAN FD is interesting but      │
  │    may be coincidental)                                │
  │                                                       │
  │  BMS CAN message structure (typical):                  │
  │    Cell voltages: 12 cells * 2 bytes = 24 bytes       │
  │    = J_2 bytes per voltage report                     │
  │    Grade: CLOSE (depends on encoding scheme)           │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

**isoSPI (ADI daisy chain)**

```
  ┌───────────────────────────────────────────────────────┐
  │  isoSPI DAISY CHAIN                                    │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  ADI ADBMS6830 daisy chain topology:                  │
  │                                                       │
  │  MCU ──[SPI]──→ IC1 ──[isoSPI]──→ IC2 ──→ ... ──→ ICn│
  │                 18ch              18ch            18ch│
  │                                                       │
  │  Max chain length: vendor says "virtually unlimited"  │
  │  Practical limit: ~12 ICs due to latency              │
  │  = sigma ICs in daisy chain                           │
  │  Grade: CLOSE (practical limit, not specification)     │
  │                                                       │
  │  Total cells: 12 ICs * 18 cells = 216 cells          │
  │  or 12 ICs * 12 cells = 144 = sigma^2                │
  │  Grade: WEAK (depends on which IC)                     │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 10. BMS-to-Cloud

**IoT 연결, 디지털 트윈 — BMS의 미래**

```
┌──────────────────────────────────────────────────────────┐
│  BMS-TO-CLOUD ARCHITECTURE                                │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─────────┐   ┌──────────┐   ┌──────────┐   ┌───────┐ │
│  │  BMS    │   │ Gateway  │   │  Cloud   │   │ Twin  │ │
│  │  SoC    │──→│ (MCU +   │──→│ Platform │──→│Digital│ │
│  │         │   │  WiFi/   │   │ (AWS/    │   │Model  │ │
│  │ CAN/SPI │   │  LTE)    │   │  Azure)  │   │       │ │
│  └─────────┘   └──────────┘   └──────────┘   └───────┘ │
│                                                          │
│  Data flow layers:                                       │
│    1. Physical (BMS IC measurements)                    │
│    2. Protocol (CAN/SPI -> MQTT/HTTP)                   │
│    3. Transport (WiFi/LTE/5G)                           │
│    4. Application (cloud analytics)                     │
│                                                          │
│  Layers: 4 = tau                                        │
│  Grade: WEAK (standard networking layer model)           │
│                                                          │
│  Digital Twin enables:                                    │
│    - Remote SOC/SOH monitoring                           │
│    - Predictive maintenance                              │
│    - Fleet-level battery analytics                       │
│    - OTA parameter updates                               │
│                                                          │
│  Update parameters (OTA):                                │
│    1. SOC calibration tables                             │
│    2. Protection thresholds                              │
│    3. Balancing algorithm parameters                     │
│    4. Charging profiles                                  │
│    OTA categories: 4 = tau                              │
│    Grade: WEAK                                           │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**Data Telemetry**

```
  ┌───────────────────────────────────────────────────────┐
  │  BMS TELEMETRY DATA POINTS                             │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Per-cell data:                                       │
  │    - Voltage (12 cells = sigma per IC)                │
  │    - Temperature (2-4 NTC per module)                 │
  │                                                       │
  │  Pack-level data:                                     │
  │    - Total voltage                                    │
  │    - Total current (charge/discharge)                 │
  │    - SOC, SOH, SOP estimates                          │
  │    - Ambient temperature                              │
  │    - Fault codes                                      │
  │                                                       │
  │  Sampling rate:                                        │
  │    Cell voltage: 10~100 Hz                            │
  │    Current: 100~1000 Hz                               │
  │    Temperature: 1~10 Hz                               │
  │    Cloud upload: 0.1~1 Hz (aggregated)                │
  │                                                       │
  │  No clean n=6 match for sampling rates                │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 11. Honesty Assessment

**정직한 평가 — n=6 연결의 강도 분석**

```
┌──────────────────────────────────────────────────────────┐
│  HONESTY ASSESSMENT                                       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  EXACT (3):                                              │
│    - 12-bit ADC = sigma (industry standard BMS)          │
│    - 48V->12V = tau:1 (real industry standard, BT-60)   │
│    - NXP MC33772C = 6 cells = n                          │
│                                                          │
│  CLOSE (5):                                              │
│    - 12 channels per AFE IC = sigma (common, not all)    │
│    - 4 protection types = tau (some ICs have 5-6)        │
│    - 3 bus types = n/phi (CAN/SPI/I2C common trio)       │
│    - 3 active balancing sub-types = n/phi                │
│    - 4 SOC methods = tau (can subdivide further)         │
│                                                          │
│  WEAK (4):                                               │
│    - 95% efficiency ~ 1-1/20 (common target)            │
│    - CAN FD 8 Mbps = sigma-tau (coincidental?)          │
│    - phi=2 balancing modes (trivial binary)              │
│    - phi=2 CC/CV phases (trivial binary)                 │
│                                                          │
│  FAIL:                                                   │
│    - Switching frequencies (no match)                    │
│    - Specific voltage thresholds (cell chemistry)        │
│    - Protection IC cell count ranges (overlap)           │
│    - Sampling rates (no match)                           │
│                                                          │
│  Score: 3/12 EXACT (25%), 5/12 CLOSE (42%)              │
│                                                          │
╚══════════════════════════════════════════════════════════╝
```

**Critical self-assessment:**

```
  ┌───────────────────────────────────────────────────────┐
  │  SELF-CRITICISM                                        │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  1. 12-bit ADC is UNIVERSAL, not BMS-specific         │
  │     Every MCU, every sensor IC uses 12-bit ADCs.      │
  │     Claiming this as "n=6 in BMS" is misleading.      │
  │     It is sigma=12, but sigma=12 appears everywhere.  │
  │                                                       │
  │  2. Binary choices (phi=2) are trivially true          │
  │     Passive/active, CC/CV, charge/discharge FET --    │
  │     ANY binary classification maps to phi=2.           │
  │     This carries zero predictive power.                │
  │                                                       │
  │  3. 48V->12V is a genuine BT-60 connection            │
  │     The DC power chain 480->48->12->1.2V is a real    │
  │     industry standard, and tau=4:1 ratio is exact.    │
  │     This is the STRONGEST connection in this document. │
  │                                                       │
  │  4. 12-channel AFE is CLOSE but not universal          │
  │     ADI uses 18ch, NXP uses 6ch, TI varies 3-16.     │
  │     12 is common but not dominant enough for EXACT.    │
  │                                                       │
  │  5. Protection thresholds tau=4 is reasonable           │
  │     OV/UV/OC/OT is a standard set, but short-circuit │
  │     protection (SCP) is often a 5th, making it 5.     │
  │                                                       │
  │  OVERALL: BMS semiconductor design shows moderate     │
  │  n=6 alignment (25% EXACT). The strongest connection  │
  │  is the DC voltage chain (BT-60), not BMS-specific    │
  │  parameters. BMS is an application domain where n=6   │
  │  constants like sigma=12 happen to coincide with      │
  │  standard engineering choices.                         │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 12. Predictions & Falsifiability

**검증 가능한 예측**

```
┌──────────────────────────────────────────────────────────┐
│  FALSIFIABLE PREDICTIONS                                  │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  P1: Next-gen BMS ADC will remain 12-bit or go to       │
│      16-bit (sigma or sigma+tau), NOT 14-bit             │
│      Falsifiable: if 14-bit BMS ADC becomes standard    │
│      Confidence: LOW (14-bit exists in some premium ICs) │
│                                                          │
│  P2: 48V->12V conversion will remain the dominant       │
│      DC-DC step in automotive/datacenter (tau:1)         │
│      Falsifiable: if 48V->5V direct replaces it         │
│      Confidence: HIGH (massive existing infrastructure)  │
│                                                          │
│  P3: Wireless BMS will use 3 = n/phi protocol types     │
│      (BLE, proprietary RF, UWB)                          │
│      Falsifiable: if only 2 or 4+ protocols emerge      │
│      Confidence: LOW (market still nascent)              │
│                                                          │
│  P4: 800V EV packs will standardize on 96S = sigma*     │
│      (sigma-tau) series cells (BT-57)                    │
│      Falsifiable: if 100S or 108S becomes standard      │
│      Confidence: MEDIUM (96S already common in 400V)     │
│                                                          │
│  P5: AI-BMS will use 4 = tau input features as core     │
│      (V, I, T, dV/dt) even as models grow               │
│      Falsifiable: if frequency-domain features dominate  │
│      Confidence: MEDIUM (these 4 are physically based)   │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**예측 정직도 요약**

```
  ┌───────────────────────────────────────────────────────┐
  │  PREDICTION CONFIDENCE SUMMARY                         │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  HIGH confidence:   1/5 predictions (P2)              │
  │  MEDIUM confidence: 2/5 predictions (P4, P5)          │
  │  LOW confidence:    2/5 predictions (P1, P3)          │
  │                                                       │
  │  Most predictions rely on existing industry trends    │
  │  rather than n=6 derivation. P2 (48V->12V) is the    │
  │  most robust because it is anchored in BT-60.        │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 13. Future Directions

**AI-BMS, wireless BMS, quantum sensing**

```
┌──────────────────────────────────────────────────────────┐
│  FUTURE DIRECTIONS                                        │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Direction 1: AI-BMS (Neural Network on-chip)            │
│  ┌──────────────────────────────────────────────┐       │
│  │  Current: EKF running on Cortex-M4           │       │
│  │  Future:  TinyML inference on BMS SoC        │       │
│  │                                              │       │
│  │  ┌──────┐  ┌───────────┐  ┌──────────┐     │       │
│  │  │ AFE  │→ │ TinyML    │→ │ SOC/SOH  │     │       │
│  │  │12-bit│  │ Inference │  │ Estimate │     │       │
│  │  └──────┘  │ (8-bit    │  └──────────┘     │       │
│  │            │  quantized)│                    │       │
│  │            └───────────┘                    │       │
│  └──────────────────────────────────────────────┘       │
│                                                          │
│  Cross-ref: HEXA-1 SoC NPU (chip-architecture)          │
│  8-bit quantization = sigma-tau bits (BT-58)             │
│                                                          │
│  Direction 2: Wireless BMS (wBMS)                        │
│  ┌──────────────────────────────────────────────┐       │
│  │  Eliminate wiring harness inside battery pack │       │
│  │                                              │       │
│  │  Cell ──[AFE+Radio]──~ ~ ~──[Central BMS]   │       │
│  │                                              │       │
│  │  Benefits:                                    │       │
│  │    - Weight reduction (Cu wiring eliminated)  │       │
│  │    - Simpler assembly                         │       │
│  │    - Improved reliability (no connectors)     │       │
│  │                                              │       │
│  │  TI has CC2662R-Q1 wBMS solution              │       │
│  │  Uses 2.4 GHz (not n=6 related)              │       │
│  └──────────────────────────────────────────────┘       │
│                                                          │
│  Direction 3: Quantum Sensing for SOC                    │
│  ┌──────────────────────────────────────────────┐       │
│  │  NV-center diamond magnetometry for           │       │
│  │  non-invasive current sensing                 │       │
│  │                                              │       │
│  │  Resolution: ~1 nT (vs ~1 uT for Hall)      │       │
│  │  1000x improvement in current accuracy        │       │
│  │                                              │       │
│  │  This is speculative (>2030 timeline)        │       │
│  └──────────────────────────────────────────────┘       │
│                                                          │
│  Direction 4: SiC/GaN PMIC for High-Voltage BMS         │
│  ┌──────────────────────────────────────────────┐       │
│  │  800V EV packs need high-voltage management   │       │
│  │  SiC MOSFET: 1200V rated (wide-bandgap)      │       │
│  │  GaN HEMT: 650V rated, higher switching freq  │       │
│  │                                              │       │
│  │  SiC bandgap: 3.26 eV ~ n/phi = 3 (CLOSE)  │       │
│  │  GaN bandgap: 3.4 eV ~ n/phi = 3 (CLOSE)   │       │
│  │  Cross-ref: BT-30 (SQ solar bridge)          │       │
│  └──────────────────────────────────────────────┘       │
│                                                          │
│  Future directions: 4 = tau                              │
│  Grade: WEAK (any list can be made to have 4 items)      │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 14. n=6 Complete Parameter Map

| # | Parameter | Value | n=6 Expression | Grade | Notes |
|---|-----------|-------|----------------|-------|-------|
| 1 | AFE channels | 12 | sigma | CLOSE | Common but 6/16/18 also exist |
| 2 | ADC resolution | 12-bit | sigma | EXACT | Universal standard, not BMS-specific |
| 3 | Protection types | 4 | tau | CLOSE | OV/UV/OC/OT; some ICs add SCP as 5th |
| 4 | Balancing modes | 2 | phi | WEAK | Trivially binary (passive/active) |
| 5 | Active balancing sub-types | 3 | n/phi | CLOSE | Cap/inductor/transformer grouping |
| 6 | CC/CV phases | 2 | phi | WEAK | Any 2-phase process |
| 7 | Bus types | 3 | n/phi | CLOSE | CAN/SPI/I2C trio |
| 8 | DC-DC 48V->12V ratio | 4:1 | tau | EXACT | Real industry standard (BT-60) |
| 9 | SOC estimation methods | 4 | tau | CLOSE | Can be subdivided further |
| 10 | CAN FD data rate | 8 Mbps | sigma-tau | WEAK | Possibly coincidental |
| 11 | isoSPI chain (practical) | 12 ICs | sigma | CLOSE | Practical limit, not spec |
| 12 | NXP MC33772C cells | 6 | n | EXACT | One specific IC among many |
| -- | **TOTAL EXACT** | **3/12** | **(25%)** | | |
| -- | **TOTAL CLOSE** | **5/12** | **(42%)** | | |
| -- | **TOTAL WEAK** | **4/12** | **(33%)** | | |

```
┌──────────────────────────────────────────────────────────┐
│  PARAMETER MAP VISUALIZATION                              │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  EXACT  [###---------]  25%  (3/12)                     │
│  CLOSE  [#####-------]  42%  (5/12)                     │
│  WEAK   [####--------]  33%  (4/12)                     │
│  FAIL   [            ]   0%  (0/12 in map; noted above) │
│                                                          │
│  Comparison with other domains:                          │
│    AI/LLM (BT-56): ~80% EXACT                           │
│    Chip arch (BT-28): ~70% EXACT                         │
│    Battery cell (BT-43): ~60% EXACT                      │
│    BMS chip (this doc): 25% EXACT                        │
│                                                          │
│  BMS semiconductor is among the WEAKER n=6 domains.     │
│  This is expected -- BMS is an application-specific      │
│  analog/mixed-signal domain where the key numbers       │
│  (voltage thresholds, frequencies) are set by physics   │
│  and safety standards, not information-theoretic         │
│  constants.                                              │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 15. Open Questions / TODO

```
┌──────────────────────────────────────────────────────────┐
│  OPEN QUESTIONS                                           │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Q1: Does the 12-channel AFE standard emerge from        │
│      12S battery modules, or independently? If 12S       │
│      modules (sigma cells in series) drove 12-channel   │
│      AFE design, there is a genuine structural reason.   │
│                                                          │
│  Q2: Will the 48V DC bus (BT-60) extend to residential  │
│      battery storage? (Currently 48V is commercial/EV)  │
│                                                          │
│  Q3: Can AI-BMS on-chip (TinyML) outperform EKF with    │
│      the same tau=4 input features? This would validate │
│      that V, I, T, dV/dt are sufficient.                 │
│                                                          │
│  Q4: What is the optimal BMS IC process node?            │
│      28nm (P_2 from BT-37) seems plausible for          │
│      next-gen integrated BMS SoC.                        │
│                                                          │
│  Q5: Does wireless BMS adoption follow the               │
│      phi=2 year doubling pattern seen in other domains?  │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**TODO Items**

```
  ┌───────────────────────────────────────────────────────┐
  │  TODO                                                  │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  [ ] Survey all TI BQ-series BMS ICs for channel      │
  │      count distribution (is 12 truly dominant?)        │
  │                                                       │
  │  [ ] Cross-verify 48V->12V conversion ratio with      │
  │      BT-60 DC power chain data                         │
  │                                                       │
  │  [ ] Investigate whether 96S = sigma*(sigma-tau)       │
  │      is becoming standard for 800V EV (BT-57)         │
  │                                                       │
  │  [ ] Prototype TinyML SOC estimator and compare       │
  │      with EKF baseline (tau=4 features)                │
  │                                                       │
  │  [ ] Analyze whether SiC/GaN bandgap ~ n/phi = 3      │
  │      is coincidental or structurally related to BT-30  │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

## 16. Links

**Internal**

```
  ┌───────────────────────────────────────────────────────┐
  │  CROSS-REFERENCES                                      │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  Battery Architecture Series:                         │
  │    Level 1: hexa-cell.md (crystal chemistry)          │
  │    Level 2: hexa-electrode.md (electrode design)      │
  │    Level 3: hexa-pack.md (pack system)                │
  │    Level 4: hexa-grid.md (grid integration)           │
  │    This:    hexa-chip.md (BMS/PMIC semiconductor)     │
  │                                                       │
  │  Chip Architecture Cross-refs:                        │
  │    ../chip-architecture/ultimate-unified-soc.md       │
  │    ../chip-architecture/hexa-core.md                  │
  │    ../chip-architecture/eda-physical-design-n6.md     │
  │                                                       │
  │  Breakthrough Theorems:                               │
  │    BT-28: Computing architecture ladder               │
  │    BT-33: Transformer sigma=12 atom                   │
  │    BT-57: Battery cell ladder (6->12->24 cells)       │
  │    BT-59: 8-layer AI stack                            │
  │    BT-60: DC power chain (120->48->12->1.2->1V)      │
  │                                                       │
  │  Energy Domain:                                       │
  │    ../battery-storage/hypotheses.md                   │
  │    ../energy-generation/hypotheses.md                 │
  │    ../power-grid/hypotheses.md                        │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

**External References**

```
  ┌───────────────────────────────────────────────────────┐
  │  EXTERNAL REFERENCES                                   │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │  BMS IC Datasheets:                                   │
  │    TI BQ76952: ti.com/product/BQ76952                 │
  │    ADI ADBMS6830: analog.com/ADBMS6830                │
  │    NXP MC33772C: nxp.com/MC33772C                     │
  │    Maxim MAX17853: maximintegrated.com/MAX17853        │
  │                                                       │
  │  PMIC References:                                     │
  │    TI TPS546B24A: 48V->12V buck (tau:1)               │
  │    Infineon XDP (digital power)                       │
  │    TI BQ25790: USB PD charger IC                      │
  │                                                       │
  │  Standards:                                           │
  │    ISO 26262 (automotive functional safety)           │
  │    SAE J2954 (wireless charging)                      │
  │    IEC 62619 (industrial battery safety)              │
  │                                                       │
  └───────────────────────────────────────────────────────┘
```

---

**Document Statistics:**

```
  Sections: 16
  EXACT parameters: 3/12 (25%)
  CLOSE parameters: 5/12 (42%)
  WEAK parameters: 4/12 (33%)
  Predictions: 5 (1 HIGH, 2 MEDIUM, 2 LOW confidence)
  Strongest connection: 48V->12V = tau:1 (BT-60)
  Weakest claims: phi=2 binary choices (trivially true)
  ASCII diagrams: 25+
  Honesty level: HIGH (explicit self-criticism included)
```
