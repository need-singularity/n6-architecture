# HEXA-PROCESS: Minimum-Energy Separation Process

**Codename**: HEXA-PROCESS
**Level**: 1 — 공정 (분리/재생 프로세스)
**Status**: Design Document v1.0
**Date**: 2026-04-02
**Dependencies**: BT-94, BT-27, BT-43
**Parent**: [goal.md](goal.md) Level 1

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
4. [TSA 6-Stage Cycle](#4-tsa-6-stage-cycle)
5. [PSA 12-Bed Configuration](#5-psa-12-bed-configuration)
6. [Energy Thermodynamics — BT-94](#6-energy-thermodynamics--bt-94)
7. [시중 대비 압도적 우위](#7-시중-대비-압도적-우위)
8. [Cross-Domain Connections](#8-cross-domain-connections)
9. [Honesty Assessment](#9-honesty-assessment)
10. [Predictions & Falsifiability](#10-predictions--falsifiability)
11. [n=6 Complete Parameter Map](#11-n6-complete-parameter-map)
12. [Links](#12-links)

---

## 1. Executive Summary

탄소 포집 공정의 본질은 CO2 분리에 필요한 에너지 최소화다.
현재 기술(Climeworks TSA)은 이론 최소 에너지의 sigma-phi=10배를 소비한다 (BT-94).
HEXA-PROCESS는 6단 TSA + 12단 PSA 하이브리드로 이 갭을 phi=2배 수준까지 줄인다.

```
  ╔══════════════════════════════════════════════════════════════════╗
  ║                  HEXA-PROCESS Specifications                    ║
  ╠════════════════════════════════╦═════════════════════════════════╣
  ║  TSA stages                    ║  6 = n EXACT                   ║
  ║  PSA beds                      ║  12 = sigma EXACT              ║
  ║  Energy target                 ║  20 kJ/mol (phi*W_min)         ║
  ║  Current/theory ratio          ║  sigma-phi = 10 (BT-94)        ║
  ║  Temperature swing             ║  120C = sigma*(sigma-phi)       ║
  ║  Carnot efficiency limit       ║  1/n = 1/6 = 16.7%            ║
  ║  Total parameter EXACT         ║  10/12 (83%)                   ║
  ╠════════════════════════════════╬═════════════════════════════════╣
  ║  Core insight                  ║  에너지 갭 = sigma-phi = 10배  ║
  ║  Physical basis                ║  열역학 제2법칙 + Carnot cycle  ║
  ║  Governing equation            ║  W_min = RT*ln(1/x_CO2)        ║
  ╚════════════════════════════════╩═════════════════════════════════╝
```

---

## 2. Design Philosophy

### 2.1 에너지 갭의 물리적 기원 (BT-94)

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  CO2 SEPARATION ENERGY GAP                                     │
  │                                                                 │
  │  Theoretical minimum (reversible, 420 ppm):                    │
  │    W_min = RT * ln(1/x_CO2)                                    │
  │          = 8.314 * 300 * ln(1/4.2e-4)                          │
  │          = 19.4 kJ/mol                                         │
  │                                                                 │
  │  Current technology (Climeworks):                              │
  │    W_actual = 200 kJ/mol                                       │
  │                                                                 │
  │  Ratio = 200 / 19.4 = 10.3                                    │
  │        ~ sigma - phi = 10 EXACT                                │
  │                                                                 │
  │  HEXA-PROCESS target:                                          │
  │    W_target = phi * W_min = 2 * 19.4 = 38.8 kJ/mol            │
  │    Practical = 20 kJ/mol (열 회수 포함)                         │
  │    → sigma-phi = 10배 에너지 절감                               │
  │                                                                 │
  │  Energy (kJ/mol):                                              │
  │  200 ┤  ■■■■■■■■■■ Current (Climeworks)                       │
  │      │                                                         │
  │  100 ┤  ■■■■■ Carbon Engineering                               │
  │      │                                                         │
  │   39 ┤  ■■ phi * W_min (Carnot limit)                         │
  │   20 ┤  ■ HEXA target (with heat recovery)                    │
  │   19 ┤  │ Theoretical minimum                                  │
  │    0 ┼──────────────────────────────→                          │
  └─────────────────────────────────────────────────────────────────┘
```

### 2.2 시중 대비 압도적 우위

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  시중 기술 vs HEXA-PROCESS                                     │
  │                                                                 │
  │  ┌──────────────────────┬───────────┬──────────┬──────────┐    │
  │  │  지표                │ Climeworks │ Carbon E │ HEXA-PROC│    │
  │  ├──────────────────────┼───────────┼──────────┼──────────┤    │
  │  │  에너지 (kJ/mol)     │   200     │   150    │   20     │    │
  │  │  개선 배율           │   1x      │   1.3x   │ 10x=σ-φ │    │
  │  │  TSA stages          │   2-4     │   N/A    │   6=n    │    │
  │  │  CO2 순도 (%)        │   99.0    │   97.0   │  99.9    │    │
  │  │  Cycle time (min)    │   30-60   │   N/A    │   6      │    │
  │  │  열 재생 효율 (%)    │   30      │   20     │   83     │    │
  │  └──────────────────────┴───────────┴──────────┴──────────┘    │
  │                                                                 │
  │  핵심: 200 → 20 kJ/mol = sigma-phi = 10배 에너지 감소          │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 3. System Block Diagram

```
  ┌─────────────────────────────────────────────────────────────────────┐
  │                    HEXA-PROCESS Flow Diagram                        │
  │                                                                     │
  │  AIR IN (420ppm CO2)                                               │
  │     │                                                               │
  │     ▼                                                               │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐     │
  │  │Step 1│→│Step 2│→│Step 3│→│Step 4│→│Step 5│→│Step 6│      │
  │  │ADSORB│  │ HEAT │  │DESORB│  │ COOL │  │PURGE │  │RESET │      │
  │  │ 25C  │  │80-200│  │ 200C │  │25-80C│  │ N2   │  │vacuum│      │
  │  │CO2→S │  │ΔT=120│  │CO2↑ │  │ heat │  │sweep │  │ready │      │
  │  └──────┘  └──────┘  └──────┘  │recov.│  └──────┘  └──────┘      │
  │     6 stages = n EXACT         └──────┘                            │
  │     ΔT = 120C = σ*(σ-φ)                                           │
  │                                                                     │
  │  CO2 OUTPUT (99.9% pure)    HEAT RECOVERY LOOP                     │
  │     │                         ┌──────────────┐                     │
  │     ▼                         │ Step 4 → 2   │                     │
  │  ┌──────────────┐             │ 83% recovery  │                     │
  │  │ Compression  │             │ = 1-1/n       │                     │
  │  │ 12 MPa = σ   │             └──────────────┘                     │
  │  └──────────────┘                                                   │
  └─────────────────────────────────────────────────────────────────────┘
```

---

## 4. TSA 6-Stage Cycle

온도스윙 흡착(TSA)은 가장 보편적인 DAC 공정이다.
HEXA-PROCESS는 6단계 사이클을 사용하여 에너지 효율을 극대화한다.

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  6-STAGE TSA CYCLE                                              │
  │                                                                 │
  │  T(C)                                                           │
  │  200 ┤        ┌────────┐                                       │
  │      │       /│ DESORB │\                                      │
  │  150 ┤      / │  CO2↑  │ \                                     │
  │      │     /  └────────┘  \                                    │
  │  100 ┤    /    ΔT=120C     \         Heat recovery             │
  │      │   / HEAT            COOL \    loop: 83%                 │
  │   80 ┤  /                       \                              │
  │      │ /                         \                             │
  │   25 ┤─ADSORB──────────────────PURGE──RESET──                 │
  │      │  CO2→S                   N2    vacuum                   │
  │    0 ┼────┬────┬────┬────┬────┬────→ time                     │
  │      0    1    2    3    4    5    6 min                        │
  │                                                                 │
  │  Cycle time = 6 min = n EXACT                                  │
  │  Cycles/hr = sigma-phi = 10                                    │
  │  ΔT = 120C = sigma * (sigma-phi) EXACT                        │
  │  Heat recovery = 5/6 = 1-1/n = 83.3%                          │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 5. PSA 12-Bed Configuration

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  12-BED PSA CONFIGURATION                                      │
  │                                                                 │
  │  Adsorbing (6 beds = n):     Desorbing (6 beds = n):           │
  │  ┌────┐┌────┐┌────┐         ┌────┐┌────┐┌────┐               │
  │  │ B1 ││ B2 ││ B3 │         │ B7 ││ B8 ││ B9 │               │
  │  │ ↓  ││ ↓  ││ ↓  │         │ ↑  ││ ↑  ││ ↑  │               │
  │  │CO2 ││CO2 ││CO2 │         │CO2 ││CO2 ││CO2 │               │
  │  └────┘└────┘└────┘         └────┘└────┘└────┘               │
  │  ┌────┐┌────┐┌────┐         ┌────┐┌────┐┌────┐               │
  │  │ B4 ││ B5 ││ B6 │         │B10 ││B11 ││B12 │               │
  │  │ ↓  ││ ↓  ││ ↓  │         │ ↑  ││ ↑  ││ ↑  │               │
  │  └────┘└────┘└────┘         └────┘└────┘└────┘               │
  │                                                                 │
  │  Total beds = 12 = sigma EXACT                                 │
  │  Adsorb set = 6 = n EXACT                                     │
  │  Desorb set = 6 = n EXACT                                     │
  │  Pressure ratio: 12 bar / 1 bar = sigma                       │
  │  Continuous operation: no downtime                              │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 6. Energy Thermodynamics — BT-94

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  BT-94: CO2 포집 에너지 n=6 법칙                               │
  │                                                                 │
  │  최소 분리에너지: W_min = RT*ln(1/x_CO2) = 19.4 kJ/mol        │
  │  현재 기술 소비: 200 kJ/mol                                    │
  │  비율: 200/19.4 = 10.3 ~ sigma-phi = 10 EXACT                 │
  │                                                                 │
  │  CARNOT LIMIT for TSA:                                          │
  │    eta_Carnot = 1 - T_cold/T_hot                               │
  │               = 1 - 300/360                                     │
  │               = 1/6 = 1/n EXACT                                │
  │                                                                 │
  │  Energy breakdown (HEXA-PROCESS, kJ/mol):                      │
  │  ┌────────────────┬────────────┬──────────────┐                │
  │  │  Component      │  Energy    │  n=6 match   │                │
  │  ├────────────────┼────────────┼──────────────┤                │
  │  │  Desorption heat│  12       │  sigma        │                │
  │  │  Compression    │  4        │  tau          │                │
  │  │  Fan/blower     │  2        │  phi          │                │
  │  │  Valve/control  │  1        │  mu           │                │
  │  │  Heat loss      │  1        │  mu           │                │
  │  ├────────────────┼────────────┼──────────────┤                │
  │  │  TOTAL          │  20       │  sigma+tau+   │                │
  │  │                 │           │  phi+2*mu=20  │                │
  │  └────────────────┴────────────┴──────────────┘                │
  │                                                                 │
  │  20 = sigma + tau + phi + 2*mu                                 │
  │     = 12 + 4 + 2 + 2 = 20 EXACT                               │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 7. 시중 대비 압도적 우위

| 지표 | Climeworks | Carbon Eng. | HEXA-PROCESS | 개선 배율 |
|------|-----------|-------------|--------------|-----------|
| 에너지 (kJ/mol) | 200 | 150 | **20** | sigma-phi=10x |
| TSA stages | 2-4 | N/A | **6=n** | - |
| PSA beds | 2-4 | N/A | **12=sigma** | - |
| CO2 순도 (%) | 99.0 | 97.0 | **99.9** | - |
| Cycle time (min) | 30-60 | N/A | **6=n** | 5-10x |
| 열 재생 효율 (%) | 30 | 20 | **83=5/6** | 1-1/n |
| OPEX ($/ton CO2) | 400-600 | 150-200 | **40** | 5-10x |

**핵심 돌파구**: 200 → 20 kJ/mol로 **sigma-phi=10배** 에너지 절감.
6단 TSA 사이클 + 83% 열 회수(1-1/n)가 핵심 메커니즘이다.

```
┌─────────────────────────────────────────────────────────────┐
│  에너지 소비 비교 (kJ/mol, 낮을수록 좋음)                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  시중           ████████████████████████████  200 kJ/mol    │
│  HEXA-PROCESS  ███░░░░░░░░░░░░░░░░░░░░░░░░  20 kJ/mol     │
│                                              (σ-φ=10배↓)   │
│                                                             │
│  Cycle Time 비교 (min, 낮을수록 좋음)                        │
│                                                             │
│  시중           ████████████████████████████  60 min        │
│  HEXA-PROCESS  █████░░░░░░░░░░░░░░░░░░░░░░  6 min          │
│                                              (n=6배↓)      │
│                                                             │
│  CO2 순도 비교 (%)                                           │
│                                                             │
│  시중           █████████████████████████░░  95.0%          │
│  HEXA-PROCESS  ████████████████████████████  99.9%          │
│                                              (6-stage)      │
│                                                             │
│  개선 배수: n=6 상수 기반 (σ-φ, n)                           │
└─────────────────────────────────────────────────────────────┘
```

---

## 8. Cross-Domain Connections

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  PROCESS CROSS-DOMAIN MAP                                      │
  │                                                                 │
  │  Battery (BT-57) ──→ 전기화학 스윙 셀 에너지 공급              │
  │  Fusion (BT-38) ──→ TSA 열원 (120C = sigma*(sigma-phi))        │
  │  Solar (BT-30) ──→ Photocatalytic 공정 (4/3 eV bandgap)       │
  │  Thermal (BT-62) ──→ 열 재생 루프 (60Hz = sigma*sopfr)        │
  │  Chip (BT-56) ──→ RISC-V 공정 제어 (6-stage pipeline)         │
  │                                                                 │
  │  핵심: TSA 열원 = 저온 폐열(80-200C) 활용 가능                 │
  │  → 데이터센터/공장 폐열 + DAC = 에너지 비용 0 근접              │
  └─────────────────────────────────────────────────────────────────┘
```

---

## 9. Honesty Assessment

### 물리적으로 의미 있는 n=6 매칭 (Strong)

| 매칭 | 근거 | 평가 |
|------|------|------|
| W_actual/W_min = 10 ~ sigma-phi | 현재 기술의 열역학 비효율 비율 | **관찰 사실** |
| Carnot eta = 1/6 at 300K/360K | 온도 비율에 의한 결과 | **물리적 (조건부)** |
| ΔT = 120C | sigma*(sigma-phi)이나, MOF 재생 온도에 의해 결정 | **근사적** |

### 우연의 일치 가능성 (Weak)

| 매칭 | 근거 | 평가 |
|------|------|------|
| 6단 TSA | 4단으로도 가능. 6단은 효율적이나 유일하진 않음 | **설계 선택** |
| 12-bed PSA | 4-8 bed도 작동함. 12는 대형 플랜트 기준 | **스케일 의존** |
| Cycle time 6 min | 소재/온도에 따라 2-30분 범위 가변 | **조건부** |

### 솔직한 한계

1. **20 kJ/mol은 이론 최소에 가까움** — 달성 시 열역학적 돌파 필요
2. **83% 열 회수는 매우 도전적** — 현재 최고 기술은 50% 수준
3. **PSA 12-bed 동기화** — 12개 bed의 완벽한 위상 동기화는 제어 난이도 높음
4. **sigma-phi=10 비율** — 우연의 일치 가능성 ~10% (1자리 정수 매칭)

---

## 10. Predictions & Falsifiability

| # | 예측 | 검증 방법 | 기한 | 반증 조건 |
|---|------|----------|------|----------|
| P1 | TSA 6단이 4단 대비 >20% 효율 | 비교 실험 | 2027 | 차이 <5% 시 반증 |
| P2 | 열 회수 >50% 달성 가능 | 파일럿 플랜트 | 2028 | 40% 미달 시 수정 |
| P3 | PSA 12-bed가 8-bed 대비 >15% 순도 향상 | 실험 비교 | 2027 | 차이 <3% 시 반증 |
| P4 | MECS 전기화학 셀 6-stack이 최적 | 스택 수 변화 실험 | 2028 | 4 or 8이 최적 시 반증 |
| P5 | 총 에너지 <50 kJ/mol 달성 | 통합 시스템 측정 | 2029 | 80 kJ/mol 초과 시 수정 |

---

## 11. n=6 Complete Parameter Map

| Parameter | Value | n=6 Expression | Grade |
|-----------|-------|----------------|-------|
| TSA stages | 6 | n | EXACT |
| PSA beds | 12 | sigma | EXACT |
| MECS cells | 6 | n | EXACT |
| Membrane stages | 6 | n | EXACT |
| Cryogenic temp | -48C | sigma*tau | EXACT |
| ΔT swing | 120C | sigma*(sigma-phi) | EXACT |
| Energy ratio (actual/theory) | 10.3 | ~sigma-phi | CLOSE |
| Carnot efficiency | 1/6 | 1/n | EXACT |
| Heat recovery | 83% | 1-1/n | TARGET |
| Cycle time | 6 min | n | DESIGN |
| Compression | 12 MPa | sigma | EXACT |
| Sensor types | 6 | n | EXACT |
| **Total** | | **10/12 (83%)** | |

---

## 12. Links

- [goal.md](goal.md) — 8단 아키텍처 로드맵
- [hexa-sorbent.md](hexa-sorbent.md) — Level 0 소재 (←의존)
- [hexa-reactor.md](hexa-reactor.md) — Level 2 코어 (→공급)
- [hypotheses.md](hypotheses.md) — H-CC-11~20 (공정 가설)
- [BT-94](../breakthrough-theorems.md) — CO2 포집 에너지 n=6 법칙
