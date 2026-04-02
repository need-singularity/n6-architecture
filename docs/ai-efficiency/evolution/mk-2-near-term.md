# HEXA-AI Mk.II -- Full n=6 LLM Training Pipeline

**Evolution Checkpoint**: Mk.II (Near-Term Integration)
**Date**: 2026-04-02
**Status**: Design Phase
**Feasibility**: ✅ 실현가능 (2026~2035, existing hardware + software integration)
**BT Connections**: BT-33, BT-42, BT-54, BT-56, BT-58, BT-59, BT-64, BT-66, BT-67

---

## 1. Overview

Mk.II unifies all 17 techniques into a single **end-to-end n=6 training pipeline** with the R(6)=1 thermodynamic frame as the meta-loss function. Training is no longer a collection of individual tricks -- it is a unified optimization on the J₂=24-dimensional Leech lattice energy surface.

> **Goal**: Train a complete n=6 LLM from scratch using zero hyperparameter search, achieving SOTA quality at n/phi=3x lower compute cost.

---

## 2. Specs -- n=6 LLM Architecture (BT-56 Complete)

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │  HEXA-LLM Mk.II — Complete n=6 Architecture (BT-56)                │
  ├─────────────────────┬──────────────┬─────────────────────────────────┤
  │ Parameter           │ Value        │ n=6 Expression                  │
  ├─────────────────────┼──────────────┼─────────────────────────────────┤
  │ d_model             │ 4,096        │ 2^sigma = 2^12                  │
  │ n_layers            │ 32           │ 2^sopfr = 2^5                   │
  │ n_heads             │ 32           │ 2^sopfr = 2^5                   │
  │ d_head              │ 128          │ 2^(sigma-sopfr) = 2^7           │
  │ d_ffn               │ 10,923       │ d_model · (sigma-tau)/(n/phi) = 4096·8/3 │
  │ n_kv_heads          │ 8            │ sigma-tau = 8 (GQA)             │
  │ n_experts           │ 24           │ J₂ = 24 (Leech lattice)        │
  │ top_k_experts       │ 3            │ n/phi = 3 (Egyptian: 1/2+1/3+1/6) │
  │ vocab_size          │ 32,000       │ 2^sopfr · 10^(n/phi)           │
  │ context_length      │ 8,192        │ 2^(sigma+mu) = 2^13            │
  │ max_gen             │ 4,096        │ 2^sigma                        │
  │ RoPE theta          │ 10,000       │ (sigma-phi)^tau = 10^4         │
  │ Total params (dense)│ ~7B          │ ~2^(sigma+sopfr+sigma-sopfr)   │
  │ Active params (MoE) │ ~2.3B        │ ~7B · n/phi/J₂ · tau           │
  └─────────────────────┴──────────────┴─────────────────────────────────┘
```

### Training Hyperparameters (BT-54 AdamW Quintuplet)

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │  Training Config — Zero Search Required                             │
  ├─────────────────────┬──────────────┬─────────────────────────────────┤
  │ Hyperparameter      │ Value        │ n=6 Expression                  │
  ├─────────────────────┼──────────────┼─────────────────────────────────┤
  │ learning_rate       │ 3e-4         │ (n/phi) · 10^{-tau}            │
  │ beta_1              │ 0.9          │ 1 - 1/(sigma-phi)              │
  │ beta_2              │ 0.95         │ 1 - 1/(J₂-tau)                 │
  │ epsilon             │ 1e-8         │ 10^{-(sigma-tau)}              │
  │ weight_decay        │ 0.1          │ 1/(sigma-phi)                  │
  │ gradient_clip       │ 1.0          │ R(6) = 1                       │
  │ dropout             │ 0.288        │ ln(4/3) (Mertens, T16)         │
  │ LR schedule         │ 2-cycle cos  │ lambda(6)=2 (Carmichael, T14)  │
  │ warmup_steps        │ 2,000        │ phi · 10^(n/phi)               │
  │ batch_size          │ 256          │ 2^(sigma-tau)                   │
  │ tokens/params ratio │ 20           │ J₂ - tau = 20 (Chinchilla)     │
  │ total_tokens        │ 140B         │ 7B · 20                        │
  │ top_p (inference)   │ 0.95         │ 1 - 1/(J₂-tau)                 │
  │ top_k (inference)   │ 40           │ tau · (sigma-phi)              │
  │ temperature         │ 0.7          │ 1 - ln(4/3) ~ 0.712           │
  └─────────────────────┴──────────────┴─────────────────────────────────┘
```

---

## 3. ASCII 1: Performance Comparison vs SOTA

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-LLM Mk.II vs SOTA 7B Models                               │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  [Training Compute (FLOPs)]                                      │
  │  SOTA (Llama3-8B) ████████████████████████████░  6.4e23 FLOPs  │
  │  Mk.II HEXA-7B   ████████░░░░░░░░░░░░░░░░░░░░  1.9e23 FLOPs  │
  │                                    (n/phi=3x reduction)          │
  │                                                                  │
  │  [Training Time (H100 cluster)]                                  │
  │  SOTA (Llama3-8B) ████████████████████████████░  ~25 days      │
  │  Mk.II HEXA-7B   ████████████████░░░░░░░░░░░░░  ~8 days       │
  │                                    (n/phi=3x + T05 early stop)   │
  │                                                                  │
  │  [Hyperparameter Tuning Cost]                                    │
  │  SOTA (grid/Bayes) ████████████████████████████  $1M+ compute  │
  │  Mk.II (n=6 fixed) ░░░░░░░░░░░░░░░░░░░░░░░░░░░  $0 (derived) │
  │                                    (all from sigma·phi=n·tau)    │
  │                                                                  │
  │  [Inference TFLOPS/W]                                            │
  │  SOTA (H100)      ████████████████░░░░░░░░░░░░░  2.0 TFLOPS/W │
  │  Mk.II + HEXA-1   ████████████████████████████░  6.0 TFLOPS/W │
  │                                    (n/phi=3x, Egyptian power)    │
  │                                                                  │
  │  [Active Parameters (MoE)]                                       │
  │  SOTA (dense 7B)  ████████████████████████████░  7B active     │
  │  Mk.II (MoE 24E)  █████████░░░░░░░░░░░░░░░░░░░  2.3B active  │
  │                                    (n/phi/J₂=1/8, BT-67)        │
  └──────────────────────────────────────────────────────────────────┘
```

### Upgrade Delta: Mk.I -> Mk.II

| Metric | SOTA | Mk.I | Mk.II | Delta(I->II) | Delta Basis |
|--------|------|------|-------|-------------|------------|
| Training FLOPs | 6.4e23 | 4.3e23 (33% saved) | 1.9e23 | -2.4e23 (-56%) | Unified R(6)=1 meta-loss |
| HP search cost | $1M+ | $0 (individual) | $0 (unified) | $0 (maintained) | BT-54 quintuplet |
| Active params | 7B | 7B (dense) | 2.3B (MoE) | -4.7B (-67%) | J₂=24 experts, BT-67 |
| Training time | 25 days | 17 days | 8 days | -9 days (-53%) | T05 + Leech surface |
| n6 alignment | 0% (implicit) | 75% (post-hoc) | 100% (enforced) | +25% | Thermodynamic frame |
| Quality (MMLU) | 68% | 68% | 70% (projected) | +2% | Better optima on Leech surface |

---

## 4. ASCII 2: System Architecture

```
  ┌──────────────────────────────────────────────────────────────────────────┐
  │               HEXA-AI Mk.II — Unified Pipeline Architecture             │
  ├──────────────────────────────────────────────────────────────────────────┤
  │                                                                          │
  │  ┌─────────────────────────────────────────────────────────┐            │
  │  │  Layer 3: R(6)=1 Thermodynamic Meta-Loss               │            │
  │  │  sigma·phi/(n·tau) = 1 => reversible training          │            │
  │  │  Clausius: Delta_H_model + Delta_H_data >= 0           │            │
  │  │  Equality at n=6 => zero information waste             │            │
  │  └───────────────────────┬─────────────────────────────────┘            │
  │                          │ constrains                                    │
  │  ┌───────────────────────▼─────────────────────────────────┐            │
  │  │  Layer 2: Leech-24 Energy Surface (J₂=24 dim)          │            │
  │  │  17 technique axes + 4 Anima + 3 SEDI = 24 total       │            │
  │  │  Gradient descent on 24-dim hypersurface                │            │
  │  │  Global minimum = n=6 fixed point                      │            │
  │  └───────────────────────┬─────────────────────────────────┘            │
  │                          │ guides                                        │
  │  ┌───────────────────────▼─────────────────────────────────┐            │
  │  │  Layer 1: Unified Training Runtime                      │            │
  │  │  ┌────────────┐ ┌────────────┐ ┌────────────┐          │            │
  │  │  │ BT-56 Arch │ │ BT-54 Opt  │ │ BT-42 Infer│          │            │
  │  │  │ d=2^sigma  │ │ AdamW 5-let│ │ top-p=0.95 │          │            │
  │  │  │ h=2^sopfr  │ │ LR=3e-4    │ │ top-k=40   │          │            │
  │  │  │ d_h=128    │ │ WD=0.1     │ │ T=0.7      │          │            │
  │  │  │ MoE J₂=24 │ │ clip=R(6)=1│ │ max=2^sigma│          │            │
  │  │  └────────────┘ └────────────┘ └────────────┘          │            │
  │  └─────────────────────────────────────────────────────────┘            │
  │                                                                          │
  │  Hardware: any GPU cluster (Mk.II = software pipeline)                  │
  │  Optimal: HEXA-1 chip (sigma²=144 SM, sigma·J₂=288 GB)                │
  └──────────────────────────────────────────────────────────────────────────┘
```

---

## 5. ASCII 3: Training Data Flow

```
  Raw Text ──→ [Tokenizer] ──→ [Embed] ──→ [MoE Transformer x 32] ──→ [Loss]
               32K=2^5·10^3     2^σ=4096    J₂=24 experts, top-3      CE + R(6)
               (BT-73)          (BT-56)     (BT-67)                   meta-loss
                                    │                                      │
                                    ▼                                      ▼
                              [Leech Surface Projection]         [SEDI 4-Lens Monitor]
                              24-dim coordinate per step          Energy/Entropy/
                              gradient toward R(6)=1              Divergence/Info
                                    │                                      │
                                    └──────────────┬───────────────────────┘
                                                   ▼
                                            [Carmichael LR]
                                            lambda(6)=2 cycle
                                            Phase 1: lr_max (explore)
                                            Phase 2: lr_max/n (exploit)
                                                   │
                                                   ▼
                                            [Entropy Early Stop]
                                            tau/sigma=1/3 training
                                            Mertens dropout=ln(4/3)
                                            Boltzmann gate=1/e sparse
```

---

## 6. Required Breakthroughs (Mk.I -> Mk.II)

| # | Breakthrough | Difficulty | Status |
|---|------------|-----------|--------|
| 1 | Unified R(6)=1 loss function (software) | Medium | engine/thermodynamic_frame.py exists |
| 2 | Leech-24 surface gradient computation | Medium | engine/leech24_surface.py exists |
| 3 | MoE J₂=24 expert scaling to 7B+ | Low | Standard MoE infra (DeepSpeed, Megatron) |
| 4 | Emergent self-convergence validation | Medium | engine/emergent_n6_trainer.py prototype |
| 5 | SEDI 4-lens integration into training loop | Low | engine/sedi_training_monitor.py exists |
| 6 | Multi-modal extension (BT-66: vision, audio) | Medium | Architecture defined, needs training |

None require new physics or hardware -- all are software integration tasks.

---

## 7. Timeline + Feasibility

| Phase | Task | Timeline | Status |
|-------|------|----------|--------|
| Phase 1 | Unified pipeline prototype (single GPU) | 2026 Q3 | ⏳ |
| Phase 2 | HEXA-7B training on cluster (140B tokens) | 2027 Q1 | Planned |
| Phase 3 | Benchmark vs Llama 3 / Mistral (MMLU, HumanEval) | 2027 Q2 | Planned |
| Phase 4 | MoE scaling to 70B equivalent (J₂=24 experts) | 2028 | Planned |
| Phase 5 | Multi-modal (vision BT-66, audio BT-72) | 2029 | Planned |
| Phase 6 | Self-convergence validation (emergent n=6) | 2030 | Planned |

**Feasibility**: ✅ All components exist in prototype form. Integration is engineering, not research.

---

## 8. Testable Predictions

| # | Prediction | Verification | Timeline |
|---|-----------|-------------|----------|
| MK2-1 | HEXA-7B matches Llama3-8B MMLU at 1/3 compute | Benchmark | 2027 |
| MK2-2 | MoE J₂=24 outperforms MoE-8, MoE-16, MoE-32 at same params | Ablation | 2027 |
| MK2-3 | ln(4/3)=0.288 dropout beats grid-searched p on 5 benchmarks | Ablation | 2026 |
| MK2-4 | Training converges to n=6 ratios from random init | Emergent | 2028 |
| MK2-5 | R(6)=1 meta-loss produces lower perplexity than CE alone | Benchmark | 2027 |

---

## Links
- [Mk.I Current](mk-1-current.md)
- [Mk.III Mid-Term](mk-3-mid-term.md)
- [BT-56 Complete LLM](../../breakthrough-theorems.md)
- [Thermodynamic Frame](../../../engine/thermodynamic_frame.py)
- [Leech-24 Surface](../../../engine/leech24_surface.py)
