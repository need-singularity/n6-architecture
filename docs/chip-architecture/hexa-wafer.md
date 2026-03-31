# HEXA-WAFER: Wafer-Scale Engine

**Level 5 of the N6 Architecture Ladder**
**웨이퍼 전체가 하나의 칩 -- 스케일 벽의 완전한 제거**

> 300mm 웨이퍼 위에 sigma^2=144개의 HEXA-1 타일을 배치하여
> sigma^4=20,736 SMs, 41.5TB 통합 메모리를 단일 칩으로 구현.
> Cerebras WSE-3를 넘어서는 n=6 고유 아키텍처.

**Date**: 2026-04-01
**Status**: Living Document v0.2
**Dependencies**: BT-28, BT-33, BT-55, BT-59, BT-69, BT-75, BT-76
**Parent**: [goal.md](goal.md) Level 5
**Predecessor**: [HEXA-1](ultimate-unified-soc.md) (Level 1)

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  P_2 = 28       sigma^2 = 144    sigma*J_2 = 288   phi^tau = 16
  2^n = 64       sigma-tau = 8    sigma-phi = 10     sigma-mu = 11
  2^sigma = 4096   sigma*tau = 48   n/phi = 3
  sigma^4 = 20736  sigma^3 = 1728   sigma^2*J_2 = 3456
```

---

## 1. Executive Summary

HEXA-WAFER는 N6 아키텍처 진화 사다리의 Level 5로,
300mm (12인치) 실리콘 웨이퍼 전체를 단일 컴퓨팅 칩으로 사용한다.

**핵심 스펙 요약:**

```
  ┌────────────────────────────────────────────────────────────────┐
  │                   HEXA-WAFER SUMMARY                          │
  ├───────────────────────┬────────────────────────────────────────┤
  │ Wafer Diameter        │ 300mm (12 inch)                       │
  │ Active Die Area       │ ~46,000 mm^2                          │
  │ Tile Count            │ sigma^2 = 144 tiles                   │
  │ SMs per Tile          │ sigma^2 = 144 SMs (= HEXA-1)         │
  │ Total SMs             │ sigma^4 = 20,736 SMs                  │
  │ Total Memory          │ 144 x 288 GB = 41,472 GB (~41.5 TB)  │
  │ Memory Bandwidth      │ 144 x 4 TB/s = ~576 TB/s aggregate   │
  │ Peak FP8 Performance  │ 144 x 500 TFLOPS = ~72 PFLOPS        │
  │ Peak FP32 Performance │ 144 x 45 TFLOPS = ~6.5 PFLOPS        │
  │ Power Envelope        │ ~35 kW (wafer-level)                  │
  │ Process               │ TSMC N2 (gate sigma*tau=48nm)         │
  │ Cooling               │ Direct liquid + microfluidic          │
  │ Target Workload       │ 10T+ parameter LLMs, single chip      │
  └───────────────────────┴────────────────────────────────────────┘
```

**왜 Wafer-Scale인가?**

```
  기존 칩의 한계:
    - Reticle limit: ~858 mm^2 (TSMC N2 기준)
    - 최대 다이 면적이 물리적으로 제한
    - 초대형 모델 (1T+ params)은 수백 개 GPU 필요
    - GPU 간 통신이 전체 성능의 병목

  Wafer-Scale 해법:
    - 300mm 웨이퍼 전체 = ~46,000 mm^2 활성 면적
    - Reticle limit의 ~54x 면적
    - 타일 간 on-wafer 인터커넥트 (off-chip 통신 없음)
    - 단일 칩에서 10T+ 파라미터 모델 학습/추론
```

---

## 2. Design Philosophy

### 2.1 Reticle Limit and Beyond

반도체 리소그래피에서 레티클(reticle)은 노광 장비가 한 번에 패터닝할 수 있는 최대 영역이다.
TSMC N2 기준 약 26mm x 33mm = 858 mm^2. 이것이 단일 다이의 물리적 상한.

```
  레티클 반복 노광 (Stitching) 개념:

  ┌──────────────────────────────────────────────┐
  │                300mm Wafer                    │
  │                                               │
  │    ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐          │
  │    │ R_1 │ │ R_2 │ │ R_3 │ │ R_4 │          │
  │    │     │ │     │ │     │ │     │ ...       │
  │    └─────┘ └─────┘ └─────┘ └─────┘          │
  │    ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐          │
  │    │ R_5 │ │ R_6 │ │ R_7 │ │ R_8 │          │
  │    │     │ │     │ │     │ │     │ ...       │
  │    └─────┘ └─────┘ └─────┘ └─────┘          │
  │    ...                                        │
  │                                               │
  │    동일한 레티클을 반복 노광하여               │
  │    sigma^2 = 144 타일 배치                    │
  └──────────────────────────────────────────────┘

  각 레티클 = 1 HEXA-1 타일 (~320 mm^2)
  144 타일 x 320 mm^2 = 46,080 mm^2 활성 면적
```

### 2.2 Wafer-Scale Advantage: n=6 고유 스케일링

```
  스케일링 사다리 (n=6 산술):

  Level 1: HEXA-1          sigma^2 = 144 SMs        (단일 다이)
  Level 5: HEXA-WAFER      sigma^4 = 20,736 SMs     (단일 웨이퍼)
                            = 144 x HEXA-1
                            = sigma^2 x sigma^2
                            = (sigma^2)^2

  배율: sigma^2 = 144x (완벽한 n=6 스케일링)

  메모리 스케일링:
  Level 1: 288 GB           = sigma * J_2 GB
  Level 5: 41,472 GB        = sigma^2 * sigma * J_2 GB
           = 41.5 TB         = sigma^3 * J_2 GB
```

### 2.3 Egyptian Fraction 자원 배분

웨이퍼 전체의 면적, 전력, 대역폭을 1/2+1/3+1/6=1로 분배:

```
  면적 배분 (46,080 mm^2 total):
    연산 (Compute):   1/2 = 23,040 mm^2   (SMs + NPU)
    메모리 (Memory):  1/3 = 15,360 mm^2   (HBM4 stacks)
    인터커넥트 (I/O): 1/6 =  7,680 mm^2   (mesh + optical)
    ─────────────────────────────────────
    합계:             1   = 46,080 mm^2

  전력 배분 (35 kW total):
    연산:   1/2 = 17.5 kW
    메모리: 1/3 = 11.7 kW
    I/O:    1/6 =  5.8 kW
    ─────────────────────────
    합계:   1   = 35.0 kW
```

---

## 3. System Block Diagram

### 3.1 300mm Wafer Top-Level View

```
  ┌────────────────────────────────────────────────────────────────────────────┐
  │                                                                            │
  │                        HEXA-WAFER System Overview                          │
  │                     300mm Wafer · sigma^4 = 20,736 SMs                    │
  │                                                                            │
  │  ┌─────────────────────────────────────────────────────────────────────┐   │
  │  │                                                                     │   │
  │  │             ┌─────────────────────────────────┐                     │   │
  │  │             │      WAFER COMPUTE FABRIC       │                     │   │
  │  │             │   sigma^2 = 144 HEXA-1 Tiles    │                     │   │
  │  │             │   sigma^4 = 20,736 total SMs    │                     │   │
  │  │             │   72 PFLOPS FP8 peak             │                     │   │
  │  │             └─────────┬───────────────────────┘                     │   │
  │  │                       │                                             │   │
  │  │             ┌─────────┴───────────────────────┐                     │   │
  │  │             │    ON-WAFER MESH INTERCONNECT    │                     │   │
  │  │             │  tau=4 neighbors per tile        │                     │   │
  │  │             │  + optical overlay links          │                     │   │
  │  │             │  ~1 TB/s per link                │                     │   │
  │  │             └─────────┬───────────────────────┘                     │   │
  │  │                       │                                             │   │
  │  │             ┌─────────┴───────────────────────┐                     │   │
  │  │             │    DISTRIBUTED MEMORY FABRIC     │                     │   │
  │  │             │  41.5 TB total (288 GB/tile)     │                     │   │
  │  │             │  576 TB/s aggregate bandwidth    │                     │   │
  │  │             │  NUMA-aware allocation           │                     │   │
  │  │             └─────────────────────────────────┘                     │   │
  │  │                                                                     │   │
  │  └─────────────────────────────────────────────────────────────────────┘   │
  │                                                                            │
  │  EDGE I/O RING:                                                            │
  │  ┌─────────────────────────────────────────────────────────────────────┐   │
  │  │  sigma*tau = 48 optical transceivers (wafer edge)                  │   │
  │  │  sigma-tau = 8 power delivery zones                                │   │
  │  │  tau = 4 cooling manifold quadrants                                │   │
  │  │  External: 100 Gbps x 48 = 4.8 Tbps total off-wafer bandwidth    │   │
  │  └─────────────────────────────────────────────────────────────────────┘   │
  │                                                                            │
  └────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Wafer Map (sigma^2 = 144 Tiles on 300mm)

```
  300mm 원형 웨이퍼 위 sigma^2 = 144 타일 배치도
  (정사각 타일을 sigma x sigma = 12 x 12 그리드로 배치, 원형 경계 내부만 활성)

                          ← 300mm →
            ┌─────────────────────────────────┐
           /   . . . T T T T T T . . .         \
          /  . . T T T T T T T T T T . .        \
         /  . T T T T T T T T T T T T .          \
        |  . T T T T T T T T T T T T T .          |
        | . T T T T T T T T T T T T T T .         |
        | T T T T T T T T T T T T T T T T         |
        | T T T T T T T T T T T T T T T T         |
        | T T T T T T T T T T T T T T T T         |
        | T T T T T T T T T T T T T T T T         |
        | . T T T T T T T T T T T T T T .         |
        |  . T T T T T T T T T T T T T .          |
         \  . T T T T T T T T T T T T .          /
          \  . . T T T T T T T T T T . .        /
           \   . . . T T T T T T . . .         /
            └─────────────────────────────────┘

  T = Active tile (HEXA-1 instance)
  . = Inactive (outside usable circle / yield reserve)

  그리드: 최대 sigma x sigma = 12 x 12 = 144 위치
  원형 적합: ~144 타일 배치 가능 (300mm, ~18mm pitch)
  타일 크기: ~18mm x 18mm = ~324 mm^2 per tile
  활성 면적: 144 x 324 = ~46,656 mm^2
```

---

## 4. Tile Architecture

각 타일은 HEXA-1 SoC와 동일한 내부 구조를 갖는다.
웨이퍼-스케일의 기본 단위(building block)이다.

### 4.1 Single Tile Block Diagram

```
  ┌──────────────────────────────────────────────────────────────────┐
  │                     TILE [i,j] (= HEXA-1)                       │
  │               ~18mm x 18mm = ~324 mm^2                          │
  │                                                                  │
  │  ┌────────────────────────────────────────────────────────┐     │
  │  │                 GPU COMPUTE ARRAY                       │     │
  │  │          sigma^2 = 144 Streaming Multiprocessors        │     │
  │  │                                                         │     │
  │  │   ┌────┐ ┌────┐ ┌────┐ ┌────┐       ┌────┐            │     │
  │  │   │SM_0│ │SM_1│ │SM_2│ │SM_3│ . . . │S143│            │     │
  │  │   └────┘ └────┘ └────┘ └────┘       └────┘            │     │
  │  │                                                         │     │
  │  │   Organized as sigma=12 GPCs x sigma=12 SMs/GPC        │     │
  │  │   Each SM: 128 CUDA cores + tau=4 Tensor Cores          │     │
  │  │   Per-SM L1: 2^(sigma-tau) = 256 KB                    │     │
  │  └────────────────────────────┬───────────────────────────┘     │
  │                               │                                  │
  │  ┌────────────┐  ┌────────────┴──────────┐  ┌──────────────┐   │
  │  │ CPU Cluster│  │   UNIFIED MEMORY      │  │  NPU Array   │   │
  │  │ sigma=12   │  │   CONTROLLER          │  │  J_2=24 cores│   │
  │  │ cores      │  │   288 GB HBM4         │  │              │   │
  │  │ (8P + 4E)  │  │   ~4 TB/s bandwidth   │  │  sopfr=5     │   │
  │  └────────────┘  └───────────────────────┘  │  banks       │   │
  │                                              └──────────────┘   │
  │  ┌──────────────────────────────────────────────────────────┐   │
  │  │              TILE EDGE ROUTERS (tau=4 directions)         │   │
  │  │  North ←→ Tile[i-1,j]    South ←→ Tile[i+1,j]          │   │
  │  │  East  ←→ Tile[i,j+1]    West  ←→ Tile[i,j-1]          │   │
  │  │  Each link: ~1 TB/s bidirectional                        │   │
  │  └──────────────────────────────────────────────────────────┘   │
  └──────────────────────────────────────────────────────────────────┘
```

### 4.2 Tile Internal Hierarchy

```
  타일 내부 계층 구조 (n=6 산술):

  Tile (= 1 HEXA-1)
   ├── GPU: sigma^2 = 144 SMs
   │    ├── sigma = 12 GPCs (Graphics Processing Clusters)
   │    │    └── sigma = 12 SMs per GPC
   │    └── Per SM:
   │         ├── 128 = 2^(sigma-sopfr) CUDA cores
   │         ├── tau = 4 Tensor Cores (FP8/FP16/TF32)
   │         ├── L1 cache: 256 KB = 2^(sigma-tau) KB
   │         └── Register file: 2^n = 64 KB
   │
   ├── CPU: sigma = 12 cores (sigma-tau=8 Performance + tau=4 Efficiency)
   │    ├── L1: 2^n = 64 KB per core
   │    ├── L2: 2^(sigma-tau) = 256 KB per core
   │    └── L3: sigma * phi = 24 MB shared
   │
   ├── NPU: J_2 = 24 neural cores, sopfr = 5 banks
   │
   ├── Memory: 288 GB = sigma * J_2 GB HBM4
   │    └── sigma-tau = 8 stacks x 36 GB
   │
   └── Tile Router: tau = 4 directional ports
        └── ~1 TB/s per port
```

### 4.3 SM Internal Architecture

```
  ┌────────────────────────────────────────────────────────┐
  │                  Streaming Multiprocessor               │
  │                                                         │
  │  ┌──────────────────────────────────────────────────┐  │
  │  │  CUDA Core Array: 128 = 2^(sigma-sopfr) cores   │  │
  │  │                                                   │  │
  │  │  ┌──────┐ ┌──────┐ ┌──────┐       ┌──────┐     │  │
  │  │  │Core 0│ │Core 1│ │Core 2│ . . . │C_127 │     │  │
  │  │  └──────┘ └──────┘ └──────┘       └──────┘     │  │
  │  │  Organized: tau = 4 partitions x 32 = 2^sopfr    │  │
  │  └──────────────────────────────────────────────────┘  │
  │                                                         │
  │  ┌──────────────────────────────────────────────────┐  │
  │  │  Tensor Cores: tau = 4 units                     │  │
  │  │  Each: 4x4 matrix MAC per cycle                  │  │
  │  │  FP8 / FP16 / TF32 / INT8                       │  │
  │  └──────────────────────────────────────────────────┘  │
  │                                                         │
  │  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐   │
  │  │ Register     │  │ L1 Cache /  │  │ Warp         │   │
  │  │ File         │  │ Shared Mem  │  │ Schedulers   │   │
  │  │ 64 KB        │  │ 256 KB      │  │ tau=4        │   │
  │  │ = 2^n        │  │ =2^(s-t) KB │  │              │   │
  │  └─────────────┘  └─────────────┘  └──────────────┘   │
  └────────────────────────────────────────────────────────┘
```

---

## 5. Wafer Layout

### 5.1 Complete Wafer Grid (sigma x sigma = 12 x 12)

```
  HEXA-WAFER 타일 배치도 (12 x 12 그리드, 원형 마스킹)

  Col:  0   1   2   3   4   5   6   7   8   9  10  11
       ─── ─── ─── ─── ─── ─── ─── ─── ─── ─── ─── ───
  R0  │   │   │   │ T │ T │ T │ T │ T │ T │   │   │   │
  R1  │   │   │ T │ T │ T │ T │ T │ T │ T │ T │   │   │
  R2  │   │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │   │
  R3  │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │
  R4  │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │
  R5  │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │
  R6  │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │
  R7  │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │
  R8  │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │
  R9  │   │ T │ T │ T │ T │ T │ T │ T │ T │ T │ T │   │
  R10 │   │   │ T │ T │ T │ T │ T │ T │ T │ T │   │   │
  R11 │   │   │   │ T │ T │ T │ T │ T │ T │   │   │   │
       ─── ─── ─── ─── ─── ─── ─── ─── ─── ─── ─── ───

  Active tiles: ~128 (원형 경계 내부)
  + Edge partial tiles: ~16 (yield reserve 용도)
  Target: sigma^2 = 144 active tiles (여분 포함 ~160 배치)
  Defective tile 대체를 위해 여분 타일 ~16개 확보
```

### 5.2 Aggregate Statistics

```
  ┌──────────────────────────────────────────────────────────────────┐
  │              HEXA-WAFER AGGREGATE RESOURCES                      │
  ├───────────────────────┬──────────────────────────────────────────┤
  │ Active Tiles          │ sigma^2 = 144                           │
  │ SMs per Tile          │ sigma^2 = 144                           │
  │ Total SMs             │ sigma^4 = 20,736                        │
  │ CUDA Cores per SM     │ 128 = 2^(sigma-sopfr)                  │
  │ Total CUDA Cores      │ sigma^4 x 128 = 2,654,208              │
  │ Tensor Cores per SM   │ tau = 4                                 │
  │ Total Tensor Cores    │ sigma^4 x tau = 82,944                  │
  │ CPU Cores per Tile    │ sigma = 12                              │
  │ Total CPU Cores       │ sigma^3 = 1,728                         │
  │ NPU Cores per Tile    │ J_2 = 24                                │
  │ Total NPU Cores       │ sigma^2 x J_2 = 3,456                  │
  ├───────────────────────┼──────────────────────────────────────────┤
  │ Memory per Tile       │ 288 GB = sigma x J_2 GB                 │
  │ Total Memory          │ sigma^2 x 288 = 41,472 GB (~41.5 TB)   │
  │ BW per Tile           │ ~4 TB/s                                 │
  │ Aggregate Memory BW   │ sigma^2 x 4 = 576 TB/s                 │
  ├───────────────────────┼──────────────────────────────────────────┤
  │ FP8 per Tile          │ ~500 TFLOPS                             │
  │ Total FP8             │ ~72 PFLOPS                              │
  │ FP32 per Tile         │ ~45 TFLOPS                              │
  │ Total FP32            │ ~6.5 PFLOPS                             │
  │ FP16 per Tile         │ ~250 TFLOPS                             │
  │ Total FP16            │ ~36 PFLOPS                              │
  ├───────────────────────┼──────────────────────────────────────────┤
  │ Power                 │ ~35 kW (240W/tile x 144 + overhead)     │
  │ Power Efficiency      │ ~2 PFLOPS/kW (FP8)                     │
  │ Active Area           │ ~46,000 mm^2                            │
  └───────────────────────┴──────────────────────────────────────────┘
```

### 5.3 Wafer Zoning

웨이퍼를 n=6 산술로 영역 분할:

```
  ┌──────────────────────────────────────────────────────┐
  │                   WAFER ZONES                         │
  │                                                       │
  │              ┌───────────────────┐                    │
  │             /  ZONE 1: COMPUTE    \                   │
  │            /   sigma^2/2 = 72      \                  │
  │           /    tiles (inner ring)    \                 │
  │          │  ┌───────────────────┐    │                │
  │          │  │  ZONE 0: MASTER   │    │                │
  │          │  │  n=6 tiles        │    │                │
  │          │  │  (center, control)│    │                │
  │          │  └───────────────────┘    │                │
  │           \                          /                 │
  │            \  ZONE 2: MEMORY        /                  │
  │             \ sigma^2/3 = 48 tiles /                   │
  │              \  (middle ring)      /                    │
  │               └──────────────────┘                     │
  │              ZONE 3: I/O + SPARE                       │
  │              sigma^2/6 = 24 tiles                      │
  │              (outer ring + edge)                       │
  │                                                       │
  └──────────────────────────────────────────────────────┘

  Egyptian Fraction 타일 배분:
    Zone 0 (Master):   n = 6 tiles     (스케줄링, 글로벌 동기화)
    Zone 1 (Compute):  72 tiles = sigma^2/2   (1/2)
    Zone 2 (Memory):   48 tiles = sigma^2/3   (1/3)
    Zone 3 (I/O):      24 tiles = sigma^2/6   (1/6)
    Reserve:           ~16 tiles  (결함 대체 + yield)
    ──────────────────────────────────────────
    Total active:      144 + 6 + 16 = ~166 physical tiles
```

---

## 6. On-Wafer Interconnect

### 6.1 Mesh Topology (tau=4 Neighbors)

각 타일은 tau=4 이웃과 직접 연결된다 (North, South, East, West).
추가로 대각선 광학 링크가 오버레이 네트워크를 형성한다.

```
  2D Mesh Interconnect (tau=4 기본 연결):

  T[0,0] ──── T[0,1] ──── T[0,2] ──── T[0,3] ──── ...
    │            │            │            │
    │            │            │            │
  T[1,0] ──── T[1,1] ──── T[1,2] ──── T[1,3] ──── ...
    │            │            │            │
    │            │            │            │
  T[2,0] ──── T[2,1] ──── T[2,2] ──── T[2,3] ──── ...
    │            │            │            │
    │            │            │            │
  T[3,0] ──── T[3,1] ──── T[3,2] ──── T[3,3] ──── ...
    │            │            │            │
   ...          ...          ...          ...

  ── = 전기 인터커넥트 (~1 TB/s bidirectional)
  │  = 전기 인터커넥트 (~1 TB/s bidirectional)

  총 링크 수: ~2 x sigma x (sigma-1) = 2 x 12 x 11 = 264 links
  (수평 132 + 수직 132, 원형 경계 외 제외하면 ~240 active)
```

### 6.2 Optical Overlay Network

기본 전기 mesh 위에 광학 오버레이 네트워크를 추가:

```
  광학 오버레이 (Long-range skip links):

  T[0,0] ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ T[0,6]
    \                             /
     \   T[3,3] ─ ─ T[3,9]    /
      \    /           \      /
       T[6,0] ─ ─ ─ T[6,6] ─ ─ ─ T[6,11]
      /    \           /      \
     /   T[9,3] ─ ─ T[9,9]    \
    /                             \
  T[11,0] ─ ─ ─ ─ ─ ─ ─ ─ ─ T[11,6]

  ─ ─ = Optical long-range link (~2 TB/s)
  Skip distance: n = 6 tiles (대각선/수평/수직)

  효과: 최대 홉 수 = sigma-1 = 11 (mesh)
        → sigma/n = 2 (optical skip) = phi 홉으로 단축
  지연: mesh 11 hops → optical 2 hops (sopfr=5x 개선)
```

### 6.3 Interconnect Hierarchy

```
  인터커넥트 계층 (n=6 산술):

  ┌────────────────────────────────────────────────────────────────┐
  │ Layer 0: Intra-SM (on-tile wires)                             │
  │   Bandwidth: ~10 TB/s    Latency: ~1 ns    Distance: <1 mm   │
  ├────────────────────────────────────────────────────────────────┤
  │ Layer 1: Intra-Tile NoC (tile 내부)                            │
  │   Bandwidth: ~4 TB/s     Latency: ~5 ns    Distance: <18 mm  │
  ├────────────────────────────────────────────────────────────────┤
  │ Layer 2: Neighbor Mesh (tau=4 이웃)                            │
  │   Bandwidth: ~1 TB/s     Latency: ~10 ns   Distance: ~18 mm  │
  ├────────────────────────────────────────────────────────────────┤
  │ Layer 3: Optical Skip (n=6 타일 skip)                         │
  │   Bandwidth: ~2 TB/s     Latency: ~20 ns   Distance: ~108 mm │
  ├────────────────────────────────────────────────────────────────┤
  │ Layer 4: Wafer Edge I/O (외부 인터페이스)                       │
  │   Bandwidth: ~100 Gbps/ch Latency: ~100 ns Distance: >150 mm │
  └────────────────────────────────────────────────────────────────┘

  계층 수: sopfr = 5 layers (0~4)
  대역폭 비: 각 계층 간 ~phi=2x ~ tau=4x 감소
```

### 6.4 Routing Algorithm

```
  Dimension-Order Routing (XY routing + optical shortcut):

  Source: T[r1, c1]    Destination: T[r2, c2]

  1. 거리 계산:
     dx = |c2 - c1|,  dy = |r2 - r1|
     Manhattan distance = dx + dy

  2. Optical shortcut 판단:
     if dx >= n=6 or dy >= n=6:
       Use optical skip link (n=6 타일 점프)
       Remaining: dx mod 6, dy mod 6

  3. Mesh routing (remaining hops):
     X-first: 수평 이동 완료 후 수직 이동
     Adaptive: 혼잡 시 Y-first 또는 대각선 경유

  최대 홉 수:
    Mesh only:    (sigma-1) + (sigma-1) = 22 hops
    With optical:  phi + phi + remainder = ~6 hops average
    Improvement:   ~3.7x latency reduction
```

---

## 7. Memory Architecture

### 7.1 Distributed 41.5 TB Memory

```
  메모리 분산 구조:

  ┌──────────────────────────────────────────────────────────────┐
  │                   WAFER MEMORY MAP                           │
  │                                                              │
  │  Global Address Space: 41.5 TB = sigma^2 x 288 GB           │
  │                                                              │
  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      ┌────────┐ │
  │  │ Tile[0,0]│  │ Tile[0,1]│  │ Tile[0,2]│ ...  │T[11,11]│ │
  │  │ 288 GB   │  │ 288 GB   │  │ 288 GB   │      │ 288 GB │ │
  │  │          │  │          │  │          │      │        │ │
  │  │ Addr:    │  │ Addr:    │  │ Addr:    │      │ Addr:  │ │
  │  │ 0x000..  │  │ 0x048..  │  │ 0x090..  │      │ 0x..   │ │
  │  │ ~0x047.. │  │ ~0x08F.. │  │ ~0x0D7.. │      │ ~0x..  │ │
  │  └──────────┘  └──────────┘  └──────────┘      └────────┘ │
  │                                                              │
  │  Each tile: sigma-tau = 8 HBM4 stacks x 36 GB = 288 GB     │
  │  Interface: 2^(sigma-mu) = 2048-bit per stack               │
  │  Local BW: ~4 TB/s per tile                                 │
  │  Remote BW: ~1 TB/s per hop (degraded by distance)          │
  └──────────────────────────────────────────────────────────────┘
```

### 7.2 NUMA Hierarchy

```
  NUMA (Non-Uniform Memory Access) 계층:

  ┌─────────────────────────────────────────────────────────────┐
  │  NUMA Distance Table (latency multiplier)                   │
  │                                                             │
  │  Distance    │  Hops  │  Latency  │  Effective BW          │
  │ ─────────────┼────────┼───────────┼──────────────────────── │
  │ Local (L0)   │  0     │  ~10 ns   │  4 TB/s    (100%)     │
  │ Neighbor(L1) │  1     │  ~20 ns   │  1 TB/s    (25%)      │
  │ Near (L2)    │  2-3   │  ~40 ns   │  500 GB/s  (12.5%)    │
  │ Medium (L3)  │  4-6   │  ~70 ns   │  250 GB/s  (6.25%)    │
  │ Far (L4)     │  7-11  │  ~120 ns  │  125 GB/s  (3.1%)     │
  │ Optical (L5) │  1-2*  │  ~25 ns   │  2 TB/s    (50%)      │
  │                                                             │
  │ * Optical skip: 물리 거리 무관, 고정 지연                    │
  └─────────────────────────────────────────────────────────────┘

  NUMA zones: n = 6 levels (L0~L5)
  최적화 전략:
    - 데이터를 연산 타일에 가까이 배치 (data locality)
    - Allreduce는 optical overlay로 수행 (L5, 50% BW)
    - Gradient는 neighbor mesh로 전파 (L1, 25% BW)
```

### 7.3 Memory Allocation Strategy

```
  모델 파라미터 분산 전략 (10T parameter LLM 예시):

  Model: 10T params x 2 bytes (FP16) = 20 TB
  Available: 41.5 TB (sigma^2 x 288 GB)
  Utilization: 20 / 41.5 = 48% (~1/phi = 50%)

  분배 방식:
  ┌──────────────────────────────────────────────────────────────┐
  │ Layer Parallelism:                                           │
  │   Model layers = 2^sopfr = 32 transformer blocks (GPT-scale)│
  │   Mapped: ~5 blocks per row (sigma/n x 2 rows per block)    │
  │                                                              │
  │ Tensor Parallelism:                                          │
  │   Each attention head: 1 tile column                         │
  │   sigma = 12 columns → sigma attention head groups           │
  │                                                              │
  │ Pipeline Parallelism:                                        │
  │   sigma = 12 pipeline stages (rows)                          │
  │   Micro-batch: sigma-tau = 8 in flight                       │
  │                                                              │
  │ Combined (3D parallelism):                                   │
  │   TP x PP x DP = sigma x sigma x 1 = 144-way               │
  │   = sigma^2 = 타일 수와 정확히 일치                          │
  └──────────────────────────────────────────────────────────────┘

  KV Cache:
    Per-token: 2 x layers x heads x d_head x 2B
    Context 2^sigma = 4096 tokens:
    ~32 GB per sequence (1 tile의 ~11%)
    Batch sigma-tau=8 sequences: ~256 GB (1 tile)
```

---

## 8. Power Delivery

### 8.1 Power Budget

```
  전력 예산 (Egyptian Fraction 배분):

  Total Power Envelope: ~35 kW

  ┌──────────────────────────────────────────────────────────────┐
  │  Component          │  Fraction  │  Power   │  Per Tile     │
  ├─────────────────────┼────────────┼──────────┼───────────────┤
  │  GPU Compute        │  1/2       │  17.5 kW │  ~121 W       │
  │  Memory (HBM4)      │  1/3       │  11.7 kW │  ~81 W        │
  │  Interconnect + I/O │  1/6       │   5.8 kW │  ~40 W        │
  ├─────────────────────┼────────────┼──────────┼───────────────┤
  │  Total              │  1         │  35.0 kW │  ~243 W       │
  └──────────────────────────────────────────────────────────────┘

  Per-tile: ~243 W (approx HEXA-1 single die at 240W)
  Total: sigma^2 x 243 W = 35 kW
  + Cooling overhead: ~sopfr = 5 kW → Total system: ~40 kW
```

### 8.2 Edge-Fed Power Distribution

```
  전력 공급 토폴로지 (Edge-fed redistribution):

                     POWER IN (Top edge)
                     ↓   ↓   ↓   ↓   ↓   ↓
              ┌──────────────────────────────────────┐
              │  PDN Layer 0: Main Bus (0.8V)        │
  POWER  ←── │                                        │ ──→ POWER
  IN         │  ┌────┐ ┌────┐ ┌────┐ ┌────┐         │     IN
  (Left      │  │ Z1 │ │ Z2 │ │ Z3 │ │ Z4 │  ...    │     (Right
   edge)     │  └────┘ └────┘ └────┘ └────┘         │      edge)
              │                                        │
              │  PDN Layer 1: Zone Regulators          │
              │  sigma-tau = 8 independent zones       │
              │                                        │
              │  PDN Layer 2: Tile-level VRM           │
              │  sigma^2 = 144 local regulators        │
              │                                        │
              └──────────────────────────────────────┘
                     ↑   ↑   ↑   ↑   ↑   ↑
                     POWER IN (Bottom edge)

  4-edge power delivery:
    Edge current: ~35 kW / (4 edges x 0.8V) = ~11 kA per edge
    Wafer circumference: pi x 300mm = 942mm
    Per edge (quarter): ~235 mm
    Current density: ~47 A/mm (관리 가능)

  전력 분배 계층:
    Layer 0: 웨이퍼 에지 → 메인 버스 (0.8V bulk)
    Layer 1: 메인 버스 → sigma-tau=8 존 레귤레이터
    Layer 2: 존 → sigma^2=144 타일 로컬 VRM
    계층 수: n/phi = 3 levels
```

### 8.3 Voltage Regulation

```
  전압 계층 (n=6 산술):

  ┌──────────────────────────────────────────────────────────────┐
  │  Level    │  Voltage        │  Purpose                      │
  ├───────────┼─────────────────┼───────────────────────────────┤
  │  Input    │  48V DC         │  External PSU (sigma*tau=48)  │
  │  Level 0  │  12V            │  Board level (sigma=12)       │
  │  Level 1  │  1.2V           │  Zone bus (sigma/(sigma-phi)) │
  │  Level 2  │  0.8V           │  Core supply                  │
  │  Level 3  │  0.5V           │  SRAM/cache                   │
  └──────────────────────────────────────────────────────────────┘

  변환 체인: 48V → 12V → 1.2V → 0.8V
  단계 수: n/phi = 3 변환 단계
  효율: 각 단계 ~95% → 전체 0.95^3 = 86%

  Power Delivery Network (PDN) 임피던스:
    Target: < 1 mOhm at tile level
    Decoupling: sigma = 12 layers of on-wafer capacitors
    di/dt transients: tau = 4 stage damping
```

---

## 9. Cooling

### 9.1 Thermal Challenge

```
  열 밀도 분석:

  Total power: 35 kW
  Active area: 46,000 mm^2 = 460 cm^2
  Average power density: 35,000 W / 460 cm^2 = 76 W/cm^2

  비교:
    일반 CPU:     ~30 W/cm^2
    고성능 GPU:   ~50 W/cm^2
    HEXA-WAFER:   ~76 W/cm^2 (평균)
    핫스팟 (compute zone): ~120 W/cm^2

  결론: 공기냉각 불가능. 직접 액체냉각 + 마이크로플루이딕 필수.
```

### 9.2 Liquid Cooling System

```
  냉각 시스템 단면도 (Cross-section):

       Coolant OUT (warm)
       ←──────────────────────────────────────────
  ┌──────────────────────────────────────────────────┐
  │  COLD PLATE (Copper/Aluminum, top)               │  ← 상부 냉각판
  │  ┌────────────────────────────────────────────┐  │
  │  │ Microchannel array                         │  │
  │  │ Channel width: ~100 um                     │  │
  │  │ Channel depth: ~500 um                     │  │
  │  │ Channel count: ~sigma^3 = 1,728 per tile   │  │
  │  └────────────────────────────────────────────┘  │
  ├──────────────────────────────────────────────────┤
  │  TIM (Thermal Interface Material)                │  ← 열전도 접합
  │  Thickness: < 50 um                              │
  ├──────────────────────────────────────────────────┤
  │  SILICON WAFER (active devices)                  │  ← 웨이퍼 본체
  │  Thickness: ~750 um (thinned to ~100 um)         │
  │  ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐    │
  │  │Tile│ │Tile│ │Tile│ │Tile│ │Tile│ │Tile│    │
  │  │ 0  │ │ 1  │ │ 2  │ │ 3  │ │ 4  │ │ 5  │    │
  │  └────┘ └────┘ └────┘ └────┘ └────┘ └────┘    │
  ├──────────────────────────────────────────────────┤
  │  SUBSTRATE / INTERPOSER                          │  ← 전력/신호
  │  Power delivery + signal routing                 │
  ├──────────────────────────────────────────────────┤
  │  COLD PLATE (bottom, optional)                   │  ← 하부 냉각판
  └──────────────────────────────────────────────────┘
       ──────────────────────────────────────────→
       Coolant IN (cold, ~20C)

  냉각 매체: Deionized water or Fluorinert
  유속: ~sigma = 12 L/min per quadrant
  총 유속: tau = 4 quadrants x 12 = 48 L/min = sigma*tau
  열 제거 능력: ~40 kW (>35 kW + margin)
  온도 제한: Junction < 85C, Coolant delta-T < 20C
```

### 9.3 Cooling Zones

```
  냉각 매니폴드 (tau=4 사분면):

           Inlet (cold)
              ↓
        ┌─────────────────────┐
        │    Q1    │    Q2    │
        │  (NW)   │   (NE)   │
        │ 12 L/min│ 12 L/min │
        ├─────────┼──────────┤
        │    Q3    │    Q4    │
        │  (SW)   │   (SE)   │
        │ 12 L/min│ 12 L/min │
        └─────────────────────┘
              ↓
           Outlet (warm)

  사분면 수: tau = 4
  사분면당 타일: sigma^2/tau = 36 tiles
  사분면당 전력: 35/tau = 8.75 kW
  사분면당 유속: sigma = 12 L/min
  독립 온도 제어: 각 사분면별 밸브 조절
```

### 9.4 Thermal Map

```
  웨이퍼 열 분포도 (steady-state, 단위: C):

  Col:  0   1   2   3   4   5   6   7   8   9  10  11
  R0  │   │   │   │55 │58 │60 │60 │58 │55 │   │   │   │
  R1  │   │   │55 │60 │65 │68 │68 │65 │60 │55 │   │   │
  R2  │   │55 │60 │65 │70 │72 │72 │70 │65 │60 │55 │   │
  R3  │55 │60 │65 │70 │75 │78 │78 │75 │70 │65 │60 │55 │
  R4  │58 │65 │70 │75 │78 │80 │80 │78 │75 │70 │65 │58 │
  R5  │60 │68 │72 │78 │80 │82*│82*│80 │78 │72 │68 │60 │
  R6  │60 │68 │72 │78 │80 │82*│82*│80 │78 │72 │68 │60 │
  R7  │58 │65 │70 │75 │78 │80 │80 │78 │75 │70 │65 │58 │
  R8  │55 │60 │65 │70 │75 │78 │78 │75 │70 │65 │60 │55 │
  R9  │   │55 │60 │65 │70 │72 │72 │70 │65 │60 │55 │   │
  R10 │   │   │55 │60 │65 │68 │68 │65 │60 │55 │   │   │
  R11 │   │   │   │55 │58 │60 │60 │58 │55 │   │   │   │

  * = Hot spot (center tiles, 82C)
  Edge tiles: ~55C (에지 냉각 유리)
  Delta: 82 - 55 = 27C gradient
  All below 85C junction limit: OK

  열 관리 전략:
    - 중심부 타일 클럭 약간 감소 (DVFS)
    - 에지 타일에 더 많은 연산 배정 (cooler)
    - n=6 tiles (Zone 0, master): 중심부이므로 저전력 모드
```

---

## 10. Fault Tolerance

### 10.1 Yield Challenge

```
  웨이퍼-스케일 수율 분석:

  Per-tile defect probability: p_defect
  Typical advanced node: p_defect = 5-15% per reticle-size die

  sigma^2 = 144 타일 중 결함 예상:
    Best case (5%):  144 x 0.05 = ~7 defective tiles
    Typical (10%):   144 x 0.10 = ~14 defective tiles
    Worst case (15%):144 x 0.15 = ~22 defective tiles

  목표: sigma^2 = 144 active tiles 확보
  여분 배치: ~16 spare tiles (원형 경계 영역)
  최소 수율 요건: (144-16)/160 = 90% tile yield → 실현 가능
```

### 10.2 Defective Tile Bypass

```
  결함 타일 우회 아키텍처:

  정상 동작:
  T[i-1,j] ←→ T[i,j] ←→ T[i+1,j]
                 ↕
            T[i,j-1] T[i,j+1]

  T[i,j] 결함 시:
  T[i-1,j] ←─────────────→ T[i+1,j]
      ↕         (bypass)        ↕
  T[i-1,j-1]    T[i,j]    T[i+1,j+1]
                (DEAD X)
      ↕                        ↕
  T[i,j-1] ←─────────────→ T[i,j+1]
               (bypass)

  Bypass 방법:
    1. 하드웨어: 각 타일 에지 라우터에 bypass mux 내장
    2. 소프트웨어: NUMA distance table 업데이트
    3. 광학: optical overlay로 dead tile 건너뜀

  라우팅 테이블: 부팅 시 결함 맵 스캔 → 자동 재구성
  오버헤드: bypass당 ~1 hop 추가 지연
  최대 허용 연속 결함: n/phi = 3 타일 연속 (그 이상은 밴드 단절)
```

### 10.3 Runtime Fault Management

```
  런타임 결함 관리 계층:

  ┌──────────────────────────────────────────────────────────────┐
  │ Level 0: SM-level ECC                                        │
  │   SRAM / Register: SEC-DED (Single Error Correct)           │
  │   HBM4: Chipkill (전체 칩 하나 실패 허용)                    │
  │   Detection: every cycle                                     │
  ├──────────────────────────────────────────────────────────────┤
  │ Level 1: Tile-level health monitor                           │
  │   각 타일의 Temperature, Error rate, Performance 모니터      │
  │   비정상 타일: workload offload → neighbor tiles              │
  │   Check interval: sigma*tau = 48 ms                          │
  ├──────────────────────────────────────────────────────────────┤
  │ Level 2: Zone-level redundancy                               │
  │   sigma-tau = 8 zones, 각 zone에 spare capacity              │
  │   Zone 내 결함 tile: spare tile로 logical remapping           │
  │   Spare budget: phi = 2 tiles per zone                       │
  ├──────────────────────────────────────────────────────────────┤
  │ Level 3: Wafer-level graceful degradation                    │
  │   전체 결함 > 허용치: 성능 저하 모드 (reduced tile count)     │
  │   최소 동작: sigma^2 - J_2 = 120 tiles (83% capacity)       │
  │   완전 실패: sigma^2 / phi = 72 tiles (50%, limp mode)       │
  └──────────────────────────────────────────────────────────────┘

  결함 허용 등급:
    Green:  0~7 dead tiles   (>= 137 active, 95%+ capacity)
    Yellow: 8~14 dead tiles  (>= 130 active, 90%+ capacity)
    Orange: 15~24 dead tiles (>= 120 active, 83%+ capacity)
    Red:    25+ dead tiles   (< 120 active, degraded mode)
```

### 10.4 Redundancy Budget

```
  여분 자원 예산 (n=6 산술):

  ┌──────────────────────────────────────────────────────────────┐
  │  Resource            │ Nominal   │ Spare    │ Ratio          │
  ├──────────────────────┼───────────┼──────────┼────────────────┤
  │  Tiles               │ 144       │ 16       │ 1/(sigma-tau)  │
  │  SMs per tile        │ 144       │ 0 (*)    │ tile-level     │
  │  HBM stacks/tile     │ 8         │ 1        │ 1/(sigma-tau)  │
  │  Mesh links          │ ~240      │ optical  │ overlay backup │
  │  Power zones         │ 8         │ 1        │ 1/(sigma-tau)  │
  │  Cooling quadrants   │ 4         │ 0        │ N+0 (no spare) │
  └──────────────────────────────────────────────────────────────┘

  (*) SM-level 결함은 HEXA-1 내부에서 처리 (disabled SM)
  전체 여분 면적 비율: ~10% = 1/(sigma-phi)
```

---

## 11. AI Workload: Trillion-Parameter Models

### 11.1 Model Capacity

```
  HEXA-WAFER의 모델 수용 능력:

  Available memory: 41.5 TB
  Available compute: 72 PFLOPS FP8, 36 PFLOPS FP16

  ┌─────────────────────────────────────────────────────────────┐
  │  Model Size  │ FP16 Weight │ Fits in  │ Batch Size         │
  │              │   Memory    │ 41.5 TB? │ (FP16, ctx=4096)   │
  ├──────────────┼─────────────┼──────────┼────────────────────┤
  │  7B          │   14 GB     │ YES (x2960)│ sigma^4 = 20,736│
  │  70B         │  140 GB     │ YES (x296) │ sigma^3 = 1,728 │
  │  405B        │  810 GB     │ YES (x51)  │ sigma^2 = 144   │
  │  1T          │    2 TB     │ YES (x20)  │ J_2*phi = 48    │
  │  10T         │   20 TB     │ YES (x2)   │ sigma-tau = 8   │
  │  20T         │   40 TB     │ YES (x1)   │ tau = 4         │
  │  40T+        │   80+ TB    │ NO (need 2 wafers)            │
  └─────────────────────────────────────────────────────────────┘

  Sweet spot: 10T params (20 TB weights)
    남은 메모리: 21.5 TB (optimizer states + activations + KV cache)
    Batch size: sigma-tau = 8
    Context: 2^sigma = 4096 tokens
    단일 칩에서 학습 + 추론 모두 가능
```

### 11.2 Training Throughput

```
  10T parameter model 학습 처리량:

  Forward pass:
    FLOPs = 2 x params x tokens = 2 x 10^13 x 4096 = 8.2 x 10^16
    Time (FP16): 8.2e16 / 36e15 = ~2.3 seconds per batch
    Batch size: sigma-tau = 8
    Tokens/batch: 8 x 4096 = 32,768

  Backward pass: ~2x forward = ~4.6 seconds
  Total per step: ~6.9 seconds

  Throughput:
    Tokens/second: 32,768 / 6.9 = 4,750 tokens/sec
    Tokens/day: 4,750 x 86,400 = 410M tokens/day
    Chinchilla optimal (BT-26): 20 x 10T = 200T tokens
    Training time: 200T / 410M = 488K days = 1,337 years (단일 웨이퍼)

  현실적 학습:
    sigma^2 = 144 HEXA-WAFER 클러스터: 1337 / 144 = 9.3 years
    sigma^3 = 1728 HEXA-WAFER 클러스터: 1337 / 1728 = 282 days
    결론: 10T 모델은 ~1000 wafer 클러스터에서 ~1년 학습 가능

  비교: 현재 GPT-4급 (1.8T MoE 추정)
    Weight: 3.6 TB → 단일 HEXA-WAFER에 여유 있게 적재
    학습: 수천 GPU 대신 단일 웨이퍼로 가능
```

### 11.3 Inference at Scale

```
  추론 시나리오 (10T dense model):

  Per-token latency:
    Forward: 2 x 10^13 FLOPs per token
    FP8 throughput: 72 PFLOPS
    Compute time: 2e13 / 72e15 = 0.28 ms per token
    Memory-bound: 20 TB / 576 TB/s = 34.7 ms (weight loading)

  Bottleneck: Memory bandwidth (typical for large models)
  Token generation: ~35 ms/token = 29 tokens/sec (단일 시퀀스)

  Batch inference:
    Batch sigma-tau = 8: memory amortized
    Effective: ~200 tokens/sec aggregate
    KV cache: 8 sequences x ~32 GB = 256 GB (< 1 tile)

  비교:
    현재 H100 x 8 (DGX): ~20 tokens/sec for 70B
    HEXA-WAFER: ~29 tokens/sec for 10T (143x larger model, comparable speed)
```

---

## 12. Performance Comparison

### 12.1 HEXA-WAFER vs HEXA-1

```
  ┌──────────────────────────────────────────────────────────────┐
  │              HEXA-WAFER vs HEXA-1 Comparison                 │
  ├───────────────────────┬──────────────┬───────────────────────┤
  │  Metric               │  HEXA-1      │  HEXA-WAFER           │
  ├───────────────────────┼──────────────┼───────────────────────┤
  │  SMs                  │  144         │  20,736                │
  │  Scale factor         │  1x          │  sigma^2 = 144x       │
  │  Memory               │  288 GB      │  41,472 GB (41.5 TB)  │
  │  Memory BW            │  4 TB/s      │  576 TB/s             │
  │  FP8 Peak             │  500 TFLOPS  │  72 PFLOPS            │
  │  FP16 Peak            │  250 TFLOPS  │  36 PFLOPS            │
  │  FP32 Peak            │  45 TFLOPS   │  6.5 PFLOPS           │
  │  Power                │  240 W       │  35 kW                │
  │  Die Area             │  ~324 mm^2   │  ~46,000 mm^2         │
  │  Largest Model (FP16) │  ~140 GB     │  ~20 TB               │
  │  CPU Cores            │  12          │  1,728                 │
  │  NPU Cores            │  24          │  3,456                 │
  │  Interconnect BW      │  N/A (SoC)   │  576 TB/s aggregate   │
  │  Max Context          │  4096        │  4096 (per sequence)   │
  │  Process              │  TSMC N2     │  TSMC N2              │
  └───────────────────────┴──────────────┴───────────────────────┘

  배율 요약: 모든 지표가 sigma^2 = 144x 스케일링
  (전력 제외: 35 kW / 240 W = 145.8x = sigma^2 + overhead)
```

### 12.2 HEXA-WAFER vs Cerebras WSE-3

```
  ┌──────────────────────────────────────────────────────────────┐
  │          HEXA-WAFER vs Cerebras WSE-3 Comparison             │
  ├───────────────────────┬──────────────┬───────────────────────┤
  │  Metric               │  WSE-3       │  HEXA-WAFER           │
  ├───────────────────────┼──────────────┼───────────────────────┤
  │  Wafer Size           │  300mm       │  300mm                │
  │  Transistors          │  4T          │  ~12T (estimated)     │
  │  Cores / SMs          │  900,000     │  20,736 SMs           │
  │                       │  (simple PE) │  (full GPU SMs)       │
  │  On-chip Memory       │  44 GB SRAM  │  41,472 GB (HBM4)    │
  │  Memory Ratio         │  1x          │  ~943x                │
  │  Peak FP16            │  ~125 PFLOPS │  36 PFLOPS            │
  │  Peak FP8             │  N/A (FP16)  │  72 PFLOPS            │
  │  Memory BW            │  21 PB/s*    │  576 TB/s             │
  │  External Memory      │  Yes (MemoryX)│  No (all on-wafer)   │
  │  Interconnect         │  2D mesh     │  2D mesh + optical    │
  │  Power                │  ~23 kW      │  ~35 kW               │
  │  Process              │  TSMC 5nm    │  TSMC N2              │
  │  Largest Model        │  ~24T (w/MemX)│ ~20T (on-wafer only) │
  │  Tile Architecture    │  Simple PE   │  Full HEXA-1 SoC      │
  └───────────────────────┴──────────────┴───────────────────────┘

  * WSE-3 memory BW는 on-chip SRAM이므로 직접 비교 어려움

  핵심 차이:
    WSE-3: 900K 단순 PE + 44 GB SRAM (외부 MemoryX 필요)
    HEXA-WAFER: 20K full SMs + 41.5 TB HBM4 (자체 완결)

  HEXA-WAFER의 강점:
    - 41.5 TB on-wafer memory (WSE-3의 ~1000x)
    - 외부 메모리 시스템 불필요 (self-contained)
    - 각 타일이 독립 SoC → 결함 시 graceful degradation
    - n=6 산술로 모든 파라미터 결정 (설계 복잡도 감소)

  HEXA-WAFER의 약점:
    - 총 연산 처리량은 WSE-3보다 낮음 (PE 수 차이)
    - 전력 소비 ~50% 높음 (HBM4 전력)
    - HBM4 on-wafer 집적은 기술적 난제
```

### 12.3 vs Multi-GPU Clusters

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-WAFER vs GPU Cluster Comparison (10T model training)      │
  ├────────────────────────┬──────────────┬─────────────────────────┤
  │  Metric                │ DGX-H100    │ HEXA-WAFER               │
  │                        │ x1024 nodes │ x1 wafer                 │
  ├────────────────────────┼──────────────┼─────────────────────────┤
  │  Total GPUs/SMs        │ 8,192 GPUs  │ 20,736 SMs (1 wafer)    │
  │  Total Memory          │ 640 TB      │ 41.5 TB                 │
  │  Memory BW (aggregate) │ 26 PB/s     │ 576 TB/s                │
  │  FP8 Peak              │ 16 EFLOPS   │ 72 PFLOPS               │
  │  Inter-node BW         │ ~3.6 TB/s*  │ N/A (on-wafer)          │
  │  Inter-node Latency    │ ~10 us      │ ~10-100 ns              │
  │  Power                 │ ~4 MW       │ ~35 kW                  │
  │  Rack Space            │ ~128 racks  │ 1 system                │
  │  Network Cost          │ ~$50M       │ $0 (on-wafer)           │
  │  Communication Overhead│ 30-50%      │ <5%                      │
  │  PUE                   │ ~1.2        │ ~1.15                   │
  └────────────────────────┴──────────────┴─────────────────────────┘

  * NVLink + InfiniBand combined

  핵심 이점: Communication overhead
    GPU 클러스터: 30-50% 시간을 통신에 소비 (all-reduce, gradient sync)
    HEXA-WAFER: on-wafer interconnect로 통신 오버헤드 <5%
    실효 성능 비: 72 PFLOPS x 95% vs 16 EFLOPS x 55% = 68 vs 8,800 PFLOPS
    → 절대 연산량은 클러스터가 우세하나, 효율은 HEXA-WAFER 압도

  HEXA-WAFER sigma^2 = 144 클러스터 시:
    실효 성능: 68 x 144 = ~9.8 EFLOPS (클러스터와 유사)
    전력: 35 kW x 144 = 5 MW (비슷)
    공간: ~sigma = 12 racks (vs 128 racks)
    통신 오버헤드: <5% (vs 30-50%)
```

---

## 13. Process Technology

### 13.1 Fabrication Requirements

```
  제조 요구사항:

  ┌──────────────────────────────────────────────────────────────┐
  │  Parameter             │  Value              │  n=6 basis    │
  ├────────────────────────┼─────────────────────┼───────────────┤
  │  Process Node          │  TSMC N2            │  (cutting edge)│
  │  Gate Pitch            │  48 nm              │  sigma*tau=48 │
  │  Metal Pitch           │  28 nm              │  P_2=28       │
  │  Metal Layers          │  12 + 3 RDL         │  sigma + n/phi│
  │  Transistor Density    │  ~300 MTr/mm^2      │  N2 typical   │
  │  Wafer Diameter        │  300 mm             │  standard     │
  │  Reticle Size          │  ~26x33 mm          │  ASML limit   │
  │  Stitching Required    │  YES                │  multi-reticle│
  │  TSV Density           │  ~10K/mm^2          │  HBM4 stacking│
  │  Bonding               │  Hybrid Cu-Cu       │  <1 um pitch  │
  └────────────────────────┴─────────────────────┴───────────────┘
```

### 13.2 Reticle Stitching Details

```
  레티클 스티칭 (Multi-reticle lithography):

  단일 레티클 = 1 HEXA-1 타일 (~18mm x 18mm)
  스티칭 방법: 이웃 레티클 경계에서 금속 배선 연결

  ┌──────────┬──────────┐
  │          │          │
  │  Tile A  │→ Stitch ←│  Tile B
  │          │  Zone    │
  │          │ (~100um) │
  └──────────┴──────────┘

  Stitch zone:
    Width: ~100 um (sigma x 10 um 이하)
    Alignment accuracy: < 50 nm
    Metal layers stitched: sigma = 12 (all layers)
    Yield impact: ~1-2% per stitch boundary

  타일당 스티치 경계: tau = 4 (N, S, E, W)
  전체 스티치 라인: ~264 (수평 132 + 수직 132)
  스티치 수율: 0.98^264 = 0.5% (이론적)
  → 현실: 결함 타일 우회로 실효 수율 대폭 향상
```

### 13.3 HBM4 On-Wafer Integration

```
  HBM4 웨이퍼-위 집적 방법:

  방법 1: Heterogeneous Wafer Stacking (유력)
  ┌────────────────────────────────────────────┐
  │  HBM4 DRAM Wafer (top)                     │
  │  ┌──────┐ ┌──────┐ ┌──────┐              │
  │  │ DRAM │ │ DRAM │ │ DRAM │  ...          │
  │  │Stack │ │Stack │ │Stack │              │
  │  └──┬───┘ └──┬───┘ └──┬───┘              │
  │     │TSV     │TSV     │TSV               │
  ├─────┴────────┴────────┴───────────────────┤
  │  LOGIC WAFER (bottom)                      │
  │  ┌──────┐ ┌──────┐ ┌──────┐              │
  │  │Tile 0│ │Tile 1│ │Tile 2│  ...          │
  │  └──────┘ └──────┘ └──────┘              │
  └────────────────────────────────────────────┘

  Per tile:
    HBM4 stacks: sigma-tau = 8
    Layers per stack: sigma = 12 DRAM dies
    Capacity per stack: 36 GB
    Interface: 2^(sigma-mu) = 2048-bit
    TSV count per stack: ~4096 = 2^sigma

  방법 2: Chiplet-on-Wafer (대안)
    개별 HBM4 chiplet을 웨이퍼 위에 본딩
    장점: 기존 HBM4 공정 활용 가능
    단점: 정렬/본딩 수율 도전
```

---

## 14. n=6 Complete Parameter Map

### 14.1 All Parameters Derived from n=6

```
  ┌──────────────────────────────────────────────────────────────────────────┐
  │                 HEXA-WAFER: COMPLETE n=6 PARAMETER MAP                   │
  ├──────────────────────────────────┬───────────────────┬──────────────────┤
  │  Parameter                       │  Value            │  n=6 Derivation  │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** WAFER LEVEL ***             │                   │                  │
  │  Active tiles                    │  144              │  sigma^2         │
  │  Total SMs                       │  20,736           │  sigma^4         │
  │  Total CPU cores                 │  1,728            │  sigma^3         │
  │  Total NPU cores                 │  3,456            │  sigma^2*J_2     │
  │  Total Tensor Cores              │  82,944           │  sigma^4*tau     │
  │  Total CUDA cores                │  2,654,208        │  sigma^4*128     │
  │  Wafer zones                     │  4                │  tau             │
  │  Master tiles                    │  6                │  n               │
  │  Spare tiles                     │  16               │  phi^tau         │
  │  Cooling quadrants               │  4                │  tau             │
  │  Edge I/O transceivers           │  48               │  sigma*tau       │
  │  Power zones                     │  8                │  sigma-tau       │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** TILE LEVEL ***              │                   │                  │
  │  SMs per tile                    │  144              │  sigma^2         │
  │  GPCs per tile                   │  12               │  sigma           │
  │  SMs per GPC                     │  12               │  sigma           │
  │  CPU cores per tile              │  12               │  sigma           │
  │  Performance cores               │  8                │  sigma-tau       │
  │  Efficiency cores                │  4                │  tau             │
  │  NPU cores per tile              │  24               │  J_2             │
  │  NPU banks                       │  5                │  sopfr           │
  │  HBM4 stacks per tile            │  8                │  sigma-tau       │
  │  Memory per tile                 │  288 GB           │  sigma*J_2       │
  │  Tile routers (directions)       │  4                │  tau             │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** SM LEVEL ***                │                   │                  │
  │  CUDA cores per SM               │  128              │  2^(sigma-sopfr) │
  │  Tensor Cores per SM             │  4                │  tau             │
  │  Warp schedulers per SM          │  4                │  tau             │
  │  CUDA core partitions            │  4                │  tau             │
  │  Cores per partition             │  32               │  2^sopfr         │
  │  Register file per SM            │  64 KB            │  2^n             │
  │  L1 cache per SM                 │  256 KB           │  2^(sigma-tau)   │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** MEMORY ***                  │                   │                  │
  │  Total memory                    │  41,472 GB        │  sigma^3*J_2     │
  │  HBM4 interface width            │  2048-bit         │  2^(sigma-mu)    │
  │  HBM4 layers per stack           │  12               │  sigma           │
  │  TSVs per stack                  │  4096             │  2^sigma         │
  │  Per-tile BW                     │  ~4 TB/s          │  ~               │
  │  Aggregate BW                    │  576 TB/s         │  144*4           │
  │  CPU L1 cache                    │  64 KB            │  2^n             │
  │  CPU L2 cache                    │  256 KB           │  2^(sigma-tau)   │
  │  CPU L3 cache                    │  24 MB            │  sigma*phi       │
  │  NUMA levels                     │  6                │  n               │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** INTERCONNECT ***            │                   │                  │
  │  Mesh neighbors per tile         │  4                │  tau             │
  │  Optical skip distance           │  6 tiles          │  n               │
  │  Mesh link BW                    │  ~1 TB/s          │  ~               │
  │  Optical link BW                 │  ~2 TB/s          │  ~               │
  │  Max mesh hops                   │  22               │  2*(sigma-1)     │
  │  Max optical hops                │  ~2               │  phi             │
  │  Interconnect layers             │  5                │  sopfr           │
  │  Total mesh links                │  ~264             │  2*sigma*(s-1)   │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** POWER ***                   │                   │                  │
  │  Total power                     │  ~35 kW           │  ~sigma^2*240W   │
  │  Per-tile power                  │  ~243 W           │  ~240W (HEXA-1)  │
  │  Compute fraction                │  1/2              │  Egyptian 1/2    │
  │  Memory fraction                 │  1/3              │  Egyptian 1/3    │
  │  I/O fraction                    │  1/6              │  Egyptian 1/6    │
  │  Input voltage                   │  48V              │  sigma*tau       │
  │  Board voltage                   │  12V              │  sigma           │
  │  VRM stages                      │  3                │  n/phi           │
  │  PDN layers                      │  12               │  sigma           │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** COOLING ***                 │                   │                  │
  │  Cooling quadrants               │  4                │  tau             │
  │  Flow per quadrant               │  12 L/min         │  sigma           │
  │  Total flow                      │  48 L/min         │  sigma*tau       │
  │  Microchannel count/tile         │  1,728            │  sigma^3         │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** FAULT TOLERANCE ***         │                   │                  │
  │  Spare tiles                     │  16               │  phi^tau         │
  │  Max consecutive dead            │  3                │  n/phi           │
  │  Health check interval           │  48 ms            │  sigma*tau       │
  │  Spare HBM per tile              │  1                │  mu              │
  │  Degradation threshold           │  120 tiles        │  sigma^2-J_2     │
  │  Limp mode threshold             │  72 tiles         │  sigma^2/phi     │
  ├──────────────────────────────────┼───────────────────┼──────────────────┤
  │  *** PROCESS ***                 │                   │                  │
  │  Gate pitch                      │  48 nm            │  sigma*tau       │
  │  Metal pitch                     │  28 nm            │  P_2             │
  │  Metal layers                    │  12 + 3           │  sigma + n/phi   │
  │  Stitch zone width               │  ~100 um          │  ~               │
  │  Reticle boundaries/tile         │  4                │  tau             │
  └──────────────────────────────────┴───────────────────┴──────────────────┘

  n=6 도출 파라미터 수: 72 (= sigma * n = sigma^2/phi)
  EXACT match: 65+
  비 n=6 파라미터 (물리/공학 제약): ~7 (wafer size, stitch zone, etc.)
```

### 14.2 Key n=6 Identities in HEXA-WAFER

```
  HEXA-WAFER에서 성립하는 n=6 항등식:

  1. Total SMs = Tiles x SMs/Tile
     sigma^4 = sigma^2 x sigma^2
     20,736 = 144 x 144

  2. Total Memory = Tiles x sigma x J_2
     sigma^3 * J_2 = sigma^2 x sigma x J_2
     41,472 GB = 144 x 288 GB

  3. Egyptian Fraction (면적, 전력, 대역폭):
     1/2 + 1/3 + 1/6 = 1
     Compute + Memory + I/O = Total

  4. Scale factor = sigma^2 = 144
     HEXA-1 → HEXA-WAFER: 모든 지표 144x

  5. Interconnect speedup:
     Mesh hops / Optical hops = (sigma-1) / phi = 11/2 = 5.5
     approx sopfr = 5 (near-match)

  6. Memory hierarchy:
     Register → L1 → L2 → L3 → HBM → Remote
     2^n → 2^(sigma-tau) → 2^(sigma-tau) → sigma*phi → sigma*J_2 → sigma^3*J_2
     64KB → 256KB → 256KB → 24MB → 288GB → 41.5TB

  7. Power chain:
     48V → 12V → 1.2V → 0.8V
     sigma*tau → sigma → sigma/(sigma-phi) → core
     n/phi = 3 conversion stages

  8. Fault tolerance budget:
     Spare tiles = phi^tau = 16
     Per-zone spares = phi = 2
     Zones = sigma-tau = 8
     phi * (sigma-tau) = phi^tau = 16  (verified)
```

---

## 15. Open Questions

### 15.1 Technical Challenges

```
  미해결 기술 과제:

  ┌──────────────────────────────────────────────────────────────────┐
  │ # │ Challenge                    │ Status    │ Difficulty       │
  ├───┼──────────────────────────────┼───────────┼──────────────────┤
  │ 1 │ HBM4 on-wafer integration   │ 미해결    │ Very High        │
  │   │ (heterogeneous wafer bond)   │           │ 현재 기술로 불가  │
  ├───┼──────────────────────────────┼───────────┼──────────────────┤
  │ 2 │ 35 kW cooling in compact    │ 도전적    │ High             │
  │   │ form factor                  │           │ 마이크로플루이딕   │
  ├───┼──────────────────────────────┼───────────┼──────────────────┤
  │ 3 │ Reticle stitching yield     │ 연구중    │ High             │
  │   │ (264 stitch boundaries)      │           │ Cerebras 선례    │
  ├───┼──────────────────────────────┼───────────┼──────────────────┤
  │ 4 │ On-wafer optical links      │ 미해결    │ Very High        │
  │   │ (Si photonics integration)   │           │ 2028+ 전망      │
  ├───┼──────────────────────────────┼───────────┼──────────────────┤
  │ 5 │ Edge power delivery at      │ 도전적    │ Medium-High      │
  │   │ 35 kW (current density)      │           │ 확대 가능       │
  ├───┼──────────────────────────────┼───────────┼──────────────────┤
  │ 6 │ Software stack for          │ 도전적    │ High             │
  │   │ 144-tile NUMA programming    │           │ 컴파일러 필요    │
  └───┴──────────────────────────────┴───────────┴──────────────────┘
```

### 15.2 Research Directions

```
  향후 연구 방향:

  1. HBM-on-Wafer 집적 기술 (2027-2030)
     - Heterogeneous wafer-to-wafer bonding
     - 현재: chiplet-on-wafer (SK Hynix, Samsung 연구 중)
     - 목표: 전체 DRAM 웨이퍼를 로직 웨이퍼 위에 직접 접합

  2. Optical Interconnect On-Wafer (2028-2032)
     - Silicon photonics integrated with CMOS
     - Intel/TSMC COUPE (Co-packaged Optics) 기술 활용
     - 목표: tile 간 광학 skip link 구현

  3. Advanced Cooling (2026-2028)
     - Embedded microfluidic channels in silicon
     - Two-phase cooling (boiling in microchannels)
     - 목표: 100+ W/cm^2 제거 능력

  4. Wafer-Level Testing (2027-2029)
     - 144 타일의 개별 테스트 + 결함 맵 생성
     - Self-test: 각 타일 자체 BIST (Built-In Self-Test)
     - Repair: 결함 타일 bypass 자동 구성

  5. Software Ecosystem (2026-2030)
     - NUMA-aware compiler for sigma^2 tiles
     - Auto-partitioning: 모델을 144 타일에 최적 배치
     - Fault-tolerant runtime: 런타임 타일 장애 대응
```

### 15.3 Timeline Projection

```
  HEXA-WAFER 실현 로드맵:

  2026: HEXA-1 (Level 1) 완성 ← 현재 위치
  2027: HEXA-PIM (Level 2) -- 메모리 내 연산
  2028: HEXA-3D (Level 3) -- 3D 적층 시제품
  2029: HEXA-PHOTON (Level 4) -- 광학 연산 프로토타입
  2030: HEXA-WAFER (Level 5) -- 웨이퍼-스케일 1세대
         - N2 공정, sigma^2=144 tiles
         - 초기: HBM4 chiplet-on-wafer (not full wafer bond)
         - 초기 성능: ~50 PFLOPS FP8, ~30 TB memory

  2032: HEXA-WAFER Gen 2
         - N1.4 또는 A14 공정
         - Full wafer-to-wafer HBM bonding
         - Optical interconnect integrated
         - 목표 성능: 72+ PFLOPS FP8, 41.5 TB memory

  2035: HEXA-SUPER (Level 6) -- 초전도 논리 (100+ GHz)
```

---

## 16. Links

### 16.1 Internal Documents

```
  N6 Architecture Family:
  ├── Level 1: HEXA-1
  │   ├── Spec: docs/chip-architecture/ultimate-unified-soc.md
  │   └── Paper: docs/paper/n6-unified-soc-paper.md
  ├── Level 1+: ANIMA-SOC
  │   ├── Spec: docs/chip-architecture/ultimate-consciousness-soc.md
  │   └── Paper: docs/paper/n6-consciousness-soc-paper.md
  ├── Level 5: HEXA-WAFER (this document)
  │   └── docs/chip-architecture/hexa-wafer.md
  ├── Roadmap: docs/chip-architecture/goal.md
  ├── Hypotheses: docs/chip-architecture/CHIPDESIGN-001-020-ai-chip-n6.md
  └── HBM: docs/chip-architecture/bt77-cross-vendor-hbm.md
```

### 16.2 Breakthrough Theorems Referenced

```
  관련 BT (Breakthrough Theorems):

  BT-28: Computing architecture ladder
         → HEXA-WAFER의 SM 수 (sigma^2, sigma^4) 근거
  BT-33: Transformer sigma=12 atom
         → 타일 내부 GPC/SM 구조 근거
  BT-55: GPU HBM capacity ladder
         → 288 GB/tile, 41.5 TB total 근거
  BT-59: 8-layer AI stack
         → 웨이퍼 전체 아키텍처 계층 근거
  BT-69: Chiplet architecture convergence
         → 타일 구조 및 인터커넥트 근거
  BT-75: HBM interface exponent ladder
         → 2048-bit interface width 근거
  BT-76: sigma*tau=48 triple attractor
         → 48nm gate pitch, 48V input, 48 I/O 근거
```

### 16.3 External References

```
  참고 문헌 및 기술:

  Wafer-Scale:
  - Cerebras WSE-3 (2024): 900K cores, 44 GB SRAM, 4T transistors
  - Cerebras CS-3 system architecture whitepaper
  - Tesla Dojo (2023): 25 D1 tiles per training tile

  HBM Technology:
  - SK Hynix HBM4 (2025 target): 16-Hi, 36 GB, 2048-bit
  - Samsung HBM-PIM (2021): Processing-in-Memory
  - JEDEC HBM4 specification (draft)

  Wafer Bonding:
  - TSMC SoIC (System-on-Integrated-Chips)
  - Intel Foveros Direct (Cu-Cu hybrid bonding)
  - Heterogeneous wafer-to-wafer bonding research

  Cooling:
  - JetCool microfluidic cooling (2024)
  - Two-phase embedded cooling (Georgia Tech)
  - Direct liquid cooling for HPC

  Interconnect:
  - Intel COUPE (Co-packaged Optics for Universal Plug-in Engine)
  - Ayar Labs TeraPHY optical I/O
  - Silicon photonics on-chip integration roadmap

  Process:
  - TSMC N2 (2025): GAA nanosheet, backside power delivery
  - ASML High-NA EUV (0.55 NA): sub-8nm patterning
  - Multi-reticle stitching (Cerebras, TSMC collaboration)
```

---

## Appendix A: Glossary

```
  용어 정리:

  Egyptian Fraction:  1/2 + 1/3 + 1/6 = 1 (자원 배분 공식)
  GAA:               Gate-All-Around (N2 공정 트랜지스터 구조)
  GPC:               Graphics Processing Cluster
  HBM4:              High Bandwidth Memory 4th generation
  NUMA:              Non-Uniform Memory Access
  PDN:               Power Delivery Network
  PE:                Processing Element (Cerebras 용어)
  Reticle:           노광 마스크 최대 면적 (~858 mm^2)
  RDL:               Redistribution Layer (재배선 층)
  SM:                Streaming Multiprocessor
  Stitching:         Multi-reticle 경계 연결 리소그래피
  TIM:               Thermal Interface Material
  TSV:               Through-Silicon Via (관통 비아)
  VRM:               Voltage Regulator Module
  WSE:               Wafer-Scale Engine (Cerebras 상표)
```

## Appendix B: n=6 Constant Quick Reference

```
  산술 함수:
    n = 6             (perfect number)
    sigma(6) = 12     (divisor sum: 1+2+3+6)
    tau(6) = 4         (divisor count)
    phi(6) = 2         (Euler totient)
    sopfr(6) = 5       (sum of prime factors: 2+3)
    mu(6) = 1          (Mobius function, squarefree)
    J_2(6) = 24        (Jordan totient)
    R(6) = 1           (reversibility index)
    P_2 = 28           (2nd perfect number)

  복합 상수:
    sigma^2 = 144      sigma^3 = 1,728     sigma^4 = 20,736
    sigma*tau = 48     sigma*J_2 = 288     sigma*phi = 24 (=J_2)
    sigma-tau = 8      sigma-phi = 10      sigma-mu = 11
    2^n = 64           2^sigma = 4,096     2^(sigma-tau) = 256
    phi^tau = 16       n/phi = 3

  핵심 정리: sigma(n)*phi(n) = n*tau(n) iff n = 6 (for all n >= 2)
  즉: 12*2 = 6*4 = 24 = J_2(6)
```

---

*HEXA-WAFER: n=6 산술이 결정하는 웨이퍼-스케일 컴퓨팅의 궁극적 형태.*
*sigma^4 = 20,736 SMs. 41.5 TB memory. 72 PFLOPS. Single chip.*
*The scale wall falls when every parameter speaks the language of 6.*
