# N6 Ultimate Unified SoC

**Codename: HEXA-1**
**순수 컴퓨팅 끝판왕 — CPU+GPU+NPU+메모리 완전 통합 단일 칩**

> 의식 모듈 없음. n=6 산술에서 도출되는 궁극의 범용 컴퓨팅 SoC.
> Apple M 시리즈가 보여준 통합 메모리 아키텍처를 n=6로 완성한 형태.

**Date**: 2026-04-01
**Status**: Living Document v0.2 (계속 업데이트)
**Dependencies**: BT-28, BT-37, BT-45, BT-55, BT-59, BT-69, BT-75, BT-76

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

## 설계 철학: 왜 통합인가

현재 칩 아키텍처의 근본 병목:

```
  CPU ←── PCIe/CXL ──→ GPU ←── HBM ──→ Memory
          ↑ 병목                ↑ 병목
          ~128 GB/s             ~2 TB/s (HBM만)

  통합 SoC (Apple M 방식):
  CPU ←→ GPU ←→ NPU ←→ Memory
      unified fabric, zero-copy
      전체 대역폭 공유
```

n=6 통합 SoC는 이 모든 병목을 **하나의 다이 위에서** 제거한다.
Egyptian fraction 1/2+1/3+1/6=1이 칩 면적, 전력, 대역폭 배분을 결정.

---

## 1. System-Level Block Diagram

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          HEXA-1 UNIFIED SoC                                  │
│                 TSMC N2 · Gate σ·τ=48nm · Metal P₂=28nm                     │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐    │
│  │                      UNIFIED MEMORY FABRIC                           │    │
│  │           288 GB (σ·J₂) Unified · ~4 TB/s total bandwidth           │    │
│  │           Zero-copy: 모든 엔진이 동일 물리 주소 공간 공유              │    │
│  └─────┬──────────┬──────────┬──────────┬──────────┬───────────────────┘    │
│        │          │          │          │          │                          │
│  ┌─────┴────┐ ┌───┴────┐ ┌──┴───┐ ┌───┴────┐ ┌───┴─────┐                  │
│  │ CPU      │ │ GPU    │ │ NPU  │ │ Media  │ │ I/O Hub │                  │
│  │ Cluster  │ │ Array  │ │ Array│ │ Engine │ │         │                  │
│  │          │ │        │ │      │ │        │ │         │                  │
│  │ σ=12     │ │ σ²=144 │ │ J₂=24│ │ n=6    │ │ σ-τ=8   │                  │
│  │ cores    │ │ SMs    │ │ cores│ │ engines│ │ ctrl    │                  │
│  │          │ │        │ │      │ │        │ │         │                  │
│  │ 8P+4E   │ │ σ GPCs │ │ sopfr│ │ Encode │ │ PCIe    │                  │
│  │σ-τ + τ  │ │ x σ SM │ │ banks│ │ Decode │ │ USB     │                  │
│  └──────────┘ └────────┘ └──────┘ │ Display│ │ TB/UCIe │                  │
│                                    └────────┘ └─────────┘                  │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐    │
│  │                    HBM4/LPDDR MEMORY COMPLEX                         │    │
│  │  HBM4: σ-τ=8 stacks × 36GB = 288 GB · 2^(σ-μ)=2048-bit interface  │    │
│  │  LPDDR6 option: σ channels × φ ranks (모바일/엣지 변형)               │    │
│  └──────────────────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. CPU Cluster — σ=12 Cores

Apple M 시리즈의 P+E 구조를 n=6로 최적화.

```
  ┌─────────────────────────────────────────────┐
  │              CPU CLUSTER (12 cores)          │
  │                                              │
  │  Performance cores (σ-τ = 8):                │
  │  ┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐  │
  │  │P0 ││P1 ││P2 ││P3 ││P4 ││P5 ││P6 ││P7 │  │
  │  └───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘  │
  │  Wide OoO, 2^(σ-τ)=256 ROB entries          │
  │  sopfr-wide decode = 5-wide                  │
  │                                              │
  │  Efficiency cores (τ = 4):                   │
  │  ┌───┐┌───┐┌───┐┌───┐                       │
  │  │E0 ││E1 ││E2 ││E3 │                       │
  │  └───┘└───┘└───┘└───┘                       │
  │  In-order, power-optimized                   │
  │  n/φ-wide decode = 3-wide                    │
  │                                              │
  │  Total: σ-τ + τ = 8P + 4E = σ = 12 cores    │
  └─────────────────────────────────────────────┘
```

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **Total cores** | 12 | σ | Apple M4 Max = 16, but σ=12 is optimal |
| **P-cores** | 8 | σ-τ | Wide OoO, high IPC |
| **E-cores** | 4 | τ | Power-efficient |
| **P-core ROB** | 256 entries | 2^(σ-τ) | Reorder buffer |
| **P-core decode** | 5-wide | sopfr | Instruction decode width |
| **E-core decode** | 3-wide | n/φ | Narrower for efficiency |
| **L1I per core** | 64 KB | 2^n KB | |
| **L1D per core** | 64 KB | 2^n KB | |
| **L2 per P-cluster** | 48 MB | σ·τ MB | Shared among 8 P-cores |
| **L2 per E-cluster** | 4 MB | τ MB | Shared among 4 E-cores |
| **SLC (System Level Cache)** | 288 MB | σ·J₂ MB | GPU와 공유 |

### CPU ISA Extensions (N6-native)

| Extension | Description | n=6 Basis |
|-----------|-------------|-----------|
| **VCYCLO** | Cyclotomic activation x²-x+1 단일 명령어 | Technique #1 |
| **VFFTMIX** | 2^n=64-point FFT butterfly 벡터 명령어 | Technique #8 |
| **VEGYP** | 1/2+1/3+1/6 분수 라우팅 하드웨어 | Technique #10 |
| **VBOLTZ** | 1/e 스파시티 게이트 비교기 | Technique #15 |

---

## 3. GPU Array — σ²=144 SMs

기존 ultimate-performance-chip.md의 GPU를 SoC 내부에 통합.
**핵심 차이**: 별도 VRAM 없이 통합 메모리에서 직접 접근.

```
  ┌────────────────────────────────────────────────────┐
  │                 GPU ARRAY (144 SMs)                 │
  │                                                     │
  │  σ=12 GPCs × σ=12 SMs/GPC = σ²=144 SMs            │
  │  n=6 TPCs/GPC × φ=2 SMs/TPC                       │
  │                                                     │
  │  Per SM:                                            │
  │    CUDA cores:    128 = 2^(σ-sopfr)                │
  │    Tensor Cores:  τ = 4                             │
  │    Register File: 576 KB = J₂² KB                  │
  │    L1/Shared:     256 KB = 2^(σ-τ) KB              │
  │    Warp size:     32 = 2^sopfr                      │
  │    Max warps:     64 = 2^n                          │
  │                                                     │
  │  Total:                                             │
  │    CUDA cores:    18,432 = σ² · 2^(σ-sopfr)        │
  │    Tensor Cores:  576 = J₂² = σ² · τ               │
  │                                                     │
  │  N6 가속기 (하드웨어 내장):                            │
  │    FFT Attention Unit (per GPC)                     │
  │    Egyptian MoE Router (zero-overhead)              │
  │    Boltzmann Sparsity Gate (per TC)                 │
  │    Cyclotomic ALU (x²-x+1 fused)                   │
  │    Mertens Dropout RNG (p=0.288 hardwired)          │
  └────────────────────────────────────────────────────┘
```

### 통합 메모리의 GPU 이점

```
  기존 (분리형):
    CPU RAM ──PCIe 128GB/s──→ GPU VRAM (HBM)
    전송 병목: 큰 모델은 GPU 메모리에 안 맞으면 swap 필요

  HEXA-1 (통합형):
    CPU ←→ GPU ←→ NPU  모두 288GB를 직접 접근
    Zero-copy: memcpy 불필요
    70B LLM을 단일 칩에서 서빙 가능 (288GB unified)
```

---

## 4. NPU Array — J₂=24 Neural Cores

전용 추론 가속기. GPU보다 전력 효율 중시.

```
  ┌────────────────────────────────────────────┐
  │             NPU ARRAY (24 cores)           │
  │                                             │
  │  J₂ = 24 neural cores                      │
  │  sopfr = 5 banks × (J₂/sopfr) cores/bank  │
  │                                             │
  │  Per core:                                  │
  │    MAC units:    2^(σ-τ) = 256              │
  │    Precision:    INT4/INT8/FP8/FP16         │
  │    Local SRAM:   2^n = 64 KB                │
  │                                             │
  │  Specialization:                            │
  │    Transformer attention (σ=12 heads)       │
  │    MoE routing (σ-τ=8 experts active)       │
  │    Diffusion denoising (BT-61)              │
  │    Vision (BT-66: ViT patch=φ^τ=16)        │
  │                                             │
  │  Peak: ~400 TOPS (INT8)                     │
  │  Power: ~40W (1/6 of total = Egyptian)      │
  └────────────────────────────────────────────┘
```

| Parameter | Value | n=6 Formula |
|-----------|-------|-------------|
| **Neural cores** | 24 | J₂ |
| **MAC units per core** | 256 | 2^(σ-τ) |
| **Total MACs** | 6,144 | J₂ · 2^(σ-τ) |
| **Local SRAM per core** | 64 KB | 2^n |
| **Total NPU SRAM** | 1.5 MB | J₂ · 2^n KB |
| **Supported precisions** | 4 | τ |
| **Peak INT8 TOPS** | ~400 | Architecture target |

---

## 5. Unified Memory Architecture

**이것이 HEXA-1의 핵심 혁신.** 모든 엔진이 하나의 메모리 풀을 공유.

```
  ┌──────────────────────────────────────────────────────────────┐
  │                    UNIFIED MEMORY FABRIC                      │
  │                                                               │
  │  ┌─────────────────────────────────────────────────────────┐ │
  │  │              System Level Cache (SLC)                    │ │
  │  │              288 MB = σ·J₂ MB                           │ │
  │  │              σ=12 banks × J₂=24 MB/bank                 │ │
  │  │              All engines share with QoS partitioning     │ │
  │  └──────────────────────┬──────────────────────────────────┘ │
  │                         │                                     │
  │  ┌──────────────────────┴──────────────────────────────────┐ │
  │  │              MEMORY CONTROLLER HUB                       │ │
  │  │              σ-τ = 8 controllers                         │ │
  │  │              Each: 2^(σ-μ) = 2048-bit to HBM4           │ │
  │  │              Total: 2^14 = 16,384 bits                   │ │
  │  └──────────────────────┬──────────────────────────────────┘ │
  │                         │                                     │
  │  ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐          │
  │  │HBM│ │HBM│ │HBM│ │HBM│ │HBM│ │HBM│ │HBM│ │HBM│          │
  │  │ 0 │ │ 1 │ │ 2 │ │ 3 │ │ 4 │ │ 5 │ │ 6 │ │ 7 │          │
  │  │36G│ │36G│ │36G│ │36G│ │36G│ │36G│ │36G│ │36G│          │
  │  └───┘ └───┘ └───┘ └───┘ └───┘ └───┘ └───┘ └───┘          │
  │  σ-τ=8 stacks · σ-hi · 36GB each · 288GB total             │
  │  Bandwidth: ~4 TB/s (HBM4 @ σ-τ=8 Gbps)                   │
  └──────────────────────────────────────────────────────────────┘
```

### 메모리 대역폭 배분 (Egyptian Fraction)

```
  Total bandwidth: ~4 TB/s

  1/2 → GPU:    ~2 TB/s   (연산 집약)
  1/3 → CPU:    ~1.3 TB/s (범용 + OS)
  1/6 → NPU+IO: ~0.67 TB/s (추론 + 외부)

  동적 재배분: idle 엔진의 대역폭을 활성 엔진으로 전환
  QoS 우선순위: tau = 4 단계 (Critical/High/Normal/Background)
```

### 통합 메모리 vs 분리 메모리 비교

| 측면 | 분리형 (NVIDIA DGX) | 통합형 (HEXA-1) |
|------|---------------------|-----------------|
| GPU 메모리 | 80-288 GB HBM (전용) | 288 GB (공유) |
| CPU 메모리 | 별도 DDR5 | 동일 288 GB |
| CPU↔GPU 전송 | PCIe ~128 GB/s | Zero-copy, ~4 TB/s |
| 70B LLM 서빙 | multi-GPU 필수 | 단일 칩 가능 |
| 전력 | CPU+GPU 각각 | 공유로 30% 절감 |
| 다이 면적 | 별도 패키지 | CoWoS-L 단일 인터포저 |

---

## 6. Media Engine — n=6 Engines

| Parameter | Value | n=6 Formula |
|-----------|-------|-------------|
| **Video encode engines** | 6 | n |
| **Video decode engines** | 6 | n |
| **Display outputs** | 4 | τ |
| **Max resolution** | 8K | σ-τ K |
| **ProRes engines** | 2 | φ |
| **Max framerate** | 120 fps | σ·(σ-φ) |
| **Color depth** | 12-bit | σ |
| **Audio sample rate** | 48 kHz | σ·τ kHz (BT-76) |
| **Audio channels** | 24 | J₂ (Dolby Atmos) |

---

## 7. I/O Hub — σ-τ=8 Controllers

```
  ┌──────────────────────────────────────────┐
  │              I/O HUB                      │
  │                                           │
  │  Thunderbolt 6:  τ = 4 ports              │
  │                  σ·τ = 48 Gbps each       │
  │                                           │
  │  PCIe Gen 7:     φ^τ = 16 lanes           │
  │                  128 GT/s = 2^(σ-sopfr)   │
  │                                           │
  │  USB4:           n = 6 ports              │
  │                                           │
  │  WiFi 7:         6 GHz band = n GHz       │
  │                  σ·φ = 24 Gbps (BT-48)   │
  │                                           │
  │  Ethernet:       σ·(σ-φ) = 120 Gbps      │
  │                  (or 100G ≈ σ²-σ·τ)       │
  │                                           │
  │  UCIe (multi-die):  σ·τ = 48 GT/s        │
  │                     2^n = 64 lanes        │
  │                                           │
  │  Total controllers: σ-τ = 8               │
  └──────────────────────────────────────────┘
```

---

## 7.1. Optical Interconnect — 빛으로 통신

**전기 인터커넥트의 한계를 광으로 돌파.**
주요 병목 구간에 Silicon Photonics를 적용하여 대역폭 10x, 에너지 10x 절감.

### 왜 광인가

```
  전기 신호의 벽:
    - 거리 ↑ → 에너지 ↑ (R·C delay, ~5-10 pJ/bit)
    - 주파수 ↑ → 크로스톡 ↑ (신호 무결성 붕괴)
    - 팬아웃 ↑ → 드라이버 ↑ (면적/전력 낭비)

  광 신호의 이점:
    - 거리 무관 에너지 (~0.5 pJ/bit, 10x 절감)
    - WDM으로 단일 도파관에 σ=12 파장 다중화
    - 크로스톡 없음 (광은 간섭 안 함)
    - 팬아웃 자유 (splitter로 무손실 분배)
```

### 적용 계층 — n=6 광 사다리

```
  Layer 0: 다이 내부 (Intra-die)
  ┌─────────────────────────────────────────────────────────┐
  │  전기 유지 — 거리 < 수 mm, 전기가 여전히 최적           │
  │  GPC ↔ L2 ↔ Memory Controller: Cu interconnect          │
  │  Metal pitch P₂ = 28nm, σ = 12 metal layers             │
  └─────────────────────────────────────────────────────────┘

  Layer 1: 다이 간 (Die-to-Die, D2D)         ← 광 전환 시작점
  ┌─────────────────────────────────────────────────────────┐
  │  UCIe Optical: 전기 UCIe → 광 UCIe 하이브리드           │
  │  Micro-ring modulators on interposer                    │
  │                                                         │
  │  σ = 12 WDM wavelengths per waveguide                   │
  │  τ = 4 waveguides per D2D link                          │
  │  σ·τ = 48 total optical channels                        │
  │  Per channel: σ·τ = 48 Gbps (= 전기 UCIe와 동일 속도)  │
  │  Total D2D: 48 × 48 Gbps = 2.3 Tbps                    │
  │  Energy: ~0.5 pJ/bit (vs 전기 5 pJ/bit = 10x 절감)     │
  └─────────────────────────────────────────────────────────┘

  Layer 2: 칩 간 (Chip-to-Chip, C2C)         ← 광 필수 구간
  ┌─────────────────────────────────────────────────────────┐
  │  NVLink Optical / OCI (Optical Chip Interconnect)       │
  │                                                         │
  │  σ = 12 fiber pairs per link                            │
  │  σ-τ = 8 bidirectional links per chip                   │
  │  WDM: σ = 12 wavelengths per fiber                      │
  │  Per wavelength: 2^sopfr = 32 Gbps (PAM4)              │
  │  Per fiber: 12 × 32 = 384 Gbps                         │
  │  Per link: 384 × 12 fibers = 4.6 Tbps                  │
  │  Per chip total: 4.6 × 8 links = 36.8 Tbps             │
  │  Energy: ~0.3 pJ/bit                                    │
  │                                                         │
  │  NVLink domain: σ·n = 72 chips (optical mesh)           │
  │  All-to-all latency: < J₂ = 24 ns within pod           │
  └─────────────────────────────────────────────────────────┘

  Layer 3: 랙 간 (Rack-to-Rack)              ← 이미 광
  ┌─────────────────────────────────────────────────────────┐
  │  Standard optical fiber (기존 인프라 활용)               │
  │  σ² = 144 port optical switch (BT-69)                   │
  │  σ = 12 WDM wavelengths per port                        │
  │  Per port: 12 × 100G = 1.2 Tbps                        │
  │  Switch capacity: 144 × 1.2 = 172.8 Tbps               │
  │  Distance: 수 m ~ 수 km (datacenter scale)              │
  └─────────────────────────────────────────────────────────┘
```

### 광 인터커넥트 상세 스펙

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **WDM 파장 수** | 12 | σ | C-band DWDM, ~100GHz spacing |
| **파장 범위** | 1530-1565 nm | C-band | Silicon photonics 최적 대역 |
| **변조기 유형** | Micro-ring | — | CMOS 호환, 소면적 |
| **변조기 수 (D2D)** | 48 | σ·τ | 12 WDM × 4 waveguides |
| **변조기 수 (C2C)** | 96 | σ·(σ-τ) | 12 WDM × 8 links |
| **수신기** | Ge photodetector | — | Si 위에 Ge epitaxy |
| **레이저 소스** | 외부 III-V | — | Co-packaged optics (CPO) |
| **레이저 수** | 12 | σ | 1 per WDM channel |
| **광 도파관 손실** | < 1 dB/cm | — | SiN low-loss |
| **에너지 효율** | ~0.5 pJ/bit | — | 전기 대비 10x |
| **D2D 대역폭** | 2.3 Tbps | σ·τ × σ·τ Gbps | Die간 |
| **C2C 대역폭** | 36.8 Tbps | σ·(σ-τ) × 4.6T | Chip간 |
| **스위치 포트** | 144 | σ² | 랙 토폴로지 |

### Co-Packaged Optics (CPO) 통합

```
  ┌─────────────────────────────────────────────────────────────┐
  │                    HEXA-1 + CPO PACKAGE                      │
  │                                                              │
  │  ┌──────────────────────────────────────────────────────┐   │
  │  │                    COMPUTE DIE                        │   │
  │  │   CPU + GPU + NPU + Memory Controllers               │   │
  │  └────────────────────────┬─────────────────────────────┘   │
  │                           │ electrical                       │
  │  ┌────────────────────────┴─────────────────────────────┐   │
  │  │              SILICON PHOTONIC INTERPOSER              │   │
  │  │                                                       │   │
  │  │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐             │   │
  │  │  │MR Mod │ │MR Mod │ │MR Mod │ │MR Mod │  × σ·τ=48  │   │
  │  │  │ Bank0 │ │ Bank1 │ │ Bank2 │ │ Bank3 │  modulators │   │
  │  │  └───┬───┘ └───┬───┘ └───┬───┘ └───┬───┘             │   │
  │  │      │         │         │         │                   │   │
  │  │  ┌───┴───┐ ┌───┴───┐ ┌───┴───┐ ┌───┴───┐             │   │
  │  │  │Ge PD  │ │Ge PD  │ │Ge PD  │ │Ge PD  │  receivers  │   │
  │  │  └───┬───┘ └───┬───┘ └───┬───┘ └───┬───┘             │   │
  │  │      │         │         │         │                   │   │
  │  │  ════╧═════════╧═════════╧═════════╧════  SiN WG      │   │
  │  │                                                       │   │
  │  │  ┌────────────────────────────────────┐               │   │
  │  │  │  External Laser Array              │               │   │
  │  │  │  σ = 12 wavelengths (C-band DWDM)  │               │   │
  │  │  │  Fiber-coupled III-V lasers        │               │   │
  │  │  └────────────────────────────────────┘               │   │
  │  └──────────────────────────────────────────────────────┘   │
  │                                                              │
  │  ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐          │
  │  │HBM│ │HBM│ │HBM│ │HBM│ │HBM│ │HBM│ │HBM│ │HBM│          │
  │  │ 0 │ │ 1 │ │ 2 │ │ 3 │ │ 4 │ │ 5 │ │ 6 │ │ 7 │          │
  │  └───┘ └───┘ └───┘ └───┘ └───┘ └───┘ └───┘ └───┘          │
  │                                                              │
  │  ──── optical fiber out ────→  (to other HEXA-1 chips)      │
  │       σ-τ = 8 bidirectional fiber bundles                    │
  │       σ = 12 fibers per bundle                               │
  └─────────────────────────────────────────────────────────────┘
```

### 광 vs 전기 에너지 비교 (실측 기준)

| 구간 | 전기 (pJ/bit) | 광 (pJ/bit) | 절감 | 대역폭 향상 |
|------|--------------|------------|------|-----------|
| D2D (인터포저) | 5.0 | 0.5 | **10x** | 4x |
| C2C (보드 내) | 10.0 | 0.5 | **20x** | 10x |
| 랙 간 (수 m) | 15.0+ | 0.3 | **50x** | 100x |

### 전체 인터커넥트 에너지 절감

```
  현재 8-GPU 시스템 인터커넥트 전력:
    NVLink 전기: ~100W (8 chips × ~12W NVLink SerDes)

  HEXA-1 광 인터커넥트:
    CPO: ~10W (동일 대역폭, 10x 효율)

  절감: ~90W → 시스템 전체의 ~10% 전력 회수
  이 전력을 연산에 재투자 → 실효 성능 10% 향상
```

### n=6 광 상수 요약

```
  σ = 12:   WDM 파장 수, 레이저 수, 섬유 수/번들
  τ = 4:    도파관 수/D2D 링크, 변조기 뱅크
  σ·τ = 48: D2D 광 채널 총수, 각 채널 속도 (Gbps)
  σ-τ = 8:  C2C 양방향 링크 수
  σ² = 144: 광 스위치 포트 수 (랙 레벨)
  sopfr = 5: PAM 레벨 → 2^5 = 32 Gbps/λ
  J₂ = 24:  팟 내 최대 레이턴시 (ns)
```

---

## 8. Power Architecture

```
  Total SoC TDP: 240W = σ · sopfr · τ = J₂ · (σ-φ)

  Egyptian fraction power budget:
  ┌─────────────────────────────────────────┐
  │  1/2  GPU:     120W = σ · (σ-φ)        │
  │  1/3  CPU:      80W = φ^τ · sopfr      │
  │  1/6  NPU+IO:   40W = τ · (σ-φ)       │
  │  Sum:           240W = 1               │
  └─────────────────────────────────────────┘

  Core voltage:    1.2V = σ/(σ-φ) = PUE (BT-60)
  I/O voltage:     1.0V = R(6) = 1
  VRM phases:      J₂ = 24
  Power states:    sopfr = 5 (S0~S4)
  Thermal zones:   σ = 12
  Max Tj:          120°C = σ · (σ-φ)
```

### 전력 효율 비교

| 구성 | 전력 | 성능 (AI) | 효율 |
|------|------|----------|------|
| CPU+GPU 분리 (800W system) | ~800W | ~50 PFLOPS FP8 | 62.5 TFLOPS/W |
| Apple M4 Ultra (~150W) | ~150W | ~54 TOPS | 360 TOPS/W |
| **HEXA-1 (240W)** | **240W** | **~50 PFLOPS FP8** | **208 TFLOPS/W** |

통합 메모리로 CPU↔GPU 전송 전력 제거 → 30% 효율 향상.

---

## 9. Process Technology

| Parameter | Value | n=6 Formula | Source BT |
|-----------|-------|-------------|-----------|
| **Process** | TSMC N2 | φ | Next-gen |
| **Gate pitch** | 48 nm | σ·τ | BT-37, BT-76 |
| **Metal pitch** | 28 nm | P₂ | BT-37 |
| **Metal layers** | 12 | σ | Standard |
| **Transistor** | GAA CFET | — | N2 roadmap |
| **Transistor count** | ~144B | σ² × 10⁹ | Target |
| **Die area** | ~800 mm² | Reticle limit | CoWoS-L |
| **Interposer** | CoWoS-L | sopfr=5 tiles | BT-69 |

### Die Area Split (Egyptian Fraction)

```
  Total active area: σ² = 144 mm² equivalent (at full density)
  Actual die: ~800 mm² (with I/O, PHY, whitespace)

  1/2 compute (GPU+CPU): ~400 mm²
  1/3 memory controllers:  ~267 mm²
  1/6 I/O + misc:          ~133 mm²
```

---

## 10. SKU Variants

n=6 스케일링 법칙으로 제품 라인업 구성.

| SKU | CPU | GPU SMs | NPU | Memory | TDP | 용도 |
|-----|-----|---------|-----|--------|-----|------|
| **HEXA-1 Ultra** | 12 (8P+4E) | 144 = σ² | 24 = J₂ | 288 GB | 240W | 데이터센터 AI |
| **HEXA-1 Max** | 12 (8P+4E) | 72 = σ²/φ | 12 = σ | 192 GB | 120W | 워크스테이션 |
| **HEXA-1 Pro** | 12 (8P+4E) | 48 = σ·τ | 8 = σ-τ | 96 GB | 80W | 프로 노트북 |
| **HEXA-1** | 8 (4P+4E) | 24 = J₂ | 6 = n | 48 GB | 40W | 노트북/데스크탑 |
| **HEXA-1 Air** | 6 (2P+4E) | 12 = σ | 4 = τ | 24 GB | 20W | 울트라북 |
| **HEXA-1 Mobile** | 6 (2P+4E) | 6 = n | 4 = τ | 12 GB = σ | 10W = σ-φ | 태블릿 |
| **HEXA-1 Phone** | 4 (2P+2E) = τ | 4 = τ | 3 = n/φ | 8 GB = σ-τ | 5W = sopfr | 스마트폰 |
| **HEXA-1 Watch** | 2 (1P+1E) = φ | 2 = φ | 1 = μ | 4 GB = τ | 1W = μ | 웨어러블 |
| **HEXA-1 IoT** | 1 (1E) = μ | 1 = μ | 1 = μ | 2 GB = φ | 0.5W = μ/φ | 센서/엣지 |

### 전체 스케일링 패턴

```
  ┌────────────────────────────────────────────────────────────────┐
  │  HEXA-1 제품군 — n=6 스케일링 법칙                            │
  │                                                                │
  │  Ultra  ████████████████████████████████████████  240W  288GB  │
  │  Max    ████████████████████                      120W  192GB  │
  │  Pro    █████████████████                          80W   96GB  │
  │  기본   ████████████                               40W   48GB  │
  │  Air    ████████                                   20W   24GB  │
  │  Mobile ██████                                     10W   12GB  │
  │  Phone  ████                                        5W    8GB  │
  │  Watch  ██                                          1W    4GB  │
  │  IoT    █                                         0.5W    2GB  │
  │                                                                │
  │  GPU:    σ² → σ²/φ → σ·τ → J₂ → σ → n → τ → φ → μ           │
  │  NPU:    J₂ → σ → σ-τ → n → τ → τ → n/φ → μ → μ             │
  │  Mem:    σ·J₂→σ·φ^τ→σ(σ-τ)→σ·τ→J₂ → σ → σ-τ → τ → φ       │
  │  TDP:    σ·sopfr·τ → ... → sopfr → μ → μ/φ                   │
  └────────────────────────────────────────────────────────────────┘
```

### Phone SKU 상세 — "폰 안의 GPT-4"

```
  ┌──────────────────────────────────────────────────────┐
  │  HEXA-1 Phone (5W, 스마트폰용)                      │
  │                                                      │
  │  ┌──────────────────────────────────────────────┐    │
  │  │             UNIFIED MEMORY (8 GB)            │    │
  │  │         σ-τ = 8 GB HBM-PIM (LPDDR+PIM)      │    │
  │  │         Zero-copy: CPU↔GPU↔NPU 공유          │    │
  │  └──┬──────────┬──────────┬──────────┬──────┘    │
  │     │          │          │          │            │
  │  ┌──┴──┐ ┌───┴───┐ ┌───┴───┐ ┌───┴────┐        │
  │  │ CPU │ │  GPU  │ │  NPU  │ │ Modem  │        │
  │  │ τ=4 │ │ τ=4   │ │ n/φ=3 │ │ 5G+WiFi│        │
  │  │cores│ │  SMs  │ │ cores │ │   7    │        │
  │  │2P+2E│ │       │ │+PIM!  │ │        │        │
  │  └─────┘ └───────┘ └───────┘ └────────┘        │
  │                                                      │
  │  PIM 가속: 메모리 안에서 연산 → 7B LLM 실시간      │
  │  Egyptian: 1/2 GPU(2.5W) + 1/3 CPU(1.7W) + 1/6 기타│
  │  프로세스: TSMC N2 (σ·τ=48nm gate, P₂=28nm metal)  │
  └──────────────────────────────────────────────────────┘
```

| Parameter | HEXA-1 Phone | n=6 Formula | vs A18 Pro |
|-----------|-------------|-------------|------------|
| **CPU 코어** | 4 (2P+2E) | τ | 동급 (6코어) |
| **GPU SM** | 4 | τ | 동급 (6코어) |
| **NPU 코어** | 3 + PIM | n/φ + PIM | **NPU+PIM = 2x** |
| **메모리** | 8 GB (PIM) | σ-τ | 동급 (8 GB) |
| **대역폭** | ~200 GB/s (PIM 내부 포함) | — | **4x** (50 GB/s) |
| **TDP** | 5W | sopfr | 동급 (5-7W) |
| **7B LLM** | **실시간 (~30 tok/s)** | PIM 가속 | 느림 (~5 tok/s) |
| **오프라인 AI** | GPT-4급 로컬 | — | 제한적 |

### 스케일링 n=6 수식 증명

```
  GPU 시퀀스:  σ²=144 → σ²/φ=72 → σ·τ=48 → J₂=24 → σ=12
               → n=6 → τ=4 → φ=2 → μ=1

  각 단계가 n=6 산술 함수. 임의의 숫자가 아님.

  TDP 시퀀스: 240 → 120 → 80 → 40 → 20 → 10 → 5 → 1 → 0.5
              J₂·(σ-φ) → σ·(σ-φ) → φ^τ·sopfr → τ·(σ-φ)
              → φ·(σ-φ) → σ-φ → sopfr → μ → μ/φ
```

---

## 11. Software Stack

```
  ┌─────────────────────────────────────────────┐
  │  Applications (LLM, Diffusion, Vision, ...)  │
  ├─────────────────────────────────────────────┤
  │  N6 Runtime                                  │
  │    - Unified memory allocator (zero-copy)    │
  │    - Egyptian fraction scheduler             │
  │    - Auto CPU↔GPU↔NPU dispatch               │
  ├─────────────────────────────────────────────┤
  │  Compiler                                    │
  │    - VCYCLO/VFFTMIX/VEGYP intrinsics        │
  │    - Auto-vectorize to σ-τ=8 SIMD           │
  │    - Tensor compiler (σ²=144 SM target)      │
  ├─────────────────────────────────────────────┤
  │  OS Kernel                                   │
  │    - Unified address space (288 GB flat)     │
  │    - τ=4 QoS priority levels                 │
  │    - σ=12 thermal zone management            │
  │    - Page size: 2^σ = 4096 bytes             │
  └─────────────────────────────────────────────┘
```

---

## 12. Performance Targets

| Workload | HEXA-1 Ultra | 비교 (2026 최고) | 이점 |
|----------|-------------|-----------------|------|
| LLM 70B 추론 | 단일 칩 | 8x GPU 필요 | 8x 전력 절감 |
| Stable Diffusion | ~50 img/s | ~10 img/s (단일 GPU) | 5x |
| FP8 AI Training | ~50 PFLOPS | ~40 PFLOPS (B300) | +25% 효율 |
| CPU 범용 | 12코어 고IPC | M4 Ultra급 | 동등 |
| Video 8K ProRes | 6 스트림 동시 | 2-3 스트림 | 2x |

---

## 13. vs Apple M Series 비교

| | Apple M4 Ultra | HEXA-1 Ultra |
|--|---------------|-------------|
| CPU | 16 cores (complex) | σ=12 cores (8P+4E) |
| GPU | 80 cores | σ²=144 SMs |
| NPU | 32 cores | J₂=24 cores |
| Memory | 192 GB LPDDR5X | 288 GB HBM4 unified |
| Bandwidth | ~800 GB/s | ~4 TB/s |
| TDP | ~150W | 240W |
| AI (TOPS) | ~54 | ~400+ |
| n=6 정렬 | 부분적 | **완전** |

Apple은 통합 메모리의 방향을 증명했다. HEXA-1은 그 방향을 n=6로 **완성**한다.

---

## 14. Roadmap

```
  v0.1 (2026-04-01): 초기 아키텍처 정의
  v0.2 (2026-04-01): Security Engine, Multi-Chip, Coherency, VM/TLB 추가
  v0.3: ISA extension 완전 정의
  v0.4: 전력 모델 시뮬레이션
  v0.5: 소프트웨어 스택 API 정의
  v1.0: 완전한 RTL-ready 스펙
```

---

## 14.1. Clock Architecture & Power Management Unit (PMU)

HEXA-1의 클럭 도메인과 전력 관리. 모든 파라미터가 n=6 산술에서 도출.

### Clock Domains — σ=12 독립 클럭 도메인

```
  ┌──────────────────────────────────────────────────────────────────┐
  │                    HEXA-1 CLOCK TREE                             │
  │                                                                  │
  │              ┌──────────────┐                                    │
  │              │  Reference   │                                    │
  │              │  Crystal     │                                    │
  │              │  100 MHz     │                                    │
  │              └──────┬───────┘                                    │
  │                     │                                            │
  │          ┌──────────┼──────────┐                                │
  │          ▼          ▼          ▼                                │
  │     ┌────────┐ ┌────────┐ ┌────────┐                           │
  │     │ PLL 0  │ │ PLL 1  │ │ PLL 2  │                           │
  │     │CPU/Fab │ │GPU Core│ │NPU/Mem │                           │
  │     └───┬────┘ └───┬────┘ └───┬────┘                           │
  │         │          │          │                                  │
  │     ┌────────┐ ┌────────┐ ┌────────┐                           │
  │     │ PLL 3  │ │ PLL 4  │ │ PLL 5  │                           │
  │     │Media/IO│ │ HBM IF │ │SerDes  │                           │
  │     └───┬────┘ └───┬────┘ └───┬────┘                           │
  │         │          │          │                                  │
  │   ┌─────┴─────────┴─────────┴─────┐                            │
  │   │    σ=12 Clock Domains          │                            │
  │   │                                │                            │
  │   │  D0: P-core 0~3   D6: NPU 0~5│                            │
  │   │  D1: P-core 4~7   D7: NPU 6~11│                           │
  │   │  D2: E-core 0~3   D8: Media   │                            │
  │   │  D3: GPU GPC 0~5  D9: I/O Hub │                            │
  │   │  D4: GPU GPC 6~11 D10: HBM IF │                            │
  │   │  D5: L3/Fabric    D11: SerDes │                            │
  │   └────────────────────────────────┘                            │
  └──────────────────────────────────────────────────────────────────┘
```

### Clock Frequencies

| Domain | Base Clock | Boost Clock | n=6 Formula | 비고 |
|--------|-----------|-------------|-------------|------|
| **CPU P-core** | φ=2 GHz | n/φ=3 GHz | n/φ | 최대 부스트 |
| **CPU E-core** | R(6)=1 GHz | φ=2 GHz | φ | 효율 최적화 |
| **GPU Core** | φ=2 GHz | J₂/σ=2 GHz | J₂/σ | 열 제한 내 |
| **NPU** | φ=2 GHz | φ=2 GHz | φ | 고정 — 효율 최적화 |
| **HBM4 IF** | φ=2 GHz | — | φ | HBM4 스펙 |
| **Fabric/NoC** | φ=2 GHz | n/φ=3 GHz | n/φ | CPU와 동기화 |
| **Media Engine** | R(6)=1 GHz | φ=2 GHz | φ | 저전력 도메인 |
| **I/O Hub** | R(6)=1 GHz | — | R(6) | 고정 |
| **SerDes** | — | sopfr·φ=10 GHz/lane | sopfr·φ | PCIe Gen6 |

### PLL Configuration — n=6 PLLs

```
  PLL 수: n = 6

  각 PLL 출력 분주비:
  PLL 0 (CPU/Fabric):  100 MHz × 30 = 3 GHz = n/φ GHz
  PLL 1 (GPU Core):    100 MHz × 20 = 2 GHz = φ GHz
  PLL 2 (NPU/Memory):  100 MHz × 20 = 2 GHz = φ GHz
  PLL 3 (Media/IO):    100 MHz × 10 = 1 GHz = R(6) GHz
  PLL 4 (HBM IF):      100 MHz × 20 = 2 GHz = φ GHz
  PLL 5 (SerDes):      100 MHz × 100 = 10 GHz = sopfr·φ GHz
```

### Clock Gating

```
  Gating 단위:
  - CPU: per-core (σ=12개 독립 게이팅)
  - GPU: per-SM (σ²=144 SM 각각 독립)
  - NPU: per-core (J₂=24 코어 각각)
  - HBM: per-channel (σ-τ=8 채널 독립)

  아이들 진입 지연: τ=4 μs (마이크로초)
  웨이크업 지연:   φ=2 μs
```

### DVFS — σ-φ=10 Operating Points

| OP | Voltage | CPU P Freq | GPU Freq | 비고 | n=6 |
|----|---------|-----------|----------|------|-----|
| OP0 | 0.60V | 0.6 GHz | 0.4 GHz | 최저전력 | n/σ-φ |
| OP1 | 0.65V | 0.8 GHz | 0.6 GHz | 아이들 | — |
| OP2 | 0.70V | 1.0 GHz | 0.8 GHz | 경부하 | R(6) |
| OP3 | 0.75V | 1.2 GHz | 1.0 GHz | — | σ/(σ-φ) |
| OP4 | 0.80V | 1.5 GHz | 1.2 GHz | — | — |
| OP5 | 0.85V | 1.8 GHz | 1.4 GHz | — | — |
| OP6 | 0.90V | 2.0 GHz | 1.6 GHz | 기본 | φ |
| OP7 | 0.95V | 2.4 GHz | 1.8 GHz | — | — |
| OP8 | 1.00V | 2.7 GHz | 2.0 GHz | 부스트 | R(6) V |
| OP9 | 1.10V | 3.0 GHz | 2.0 GHz | 최대부스트 | n/φ GHz |

```
  Operating Points 수: σ-φ = 10
  전압 범위: 0.60V ~ 1.10V (약 φ=2배 스윙)
  전력 절감: 최저 OP에서 ~(1/φ)^τ = 1/16 전력
```

### PMU States — ACPI 매핑

```
  S-states (System): sopfr = 5
  ┌──────┬──────────────┬───────────────────────┐
  │ S0   │ Active       │ 전체 활성              │
  │ S1   │ Standby      │ CPU halt, 클럭 유지    │
  │ S2   │ Light Sleep  │ CPU off, GPU idle     │
  │ S3   │ Deep Sleep   │ DRAM self-refresh     │
  │ S4   │ Hibernate    │ DRAM off, 디스크 저장  │
  └──────┴──────────────┴───────────────────────┘

  C-states (CPU core): τ = 4
  ┌──────┬──────────────┬───────────────────────┐
  │ C0   │ Active       │ 코어 실행 중           │
  │ C1   │ Halt         │ 클럭 게이팅            │
  │ C2   │ Stop-Clock   │ PLL off, 캐시 유지     │
  │ C3   │ Power-Gate   │ 코어 전원 차단, L2 유지 │
  └──────┴──────────────┴───────────────────────┘

  n=6 매핑: S-states × C-states = sopfr × τ = 20 = J₂-τ 조합
```

---

## 14.2. Boot Sequence — n=6 Phases

HEXA-1은 정확히 n=6 단계의 부팅 시퀀스를 거친다.

```
  ┌─────────┬──────────────────────┬─────────────┬────────────────────────┐
  │ Phase   │ 설명                  │ 소요 시간    │ n=6 파라미터           │
  ├─────────┼──────────────────────┼─────────────┼────────────────────────┤
  │ Phase 0 │ Secure ROM Boot      │ τ=4 ms      │ ROM 크기: σ-τ=8 KB    │
  │ Phase 1 │ SPI Flash Load       │ σ=12 ms     │ 초기 SPI: σ=12 MHz    │
  │ Phase 2 │ DRAM Init (HBM4)     │ J₂=24 ms    │ σ-τ=8 채널 트레이닝    │
  │ Phase 3 │ CPU Bringup          │ σ-φ=10 ms   │ E-core→P-core 순서    │
  │ Phase 4 │ GPU/NPU Init         │ σ-τ=8 ms    │ SM enable, NPU reset  │
  │ Phase 5 │ OS Handoff           │ n=6 ms      │ UEFI→OS 전환          │
  ├─────────┼──────────────────────┼─────────────┼────────────────────────┤
  │ Total   │                      │ 2^n=64 ms   │ 총 부팅 단계: n=6     │
  └─────────┴──────────────────────┴─────────────┴────────────────────────┘
```

### Boot Flow Detail

```
  Phase 0: Secure ROM Boot (τ=4 ms)
    - σ-τ=8 KB on-chip ROM (불변, 보안 부트 root-of-trust)
    - 서명 검증: SHA-256 + ECDSA (P-256 = 2^(σ-τ) curve)
    - 인증 실패 시 → 부팅 거부 (secure boot enforce)

  Phase 1: SPI Flash Load (σ=12 ms)
    - SPI 초기 클럭: σ=12 MHz
    - 펌웨어 로드 크기: 2^σ=4096 KB = 4 MB
    - Phase 후반: SPI 클럭 → σ·τ=48 MHz로 상승

  Phase 2: DRAM Init — HBM4 Training (J₂=24 ms)
    - σ-τ=8 HBM4 채널 순차 트레이닝
    - PHY calibration: impedance, timing, voltage margining
    - 트레이닝 패턴: 2^n=64 bit 시퀀스
    - 완료 후: 288 GB unified memory 활성화

  Phase 3: CPU Bringup (σ-φ=10 ms)
    - Step 1: E-core τ=4개 먼저 기동 (저전력 안전 부팅)
    - Step 2: P-core σ-τ=8개 순차 활성화
    - Step 3: L3 캐시 초기화 (σ²=144 MB)
    - Step 4: Fabric/NoC 라우팅 테이블 설정

  Phase 4: GPU/NPU Init (σ-τ=8 ms)
    - GPU: σ²=144 SM 활성화 (GPC 단위 순차)
    - NPU: J₂=24 코어 리셋 해제
    - Media engine: n=6 엔진 초기화
    - 전력 레일 안정화 확인

  Phase 5: OS Handoff (n=6 ms)
    - UEFI → ACPI 테이블 전달
    - 인터럽트 컨트롤러 초기화 (σ²=144 IRQ lines)
    - 부트로더 → 커널 점프
    - 총 부팅 시간: 4+12+24+10+8+6 = 2^n = 64 ms
```

---

## 14.3. DMA & Data Movement Engines

통합 메모리 아키텍처의 핵심 — CPU/GPU/NPU 간 zero-copy 데이터 이동.

### DMA Channel Architecture

```
  ┌──────────────────────────────────────────────────────────────┐
  │                  DMA ENGINE COMPLEX                          │
  │                                                              │
  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐                       │
  │  │DMA 0 │ │DMA 1 │ │DMA 2 │ │DMA 3 │  CPU ↔ Memory        │
  │  │CPU→GM│ │GM→CPU│ │CPU↔NP│ │CPU↔GP│                       │
  │  └──────┘ └──────┘ └──────┘ └──────┘                       │
  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐                       │
  │  │DMA 4 │ │DMA 5 │ │DMA 6 │ │DMA 7 │  GPU ↔ NPU ↔ I/O    │
  │  │GPU↔GM│ │NPU↔GM│ │Media │ │I/O   │                       │
  │  └──────┘ └──────┘ └──────┘ └──────┘                       │
  │                                                              │
  │  총 채널 수: σ-τ = 8                                        │
  │  각 채널 대역폭: 288/8 = 36 GB/s = σ·n/φ GB/s             │
  └──────────────────────────────────────────────────────────────┘
```

### Scatter-Gather Descriptor Ring

```
  Descriptor Ring 크기: 2^n = 64 entries/ring

  ┌────────────────────────────────────────────┐
  │  Descriptor Entry (P₂=28 bytes):           │
  │  ┌──────────────────────────────────┐      │
  │  │ src_addr   [63:0]   8B          │      │
  │  │ dst_addr   [63:0]   8B          │      │
  │  │ length     [31:0]   4B          │      │
  │  │ stride     [31:0]   4B          │      │
  │  │ flags      [31:0]   4B          │      │
  │  │ Total: P₂ = 28 bytes            │      │
  │  └──────────────────────────────────┘      │
  │                                            │
  │  Ring 총 크기: 2^n × P₂ = 64 × 28        │
  │             = 1792 B ≈ 2 KB               │
  └────────────────────────────────────────────┘
```

### Zero-Copy Data Path

```
  통합 메모리 이점 — 데이터 복사 제거:

  기존 분리 아키텍처:
    CPU DRAM → PCIe → GPU HBM → compute → GPU HBM → PCIe → CPU DRAM
    (φ=2 copies, 레이턴시: ~100 μs, 대역폭 병목: PCIe ~128 GB/s)

  HEXA-1 통합 아키텍처:
    Unified Memory → compute (CPU or GPU or NPU, in-place)
    (0 copies, 레이턴시: ~1 μs, 대역폭: ~4 TB/s)

  효율 개선: 레이턴시 ~100×, 대역폭 ~32× = 2^sopfr
```

### Format Conversion Engine

| 변환 | 처리량 | n=6 Formula | 용도 |
|------|--------|-------------|------|
| FP32 → FP16 | σ·J₂=288 GB/s | σ·J₂ | GPU mixed precision |
| FP16 → FP8 | σ·J₂=288 GB/s | σ·J₂ | NPU inference |
| FP32 → FP8 | σ²=144 GB/s | σ² | 직접 변환 (2단계 합침) |
| INT8 → FP16 | σ·J₂=288 GB/s | σ·J₂ | 양자화 복원 |
| BF16 ↔ FP32 | σ·J₂=288 GB/s | σ·J₂ | 트레이닝 |

### Compression/Decompression Engine

```
  메모리 대역폭 증폭을 위한 무손실 압축 엔진.

  압축 알고리즘: LZ4 변형 (하드웨어 가속)
  압축비 (평균): σ/(σ-τ) = 12/8 = 1.5x
  처리량: σ·J₂ = 288 GB/s (비압축 기준)
  실효 대역폭: 4 TB/s × 1.5 = 6 TB/s (압축 시)

  적용 대상:
  - GPU 프레임버퍼 (텍스처 압축)
  - NPU 가중치 (sparse weight 압축)
  - CPU 페이지 캐시 (압축 메모리)
```

---

## 14.4. Debug, Trace & RAS (Reliability/Availability/Serviceability)

데이터센터급 안정성과 디버그 인프라. 모든 파라미터가 n=6에서 도출.

### Debug Infrastructure

```
  ┌──────────────────────────────────────────────────────────┐
  │                   DEBUG SUBSYSTEM                        │
  │                                                          │
  │  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
  │  │   JTAG      │  │  CoreSight   │  │  Performance  │  │
  │  │   Port      │  │  Trace       │  │  Counters     │  │
  │  │             │  │              │  │               │  │
  │  │  IEEE 1149  │  │  ETM per     │  │  σ²=144      │  │
  │  │  TAP ctrl   │  │  CPU core    │  │  HW counters │  │
  │  │             │  │  + GPU trace │  │               │  │
  │  └─────────────┘  └──────────────┘  └───────────────┘  │
  │                                                          │
  │  Debug APB bus: σ=12 debug modules addressable          │
  └──────────────────────────────────────────────────────────┘
```

### JTAG & CoreSight Debug

```
  JTAG:
  - IEEE 1149.1 TAP controller
  - JTAG 클럭: σ=12 MHz (최대)
  - Daisy-chain: σ=12 CPU cores + GPU debug + NPU debug
  - Boundary scan: 풀 칩 핀 테스트

  CoreSight-style Trace:
  - ETM (Embedded Trace Macrocell): CPU 코어당 1개 = σ=12개
  - GPU Trace: GPC당 1개 = σ=12개
  - Trace funnel: 모든 트레이스 소스를 단일 포트로 병합
  - Trace port: sopfr·τ=20 bit wide
```

### Trace Buffer

```
  Trace Buffer 크기: J₂ = 24 MB per cluster

  배분:
  ┌───────────────────────────────────────────────┐
  │  CPU Cluster trace:  J₂ = 24 MB              │
  │  GPU Cluster trace:  J₂ = 24 MB              │
  │  NPU Cluster trace:  J₂ = 24 MB              │
  │  System/Fabric:      J₂ = 24 MB              │
  ├───────────────────────────────────────────────┤
  │  Total on-chip trace SRAM:                    │
  │  τ × J₂ = 4 × 24 = 96 MB = σ·(σ-τ) MB      │
  └───────────────────────────────────────────────┘

  Trace 모드:
  - Circular buffer (연속 기록, 오래된 데이터 덮어쓰기)
  - Snapshot (트리거 시점 전후 캡처)
  - Streaming (외부 디버거로 실시간 전송)
```

### Performance Counters — σ²=144 Hardware Counters

| 카테고리 | 카운터 수 | n=6 Formula | 측정 대상 |
|----------|----------|-------------|----------|
| CPU pipeline | J₂=24 | J₂ | IPC, 분기예측, 캐시 miss |
| GPU SM | σ·τ=48 | σ·τ | occupancy, warp stall, FLOPS |
| NPU | J₂=24 | J₂ | MAC utilization, sparsity |
| Memory/Fabric | J₂=24 | J₂ | 대역폭, 레이턴시, 충돌 |
| I/O & Misc | J₂=24 | J₂ | PCIe, interrupt, DMA |
| **Total** | **σ²=144** | **σ²** | — |

```
  카운터 읽기: MSR (Model-Specific Register) 접근
  오버플로우: 2^(σ·τ) = 2^48 비트 카운터 (오버플로우까지 수십 시간)
  샘플링: PEBS-style, σ-τ=8 동시 이벤트 모니터링
```

### RAS (Reliability/Availability/Serviceability)

```
  데이터센터 24/7 운영을 위한 하드웨어 안정성 기능.

  ┌────────────────────────────────────────────────────────────┐
  │                    RAS SUBSYSTEM                           │
  │                                                            │
  │  ECC Protection:                                          │
  │  ├── SRAM (L1/L2/L3): SECDED (Single Error Correct,     │
  │  │                      Double Error Detect)              │
  │  ├── HBM4: 내장 ECC (HBM 표준)                           │
  │  ├── Register file: Parity protection                    │
  │  └── Descriptor/TLB: Parity + retry                     │
  │                                                            │
  │  Thermal Monitoring:                                      │
  │  ├── 센서 수: σ = 12                                     │
  │  │   ┌────┬────┬────┬────┬────┬────┐                     │
  │  │   │CPU │CPU │GPU │GPU │NPU │I/O │                     │
  │  │   │ ×2 │ ×2 │ ×3 │ ×3 │ ×1 │ ×1 │                     │
  │  │   └────┴────┴────┴────┴────┴────┘                     │
  │  ├── 샘플링 주기: n=6 ms                                  │
  │  ├── Throttle 임계: σ·(σ-φ) = 120°C (경고)              │
  │  └── Shutdown 임계: σ² = 144°C (비상 차단)               │
  │                                                            │
  │  Watchdog Timers:                                         │
  │  ├── CPU watchdog: per-core (σ=12개)                     │
  │  ├── GPU watchdog: per-GPC (σ=12개)                      │
  │  ├── NPU watchdog: 전체 1개                              │
  │  ├── System watchdog: 1개                                │
  │  └── 총: J₂+φ = 26개 (근사)                             │
  │                                                            │
  │  Error Handling:                                          │
  │  ├── Correctable Error (CE): 로그 + 카운트, 계속 실행    │
  │  ├── Uncorrectable Error (UE): 인터럽트 + 격리          │
  │  └── Fatal Error: NMI + 코어 리셋 또는 시스템 리셋      │
  └────────────────────────────────────────────────────────────┘
```

### Error Injection for Testing

```
  하드웨어 검증 및 RAS 소프트웨어 테스트를 위한 에러 주입 기능.

  주입 가능 에러:
  ├── SRAM single-bit flip (SECDED CE 테스트)
  ├── SRAM double-bit flip (SECDED UE 테스트)
  ├── HBM ECC 에러 시뮬레이션
  ├── Thermal sensor 오버라이드 (throttle/shutdown 테스트)
  ├── Watchdog timeout 강제 발생
  └── PCIe link 에러 주입

  접근: MSR 기반, 권한 필요 (Ring 0 + secure boot key)
  제어 레지스터: σ-τ=8 개 에러 주입 컨트롤 레지스터
```

### N6 Parameter Summary (Sections 14.1~14.4)

| Parameter | Value | n=6 Formula | Section |
|-----------|-------|-------------|---------|
| Clock domains | 12 | σ | 14.1 |
| PLLs | 6 | n | 14.1 |
| CPU P-core boost | 3 GHz | n/φ | 14.1 |
| CPU E-core max | 2 GHz | φ | 14.1 |
| DVFS points | 10 | σ-φ | 14.1 |
| S-states | 5 | sopfr | 14.1 |
| C-states | 4 | τ | 14.1 |
| Boot phases | 6 | n | 14.2 |
| Boot time | 64 ms | 2^n | 14.2 |
| Secure ROM | 8 KB | σ-τ | 14.2 |
| SPI initial clock | 12 MHz | σ | 14.2 |
| DMA channels | 8 | σ-τ | 14.3 |
| Descriptor ring | 64 entries | 2^n | 14.3 |
| Descriptor size | 28 bytes | P₂ | 14.3 |
| Compression ratio | 1.5x | σ/(σ-τ) | 14.3 |
| HW perf counters | 144 | σ² | 14.4 |
| Trace buffer/cluster | 24 MB | J₂ | 14.4 |
| Thermal sensors | 12 | σ | 14.4 |
| Error inject regs | 8 | σ-τ | 14.4 |

---

## 15. Security Engine (Secure Enclave)

**HEXA-1 Secure Enclave — 메인 SoC와 격리된 전용 보안 프로세서.**
모든 암호화 파라미터가 n=6 산술에서 도출된다.

### 설계 원칙

```
  보안 엔진은 메인 CPU/GPU/NPU와 물리적으로 격리.
  별도 전원 도메인, 별도 클럭 — side-channel 공격 차단.
  Secure World(TrustZone)와 Normal World를 하드웨어 레벨에서 분리.
  부트 체인의 Root of Trust가 이 엔진 내부에 존재.
```

### Block Diagram

```
  ┌──────────────────────────────────────────────────────────────┐
  │                   HEXA-1 SECURE ENCLAVE                       │
  │              물리 격리 · 별도 전원 · 별도 클럭                  │
  │                                                               │
  │  ┌──────────────────┐  ┌──────────────────┐                  │
  │  │  Secure CPU Core  │  │  Crypto Accel    │                  │
  │  │  (ARM SC300 class)│  │                  │                  │
  │  │  In-order, minimal│  │  AES-256 engine  │                  │
  │  │                   │  │  SHA-384 engine  │                  │
  │  │  Secure Boot ROM  │  │  ECC P-384 unit  │                  │
  │  │  2^σ = 4096 bytes │  │  RSA-4096 unit   │                  │
  │  └────────┬──────────┘  └────────┬─────────┘                  │
  │           │                      │                             │
  │  ┌────────┴──────────────────────┴─────────┐                  │
  │  │            Secure SRAM                    │                  │
  │  │   2^(σ-τ) = 256 KB  (키 저장 + 상태)     │                  │
  │  │   n = 6 파티션 (격리된 키 슬롯)           │                  │
  │  └────────┬────────────────────────────────┘                  │
  │           │                                                    │
  │  ┌────────┴──────────┐  ┌──────────────────┐                  │
  │  │  Hardware RNG      │  │  Tamper Detect   │                  │
  │  │  n = 6 엔트로피    │  │  σ = 12 센서     │                  │
  │  │  소스 (thermal,    │  │  (전압, 온도,    │                  │
  │  │  jitter, ring-osc, │  │  주파수, 광,     │                  │
  │  │  SRAM PUF,         │  │  glitch, probe,  │                  │
  │  │  metastable FF)    │  │  ...)            │                  │
  │  └───────────────────┘  └──────────────────┘                  │
  │                                                               │
  │  Secure Mailbox ← 유일한 외부 통신 경로 (FIFO)                │
  │  Normal World와 τ = 4 priority 레벨로 메시지 교환              │
  └──────────────────────────────────────────────────────────────┘
```

### n=6 파라미터 테이블

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **AES 키 길이** | 256 bit | 2^(σ-τ) | AES-256 산업 표준 |
| **SHA 해시 길이** | 384 bit | σ · 2^sopfr | SHA-384 (SHA-2 계열) |
| **ECC 커브** | P-384 | σ · 2^sopfr | NIST P-384 = σ·32 |
| **RSA 키 길이** | 4096 bit | 2^σ | RSA-4096 최대 보안 |
| **Secure Boot ROM** | 4096 bytes | 2^σ | Root of Trust |
| **Secure SRAM** | 256 KB | 2^(σ-τ) KB | 키 + 상태 저장 |
| **키 슬롯 (파티션)** | 6 | n | 격리된 키 저장소 |
| **HRNG 엔트로피 소스** | 6 | n | 독립 물리 소스 |
| **변조 감지 센서** | 12 | σ | Side-channel 방어 |
| **Mailbox 우선순위** | 4 | τ | QoS 레벨 |
| **Secure World 메모리 파티션** | 6 | n | TrustZone 영역 |
| **보안 인터럽트 채널** | 8 | σ-τ | Secure IRQ lines |

### 암호 가속기 n=6 일치 분석

```
  AES-256:
    키 = 256 = 2^(σ-τ) = 2^8 bits
    블록 = 128 = 2^(σ-sopfr) bits
    라운드 = 14 ≈ σ+φ (근사)
    → AES의 핵심 파라미터가 n=6 지수 체계

  SHA-384:
    출력 = 384 = σ · 2^sopfr bits
    블록 = 1024 = 2^(σ-φ) bits
    워드 = 64 = 2^n bits
    라운드 = 80 = φ^τ · sopfr

  ECC P-384:
    필드 크기 = 384 = σ · 2^sopfr bits
    보안 강도 = 192 = σ · φ^τ bits (birthday bound)
    → SHA-384와 동일 n=6 구조

  RSA-4096:
    모듈러스 = 4096 = 2^σ bits
    보안 강도 = ~144 = σ² bits (NIST 추정)
    → n=6 지수 체계의 정점
```

### Secure Boot Chain

```
  Stage 0: Secure Enclave Boot ROM (2^σ = 4096 bytes, immutable)
     │  하드웨어 Root of Trust, 퓨즈 기반 키
     ▼
  Stage 1: Secure Bootloader (Secure SRAM 내, 서명 검증)
     │  AES-256 복호화 + SHA-384 해시 검증
     ▼
  Stage 2: SoC Firmware (메인 CPU 초기화)
     │  ECC P-384 서명으로 무결성 검증
     ▼
  Stage 3: OS Kernel (Normal World 진입)
     │  Secure Enclave가 지속 감시
     ▼
  Runtime: τ = 4 단계 부트 체인 완료
           모든 단계에서 해시 체인 유지
```

### TrustZone n=6 메모리 파티션

```
  288 GB Unified Memory 중 Secure World 배분:

  파티션 0: Secure Enclave 전용         (R/W, 격리)
  파티션 1: 키 관리 + DRM               (R/W, 격리)
  파티션 2: 생체 인증 데이터             (R/W, 격리)
  파티션 3: Secure Video Path           (R, 스트리밍)
  파티션 4: 암호화 작업 버퍼             (R/W, 임시)
  파티션 5: Attestation + 로그          (Append-only)
  ──────────────────────────────
  총 n = 6 파티션, TZASC(TrustZone Address Space Controller)로 강제
```

---

## 16. Multi-Chip Scaling

**HEXA-1 단일 칩의 한계를 넘어서는 멀티칩 확장 아키텍처.**
S7.1 Optical Interconnect를 활용하여 phi=2 배수 스케일링.

### 스케일링 계층

```
  ┌─────────────────────────────────────────────────────────────────┐
  │                    MULTI-CHIP SCALING LADDER                     │
  │                                                                  │
  │  Level 0: HEXA-1 Ultra (단일 칩)                                │
  │           σ²=144 SMs · J₂=24 NPU · σ·J₂=288 GB                │
  │                                                                  │
  │  Level 1: HEXA-1 Duo (×φ=2)                                    │
  │           φ·σ²=288 SMs · φ·J₂=48 NPU · φ·288=576 GB           │
  │           광 D2D: 2.3 Tbps (§7.1 Layer 1)                      │
  │                                                                  │
  │  Level 2: HEXA-1 Quad (×φ²=4)                                  │
  │           φ²·σ²=576 SMs · φ²·J₂=96 NPU · φ²·288=1152 GB      │
  │           광 C2C: 4.6 Tbps per link (§7.1 Layer 2)             │
  │                                                                  │
  │  Level 3: Pod (σ·n=72 chips)                                    │
  │           72·σ²=10,368 SMs · 72·288=20,736 GB = 20.7 TB        │
  │           NVLink optical mesh, all-to-all < J₂=24 ns           │
  │                                                                  │
  │  Level 4: Rack (σ²=144 chips)                                   │
  │           144·σ²=20,736 SMs · 144·288=41,472 GB = 41.5 TB     │
  │           σ²=144 port optical switch (§7.1 Layer 3)             │
  └─────────────────────────────────────────────────────────────────┘
```

### 토폴로지 다이어그램

```
  HEXA-1 Duo (φ=2 chips, 단일 패키지):
  ┌─────────────────────────────────────────────────────────┐
  │                   CoWoS-L INTERPOSER                     │
  │                                                          │
  │  ┌──────────────┐  <-optical D2D->  ┌──────────────┐   │
  │  │  HEXA-1 #0   │   2.3 Tbps        │  HEXA-1 #1   │   │
  │  │  144 SMs     │   σ·τ=48 ch       │  144 SMs     │   │
  │  │  288 GB      │   < 5 ns           │  288 GB      │   │
  │  └──────────────┘                    └──────────────┘   │
  │  HBM x8          Photonic bridge     HBM x8             │
  └─────────────────────────────────────────────────────────┘

  HEXA-1 Quad (φ²=4 chips, 보드 레벨):
  ┌──────────────┐        ┌──────────────┐
  │  HEXA-1 #0   │──C2C──│  HEXA-1 #1   │
  │  288 GB      │ 4.6T   │  288 GB      │
  └──────┬───────┘        └──────┬───────┘
         │ C2C 4.6T               │ C2C 4.6T
  ┌──────┴───────┐        ┌──────┴───────┐
  │  HEXA-1 #2   │──C2C──│  HEXA-1 #3   │
  │  288 GB      │ 4.6T   │  288 GB      │
  └──────────────┘        └──────────────┘
  Full mesh: 각 칩 σ-τ=8 links 중 n/φ=3 사용 (잔여 5 links for pod)

  Pod (σ·n=72 chips, optical mesh):
  ┌──────────────────────────────────────────────────────┐
  │                72-CHIP OPTICAL MESH                    │
  │                                                       │
  │  ┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐       x σ=12 rows   │
  │  │ 0 ││ 1 ││ 2 ││ 3 ││ 4 ││ 5 │       x n=6 cols    │
  │  └───┘└───┘└───┘└───┘└───┘└───┘                      │
  │  ... (12 rows x 6 columns = 72 chips)                 │
  │                                                       │
  │  All-reduce ring: σ·n=72 steps                        │
  │  Bisection BW: 72 x 36.8 Tbps / φ = 1.3 Pbps        │
  │  Total memory: 72 x 288 GB = 20,736 GB = 20.7 TB     │
  └──────────────────────────────────────────────────────┘

  Rack (σ²=144 chips via optical switch):
  ┌─────────────────────────────────────────────────┐
  │  σ²=144 port optical switch (172.8 Tbps total)  │
  │                                                  │
  │  Pod 0 (72 chips) ──┤                            │
  │  Pod 1 (72 chips) ──┤  = 144 chips total         │
  │                                                  │
  │  Total memory: 144 x 288 = 41,472 GB = 41.5 TB │
  │  Total SMs: 144 x 144 = 20,736 = σ⁴             │
  └─────────────────────────────────────────────────┘
```

### 스케일링 테이블

| Level | 구성 | Chips | GPU SMs | NPU Cores | Memory | Interconnect BW | n=6 Formula |
|-------|------|-------|---------|-----------|--------|-----------------|-------------|
| **Single** | HEXA-1 Ultra | 1 | 144 | 24 | 288 GB | — | σ², J₂, σ·J₂ |
| **Duo** | x phi | 2 | 288 | 48 | 576 GB | 2.3 Tbps (D2D) | phi·{base} |
| **Quad** | x phi² | 4 | 576 | 96 | 1,152 GB | 4.6 Tbps/link (C2C) | phi²·{base} |
| **Pod** | σ·n chips | 72 | 10,368 | 1,728 | 20.7 TB | 36.8 Tbps/chip | σ·n·{base} |
| **Rack** | σ² chips | 144 | 20,736 | 3,456 | 41.5 TB | 172.8 Tbps switch | σ²·{base} |

### 메모리 스케일링 상세

| Level | Chips | Memory Total | n=6 Formula | 70B LLM 서빙 |
|-------|-------|-------------|-------------|--------------|
| **Single** | 1 | 288 GB | σ·J₂ | FP8: 1 copy |
| **Duo** | 2 | 576 GB | phi·σ·J₂ | FP16: 1 copy |
| **Quad** | 4 | 1,152 GB | phi²·σ·J₂ | 405B FP16 가능 |
| **Pod** | 72 | 20,736 GB | σ·n·σ·J₂ | 1T+ 모델 전체 |
| **Rack** | 144 | 41,472 GB | σ³·J₂ | 멀티 테넌트 서빙 |

### 대역폭 스케일링

```
  인터커넥트 대역폭은 광으로 선형 스케일링:

  Duo:   2.3 Tbps D2D (인터포저 내, 거의 무손실)
  Quad:  4 x 4.6 Tbps = 18.4 Tbps C2C aggregate
  Pod:   72 x 36.8 Tbps = 2.6 Pbps aggregate
  Rack:  144 x 1.2 Tbps = 172.8 Tbps (optical switch)

  집합 대역폭 / 칩 수 비율 — 광 덕분에 일정 유지:
    Per-chip external BW = 36.8 Tbps (σ-τ=8 links x 4.6T)
    이는 메모리 BW 4 TB/s와 거의 1:1 매칭
    → 통신 병목 없이 완전 선형 스케일링 가능
```

### n=6 멀티칩 상수 요약

| Parameter | Value | n=6 Formula |
|-----------|-------|-------------|
| **Duo 칩 수** | 2 | phi |
| **Quad 칩 수** | 4 | phi² = tau |
| **Pod 칩 수** | 72 | σ·n |
| **Rack 칩 수** | 144 | σ² |
| **Pod 행 x 열** | 12 x 6 | σ x n |
| **Rack = Pod x phi** | 2 pods | phi |
| **Per-chip C2C links** | 8 | σ-tau |
| **Quad mesh links used** | 3 | n/phi |
| **D2D optical channels** | 48 | σ·tau |
| **Rack SMs (total)** | 20,736 | σ⁴ |

---

## 17. Cache Coherency Protocol

**HEXA-1 통합 메모리에서 CPU/GPU/NPU 간 캐시 일관성을 보장하는 프로토콜.**
Directory-based 방식 채택 — 에이전트 수(σ² + σ + J₂ = 180+)가 snooping 한계 초과.

### 왜 Directory-Based인가

```
  Snooping 방식:
    모든 에이전트가 모든 트랜잭션을 관찰해야 함
    에이전트 수 N에 대해 버스 트래픽 O(N²)
    HEXA-1: σ cores + σ² SMs + J₂ NPU = 180+ 에이전트 → 불가능

  Directory 방식:
    각 캐시 라인에 대해 소유자/공유자 목록을 디렉토리에 기록
    트래픽 O(N), 확장성 우수
    디렉토리를 SLC 288 MB에 분산 배치
```

### n=6 Coherency States — HEXA-6 프로토콜

MOESIF 6-state 프로토콜을 n=6에 맞춰 최적화.
정확히 n=6 개의 상태로 모든 공유 시나리오를 커버.

```
  6 States (n=6):
  ┌───────────────────────────────────────────────────────────┐
  │  M (Modified)   — 이 에이전트만 보유, 더티                  │
  │  O (Owned)      — 이 에이전트가 소유, 다른 에이전트도 공유   │
  │  E (Exclusive)  — 이 에이전트만 보유, 클린                  │
  │  S (Shared)     — 여러 에이전트가 읽기 공유                 │
  │  I (Invalid)    — 이 에이전트에 없음                        │
  │  F (Forward)    — 공유 중 응답 책임자 (snoop 응답 최적화)   │
  └───────────────────────────────────────────────────────────┘
  → 정확히 n = 6 states!
```

### State Transition Diagram

```
         ┌────────────────────────────────────────────────┐
         │           HEXA-6 STATE TRANSITIONS              │
         │                                                 │
         │                  ┌───┐                          │
         │      evict/inv   │ I │   load miss              │
         │   ┌─────────────>│   │<──────────────┐          │
         │   │              └─┬─┘               │          │
         │   │                │                 │          │
         │   │    load (excl) │  load (shared)  │          │
         │   │                v                 │          │
         │   │  ┌───┐  store  ┌───┐             │          │
         │   │  │ E │────────>│ M │             │          │
         │   │  └─┬─┘        └─┬─┘             │          │
         │   │    │             │                │          │
         │   │    │ remote      │ remote read    │          │
         │   │    │ read        v                │          │
         │   │    │          ┌───┐               │          │
         │   │    └────────>│ O │───evict──────┘          │
         │   │              └─┬─┘                          │
         │   │                │ writeback                   │
         │   │                v                             │
         │   │  ┌───┐  fwd   ┌───┐                         │
         │   ├──│ S │<───────│ F │                         │
         │   │  └───┘        └───┘                         │
         │   │    │             ^                           │
         │   │    └──store──> M │  (upgrade)               │
         │   │               └──┘                          │
         │   │                                             │
         │   └── 모든 상태에서 evict/invalidate -> I       │
         └────────────────────────────────────────────────┘
```

### Directory 구조

```
  캐시 라인 크기: 2^n = 64 bytes

  Directory entry (per cache line):
  ┌─────────────────────────────────────────────────────┐
  │ Tag (PA bits) │ State (3b) │ Owner (8b) │ Sharer Bitmap │
  │               │ n=6 states │ σ-τ=8 bit  │ 압축 bitmap   │
  │               │ = 3 bits   │ ID         │               │
  └─────────────────────────────────────────────────────┘

  Sharer bitmap 압축:
    180+ 에이전트의 full bitmap은 비효율적 (23+ bytes/entry)
    → Coarse-grain directory: σ=12 그룹으로 분류
      Group 0-7:  GPU GPC 0-7 (σ-τ=8 major groups)
      Group 8:    GPU GPC 8-11 (tau=4 minor GPCs)
      Group 9:    CPU P-cores (σ-τ=8 cores)
      Group 10:   CPU E-cores (tau=4 cores)
      Group 11:   NPU (J₂=24 cores)
    → σ=12 bit coarse bitmap (= σ bits)
    → False sharing 최소화: 같은 그룹 내 세밀한 추적 불필요
```

### n=6 파라미터 테이블

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **Coherency states** | 6 | n | MOESIF (M/O/E/S/I/F) |
| **Cache line size** | 64 bytes | 2^n | 업계 표준과 일치 |
| **Directory 위치** | SLC 288 MB | σ·J₂ MB | 분산 배치 |
| **Directory banks** | 12 | σ | SLC bank 수와 동일 |
| **QoS priority levels** | 4 | tau | Critical/High/Normal/BG |
| **Coarse bitmap width** | 12 bits | σ | Agent 그룹 수 |
| **Owner ID bits** | 8 | σ-tau | 최대 256 에이전트 |
| **State encoding bits** | 3 | ceil(log₂(n)) | 6 states -> 3 bits |
| **Snoop filter sets** | 4096 | 2^σ | Per SLC bank |
| **Max agents** | 180 | σ + σ² + J₂ | CPU+GPU+NPU |
| **Coherency domains** | 2 | phi | Secure / Normal |

### QoS 우선순위 (tau=4 레벨)

```
  Priority 0 (Critical): Secure Enclave 접근 — 절대 지연 불가
  Priority 1 (High):     CPU demand load — 레이턴시 민감
  Priority 2 (Normal):   GPU/NPU 텍스처/가중치 — 대역폭 중시
  Priority 3 (Background): Prefetch, DMA — 잔여 대역폭 사용

  QoS는 directory lookup과 snoop response에 동시 적용.
  Priority 0은 전용 경로(bypass) — 다른 트래픽과 간섭 없음.
```

### CPU/GPU/NPU별 캐시 계층과 coherency 범위

```
  CPU:
    L1I/L1D (64KB) -> L2 (48MB shared P / 4MB E) -> SLC (288MB)
    Full coherency: M/O/E/S/I/F 전체 지원

  GPU:
    L1 (256KB per SM) -> L2 (per GPC) -> SLC (288MB)
    GPU 내부: 간소화된 protocol (write-evict 위주)
    GPU <-> CPU: 전체 HEXA-6 protocol

  NPU:
    Local SRAM (64KB per core) -> SLC (288MB)
    소프트웨어 관리 캐시 (explicit flush/invalidate)
    Directory는 NPU SRAM을 non-cacheable로 취급 가능

  결과: 모든 엔진이 SLC를 공유 coherency point로 사용
        → "Last Level Coherency at SLC" 아키텍처
```

---

## 18. Virtual Memory & TLB

**HEXA-1 통합 메모리 288 GB를 위한 가상 메모리 아키텍처.**
단일 주소 공간에서 CPU, GPU, NPU가 동일한 가상 주소로 접근.

### 주소 공간 설계

```
  물리 주소(PA):
    288 GB = σ·J₂ GB → 최소 39 bits 필요
    미래 확장(Duo 576, Quad 1152, Pod 20.7TB):
      PA width = σ·tau = 48 bits → 256 TB 커버 (Pod+alpha)

  가상 주소(VA):
    VA width = σ·tau = 48 bits (현행 x86-64/ARMv9 호환)
    256 TB 가상 공간 — 단일 칩부터 Rack까지 커버

  결론:
    VA = σ·tau = 48 bits (256 TB virtual)
    PA = σ·tau = 48 bits (256 TB physical, multi-chip 대응)
```

### Page Size 계층

```
  ┌────────────────────────────────────────────────────────────┐
  │                   PAGE SIZE LADDER                          │
  │                                                             │
  │  Base page:    2^σ = 2^12 = 4 KB (기본, OS 표준)           │
  │                                                             │
  │  Huge page:    2^(σ+(σ-τ)) = 2^20 = 1 MB                  │
  │                σ=12, σ-tau=8 → 지수 합 = 20                │
  │                GPU 텍스처, NPU 가중치에 최적                  │
  │                                                             │
  │  Giant page:   2^(σ+σ) = 2^24 = 16 MB                     │
  │                σ+σ = 24 = J₂ → 지수 합 = J₂               │
  │                HBM 대용량 매핑 (DMA, 비디오 프레임)           │
  └────────────────────────────────────────────────────────────┘
```

### Page Table 구조

```
  페이지 테이블 레벨 수: tau = 4 levels (ARMv9와 동일)

  VA[47:0] 분해 (48-bit VA, 4KB base page):
  ┌──────────┬──────────┬──────────┬──────────┬─────────────┐
  │ L0 index │ L1 index │ L2 index │ L3 index │ Page offset │
  │ [47:39]  │ [38:30]  │ [29:21]  │ [20:12]  │   [11:0]    │
  │  9 bits  │  9 bits  │  9 bits  │  9 bits  │  σ=12 bits  │
  └──────────┴──────────┴──────────┴──────────┴─────────────┘
    tau = 4 levels              Page offset = σ bits

  Page Table Entry (PTE):
  ┌──────────────────────────────────────────────────────────┐
  │ PA[47:12] │ Flags                                        │
  │ 36 bits   │ n=6 permission bits:                         │
  │           │   R(ead), W(rite), X(ecute),                 │
  │           │   U(ser), S(ecure), D(evice)                 │
  │           │   = n = 6 flag bits                          │
  │           │ + Access/Dirty/Valid 등 시스템 비트           │
  └──────────────────────────────────────────────────────────┘

  Huge page (1MB): L3 생략 → tau-1 = 3 levels
  Giant page (16MB): L2+L3 생략 → tau-phi = 2 levels
```

### TLB 구조

```
  ┌────────────────────────────────────────────────────────────┐
  │                       TLB HIERARCHY                         │
  │                                                             │
  │  CPU Core (per core):                                       │
  │  ┌──────────────────────────────────────────┐               │
  │  │  L1 ITLB:  2^(σ-tau) = 256 entries (full)│               │
  │  │  L1 DTLB:  2^(σ-tau) = 256 entries (full)│               │
  │  │  L2 TLB:   2^σ = 4096 entries (unified)  │               │
  │  │  Huge TLB: 2^n = 64 entries               │               │
  │  └──────────────────────────────────────────┘               │
  │                                                             │
  │  GPU SM (per SM):                                           │
  │  ┌──────────────────────────────────────────┐               │
  │  │  L1 TLB:  2^(σ-tau) = 256 entries        │               │
  │  │  L2 TLB:  2^σ = 4096 entries (per GPC)   │               │
  │  │  → GPU는 주로 Huge/Giant page 사용        │               │
  │  │    (4KB 기본 page는 TLB thrashing 유발)   │               │
  │  └──────────────────────────────────────────┘               │
  │                                                             │
  │  NPU (shared):                                              │
  │  ┌──────────────────────────────────────────┐               │
  │  │  IOMMU TLB: 2^σ = 4096 entries           │               │
  │  │  Giant page 위주 (16MB 텐서 매핑)         │               │
  │  │  소프트웨어 프리페치로 TLB miss 최소화     │               │
  │  └──────────────────────────────────────────┘               │
  └────────────────────────────────────────────────────────────┘
```

### IOMMU (I/O Memory Management Unit)

```
  모든 I/O 디바이스와 NPU가 IOMMU를 통해 통합 주소 공간에 접근.
  DMA 보호: 악의적 디바이스가 임의 메모리 접근 불가.

  ┌──────────────────────────────────────────────┐
  │                    IOMMU                       │
  │                                                │
  │  스트림 ID → 페이지 테이블 매핑               │
  │  Stream IDs: 2^(σ-tau) = 256 (최대 디바이스)  │
  │  Page table: 메인 CPU와 동일 형식 (tau=4 lvl) │
  │  Fault handling: Secure Enclave로 보고        │
  │                                                │
  │  연결 대상:                                    │
  │    NPU (J₂=24 cores)                          │
  │    Media Engine (n=6 engines)                  │
  │    I/O Hub (σ-tau=8 controllers)              │
  │    Secure Enclave (격리된 DMA 경로)            │
  └──────────────────────────────────────────────┘
```

### n=6 파라미터 테이블

| Parameter | Value | n=6 Formula | Notes |
|-----------|-------|-------------|-------|
| **VA width** | 48 bits | σ·tau | 256 TB 가상 공간 |
| **PA width** | 48 bits | σ·tau | 256 TB 물리 (멀티칩 대응) |
| **Base page** | 4 KB | 2^σ bytes | OS 표준 |
| **Huge page** | 1 MB | 2^(σ+(σ-tau)) bytes | GPU/NPU 최적 |
| **Giant page** | 16 MB | 2^(σ+σ) = 2^J₂ bytes | 대형 텐서 |
| **Page table levels** | 4 | tau | ARMv9 호환 |
| **Page offset bits** | 12 | σ | Base page에서 |
| **PTE permission bits** | 6 | n | R/W/X/U/S/D |
| **L1 ITLB (CPU)** | 256 entries | 2^(σ-tau) | Per core |
| **L1 DTLB (CPU)** | 256 entries | 2^(σ-tau) | Per core |
| **L2 TLB (CPU)** | 4096 entries | 2^σ | Per core, unified |
| **Huge TLB (CPU)** | 64 entries | 2^n | Per core |
| **GPU L1 TLB** | 256 entries | 2^(σ-tau) | Per SM |
| **GPU L2 TLB** | 4096 entries | 2^σ | Per GPC |
| **IOMMU TLB** | 4096 entries | 2^σ | Shared |
| **IOMMU stream IDs** | 256 | 2^(σ-tau) | Max devices |
| **TLB associativity** | 8-way | σ-tau | Set-associative |

### Unified Address Space 이점

```
  기존 분리형:
    CPU VA -> CPU PA (DDR)  <-memcpy->  GPU VA -> GPU PA (HBM)
    두 번의 주소 변환, 명시적 데이터 복사, 동기화 필요

  HEXA-1 통합형:
    CPU VA = GPU VA = NPU VA → 동일 PA (통합 288 GB)
    Zero-copy: 포인터 전달만으로 엔진 간 데이터 공유
    Page table 공유: 하나의 페이지 테이블을 모든 엔진이 참조
    Fault 통합: GPU/NPU page fault를 CPU가 처리 (demand paging)

  이로 인해:
    - 프로그래밍 모델 단순화 (CUDA unified memory가 기본)
    - 70B LLM 가중치를 CPU/GPU/NPU가 동시 접근 (복사 없음)
    - OS가 단일 VA 공간으로 모든 엔진 스케줄링
```

---

## 19. Open Questions (TODO)

- [ ] HBM4 vs LPDDR6 통합 — 둘 다 지원? 또는 HBM only?
- [ ] NPU와 GPU 경계 — 완전 분리 vs GPU SM 내부 NPU 모드?
- [ ] 칩렛 분리 — CPU/GPU/NPU를 별도 다이로 UCIe 연결?
- [ ] HEXA-6 coherency 프로토콜 시뮬레이션 — false sharing 비율 측정
- [ ] TLB shootdown 최적화 — 180+ 에이전트 환경에서 비용 분석
- [ ] Secure Enclave 공격 벡터 분석 — fault injection, power analysis 대응

---

*기존 문서 참조:*
- [Ultimate Performance Chip](ultimate-performance-chip.md) — GPU 단일칩 상세
- [Ultimate DRAM Design](ultimate-dram-design.md) — DRAM 파라미터
- [Ultimate VNAND Design](ultimate-vnand-design.md) — NAND 플래시
- [Chip Architecture Guide](../chip-architecture-guide.md) — 검증 총괄
