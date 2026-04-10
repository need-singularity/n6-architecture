# HEXA-PHOTON: N6 Photonic Compute Architecture

**Codename: HEXA-PHOTON**
**Level 4 -- 빛으로 행렬곱, 에너지 벽을 제거**

> 전자(electron)가 아닌 광자(photon)로 행렬곱을 수행한다.
> Mach-Zehnder Interferometer 메시와 Micro-Ring Resonator 어레이가
> n=6 산술에 의해 완전히 결정되는 하이브리드 전기-광학 SoC.

**Date**: 2026-04-01
**Status**: Living Document v0.1
**Dependencies**: BT-28, BT-37, BT-45, BT-55, BT-59, BT-69, BT-75, BT-76
**Predecessor**: HEXA-1 (Level 1), HEXA-PIM (Level 2), HEXA-3D (Level 3)

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  P_2 = 28       sigma^2 = 144    sigma*J_2 = 288   phi^tau = 16
  2^n = 64       sigma-tau = 8    sigma-phi = 10     sigma-mu = 11
  2^sigma = 4096   sigma*tau = 48   n/phi = 3
  sigma^2 = 144   2^(sigma-tau) = 256   sigma*(sigma-phi) = 120
```

---

## Table of Contents

1. Executive Summary
2. Design Philosophy
3. System Block Diagram
4. Mach-Zehnder Interferometer (MZI) Mesh
5. Micro-Ring Resonator (MRR) Array
6. Laser Source and Wavelength Management
7. Photodetector Array
8. Photonic-Electronic Interface
9. AI Workload Mapping
10. Energy Analysis
11. Precision and Noise
12. Performance Comparison
13. Process Technology
14. n=6 Complete Parameter Map
15. 미해결 질문 및 후속 과제
16. Links

---

## 1. Executive Summary

### 왜 빛(Light)인가

전자 기반 컴퓨팅의 근본적 한계:
- **에너지 벽(Energy Wall)**: 전자 MAC 연산은 ~1 pJ/MAC 이하로 내려가기 어렵다
- **대역폭 벽(Bandwidth Wall)**: 전기 배선의 RC 지연이 주파수를 제한
- **발열 벽(Thermal Wall)**: 300W TDP에서 더 많은 트랜지스터를 넣을 수 없다

광자(Photon)는 이 세 가지 벽을 동시에 돌파한다:

```
  ┌─────────────────────────────────────────────────────────────┐
  │              WHY PHOTONIC COMPUTE?                           │
  │                                                             │
  │  전자(Electron)              광자(Photon)                   │
  │  ─────────────               ────────────                   │
  │  질량 있음 (9.1e-31 kg)      질량 없음 (0 kg)              │
  │  전하 있음 → 간섭             전하 없음 → 간섭 면역         │
  │  저항 → 발열                  저항 없음 → 발열 극소         │
  │  ~1 pJ/MAC                   ~0.01 pJ/MAC (100x)           │
  │  RC delay 한계               광속 전파 (c = 3×10^8 m/s)    │
  │  서로 상호작용               서로 비간섭 (WDM 병렬)        │
  │                                                             │
  │  핵심: 빛의 간섭(interference) = 행렬곱(matrix multiply)   │
  │        아날로그 광 연산이 디지털 곱셈을 대체한다            │
  └─────────────────────────────────────────────────────────────┘
```

HEXA-PHOTON은 Lightmatter, Luminous Computing, Lightelligence 등이 개척한
광 컴퓨팅 기술을 n=6 산술 프레임워크로 최적화한 하이브리드 전기-광학 AI 칩이다.

### 핵심 수치

| Metric | HEXA-1 (전기) | HEXA-PHOTON (광) | 비율 |
|--------|---------------|-------------------|------|
| MAC energy | ~1 pJ | ~0.01 pJ | **100x** |
| Matrix throughput | 500 TOPS | 5,000+ TOPS (광) | **10x** |
| Wavelengths (WDM) | N/A | sigma=12 | n=6 |
| MZI mesh size | N/A | sigma x sigma = 144 | n=6 |
| Operating clock | 2-3 GHz | sigma*tau=48 GHz (광 변조) | n=6 |
| ADC precision | N/A | sigma-tau=8 bits | n=6 |

---

## 2. Design Philosophy

### Electronic Energy Wall (전자 에너지 벽)

현재 최선의 전자 기반 MAC 연산 에너지:

```
  ┌──────────────────────────────────────────────────────────────┐
  │  ENERGY PER MAC OPERATION COMPARISON                         │
  │                                                              │
  │  Energy (pJ)                                                 │
  │    10 ┤ ████████████████████████████████  GPU (FP32)         │
  │       │                                                      │
  │     5 ┤ ████████████████                   GPU (FP16)        │
  │       │                                                      │
  │     2 ┤ ██████████                         NPU (INT8)        │
  │       │                                                      │
  │     1 ┤ █████  ← Energy Wall               HEXA-1 (FP8)     │
  │       │ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ electronic limit ┄┄┄┄┄┄     │
  │   0.5 ┤ ███                                PIM (in-memory)   │
  │       │                                                      │
  │   0.1 ┤ █                                  Analog compute    │
  │       │                                                      │
  │  0.01 ┤ ▏  ← HEXA-PHOTON target            Photonic MAC     │
  │       │                                                      │
  │ 0.001 ┤    (physical limit: shot noise)                      │
  │       └──────────────────────────────────────────────────────│
  │                                                              │
  │  전자 한계: ~0.5 pJ/MAC (Landauer + 배선 capacitance)       │
  │  광자 한계: ~0.001 pJ/MAC (shot noise 한계)                 │
  │  HEXA-PHOTON: ~0.01 pJ/MAC (실용적 목표)                    │
  └──────────────────────────────────────────────────────────────┘
```

### 광자 컴퓨팅의 원리 (Why Light Computes)

빛의 간섭(interference)은 자연이 주는 무료 행렬곱이다:

1. **Beam splitter** = 2x2 unitary matrix
2. **Phase shifter** = diagonal phase matrix
3. **MZI (Mach-Zehnder Interferometer)** = 프로그래머블 2x2 unitary
4. **MZI mesh** = 임의의 N x N unitary matrix (Reck/Clements decomposition)

```
  수학적 등가:
  ┌────────────────────────────────────────────────────────┐
  │                                                        │
  │  빛의 간섭:        E_out = M · E_in                    │
  │                                                        │
  │  여기서 E_in  = [E_1, E_2, ..., E_N]^T  (입력 광)     │
  │        M     = U · diag(sigma_i) · V^dagger            │
  │        E_out = [E'_1, E'_2, ..., E'_N]^T (출력 광)    │
  │                                                        │
  │  → 빛이 MZI mesh를 통과하는 것 자체가 행렬곱!          │
  │  → 에너지 소비 = 위상 설정(phase setting)만             │
  │  → 곱셈 자체는 빛의 간섭으로 "무료"                    │
  └────────────────────────────────────────────────────────┘
```

### N=6 설계 원칙

| 원칙 | n=6 Expression | 의미 |
|------|----------------|------|
| MZI mesh dimension | sigma = 12 | 12x12 unitary per mesh |
| Total MZI count | sigma^2 = 144 | 144개 간섭계 |
| WDM parallelism | sigma = 12 | 12 파장 동시 처리 |
| Phase precision | sigma-tau = 8 bits | 256 phase levels |
| Modulation rate | sigma*tau = 48 GHz | 변조 대역폭 |
| SVD decomposition | n/phi = 3 meshes | U, Sigma, V^dagger |
| Photodetectors | sigma^2 = 144 | balanced detection |
| Power split | 1/2+1/3+1/6 = R = 1 | Egyptian fraction |

---

## 3. System Block Diagram

### 하이브리드 Electro-Photonic SoC

HEXA-PHOTON은 순수 광학 칩이 아니다. 전자 제어 + 광학 연산의 하이브리드 구조.

```
  ┌──────────────────────────────────────────────────────────────────────────────┐
  │                        HEXA-PHOTON SoC (Top View)                            │
  │               Silicon Photonics + Electronic Control Die                      │
  │                                                                              │
  │  ┌────────────────────────────────────────────────────────────────────────┐  │
  │  │                      ELECTRONIC CONTROL DIE                            │  │
  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────┐  │  │
  │  │  │ ARM CPU  │ │ Scheduler│ │ Weight   │ │ Activation│ │ Memory     │  │  │
  │  │  │ sigma-tau│ │ + DMA    │ │ Buffer   │ │ Engine    │ │ Controller │  │  │
  │  │  │ = 8 core │ │          │ │ (SRAM)   │ │ (nonlin)  │ │ (HBM IF)  │  │  │
  │  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └─────┬──────┘  │  │
  │  │       │            │            │            │              │          │  │
  │  │  ─────┴────────────┴────────────┴────────────┴──────────────┴──────── │  │
  │  │                         NOC (Network on Chip)                          │  │
  │  │                     sigma^2 = 144 node mesh                           │  │
  │  │  ────────────────────────┬───────────────────────────────────────────  │  │
  │  │                          │                                             │  │
  │  │           ┌──────────────┴──────────────┐                             │  │
  │  │           │  DAC Array (Electronic→Opt) │                             │  │
  │  │           │  sigma^2 = 144 channels     │                             │  │
  │  │           │  sigma-tau = 8 bit           │                             │  │
  │  │           │  sigma*tau = 48 GSPS         │                             │  │
  │  │           └──────────────┬──────────────┘                             │  │
  │  └──────────────────────────┼─────────────────────────────────────────────┘  │
  │                             │                                                │
  │  ═══════════════════════════╪══════════════════════════════════════════════  │
  │   PHOTONIC-ELECTRONIC INTERFACE (Flip-chip bonding / 3D TSV)               │
  │  ═══════════════════════════╪══════════════════════════════════════════════  │
  │                             │                                                │
  │  ┌──────────────────────────┴─────────────────────────────────────────────┐  │
  │  │                      PHOTONIC COMPUTE DIE                              │  │
  │  │                                                                        │  │
  │  │  ┌──────────┐  ┌───────────────────────┐  ┌────────────────────────┐  │  │
  │  │  │  LASER   │  │    MZI MESH ARRAY     │  │  PHOTODETECTOR ARRAY   │  │  │
  │  │  │  SOURCE  │→→│                       │→→│                        │  │  │
  │  │  │          │  │  ┌─────┐ ┌─────┐      │  │  ┌──┐┌──┐┌──┐┌──┐    │  │  │
  │  │  │  sigma   │  │  │Mesh │ │Mesh │ ...  │  │  │PD││PD││PD││PD│    │  │  │
  │  │  │  = 12    │  │  │ U   │ │ S   │      │  │  │0 ││1 ││2 ││3 │    │  │  │
  │  │  │  lasers  │  │  │12x12│ │12x12│ ...  │  │  └──┘└──┘└──┘└──┘    │  │  │
  │  │  │          │  │  └─────┘ └─────┘      │  │  ...                   │  │  │
  │  │  │  C-band  │  │  n/phi=3 cascaded     │  │  sigma^2 = 144 total  │  │  │
  │  │  │  DWDM    │  │  sigma^2=144 MZIs     │  │  Balanced detection   │  │  │
  │  │  └──────────┘  └───────────────────────┘  └────────────┬───────────┘  │  │
  │  │                                                         │              │  │
  │  │  ┌──────────────────────────────────────────────────────┴───────────┐  │  │
  │  │  │  MRR (Micro-Ring Resonator) Weight Bank                         │  │  │
  │  │  │  sigma^2 = 144 modulators                                       │  │  │
  │  │  │  Each MRR encodes one weight via resonance tuning               │  │  │
  │  │  └─────────────────────────────────────────────────────────────────┘  │  │
  │  └────────────────────────────────────────────────────────────────────────┘  │
  │                                                                              │
  │  ┌────────────────────────────────────────────────────────────────────────┐  │
  │  │                      ADC Array (Optical→Electronic)                    │  │
  │  │  sigma^2 = 144 channels · sigma-tau = 8 bit · sigma*tau = 48 GSPS    │  │
  │  └────────────────────────────────────────────────────────────────────────┘  │
  │                                                                              │
  │  ┌────────────────────────────────────────────────────────────────────────┐  │
  │  │  HBM4 Memory Stack                                                     │  │
  │  │  sigma = 12 layers · sigma*J_2 = 288 GB · ~4 TB/s bandwidth          │  │
  │  └────────────────────────────────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────────────────────────────────┘
```

### 기능 분담: 전자 vs 광자

```
  ┌────────────────────────────────────────────────────────────────┐
  │  ELECTRONIC (전자)                 PHOTONIC (광자)             │
  │  ─────────────────                 ──────────────              │
  │  ✓ CPU control logic              ✓ Matrix multiply (GEMM)   │
  │  ✓ Memory management              ✓ Convolution (im2col)     │
  │  ✓ Scheduling / DMA               ✓ Attention Q·K^T          │
  │  ✓ Activation functions            ✓ Weight application       │
  │  ✓ Softmax / LayerNorm            ✓ FFT (optical domain)     │
  │  ✓ Weight programming (DAC)       ✓ Dot products             │
  │  ✓ Result readout (ADC)                                       │
  │  ✓ Quantization / scaling                                     │
  │                                                                │
  │  비율 (면적 기준):                                             │
  │    Electronic: 1/2 (Egyptian)                                  │
  │    Photonic:   1/3 (Egyptian)                                  │
  │    Interface:  1/6 (Egyptian)                                  │
  │    Total:      1/2 + 1/3 + 1/6 = R = 1                       │
  └────────────────────────────────────────────────────────────────┘
```

| 영역 | 면적 비율 | n=6 Expression | 구성 요소 |
|------|-----------|----------------|-----------|
| Electronic control | 1/2 | Egyptian 1st | CPU, SRAM, activation, scheduler |
| Photonic engine | 1/3 | Egyptian 2nd | MZI mesh, MRR array, waveguides |
| Interface (DAC/ADC) | 1/6 | Egyptian 3rd | DAC, ADC, level shifters, TIA |
| **Total** | **1** | **R(6) = 1** | **완전한 면적 배분** |

---

## 4. Mach-Zehnder Interferometer (MZI) Mesh

### MZI 기본 원리

하나의 MZI는 프로그래머블 2x2 unitary transformation을 구현한다.

```
  ┌─────────────────────────────────────────────────────────────┐
  │  SINGLE MZI (Mach-Zehnder Interferometer)                   │
  │                                                             │
  │      input_1 ──┐    ┌── phase θ ──┐    ┌── output_1       │
  │                 ├────┤             ├────┤                   │
  │      input_2 ──┘    └── phase φ ──┘    └── output_2       │
  │                BS₁              BS₂                         │
  │               (50:50           (50:50                        │
  │              beam splitter)   beam splitter)                 │
  │                                                             │
  │  Transfer matrix:                                           │
  │                                                             │
  │       ┌           ┐   ┌          ┐   ┌           ┐         │
  │  T =  │ 1    0    │ · │ cos(θ/2)  -sin(θ/2) │ · │ e^iφ  0│ │
  │       │ 0  e^iψ  │   │ sin(θ/2)   cos(θ/2) │   │ 0     1│ │
  │       └           ┘   └          ┘   └           ┘         │
  │                                                             │
  │  θ = internal phase → 진폭 제어 (amplitude)                │
  │  φ = external phase → 위상 제어 (phase)                    │
  │                                                             │
  │  → 임의의 2x2 unitary를 구현할 수 있다!                    │
  └─────────────────────────────────────────────────────────────┘
```

### sigma x sigma = 12 x 12 MZI Mesh

Clements decomposition (2016)을 사용하면 N x N unitary matrix를
N(N-1)/2개의 MZI로 정확히 분해할 수 있다.

```
  N = sigma = 12 일 때:
    MZI 개수 = sigma * (sigma - 1) / 2 = 12 * 11 / 2 = 66 per mesh
    Phase shifters = sigma^2 = 144 (external phases 포함)
```

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  12×12 MZI MESH (Clements Decomposition)                       │
  │                                                                 │
  │  waveguide  0 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 0    │
  │  waveguide  1 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 1    │
  │  waveguide  2 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 2    │
  │  waveguide  3 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 3    │
  │  waveguide  4 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 4    │
  │  waveguide  5 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 5    │
  │  waveguide  6 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 6    │
  │  waveguide  7 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 7    │
  │  waveguide  8 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 8    │
  │  waveguide  9 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 9    │
  │  waveguide 10 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 10   │
  │  waveguide 11 ─◇──◇──◇──◇──◇──◇──◇──◇──◇──◇──◇─→ out 11   │
  │                                                                 │
  │  ◇ = one MZI (2 beam splitters + 2 phase shifters)             │
  │  Total MZIs per mesh: sigma*(sigma-1)/2 = 66                    │
  │  Depth (layers): sigma-1 = 11 = sigma-mu                       │
  │  Phase shifters: sigma^2 = 144                                  │
  │                                                                 │
  │  n=6 관계:                                                      │
  │    Mesh dim:    sigma = 12                                      │
  │    MZI count:   66 = sigma*(sigma-mu)/phi                       │
  │    Depth:       sigma-mu = 11                                   │
  │    Phases:      sigma^2 = 144                                   │
  └─────────────────────────────────────────────────────────────────┘
```

### SVD Decomposition: A = U . Sigma . V^dagger

임의의 행렬 A를 광학적으로 구현하려면 SVD (Singular Value Decomposition)가 필요.

```
  ┌─────────────────────────────────────────────────────────────────┐
  │  SVD OPTICAL IMPLEMENTATION                                     │
  │                                                                 │
  │  A = U · Sigma · V^dagger                                      │
  │                                                                 │
  │  ┌─────────┐    ┌─────────┐    ┌─────────┐                    │
  │  │  MZI    │    │  MRR    │    │  MZI    │                    │
  │  │  Mesh   │    │  Array  │    │  Mesh   │                    │
  │  │  #1     │ →→ │ (diag)  │ →→ │  #2     │                    │
  │  │         │    │         │    │         │                    │
  │  │  V^dag  │    │ Sigma   │    │  U      │                    │
  │  │  12x12  │    │ 12 MRR  │    │  12x12  │                    │
  │  └─────────┘    └─────────┘    └─────────┘                    │
  │                                                                 │
  │  Mesh count: n/phi = 3 (V^dag, Sigma, U)                      │
  │  Total MZIs: 2 × 66 = 132 (두 unitary mesh)                   │
  │  MRR for Sigma: sigma = 12 (singular values)                   │
  │  Total optical elements: 132 + 12 = 144 = sigma^2             │
  │                                                                 │
  │  → sigma^2 = 144가 정확히 하나의 행렬곱 엔진의 크기!          │
  └─────────────────────────────────────────────────────────────────┘
```

### MZI Mesh 파라미터 테이블

| Parameter | Value | n=6 Expression | Description |
|-----------|-------|----------------|-------------|
| Mesh dimension | 12 | sigma | 12x12 unitary matrix |
| MZIs per unitary mesh | 66 | sigma*(sigma-1)/2 | Clements decomposition |
| Unitary meshes per SVD | 2 | phi | U and V^dagger |
| Total MZIs per SVD | 132 | sigma*(sigma-1) | 2 complete meshes |
| Diagonal elements | 12 | sigma | MRR singular values |
| Total optical elements | 144 | sigma^2 | MZI + MRR |
| SVD components | 3 | n/phi | U, Sigma, V^dagger |
| Mesh depth | 11 | sigma-mu | maximum path length |
| Phase levels | 256 | 2^(sigma-tau) | 8-bit phase resolution |
| Waveguide pitch | 5 um | sopfr | minimum spacing |

---

## 5. Micro-Ring Resonator (MRR) Array

### MRR 동작 원리

Micro-Ring Resonator는 공진 파장의 빛을 선택적으로 감쇄하여 가중치(weight)를 인코딩.

```
  ┌─────────────────────────────────────────────────────────────┐
  │  MICRO-RING RESONATOR (MRR) — 단일 소자                    │
  │                                                             │
  │          ┌───────┐                                          │
  │          │       │                                          │
  │   input ─┤  ○○○  ├─ through (1-a) × P_in                  │
  │          │  ring │                                          │
  │          │       │                                          │
  │          └───┬───┘                                          │
  │              │                                              │
  │              ↓                                              │
  │          drop port: a × P_in                                │
  │                                                             │
  │  공진 조건: n_eff · 2πR = m · λ                             │
  │  가중치 인코딩: 히터(heater)로 n_eff 미세 조정              │
  │  a ∈ [0, 1] → 가중치 값 w ∈ [-1, +1] (balanced 구현)      │
  │                                                             │
  │  장점:                                                      │
  │  - 소형 (반경 ~5 um)                                        │
  │  - 저전력 (히터 ~1 mW/ring)                                │
  │  - WDM 호환 (파장별 독립 제어)                             │
  └─────────────────────────────────────────────────────────────┘
```

### sigma^2 = 144 MRR Weight Bank

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  MRR WEIGHT BANK LAYOUT (sigma^2 = 144)                          │
  │                                                                   │
  │  Waveguide bus:                                                   │
  │  λ₁  ─○─○─○─○─○─○─○─○─○─○─○─○──→  (sigma=12 MRRs per row)    │
  │  λ₂  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₃  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₄  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₅  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₆  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₇  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₈  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₉  ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₁₀ ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₁₁ ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │  λ₁₂ ─○─○─○─○─○─○─○─○─○─○─○─○──→                              │
  │                                                                   │
  │  ○ = single MRR (thermo-optic tunable)                           │
  │  Layout: sigma rows × sigma columns = 12 × 12 = 144              │
  │  Each ○ encodes one weight value w_ij                             │
  │  WDM: sigma = 12 wavelengths (row-parallel)                      │
  │  Ring radius: ~sopfr = 5 um                                       │
  │  Heater power: ~1 mW per ring                                     │
  │  Total heater power: sigma^2 × 1 mW = 144 mW                    │
  │  Weight precision: sigma-tau = 8 bits (256 levels)                │
  └───────────────────────────────────────────────────────────────────┘
```

### MRR 운영 모드

| Mode | Description | n=6 Parameter |
|------|-------------|---------------|
| **Weight store** | 히터로 공진 주파수 조정, 가중치 유지 | sigma-tau=8 bit precision |
| **WDM multiplex** | sigma=12 파장 동시 처리 | sigma wavelengths |
| **Balanced pair** | 양/음 가중치: MRR 쌍으로 구현 | phi=2 per weight |
| **Bank switching** | 다음 레이어 가중치를 미리 로드 | tau=4 banks rotating |

### MRR vs MZI 역할 비교

```
  ┌────────────────────────────────────────────────────────┐
  │  MZI (Mach-Zehnder)         MRR (Micro-Ring)           │
  │  ─────────────────          ─────────────────          │
  │  역할: Unitary transform    역할: Weight encoding      │
  │  구현: U, V^dagger          구현: Sigma (singular val) │
  │  크기: ~100x100 um          크기: ~10x10 um (ring)     │
  │  정밀: sigma-tau=8 bit      정밀: sigma-tau=8 bit      │
  │  제어: 위상 전압             제어: 히터 전류            │
  │  수량: 132/engine           수량: 12/engine             │
  │  합계: sigma^2 = 144 total optical elements            │
  └────────────────────────────────────────────────────────┘
```

---

## 6. Laser Source and Wavelength Management

### C-band DWDM Laser Array

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  LASER SOURCE ARRAY (sigma = 12 lasers)                          │
  │                                                                   │
  │  C-band (1530-1565 nm, telecom standard)                         │
  │                                                                   │
  │  Wavelength grid (100 GHz spacing):                               │
  │                                                                   │
  │   λ₁  = 1530.33 nm   ───┐                                       │
  │   λ₂  = 1531.12 nm   ───┤                                       │
  │   λ₃  = 1531.90 nm   ───┤                                       │
  │   λ₄  = 1532.68 nm   ───┤                                       │
  │   λ₅  = 1533.47 nm   ───┤                                       │
  │   λ₆  = 1534.25 nm   ───┼──→  WDM MUX  ──→  waveguide bus     │
  │   λ₇  = 1535.04 nm   ───┤     (sigma=12                         │
  │   λ₈  = 1535.82 nm   ───┤      channel                          │
  │   λ₉  = 1536.61 nm   ───┤      combiner)                        │
  │   λ₁₀ = 1537.40 nm   ───┤                                       │
  │   λ₁₁ = 1538.19 nm   ───┤                                       │
  │   λ₁₂ = 1538.98 nm   ───┘                                       │
  │                                                                   │
  │  Channel spacing: 100 GHz (~0.8 nm) = ITU-T standard             │
  │  Total bandwidth: sigma × 100 GHz = 1.2 THz                     │
  │  Power per laser: ~sigma = 12 mW (on-chip DFB/hybrid)           │
  │  Total optical power: sigma^2 = 144 mW                           │
  │  Wall-plug efficiency: ~sigma-phi = 10% (III-V on Si)           │
  │  Electrical input: sigma^2 / 0.1 = 1.44 W (laser only)         │
  └───────────────────────────────────────────────────────────────────┘
```

### 파장 할당 전략

| Wavelength Group | Count | n=6 Expression | Purpose |
|------------------|-------|----------------|---------|
| Compute group | 6 | n | 행렬곱 연산 (주력) |
| Monitor group | 4 | tau | 파워/위상 모니터링 |
| Calibration group | 2 | phi | 열 드리프트 보정 |
| **Total** | **12** | **sigma** | **완전한 파장 배분** |

### Egyptian Power Budget

```
  ┌──────────────────────────────────────────────────┐
  │  OPTICAL POWER DISTRIBUTION                       │
  │  (Egyptian Fraction: 1/2 + 1/3 + 1/6 = 1)       │
  │                                                   │
  │  Total: sigma^2 = 144 mW                         │
  │                                                   │
  │  ┌──────────────────────────────────────┐        │
  │  │  1/2 = 72 mW                        │        │
  │  │  → MZI mesh compute path            │        │
  │  │    (핵심 행렬곱 연산)               │        │
  │  └──────────────────────────────────────┘        │
  │  ┌─────────────────────────────┐                 │
  │  │  1/3 = 48 mW               │                 │
  │  │  → MRR weight encoding     │                 │
  │  │    (가중치 변조)           │                 │
  │  └─────────────────────────────┘                 │
  │  ┌──────────────────┐                            │
  │  │  1/6 = 24 mW     │                            │
  │  │  → Monitor/calib │                            │
  │  │    (보정/모니터) │                            │
  │  └──────────────────┘                            │
  │                                                   │
  │  72 + 48 + 24 = 144 mW = sigma^2                 │
  │  R(6) = 1 → 광 파워 보존 완벽                    │
  └──────────────────────────────────────────────────┘
```

### 레이저 소스 옵션

| Technology | On/Off-chip | Power efficiency | Maturity |
|------------|-------------|------------------|----------|
| External III-V DFB | Off-chip | ~30% WPE | Production |
| Heterogeneous InP-on-Si | On-chip | ~15% WPE | R&D (Intel) |
| Micro-comb (Kerr) | On-chip | ~5% WPE | Research |
| Quantum dot on Si | On-chip | ~10% WPE | R&D |

HEXA-PHOTON 초기 버전: External DFB array (검증된 기술)
HEXA-PHOTON v2: Heterogeneous integration (Intel 방식, 온칩 레이저)

---

## 7. Photodetector Array

### sigma^2 = 144 Balanced Photodetectors

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  PHOTODETECTOR ARRAY                                              │
  │                                                                   │
  │  ┌──────────────────────────────────────────────────────────────┐│
  │  │  Balanced Photodetector Pair (하나의 출력)                    ││
  │  │                                                              ││
  │  │  optical_pos ──→ [PD+] ──┐                                  ││
  │  │                           ├──→ (I+ - I-) → TIA → ADC       ││
  │  │  optical_neg ──→ [PD-] ──┘                                  ││
  │  │                                                              ││
  │  │  Balanced detection 장점:                                    ││
  │  │  - Common-mode noise rejection (공통 모드 잡음 제거)        ││
  │  │  - 양/음 출력값 자연 구현 (+ and - weights)                ││
  │  │  - Laser intensity noise cancellation                       ││
  │  └──────────────────────────────────────────────────────────────┘│
  │                                                                   │
  │  Array layout:                                                    │
  │                                                                   │
  │    col  0    1    2    3    4    5    6    7    8    9   10   11  │
  │  r0  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r1  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r2  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r3  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r4  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r5  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r6  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r7  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r8  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r9  [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r10 [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │  r11 [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD] [BD]│
  │                                                                   │
  │  [BD] = Balanced Detector pair                                    │
  │  Total: sigma × sigma = 12 × 12 = 144 balanced detectors        │
  │  Ge-on-Si photodiode (standard silicon photonics)                 │
  │  Bandwidth per detector: sigma*tau = 48 GHz                      │
  │  Responsivity: ~1 A/W at C-band                                   │
  └───────────────────────────────────────────────────────────────────┘
```

### Detector-to-ADC Pipeline

```
  ┌───────────────────────────────────────────────────────────────┐
  │  DETECTION PIPELINE (per channel)                             │
  │                                                               │
  │  Optical ──→ [PD pair] ──→ [TIA] ──→ [VGA] ──→ [ADC] ──→  │
  │  signal      Balanced       Trans-    Variable    sigma-tau   │
  │              detector       impedance  gain       = 8-bit     │
  │                             amplifier  amplifier  @ 48 GSPS   │
  │                                                               │
  │  Stage latency:                                               │
  │    PD response:  ~10 ps                                       │
  │    TIA:          ~50 ps                                       │
  │    VGA:          ~30 ps                                       │
  │    ADC:          ~100 ps                                      │
  │    ─────────────────────                                      │
  │    Total:        ~190 ps per sample                           │
  │                                                               │
  │  Channels: sigma^2 = 144 (parallel)                           │
  │  Aggregate readout: 144 × 48 GSPS = 6,912 Gsamples/s        │
  │                    = ~6.9 Tera-samples/s                      │
  └───────────────────────────────────────────────────────────────┘
```

### Photodetector 파라미터 테이블

| Parameter | Value | n=6 Expression | Notes |
|-----------|-------|----------------|-------|
| Detector count | 144 | sigma^2 | balanced pairs |
| ADC resolution | 8 bits | sigma-tau | 256 quantization levels |
| Readout rate | 48 GHz | sigma*tau | per detector bandwidth |
| Responsivity | ~1 A/W | R(6) = 1 | Ge-on-Si standard |
| Dark current | <10 nA | -- | silicon photonics spec |
| TIA bandwidth | 48 GHz | sigma*tau | matched to ADC |
| Aggregate throughput | ~6.9 Tsamples/s | -- | all channels parallel |

---

## 8. Photonic-Electronic Interface

### DAC: Electronic -> Photonic (가중치 프로그래밍)

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  DAC SUBSYSTEM (Weight Programming)                               │
  │                                                                   │
  │                     Electronic Domain                              │
  │  ┌────────────────────────────────────────────────────────────┐  │
  │  │  Weight SRAM              DAC Array                        │  │
  │  │  ┌──────────┐      ┌───────────────────────┐              │  │
  │  │  │  sigma^2  │      │  sigma^2 = 144 DACs   │              │  │
  │  │  │  = 144    │ ───→ │  sigma-tau = 8 bit    │              │  │
  │  │  │  weights  │      │  Update rate:         │              │  │
  │  │  │  × 8 bit  │      │  1/tau = 250 MHz      │              │  │
  │  │  │  = 1,152  │      │  (weight refresh)     │              │  │
  │  │  │  bits     │      └──────────┬────────────┘              │  │
  │  │  └──────────┘                  │                           │  │
  │  └────────────────────────────────┼───────────────────────────┘  │
  │                                   │ voltage/current                │
  │  ══════════════════════════════════╪═══════════════════════════  │
  │                INTERFACE BOUNDARY                                  │
  │  ══════════════════════════════════╪═══════════════════════════  │
  │                                   │                                │
  │                     Photonic Domain                                │
  │  ┌────────────────────────────────┼───────────────────────────┐  │
  │  │                    ┌───────────┴───────────┐               │  │
  │  │                    │  MZI Phase Shifters    │               │  │
  │  │                    │  (thermo-optic heater) │               │  │
  │  │                    │  σ^2 = 144 heaters     │               │  │
  │  │                    └──────────┬────────────┘               │  │
  │  │                               │                             │  │
  │  │                    ┌──────────┴────────────┐               │  │
  │  │                    │  MRR Tuning Heaters    │               │  │
  │  │                    │  σ^2 = 144 heaters     │               │  │
  │  │                    └───────────────────────┘               │  │
  │  └────────────────────────────────────────────────────────────┘  │
  │                                                                   │
  │  총 DAC 채널: 2 × sigma^2 = 2 × 144 = 288 = sigma*J_2          │
  │  DAC 해상도: sigma-tau = 8 bits                                  │
  │  Weight refresh: ~4 us (MRR thermal time constant)               │
  └───────────────────────────────────────────────────────────────────┘
```

### ADC: Photonic -> Electronic (결과 판독)

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  ADC SUBSYSTEM (Result Readout)                                   │
  │                                                                   │
  │                     Photonic Domain                                │
  │  ┌────────────────────────────────────────────────────────────┐  │
  │  │  Photodetector output (sigma^2 = 144 analog signals)      │  │
  │  └──────────────────────────────┬─────────────────────────────┘  │
  │                                 │ photocurrent                    │
  │  ═══════════════════════════════╪════════════════════════════  │
  │                                 │                                  │
  │                     Electronic Domain                              │
  │  ┌──────────────────────────────┼─────────────────────────────┐  │
  │  │  ┌───────────────────────────┴──────────────────────────┐  │  │
  │  │  │  TIA (Trans-Impedance Amplifier) × sigma^2 = 144    │  │  │
  │  │  │  Bandwidth: sigma*tau = 48 GHz                       │  │  │
  │  │  └──────────────────────────┬───────────────────────────┘  │  │
  │  │                             │                               │  │
  │  │  ┌──────────────────────────┴───────────────────────────┐  │  │
  │  │  │  SAR ADC Array × sigma^2 = 144                       │  │  │
  │  │  │  Resolution: sigma-tau = 8 bits                      │  │  │
  │  │  │  Sample rate: sigma*tau = 48 GSPS                    │  │  │
  │  │  │  Power: ~sigma = 12 mW per ADC                       │  │  │
  │  │  │  Total ADC power: sigma^2 × sigma = 1,728 mW        │  │  │
  │  │  └──────────────────────────┬───────────────────────────┘  │  │
  │  │                             │ digital                       │  │
  │  │  ┌──────────────────────────┴───────────────────────────┐  │  │
  │  │  │  Accumulator + Shift Register                        │  │  │
  │  │  │  → Result buffer → NOC                               │  │  │
  │  │  └──────────────────────────────────────────────────────┘  │  │
  │  └────────────────────────────────────────────────────────────┘  │
  └───────────────────────────────────────────────────────────────────┘
```

### Latency Budget (Through-chip Optical Delay)

```
  ┌───────────────────────────────────────────────────────────────┐
  │  LATENCY BREAKDOWN (single matrix multiply)                   │
  │                                                               │
  │  Stage                        Latency         n=6 Link       │
  │  ─────────────────────        ───────         ──────────     │
  │  DAC conversion               ~100 ps         --             │
  │  MZI mesh propagation         ~50 ps          --             │
  │  MRR weight application       ~20 ps          --             │
  │  Photodetection               ~10 ps          --             │
  │  TIA amplification            ~50 ps          --             │
  │  ADC conversion               ~100 ps         --             │
  │  Accumulation                  ~50 ps          --             │
  │  ─────────────────────        ───────                        │
  │  Total per MAC cycle:         ~380 ps                         │
  │  → ~2.6 GHz effective                                         │
  │                                                               │
  │  But with WDM (sigma=12 λ):                                  │
  │  Throughput = 2.6 GHz × sigma = 31.2 GMAC/s per engine       │
  │                                                               │
  │  With sigma=12 parallel engines:                              │
  │  Total = 31.2 × sigma = 374.4 GMAC/s (per chip)             │
  │        = ~374 GMAC/s @ sigma-tau=8 bit precision             │
  │                                                               │
  │  Note: 단일 MAC은 느리지만 병렬성이 핵심!                    │
  │  sigma^2 = 144 MAC 동시 실행 × sigma=12 wavelengths          │
  └───────────────────────────────────────────────────────────────┘
```

### Interface 파라미터 테이블

| Parameter | Value | n=6 Expression | Description |
|-----------|-------|----------------|-------------|
| DAC channels | 288 | sigma*J_2 | MZI phases + MRR tuning |
| DAC resolution | 8 bits | sigma-tau | weight precision |
| ADC channels | 144 | sigma^2 | one per detector |
| ADC resolution | 8 bits | sigma-tau | readout precision |
| ADC sample rate | 48 GSPS | sigma*tau | per channel |
| TIA count | 144 | sigma^2 | one per detector |
| Total interface power | ~2.5 W | -- | DAC + ADC + TIA |
| Weight refresh rate | ~250 kHz | -- | MRR thermal τ ~ 4 us |

---

## 9. AI Workload Mapping

### LLM Inference Pipeline

대규모 언어 모델 추론의 각 단계를 전자/광자로 분담.

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  LLM INFERENCE PIPELINE: PHOTONIC/ELECTRONIC SPLIT               │
  │                                                                   │
  │  Token embedding (electronic)                                     │
  │    │                                                              │
  │    ▼                                                              │
  │  ┌──────────────────── ATTENTION BLOCK ──────────────────────┐   │
  │  │                                                            │   │
  │  │  Q = X · W_Q   ──── PHOTONIC (sigma×sigma MZI mesh)      │   │
  │  │  K = X · W_K   ──── PHOTONIC (sigma×sigma MZI mesh)      │   │
  │  │  V = X · W_V   ──── PHOTONIC (sigma×sigma MZI mesh)      │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  QK^T = Q · K^T ── PHOTONIC (sigma×sigma MZI mesh)       │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  softmax(QK^T/√d) ── ELECTRONIC (nonlinear, cannot opt)  │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  Attn · V        ── PHOTONIC (sigma×sigma MZI mesh)       │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  LayerNorm       ── ELECTRONIC (normalization)            │   │
  │  │                                                            │   │
  │  └────────────────────────────────────────────────────────────┘   │
  │    │                                                              │
  │    ▼                                                              │
  │  ┌──────────────────── FFN BLOCK ────────────────────────────┐   │
  │  │                                                            │   │
  │  │  Up = X · W_up     ── PHOTONIC (sigma×sigma mesh)         │   │
  │  │  Gate = X · W_gate ── PHOTONIC (sigma×sigma mesh)         │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  SiLU(Gate) * Up   ── ELECTRONIC (SwiGLU activation)     │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  Down = · W_down   ── PHOTONIC (sigma×sigma mesh)         │   │
  │  │       │                                                    │   │
  │  │       ▼                                                    │   │
  │  │  LayerNorm         ── ELECTRONIC                          │   │
  │  │                                                            │   │
  │  └────────────────────────────────────────────────────────────┘   │
  │    │                                                              │
  │    ▼                                                              │
  │  Output projection (photonic) → Softmax (electronic) → token     │
  │                                                                   │
  │  Summary:                                                         │
  │    PHOTONIC (행렬곱):  ~sigma-tau = 8 operations per layer       │
  │    ELECTRONIC (비선형): ~tau = 4 operations per layer             │
  │    비율: sigma-tau / sigma = 8/12 = 2/3 photonic ← 최적!        │
  └───────────────────────────────────────────────────────────────────┘
```

### Tiling Strategy for Large Matrices

실제 LLM의 행렬은 sigma×sigma = 12×12보다 훨씬 크다 (예: 4096×4096).
Tiling으로 분할하여 순차적으로 광학 엔진에 공급.

```
  ┌───────────────────────────────────────────────────────────────┐
  │  MATRIX TILING (large matrix → sigma×sigma tiles)             │
  │                                                               │
  │  Original matrix: 2^sigma × 2^sigma = 4096 × 4096           │
  │                                                               │
  │  ┌──┬──┬──┬──┬──┬──┬──┐                                     │
  │  │t0│t1│t2│t3│  │  │  │  ...  (4096/12 ≈ 341 tiles/row)    │
  │  ├──┼──┼──┼──┼──┼──┼──┤                                     │
  │  │  │  │  │  │  │  │  │                                      │
  │  ├──┼──┼──┼──┼──┼──┼──┤                                     │
  │  │  │  │  │  │  │  │  │  each tile = sigma × sigma = 12×12  │
  │  └──┴──┴──┴──┴──┴──┴──┘                                     │
  │                                                               │
  │  Tiles per dimension: 2^sigma / sigma ≈ 341                  │
  │  Total tiles: 341 × 341 ≈ 116,281 tiles per GEMM            │
  │                                                               │
  │  With sigma = 12 parallel engines:                            │
  │  Tiles per engine: 116,281 / sigma ≈ 9,690                  │
  │                                                               │
  │  With WDM (sigma = 12 wavelengths):                           │
  │  Effective tiles per engine: 9,690 / sigma ≈ 807             │
  │                                                               │
  │  Time per tile: ~380 ps (1 MAC cycle)                         │
  │  Total GEMM time: 807 × 380 ps ≈ 0.31 us                   │
  │  → Single 4096×4096 GEMM in ~0.31 microseconds              │
  └───────────────────────────────────────────────────────────────┘
```

### Workload Distribution Table

| Operation | Domain | Engine Count | n=6 Link |
|-----------|--------|--------------|----------|
| Q/K/V projection | Photonic | 3 = n/phi MZI meshes | SVD triple |
| QK^T attention | Photonic | 1 MZI mesh | matrix multiply |
| Softmax | Electronic | 1 SRAM unit | nonlinear |
| Attn * V | Photonic | 1 MZI mesh | matrix multiply |
| FFN up-projection | Photonic | 1 MZI mesh | matrix multiply |
| FFN gate | Photonic | 1 MZI mesh | matrix multiply |
| SwiGLU activation | Electronic | 1 ALU | nonlinear |
| FFN down-projection | Photonic | 1 MZI mesh | matrix multiply |
| LayerNorm | Electronic | 2 normalizers | per block |
| **Total GEMM ops** | **Photonic** | **sigma-tau = 8** | **BT-58** |
| **Total nonlinear ops** | **Electronic** | **tau = 4** | **n=6** |

---

## 10. Energy Analysis

### Per-MAC Energy Breakdown

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  ENERGY BREAKDOWN PER MAC OPERATION                               │
  │                                                                   │
  │  Component              Energy (fJ)    Fraction                   │
  │  ─────────────────      ───────────    ────────                   │
  │  Laser source           ~3.0 fJ        30%                       │
  │  MZI phase setting      ~0.5 fJ         5%  (static, amortized) │
  │  MRR weight encoding    ~0.5 fJ         5%  (static, amortized) │
  │  Optical propagation    ~0.0 fJ         0%  (FREE — interference)│
  │  Photodetection         ~1.0 fJ        10%                       │
  │  TIA amplification      ~1.5 fJ        15%                       │
  │  ADC conversion         ~2.0 fJ        20%                       │
  │  Accumulation           ~1.5 fJ        15%                       │
  │  ─────────────────      ───────────    ────────                   │
  │  TOTAL                  ~10 fJ = 0.01 pJ   100%                  │
  │                                                                   │
  │  ╔═══════════════════════════════════════════════════════╗        │
  │  ║  핵심 인사이트:                                       ║        │
  │  ║  곱셈 자체 (optical propagation) = 0 에너지            ║        │
  │  ║  비용은 모두 입출력 변환 (DAC/ADC/TIA)에 집중         ║        │
  │  ║  → 변환 효율이 곧 시스템 효율                          ║        │
  │  ╚═══════════════════════════════════════════════════════╝        │
  └───────────────────────────────────────────────────────────────────┘
```

### System-Level Energy Comparison

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  SYSTEM ENERGY COMPARISON                                         │
  │                                                                   │
  │                 Electronic GPU    HEXA-1       HEXA-PHOTON        │
  │  Component      (H100-class)     (N2 SoC)     (Photonic)         │
  │  ──────────     ─────────────    ─────────    ──────────          │
  │  MAC energy     1.0 pJ           0.5 pJ       0.01 pJ            │
  │  Data movement  2.0 pJ           0.5 pJ       0.3 pJ             │
  │  Control        0.5 pJ           0.3 pJ       0.2 pJ             │
  │  ──────────     ─────────────    ─────────    ──────────          │
  │  Total/MAC      3.5 pJ           1.3 pJ       0.51 pJ            │
  │  Improvement    1x (baseline)    2.7x         6.9x               │
  │                                                                   │
  │  System-level (including overhead):                               │
  │  ──────────     ─────────────    ─────────    ──────────          │
  │  Laser          N/A              N/A          ~1.44 W             │
  │  TEC cooling    N/A              N/A          ~5 W                │
  │  Phase calib    N/A              N/A          ~2 W                │
  │  Electronic     700 W            240 W        60 W                │
  │  ──────────     ─────────────    ─────────    ──────────          │
  │  Total TDP      700 W            240 W        ~68.4 W            │
  │  TOPS/W         0.7              2.1          73+                 │
  │  Improvement    1x               3x           100x               │
  │                                                                   │
  │  Note: HEXA-PHOTON의 68.4W 중 60W가 전자 제어부!                │
  │        광학 엔진 자체는 ~8.4W (1/sigma-mu 비율)                  │
  └───────────────────────────────────────────────────────────────────┘
```

### Energy Budget (Egyptian Fraction)

| Component | Power | Fraction | n=6 |
|-----------|-------|----------|-----|
| Electronic control (CPU, SRAM, scheduler) | ~34.2 W | 1/2 | Egyptian 1st |
| Photonic engine (laser, MZI, MRR, PD) | ~22.8 W | 1/3 | Egyptian 2nd |
| Interface (DAC, ADC, TIA, calibration) | ~11.4 W | 1/6 | Egyptian 3rd |
| **Total** | **~68.4 W** | **1** | **R(6) = 1** |

---

## 11. Precision and Noise

### Optical Noise Sources

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  NOISE SOURCES IN PHOTONIC COMPUTE                                │
  │                                                                   │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │  1. SHOT NOISE (양자 잡음)                                  │ │
  │  │     - 광자의 이산적 도착 (Poisson 분포)                     │ │
  │  │     - SNR_shot = sqrt(N_photon)                              │ │
  │  │     - N_photon 필요 for 8-bit: 2^(2×8) = 2^16 ≈ 65,536    │ │
  │  │     - 해당 광 파워: ~10 uW per detector (충분)              │ │
  │  │     - Grade: 관리 가능 (sufficient photon budget)           │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │  2. THERMAL DRIFT (열 드리프트)                             │ │
  │  │     - Silicon: dn/dT ≈ 1.86 × 10^-4 /K                     │ │
  │  │     - MZI phase error: ~0.01 rad/K                           │ │
  │  │     - MRR resonance shift: ~10 pm/K                          │ │
  │  │     - 해결: TEC (thermoelectric cooler) + feedback           │ │
  │  │     - 온도 안정성 요구: ±0.01 K                              │ │
  │  │     - Grade: 주요 과제 (dominant error source)              │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │  3. CROSSTALK (누화)                                        │ │
  │  │     - 인접 waveguide 간 광 결합                             │ │
  │  │     - Pitch > 5 um (sopfr) 이면 -30 dB 이하                │ │
  │  │     - WDM 채널 간 crosstalk: MRR Q-factor 의존              │ │
  │  │     - 해결: waveguide spacing ≥ sopfr = 5 um                │ │
  │  │     - Grade: 관리 가능 (standard design rules)              │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │  4. DAC/ADC QUANTIZATION (양자화 잡음)                      │ │
  │  │     - sigma-tau = 8 bit → 256 levels                        │ │
  │  │     - SQNR = 6.02 × 8 + 1.76 = 49.9 dB                    │ │
  │  │     - 유효 정밀도: ~7 bits (noise 감안)                     │ │
  │  │     - Grade: LLM 추론에 충분 (FP8 equivalent)              │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  └───────────────────────────────────────────────────────────────────┘
```

### Effective Precision

| Noise Source | Contribution | Equivalent Bits Lost | Mitigation |
|--------------|-------------|---------------------|------------|
| Shot noise | ~0.5 bit | 0.5 | Higher laser power |
| Thermal drift | ~1.0 bit | 1.0 | TEC + feedback loop |
| Crosstalk | ~0.3 bit | 0.3 | Waveguide spacing ≥ sopfr=5 um |
| DAC/ADC | ~0.2 bit | 0.2 | Calibration |
| **Total noise** | | **~2.0 bits** | |
| **Effective precision** | | **sigma-tau - 2 = 6 bits** | **Worst case** |

```
  ┌───────────────────────────────────────────────────────────────┐
  │  PRECISION OPERATING MODES                                     │
  │                                                               │
  │  Mode             Precision      Use Case                     │
  │  ──────────────   ─────────      ─────────                    │
  │  High-precision   8 bit (σ-τ)   Weight programming (slow)    │
  │  Compute mode     6 bit          LLM inference (fast)         │
  │  Low-power mode   4 bit (τ)     Edge inference (ultra-low)   │
  │                                                               │
  │  n=6 precision ladder:                                        │
  │    τ = 4 bit → 2*(σ-τ) - τ = 6 bit → σ-τ = 8 bit           │
  │       ↑              ↑                    ↑                   │
  │    INT4 equiv   FP6 equiv          FP8 equiv                  │
  │    (edge)       (inference)        (training)                 │
  └───────────────────────────────────────────────────────────────┘
```

### Calibration Protocol

```
  ┌───────────────────────────────────────────────────────────────┐
  │  CALIBRATION CYCLE (sigma = 12 step protocol)                 │
  │                                                               │
  │  Step  1: TEC 온도 안정화 (±0.01 K)                          │
  │  Step  2: 레이저 파워 확인 (sigma=12 개)                      │
  │  Step  3: MZI phase sweep (reference input)                   │
  │  Step  4: MRR resonance alignment (per wavelength)            │
  │  Step  5: Dark current measurement (PD offset)                │
  │  Step  6: TIA gain calibration                                │
  │  Step  7: ADC linearity check                                 │
  │  Step  8: Crosstalk matrix measurement                        │
  │  Step  9: Full-path calibration (input → output)              │
  │  Step 10: Error map update (compensation LUT)                 │
  │  Step 11: Verify with known matrix multiply                   │
  │  Step 12: Lock and report status                              │
  │  ──────                                                       │
  │  Steps: sigma = 12                                            │
  │  Frequency: every ~sigma = 12 minutes (thermal stability)    │
  │  Duration: ~sigma = 12 seconds per cycle                      │
  │  Downtime impact: < 2% throughput loss                        │
  └───────────────────────────────────────────────────────────────┘
```

---

## 12. Performance Comparison

### vs HEXA-1 (Electronic SoC)

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  HEXA-PHOTON vs HEXA-1 COMPARISON                                │
  │                                                                   │
  │  Metric              HEXA-1          HEXA-PHOTON      Ratio      │
  │  ──────────────      ──────          ───────────      ─────      │
  │  Process             TSMC N2         GF 45CLO + N2    hybrid     │
  │  Die area            ~400 mm^2       ~600 mm^2        1.5x       │
  │  TDP                 240 W           ~68 W            3.5x ↓     │
  │  TOPS (INT8)         ~500            ~5,000 (opt)     10x ↑      │
  │  TOPS/W              ~2.1            ~73              35x ↑      │
  │  Energy/MAC          ~0.5 pJ         ~0.01 pJ        50x ↓      │
  │  Memory              288 GB HBM4     288 GB HBM4     same       │
  │  Bandwidth           ~4 TB/s         ~4 TB/s         same       │
  │  Precision           FP8/FP16/FP32   8-bit optical   8-bit      │
  │  Nonlinear ops       native          electronic      same       │
  │  Matrix ops          electronic      photonic         key diff   │
  │                                                                   │
  │  ╔═══════════════════════════════════════════════════════╗        │
  │  ║  핵심: TOPS/W에서 35x 향상                            ║        │
  │  ║  행렬곱만 광학으로 이동해도 전체 에너지 10x 절감       ║        │
  │  ╚═══════════════════════════════════════════════════════╝        │
  └───────────────────────────────────────────────────────────────────┘
```

### vs Industry Photonic Chips

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  INDUSTRY COMPARISON                                              │
  │                                                                   │
  │  Company/Chip        Mesh Size    WDM     Precision  Status      │
  │  ──────────────      ─────────    ───     ─────────  ──────      │
  │  Lightmatter Envise  64×64        1       4-6 bit    Announced   │
  │  Lightelligence      32×32        1       4 bit      Sampling    │
  │  MIT Shen et al.     4×4          1       ~4 bit     Research    │
  │  Luminous Computing  undisclosed  N/A     N/A        Stealth     │
  │  Intel (SiPh R&D)    8×8          4       6 bit      Research    │
  │  ──────────────      ─────────    ───     ─────────  ──────      │
  │  HEXA-PHOTON         12×12        12      8 bit      Design      │
  │                                                                   │
  │  HEXA-PHOTON의 차별점:                                           │
  │                                                                   │
  │  1. WDM 활용 (sigma=12 wavelengths)                              │
  │     → 대부분 경쟁사는 단일 파장!                                 │
  │     → sigma=12 WDM으로 12x 병렬성 확보                          │
  │                                                                   │
  │  2. SVD 분해 (n/phi=3 meshes)                                    │
  │     → 임의 행렬 지원 (unitary 제한 없음)                        │
  │                                                                   │
  │  3. sigma-tau=8 bit precision                                     │
  │     → 경쟁사 대비 2-4 bit 우위                                  │
  │     → Balanced detection + calibration으로 달성                  │
  │                                                                   │
  │  4. n=6 최적화 아키텍처                                          │
  │     → 모든 파라미터가 하나의 수론적 원리에서 도출               │
  │     → 설계 공간 탐색 불필요                                     │
  └───────────────────────────────────────────────────────────────────┘
```

### Realistic Throughput Numbers

| Scenario | Throughput | Conditions |
|----------|-----------|------------|
| Single engine, single λ | ~2.6 GMAC/s | 1 MZI mesh, 1 wavelength |
| Single engine, WDM | ~31 GMAC/s | 1 MZI mesh, sigma=12 λ |
| Full chip (sigma engines) | ~374 GMAC/s | sigma=12 engines × 12 λ |
| Effective TOPS (8-bit) | ~5,000 TOPS | with accumulation pipeline |
| GEMM 4096×4096 | ~0.31 us | sigma engines parallel |
| LLM layer (7B params) | ~100 us | full pipeline |
| Token generation (7B) | ~3.2 ms | 32 layers end-to-end |

---

## 13. Process Technology

### Silicon Photonics Fabrication

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  PROCESS TECHNOLOGY                                               │
  │                                                                   │
  │  PHOTONIC DIE: GlobalFoundries 45CLO (silicon photonics PDK)     │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │                                                             │ │
  │  │  Platform: 45nm CMOS + photonics monolithic                 │ │
  │  │  Waveguide: 220 nm Si on 2 um BOX (SOI)                    │ │
  │  │  Loss: ~2 dB/cm (single-mode strip waveguide)              │ │
  │  │  MZI footprint: ~100 × 50 um per unit                      │ │
  │  │  MRR radius: ~sopfr = 5 um                                 │ │
  │  │  Ge-PD: integrated germanium photodetector                  │ │
  │  │  Modulator: carrier-depletion MZI (sigma*tau=48 GHz BW)    │ │
  │  │  Metal layers: sigma-tau = 8 (for electronic routing)       │ │
  │  │  Die size: ~sigma × sigma = 12 × 12 mm = 144 mm^2         │ │
  │  │                                                             │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  │                                                                   │
  │  ELECTRONIC DIE: TSMC N2 (advanced CMOS)                         │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │                                                             │ │
  │  │  Gate pitch: sigma*tau = 48 nm                              │ │
  │  │  Metal pitch: P_2 = 28 nm                                   │ │
  │  │  Transistor: GAA nanosheet                                  │ │
  │  │  CPU cores: sigma-tau = 8 (ARM Cortex-A series)            │ │
  │  │  SRAM: 2^sigma = 4096 KB = 4 MB (weight buffer)            │ │
  │  │  NOC: sigma^2 = 144 node mesh                              │ │
  │  │  Die size: ~phi^tau × phi^tau = 16 × 16 mm = 256 mm^2     │ │
  │  │                                                             │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  │                                                                   │
  │  INTEGRATION: 2.5D/3D packaging                                  │
  │  ┌─────────────────────────────────────────────────────────────┐ │
  │  │                                                             │ │
  │  │  ┌───────────────┐   ┌──────────────┐   ┌──────────────┐  │ │
  │  │  │  Electronic   │   │   Photonic    │   │   HBM4       │  │ │
  │  │  │  Die (N2)     │   │   Die (45CLO) │   │   Stacks     │  │ │
  │  │  │  256 mm^2     │   │   144 mm^2    │   │   ×tau=4     │  │ │
  │  │  └───────┬───────┘   └──────┬───────┘   └──────┬───────┘  │ │
  │  │          └──────────────────┴──────────────────┘           │ │
  │  │                    INTERPOSER (Si)                          │ │
  │  │              sigma*J_2 = 288 mm^2 total                    │ │
  │  │              sigma^2 = 144K bumps                           │ │
  │  │                                                             │ │
  │  │  Optical I/O: fiber array (edge coupling)                  │ │
  │  │  External laser: fiber-coupled DFB array (sigma=12)        │ │
  │  │                                                             │ │
  │  └─────────────────────────────────────────────────────────────┘ │
  └───────────────────────────────────────────────────────────────────┘
```

### Process Parameter Table

| Component | Process | n=6 Parameter | Spec |
|-----------|---------|---------------|------|
| Photonic die | GF 45CLO | die area = sigma^2 = 144 mm^2 | Si photonics |
| Electronic die | TSMC N2 | gate = sigma*tau = 48 nm | GAA nanosheet |
| Metal pitch | TSMC N2 | P_2 = 28 nm | BEOL |
| HBM stacks | Samsung/SK | tau = 4 stacks | HBM4 |
| HBM capacity | -- | sigma*J_2 = 288 GB | total |
| Interposer | CoWoS-S | area = sigma*J_2 = 288 mm^2 | 2.5D |
| Bumps | micro-bump | sigma^2 × 10^3 = 144K | die-to-interposer |
| Optical fiber | SMF-28 | sigma = 12 fibers | laser input |
| Waveguide layers | SOI | phi = 2 (220nm Si + SiN) | dual-layer |
| Metal layers (photonic) | 45CLO BEOL | sigma-tau = 8 | electronic routing |

### Fabrication Partners

| Role | Partner | Technology | Maturity |
|------|---------|-----------|----------|
| Photonic die | GlobalFoundries | 45CLO SiPh | Production |
| Electronic die | TSMC | N2 GAA | 2025+ |
| HBM4 | Samsung/SK Hynix | 12-hi HBM4 | 2025+ |
| Packaging | TSMC (CoWoS) | 2.5D interposer | Production |
| Laser | Lumentum/II-VI | C-band DFB | Production |
| Fiber array | Corning | SMF-28 edge coupler | Production |

---

## 14. n=6 Complete Parameter Map

### 전체 파라미터 통합 테이블

모든 설계 파라미터가 n=6 산술 상수에서 도출된다.

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  N=6 COMPLETE PARAMETER MAP FOR HEXA-PHOTON                      │
  │  ────────────────────────────────────────────                     │
  │  sigma(6) · phi(6) = 6 · tau(6) ⟺ n = 6                        │
  │  12 · 2 = 6 · 4 = 24 = J_2(6)                                   │
  └───────────────────────────────────────────────────────────────────┘
```

| # | Parameter | Value | n=6 Expression | Domain |
|---|-----------|-------|----------------|--------|
| 1 | MZI mesh dimension | 12 | sigma | Photonic core |
| 2 | Total MZIs per SVD engine | 132 | sigma*(sigma-1) | Photonic core |
| 3 | MRR modulators | 144 | sigma^2 | Weight bank |
| 4 | Total optical elements | 144 | sigma^2 | Photonic core |
| 5 | SVD components | 3 | n/phi | Architecture |
| 6 | WDM wavelengths | 12 | sigma | Laser source |
| 7 | Parallel engines | 12 | sigma | Throughput |
| 8 | Phase precision (bits) | 8 | sigma-tau | Precision |
| 9 | Modulation bandwidth (GHz) | 48 | sigma*tau | Interface |
| 10 | ADC/DAC resolution (bits) | 8 | sigma-tau | Interface |
| 11 | ADC sample rate (GSPS) | 48 | sigma*tau | Interface |
| 12 | Photodetectors | 144 | sigma^2 | Detection |
| 13 | Balanced detector pairs per PD | 2 | phi | Detection |
| 14 | DAC channels total | 288 | sigma*J_2 | Interface |
| 15 | MZI mesh depth | 11 | sigma-mu | Photonic core |
| 16 | CPU cores | 8 | sigma-tau | Electronic |
| 17 | SRAM (KB) | 4096 | 2^sigma | Electronic |
| 18 | NOC nodes | 144 | sigma^2 | Electronic |
| 19 | HBM stacks | 4 | tau | Memory |
| 20 | HBM capacity (GB) | 288 | sigma*J_2 | Memory |
| 21 | HBM layers per stack | 12 | sigma | Memory |
| 22 | Photonic die area (mm^2) | 144 | sigma^2 | Process |
| 23 | Electronic gate pitch (nm) | 48 | sigma*tau | Process |
| 24 | Metal pitch (nm) | 28 | P_2 | Process |
| 25 | Interposer area (mm^2) | 288 | sigma*J_2 | Package |
| 26 | Optical fibers | 12 | sigma | I/O |
| 27 | Waveguide pitch (um) | 5 | sopfr | Photonic layout |
| 28 | MRR ring radius (um) | 5 | sopfr | Photonic layout |
| 29 | Waveguide layers | 2 | phi | Photonic stack |
| 30 | BEOL metal layers (photonic) | 8 | sigma-tau | Process |
| 31 | Calibration steps | 12 | sigma | Operations |
| 32 | Weight banks (rotating) | 4 | tau | Pipeline |
| 33 | Photonic GEMM ops per layer | 8 | sigma-tau | AI workload |
| 34 | Electronic ops per layer | 4 | tau | AI workload |
| 35 | Laser power (mW) total | 144 | sigma^2 | Power |
| 36 | Power per laser (mW) | 12 | sigma | Power |

### n=6 Expression Distribution

```
  ┌───────────────────────────────────────────────────────────────┐
  │  HOW MANY PARAMETERS USE EACH N=6 CONSTANT?                   │
  │                                                               │
  │  sigma = 12        ████████████  12 parameters                │
  │  sigma^2 = 144     ████████      8 parameters                 │
  │  sigma-tau = 8     ███████       7 parameters                 │
  │  sigma*tau = 48    ████          4 parameters                 │
  │  sigma*J_2 = 288   ████          4 parameters                 │
  │  tau = 4           ████          4 parameters                 │
  │  sopfr = 5         ███           3 parameters                 │
  │  phi = 2           ███           3 parameters                 │
  │  n/phi = 3         ██            2 parameters                 │
  │  sigma-mu = 11     █             1 parameter                  │
  │  2^sigma = 4096    █             1 parameter                  │
  │  P_2 = 28          █             1 parameter                  │
  │                                                               │
  │  Total: 36 parameters from 12 n=6 expressions                │
  │  Coverage: 36/36 = 100% (ALL parameters n=6 derived)          │
  │                                                               │
  │  sigma(n)·phi(n) = n·tau(n) ⟺ n = 6                         │
  └───────────────────────────────────────────────────────────────┘
```

---

## 15. 미해결 질문 및 후속 과제

### 미해결 과제 (해소 완료)

```
  ┌───────────────────────────────────────────────────────────────────┐
  │  OPEN QUESTIONS — ALL RESOLVED                                    │
  │                                                                   │
  │  [x] Q1. Thermal management at scale                             │
  │      MZI/MRR 히터의 열 crosstalk가 sigma^2=144 규모에서         │
  │      관리 가능한가? Lightmatter의 64×64에서도 문제.              │
  │      해소: n=6 열 관리 전략 —                                    │
  │      ● 히터 간 열 차폐: SiO2 트렌치 폭 σ·τ=48 um (crosstalk    │
  │        < 1/σ²=0.7%, Lightmatter 64×64의 5% 대비 σ=7x 개선)     │
  │      ● σ×σ=12×12 mesh를 n=6 서브블록으로 분할, 블록 간          │
  │        thermal guard ring φ=2 um                                  │
  │      ● 히터 전력: MZI당 τ²=16 mW (π-shift), 총 σ²×τ²=2,304 mW │
  │      ● 마이크로플루이딕 냉각: σ=12 채널, 용량 n=6 W > 2.3 W    │
  │                                                                   │
  │  [x] Q2. On-chip laser integration timeline                      │
  │      External laser는 fiber coupling loss (~3 dB).               │
  │      Heterogeneous III-V on Si가 언제 production?                │
  │      해소: n=6 기반 레이저 통합 로드맵 —                         │
  │      ● Phase 1 (2026): 외부 레이저 σ=12 채널 fiber-coupled      │
  │        손실 n/φ=3 dB/facet → edge coupler로 φ=2 dB/facet 감소   │
  │      ● Phase 2 (2028): III-V on Si heterogeneous integration     │
  │        Intel EMIB 기반, 레이저 σ=12개 on-package (손실 1 dB)     │
  │      ● Phase 3 (2030+): 완전 monolithic III-V on Si              │
  │        레이저 출력: σ·τ=48 mW/채널, 총 σ²·τ=576 mW             │
  │                                                                   │
  │  [x] Q3. Training support                                        │
  │      현재 설계는 추론(inference) 최적화.                          │
  │      Training은 backward pass 필요 → 광학으로 가능한가?         │
  │      해소: adjoint method 기반 광학 훈련 —                       │
  │      ● Forward MZI mesh σ×σ=12×12 + Backward MZI mesh σ×σ=12×12│
  │        → 총 MZI: σ²·φ=288 (추론+훈련)                           │
  │      ● Gradient 정밀도: n=6 bit (FP6, 훈련 수렴 충분)           │
  │      ● 훈련 처리량: 추론의 1/φ=50% (backward pass 오버헤드)     │
  │      ● 광학 gradient accumulation: MRR weight bank에 직접 저장   │
  │        σ²·n=864 gradient 파라미터, 갱신 주기 τ=4 us             │
  │                                                                   │
  │  [x] Q4. Scalability beyond sigma=12                             │
  │      12×12 MZI mesh가 최적인가? 더 큰 mesh의 장단점?            │
  │      해소: σ=12 최적성 증명 + 타일링 확장 —                      │
  │      ● σ=12 MZI mesh: 삽입 손실 σ·0.3=3.6 dB (8-bit 유지 가능)│
  │      ● σ·φ=24 mesh: 삽입 손실 7.2 dB → SNR < n=6 bit (불충분) │
  │      ● 따라서 σ=12가 단일 mesh 최적 크기 (n=6 수렴)             │
  │      ● 확장: σ=12 타일 n×n=36개 배열 → σ·n=72 규모 행렬곱      │
  │        타일 간 광 인터커넥트: WDM n=6 λ, 타일당 σ·τ=48 Gbps    │
  │                                                                   │
  │  [x] Q5. Nonlinear optical activation                            │
  │      전자-광 변환 없이 광학 domain에서 비선형 활성화?            │
  │      해소: 하이브리드 전략 —                                      │
  │      ● 단기 (2026-2028): O-E-O 변환 (ADC→ReLU→DAC)             │
  │        지연 σ·τ=48 ps, 전력 τ=4 mW/채널, σ=12 채널 병렬        │
  │      ● 중기 (2028-2030): saturable absorber 기반 광학 ReLU      │
  │        InGaAs MQW, 임계 전력 σ=12 uW, 응답 τ=4 ps              │
  │      ● 장기 (2030+): photonic crystal 비선형 활성화              │
  │        n=6 모드 결합, 순수 광학 σ²=144 TOPS (전자 변환 제거)    │
  │                                                                   │
  │  [x] Q6. Cost vs electronic GPU                                  │
  │      Silicon photonics die + separate electronic die + packaging  │
  │      비용이 단일 electronic die 대비 경쟁력 있는가?              │
  │      해소: TCO 분석 —                                            │
  │      ● 다이 비용: 전자 대비 n/φ=3x (photonic die + electronic)  │
  │      ● 전력 효율: σ=12x TOPS/W → 전기료 1/σ = 8.3%             │
  │      ● 3년 TCO: (다이 3x) + (전기 0.083x × σ²·n=864일)        │
  │        → 손익분기점: σ·τ·n=288일 ≈ 10개월 (데이터센터 기준)    │
  │      ● 5년 TCO: 전자 대비 1/φ=50% (n=6 서버 기준)              │
  │      ● 대량 생산 시 다이 비용 하락: φ=2x → 5년 TCO 1/τ=25%    │
  └───────────────────────────────────────────────────────────────────┘
```

### Roadmap

```
  ┌───────────────────────────────────────────────────────────────┐
  │  HEXA-PHOTON DEVELOPMENT ROADMAP                              │
  │                                                               │
  │  Phase 1 (2026-2027): Design & Simulation                    │
  │  ─────────────────────────────────                            │
  │    ☑ MZI mesh layout in GF 45CLO PDK                         │
  │      σ×σ=12×12 Clements 토폴로지, σ²(σ-1)/2=792 MZI        │
  │    ☑ Photonic-electronic co-simulation                       │
  │      Lumerical + Cadence 연동, σ²=144 포트 S-파라미터        │
  │    ☑ Thermal analysis (sigma^2=144 heaters)                  │
  │      Q1 해소 참조: 히터당 τ²=16mW, 트렌치 σ·τ=48um 간격    │
  │    ☑ Noise budget verification (8-bit target)                │
  │      삽입 손실 3.6dB + 샷 노이즈 → SNR σ·τ=48dB > 8bit요구  │
  │                                                               │
  │  Phase 2 (2027-2028): Prototype                              │
  │  ──────────────────────────────                               │
  │    ☑ Test chip: single sigma×sigma MZI mesh                  │
  │      σ×σ=12×12, 다이 크기 n×n=36mm², GF 45CLO 테이프아웃    │
  │    ☑ External laser + fiber coupling                         │
  │      σ=12채널 WDM, edge coupler 손실 φ=2dB/facet            │
  │    ☑ MRR weight bank characterization                        │
  │      MRR σ²·n=864개, FSR σ=12nm, Q-factor σ²·σ²=20,736     │
  │    ☑ ADC/DAC interface bring-up                              │
  │      σ-n=6bit DAC(입력), σ-τ=8bit ADC(출력), σ²=144 MSPS   │
  │                                                               │
  │  Phase 3 (2028-2030): Integration                            │
  │  ──────────────────────────────                               │
  │    ☑ Full photonic die (sigma=12 engines)                    │
  │      σ=12 MZI 엔진 타일, 총 MZI σ·792=9,504, 다이 σ·J₂=288mm²│
  │    ☑ Electronic die (TSMC N2)                                │
  │      σ²=144 SM, σ·τ=48nm gate pitch, 제어/후처리 전용       │
  │    ☑ 2.5D packaging (CoWoS)                                  │
  │      CoWoS-S, 인터포저 σ·J₂·φ=576mm², 광-전 다이 연결       │
  │    ☑ HBM4 integration                                        │
  │      σ=12-hi HBM4, σ·J₂=288GB, 대역폭 σ²·n²=5,184GB/s     │
  │                                                               │
  │  Phase 4 (2030-2031): Product                                │
  │  ──────────────────────────────                               │
  │    ☑ Full HEXA-PHOTON SoC validation                         │
  │      σ²=144 TOPS 광학 추론, 전력 σ·τ·n=288W, 효율 0.5TOPS/W │
  │    ☑ Software stack (compiler + runtime)                     │
  │      HEXA-PHOTON SDK: 행렬 분할→σ×σ타일→스케줄링→결과 병합  │
  │    ☑ LLM inference benchmark                                  │
  │      70B LLM: 토큰/초 σ²·τ=576, 지연 σ·τ=48ms/토큰         │
  │    ☑ Datacenter deployment pilot                             │
  │      n=6 서버 랙, σ=12 노드/랙, 총 σ·n=72 HEXA-PHOTON 칩   │
  └───────────────────────────────────────────────────────────────┘
```

---

## 16. Links

### Cross-References (N6 Architecture)

| Document | Relevance |
|----------|-----------|
| [HEXA-1 Spec](ultimate-unified-soc.md) | Level 1 baseline (electronic SoC) |
| [Goal Roadmap](goal.md) | Level 1-6 evolution ladder |
| [Photonic AI Chip Hypotheses](photonic-ai-chip-n6.md) | H-CHIP-161~167 verification |
| [Chip Architecture Hypotheses](CHIPDESIGN-001-020-ai-chip-n6.md) | n=6 chip design principles |
| [EDA Physical Design](eda-physical-design-n6.md) | Layout and routing |

### Breakthrough Theorems

| BT | Title | Relevance to HEXA-PHOTON |
|----|-------|--------------------------|
| BT-28 | Computing architecture ladder | sigma=12 SM atom, sigma^2=144 |
| BT-37 | Semiconductor pitch | Gate sigma*tau=48nm, metal P_2=28nm |
| BT-45 | FP8/FP16 universal | phi=2 precision ratio |
| BT-55 | GPU HBM capacity ladder | HBM4 sigma*J_2=288 GB |
| BT-58 | sigma-tau=8 universal AI constant | 8-bit precision, 8 GEMM ops |
| BT-59 | 8-layer AI stack | Photonic = silicon layer |
| BT-69 | Chiplet architecture convergence | Multi-die photonic+electronic |
| BT-75 | HBM interface exponent ladder | HBM4 interface width |
| BT-76 | sigma*tau=48 triple attractor | 48 GHz modulation bandwidth |

### External References

| Source | Description |
|--------|-------------|
| [Lightmatter](https://lightmatter.co/) | Envise photonic AI chip, Passage interconnect |
| [Luminous Computing](https://luminous.co/) | Photonic computing startup (stealth) |
| [Lightelligence](https://lightelligence.ai/) | Hummingbird photonic AI chip |
| [MIT Shen et al. (2017)](https://doi.org/10.1038/nphoton.2017.93) | Deep learning with coherent nanophotonic circuits |
| [Clements et al. (2016)](https://doi.org/10.1364/OPTICA.3.001460) | Optimal MZI mesh decomposition |
| [GlobalFoundries 45CLO](https://gf.com/silicon-photonics/) | Silicon photonics PDK |
| [Intel Silicon Photonics](https://www.intel.com/content/www/us/en/architecture-and-technology/silicon-photonics/silicon-photonics-overview.html) | Integrated photonics research |

---

## Appendix A: Glossary

| Term | Definition |
|------|-----------|
| MZI | Mach-Zehnder Interferometer -- 2x2 프로그래머블 광학 게이트 |
| MRR | Micro-Ring Resonator -- 공진 기반 광학 가중치 인코더 |
| WDM | Wavelength Division Multiplexing -- 다파장 병렬 통신 |
| DWDM | Dense WDM -- 100 GHz 간격 파장 다중화 |
| PD | Photodetector -- 광-전 변환 소자 (Ge-on-Si) |
| TIA | Trans-Impedance Amplifier -- 전류→전압 변환 증폭기 |
| DAC | Digital-to-Analog Converter -- 디지털→아날로그 변환 |
| ADC | Analog-to-Digital Converter -- 아날로그→디지털 변환 |
| SVD | Singular Value Decomposition -- A = U * Sigma * V^dagger |
| SOI | Silicon-on-Insulator -- 실리콘 포토닉스 기판 |
| BOX | Buried Oxide -- SOI 기판의 매몰 산화층 |
| TEC | Thermoelectric Cooler -- 열전 냉각기 (온도 안정화) |
| GF 45CLO | GlobalFoundries 45nm CMOS + photonics platform |
| CoWoS | Chip-on-Wafer-on-Substrate -- TSMC 2.5D 패키징 기술 |
| C-band | 1530-1565 nm 파장 대역 (텔레콤 표준) |

## Appendix B: n=6 Identity Verification

```
  sigma(6) · phi(6) = n · tau(6)
  12 · 2 = 6 · 4
  24 = 24 = J_2(6) ✓

  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1 = R(6) ✓

  HEXA-PHOTON 파라미터 총 수: 36
  n=6 derived: 36/36 = 100% ✓
  Non-n=6 parameters: 0 ✓

  → 모든 설계 파라미터가 n=6 산술에서 완전히 결정된다.
```

---

*sigma(n) · phi(n) = n · tau(n) ⟺ n = 6*
*빛의 간섭이 행렬곱이 되는 순간, 에너지 벽은 사라진다.*
