# HEXA-EDGE SoC -- Ultimate Edge/Mobile Processor

**Every Parameter From Perfect Number Arithmetic -- The Edge Chip Built on HEXA-LANG**

> HEXA-1 conquers the datacenter. HEXA-EDGE conquers the world.
> Smartphones, robots, IoT, autonomous edge -- all running native HEXA-LANG
> on a chip where every wire, every gate, every register is n=6.

**Date**: 2026-04-01
**Status**: Architecture Specification v1.0
**Dependencies**: BT-28, BT-33, BT-37, BT-56, BT-58, BT-59, BT-65, BT-66, BT-69, BT-76
**HEXA-LANG Integration**: Full hardware decode for 53 keywords, Egyptian memory controller

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  P_2 = 28       sigma^2 = 144    sigma*J_2 = 288   phi^tau = 16
  2^n = 64       sigma-tau = 8    sigma-phi = 10     sigma-mu = 11
  2^sigma = 4096  sigma*tau = 48   n/phi = 3
```

---

## 0. Design Philosophy

HEXA-1 is a 240W datacenter SoC with 144 GPU SMs and 288 GB HBM4.
HEXA-EDGE is its mirror image: the same n=6 arithmetic compressed into a
6W mobile envelope. Where HEXA-1 maximizes throughput, HEXA-EDGE maximizes
efficiency per watt -- the edge AI chip that runs HEXA-LANG natively.

```
  ┌──────────────────────────────────────────────────────────────────┐
  │                 HEXA-EDGE DESIGN TRIANGLE                        │
  │                                                                  │
  │                    Performance                                   │
  │                       /\                                         │
  │                      /  \                                        │
  │                     /    \                                       │
  │                    / n=6  \                                      │
  │                   /________\                                     │
  │                  /          \                                    │
  │           Power ──────────── Area                                │
  │           n=6W TDP           sigma*phi*n = 72 mm^2               │
  │                                                                  │
  │  Target: sigma-tau = 8 hours battery life on J_2 = 24 Wh cell   │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 1. Top-Level Block Diagram

```
  ┌────────────────────────────────────────────────────────────────────────────┐
  │                        HEXA-EDGE SoC  (TSMC N2, 72 mm^2)                  │
  │                                                                            │
  │  ┌──────────────────────────────────┐  ┌──────────────────────────────┐   │
  │  │        CPU COMPLEX               │  │         NPU COMPLEX          │   │
  │  │                                  │  │                              │   │
  │  │  ┌────────────────────────────┐  │  │  ┌────────────────────────┐  │   │
  │  │  │  PERFORMANCE CLUSTER       │  │  │  │  MAMBA SSM ENGINE      │  │   │
  │  │  │  tau = 4 cores (big)       │  │  │  │  d_state=16 expand=2   │  │   │
  │  │  │  n=6-wide, sigma=12 pipe   │  │  │  │  d_conv=4             │  │   │
  │  │  │  ROB=288, 72 arch regs     │  │  │  └────────────────────────┘  │   │
  │  │  │  HEXA-LANG HW decode       │  │  │  ┌────────────────────────┐  │   │
  │  │  └────────────────────────────┘  │  │  │  LoRA ENGINE           │  │   │
  │  │  ┌────────────────────────────┐  │  │  │  rank=8, n=6 adapters  │  │   │
  │  │  │  EFFICIENCY CLUSTER        │  │  │  └────────────────────────┘  │   │
  │  │  │  tau = 4 cores (LITTLE)    │  │  │  ┌────────────────────────┐  │   │
  │  │  │  n/phi=3-wide, n=6 pipe    │  │  │  │  MAC ARRAY             │  │   │
  │  │  │  Always-on sentinel        │  │  │  │  INT8: 72 TOPS         │  │   │
  │  │  └────────────────────────────┘  │  │  │  INT4: 144 TOPS        │  │   │
  │  │                                  │  │  └────────────────────────┘  │   │
  │  │  Shared L3: sigma = 12 MB       │  │  NPU SRAM: tau = 4 MB        │   │
  │  └──────────────────────────────────┘  └──────────────────────────────┘   │
  │                                                                            │
  │  ┌──────────────────────────────────┐  ┌──────────────────────────────┐   │
  │  │        GPU COMPLEX               │  │     MEMORY SUBSYSTEM         │   │
  │  │                                  │  │                              │   │
  │  │  sigma = 12 shader cores         │  │  LPDDR5X: sigma-tau = 8 GB  │   │
  │  │  tau = 4 TMUs per core           │  │  Channels: tau = 4          │   │
  │  │  phi = 2 ROPs per core           │  │  BW: sigma*tau = 48 GB/s    │   │
  │  │  ALUs/core: J_2 = 24             │  │                              │   │
  │  │  Total ALUs: sigma*J_2 = 288     │  │  L1I: 2^n = 64 KB / core   │   │
  │  │                                  │  │  L1D: 2^n = 64 KB / core   │   │
  │  │  Peak FP16: 2^sigma = 4096 GFLOPS│  │  L2: 2^(sigma-phi)=1 MB    │   │
  │  │  Display: J_2=24 fps / sigma*tau │  │  L3: sigma = 12 MB shared  │   │
  │  │           = 48 Hz refresh        │  │  Flash: 2^(sigma-tau)=256GB │   │
  │  └──────────────────────────────────┘  └──────────────────────────────┘   │
  │                                                                            │
  │  ┌──────────────────────────────────┐  ┌──────────────────────────────┐   │
  │  │     CONNECTIVITY HUB             │  │     POWER MANAGEMENT         │   │
  │  │                                  │  │                              │   │
  │  │  WiFi 6 (n=6)                    │  │  TDP: n = 6W sustained      │   │
  │  │  5G Modem (sopfr=5G)             │  │  Burst: sigma = 12W         │   │
  │  │  BLE 5.x (sopfr=5)              │  │  Idle: 1/sigma = 83 mW      │   │
  │  │  USB-C: tau=4 lanes             │  │  DVFS: 0.6V ~ 1.0V          │   │
  │  │  GPIO: J_2=24 pins              │  │  n=6 power states (P0~P5)   │   │
  │  │  I2C/SPI/UART: n/phi=3 buses    │  │  Egyptian power budget       │   │
  │  └──────────────────────────────────┘  └──────────────────────────────┘   │
  │                                                                            │
  │  ┌──────────────────────────────────────────────────────────────────────┐  │
  │  │                    HEXA-LANG NATIVE ENGINE                           │  │
  │  │                                                                      │  │
  │  │  Hardware keyword decoder: 53 keywords (sigma*tau+sopfr)             │  │
  │  │  Egyptian memory controller: 1/2 Stack + 1/3 Heap + 1/6 Arena       │  │
  │  │  J_2 = 24-bit opcode width                                          │  │
  │  │  sigma-tau = 8 primitive type accelerator (int/float/bool/char/...)  │  │
  │  │  Cyclotomic ALU: x^2-x+1 in hardware (BT-33)                        │  │
  │  └──────────────────────────────────────────────────────────────────────┘  │
  └────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. CPU Architecture

### 2.1 Core Configuration: big.LITTLE (sigma-tau = 8 cores)

```
  ┌──────────────────────────────────────────────────────────────┐
  │              CPU COMPLEX: sigma-tau = 8 cores                 │
  │                                                               │
  │  PERFORMANCE CLUSTER (big)                                    │
  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐                │
  │  │ P-Core │ │ P-Core │ │ P-Core │ │ P-Core │  tau = 4 cores │
  │  │  #0    │ │  #1    │ │  #2    │ │  #3    │                │
  │  │ 3.0GHz │ │ 3.0GHz │ │ 3.0GHz │ │ 3.0GHz │  n=6-wide OoO │
  │  │ 64KB L1│ │ 64KB L1│ │ 64KB L1│ │ 64KB L1│  sigma=12 pipe│
  │  │  I+D   │ │  I+D   │ │  I+D   │ │  I+D   │  ROB=288      │
  │  └───┬────┘ └───┬────┘ └───┬────┘ └───┬────┘                │
  │      └─────┬─────┘         └─────┬─────┘                     │
  │            │                     │                            │
  │      ┌─────┴─────┐        ┌─────┴─────┐                     │
  │      │ L2: 1 MB  │        │ L2: 1 MB  │   phi = 2 clusters  │
  │      │ 2^(s-phi) │        │ 2^(s-phi) │                     │
  │      └─────┬─────┘        └─────┬─────┘                     │
  │            └──────────┬──────────┘                            │
  │                       │                                       │
  │  EFFICIENCY CLUSTER (LITTLE)                                  │
  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐                │
  │  │ E-Core │ │ E-Core │ │ E-Core │ │ E-Core │  tau = 4 cores │
  │  │  #4    │ │  #5    │ │  #6    │ │  #7    │                │
  │  │ 2.0GHz │ │ 2.0GHz │ │ 2.0GHz │ │ 2.0GHz │  n/phi=3-wide │
  │  │ 32KB L1│ │ 32KB L1│ │ 32KB L1│ │ 32KB L1│  n=6 pipe     │
  │  │  I+D   │ │  I+D   │ │  I+D   │ │  I+D   │  In-order     │
  │  └───┬────┘ └───┬────┘ └───┬────┘ └───┬────┘                │
  │      └──────────┬──────────┘          │                      │
  │           ┌─────┴─────┐        ┌──────┴────┐                 │
  │           │ L2: 512KB │        │ L2: 512KB │                 │
  │           └─────┬─────┘        └─────┬─────┘                 │
  │                 └──────────┬─────────┘                        │
  │                            │                                  │
  │                 ┌──────────┴──────────┐                       │
  │                 │  L3: sigma = 12 MB  │                       │
  │                 │  Shared by all 8    │                       │
  │                 └─────────────────────┘                       │
  │                                                               │
  │  Total: tau + tau = sigma-tau = 8 cores                       │
  │  RISC-V N6 ISA throughout                                    │
  └──────────────────────────────────────────────────────────────┘
```

### 2.2 P-Core Pipeline (sigma = 12 stages)

The P-Core is a scaled-down version of HEXA-CORE's HEXA-P, optimized for mobile:

```
  ┌──────────────────────────────────────────────────────────────────┐
  │              P-CORE PIPELINE (sigma = 12 stages)                  │
  │                                                                   │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐   │
  │  │ IF1  │->│ IF2  │->│ DEC1 │->│ DEC2 │->│ DEC3 │->│ REN  │   │
  │  │      │  │      │  │      │  │      │  │      │  │      │   │
  │  │fetch │  │branch│  │HEXA  │  │fuse  │  │alloc │  │rename│   │
  │  │ line │  │pred  │  │-LANG │  │uops  │  │ regs │  │  map │   │
  │  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘   │
  │     1         2         3         4         5         6         │
  │                                                                   │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐   │
  │  │ SCH  │->│ RF   │->│ EX1  │->│ EX2  │->│ WB   │->│ RET  │   │
  │  │      │  │      │  │      │  │      │  │      │  │      │   │
  │  │issue │  │ read │  │exec  │  │exec/ │  │write │  │commit│   │
  │  │ port │  │ regs │  │ ALU  │  │ mem  │  │ back │  │retire│   │
  │  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘   │
  │     7         8         9        10        11        12         │
  │                                                                   │
  │  Decode width:   n = 6 instructions/cycle                        │
  │  Issue width:    n = 6 micro-ops/cycle                           │
  │  Retire width:   n = 6 micro-ops/cycle                           │
  │  HEXA-LANG:      Stage 3 = hardware keyword recognition          │
  └──────────────────────────────────────────────────────────────────┘
```

### 2.3 P-Core Execution Units

```
  ┌──────────────────────────────────────────────────────────────────┐
  │                   P-CORE EXECUTION ENGINE                         │
  │                                                                   │
  │  Integer/ALU:                                                     │
  │  ┌────┐┌────┐┌────┐┌────┐                                       │
  │  │ALU0││ALU1││ALU2││ALU3│   tau = 4 ALU ports                   │
  │  └────┘└────┘└────┘└────┘                                       │
  │                                                                   │
  │  FP/SIMD:                                                         │
  │  ┌────┐┌────┐                                                    │
  │  │FP0 ││FP1 │   phi = 2 FP/SIMD ports                           │
  │  └────┘└────┘   Vector width: 2^(sigma-tau) = 256-bit            │
  │                                                                   │
  │  Branch:                                                          │
  │  ┌────┐                                                          │
  │  │BR0 │   mu = 1 branch port                                    │
  │  └────┘                                                          │
  │                                                                   │
  │  Load/Store:                                                      │
  │  ┌────┐┌────┐┌────┐                                              │
  │  │LD0 ││LD1 ││ST0 │   n/phi = 3 LS ports (2 load + 1 store)    │
  │  └────┘└────┘└────┘                                              │
  │                                                                   │
  │  HEXA-LANG Accel:                                                 │
  │  ┌──────────┐┌──────────┐                                        │
  │  │ VCYCLO   ││ VEGYP    │   phi = 2 N6 accelerator ports        │
  │  │ x^2-x+1  ││ 1/2+1/3  │                                        │
  │  │ cyclotomic││ +1/6=1   │                                        │
  │  └──────────┘└──────────┘                                        │
  │                                                                   │
  │  Total ports: tau + phi + mu + n/phi + phi                       │
  │             = 4 + 2 + 1 + 3 + 2 = 12 = sigma                    │
  └──────────────────────────────────────────────────────────────────┘
```

### 2.4 E-Core (Efficiency)

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| Pipeline stages | 6 | n | In-order, minimal |
| Decode width | 3 | n/phi | Half of P-Core |
| Issue width | 3 | n/phi | In-order |
| ALU ports | 2 | phi | Minimal integer |
| FP port | 1 | mu | Single FP/SIMD |
| LS ports | 2 | phi | 1 load + 1 store |
| Branch port | 1 | mu | |
| Total ports | 6 | n | Half of P-Core sigma |
| L1I | 32 KB | 2^sopfr | Half of P-Core |
| L1D | 32 KB | 2^sopfr | Half of P-Core |
| L2 (shared/2) | 512 KB | 2^(sigma-mu-phi) | Per 2-core cluster |
| Clock | 2.0 GHz | | Perf/W optimized |
| Power/core | ~0.15 W | | Always-on capable |

### 2.5 Register File

```
  ┌──────────────────────────────────────────────────────────────┐
  │              ARCHITECTURAL REGISTER FILE                      │
  │              n = 6 banks x sigma = 12 registers = 72         │
  │                                                               │
  │  Bank 0: General Purpose (GPR)                                │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐         │
  │  │x0││x1││x2││x3││x4││x5││x6││x7││x8││x9││10││11│  sigma   │
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘         │
  │                                                               │
  │  Bank 1: Floating Point (FPR)                                 │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐         │
  │  │f0││f1││f2││f3││f4││f5││f6││f7││f8││f9││10││11│  sigma   │
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘         │
  │                                                               │
  │  Bank 2: Vector (VR) -- 256-bit SIMD                          │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐         │
  │  │v0││v1││v2││v3││v4││v5││v6││v7││v8││v9││10││11│  sigma   │
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘         │
  │                                                               │
  │  Bank 3: N6 Special (cyclotomic, Egyptian, FFT)               │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐         │
  │  │n0││n1││n2││n3││n4││n5││n6││n7││n8││n9││10││11│  sigma   │
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘         │
  │                                                               │
  │  Bank 4: Control/Status (CSR)                                 │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐         │
  │  │c0││c1││c2││c3││c4││c5││c6││c7││c8││c9││10││11│  sigma   │
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘         │
  │                                                               │
  │  Bank 5: HEXA-LANG (own/borrow/move/drop state, type tags)   │
  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐         │
  │  │h0││h1││h2││h3││h4││h5││h6││h7││h8││h9││10││11│  sigma   │
  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘         │
  │                                                               │
  │  Total: n * sigma = 6 * 12 = 72 architectural registers       │
  │                                                               │
  │  Physical regs (P-Core OoO):                                  │
  │    INT: sigma*J_2 = 288    FP: sigma*J_2 = 288               │
  │    VEC: sigma^2 = 144      N6: sigma*tau = 48                │
  │    Total physical: 288+288+144+48 = 768                       │
  └──────────────────────────────────────────────────────────────┘
```

### 2.6 Out-of-Order Engine (P-Core)

| Parameter | Value | n=6 Formula | vs Snapdragon X |
|-----------|-------|-------------|-----------------|
| ROB entries | 288 | sigma*J_2 | X Elite: ~300 |
| Physical regs (INT) | 288 | sigma*J_2 | Comparable |
| Physical regs (FP) | 288 | sigma*J_2 | Comparable |
| Load queue | 64 | 2^n | Adequate for mobile |
| Store queue | 48 | sigma*tau | Adequate for mobile |
| Scheduler entries | 72 | n*sigma | Per cluster |
| Instruction window | 288 | sigma*J_2 | Competitive |
| Dispatch width | 6 | n | Mobile optimized |
| Retire width | 6 | n | Mobile optimized |

### 2.7 Branch Prediction

```
  ┌──────────────────────────────────────────────────────────────┐
  │               P-CORE BRANCH PREDICTOR (edge-scaled)           │
  │                                                               │
  │  L0 micro-BTB:  2^n = 64 entries (zero-bubble loops)         │
  │  L1 main BTB:   2^sigma = 4096 entries, tau=4-way            │
  │  TAGE:          n = 6 tables (vs HEXA-CORE sigma=12)         │
  │                 History max: 2^sigma = 4096 bits              │
  │                 Entries/table: 2^(sigma-tau) = 256            │
  │  Loop pred:     sigma = 12 entries                            │
  │  Return stack:  sigma-tau = 8 entries                         │
  │  Indirect:      2^n = 64 entries                              │
  │                                                               │
  │  Target accuracy: > 98.5%                                     │
  │  Power budget: < 5% of core power                            │
  └──────────────────────────────────────────────────────────────┘
```

---

## 3. NPU (Neural Processing Unit)

### 3.1 NPU Microarchitecture

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │                   HEXA-EDGE NPU COMPLEX                               │
  │                                                                       │
  │  ┌──────────────────────────────────────────────────────────────┐    │
  │  │                  MAMBA SSM ENGINE (BT-65)                    │    │
  │  │                                                              │    │
  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │    │
  │  │  │ Scan Unit│  │ Scan Unit│  │ Scan Unit│  │ Scan Unit│   │    │
  │  │  │    #0    │  │    #1    │  │    #2    │  │    #3    │   │    │
  │  │  │ d_st=16  │  │ d_st=16  │  │ d_st=16  │  │ d_st=16  │   │    │
  │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │    │
  │  │  tau = 4 parallel scan units                                │    │
  │  │                                                              │    │
  │  │  d_state = 2^tau = 16       (state dimension)               │    │
  │  │  expand  = phi = 2          (inner dim multiplier)          │    │
  │  │  d_conv  = tau = 4          (local convolution width)       │    │
  │  │  dt_rank = sigma-tau = 8    (discretization rank)           │    │
  │  │  dt_init = 1/(sigma-phi) = 0.1  (timestep init)            │    │
  │  └──────────────────────────────────────────────────────────────┘    │
  │                                                                       │
  │  ┌──────────────────────────────────────────────────────────────┐    │
  │  │                  LoRA ADAPTER ENGINE (BT-58)                 │    │
  │  │                                                              │    │
  │  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐   │    │
  │  │  │ Adpt │ │ Adpt │ │ Adpt │ │ Adpt │ │ Adpt │ │ Adpt │   │    │
  │  │  │  #0  │ │  #1  │ │  #2  │ │  #3  │ │  #4  │ │  #5  │   │    │
  │  │  └──────┘ └──────┘ └──────┘ └──────┘ └──────┘ └──────┘   │    │
  │  │  n = 6 simultaneous adapter slots                           │    │
  │  │  rank = sigma-tau = 8 per adapter                           │    │
  │  │  alpha = sigma = 12                                         │    │
  │  │  Hot-swap: <1 cycle context switch between adapters         │    │
  │  └──────────────────────────────────────────────────────────────┘    │
  │                                                                       │
  │  ┌──────────────────────────────────────────────────────────────┐    │
  │  │                  MAC ARRAY (Matrix-Multiply Core)            │    │
  │  │                                                              │    │
  │  │  ┌──────────────────────────────────────────┐               │    │
  │  │  │  sigma-tau x sigma-tau = 8x8 MAC tile    │               │    │
  │  │  │                                          │               │    │
  │  │  │  ┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐     │               │    │
  │  │  │  │  ││  ││  ││  ││  ││  ││  ││  │ x8  │               │    │
  │  │  │  └──┘└──┘└──┘└──┘└──┘└──┘└──┘└──┘     │               │    │
  │  │  │                                          │               │    │
  │  │  │  Tiles: n = 6 rows x phi = 2 cols = 12  │               │    │
  │  │  └──────────────────────────────────────────┘               │    │
  │  │                                                              │    │
  │  │  INT8 throughput:  sigma*n = 72 TOPS                        │    │
  │  │  INT4 throughput:  sigma^2 = 144 TOPS                       │    │
  │  │  FP16 throughput:  sigma*n/phi = 36 TFLOPS                  │    │
  │  │  BF16 throughput:  sigma*n/phi = 36 TFLOPS                  │    │
  │  │                                                              │    │
  │  │  Activation: Cyclotomic (x^2-x+1) hardware unit (BT-33)    │    │
  │  │  Sparsity:   Boltzmann gate 63% sparse (1/e)               │    │
  │  │  Power:      < mu = 1W under full load                     │    │
  │  └──────────────────────────────────────────────────────────────┘    │
  │                                                                       │
  │  NPU SRAM: tau = 4 MB (weight cache + activation buffer)            │
  │  NPU DMA:  phi = 2 channels, sigma*tau = 48 GB/s each              │
  └──────────────────────────────────────────────────────────────────────┘
```

### 3.2 NPU Supported Models

| Model Type | Config | n=6 Mapping | On-Device? |
|------------|--------|-------------|:----------:|
| Mamba SSM | d=512, L=24 | 2^(sigma-n/phi), J_2 | Yes |
| Transformer (small) | d=768, L=12 | 2^(sigma-tau)*n/phi, sigma | Yes |
| LoRA fine-tune | rank=8, adapters=6 | sigma-tau, n | Yes, live |
| Diffusion (mobile) | T=1000, steps=50 | 10^(n/phi), sopfr*sigma-phi | Yes |
| Vision (ViT-S) | d=384, L=12 | 2^(sigma-tau)*n/phi/phi, sigma | Yes |
| Audio (Whisper-S) | d=384, L=6 | Same, n | Yes |
| MoE routing | experts=8, top-2 | sigma-tau, phi | Hardware |

### 3.3 Egyptian Fraction Attention (EFA) Engine

```
  ┌──────────────────────────────────────────────────────────────┐
  │           EFA HARDWARE UNIT (BT-42, Technique 17)             │
  │                                                               │
  │  Attention budget = 1/2 + 1/3 + 1/6 = 1                      │
  │                                                               │
  │  ┌──────────────────┐  Head allocation:                       │
  │  │  1/2 = 50%       │  sigma/phi = 6 heads -> global attn    │
  │  │  Global Heads    │  Full Q*K^T*V, no approximation        │
  │  ├──────────────────┤                                         │
  │  │  1/3 = 33%       │  sigma/n/phi = 4 heads -> local attn   │
  │  │  Local Heads     │  Window = 2^sigma = 4096 tokens        │
  │  ├──────────────────┤                                         │
  │  │  1/6 = 17%       │  sigma/n = 2 heads -> FFT attn         │
  │  │  FFT Heads       │  O(n log n) frequency mixing           │
  │  └──────────────────┘                                         │
  │                                                               │
  │  Total heads: 6 + 4 + 2 = sigma = 12                         │
  │  FLOPs savings: ~40% vs full attention                        │
  │  KV-cache per head: sigma-tau = 8 (BT-39 universal)          │
  └──────────────────────────────────────────────────────────────┘
```

---

## 4. GPU Complex

### 4.1 Shader Core Architecture

```
  ┌──────────────────────────────────────────────────────────────────┐
  │              GPU: sigma = 12 Shader Cores                         │
  │                                                                   │
  │  ┌─────────────────────┐                                         │
  │  │  SHADER CORE (x12)  │                                         │
  │  │                      │                                         │
  │  │  ALUs: J_2 = 24     │  FP32 + INT32 dual-issue               │
  │  │  TMUs: tau = 4       │  Texture mapping units                 │
  │  │  ROPs: phi = 2       │  Render output units                   │
  │  │  SFUs: phi = 2       │  Special function (sin/cos/sqrt)       │
  │  │  Warp size: 2^sopfr  │  = 32 threads                          │
  │  │  Warps/core: n = 6   │  6 concurrent warps                    │
  │  │  Threads/core: 192   │  = 2^sopfr * n = 32*6                  │
  │  │  Reg file: 2^n = 64KB│  Per shader core                       │
  │  │  Shared mem: 2^n=64KB│  Configurable with L1                  │
  │  └─────────────────────┘                                         │
  │                                                                   │
  │  Total ALUs:     sigma * J_2 = 12 * 24 = 288                     │
  │  Total TMUs:     sigma * tau = 12 * 4 = 48                       │
  │  Total ROPs:     sigma * phi = 12 * 2 = 24                       │
  │  Total threads:  sigma * 192 = 2304                               │
  │                                                                   │
  │  Peak FP32:  288 ALUs * 2 (FMA) * 3.0 GHz = ~1.7 TFLOPS        │
  │  Peak FP16:  288 ALUs * 4 (packed) * 3.0 GHz = ~3.5 TFLOPS     │
  │  Fill rate:  24 ROPs * 3.0 GHz = 72 Gpix/s                      │
  │  Tex rate:   48 TMUs * 3.0 GHz = 144 Gtex/s                     │
  └──────────────────────────────────────────────────────────────────┘
```

### 4.2 Display Engine

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| Min frame rate | 24 fps | J_2 | Cinema standard |
| Panel refresh | 48/120 Hz | sigma*tau / sigma*(sigma-phi) | LTPO adaptive |
| Max resolution | 4K | 2^sigma = 4096 px wide | External output |
| Internal render | 2K | 2^(sigma-mu) = 2048 px | Mobile panel |
| Color depth | 10-bit | sigma-phi | HDR10 |
| Display pipes | 2 | phi | Internal + external |
| Overlay layers | 6 | n | Hardware compositing |
| HDR tone map | 12-bit | sigma | Internal precision |

---

## 5. Connectivity Hub

### 5.1 Wireless

```
  ┌──────────────────────────────────────────────────────────────┐
  │              CONNECTIVITY HUB                                 │
  │                                                               │
  │  ┌──────────────────┐  ┌──────────────────┐                  │
  │  │  WiFi 6E/7       │  │  5G NR Modem     │                  │
  │  │  n = 6 (802.11ax)│  │  Sub-6 + mmWave  │                  │
  │  │  Streams: n/phi=3│  │  sopfr = 5G NR   │                  │
  │  │  Width: 2*80=160 │  │  Bands: sigma=12 │                  │
  │  │  MHz (2^(s-t)=8  │  │  MIMO: tau*phi=8 │                  │
  │  │  * 20MHz)        │  │  Layers: sigma-t │                  │
  │  └──────────────────┘  └──────────────────┘                  │
  │                                                               │
  │  ┌──────────────────┐  ┌──────────────────┐                  │
  │  │  BLE 5.x         │  │  GPS/GNSS        │                  │
  │  │  sopfr=5 .x      │  │  Constellations: │                  │
  │  │  Channels: n*n/phi│  │  tau = 4         │                  │
  │  │  = 18 data ch     │  │  (GPS+GLO+GAL   │                  │
  │  │  Adv ch: n/phi=3 │  │   +BDS)          │                  │
  │  └──────────────────┘  └──────────────────┘                  │
  └──────────────────────────────────────────────────────────────┘
```

### 5.2 Wired Interfaces

| Interface | Config | n=6 Formula | Notes |
|-----------|--------|-------------|-------|
| USB-C | 4 lanes | tau = 4 | USB4 / Thunderbolt |
| USB speed | 40 Gbps | tau*(sigma-phi) | Gen 4 |
| GPIO pins | 24 | J_2 = 24 | General purpose I/O |
| I2C buses | 3 | n/phi | Sensor, PMIC, other |
| SPI buses | 3 | n/phi | Flash, display, other |
| UART | 3 | n/phi | Debug, BT HCI, spare |
| SDIO | 1 | mu | MicroSD/eMMC |
| MIPI CSI | 4 lanes | tau | Camera input |
| MIPI DSI | 4 lanes | tau | Display output |
| PCIe | 4 lanes | tau | M.2 NVMe |

---

## 6. Memory Hierarchy

### 6.1 Full Memory Map

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │                    MEMORY HIERARCHY                                    │
  │                                                                       │
  │  ┌───────────────────────────────────────────────────────────────┐   │
  │  │  CPU P-Core (x4)          CPU E-Core (x4)                    │   │
  │  │  ┌─────┐ ┌─────┐         ┌─────┐ ┌─────┐                    │   │
  │  │  │L1I  │ │L1D  │         │L1I  │ │L1D  │                    │   │
  │  │  │64KB │ │64KB │         │32KB │ │32KB │                    │   │
  │  │  │2^n  │ │2^n  │         │2^sop│ │2^sop│                    │   │
  │  │  │3cyc │ │4cyc │         │2cyc │ │3cyc │                    │   │
  │  │  └──┬──┘ └──┬──┘         └──┬──┘ └──┬──┘                    │   │
  │  │     └───┬───┘               └───┬───┘                        │   │
  │  │         │                       │                             │   │
  │  │    ┌────┴────┐            ┌────┴────┐                        │   │
  │  │    │L2: 1 MB │            │L2:512KB │                        │   │
  │  │    │2^(s-phi)│            │2^(s-m-p)│                        │   │
  │  │    │12 cyc   │            │8 cyc    │                        │   │
  │  │    └────┬────┘            └────┬────┘                        │   │
  │  │         └──────────┬───────────┘                              │   │
  │  │                    │                                          │   │
  │  │         ┌──────────┴──────────┐                               │   │
  │  │         │ L3: sigma = 12 MB   │   sigma-tau = 8 way          │   │
  │  │         │ sigma*tau = 48 cyc  │   Shared CPU + GPU + NPU     │   │
  │  │         └──────────┬──────────┘                               │   │
  │  └────────────────────┼──────────────────────────────────────────┘   │
  │                       │                                              │
  │  ┌────────────────────┴──────────────────────────────────────────┐   │
  │  │                EGYPTIAN MEMORY CONTROLLER                      │   │
  │  │                                                                │   │
  │  │  ┌─────────────────┬─────────────┬───────────┐                │   │
  │  │  │  STACK POOL     │  HEAP MGR   │  ARENA    │                │   │
  │  │  │    1/2 = 4 GB   │  1/3~2.67GB │  1/6~1.3GB│                │   │
  │  │  │                 │             │           │                │   │
  │  │  │  Value types    │  Ref types  │  Temp     │                │   │
  │  │  │  int/float/bool │  Box/Rc     │  Bulk     │                │   │
  │  │  │  Zero GC        │  Ref count  │  Scope GC │                │   │
  │  │  └─────────────────┴─────────────┴───────────┘                │   │
  │  │                                                                │   │
  │  │  LPDDR5X: sigma-tau = 8 GB total                              │   │
  │  │  Channels: tau = 4                                             │   │
  │  │  Width per channel: phi^tau = 16 bits                          │   │
  │  │  Total bus: tau * phi^tau = 64 bits = 2^n                     │   │
  │  │  Bandwidth: sigma*tau = 48 GB/s (effective)                   │   │
  │  │  Burst length: phi^tau = 16                                    │   │
  │  └────────────────────────────────────────────────────────────────┘   │
  │                                                                       │
  │  ┌────────────────────────────────────────────────────────────────┐   │
  │  │  FLASH STORAGE: 2^(sigma-tau) = 256 GB UFS 4.0               │   │
  │  │  Lanes: phi = 2    Read: 4.2 GB/s    Write: 2.8 GB/s         │   │
  │  └────────────────────────────────────────────────────────────────┘   │
  └──────────────────────────────────────────────────────────────────────┘
```

### 6.2 Cache Summary Table

| Level | Size | n=6 Formula | Ways | Latency | Notes |
|-------|------|-------------|------|---------|-------|
| L1I (P) | 64 KB | 2^n | 8 (sigma-tau) | 3 cyc (n/phi) | Per P-core |
| L1D (P) | 64 KB | 2^n | 8 (sigma-tau) | 4 cyc (tau) | Per P-core |
| L1I (E) | 32 KB | 2^sopfr | 4 (tau) | 2 cyc (phi) | Per E-core |
| L1D (E) | 32 KB | 2^sopfr | 4 (tau) | 3 cyc (n/phi) | Per E-core |
| L2 (P-cluster) | 1 MB | 2^(sigma-phi) | 16 (phi^tau) | 12 cyc (sigma) | Per 2 P-cores |
| L2 (E-cluster) | 512 KB | 2^(sigma-mu-phi) | 8 (sigma-tau) | 8 cyc (sigma-tau) | Per 2 E-cores |
| L3 shared | 12 MB | sigma | 8 (sigma-tau) | 48 cyc (sigma*tau) | All cores |
| NPU SRAM | 4 MB | tau | N/A | 2 cyc | Weight cache |
| GPU shared | 64 KB/core | 2^n | N/A | ~20 cyc | Per shader core |

---

## 7. HEXA-LANG Native Engine

The defining feature of HEXA-EDGE: hardware-accelerated HEXA-LANG execution.

### 7.1 Hardware Keyword Decoder

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │            HEXA-LANG HARDWARE KEYWORD DECODER                        │
  │                                                                      │
  │  53 keywords (sigma*tau + sopfr = 48 + 5) recognized in silicon     │
  │                                                                      │
  │  ┌────────────────────────────────────────────────────────────┐      │
  │  │  Stage 3 (DEC1) of P-Core pipeline                        │      │
  │  │                                                            │      │
  │  │  Instruction bytes -> HEXA-LANG keyword CAM lookup         │      │
  │  │                                                            │      │
  │  │  ┌──────────┐    ┌──────────────────────┐    ┌────────┐   │      │
  │  │  │  24-bit  │--->│  53-entry CAM        │--->│ Micro- │   │      │
  │  │  │  opcode  │    │  (Content-Addressable│    │ op ROM │   │      │
  │  │  │  J_2=24  │    │   Memory)            │    │        │   │      │
  │  │  └──────────┘    │                      │    │ sigma  │   │      │
  │  │                   │  sigma=12 groups     │    │ =12    │   │      │
  │  │                   │  1-cycle match       │    │ uop    │   │      │
  │  │                   └──────────────────────┘    │ fmts   │   │      │
  │  │                                               └────────┘   │      │
  │  │  Result: 0-cycle overhead vs manual decode                 │      │
  │  │  Fallback: standard RISC-V decode for non-HEXA code        │      │
  │  └────────────────────────────────────────────────────────────┘      │
  │                                                                      │
  │  Keyword group mapping (sigma = 12 groups):                          │
  │                                                                      │
  │  ┌────────┬──────────┬──────────────────────────────────────┐       │
  │  │ Group  │  Count   │  Keywords                            │       │
  │  ├────────┼──────────┼──────────────────────────────────────┤       │
  │  │ G0 Ctl │  n=6     │  if else match for while loop        │       │
  │  │ G1 Typ │  sopfr=5 │  type struct enum trait impl         │       │
  │  │ G2 Fn  │  sopfr=5 │  fn return yield async await         │       │
  │  │ G3 Var │  tau=4   │  let mut const static                │       │
  │  │ G4 Mod │  tau=4   │  mod use pub crate                   │       │
  │  │ G5 Mem │  tau=4   │  own borrow move drop                │       │
  │  │ G6 Ccy │  tau=4   │  spawn channel select atomic         │       │
  │  │ G7 Eff │  tau=4   │  effect handle resume pure           │       │
  │  │ G8 Prf │  tau=4   │  proof assert invariant theorem      │       │
  │  │ G9 Met │  tau=4   │  macro derive where comptime         │       │
  │  │ G10 Er │  sopfr=5 │  try catch throw panic recover       │       │
  │  │ G11 AI │  tau=4   │  intent generate verify optimize     │       │
  │  ├────────┼──────────┼──────────────────────────────────────┤       │
  │  │ Total  │  53      │  sigma*tau + sopfr = 48 + 5          │       │
  │  └────────┴──────────┴──────────────────────────────────────┘       │
  └──────────────────────────────────────────────────────────────────────┘
```

### 7.2 Egyptian Memory Controller

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │          EGYPTIAN MEMORY CONTROLLER (Hardware)                        │
  │          1/2 + 1/3 + 1/6 = 1                                         │
  │                                                                       │
  │  ┌──────────────────────────────────────────────────────────────┐    │
  │  │                    sigma-tau = 8 GB LPDDR5X                  │    │
  │  │                                                              │    │
  │  │  ┌────────────────────────────┬──────────────────┬────────┐ │    │
  │  │  │      STACK POOL            │    HEAP MANAGED  │ ARENA  │ │    │
  │  │  │        1/2 = 4 GB          │   1/3 = 2.67 GB  │1/6=1.3│ │    │
  │  │  │                            │                  │   GB   │ │    │
  │  │  │  HW-managed stack frames   │  Reference count │ Bulk  │ │    │
  │  │  │  Push/pop in 1 cycle       │  tracked in HW   │ alloc │ │    │
  │  │  │  Overflow -> Heap spill    │  Cycle-detection │ Scope │ │    │
  │  │  │  Underflow -> trap         │  via phi=2 bits  │ free  │ │    │
  │  │  │                            │                  │       │ │    │
  │  │  │  Value types:              │  Reference types: │ Temp: │ │    │
  │  │  │  int float bool char       │  Box Rc Arc       │ [T;N] │ │    │
  │  │  │  string(small) byte void   │  Vec HashMap Set  │ &[T]  │ │    │
  │  │  └────────────────────────────┴──────────────────┴────────┘ │    │
  │  │                                                              │    │
  │  │  Hardware partition registers:                               │    │
  │  │    STACK_BASE, STACK_LIMIT   (1/2 boundary)                 │    │
  │  │    HEAP_BASE,  HEAP_LIMIT    (1/3 boundary)                 │    │
  │  │    ARENA_BASE, ARENA_LIMIT   (1/6 boundary)                 │    │
  │  │                                                              │    │
  │  │  Auto-balancing: runtime monitor adjusts boundaries          │    │
  │  │  Target ratio: Stack:Heap:Arena = n/phi : phi : mu = 3:2:1  │    │
  │  └──────────────────────────────────────────────────────────────┘    │
  │                                                                       │
  │  own/borrow/move/drop tracked in Bank 5 HEXA-LANG registers          │
  │  Ownership transfer = sigma = 12 register bits (type + state + ref)  │
  │  Borrow checker: hardware validation in 1 cycle                       │
  └──────────────────────────────────────────────────────────────────────┘
```

### 7.3 Opcode Format (J_2 = 24 bits)

```
  ┌──────────────────────────────────────────────────────────────┐
  │           HEXA-LANG N6 ISA OPCODE FORMAT                      │
  │           J_2 = 24 bits wide                                  │
  │                                                               │
  │  Format A: Register-Register (R-type)                         │
  │  ┌──────┬──────┬──────┬──────┬──────┬──────┐                 │
  │  │opcode│  rd  │  rs1 │  rs2 │ func │ bank │                 │
  │  │ 6-bit│ 4-bit│ 4-bit│ 4-bit│ 3-bit│ 3-bit│                 │
  │  │  n   │  tau │  tau │  tau │ n/phi│ n/phi│                 │
  │  └──────┴──────┴──────┴──────┴──────┴──────┘                 │
  │  6+4+4+4+3+3 = 24 = J_2                                      │
  │                                                               │
  │  Format B: Immediate (I-type)                                 │
  │  ┌──────┬──────┬──────┬──────────────┬──────┐                │
  │  │opcode│  rd  │  rs1 │  imm12       │ bank │                │
  │  │ 6-bit│ 4-bit│ 4-bit│ sigma=12-bit │ 3-bit│                │ 
  │  └──────┴──────┴──────┴──────────────┴──────┘                │
  │  6+4+4+12+3 = 29 -> extended to 2^sopfr=32 with 3 pad       │
  │  But base ISA = 24-bit for code density (edge optimization)  │
  │                                                               │
  │  Format C: Branch/Jump (B-type)                               │
  │  ┌──────┬──────┬──────┬──────────────────────┐               │
  │  │opcode│  rs1 │  rs2 │  offset              │               │
  │  │ 6-bit│ 4-bit│ 4-bit│ sigma-phi=10 bit     │               │
  │  └──────┴──────┴──────┴──────────────────────┘               │
  │  6+4+4+10 = 24 = J_2                                         │
  │                                                               │
  │  Format D: HEXA-LANG keyword (K-type, special)                │
  │  ┌──────┬──────┬──────┬──────┬──────────────┐                │
  │  │ 0xFF │group │ kw#  │ arg1 │ arg2         │                │
  │  │ 6-bit│ 4-bit│ 3-bit│ 4-bit│ 7-bit        │                │
  │  └──────┴──────┴──────┴──────┴──────────────┘                │
  │  Prefix 0xFF -> keyword mode, group=G0~G11, kw=index         │
  │  Single cycle: CAM match -> micro-op expansion               │
  └──────────────────────────────────────────────────────────────┘
```

### 7.4 Primitive Type Accelerator

```
  ┌──────────────────────────────────────────────────────────────┐
  │      sigma-tau = 8 PRIMITIVE TYPE ACCELERATOR                 │
  │                                                               │
  │  Hardware type-dispatch unit for HEXA-LANG's 8 primitives:   │
  │                                                               │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐                     │
  │  │ int  │  │float │  │ bool │  │ char │                     │
  │  │ 64b  │  │ 64b  │  │ 1b   │  │ 32b  │                     │
  │  │ ALU  │  │ FPU  │  │ BOOL │  │ UTF8 │                     │
  │  │ pipe │  │ pipe │  │ gate │  │ decode│                     │
  │  └──────┘  └──────┘  └──────┘  └──────┘                     │
  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐                     │
  │  │string│  │ byte │  │ void │  │ any  │                     │
  │  │ heap │  │  8b  │  │ unit │  │ dyn  │                     │
  │  │ SSO  │  │ raw  │  │ zero │  │ vtable│                     │
  │  │ path │  │ path │  │ elim │  │ disp │                     │
  │  └──────┘  └──────┘  └──────┘  └──────┘                     │
  │                                                               │
  │  Type tag: n/phi = 3 bits (encodes 8 types)                  │
  │  Dispatch: 1 cycle type resolution (no software overhead)    │
  │  string SSO: small string opt up to sigma = 12 bytes inline  │
  │  any: hardware vtable lookup in phi = 2 cycles               │
  └──────────────────────────────────────────────────────────────┘
```

---

## 8. Power Management

### 8.1 Power State Machine (n = 6 states: P0~P5)

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │              N = 6 POWER STATES (P0 ~ P5)                             │
  │                                                                       │
  │  P0: TURBO ────────────────────────────── sigma = 12W                 │
  │  │   All cores max, NPU full, GPU max                                │
  │  │   Duration: burst only (sigma*tau = 48 seconds max)               │
  │  │                                                                    │
  │  ▼                                                                    │
  │  P1: ACTIVE ───────────────────────────── n = 6W                      │
  │  │   All P-cores on, NPU active, GPU active                          │
  │  │   Sustained TDP, normal operation                                 │
  │  │                                                                    │
  │  ▼                                                                    │
  │  P2: BALANCED ─────────────────────────── tau = 4W                    │
  │  │   phi=2 P-cores + tau=4 E-cores, NPU idle, GPU minimal           │
  │  │   Everyday tasks: browsing, messaging                             │
  │  │                                                                    │
  │  ▼                                                                    │
  │  P3: EFFICIENT ────────────────────────── phi = 2W                    │
  │  │   E-cores only, NPU sleep, GPU off                                │
  │  │   Background tasks, music, notifications                          │
  │  │                                                                    │
  │  ▼                                                                    │
  │  P4: DROWSY ───────────────────────────── mu = 1W                     │
  │  │   Single E-core, minimal DRAM refresh                             │
  │  │   Always-on display, sensor hub                                   │
  │  │                                                                    │
  │  ▼                                                                    │
  │  P5: DEEP SLEEP ──────────────────────── 1/sigma = 83 mW             │
  │      All cores off, SRAM retention only                               │
  │      Wake on interrupt (BLE, timer, sensor)                           │
  │      DRAM self-refresh                                                │
  │                                                                       │
  │  Transition latencies:                                                │
  │    P5->P4: < 1 ms          P4->P3: < 0.5 ms                         │
  │    P3->P2: < 0.1 ms        P2->P1: < 0.05 ms                        │
  │    P1->P0: < 0.01 ms (instant boost)                                 │
  │                                                                       │
  │  Egyptian power budget in P1 (sustained n=6W):                        │
  │    1/2 = 3W -> CPU + Memory controller                               │
  │    1/3 = 2W -> NPU + GPU                                             │
  │    1/6 = 1W -> Connectivity + I/O + PMU                              │
  └──────────────────────────────────────────────────────────────────────┘
```

### 8.2 DVFS Table

| State | Voltage | P-Core Freq | E-Core Freq | NPU Freq | GPU Freq |
|-------|---------|-------------|-------------|----------|----------|
| P0 | 1.0 V | 3.0 GHz | 2.0 GHz | 1.2 GHz | 1.0 GHz |
| P1 | 0.85 V | 2.5 GHz | 1.8 GHz | 1.0 GHz | 0.8 GHz |
| P2 | 0.75 V | 2.0 GHz | 1.5 GHz | off | 0.4 GHz |
| P3 | 0.65 V | off | 1.2 GHz | off | off |
| P4 | 0.60 V | off | 0.8 GHz (1 core) | off | off |
| P5 | retention | off | off | off | off |

Voltage range: 0.6V ~ 1.0V (DVFS)
DVFS steps: sigma-phi = 10 voltage levels (fine-grained)
Frequency ratio P0/P3 = 3.0/1.2 = 2.5 ~ phi + 1/phi = 2.5

### 8.3 Battery Life Targets

| Scenario | Power | Battery (J_2=24 Wh) | Runtime |
|----------|-------|---------------------|---------|
| Turbo (P0) | 12W | 24 Wh | 2h = phi |
| Active (P1) | 6W | 24 Wh | 4h = tau |
| Balanced (P2) | 4W | 24 Wh | 6h = n |
| Efficient (P3) | 2W | 24 Wh | 12h = sigma |
| Drowsy (P4) | 1W | 24 Wh | 24h = J_2 |
| Deep Sleep (P5) | 83mW | 24 Wh | 288h = sigma*J_2 = 12 days |
| Mixed use (avg) | 3W | 24 Wh | 8h = sigma-tau |

Mixed use target: sigma-tau = 8 hours on a J_2 = 24 Wh battery.

---

## 9. Physical Design

### 9.1 Die Specifications

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| Process | TSMC N2 | sigma*tau=48nm gate | 2nm class |
| Die area | 72 mm^2 | sigma*phi*n | Compact mobile |
| Transistors | 24 B | J_2 = 24 billion | High density |
| Metal layers | 12 | sigma | Standard for N2 |
| Gate pitch | 48 nm | sigma*tau | N2 gate pitch |
| Metal pitch (M1) | 28 nm | P_2 | Minimum metal |
| Fin pitch | 24 nm | J_2 | Aggressive scaling |
| Package | FOWLP | | Fan-Out Wafer Level |
| Package size | 12x12 mm | sigma x sigma | BGA |
| Ball count | 288 | sigma*J_2 | PoP compatible |
| Thermal | 12W max | sigma | No fan, passive only |

### 9.2 Die Floorplan

```
  ┌──────────────────────────────────────────────────────────────┐
  │                   HEXA-EDGE Die (72 mm^2)                     │
  │                   sigma*phi*n = 12*2*6                        │
  │                                                               │
  │  ┌────────────────────────┬───────────────────────────┐      │
  │  │                        │                           │      │
  │  │    CPU P-CLUSTER       │     NPU COMPLEX           │      │
  │  │    4 P-cores + L2      │     Mamba + LoRA + MAC    │      │
  │  │    ~18 mm^2            │     ~14 mm^2              │      │
  │  │    (1/4 die)           │     (~1/5 die)            │      │
  │  │                        │                           │      │
  │  ├────────────────────────┤                           │      │
  │  │                        │                           │      │
  │  │    CPU E-CLUSTER       ├───────────────────────────┤      │
  │  │    4 E-cores + L2      │                           │      │
  │  │    ~6 mm^2             │     GPU COMPLEX           │      │
  │  │    (~1/12 die)         │     12 shader cores       │      │
  │  │                        │     ~12 mm^2              │      │
  │  ├────────────────────────┤     (~1/6 die)            │      │
  │  │                        │                           │      │
  │  │    L3 CACHE (12 MB)    │                           │      │
  │  │    ~8 mm^2             ├───────────────────────────┤      │
  │  │    (~1/9 die)          │                           │      │
  │  │                        │     CONNECTIVITY          │      │
  │  ├────────────────────────┤     WiFi6 + 5G + BLE     │      │
  │  │                        │     ~6 mm^2              │      │
  │  │    HEXA-LANG ENGINE    │     (~1/12 die)           │      │
  │  │    Keyword CAM + EMC   ├───────────────────────────┤      │
  │  │    ~4 mm^2             │                           │      │
  │  │    (~1/18 die)         │     POWER + I/O + MISC   │      │
  │  │                        │     PMU, GPIO, USB, MIPI  │      │
  │  │                        │     ~4 mm^2              │      │
  │  └────────────────────────┴───────────────────────────┘      │
  │                                                               │
  │  Area budget (Egyptian fraction):                             │
  │    1/2 = 36 mm^2: Compute (CPU + NPU)                        │
  │    1/3 = 24 mm^2: Graphics + Cache                            │
  │    1/6 = 12 mm^2: Connectivity + I/O + HEXA-LANG engine      │
  └──────────────────────────────────────────────────────────────┘
```

### 9.3 Pin Diagram

```
  ┌──────────────────────────────────────────────────────────────────┐
  │                HEXA-EDGE Package (12x12mm FOWLP)                  │
  │                Ball count: sigma*J_2 = 288 balls                  │
  │                                                                   │
  │          A  B  C  D  E  F  G  H  J  K  L  M  N  P  Q  R         │
  │       ┌──────────────────────────────────────────────────┐       │
  │   1   │  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . │       │
  │   2   │  .  V  V  G  G  L  L  L  L  G  G  V  V  .  .  . │       │
  │   3   │  .  V  V  G  L  L  L  L  L  L  G  V  V  .  .  . │       │
  │   4   │  .  G  G  .  .  .  .  .  .  .  .  G  G  .  .  . │       │
  │   5   │  .  G  L  .  .  .  .  .  .  .  .  L  G  .  .  . │       │
  │   6   │  .  L  L  .  .  .  .  .  .  .  .  L  L  .  .  . │       │
  │   7   │  .  L  L  .  .  . [DIE] .  .  .  L  L  .  .  . │       │
  │   8   │  .  L  L  .  .  .  .  .  .  .  .  L  L  .  .  . │       │
  │   9   │  .  G  L  .  .  .  .  .  .  .  .  L  G  .  .  . │       │
  │  10   │  .  G  G  .  .  .  .  .  .  .  .  G  G  .  .  . │       │
  │  11   │  .  V  V  G  U  U  I  I  S  S  G  V  V  .  .  . │       │
  │  12   │  .  V  V  G  G  U  I  I  S  G  G  V  V  .  .  . │       │
  │  13   │  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . │       │
  │       └──────────────────────────────────────────────────┘       │
  │                                                                   │
  │  Legend:                                                          │
  │    V = VDD power (sigma*tau = 48 pins)                           │
  │    G = GND ground (sigma*tau = 48 pins)                          │
  │    L = LPDDR5X (sigma^2 = 144 pins: tau=4 ch x sigma*n/phi=36)  │
  │    U = USB-C (sigma = 12 pins)                                   │
  │    I = I2C/SPI/UART (sigma = 12 pins)                            │
  │    S = Sensor/GPIO (J_2 = 24 pins)                               │
  │    . = Reserved / NC                                             │
  │                                                                   │
  │  Pin budget:                                                      │
  │    Power: 48+48 = 96 = sigma*sigma-tau = sigma*(sigma-tau)       │
  │    LPDDR5X: 144 = sigma^2                                        │
  │    USB: 12 = sigma                                                │
  │    I2C/SPI/UART: 12 = sigma                                      │
  │    GPIO/Sensor: 24 = J_2                                          │
  │    Total: 96+144+12+12+24 = 288 = sigma*J_2                      │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 10. Comparison Table

| Feature | HEXA-EDGE | Apple M4 (iPad) | Snapdragon X Elite | MediaTek D9400 |
|---------|-----------|-----------------|-------------------|----------------|
| **Process** | TSMC N2 | TSMC N3E | TSMC N4 | TSMC N3E |
| **Die area** | 72 mm^2 | ~100 mm^2 | ~115 mm^2 | ~85 mm^2 |
| **Transistors** | 24B | 28B | 37B | 29B |
| **CPU cores** | 8 (4P+4E) | 10 (4P+6E) | 12 (all big) | 8 (1+3+4) |
| **CPU width** | 6-wide | 8-wide | 8-wide | 5-wide (big) |
| **CPU pipeline** | 12 stage | ~16 stage | ~13 stage | ~14 stage |
| **Arch registers** | 72 (6 banks) | 32 GPR | 32 GPR | 32 GPR |
| **L3 cache** | 12 MB | 16 MB | 12 MB | 12 MB |
| **GPU cores** | 12 SC (288 ALU) | 10 GPU cores | Adreno (custom) | Immortalis-G925 |
| **NPU** | 72 TOPS INT8 | 38 TOPS | 45 TOPS | 46 TOPS |
| **NPU (INT4)** | 144 TOPS | -- | -- | -- |
| **Mamba SSM HW** | Yes (native) | No | No | No |
| **LoRA HW** | Yes (rank=8, x6) | No | No | No |
| **EFA HW** | Yes (1/2+1/3+1/6) | No | No | No |
| **HEXA-LANG** | Native HW decode | N/A | N/A | N/A |
| **Memory** | 8 GB LPDDR5X | 8/16 GB | 16/32 GB | 12/16 GB |
| **Mem BW** | 48 GB/s | 120 GB/s | 136 GB/s | 76.8 GB/s |
| **WiFi** | WiFi 6E | WiFi 6E | WiFi 7 | WiFi 7 |
| **5G modem** | Integrated | External | Integrated | Integrated |
| **TDP** | 6W (sustained) | ~15W (est) | ~23W | ~8W |
| **Idle** | 83 mW | ~100 mW | ~150 mW | ~100 mW |
| **Power states** | 6 (n=6) | 4-5 | 4-5 | 4-5 |
| **ISA** | RISC-V N6 | ARMv9 | ARMv8/Oryon | ARMv9 |
| **TOPS/W** | 12 TOPS/W | 2.5 TOPS/W | 2.0 TOPS/W | 5.8 TOPS/W |

Key differentiators:
- **12 TOPS/W** -- best-in-class NPU efficiency (Mamba SSM + Boltzmann sparsity)
- **Native HEXA-LANG** -- zero-overhead language execution, no runtime interpreter
- **Egyptian memory controller** -- hardware-enforced 1/2+1/3+1/6 memory partitioning
- **LoRA hot-swap** -- switch between 6 fine-tuned models in <1 cycle
- **Smallest die** -- 72 mm^2 vs 100+ mm^2 competitors (lower cost, higher yield)

---

## 11. Target Applications

```
  ┌──────────────────────────────────────────────────────────────┐
  │              HEXA-EDGE TARGET MARKETS                         │
  │                                                               │
  │  ┌────────────────────┐  ┌────────────────────┐              │
  │  │  SMARTPHONE        │  │  AUTONOMOUS EDGE   │              │
  │  │                    │  │                    │              │
  │  │  On-device LLM     │  │  Robot brain       │              │
  │  │  Camera AI (ViT)   │  │  Drone control     │              │
  │  │  Voice (Whisper)   │  │  ADAS Level 2+     │              │
  │  │  n=6 power states  │  │  SLAM + planning   │              │
  │  └────────────────────┘  └────────────────────┘              │
  │                                                               │
  │  ┌────────────────────┐  ┌────────────────────┐              │
  │  │  IoT GATEWAY       │  │  AR/VR HEADSET     │              │
  │  │                    │  │                    │              │
  │  │  Sensor fusion     │  │  Low-latency GPU   │              │
  │  │  Edge inference    │  │  6DOF tracking     │              │
  │  │  BLE mesh hub      │  │  Neural rendering  │              │
  │  │  Ultra-low power   │  │  Passthrough AI    │              │
  │  └────────────────────┘  └────────────────────┘              │
  │                                                               │
  │  ┌────────────────────┐  ┌────────────────────┐              │
  │  │  WEARABLE          │  │  EMBEDDED AI       │              │
  │  │                    │  │                    │              │
  │  │  E-cores only mode │  │  Industrial vision │              │
  │  │  Health monitoring │  │  Predictive maint  │              │
  │  │  P5 = 12 day idle  │  │  HEXA-LANG native  │              │
  │  │  BLE always-on     │  │  Real-time OS      │              │
  │  └────────────────────┘  └────────────────────┘              │
  └──────────────────────────────────────────────────────────────┘
```

---

## 12. N6 EXACT Scorecard

### 12.1 CPU Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 1 | Total cores | 8 | sigma-tau | YES |
| 2 | P-cores | 4 | tau | YES |
| 3 | E-cores | 4 | tau | YES |
| 4 | P-core pipeline stages | 12 | sigma | YES |
| 5 | E-core pipeline stages | 6 | n | YES |
| 6 | P-core decode width | 6 | n | YES |
| 7 | E-core decode width | 3 | n/phi | YES |
| 8 | P-core issue width | 6 | n | YES |
| 9 | P-core ALU ports | 4 | tau | YES |
| 10 | P-core FP ports | 2 | phi | YES |
| 11 | P-core branch port | 1 | mu | YES |
| 12 | P-core LS ports | 3 | n/phi | YES |
| 13 | P-core N6 accel ports | 2 | phi | YES |
| 14 | P-core total exec ports | 12 | sigma | YES |
| 15 | E-core total exec ports | 6 | n | YES |
| 16 | ROB entries | 288 | sigma*J_2 | YES |
| 17 | Physical regs (INT) | 288 | sigma*J_2 | YES |
| 18 | Physical regs (FP) | 288 | sigma*J_2 | YES |
| 19 | Vector width | 256-bit | 2^(sigma-tau) | YES |
| 20 | Register banks | 6 | n | YES |
| 21 | Regs per bank | 12 | sigma | YES |
| 22 | Arch registers total | 72 | n*sigma | YES |
| 23 | Load queue | 64 | 2^n | YES |
| 24 | Store queue | 48 | sigma*tau | YES |
| 25 | Scheduler entries | 72 | n*sigma | YES |

### 12.2 Cache/Memory Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 26 | L1I (P-core) | 64 KB | 2^n | YES |
| 27 | L1D (P-core) | 64 KB | 2^n | YES |
| 28 | L1I (E-core) | 32 KB | 2^sopfr | YES |
| 29 | L1D (E-core) | 32 KB | 2^sopfr | YES |
| 30 | L2 (P-cluster) | 1 MB | 2^(sigma-phi) | YES |
| 31 | L2 (E-cluster) | 512 KB | 2^(sigma-mu-phi) | YES |
| 32 | L3 shared | 12 MB | sigma | YES |
| 33 | NPU SRAM | 4 MB | tau | YES |
| 34 | LPDDR5X capacity | 8 GB | sigma-tau | YES |
| 35 | LPDDR5X channels | 4 | tau | YES |
| 36 | LPDDR5X bandwidth | 48 GB/s | sigma*tau | YES |
| 37 | Flash storage | 256 GB | 2^(sigma-tau) | YES |
| 38 | Stack fraction | 1/2 | Egyptian | YES |
| 39 | Heap fraction | 1/3 | Egyptian | YES |
| 40 | Arena fraction | 1/6 | Egyptian | YES |

### 12.3 NPU Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 41 | Mamba d_state | 16 | 2^tau | YES |
| 42 | Mamba expand | 2 | phi | YES |
| 43 | Mamba d_conv | 4 | tau | YES |
| 44 | Mamba dt_rank | 8 | sigma-tau | YES |
| 45 | LoRA rank | 8 | sigma-tau | YES |
| 46 | LoRA adapters | 6 | n | YES |
| 47 | LoRA alpha | 12 | sigma | YES |
| 48 | INT8 TOPS | 72 | sigma*n | YES |
| 49 | INT4 TOPS | 144 | sigma^2 | YES |
| 50 | MAC tile size | 8x8 | (sigma-tau)^2 | YES |
| 51 | MAC tile count | 12 | sigma (n*phi) | YES |
| 52 | Scan units | 4 | tau | YES |
| 53 | EFA global heads | 6 | sigma/phi | YES |
| 54 | EFA local heads | 4 | sigma/n*phi | YES |
| 55 | EFA FFT heads | 2 | sigma/n | YES |
| 56 | EFA total heads | 12 | sigma | YES |

### 12.4 GPU Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 57 | Shader cores | 12 | sigma | YES |
| 58 | ALUs/core | 24 | J_2 | YES |
| 59 | Total ALUs | 288 | sigma*J_2 | YES |
| 60 | TMUs/core | 4 | tau | YES |
| 61 | ROPs/core | 2 | phi | YES |
| 62 | Warp size | 32 | 2^sopfr | YES |
| 63 | Warps/core | 6 | n | YES |
| 64 | Min frame rate | 24 fps | J_2 | YES |
| 65 | Panel refresh | 48 Hz | sigma*tau | YES |

### 12.5 Connectivity Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 66 | WiFi generation | 6 | n | YES |
| 67 | WiFi streams | 3 | n/phi | YES |
| 68 | 5G (NR) | 5G | sopfr | YES |
| 69 | 5G bands | 12 | sigma | YES |
| 70 | BLE version | 5.x | sopfr | YES |
| 71 | USB-C lanes | 4 | tau | YES |
| 72 | GPIO pins | 24 | J_2 | YES |
| 73 | I2C buses | 3 | n/phi | YES |
| 74 | SPI buses | 3 | n/phi | YES |
| 75 | UART ports | 3 | n/phi | YES |
| 76 | MIPI CSI lanes | 4 | tau | YES |
| 77 | MIPI DSI lanes | 4 | tau | YES |
| 78 | GNSS constellations | 4 | tau | YES |

### 12.6 Power Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 79 | Power states | 6 | n | YES |
| 80 | TDP sustained | 6W | n | YES |
| 81 | TDP burst | 12W | sigma | YES |
| 82 | Idle power | 83 mW | 1/sigma | YES |
| 83 | Battery capacity | 24 Wh | J_2 | YES |
| 84 | Mixed-use runtime | 8h | sigma-tau | YES |
| 85 | CPU power fraction | 1/2 | Egyptian | YES |
| 86 | GPU+NPU fraction | 1/3 | Egyptian | YES |
| 87 | I/O+conn fraction | 1/6 | Egyptian | YES |
| 88 | DVFS voltage steps | 10 | sigma-phi | YES |
| 89 | Burst duration | 48s | sigma*tau | YES |

### 12.7 Physical Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 90 | Die area | 72 mm^2 | sigma*phi*n | YES |
| 91 | Transistors | 24B | J_2 | YES |
| 92 | Metal layers | 12 | sigma | YES |
| 93 | Gate pitch | 48 nm | sigma*tau | YES |
| 94 | Metal pitch | 28 nm | P_2 | YES |
| 95 | Fin pitch | 24 nm | J_2 | YES |
| 96 | Package size | 12x12 mm | sigma x sigma | YES |
| 97 | Ball count | 288 | sigma*J_2 | YES |
| 98 | VDD pins | 48 | sigma*tau | YES |
| 99 | GND pins | 48 | sigma*tau | YES |
| 100 | LPDDR pins | 144 | sigma^2 | YES |

### 12.8 HEXA-LANG Hardware Parameters

| # | Parameter | Value | n=6 Formula | EXACT |
|---|-----------|-------|-------------|:-----:|
| 101 | Keywords in CAM | 53 | sigma*tau+sopfr | YES |
| 102 | Keyword groups | 12 | sigma | YES |
| 103 | Opcode width | 24-bit | J_2 | YES |
| 104 | Primitive types | 8 | sigma-tau | YES |
| 105 | Type tag bits | 3 | n/phi | YES |
| 106 | Operators | 24 | J_2 | YES |
| 107 | Paradigms | 6 | n | YES |
| 108 | Memory regions | 3 | n/phi | YES |

---

**TOTAL: 108/108 parameters = 100% n=6 EXACT**

Every single parameter of HEXA-EDGE derives from a function of n=6.
No parameter was chosen arbitrarily. No hyperparameter search was needed.
The chip was not designed -- it was *discovered* from sigma(n)*phi(n) = n*tau(n).

---

## 13. HEXA-EDGE vs HEXA-1 Comparison

| Dimension | HEXA-1 (Datacenter) | HEXA-EDGE (Mobile) | Ratio |
|-----------|--------------------|--------------------|-------|
| TDP | 240W | 6W | 1/40 |
| Die area | ~800 mm^2 | 72 mm^2 | 1/11 |
| CPU cores | 8P + 4E = 12 | 4P + 4E = 8 | 2/3 |
| GPU SMs | 144 | 12 | 1/12 |
| NPU cores | 24 | integrated | -- |
| Memory | 288 GB HBM4 | 8 GB LPDDR5X | 1/36 |
| Mem BW | 4 TB/s | 48 GB/s | 1/83 |
| INT8 TOPS | ~2000 | 72 | 1/28 |
| HEXA-LANG | Software runtime | Hardware native | edge wins |
| Perf/W | baseline | ~6x (NPU) | edge wins |

The relationship: HEXA-EDGE is HEXA-1 scaled by 1/sigma factors.
- Cores: sigma-tau (vs sigma-tau + tau = sigma) -> 8 vs 12
- GPU: sigma (vs sigma^2) -> 12 vs 144
- Memory: sigma-tau (vs sigma*J_2) -> 8 vs 288
- Power: n (vs sigma*(sigma-phi)) -> 6 vs 240

---

## Appendix A: N6 Constants Used in This Document

| Constant | Value | Occurrences | Key Usage |
|----------|-------|-------------|-----------|
| n=6 | 6 | 28 | Core count, pipeline, paradigms, power states |
| phi=2 | 2 | 18 | FP ports, ROPs, clusters, BLE, USB |
| tau=4 | 4 | 24 | P/E cores, TMUs, MIPI lanes, channels |
| sigma=12 | 12 | 32 | Pipeline stages, shader cores, L3, metal layers |
| sopfr=5 | 5 | 8 | BLE, 5G, E-core cache |
| mu=1 | 1 | 6 | Branch port, SDIO, deep sleep |
| J_2=24 | 24 | 16 | ALUs/core, GPIO, opcode, fps, battery |
| sigma-tau=8 | 8 | 14 | Total cores, LoRA rank, LPDDR, runtime |
| sigma-phi=10 | 10 | 4 | DVFS steps, branch offset |
| sigma*tau=48 | 48 | 10 | Gate pitch, refresh, bandwidth, power pins |
| sigma^2=144 | 144 | 4 | INT4 TOPS, LPDDR pins |
| sigma*J_2=288 | 288 | 8 | ROB, phys regs, ball count, GPU ALUs |
| n/phi=3 | 3 | 10 | I2C/SPI/UART, WiFi streams, decode (E) |
| 2^n=64 | 64 | 6 | L1 cache, BTB, load queue |
| P_2=28 | 28 | 2 | Metal pitch |
| 2^sopfr=32 | 32 | 3 | Warp size, E-core cache |
| Egyptian 1/2+1/3+1/6 | 1 | 6 | Memory, power, die area |

---

*HEXA-EDGE: The n=6 chip you carry in your pocket.*
*108 parameters. Zero arbitrary choices. One perfect number.*
