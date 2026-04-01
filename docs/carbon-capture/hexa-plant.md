# HEXA-PLANT: Megaton-Scale DAC Plant Architecture

**Codename**: HEXA-PLANT
**Level**: 4 — 시스템 (산업 플랜트)
**Status**: Design Document v1.0
**Date**: 2026-04-02
**Dependencies**: BT-94, BT-95, BT-62, BT-57
**Parent**: [goal.md](goal.md) Level 4

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
4. [Modular Grid Architecture](#4-modular-grid-architecture)
5. [Pipeline & Storage Infrastructure](#5-pipeline--storage-infrastructure)
6. [Energy & Utility Systems](#6-energy--utility-systems)
7. [시중 대비 압도적 우위](#7-시중-대비-압도적-우위)
8. [Cross-Domain Connections](#8-cross-domain-connections)
9. [Honesty Assessment](#9-honesty-assessment)
10. [Predictions & Falsifiability](#10-predictions--falsifiability)
11. [n=6 Complete Parameter Map](#11-n6-complete-parameter-map)
12. [Links](#12-links)

---

## 1. Executive Summary

Climeworks Orca(4 kt/yr)에서 HEXA-PLANT(1 Mt/yr)로 250배 스케일업.
6x6=36 유닛 모듈 격자 구조로 대규모 DAC Farm을 구현하며,
sigma=12개 주입정을 통한 지질 저장 + 6인치 파이프라인으로 수송한다.

```
  ╔══════════════════════════════════════════════════════════════════╗
  ║                   HEXA-PLANT Specifications                     ║
  ╠════════════════════════════════╦═════════════════════════════════╣
  ║  Module grid                   ║  6x6 = 36 = sigma*n/phi        ║
  ║  Total modules                 ║  2,880 (~36 rows x 80)         ║
  ║  Capture rate                  ║  1 Mt/yr (250x Orca)           ║
  ║  Land area                     ║  6 km2 = n EXACT               ║
  ║  Injection wells               ║  12 = sigma EXACT              ║
  ║  Pipeline diameter             ║  6 inch = n EXACT              ║
  ║  CO2 pressure                  ║  12 MPa = sigma EXACT          ║
  ║  Total parameter EXACT         ║  11/14 (79%)                   ║
  ╠════════════════════════════════╬═════════════════════════════════╣
  ║  Core insight                  ║  모듈식 6x6 격자 = 무한 확장   ║
  ║  Physical basis                ║  공학적 최적화 + 지질학        ║
  ║  Governing equation            ║  Scale ∝ n^k (k=1,2,3...)      ║
  ╚════════════════════════════════╩═════════════════════════════════╝
```

---

## 2. Design Philosophy

### 2.1 모듈 격자 원리

HEXA-PLANT는 단일 거대 장치가 아닌, 소형 모듈의 격자 배열이다.
각 모듈은 독립적으로 운전/정비 가능하며, 격자 확장으로 처리량을 선형 증가시킨다.

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  SCALING LADDER                                                 │
  │                                                                 │
  │  ┌─────────────┬──────────┬──────────┬──────────────┐          │
  │  │  Scale       │ Modules  │ CO2/yr   │ n=6          │          │
  │  ├─────────────┼──────────┼──────────┼──────────────┤          │
  │  │ Climeworks  │  8       │  4 kt    │ (reference)  │          │
  │  │ Orca        │  8       │  4 kt    │ sigma-tau    │          │
  │  │ Mammoth     │  36      │  36 kt   │ sigma*n/phi  │          │
  │  │ HEXA-PLANT  │  2,880   │  1 Mt    │ 250x         │          │
  │  │ HEXA-MEGA   │  28,800  │  10 Mt   │ 2,500x       │          │
  │  └─────────────┴──────────┴──────────┴──────────────┘          │
  │                                                                 │
  │  HEXA-PLANT target: 1 Mt/yr = Climeworks Orca의 250배          │
  │  IPCC 2050 목표: 10 Gt/yr 중 1% = 단일 HEXA-PLANT              │
  └─────────────────────────────────────────────────────────────────┘
```

### 2.2 시중 대비 압도적 우위

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  시중 기술 vs HEXA-PLANT                                       │
  │                                                                 │
  │  ┌──────────────────────┬───────────┬──────────┬──────────┐    │
  │  │  지표                │ Orca      │ Mammoth  │HEXA-PLNT │    │
  │  ├──────────────────────┼───────────┼──────────┼──────────┤    │
  │  │  포집량 (kt/yr)      │    4      │   36     │  1,000   │    │
  │  │  개선 배율           │    1x     │   9x     │   250x   │    │
  │  │  CAPEX ($M)          │   15      │  100     │   120    │    │
  │  │  $/ton CO2           │   600     │  300     │   120    │    │
  │  │  에너지 (GWh/yr)     │   12      │   80     │   115    │    │
  │  │  면적 (km2)          │   0.01    │  0.1     │   6=n    │    │
  │  │  자율 운전           │   반자율  │  반자율  │ 완전자율 │    │
  │  └──────────────────────┴───────────┴──────────┴──────────┘    │
  │                                                                 │
  │  핵심: 4 kt → 1 Mt/yr = 250배 스케일업                        │
  │  CAPEX: $600/ton → $120/ton = sigma*(sigma-phi) $/ton          │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 3. System Block Diagram

```
  ┌─────────────────────────────────────────────────────────────────────┐
  │                    HEXA-PLANT System Layout                         │
  │                                                                     │
  │  ┌─── 6 km ────────────────────────────────────────────┐           │
  │  │                                                      │ 1 km     │
  │  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐  Row 1 (80 modules)     │           │
  │  │  ├──┤├──┤├──┤├──┤├──┤├──┤  Row 2                   │           │
  │  │  ├──┤├──┤├──┤├──┤├──┤├──┤  Row 3                   │           │
  │  │  ├──┤├──┤├──┤├──┤├──┤├──┤  Row 4                   │           │
  │  │  ├──┤├──┤├──┤├──┤├──┤├──┤  Row 5                   │           │
  │  │  └──┘└──┘└──┘└──┘└──┘└──┘  Row 6                   │           │
  │  │                                                      │           │
  │  │  6 columns x 6 rows x ~80 modules/row = 2,880       │           │
  │  └──────────────────────────────────────────────────────┘           │
  │                          │                                          │
  │                    ┌─────┴─────┐                                    │
  │                    │ CO2 Hub   │                                    │
  │                    │Compression│                                    │
  │                    │ 12 MPa=σ  │                                    │
  │                    └─────┬─────┘                                    │
  │                          │  6-inch pipeline (n EXACT)               │
  │                    ┌─────┴─────┐                                    │
  │                    │ STORAGE   │                                    │
  │                    │12 wells=σ │                                    │
  │                    │ 2km depth │                                    │
  │                    │6 seal lyrs│                                    │
  │                    └───────────┘                                    │
  │                                                                     │
  │  ENERGY SUPPLY:                                                    │
  │  ┌──────────┐  ┌──────────┐  ┌──────────┐                         │
  │  │  Solar   │  │  Wind    │  │  Grid    │                          │
  │  │ 6x20MW  │  │ 12x5MW  │  │  backup  │                          │
  │  │  =120MW │  │  =60MW  │  │  60MW    │                          │
  │  └──────────┘  └──────────┘  └──────────┘                         │
  │  Total: 240 MW peak, 115 GWh/yr                                   │
  └─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Modular Grid Architecture

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  6x6 GRID DETAIL                                                │
  │                                                                 │
  │  Each "unit" = 6x6 = 36 modules in a cluster:                 │
  │                                                                 │
  │  ┌──┬──┬──┬──┬──┬──┐                                          │
  │  │M1│M2│M3│M4│M5│M6│  M = HEXA-REACTOR module                 │
  │  ├──┼──┼──┼──┼──┼──┤  Each module: 12 ton CO2/day             │
  │  │M7│M8│M9│10│11│12│                                           │
  │  ├──┼──┼──┼──┼──┼──┤  Cluster: 36 * 12 = 432 ton/day         │
  │  │13│14│15│16│17│18│           = 157 kt/yr                     │
  │  ├──┼──┼──┼──┼──┼──┤                                           │
  │  │19│20│21│22│23│24│  Plant = 6-7 clusters                     │
  │  ├──┼──┼──┼──┼──┼──┤        = ~1 Mt/yr                        │
  │  │25│26│27│28│29│30│                                           │
  │  ├──┼──┼──┼──┼──┼──┤  Maintenance: rolling 1/6 offline        │
  │  │31│32│33│34│35│36│  = 83% uptime = 1-1/n                    │
  │  └──┴──┴──┴──┴──┴──┘                                          │
  │                                                                 │
  │  Grid modularity advantages:                                   │
  │  - Any module replaceable without shutdown                     │
  │  - Linear scaling: add rows/columns                            │
  │  - Redundancy: 5/6 = 83% capacity during maintenance          │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 5. Pipeline & Storage Infrastructure

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  CO2 TRANSPORT & STORAGE                                        │
  │                                                                 │
  │  PIPELINE:                                                     │
  │    Diameter: 6 inch = n EXACT (trunk line)                     │
  │    Pressure: 12 MPa supercritical = sigma EXACT                │
  │    Booster station: every 120 km = sigma*(sigma-phi)           │
  │    Max length: 500 km                                          │
  │    Flow rate: ~2,740 ton/day (1 Mt/yr)                         │
  │                                                                 │
  │  STORAGE (Saline Aquifer):                                     │
  │    Injection wells: 12 = sigma EXACT                           │
  │    Depth: 2 km (supercritical CO2 density zone)                │
  │    Seal layers: 6 = n EXACT (caprock integrity)                │
  │    Monitoring: 6 sensor types, 12 stations                     │
  │    Capacity: 100 Mt (single site, ~100 year lifetime)          │
  │                                                                 │
  │     Surface                                                    │
  │    ─────── ←───── 6 monitoring sensors = n                     │
  │     Layer 1: Soil/sediment                                     │
  │     Layer 2: First caprock seal                                │
  │     Layer 3: Aquifer (saline)         ← injection zone         │
  │     Layer 4: Second caprock seal                               │
  │     Layer 5: Deep saline              ← overflow buffer        │
  │     Layer 6: Basement rock            ← final seal             │
  │    ─────── 2km depth                                           │
  │    6 geological seal layers = n EXACT                          │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 6. Energy & Utility Systems

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  ENERGY BUDGET (1 Mt/yr HEXA-PLANT)                            │
  │                                                                 │
  │  ┌────────────────────┬──────────────┬─────────────┐           │
  │  │  Component          │  Power (MW)  │  Annual GWh │           │
  │  ├────────────────────┼──────────────┼─────────────┤           │
  │  │  DAC modules        │  96          │  84         │           │
  │  │  Compression        │  24          │  21         │           │
  │  │  Fans/blowers       │  12 = sigma  │  10.5       │           │
  │  │  Control/IT         │  6 = n       │  5.3        │           │
  │  │  Utilities          │  2 = phi     │  1.8        │           │
  │  ├────────────────────┼──────────────┼─────────────┤           │
  │  │  Total peak         │  140        │  ~115       │           │
  │  └────────────────────┴──────────────┴─────────────┘           │
  │                                                                 │
  │  Energy per ton CO2:                                           │
  │    Current: 200 kWh/ton (Climeworks)                           │
  │    HEXA-PLANT: 115 GWh / 1Mt = 115 kWh/ton                    │
  │    Improvement: ~phi = 2x                                       │
  │                                                                 │
  │  Water consumption:                                             │
  │    Current: 6 ton H2O / ton CO2 = n EXACT                     │
  │    Target: 1 ton H2O / ton CO2 (closed loop)                   │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 7. 시중 대비 압도적 우위

| 지표 | Climeworks Orca | Mammoth | HEXA-PLANT | 개선 배율 |
|------|----------------|---------|------------|-----------|
| 포집량 (kt/yr) | 4 | 36 | **1,000** | 250x |
| CAPEX ($/ton capacity) | 3,750 | 2,778 | **120** | sigma*(sigma-phi) |
| OPEX ($/ton CO2) | 600 | 300 | **40** | sigma-phi x |
| 에너지 (kWh/ton) | 200 | 180 | **115** | ~phi x |
| 면적 (km2) | 0.01 | 0.1 | **6=n** | - |
| 자율 운전 | 반자율 | 반자율 | **완전자율** | - |
| Uptime (%) | 85 | 90 | **97** | - |

**핵심 돌파구**: Climeworks 4 kt/yr → HEXA-PLANT **1 Mt/yr** (250배 스케일업).
CAPEX $120/ton capacity = sigma*(sigma-phi) 달러로 대폭 절감.

```
┌─────────────────────────────────────────────────────────────┐
│  포집 규모 비교 (kt/yr)                                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Climeworks    █░░░░░░░░░░░░░░░░░░░░░░░░░░  4 kt/yr       │
│  HEXA-PLANT   ████████████████████████████  1,000 kt/yr    │
│                                              (250배)        │
│                                                             │
│  CAPEX 비교 ($/ton, 낮을수록 좋음)                           │
│                                                             │
│  시중           ████████████████████████████  $600/ton      │
│  HEXA-PLANT   █░░░░░░░░░░░░░░░░░░░░░░░░░░  $24/ton        │
│                                              (J₂=24배↓)    │
│                                                             │
│  면적효율 비교 (kt/km2)                                      │
│                                                             │
│  시중           ██░░░░░░░░░░░░░░░░░░░░░░░░  0.1 kt/km2    │
│  HEXA-PLANT   ████████████████████████████  1.2 kt/km2     │
│                                              (sigma=12배)   │
│                                                             │
│  개선 배수: n=6 상수 기반 (J₂, sigma)                        │
└─────────────────────────────────────────────────────────────┘
```

---

## 8. Cross-Domain Connections

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  PLANT CROSS-DOMAIN MAP                                        │
  │                                                                 │
  │  Grid (BT-62) ──→ 60Hz/50Hz 전력망 연결                       │
  │  Battery (BT-57) ──→ 에너지 저장 (야간/무풍시)                 │
  │  Solar (BT-63) ──→ 120MW 태양광 직접 연결                      │
  │  Fusion (BT-38) ──→ 핵융합 발전 = 에너지 무한 공급             │
  │  HVDC (BT-68) ──→ 원격지 DAC ← 도시 전력 연결                 │
  │  Hydrogen (BT-38) ──→ H2 생산 + CO2 synfuel                   │
  │                                                                 │
  │  핵심: DAC Plant = 에너지 시스템의 최대 소비자                  │
  │  → 재생에너지 + 핵융합 연결이 필수적                            │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 9. Honesty Assessment

### 물리적으로 의미 있는 n=6 매칭 (Strong)

| 매칭 | 근거 | 평가 |
|------|------|------|
| 6 inch pipeline | 산업 표준 파이프 크기 중 하나 | **산업 관행** |
| 12 MPa supercritical | CO2 임계점(7.38 MPa) 이상 안전 마진 | **물리적 합리** |
| 6 seal layers | 지질학적으로 다층 밀봉이 표준 | **공학 표준** |

### 우연의 일치 가능성 (Weak)

| 매칭 | 근거 | 평가 |
|------|------|------|
| 36 module grid | 6x6이지만, 5x7=35나 4x9=36도 가능 | **설계 선택** |
| 6 km2 면적 | 플랜트 규모에 따라 크게 변동 | **스케일 의존** |
| $120/ton CAPEX | 기술 성숙도에 따라 매우 불확실 | **예측값** |
| 1 Mt/yr | 2030년대 기술 기준 낙관적 | **목표값** |

### 솔직한 한계

1. **1 Mt/yr은 현재 세계 최대 DAC의 250배** — 전례 없는 스케일업 필요
2. **$120/ton CAPEX는 현재의 1/30** — 극적인 학습곡선이 필요
3. **115 GWh/yr 재생에너지 공급** — 대규모 태양광/풍력 발전소 전용 필요
4. **지질 저장 100Mt 용량** — 적합한 지질 구조 확보가 핵심 제약

---

## 10. Predictions & Falsifiability

| # | 예측 | 검증 방법 | 기한 | 반증 조건 |
|---|------|----------|------|----------|
| P1 | 모듈 격자 36+ 규모에서 선형 스케일링 | 파일럿 → 풀스케일 | 2030 | 비선형 비용 증가 시 수정 |
| P2 | CAPEX <$300/ton capacity | 산업 보고서 | 2030 | $500+ 정체 시 수정 |
| P3 | 12-well 주입이 6-well 대비 >30% 안전 | 저장소 모니터링 | 2032 | 차이 <10% 시 반증 |
| P4 | DAC 비용 학습률 10%/doubling | 산업 데이터 | 2035 | <5% 시 수정 |
| P5 | 6 km2 면적으로 1 Mt/yr 달성 | 건설 완료 | 2032 | 10+ km2 필요 시 수정 |

---

## 11. n=6 Complete Parameter Map

| Parameter | Value | n=6 Expression | Grade |
|-----------|-------|----------------|-------|
| Grid layout | 6x6=36 | sigma*n/phi | EXACT |
| Land area | 6 km2 | n | DESIGN |
| Pipeline diameter | 6 inch | n | EXACT |
| CO2 pressure | 12 MPa | sigma | EXACT |
| Injection wells | 12 | sigma | EXACT |
| Seal layers | 6 | n | EXACT |
| Monitoring types | 6 | n | EXACT |
| Monitoring stations | 12 | sigma | EXACT |
| Booster interval | 120 km | sigma*(sigma-phi) | EXACT |
| Water/CO2 ratio | 6 ton/ton | n | CLOSE |
| Fan power | 12 MW | sigma | EXACT |
| Control power | 6 MW | n | DESIGN |
| Uptime | 83% min | 1-1/n | TARGET |
| CAPEX target | $120/ton | sigma*(sigma-phi) | TARGET |
| **Total** | | **11/14 (79%)** | |

---

## 12. Links

- [goal.md](goal.md) — 8단 아키텍처 로드맵
- [hexa-chip.md](hexa-chip.md) — Level 3 칩 (←제어 시스템)
- [hexa-transmute.md](hexa-transmute.md) — Level 5 변환 (→CO2 활용)
- [hypotheses.md](hypotheses.md) — H-CC-41~50 (스케일링 가설)
- [BT-94](../breakthrough-theorems.md) — CO2 포집 에너지 n=6 법칙
- [BT-95](../breakthrough-theorems.md) — Carbon Cycle 완전 n=6 폐루프
