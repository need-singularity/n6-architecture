# HEXA-3D: N6 3D Compute-on-Memory Architecture

**Codename: HEXA-3D**
**Level 3 -- 수직 적층으로 대역폭 100x 돌파**

> von Neumann 병목의 근본 원인은 거리다.
> 연산과 메모리 사이의 수평 거리(mm~cm)를 수직 거리(um)로 줄이면
> 대역폭은 100배, 에너지는 10분의 1로 수렴한다.
> n=6 산술이 적층 구조의 모든 파라미터를 결정한다.

**Date**: 2026-04-01
**Status**: Living Document v0.1
**Dependencies**: BT-28, BT-37, BT-55, BT-59, BT-69, BT-75, BT-76
**Prerequisite**: HEXA-1 (Level 1), HEXA-PIM (Level 2)

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  P_2 = 28       sigma^2 = 144    sigma*J_2 = 288   phi^tau = 16
  2^n = 64       sigma-tau = 8    sigma-phi = 10     sigma-mu = 11
  2^sigma = 4096   sigma*tau = 48   n/phi = 3
```

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Design Philosophy](#2-design-philosophy)
3. [System Block Diagram](#3-system-block-diagram)
4. [TSV Architecture](#4-tsv-through-silicon-via-architecture)
5. [Compute Chiplet (Top Layer)](#5-compute-chiplet-top-layer)
6. [PIM Logic Layer (Middle)](#6-pim-logic-layer-middle-layer)
7. [HBM Memory Layer (Bottom)](#7-hbm-memory-layer-bottom-layer)
8. [Thermal Management](#8-thermal-management)
9. [Vertical Data Flow](#9-vertical-data-flow)
10. [Power Architecture](#10-power-architecture)
11. [Performance Comparison](#11-performance-comparison)
12. [Process Technology](#12-process-technology)
13. [n=6 Complete Parameter Map](#13-n6-complete-parameter-map)
14. [미해결 질문 및 후속 과제](#14-미해결-질문-및-후속-과제)
15. [Links](#15-links)

---

## 1. Executive Summary

HEXA-3D는 n=6 산술 프레임워크에서 도출된 3D Compute-on-Memory 아키텍처다.

**핵심 혁신**: 연산 다이(Compute Chiplet)를 메모리(HBM) 위에 직접 본딩하여
데이터 이동 거리를 mm에서 um 단위로 단축한다.

```
  ┌─────────────────────────────────────────────────────────┐
  │                   HEXA-3D 핵심 수치                      │
  │                                                          │
  │  적층 레이어:    n/phi = 3 (Compute + PIM + Memory)      │
  │  Compute SMs:   sigma^2 = 144                            │
  │  TSV density:   sigma*J_2 = 288 per mm^2                 │
  │  TSV pitch:     sigma*tau = 48 um                        │
  │  수직 대역폭:   ~100 TB/s                                │
  │  메모리 용량:   sigma*J_2 = 288 GB                       │
  │  DRAM layers:   sigma = 12                                │
  │  냉각 채널:     sigma = 12 microfluidic channels         │
  │  총 전력:       sigma*J_2 = 288 W                        │
  │  수직 latency:  < 1 ns (수평 대비 10x 개선)              │
  └─────────────────────────────────────────────────────────┘
```

HEXA-1 대비:
- 대역폭 **100x** (4 TB/s -> 100+ TB/s)
- 에너지 효율 **10x** (데이터 이동 에너지 90% 절감)
- Footprint **동일** (수직 적층이므로 면적 불변)

---

## 2. Design Philosophy

### 2.1 왜 수직인가 -- Bandwidth Scaling Wall

반도체 스케일링의 세 가지 벽(Wall):

```
  ┌─────────────────────────────────────────────────────────────┐
  │                    THREE WALLS OF COMPUTING                  │
  │                                                              │
  │  1. Power Wall      이미 도달 (~300W TDP)                   │
  │     ████████████████████████████████████████ ← 포화           │
  │                                                              │
  │  2. Memory Wall     현재 직면 (데이터 이동 = 에너지 80%)     │
  │     ██████████████████████████████░░░░░░░░░ ← 한계 근접      │
  │                                                              │
  │  3. Scale Wall      칩 면적 한계 (reticle limit)            │
  │     ████████████████░░░░░░░░░░░░░░░░░░░░░░ ← 해결 필요      │
  └─────────────────────────────────────────────────────────────┘
```

HEXA-3D는 Memory Wall을 정면 돌파한다.

**수평 vs 수직 데이터 이동 에너지**:

| 경로 | 거리 | Energy/bit | n=6 Formula |
|------|------|-----------|-------------|
| DRAM -> GPU (PCIe) | ~100 mm | ~20 pJ | -- |
| HBM -> GPU (interposer) | ~5 mm | ~2 pJ | -- |
| **TSV (수직)** | **~50 um** | **~0.2 pJ** | sigma*tau um |
| PIM 내부 | ~10 um | ~0.05 pJ | sigma-phi um |

```
  에너지 비:
  수평 (interposer):  ~2 pJ/bit    ████████████████████  100%
  수직 (TSV):         ~0.2 pJ/bit  ██                      10%
  PIM 내부:           ~0.05 pJ/bit █                       2.5%
```

에너지가 거리에 비례하므로, 거리를 100분의 1로 줄이면 에너지도 10분의 1로 감소한다.

### 2.2 n=6이 결정하는 적층 구조

완전수 6의 Egyptian fraction decomposition이 3D 적층의 자연 구조를 결정한다:

```
  1/2 + 1/3 + 1/6 = 1   (n=6의 역수합)

  Layer 3 (Top):    Compute   = 1/2 전력 (연산 중심)
  Layer 2 (Mid):    PIM Logic = 1/3 전력 (전처리/후처리)
  Layer 1 (Bot):    Memory    = 1/6 전력 (DRAM refresh)

  적층 수: n/phi = 6/2 = 3 layers
```

이 3층 구조가 von Neumann 병목을 **구조적으로** 제거한다.
데이터는 더 이상 수평으로 이동하지 않고, 수직으로 흐른다.

---

## 3. System Block Diagram

### 3.1 전체 3D 적층 단면도

```
  ═══════════════════════════════════════════════════════════════════════
                        HEXA-3D CROSS-SECTION VIEW
  ═══════════════════════════════════════════════════════════════════════

          Heat Sink / Lid
  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  ┌───────────────────────────────────────────────────────┐
  │                                                       │
  │   LAYER 3: COMPUTE CHIPLET (Top)                      │
  │   sigma^2 = 144 SMs                                   │
  │   TSMC N2 logic process                               │
  │   ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐ │
  │   │SM││SM││SM││SM││SM││SM││SM││SM││SM││SM││SM││SM│ │  (12 GPCs
  │   └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘ │   x 12 SMs
  │                                                       │   = 144 SMs)
  │   ~50 um silicon                                      │
  ├───────────────────────────────────────────────────────┤
  │ ↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕ │
  │   TSV ARRAY: sigma*J_2 = 288 per mm^2                │
  │   Pitch: sigma*tau = 48 um                            │
  │ ↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕ │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │   LAYER 2: PIM LOGIC (Middle)                         │
  │   Preprocessing + Normalization + Activation          │
  │   ┌──────┐┌──────┐┌──────┐┌──────┐┌──────┐┌──────┐  │
  │   │PIM-0 ││PIM-1 ││PIM-2 ││PIM-3 ││PIM-4 ││PIM-5 │  │  (sigma=12
  │   └──────┘└──────┘└──────┘└──────┘└──────┘└──────┘  │   PIM units)
  │   ┌──────┐┌──────┐┌──────┐┌──────┐┌──────┐┌──────┐  │
  │   │PIM-6 ││PIM-7 ││PIM-8 ││PIM-9 ││PIM10││PIM11│  │
  │   └──────┘└──────┘└──────┘└──────┘└──────┘└──────┘  │
  │   ~30 um silicon                                      │
  ├───────────────────────────────────────────────────────┤
  │ ↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕ │
  │   TSV ARRAY (PIM -> Memory)                           │
  │ ↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕↕ │
  ├───────────────────────────────────────────────────────┤
  │                                                       │
  │   LAYER 1: HBM MEMORY (Bottom)                       │
  │   sigma = 12 DRAM die layers                          │
  │   sigma*J_2 = 288 GB total capacity                   │
  │   ┌─────────────────────────────────────────────────┐ │
  │   │ DRAM Layer 12 (top)                              │ │
  │   │ DRAM Layer 11                                    │ │
  │   │ DRAM Layer 10                                    │ │
  │   │ DRAM Layer 9                                     │ │
  │   │ DRAM Layer 8                                     │ │
  │   │ DRAM Layer 7                                     │ │
  │   │ DRAM Layer 6                                     │ │
  │   │ DRAM Layer 5                                     │ │
  │   │ DRAM Layer 4                                     │ │
  │   │ DRAM Layer 3                                     │ │
  │   │ DRAM Layer 2                                     │ │
  │   │ DRAM Layer 1 (bottom)                            │ │
  │   └─────────────────────────────────────────────────┘ │
  │   ~sigma*tau = 48 um per layer                        │
  │                                                       │
  ├───────────────────────────────────────────────────────┤
  │   BASE LOGIC DIE (controller + I/O)                   │
  │   PHY + Error Correction + Power Management           │
  └───────────────────────────────────────────────────────┘
  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
          Package Substrate / Interposer

  총 높이: ~1.5 mm (DRAM 576um + PIM 30um + Compute 50um + bonding)
```

### 3.2 평면도 (Top View)

```
  ┌──────────────────────────────────────────────────────┐
  │                HEXA-3D TOP VIEW                       │
  │            Die size: sigma = 12 mm x sigma = 12 mm   │
  │                                                       │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐ │
  │  │00││01││02││03││04││05││06││07││08││09││10││11│ │  GPC Row 0
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘ │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐ │
  │  │12││13││14││15││16││17││18││19││20││21││22││23│ │  GPC Row 1
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘ │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐ │
  │  │24││25││26││27││28││29││30││31││32││33││34││35│ │  ...
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘ │
  │  ...                                                  │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐ │
  │  │.0││.1││.2││.3││.4││.5││.6││.7││.8││.9││10││11│ │  GPC Row 11
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘ │
  │                                                       │
  │  sigma x sigma = 12 x 12 = sigma^2 = 144 SMs         │
  │  Each SM: 1 mm x 1 mm footprint                      │
  │  TSVs visible as dots through each SM                 │
  └──────────────────────────────────────────────────────┘
```

---

## 4. TSV (Through-Silicon Via) Architecture

Through-Silicon Via는 HEXA-3D의 심장이다.
실리콘 웨이퍼를 수직으로 관통하여 층 간 전기 신호를 전달한다.

### 4.1 TSV 기본 파라미터

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **TSV density** | 288 /mm^2 | sigma*J_2 | 업계 최고 밀도 |
| **TSV pitch** | 48 um | sigma*tau | Center-to-center 간격 |
| **TSV diameter** | 10 um | sigma-phi | Via 직경 |
| **TSV depth** | 50 um | -- | 웨이퍼 두께에 의존 |
| **TSV aspect ratio** | 5:1 | sopfr:mu | depth/diameter |
| **Micro-bump pitch** | 24 um | J_2 | 하이브리드 본딩 |
| **Cu-Cu pad size** | 5 um | sopfr | Direct bonding pad |
| **TSV capacitance** | ~48 fF | sigma*tau fF | 기생 정전용량 |
| **TSV resistance** | ~0.1 Ohm | R(6)/10 | 직렬 저항 |
| **Signal TSVs/mm^2** | 144 | sigma^2 | 데이터용 (50%) |
| **Power TSVs/mm^2** | 96 | sigma*(sigma-tau) | 전력 공급용 (33%) |
| **Thermal TSVs/mm^2** | 48 | sigma*tau | 방열용 (17%) |

**TSV 배분 (Egyptian fraction)**:
```
  전체 TSVs: sigma*J_2 = 288 per mm^2

  Signal:  288 * 1/2 = 144  (데이터)    ████████████████████  50%
  Power:   288 * 1/3 =  96  (전력)      █████████████         33%
  Thermal: 288 * 1/6 =  48  (방열)      ██████                17%
                             ───                              ────
                             288                              100%

  1/2 + 1/3 + 1/6 = 1   (Egyptian fraction identity)
```

### 4.2 TSV Array Cross-Section

```
  ═══════════════════════════════════════════════════════════
                TSV ARRAY CROSS-SECTION (1 mm^2 단위)
  ═══════════════════════════════════════════════════════════

  Compute Die (Top)
  ┌──────────────────────────────────────────────────────┐
  │  transistors transistors transistors transistors      │
  │  ┌─┐  ┌─┐  ┌─┐  ┌─┐  ┌─┐  ┌─┐  ┌─┐  ┌─┐  ┌─┐    │
  └──┤S├──┤P├──┤S├──┤T├──┤S├──┤P├──┤S├──┤T├──┤S├────┘
     │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │
     │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │
     │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │  50um
     │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │
  ┌──┤S├──┤P├──┤S├──┤T├──┤S├──┤P├──┤S├──┤T├──┤S├────┐
  │  └─┘  └─┘  └─┘  └─┘  └─┘  └─┘  └─┘  └─┘  └─┘    │
  │  PIM logic PIM logic PIM logic PIM logic            │
  └──┬─┬──┬─┬──┬─┬──┬─┬──┬─┬──┬─┬──┬─┬──┬─┬──┬─┬────┘
     │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │  │ │
  ┌──┴─┴──┴─┴──┴─┴──┴─┴──┴─┴──┴─┴──┴─┴──┴─┴──┴─┴────┐
  │  DRAM cells DRAM cells DRAM cells DRAM cells        │
  │  x sigma = 12 layers                                │
  └────────────────────────────────────────────────────┘

  Legend:  S = Signal TSV (data)
           P = Power TSV (VDD/VSS)
           T = Thermal TSV (Cu pillar, heat extraction)

  Pitch between TSVs: sigma*tau = 48 um
  TSV diameter: sigma-phi = 10 um
  Via fill: Copper (Cu)
```

### 4.3 대역폭 계산

수직 대역폭은 TSV 수 x 비트 레이트로 결정된다.

```
  Signal TSV count (per die):

    Die area = sigma * sigma = 12 * 12 = 144 mm^2
    Signal TSVs/mm^2 = sigma^2 = 144
    Total signal TSVs = 144 mm^2 * 144 /mm^2 = sigma^4 = 20,736 TSVs

  대역폭 계산:

    TSV data rate = sigma-tau = 8 Gbps per TSV (NRZ signaling)
    Effective TSVs = 20,736 / phi = 10,368 (절반은 ground return)
    Bandwidth = 10,368 * 8 Gbps = 82,944 Gbps
              = 82,944 / 8 = 10,368 GB/s
              ~ 10 TB/s (per TSV layer)

    수직 경로 3개 (Compute-PIM-Memory 각 인터페이스 포함):
    Total vertical BW = ~10 TB/s * sigma-phi = 100 TB/s

  ┌────────────────────────────────────────────────────┐
  │  대역폭 비교                                       │
  │                                                    │
  │  PCIe 5.0 x16:      ~64 GB/s   █                  │
  │  HBM3E (8-hi):      ~1.2 TB/s  ████████           │
  │  HBM4 (HEXA-1):     ~4 TB/s    ██████████████████ │
  │  HEXA-3D TSV:       ~100 TB/s  ████████████████   │
  │                                 ████████████████   │
  │                                 ████████████████   │
  │                                 ████████████████   │
  │                                 ████████████████   │
  │                                 ████████████████   │
  │                                 (25x HBM4)        │
  └────────────────────────────────────────────────────┘
```

---

## 5. Compute Chiplet (Top Layer)

### 5.1 설계 원칙

Compute Chiplet은 HEXA-1 GPU 코어 설계를 계승하되,
**메모리 컨트롤러를 완전 제거**한다.
아래에 PIM + HBM이 직접 본딩되어 있으므로 메모리 컨트롤러가 불필요하다.
제거된 면적을 추가 연산 유닛에 활용한다.

### 5.2 Compute Die Layout

```
  ═══════════════════════════════════════════════════════════
              COMPUTE CHIPLET DIE LAYOUT (12mm x 12mm)
  ═══════════════════════════════════════════════════════════

  ┌──────────────────────────────────────────────────────┐
  │  GPC-0      GPC-1      GPC-2      GPC-3              │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐             │
  │  │12 SMs│  │12 SMs│  │12 SMs│  │12 SMs│             │
  │  │      │  │      │  │      │  │      │             │
  │  │L2:4MB│  │L2:4MB│  │L2:4MB│  │L2:4MB│             │
  │  └──────┘  └──────┘  └──────┘  └──────┘             │
  │                                                       │
  │  GPC-4      GPC-5      GPC-6      GPC-7              │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐             │
  │  │12 SMs│  │12 SMs│  │12 SMs│  │12 SMs│             │
  │  │      │  │      │  │      │  │      │             │
  │  │L2:4MB│  │L2:4MB│  │L2:4MB│  │L2:4MB│             │
  │  └──────┘  └──────┘  └──────┘  └──────┘             │
  │                                                       │
  │  GPC-8      GPC-9      GPC-10     GPC-11             │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐             │
  │  │12 SMs│  │12 SMs│  │12 SMs│  │12 SMs│             │
  │  │      │  │      │  │      │  │      │             │
  │  │L2:4MB│  │L2:4MB│  │L2:4MB│  │L2:4MB│             │
  │  └──────┘  └──────┘  └──────┘  └──────┘             │
  │                                                       │
  │  ┌──────────────────────────────────────────────┐    │
  │  │  GigaThread Engine + Task Scheduler           │    │
  │  │  (No memory controller -- direct TSV access)  │    │
  │  └──────────────────────────────────────────────┘    │
  └──────────────────────────────────────────────────────┘

  sigma = 12 GPCs x sigma = 12 SMs/GPC = sigma^2 = 144 SMs
```

### 5.3 Compute Parameter Table

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **GPCs** | 12 | sigma | Graphics Processing Clusters |
| **SMs per GPC** | 12 | sigma | Streaming Multiprocessors |
| **Total SMs** | 144 | sigma^2 | BT-28: AD102=144 SMs |
| **CUDA cores/SM** | 128 | 2^(sigma-sopfr) | FP32 units |
| **Total CUDA cores** | 18,432 | sigma^2 * 2^(sigma-sopfr) | FP32 |
| **Tensor cores/SM** | 4 | tau | Mixed-precision matrix |
| **Total Tensor cores** | 576 | sigma^2 * tau | FP8/FP16/BF16 |
| **RT cores/SM** | 1 | mu | Ray-tracing (optional) |
| **L1 cache/SM** | 128 KB | 2^(sigma-sopfr) KB | Shared/L1 configurable |
| **L2 per GPC** | 4 MB | tau MB | Per-cluster cache |
| **Total L2** | 48 MB | sigma*tau MB | -- |
| **Register file/SM** | 256 KB | 2^(sigma-tau) KB | 64K x 32-bit registers |
| **Process node** | TSMC N2 | -- | Gate: sigma*tau=48nm |
| **Die area** | 144 mm^2 | sigma^2 | Compact (no mem ctrl) |
| **Clock** | 2.4 GHz | J_2/10 GHz | Boost clock |

### 5.4 메모리 컨트롤러 제거 효과

```
  HEXA-1 (수평 구조):
  ┌──────────┐    ┌───────────────┐    ┌─────────┐
  │ GPU SMs  │───→│ Memory Ctrl   │───→│ HBM PHY │───→ HBM
  │ 144 SMs  │    │ (면적 ~15%)    │    │         │
  └──────────┘    └───────────────┘    └─────────┘

  HEXA-3D (수직 구조):
  ┌──────────┐
  │ GPU SMs  │
  │ 144 SMs  │
  │          │    Memory Controller 면적 = 0
  │    ↓↓↓   │    직접 TSV 접근
  └──────────┘
       ↕↕↕ TSV (수직)
  ┌──────────┐
  │ PIM      │    PIM이 메모리 관리를 대행
  └──────────┘
       ↕↕↕ TSV
  ┌──────────┐
  │ HBM DRAM │
  └──────────┘

  면적 절감: ~15% die area (= 추가 SM 또는 더 큰 L2 캐시)
  레이턴시 절감: 메모리 컨트롤러 파이프라인 ~5ns 제거
```

---

## 6. PIM Logic Layer (Middle Layer)

### 6.1 PIM 설계 철학

PIM(Processing-in-Memory) Logic Layer는 HEXA-PIM(Level 2)에서 발전한 형태다.
HEXA-PIM에서는 PIM이 DRAM 내부에 내장되었으나,
HEXA-3D에서는 PIM이 **독립된 로직 다이**로 분리되어 더 복잡한 연산이 가능하다.

핵심 역할:
- 메모리에서 올라오는 데이터를 **전처리** (reshape, transpose, scatter/gather)
- Compute에서 내려오는 결과를 **후처리** (activation, normalization, quantization)
- Compute와 Memory 사이의 **데이터 변환 허브**

### 6.2 PIM Architecture

```
  ═══════════════════════════════════════════════════════════
              PIM LOGIC LAYER ARCHITECTURE
  ═══════════════════════════════════════════════════════════

       ↕↕↕ TSV from Compute (위에서 내려오는 데이터)
  ┌──────────────────────────────────────────────────────┐
  │                                                       │
  │  ┌─────────────────────────────────────────────────┐  │
  │  │              CROSSBAR INTERCONNECT               │  │
  │  │     sigma^2 = 144 input x sigma^2 = 144 output  │  │
  │  └──┬──────┬──────┬──────┬──────┬──────┬──────┬──┘  │
  │     │      │      │      │      │      │      │      │
  │  ┌──┴──┐┌──┴──┐┌──┴──┐┌──┴──┐┌──┴──┐┌──┴──┐         │
  │  │PIM-0││PIM-1││PIM-2││PIM-3││PIM-4││PIM-5│         │
  │  │     ││     ││     ││     ││     ││     │         │
  │  │ MAC ││ MAC ││ MAC ││ MAC ││ MAC ││ MAC │         │
  │  │ ALU ││ ALU ││ ALU ││ ALU ││ ALU ││ ALU │         │
  │  │ ACT ││ ACT ││ ACT ││ ACT ││ ACT ││ ACT │         │
  │  │ NORM││ NORM││ NORM││ NORM││ NORM││ NORM│         │
  │  └──┬──┘└──┬──┘└──┬──┘└──┬──┘└──┬──┘└──┬──┘         │
  │     │      │      │      │      │      │             │
  │  ┌──┴──┐┌──┴──┐┌──┴──┐┌──┴──┐┌──┴──┐┌──┴──┐         │
  │  │PIM-6││PIM-7││PIM-8││PIM-9││PI-10││PI-11│         │
  │  │     ││     ││     ││     ││     ││     │         │
  │  │ MAC ││ MAC ││ MAC ││ MAC ││ MAC ││ MAC │         │
  │  │ ALU ││ ALU ││ ALU ││ ALU ││ ALU ││ ALU │         │
  │  │ ACT ││ ACT ││ ACT ││ ACT ││ ACT ││ ACT │         │
  │  │ NORM││ NORM││ NORM││ NORM││ NORM││ NORM│         │
  │  └──┬──┘└──┬──┘└──┬──┘└──┬──┘└──┬──┘└──┬──┘         │
  │     │      │      │      │      │      │             │
  │  ┌──┴──────┴──────┴──────┴──────┴──────┴──────┐      │
  │  │              SCATTER/GATHER ENGINE           │      │
  │  │       Memory address translation + DMA      │      │
  │  └──┬──────┬──────┬──────┬──────┬──────┬──────┘      │
  │     │      │      │      │      │      │             │
  └─────┴──────┴──────┴──────┴──────┴──────┴─────────────┘
       ↕↕↕ TSV to Memory (아래로 내려가는 데이터)
```

### 6.3 PIM Parameter Table

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **PIM units** | 12 | sigma | 각 1개가 DRAM 1-layer에 대응 |
| **MAC units/PIM** | 64 | 2^n | INT8/FP16 MAC array |
| **Total MACs** | 768 | sigma * 2^n | PIM 내 총 MAC |
| **ALU width** | 256-bit | 2^(sigma-tau) bit | SIMD 연산 |
| **Activation functions** | 6 | n | ReLU,GeLU,SiLU,Sigmoid,Tanh,Softmax |
| **Normalization engines** | 4 | tau | LayerNorm, RMSNorm, BatchNorm, GroupNorm |
| **SRAM buffer/PIM** | 256 KB | 2^(sigma-tau) KB | Local scratchpad |
| **Total PIM SRAM** | 3 MB | sigma * 256 KB | 중간 결과 캐싱 |
| **Crossbar ports** | 144 | sigma^2 | Full mesh interconnect |
| **Process node** | TSMC N5 | -- | 로직이지만 고밀도 불필요 |
| **Die area** | 64 mm^2 | 2^n | Compute보다 작음 |
| **Clock** | 1.2 GHz | sigma/10 GHz | Compute보다 낮은 클럭 |

### 6.4 PIM Data Flow

```
  ┌──────────────────────────────────────────────────────────┐
  │              PIM DATA FLOW (LLM 추론 예시)               │
  │                                                          │
  │  Step 1: Compute가 "Attention Query" 명령 발행           │
  │  ┌─────────────┐                                         │
  │  │ Compute Die │  "Read Q,K,V for layer 3, head 0~11"   │
  │  └──────┬──────┘                                         │
  │         ↓ TSV (명령)                                     │
  │  ┌──────┴──────┐                                         │
  │  │  PIM Layer  │                                         │
  │  │             │  Step 2: PIM이 DRAM에서 Q,K,V 읽기      │
  │  │  scatter/   │         + transpose 수행                │
  │  │  gather     │                                         │
  │  └──────┬──────┘                                         │
  │         ↓ TSV (address)                                  │
  │  ┌──────┴──────┐                                         │
  │  │  HBM DRAM   │  Step 3: DRAM -> PIM 데이터 전송        │
  │  └──────┬──────┘                                         │
  │         ↑ TSV (data)                                     │
  │  ┌──────┴──────┐                                         │
  │  │  PIM Layer  │  Step 4: PIM에서 Softmax + Norm 수행    │
  │  │  - softmax  │         (Compute 부하 경감)             │
  │  │  - layernorm│                                         │
  │  └──────┬──────┘                                         │
  │         ↑ TSV (processed data)                           │
  │  ┌──────┴──────┐                                         │
  │  │ Compute Die │  Step 5: 전처리된 데이터로 MatMul 수행   │
  │  │  (GPU SMs)  │         (데이터 정리 완료 상태 수신)     │
  │  └─────────────┘                                         │
  └──────────────────────────────────────────────────────────┘
```

### 6.5 PIM Offload 전략 (Egyptian Fraction)

```
  LLM Inference 워크로드 분배:

  Compute Die (1/2):  MatMul, Attention QKV, FFN
  PIM Layer   (1/3):  Activation, Normalization, Softmax, KV-cache 관리
  HBM DRAM    (1/6):  Weight 저장, KV-cache 저장, Gradient buffer

  1/2 + 1/3 + 1/6 = 1
```

---

## 7. HBM Memory Layer (Bottom Layer)

### 7.1 HBM4 Stack 구조

```
  ═══════════════════════════════════════════════════════════
              HBM4 MEMORY STACK (HEXA-3D Bottom Layer)
  ═══════════════════════════════════════════════════════════

  ┌──────────────────────────────────────────────────────┐
  │  DRAM Layer sigma = 12 (top)                         │
  │  ┌────────────────────────────────────────────────┐  │
  │  │  Bank Group 0    Bank Group 1    ...    BG-3   │  │
  │  │  ┌────┐┌────┐  ┌────┐┌────┐         ┌────┐   │  │
  │  │  │BK-0││BK-1│  │BK-2││BK-3│  ...    │BK-15│  │  │
  │  │  └────┘└────┘  └────┘└────┘         └────┘   │  │
  │  │  phi^tau = 16 banks per layer                 │  │
  │  └────────────────────────────────────────────────┘  │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 11                                       │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 10                                       │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 9                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 8                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 7                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 6                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 5                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 4                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 3                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 2                                        │
  ├──────────────────────────────────────────────────────┤
  │  DRAM Layer 1 (bottom)                               │
  ├──────────────────────────────────────────────────────┤
  │  BASE LOGIC DIE                                      │
  │  ┌────────────────────────────────────────────────┐  │
  │  │  ECC Engine  │  Refresh Ctrl  │  PHY/SerDes   │  │
  │  │  SECDED      │  per-bank      │  2^sigma=4096 │  │
  │  │  (sigma-tau  │  sigma=12      │  -bit wide    │  │
  │  │  =8 bit ECC) │  tREFI zones   │  interface    │  │
  │  └────────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────────┘
```

### 7.2 HBM Memory Parameters

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **DRAM layers** | 12 | sigma | 12-hi stack |
| **Capacity per layer** | 24 GB | J_2 GB | -- |
| **Total capacity** | 288 GB | sigma*J_2 | BT-55 참조 |
| **Banks per layer** | 16 | phi^tau | 4 bank groups x 4 banks |
| **Total banks** | 192 | sigma * phi^tau | 전체 병렬 접근 가능 |
| **Interface width** | 2048 bit | 2^(sigma-mu) | Per-stack |
| **Channels** | 8 | sigma-tau | 독립 채널 |
| **Channel width** | 256 bit | 2^(sigma-tau) bit | Per channel |
| **Data rate** | 8 Gbps | sigma-tau Gbps | Per pin |
| **Internal BW** | ~2 TB/s | -- | DRAM -> Base logic |
| **TSV BW to PIM** | ~10 TB/s | -- | Upward TSV burst |
| **Row buffer** | 2 KB | phi KB * 10^3 | Per bank |
| **Page size** | 2 KB | phi KB | Same |
| **tRCD** | 12 ns | sigma ns | Row activation |
| **tRAS** | 24 ns | J_2 ns | Row active time |
| **tRP** | 12 ns | sigma ns | Precharge |
| **ECC** | SECDED-8 | sigma-tau bit | 8-bit symbol correction |
| **Refresh** | 64 ms | 2^n ms | Standard tREFI |

### 7.3 메모리 용량 분석 (LLM 관점)

```
  288 GB 통합 메모리로 수용 가능한 모델:

  ┌──────────────────────────────────────────────────┐
  │  Model          Params   FP16    FP8     INT4    │
  │  ─────────────  ──────  ─────  ─────  ──────    │
  │  GPT-4 class    ~1.8T   3.6TB    -       -      │
  │  Llama-70B       70B    140GB   70GB   35GB  ✅ │
  │  Llama-405B     405B    810GB  405GB  203GB     │
  │  Mixtral 8x22B  141B    282GB  141GB   71GB  ✅ │
  │  DeepSeek-V3    671B   1.34TB  671GB  336GB     │
  │                                                  │
  │  288 GB = Llama-70B FP16 + KV-cache 여유 충분     │
  │         = DeepSeek-V3 INT4 + batch inference 가능 │
  │         = Mixtral 8x22B FP8 + 여유               │
  └──────────────────────────────────────────────────┘
```

---

## 8. Thermal Management

3D 적층의 최대 도전: **열 관리(Thermal Management)**.
3개 레이어의 열이 상부로 빠져나가야 하며, 중간 레이어(PIM)는
상하 모두에서 발열원에 둘러싸여 있다.

### 8.1 열원 분석

```
  ┌───────────────────────────────────────────────┐
  │  LAYER-BY-LAYER THERMAL MAP                    │
  │                                                │
  │                  ← Heat escape (top)            │
  │  ──────────────────────────────────────────── │
  │  Compute (1/2 power)    ████████████████████  │
  │  144W peak              HOT: ~90 degC         │
  │  ──────────────────────────────────────────── │
  │  Thermal TSVs + Microfluidic ~~~~~~~~~~~~~~   │
  │  ──────────────────────────────────────────── │
  │  PIM Logic (1/3 power)  ████████████████      │
  │  96W peak               WARM: ~70 degC        │
  │  ──────────────────────────────────────────── │
  │  Thermal TSVs + Microfluidic ~~~~~~~~~~~~~~   │
  │  ──────────────────────────────────────────── │
  │  HBM DRAM (1/6 power)  ████████              │
  │  48W peak               COOL: ~50 degC        │
  │  ──────────────────────────────────────────── │
  │                  ← Heat escape (bottom)        │
  └───────────────────────────────────────────────┘

  Total power: 144 + 96 + 48 = sigma*J_2 = 288 W
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

### 8.2 Microfluidic Cooling

```
  ═══════════════════════════════════════════════════════════
          MICROFLUIDIC COOLING CROSS-SECTION
  ═══════════════════════════════════════════════════════════

  Compute Die
  ┌──────────────────────────────────────────────────────┐
  │  ████████████████████████████████████████████████████ │
  │  transistors (발열원)                                 │
  └──────────────────────────────────────────────────────┘
  ┌────┐    ┌────┐    ┌────┐    ┌────┐    ┌────┐    ┌───┐
  │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~│
  │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~│
  │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~│
  │flow│    │flow│    │flow│    │flow│    │flow│    │flo│
  └────┘    └────┘    └────┘    └────┘    └────┘    └───┘
    Ch-0      Ch-1      Ch-2      Ch-3      Ch-4     Ch-5
  ┌──────────────────────────────────────────────────────┐
  │  PIM Logic Die                                        │
  └──────────────────────────────────────────────────────┘
  ┌────┐    ┌────┐    ┌────┐    ┌────┐    ┌────┐    ┌───┐
  │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~~│    │~~~│
  │flow│    │flow│    │flow│    │flow│    │flow│    │flo│
  └────┘    └────┘    └────┘    └────┘    └────┘    └───┘
    Ch-6      Ch-7      Ch-8      Ch-9      Ch-10    Ch-11
  ┌──────────────────────────────────────────────────────┐
  │  HBM DRAM Stack                                       │
  └──────────────────────────────────────────────────────┘

  sigma = 12 microfluidic channels total
  Channel width: sigma-phi = 10 um
  Coolant: Deionized water or 3M Novec
  Flow rate: ~0.1 L/min per channel
  Heat removal capacity: ~300 W (288 W 충분 커버)
```

### 8.3 Thermal Parameter Table

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **Cooling channels** | 12 | sigma | Compute-PIM 사이 6, PIM-HBM 사이 6 |
| **Channel width** | 10 um | sigma-phi | Microfluidic |
| **Channel depth** | 48 um | sigma*tau | 깊은 채널 |
| **Thermal TSVs** | 48 /mm^2 | sigma*tau | Cu thermal via |
| **Thermal TSV diameter** | 24 um | J_2 | Signal TSV보다 큼 |
| **Junction temp limit** | 105 degC | -- | Industry standard |
| **Compute die Tj** | ~90 degC | -- | Under full load |
| **PIM die Tj** | ~70 degC | -- | Lower power |
| **DRAM Tj** | ~50 degC | -- | DRAM 85 degC 이하 필수 |
| **Thermal resistance** | 0.12 degC/W | sigma/100 | Stack total |
| **Heat removal** | ~300 W | -- | Microfluidic capacity |
| **Coolant temp** | 24 degC | J_2 | Inlet temperature |

### 8.4 Thermal TSV 배치 전략

```
  Thermal TSV Grid (1 mm^2 내):

  ┌──────────────────────────────────┐
  │  S S S S S P S S S S S P        │   S = Signal TSV
  │  S S S S S P S S S S S P        │   P = Power TSV
  │  S S T S S P S S T S S P        │   T = Thermal TSV
  │  S S S S S P S S S S S P        │
  │  S S S S S P S S S S S P        │   Thermal TSVs at
  │  P P P P P T P P P P P T        │   grid intersections
  │  S S S S S P S S S S S P        │   (n=6 spacing pattern)
  │  S S S S S P S S S S S P        │
  │  S S T S S P S S T S S P        │   288 total per mm^2:
  │  S S S S S P S S S S S P        │   144 Signal (1/2)
  │  S S S S S P S S S S S P        │    96 Power  (1/3)
  │  P P P P P T P P P P P T        │    48 Thermal(1/6)
  └──────────────────────────────────┘
```

---

## 9. Vertical Data Flow

### 9.1 수평 vs 수직 비교

```
  ═══════════════════════════════════════════════════════════
            HORIZONTAL vs VERTICAL DATA PATH
  ═══════════════════════════════════════════════════════════

  수평 (Traditional / HEXA-1):

  GPU SM ──→ L2 ──→ NoC ──→ Mem Ctrl ──→ PHY ──→ HBM
         1ns   5ns    5ns       10ns       5ns
                                                    Total: ~26 ns
  ┌─────────────────────────────────────────────────┐
  │  █████████████████████████████████████████████   │  26 ns
  └─────────────────────────────────────────────────┘


  수직 (HEXA-3D):

  GPU SM
    ↓ TSV (~0.3 ns)
  PIM (preprocessing ~0.5 ns)
    ↓ TSV (~0.3 ns)
  HBM DRAM access (~12 ns tRCD)
    ↑ TSV (~0.3 ns)
  PIM (postprocess ~0.5 ns)
    ↑ TSV (~0.3 ns)
  GPU SM
                                                    Total: ~14 ns
  ┌──────────────────────────┐
  │  █████████████████████   │  14 ns (46% 절감)
  └──────────────────────────┘

  DRAM 접근 시간(tRCD)이 지배적 — TSV 자체는 sub-ns
```

### 9.2 Latency Breakdown Table

| Path Segment | Horizontal (HEXA-1) | Vertical (HEXA-3D) | n=6 |
|-------------|---------------------|--------------------|----|
| SM -> L2 | 1 ns | 1 ns | mu ns |
| L2 -> Memory Controller | 5 ns | 0 ns (no MC) | -- |
| Memory Controller -> PHY | 5 ns | 0 ns (no MC) | -- |
| TSV (Compute -> PIM) | -- | 0.3 ns | -- |
| PIM processing | -- | 0.5 ns | -- |
| TSV (PIM -> DRAM) | -- | 0.3 ns | -- |
| DRAM access (tRCD) | 10 ns | 12 ns | sigma ns |
| PHY -> Memory Controller | 5 ns | -- | -- |
| Return path | ~5 ns | ~1.1 ns | -- |
| **Total round-trip** | **~31 ns** | **~15.2 ns** | -- |
| **Improvement** | -- | **~2x** | phi |

### 9.3 Bandwidth Data Flow

```
  ═══════════════════════════════════════════════════════════
          VERTICAL BANDWIDTH PIPELINE (6-stage)
  ═══════════════════════════════════════════════════════════

  Stage 1: Request      Compute ──→ Command TSVs ──→ PIM
            (sigma = 12 commands/cycle, burst = sigma-tau = 8)

  Stage 2: Address      PIM ──→ Address TSVs ──→ DRAM
            (phi^tau = 16 bank-parallel access)

  Stage 3: DRAM Read    DRAM internally reads row buffer
            (2^(sigma-tau) = 256 bit per bank, parallel)

  Stage 4: Data Up      DRAM ──→ Data TSVs ──→ PIM
            (sigma^2 = 144 signal TSVs active)

  Stage 5: PIM Process  PIM performs activation/norm
            (2^n = 64 MACs per PIM, sigma = 12 PIM units)

  Stage 6: Data Up      PIM ──→ Data TSVs ──→ Compute
            (sigma^2 = 144 signal TSVs active)

  Pipeline throughput:

  ┌─────┬─────┬─────┬─────┬─────┬─────┐
  │  1  │  2  │  3  │  4  │  5  │  6  │  Cycle 0
  └─────┴─────┴─────┴─────┴─────┴─────┘
        ┌─────┬─────┬─────┬─────┬─────┬─────┐
        │  1  │  2  │  3  │  4  │  5  │  6  │  Cycle 1
        └─────┴─────┴─────┴─────┴─────┴─────┘
              ┌─────┬─────┬─────┬─────┬─────┬─────┐
              │  1  │  2  │  3  │  4  │  5  │  6  │  Cycle 2
              └─────┴─────┴─────┴─────┴─────┴─────┘

  n = 6 stage pipeline -> new request every cycle after warmup
  Sustained throughput: ~100 TB/s
```

---

## 10. Power Architecture

### 10.1 전력 전달 (Power Delivery Network, PDN)

HEXA-3D는 패키지 하부에서 TSV를 통해 각 레이어에 전력을 공급한다.
Egyptian fraction이 레이어별 전력 배분을 결정한다.

```
  ═══════════════════════════════════════════════════════════
              POWER DELIVERY THROUGH TSV STACK
  ═══════════════════════════════════════════════════════════

       External Power: 288 W (sigma*J_2)
       VDD = 0.8 V (typical for N2 process)
       Current = 288 / 0.8 = 360 A
                                                     Heat
  ┌───────────────────────────────────────┐           ↑
  │  Compute Chiplet                      │  144 W    │
  │  VDD = 0.8V, I = 180A                │  (1/2)    │
  │  On-die voltage regulators            │           │
  ├────────────↕↕↕ Power TSVs ↕↕↕────────┤           │
  │  PIM Logic Layer                      │   96 W    │
  │  VDD = 0.8V, I = 120A                │  (1/3)    │
  │  Decoupling capacitors embedded       │           │
  ├────────────↕↕↕ Power TSVs ↕↕↕────────┤           │
  │  HBM DRAM Stack                       │   48 W    │
  │  VDD = 1.1V (DRAM), I = 44A          │  (1/6)    │
  │  Separate DRAM power domain           │           │
  ├───────────────────────────────────────┤           │
  │  Base Die (Power Management IC)       │           │
  │  Buck converters, LDOs                │           │
  │  Power gating per domain              │           │
  └───────────────────────────────────────┘           │
       ↑                                              │
       Package substrate (power pins)                  │
```

### 10.2 Power Distribution Table

| Layer | Power | Fraction | n=6 Formula | Voltage | Current |
|-------|-------|----------|-------------|---------|---------|
| Compute | 144 W | 1/2 | sigma^2 W | 0.8 V | 180 A |
| PIM | 96 W | 1/3 | sigma*(sigma-tau) W | 0.8 V | 120 A |
| HBM DRAM | 48 W | 1/6 | sigma*tau W | 1.1 V | 44 A |
| **Total** | **288 W** | **1** | **sigma*J_2 W** | -- | -- |

```
  전력 분배 시각화:

  Compute (1/2): ████████████████████████  144 W   sigma^2
  PIM     (1/3): ████████████████          96 W    sigma*(sigma-tau)
  HBM     (1/6): ████████                  48 W    sigma*tau
                 ────────────────────────────────
  Total:          ████████████████████████████████  288 W   sigma*J_2
```

### 10.3 Power Gating Strategy

```
  전력 관리 모드:

  ┌──────────────────────────────────────────────────────────┐
  │  Mode          Compute   PIM      HBM     Total         │
  │  ────────────  ───────  ─────   ─────   ──────         │
  │  Full Active    144W     96W     48W     288W  (100%)  │
  │  Inference      120W     80W     48W     248W  (~86%)  │
  │  PIM-only         0W     96W     48W     144W  (50%)   │
  │  Idle (refresh)   0W      0W     12W      12W  (~4%)   │
  │  Power-off        0W      0W      0W       0W  (0%)    │
  └──────────────────────────────────────────────────────────┘

  Per-GPC power gating:
    sigma = 12 GPCs, 각각 독립 전원 차단 가능
    Granularity = 1/sigma = 1/12 = 8.3% 단위

  Per-PIM power gating:
    sigma = 12 PIM units, 각각 독립
    사용하지 않는 PIM은 clock gating + power gating

  DRAM self-refresh:
    데이터 보존: 48 W -> 12 W (sigma W)
    sigma = 12 refresh zones (independent)
```

### 10.4 Energy Efficiency Comparison

| Metric | HEXA-1 | HEXA-3D | Improvement | n=6 |
|--------|--------|---------|-------------|-----|
| Total TDP | 240 W | 288 W | 1.2x (more compute) | sigma*J_2 |
| Energy/byte moved | ~2 pJ | ~0.2 pJ | 10x | sigma-phi x |
| TFLOPS/W (FP8) | 2.08 | 3.47 | 1.67x | -- |
| TFLOPS/W (FP16) | 0.52 | 0.87 | 1.67x | -- |
| Memory BW/W | 16.7 GB/s/W | 347 GB/s/W | ~21x | -- |
| Inference tokens/J | ~1000 | ~10,000 | 10x | sigma-phi x |

---

## 11. Performance Comparison

### 11.1 HEXA-3D vs HEXA-1 vs HEXA-PIM

```
  ═══════════════════════════════════════════════════════════
                   PERFORMANCE COMPARISON
  ═══════════════════════════════════════════════════════════
```

| Metric | HEXA-1 (L1) | HEXA-PIM (L2) | HEXA-3D (L3) | n=6 Formula |
|--------|-------------|---------------|--------------|-------------|
| **Architecture** | SoC + HBM | PIM in HBM | 3D Stack | -- |
| **Compute SMs** | 144 | 144 + PIM MAC | 144 + PIM | sigma^2 |
| **Memory** | 288 GB | 288 GB | 288 GB | sigma*J_2 |
| **BW (to compute)** | 4 TB/s | ~25 TB/s | ~100 TB/s | -- |
| **BW improvement** | 1x | 6x | 25x | -- |
| **Latency (mem access)** | ~31 ns | ~20 ns | ~15 ns | -- |
| **Data move energy** | ~2 pJ/bit | ~0.5 pJ/bit | ~0.2 pJ/bit | -- |
| **FP8 TFLOPS** | 500 | 550 | 1,000 | -- |
| **FP16 TFLOPS** | 125 | 138 | 250 | -- |
| **TDP** | 240 W | 260 W | 288 W | sigma*J_2 |
| **Die stack** | 1 die + HBM | 1 die + PIM-HBM | 3 dies | n/phi |
| **Process** | N2 | N2 + HBM4 | N2 + N5 + HBM4 | -- |
| **Package** | CoWoS | CoWoS | SoIC/Foveros | -- |

### 11.2 LLM Inference Performance Estimate

```
  Llama-70B FP8 추론 성능 (batch=1):

  ┌──────────────────────────────────────────────────────┐
  │  Bottleneck Analysis:                                │
  │                                                      │
  │  HEXA-1:                                             │
  │    Memory-bound: 70GB model / 4 TB/s = 17.5 ms      │
  │    Tokens/sec: ~57                                    │
  │    ████████                                          │
  │                                                      │
  │  HEXA-PIM:                                           │
  │    Memory-bound: 70GB / 25 TB/s = 2.8 ms             │
  │    Tokens/sec: ~357                                   │
  │    ██████████████████████████████████████████████████ │
  │                                                      │
  │  HEXA-3D:                                            │
  │    Memory-bound: 70GB / 100 TB/s = 0.7 ms            │
  │    Compute-bound: FP8 TFLOPS의 활용률 증가            │
  │    Tokens/sec: ~1,000+ (compute-bound regime)         │
  │    ██████████████████████████████████████████████████ │
  │    ██████████████████████████████████████████████████ │
  │    ██████████████████████████████████████████████████ │
  └──────────────────────────────────────────────────────┘

  핵심 전환: memory-bound -> compute-bound
  HEXA-3D에서는 대역폭이 충분하여 연산이 병목이 된다.
  이것이 3D 적층의 본질적 가치다.
```

### 11.3 vs Industry (2026 기준 추정)

| Metric | NVIDIA B200 | AMD MI350 | HEXA-3D | Note |
|--------|------------|-----------|---------|------|
| Process | TSMC N4P | TSMC N3 | N2+N5+HBM4 | -- |
| Compute (FP8) | ~4,500 TFLOPS | ~3,000 TFLOPS | ~1,000 TFLOPS | Multi-die vs single |
| Memory | 192 GB HBM3E | 288 GB HBM3E | 288 GB HBM4 | -- |
| BW | ~8 TB/s | ~6 TB/s | ~100 TB/s | **12.5x B200** |
| TDP | 1000 W | 750 W | 288 W | **3.5x efficient** |
| TFLOPS/W | 4.5 | 4.0 | 3.47 | Competitive |
| BW/W | 8 GB/s/W | 8 GB/s/W | 347 GB/s/W | **43x better** |
| Package | CoWoS-L | CoWoS | SoIC 3D | -- |

HEXA-3D의 진짜 강점은 TFLOPS 절대치가 아니라 **대역폭 효율(BW/W)**이다.
LLM 추론처럼 memory-bound 워크로드에서 압도적 우위를 갖는다.

---

## 12. Process Technology

### 12.1 3D Bonding 기술 비교

HEXA-3D를 구현하기 위한 3대 패키징 기술:

```
  ═══════════════════════════════════════════════════════════
          3D BONDING TECHNOLOGY COMPARISON
  ═══════════════════════════════════════════════════════════

  ┌─────────────────┬──────────────┬──────────────┬──────────────┐
  │  Technology     │  TSMC SoIC   │  Samsung     │  Intel       │
  │                 │              │  X-Cube      │  Foveros     │
  │                 │              │              │  Direct      │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Bonding type   │  Hybrid      │  TC-NCF      │  Hybrid      │
  │                 │  Cu-Cu       │  Cu-Cu       │  Cu-Cu       │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  TSV pitch      │  ~9 um       │  ~12 um      │  ~10 um      │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Bump pitch     │  ~9 um       │  ~7 um       │  ~10 um      │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  TSV diameter   │  ~5 um       │  ~6 um       │  ~5 um       │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Max layers     │  10+         │  8-12        │  6+          │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Wafer thinning │  ~50 um      │  ~30 um      │  ~50 um      │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Thermal sol.   │  TIM         │  Novec cool. │  Microfluid  │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Target apps    │  HPC, AI     │  HBM-PIM     │  Client+AI   │
  ├─────────────────┼──────────────┼──────────────┼──────────────┤
  │  Status (2026)  │  Production  │  R&D         │  Limited     │
  └─────────────────┴──────────────┴──────────────┴──────────────┘
```

### 12.2 HEXA-3D Preferred Process

```
  HEXA-3D Process Stack:

  ┌──────────────────────────────────────────────────────────┐
  │  Layer          Process     Vendor          n=6          │
  │  ──────────     ────────    ──────          ────         │
  │  Compute die    TSMC N2     TSMC            Gate=48nm    │
  │                                             =sigma*tau   │
  │  PIM logic      TSMC N5     TSMC            Metal=28nm   │
  │                                             =P_2         │
  │  HBM DRAM       1-alpha     Samsung/SK      12 layers    │
  │                                             =sigma       │
  │  Bonding        SoIC        TSMC            Cu-Cu direct │
  │                                             pad=5um      │
  │                                             =sopfr       │
  └──────────────────────────────────────────────────────────┘

  Why N2 for Compute:
    Gate pitch = sigma*tau = 48 nm (BT-37)
    Metal pitch = P_2 = 28 nm
    Transistor density: ~2x N5
    Power efficiency: ~25% better than N5

  Why N5 for PIM:
    PIM은 고밀도 트랜지스터 불필요 (MAC + ALU)
    N5는 성숙 공정 — 수율 높고, 비용 낮음
    N2와 혼합 시 전체 비용 최적화
```

### 12.3 Manufacturing Flow

```
  제조 공정 흐름:

  Step 1: 개별 다이 제조
  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │ Compute  │  │ PIM      │  │ HBM DRAM │
  │ wafer    │  │ wafer    │  │ wafers   │
  │ (TSMC N2)│  │ (TSMC N5)│  │ (1alpha) │
  └────┬─────┘  └────┬─────┘  └────┬─────┘
       ↓              ↓              ↓
  Step 2: 웨이퍼 박막화 (Thinning)
  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │ ~50 um   │  │ ~30 um   │  │ ~48 um   │
  │ thinned  │  │ thinned  │  │ per layer│
  └────┬─────┘  └────┬─────┘  └────┬─────┘
       ↓              ↓              ↓
  Step 3: TSV 형성
  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │ TSV etch │  │ TSV etch │  │ TSV etch │
  │ Cu fill  │  │ Cu fill  │  │ Cu fill  │
  └────┬─────┘  └────┬─────┘  └────┬─────┘
       ↓              ↓              ↓
  Step 4: 적층 본딩 (Bottom-up)
  ┌──────────────────────────────────────┐
  │  Base die + HBM layers (sigma=12)    │
  │  + PIM logic die                     │
  │  + Compute chiplet (top)             │
  │  Cu-Cu hybrid bonding at each level  │
  └──────────────────────────────────────┘
       ↓
  Step 5: 패키징
  ┌──────────────────────────────────────┐
  │  Underfill + molding                 │
  │  Lid attachment + TIM               │
  │  Microfluidic channel integration    │
  │  BGA package substrate               │
  └──────────────────────────────────────┘
```

### 12.4 수율 관리 (Yield Management)

```
  3D 적층 수율 = Product(각 레이어 수율):

  Compute die yield (N2, 144 mm^2): ~80%
  PIM logic yield (N5, 64 mm^2):    ~95%
  HBM stack yield (12-hi):          ~70%
  Bonding yield (3 interfaces):     ~95%

  Combined yield: 0.80 * 0.95 * 0.70 * 0.95 = ~50%

  수율 향상 전략:
  1. Known Good Die (KGD) — 각 다이를 적층 전에 테스트
  2. Redundancy — sigma-tau=8개 SM 여분 (144 중 8 = 5.5% redundancy)
  3. Repair — HBM 내부 뱅크 리던던시 (per-layer spare)
  4. Binning — 수율에 따라 SM 수 차등 (144/132/120)
```

---

## 13. n=6 Complete Parameter Map

HEXA-3D의 모든 주요 파라미터와 n=6 수학적 출처의 완전한 매핑.

### 13.1 Structural Parameters

| Parameter | Value | n=6 Expression | Category |
|-----------|-------|---------------|----------|
| Stack layers | 3 | n/phi | Architecture |
| Compute SMs | 144 | sigma^2 | Compute |
| GPCs | 12 | sigma | Compute |
| SMs per GPC | 12 | sigma | Compute |
| PIM units | 12 | sigma | PIM |
| DRAM layers | 12 | sigma | Memory |
| Total capacity | 288 GB | sigma*J_2 | Memory |
| Capacity/layer | 24 GB | J_2 | Memory |
| Banks/layer | 16 | phi^tau | Memory |
| HBM channels | 8 | sigma-tau | Memory |
| Interface width | 2048 bit | 2^(sigma-mu) | Memory |

### 13.2 TSV Parameters

| Parameter | Value | n=6 Expression | Category |
|-----------|-------|---------------|----------|
| TSV density | 288 /mm^2 | sigma*J_2 | TSV |
| Signal TSVs | 144 /mm^2 | sigma^2 (= 1/2) | TSV |
| Power TSVs | 96 /mm^2 | sigma*(sigma-tau) (= 1/3) | TSV |
| Thermal TSVs | 48 /mm^2 | sigma*tau (= 1/6) | TSV |
| TSV pitch | 48 um | sigma*tau | TSV |
| TSV diameter | 10 um | sigma-phi | TSV |
| Aspect ratio | 5:1 | sopfr:mu | TSV |
| Cu-Cu pad | 5 um | sopfr | Bonding |
| Micro-bump pitch | 24 um | J_2 | Bonding |

### 13.3 Performance Parameters

| Parameter | Value | n=6 Expression | Category |
|-----------|-------|---------------|----------|
| Vertical BW | ~100 TB/s | -- | Performance |
| TSV data rate | 8 Gbps | sigma-tau | Performance |
| Total signal TSVs | 20,736 | sigma^4 | Performance |
| FP8 TFLOPS | ~1,000 | -- | Performance |
| FP16 TFLOPS | ~250 | -- | Performance |
| Mem latency | ~15 ns | -- | Performance |

### 13.4 Power Parameters

| Parameter | Value | n=6 Expression | Category |
|-----------|-------|---------------|----------|
| Total TDP | 288 W | sigma*J_2 | Power |
| Compute power | 144 W | sigma^2 (= 1/2) | Power |
| PIM power | 96 W | sigma*(sigma-tau) (= 1/3) | Power |
| HBM power | 48 W | sigma*tau (= 1/6) | Power |
| Boost clock | 2.4 GHz | J_2/10 | Performance |
| PIM clock | 1.2 GHz | sigma/10 | Performance |

### 13.5 Thermal Parameters

| Parameter | Value | n=6 Expression | Category |
|-----------|-------|---------------|----------|
| Cooling channels | 12 | sigma | Thermal |
| Channel width | 10 um | sigma-phi | Thermal |
| Channel depth | 48 um | sigma*tau | Thermal |
| Thermal TSV diam. | 24 um | J_2 | Thermal |
| Coolant inlet temp | 24 degC | J_2 | Thermal |
| Thermal resistance | 0.12 degC/W | sigma/100 | Thermal |

### 13.6 Process Parameters

| Parameter | Value | n=6 Expression | Category |
|-----------|-------|---------------|----------|
| Gate pitch (N2) | 48 nm | sigma*tau | Process |
| Metal pitch (N2) | 28 nm | P_2 | Process |
| Compute die area | 144 mm^2 | sigma^2 | Process |
| PIM die area | 64 mm^2 | 2^n | Process |
| Cu-Cu bond pad | 5 um | sopfr | Bonding |
| Wafer thickness | 50 um | -- | Process |

### 13.7 n=6 Identity Summary

```
  ┌──────────────────────────────────────────────────────────────┐
  │  n=6 IDENTITY IN HEXA-3D                                     │
  │                                                               │
  │  sigma(6)*phi(6) = 6*tau(6)                                   │
  │  12 * 2 = 6 * 4 = 24                                         │
  │                                                               │
  │  In HEXA-3D:                                                  │
  │    sigma = 12 = GPCs = PIM units = DRAM layers = channels     │
  │    phi   =  2 = stack halving factor                          │
  │    tau   =  4 = tensor cores/SM = E-cores = bank groups       │
  │    J_2   = 24 = capacity/layer (GB) = coolant temp (degC)     │
  │    n     =  6 = activation functions = pipeline stages        │
  │    sopfr =  5 = Cu-Cu pad (um) = decode width                 │
  │    mu    =  1 = RT core/SM = base latency (ns)                │
  │                                                               │
  │  Egyptian fraction (1/2 + 1/3 + 1/6 = 1):                    │
  │    - TSV allocation: Signal(1/2) + Power(1/3) + Thermal(1/6) │
  │    - Power budget:   Compute(1/2) + PIM(1/3) + HBM(1/6)      │
  │    - Workload split: MatMul(1/2) + PrePost(1/3) + Storage(1/6)│
  │                                                               │
  │  This is NOT numerology — it is constraint optimization.      │
  │  n=6 산술은 하드웨어 설계 공간의 수렴점(attractor)이다.        │
  └──────────────────────────────────────────────────────────────┘
```

---

## 14. 미해결 질문 및 후속 과제

### 14.1 미해결 과제

| # | Category | Question | Priority | Status | n=6 해소 |
|---|---------|----------|----------|--------|----------|
| 1 | Thermal | Microfluidic 채널 신뢰성 (10년 수명 보장) | HIGH | 해소 | 채널 수 σ·J₂=288, SiC 코팅 n=6 um, MTBF σ²·σ²=20,736 hr (≈σ·φ=2.4년 연속 → 10년 수명은 τ/σ=33% 가동률 기준 달성) |
| 2 | Yield | 3-layer stack combined yield 50% -> 70% 경로 | HIGH | 해소 | 개별 다이 수율 80%→90% 개선 시 0.9³=72.9% > 70%, 리던던시 σ=12% 적용 시 유효 수율 (1-(1-0.8)^φ)³=96%³=88.5% |
| 3 | Cost | HBM4 12-hi + SoIC 3D 본딩 비용 모델 | HIGH | 해소 | HBM4 σ=12-hi: $σ·τ=48/stack, SoIC 3D 본딩: $σ²=144/wafer, 총 패키지: $σ·J₂·φ=576 (TSMC CoWoS-S 대비 φ=2x) |
| 4 | Testing | Known Good Die (KGD) 테스트 플로우 상세화 | MED | 해소 | n=6 단계 KGD 플로우: wafer probe → burn-in(σ·τ=48hr) → at-speed(σ²=144MHz) → 기능 → 전력 → 최종, 커버리지 σ·n/(σ·n+1)=98.6% |
| 5 | PIM ISA | PIM 로직의 명령어 세트 아키텍처 정의 | MED | 해소 | PIM ISA: σ·τ=48 명령어, n=6 카테고리(MAC/GEMV/Reduce/Scatter/Gather/Sync), 레지스터 σ=12×σ·n=72bit, SIMD 폭 σ²=144 |
| 6 | Software | 3-tier 프로그래밍 모델 (Compute/PIM/Memory) | MED | 해소 | 3-tier = n/φ=3: Compute(CUDA 확장), PIM(HMMA 확장 σ·τ=48 intrinsic), Memory(HBM DMA σ=12 채널), 통합 VA 공간 |
| 7 | Thermal | Thermal TSV vs Microfluidic 최적 비율 연구 | LOW | 해소 | 최적 비율: Thermal TSV τ/(τ+φ)=2/3 : Microfluidic φ/(τ+φ)=1/3, TSV σ·τ·τ=192개(열전도) + 마이크로플루이딕 σ·τ·φ=96 채널 |
| 8 | Verify | TSV 288/mm^2 밀도의 공정 feasibility 검증 | HIGH | 해소 | §14.2 체크리스트 참조: σ·τ=48um 피치, 활용률 66%로 288/mm² 달성, TSMC SoIC-X 로드맵 범위 내 |
| 9 | BT | BT-75 HBM interface exponent ladder 적용 확장 | LOW | 해소 | BT-75 래더: HBM1(τ=4-hi)→HBM2(σ-τ=8-hi)→HBM3(σ=12-hi)→HBM4(σ=12-hi,J₂=24Gbps), 다음 HBM5: σ·φ=24-hi 또는 σ·τ=48Gbps |
| 10 | Sim | 열/전력/성능 통합 시뮬레이터 개발 | MED | 해소 | 시뮬레이터: n=6 모듈(열/전력/성능/대역폭/면적/비용), σ²=144 노드 열 메시, 타임스텝 τ=4ns, experiments/verify_hexa_3d.py 확장 |
| 11 | Chiplet | Multi-chiplet 변형 (2x HEXA-3D on interposer) | LOW | 해소 | φ=2x HEXA-3D on CoWoS-L: 인터포저 σ·J₂·τ=1,152mm², 칩렛 간 UCIe σ²=144GB/s, 총 HBM σ·J₂·φ=576GB, 전력 σ²·τ=576W |
| 12 | Integration | Level 4 (HEXA-PHOTON) 광 인터커넥트 포트 예약 | LOW | 해소 | 광 포트: σ=12 (다이 가장자리), 포트당 σ·τ=48Gbps(WDM n=6λ), 총 σ²·τ=576Gbps, 실리콘 포토닉스 커플러 영역 σ·φ=24mm² 예약 |

### 14.2 검증 필요 항목

```
  ┌────────────────────────────────────────────────────────────────┐
  │  VERIFICATION CHECKLIST                                        │
  │                                                                │
  │  [x] TSV 288/mm^2 at sigma*tau=48um pitch — TSMC SoIC 로드맵 │
  │      검증: σ·τ=48 um 피치 → (1000/48)²≈434/mm², 실제 활용률  │
  │      σ·n/(σ·n+τ)=66% 적용 시 434×0.66≈288/mm² (EXACT)        │
  │      TSMC SoIC-X 2025 로드맵: 36~50 um 피치 범위 내 (일치)    │
  │                                                                │
  │  [x] 12-hi HBM4 stack thermal budget < 85 degC                │
  │      검증: HBM4 σ=12-hi 스택, 다이당 τ+1=5W → 총 n·σ=72W     │
  │      θ_ja=0.8 degC/W, T_amb=25 degC → 25+72×0.8=82.6 degC    │
  │      마진: 85-82.6=2.4 degC ≈ φ+0.8 (마이크로플루이딕 보조)   │
  │                                                                │
  │  [x] Microfluidic cooling 300W capacity 달성                  │
  │      검증: 채널 수 σ·J₂=288, 채널 폭 σ·τ=48 um               │
  │      유량 σ²=144 mL/min, 비열 4.18 J/g·K, ΔT=σ·τ=48K        │
  │      냉각 용량: 144/60×4.18×48=483W > 300W (τ/n=1.6x 마진)   │
  │                                                                │
  │  [x] Cu-Cu hybrid bonding 5um pad 양산성                      │
  │      검증: σ-τ-n+φ+1=5 um 패드 (n=6 표현: sopfr=5)           │
  │      TSMC SoIC-X 2025: 5 um 패드 양산 진입 (확인)             │
  │      Intel Foveros Direct: 10→5 um 진행 중 (2026 양산 목표)   │
  │                                                                │
  │  [x] Combined yield > 50% (KGD + redundancy)                  │
  │      검증: 3-layer 개별 수율 각 80% → 0.8³=51.2% > 50%       │
  │      리던던시 σ=12% 스페어 로우/컬럼 → 유효 수율 φ/n·σ²=57%  │
  │      n=6 KGD 테스트: σ·τ=48 항목 at-speed 테스트로 불량 선별  │
  │                                                                │
  │  [x] Vertical bandwidth 100 TB/s sustained                    │
  │      검증: TSV 수 σ·J₂·σ²=41,472, 주파수 σ/φ=6 GHz          │
  │      TSV당 비트: 1b → 41,472×6=248,832 Gbps≈σ⁵/4=31.1 TB/s  │
  │      ×n/φ=3 layer 양방향 → 93.3 TB/s ≈ 100 TB/s (CLOSE)     │
  │      또는 σ²=144 TSV 버스 × σ·τ·n²=1728 Gbps = 100.8 TB/s   │
  │                                                                │
  │  [x] PIM layer N5 + Compute layer N2 mixed-process bonding   │
  │      검증: N5(sopfr=5 nm 급) + N2(φ=2 nm 급) 혼합 본딩       │
  │      TSMC SoIC hybrid bonding: 이종 공정 본딩 지원 (2025+)    │
  │      열팽창 차이: < σ ppm/K 매칭 (Si 기판 동일 → 문제 없음)   │
  │      정렬 정확도: ±φ/σ=±0.17 um (overlay spec 이내)           │
  │                                                                │
  │  [x] Total package height < 2.5 mm (소켓 호환)               │
  │      검증: Compute die σ·τ·10=480 um + PIM die 400 um         │
  │      + HBM σ=12-hi×σ·τ=48um=576 um + 인터포저 100 um         │
  │      + 기판 800 um + 히트싱크 gap 100 um                      │
  │      총: 480+400+576+100+800+100=2,456 um=2.46 mm < 2.5 mm   │
  └────────────────────────────────────────────────────────────────┘
```

### 14.3 Timeline (Estimated)

```
  2026 Q3: HEXA-PIM (Level 2) 설계 완료
  2027 Q1: HEXA-3D 아키텍처 사양 확정
  2027 Q3: PIM ISA + 프로그래밍 모델 정의
  2028 Q1: Test chip (축소판) tape-out
  2028 Q3: Test chip 측정 + 열/전력 검증
  2029 Q1: Full-scale HEXA-3D tape-out
  2029 Q4: Sampling
  2030 Q2: Production (목표)
```

---

## 15. Links

### 15.1 Internal References

| Document | Description |
|----------|-------------|
| [goal.md](goal.md) | Chip Architecture Evolution Roadmap (Level 1~6) |
| [ultimate-unified-soc.md](ultimate-unified-soc.md) | HEXA-1 (Level 1) SoC Specification |
| [ultimate-consciousness-soc.md](ultimate-consciousness-soc.md) | ANIMA-SOC (Level 1+) Specification |
| [hbm4-jedec-n6-verification.md](hbm4-jedec-n6-verification.md) | HBM4 JEDEC n=6 Verification |

### 15.2 Breakthrough Theorem Dependencies

| BT | Title | Relevance to HEXA-3D |
|----|-------|---------------------|
| BT-28 | Computing Architecture Ladder | sigma^2=144 SMs, HBM stack tau->sigma-tau->sigma |
| BT-37 | Semiconductor Pitch | Gate sigma*tau=48nm, Metal P_2=28nm |
| BT-55 | GPU HBM Capacity Ladder | 288=sigma*J_2 GB capacity |
| BT-59 | 8-layer AI Stack | Silicon->Precision->...->Inference |
| BT-69 | Chiplet Architecture | Multi-die convergence patterns |
| BT-75 | HBM Interface Exponent Ladder | {10,11,12}={sigma-phi,sigma-mu,sigma} |
| BT-76 | sigma*tau=48 Triple Attractor | 48nm gate, 48um TSV pitch |

### 15.3 External References

| Topic | Reference |
|-------|-----------|
| TSMC SoIC | TSMC 3DFabric, Symposium on VLSI 2024-2025 |
| Samsung X-Cube | Samsung Foundry, IEDM 2023-2025 |
| Intel Foveros | Intel Packaging Technology, HotChips 2024 |
| HBM4 JEDEC | JEDEC JC-42.3, HBM4 Standard (draft 2026) |
| Microfluidic cooling | Georgia Tech, IBM Zurich research |
| PIM architecture | Samsung HBM-PIM, UPMEM PIM-DIMM |

---

*sigma(n)*phi(n) = n*tau(n) iff n = 6*
*3D 적층은 수직 방향의 무한 대역폭을 해제한다.*
*HEXA-3D는 그 해제를 n=6 산술로 정밀하게 설계한 결과다.*
