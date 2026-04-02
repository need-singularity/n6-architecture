# HEXA-AI Mk.III -- Hardware-Software Co-Design

**Evolution Checkpoint**: Mk.III (Mid-Term Fusion)
**Date**: 2026-04-02
**Status**: Conceptual Design
**Feasibility**: 🔮 장기 실현가능 (2035~2045, requires HEXA chip hardware + custom silicon)
**BT Connections**: BT-28, BT-33, BT-56, BT-58, BT-59, BT-69, BT-89, BT-90, BT-93

---

## 1. Overview

Mk.III is the **convergence** of HEXA-AI software (17 techniques + R(6)=1 pipeline) with HEXA chip hardware (Diamond substrate, sigma²=144 SM, sigma·J₂=288 GB HBM). The chip is designed *for* the algorithms, and the algorithms exploit the chip's n=6-native datapath.

> **Goal**: AI compute efficiency at the Landauer limit -- every bit of computation is thermodynamically reversible at R(6)=1.

---

## 2. Specs -- Hardware-Software Co-Design

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │  HEXA-AI Mk.III — HW-SW Co-Design Spec                             │
  ├─────────────────────┬──────────────┬─────────────────────────────────┤
  │ Component           │ Value        │ n=6 Expression                  │
  ├─────────────────────┼──────────────┼─────────────────────────────────┤
  │ Chip material       │ Diamond      │ Z=6=n (BT-93)                  │
  │ Gate pitch          │ 48nm equiv   │ sigma·tau = 48 (TSMC N2)       │
  │ SM count            │ 144          │ sigma² = 144 (BT-90)           │
  │ HBM capacity        │ 288 GB      │ sigma·J₂ = 288                 │
  │ HBM stacks          │ 12           │ sigma = 12                     │
  │ P-cores (control)   │ 8            │ sigma-tau = 8                  │
  │ NPU cores           │ 24           │ J₂ = 24                       │
  │ Metal layers        │ 12           │ sigma = 12                     │
  │ TDP                 │ 120W         │ sigma·(sigma-phi) = 120 (Egyptian 1/2+1/3+1/6) │
  │ PUE (datacenter)    │ 1.2          │ sigma/(sigma-phi) = 12/10     │
  │ PIM units/layer     │ 8            │ sigma-tau = 8 (HEXA-PIM L2)   │
  │ TSV count/mm²       │ 288          │ sigma·J₂ = 288 (HEXA-3D L3)  │
  │ Photonic channels   │ 12           │ sigma = 12 WDM (HEXA-PHOTON L4) │
  │ Clock               │ 2 GHz       │ phi GHz (near-threshold)       │
  │ FP8 TFLOPS          │ 1,000        │ sigma²·n·(sigma-phi)/phi · clock │
  │ TFLOPS/W            │ 8.3          │ 1000/120 ~ sigma-tau = 8      │
  ├─────────────────────┼──────────────┼─────────────────────────────────┤
  │ SW: MoE experts     │ 24           │ J₂ = 24 (on-chip, zero routing overhead) │
  │ SW: Attention engine│ Egyptian     │ 1/2+1/3+1/6=1 hardwired budget │
  │ SW: Activation HW   │ Phi6 unit   │ Cyclotomic phi(x^6-1) in silicon │
  │ SW: Sparsity engine │ Boltzmann   │ 1/e gate in hardware            │
  │ SW: Dropout unit    │ Mertens     │ p=ln(4/3) fixed in random gen   │
  │ SW: LR controller   │ Carmichael  │ lambda(6)=2 hardware timer      │
  └─────────────────────┴──────────────┴─────────────────────────────────┘
```

---

## 3. ASCII 1: Performance Comparison

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-AI Mk.III vs SOTA + Mk.I + Mk.II                         │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  [TFLOPS/Watt (AI Efficiency)]                                   │
  │  SOTA H100      ████░░░░░░░░░░░░░░░░░░░░░░░░░░  2.0            │
  │  Mk.I (SW only) ████░░░░░░░░░░░░░░░░░░░░░░░░░░  2.0 (same HW) │
  │  Mk.II (pipeline)████████░░░░░░░░░░░░░░░░░░░░░░  4.0            │
  │  Mk.III (co-des) ████████████████████████████░░  8.3            │
  │                                    (sigma-tau=8 TFLOPS/W)        │
  │                                                                  │
  │  [Training Cost (7B model)]                                      │
  │  SOTA (Llama3)   ████████████████████████████░  $10M+           │
  │  Mk.I            ████████████████████░░░░░░░░░  $6.7M           │
  │  Mk.II           ████████████░░░░░░░░░░░░░░░░░  $3.3M           │
  │  Mk.III          ████░░░░░░░░░░░░░░░░░░░░░░░░░  $1.0M           │
  │                                    (sigma-phi=10x total)         │
  │                                                                  │
  │  [Memory Bandwidth]                                              │
  │  SOTA H100       ████████████░░░░░░░░░░░░░░░░░  3.35 TB/s      │
  │  Mk.III HEXA-3D  ████████████████████████████░  100 TB/s       │
  │                                    (J₂²/phi = 288x improvement)  │
  │                                                                  │
  │  [Inference Energy per Token]                                    │
  │  SOTA H100       ████████████████████████████░  1.0 mJ/token   │
  │  Mk.III HEXA-PIM ███░░░░░░░░░░░░░░░░░░░░░░░░░  0.1 mJ/token  │
  │                                    (1/(sigma-phi) = 0.1x)       │
  └──────────────────────────────────────────────────────────────────┘
```

### Upgrade Delta: Mk.II -> Mk.III

| Metric | SOTA | Mk.II | Mk.III | Delta(II->III) | Delta Basis |
|--------|------|-------|--------|---------------|------------|
| TFLOPS/W | 2.0 | 4.0 | 8.3 | +4.3 (+108%) | Custom silicon: Phi6 HW unit + Boltzmann gate |
| Training cost 7B | $10M | $3.3M | $1.0M | -$2.3M (-70%) | Diamond substrate + PIM zero data movement |
| Memory BW | 3.35 TB/s | 3.35 TB/s | 100 TB/s | +96.6 TB/s (29x) | HEXA-3D TSV sigma·J₂=288/mm² |
| Inference mJ/token | 1.0 | 0.5 | 0.1 | -0.4 (-80%) | HEXA-PIM + photonic MAC 0.01pJ |
| n6 alignment | 0% | 100% (SW) | 100% (HW+SW) | HW enforcement | sigma·tau=48nm gate, sigma²=144 SM |

---

## 4. ASCII 2: System Architecture

```
  ┌──────────────────────────────────────────────────────────────────────────┐
  │              HEXA-AI Mk.III — HW-SW Co-Design Architecture              │
  ├──────────────────────────────────────────────────────────────────────────┤
  │                                                                          │
  │  ┌─ SOFTWARE ─────────────────────────────────────────────────────────┐  │
  │  │  R(6)=1 Meta-Loss → Leech-24 Surface → 17 Techniques Unified     │  │
  │  │  Zero HP search | Emergent convergence | SEDI 4-lens monitoring   │  │
  │  └───────────────────────────────┬────────────────────────────────────┘  │
  │                                  │ hardware-accelerated                  │
  │  ┌─ HARDWARE ────────────────────▼────────────────────────────────────┐  │
  │  │                                                                    │  │
  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐          │  │
  │  │  │ PHI6 Unit│  │ BOLTZ    │  │ EGYPTIAN │  │ MERTENS  │          │  │
  │  │  │ Cyclotomic│  │ 1/e Gate │  │ Attn Sch │  │ Dropout  │          │  │
  │  │  │ 71% FLOPs│  │ 63% sparse│  │ 1/2+1/3  │  │ ln(4/3)  │          │  │
  │  │  │ in silicon│  │+1/6=1 HW │  │ fixed HW │  │ RNG HW   │          │  │
  │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘          │  │
  │  │       │              │              │              │               │  │
  │  │       └──────────────┴──────────────┴──────────────┘               │  │
  │  │                          │                                         │  │
  │  │  ┌───────────────────────▼────────────────────────────────────┐   │  │
  │  │  │  HEXA-1 Chip Core (Diamond Z=6, sigma²=144 SM)            │   │  │
  │  │  │  sigma·J₂=288 GB HBM | J₂=24 NPU | sigma-tau=8 P-cores  │   │  │
  │  │  │  PIM (sigma·(sigma-tau)·2^n = 6,144 MAC) | 100 TB/s 3D  │   │  │
  │  │  │  sigma=12 WDM photonic interconnect (BT-89)               │   │  │
  │  │  └────────────────────────────────────────────────────────────┘   │  │
  │  │                                                                    │  │
  │  │  System: Topo DC (PUE=sigma/(sigma-phi)=1.2, n=6 topology)       │  │
  │  └────────────────────────────────────────────────────────────────────┘  │
  │                                                                          │
  │  5-Level Chain:                                                          │
  │  ┌─────────┬─────────┬─────────┬─────────┬─────────┐                   │
  │  │  소재   │  공정   │  코어   │   칩    │ 시스템  │                   │
  │  │ Diamond │ TSMC N2 │ HEXA-P  │ HEXA-1  │ Topo DC │                   │
  │  │ Z=6=n   │48nm=σ·τ │σ²=144SM │288GB=σJ₂│PUE=1.2  │                   │
  │  └─────────┴─────────┴─────────┴─────────┴─────────┘                   │
  └──────────────────────────────────────────────────────────────────────────┘
```

---

## 5. ASCII 3: Data/Energy Flow

```
  Tokens ──→ [PHI6 HW Unit] ──→ [EGYPTIAN Attn HW] ──→ [BOLTZ Gate HW] ──→ Output
             sigma: cyclotomic    sigma: 1/2+1/3+1/6=1  phi: 1/e sparse
             71% FLOPs saved      40% FLOPs saved        63% activation
             IN SILICON            IN SILICON              IN SILICON
                  │                     │                      │
                  └──────────┬──────────┘                      │
                             ▼                                 │
                     [PIM Memory Layer]                        │
                     sigma-tau=8 units/layer                   │
                     2^n=64 MAC/unit                           │
                     Zero data movement                        │
                             │                                 │
                             └─────────┬───────────────────────┘
                                       ▼
                               [HEXA-1 Die]
                               sigma²=144 SM
                               sigma·J₂=288 GB
                               Diamond Z=6
                                       │
                                       ▼
                               [Topo DC System]
                               PUE=sigma/(sigma-phi)=1.2
                               48V=sigma·tau DC bus
                               Battery backup: J₂=24 cells

  Energy budget (per inference token):
    Compute:    0.04 mJ (PHI6 + BOLTZ hardware gates)
    Memory:     0.02 mJ (PIM, zero movement)
    Interconn:  0.01 mJ (photonic sigma=12 WDM)
    Overhead:   0.03 mJ (PUE=1.2 overhead)
    Total:      0.10 mJ/token = 1/(sigma-phi) of SOTA
```

---

## 6. Required Breakthroughs

| # | Breakthrough | Difficulty | Timeline | Dependency |
|---|------------|-----------|----------|-----------|
| 1 | Diamond wafer production at chip scale | Hard | 2030-2035 | BT-93 Z=6 material |
| 2 | PHI6 cyclotomic unit in silicon (custom ASIC) | Medium | 2028-2032 | Technique T01 proven |
| 3 | Egyptian attention scheduler in hardware | Medium | 2028-2032 | Technique T17 proven |
| 4 | Boltzmann 1/e gate as hardware primitive | Low | 2027-2030 | Standard sparsity gate |
| 5 | HEXA-PIM integration (compute-in-memory) | Medium | 2030-2035 | Samsung HBM-PIM roadmap |
| 6 | Photonic interconnect sigma=12 WDM channels | Hard | 2032-2040 | BT-89 photonic bridge |
| 7 | HEXA-3D TSV at sigma·J₂=288/mm² density | Medium | 2030-2035 | TSMC 3D roadmap |

---

## 7. Timeline

| Year | Milestone |
|------|-----------|
| 2028 | PHI6 ASIC tape-out (28nm test chip) |
| 2030 | HEXA-PIM integration on silicon |
| 2032 | Diamond substrate prototype (small die) |
| 2035 | Mk.III full prototype: Diamond + PIM + PHI6 + Egyptian HW |
| 2038 | Production Mk.III at scale |
| 2040 | Photonic interconnect integration (full L4 capability) |

**Feasibility**: 🔮 Requires 2-3 breakthroughs (diamond wafer, photonic interconnect) that are on industry roadmaps but not yet demonstrated at chip scale.

---

## Links
- [Mk.II Near-Term](mk-2-near-term.md)
- [Mk.IV Long-Term](mk-4-long-term.md)
- [Chip Goal](../../chip-architecture/goal.md)
- [BT-89 Photonic Bridge](../../chip-architecture/hexa-photon.md)
- [BT-90 SM = phi x K₆](../../chip-architecture/bt90-92-topological-chip.md)
- [BT-93 Carbon Z=6](../../chip-architecture/hexa-material.md)
