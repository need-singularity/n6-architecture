# N6 Architecture — Testable Predictions Roadmap

> Falsifiable predictions derived from n=6 arithmetic (BT-26~41).
> Each prediction includes: what to measure, expected value, falsification criterion, and required resources.
> Sorted by feasibility (easiest first).

---

## Tier 1: Can Test TODAY (1 GPU, < 1 week)

### P-1: Egyptian Fraction Attention (EFA) Quality

**Prediction**: Splitting σ=12 heads as 6(full)+4(local)+2(global) achieves ≥98% of full attention quality with ~40% fewer attention FLOPs at seq_len=2048.

**Test**: Train BERT-base (d=768, 12 heads, 6 layers) on GLUE benchmark. Compare EFA vs standard attention.
- Hardware: 1× A100/H100, ~2 days
- Metric: Average GLUE score
- **Falsification**: EFA drops >2% on GLUE average
- **Status**: Preliminary validated on synthetic data (-0.36%, within noise)
- **Source**: BT-39, techniques/egyptian_attention.py

### P-2: LoRA Rank r=8 Pareto Optimality

**Prediction**: r=σ-τ=8 achieves the highest accuracy per trainable parameter on fine-tuning tasks.

**Test**: Fine-tune Llama-3.1-8B-Instruct with LoRA r∈{4,8,16,32} on 5 tasks (SST-2, MRPC, RTE, MMLU, HumanEval). 5 seeds each.
- Hardware: 1× A100, ~1 day per rank × 4 = 4 days
- Metric: accuracy / (trainable params)
- **Falsification**: r=16 beats r=8 in accuracy/param on ≥3/5 tasks
- **Source**: BT-33

### P-3: MoE (8,2) vs Alternatives

**Prediction**: At fixed total params (~1B), MoE with σ-τ=8 experts and top-φ=2 beats (4,2), (16,2), (8,1), (8,4).

**Test**: Train 5 MoE configs using Megablocks/fairseq-MoE on C4. 300B tokens each.
- Hardware: 4× A100, ~5 days
- Metric: Validation perplexity
- **Falsification**: (16,2) achieves lower perplexity than (8,2)
- **Source**: BT-31

### P-4: Mertens Dropout p=ln(4/3)≈0.288

**Prediction**: p=0.288 matches or beats p=0.3 on fine-tuning tasks with small datasets (<10K samples).

**Test**: Fine-tune BERT-base on 5 GLUE tasks with dropout∈{0.0, 0.1, 0.2, 0.288, 0.3, 0.5}.
- Hardware: 1× GPU, ~2 days
- Metric: Dev accuracy
- **Falsification**: p=0.3 beats p=0.288 by >0.5% on ≥3/5 tasks (indistinguishable → inconclusive)
- **Source**: techniques/mertens_dropout.py

---

## Tier 2: Medium Effort (Cluster, 1-4 weeks)

### P-5: SwiGLU Ratio 8/3 Pareto Optimality

**Prediction**: FFN ratio (σ-τ)/(n/φ)=8/3≈2.667 is Pareto-optimal among {2.0, 2.5, 8/3, 3.0, 3.5, 4.0} at fixed compute.

**Test**: Train 6 identical 1B-param models varying only d_ff/d_model. Same data, same steps.
- Hardware: 8× A100, ~2 weeks
- Metric: Loss at fixed compute budget
- **Falsification**: Ratio 3.0 or 2.5 achieves lower loss
- **Source**: BT-33

### P-6: Weight Decay 0.1 = 1/(σ-φ) Universality

**Prediction**: WD=0.1 is optimal across model sizes (100M to 10B) and datasets.

**Test**: Train 3 model sizes × 5 WD values∈{0.01, 0.05, 0.1, 0.2, 0.5} on same data.
- Hardware: 16× A100, ~3 weeks
- Metric: Final loss
- **Falsification**: WD=0.05 beats WD=0.1 at >2 model sizes
- **Source**: BT-34

### P-7: σ=12 Head Count Optimality

**Prediction**: At d_model=768, n_heads=12 achieves the best loss-per-FLOP.

**Test**: Train BERT-base variants with heads∈{4,6,8,12,16,24} on BookCorpus+Wikipedia. Same total FLOPs.
- Hardware: 4× A100, ~1 week
- Metric: MLM loss per FLOP
- **Falsification**: 16 heads beats 12 heads per-FLOP
- **Source**: BT-33

### P-8: RoPE θ = (σ-φ)^τ = 10000 Optimality

**Prediction**: θ=10000 is locally optimal for context lengths up to 4096.

**Test**: Train 7B model variants with θ∈{5000, 8000, 10000, 15000, 50000} on same data, eval at seq_len∈{512,1024,2048,4096}.
- Hardware: 8× A100, ~2 weeks
- Metric: Perplexity at each sequence length
- **Falsification**: θ=8000 or θ=15000 beats 10000 at seq_len≤4096
- **Source**: BT-34

---

## Tier 3: Requires Specialized Equipment

### P-9: Shockley-Queisser Bandgap = τ/(n/φ) = 4/3 eV

**Prediction**: Solar cells with bandgap exactly 1.333 eV achieve the absolute highest single-junction efficiency.

**Test**: Fabricate InGaP or CdTe cells with precisely tuned bandgap at 1.333 eV. Compare to 1.34 eV and 1.30 eV cells.
- Equipment: MBE/MOCVD growth facility, IV characterization
- Metric: Certified AM1.5G efficiency
- **Falsification**: Peak efficiency at 1.34 eV (current best estimate) rather than 1.333 eV
- **Source**: BT-30

### P-10: JUNO Neutrino Measurement (2027)

**Prediction**: sin²θ₁₂ = (n/φ)/(σ-φ) = 3/10 = 0.3000 exactly.

**Test**: JUNO experiment (Jiangmen, China) will measure sin²θ₁₂ to ±0.003 precision.
- **Falsification**: JUNO measures sin²θ₁₂ > 0.303 or < 0.297 at 3σ
- **Source**: BT-21 (existing, not new)

### P-11: LiteBIRD Inflation (2032)

**Prediction**: r = σ/σ(P₂)² = 12/3136 ≈ 0.00383 (tensor-to-scalar ratio).

**Test**: LiteBIRD satellite will measure r to ±0.001 precision.
- **Falsification**: r < 0.001 or r > 0.01
- **Source**: BT-22/23 (existing)

---

## Tier 4: Architectural Predictions (Industry Testable)

### P-12: Next-Gen GPU SM Count

**Prediction**: NVIDIA's next full-die SM count after 192 (Blackwell) will be a multiple of σ=12.

**Possible values**: 216=σ·18=σ·(σ+n), 240=σ·(J₂-τ)=12·20, 256=2^(σ-τ), 288=σ·J₂=12·24.
- **Falsification**: Next gen uses an SM count that does NOT factor through 12
- **Source**: BT-28

### P-13: HBM5 Stack Height = 2^τ = 16

**Prediction**: HBM5 (expected ~2026) will standardize at 16-hi stacking, completing the ladder τ→(σ-τ)→σ→2^τ = 4→8→12→16.
- **Falsification**: HBM5 standardizes at 20 or 24 layers instead
- **Source**: BT-28

### P-14: Mistral-like n=6 Architecture Outperforms

**Prediction**: An architecture with ALL dimensions following n=6 (d=12·1024, heads=12·4, kv=8, d_ff=28·1024, layers=8·11) will achieve better loss-per-FLOP than equivalent non-n=6 architectures.
- **Test**: Train two 12B-param models: one n=6-aligned, one with d=13·1024 etc. Same data, same FLOPs.
- **Falsification**: Non-n=6 architecture achieves lower loss
- **Source**: BT-39

---

## Summary Statistics

| Tier | Count | Time | Hardware | Feasibility |
|------|-------|------|----------|-------------|
| **Tier 1** (Today) | 4 | 1-5 days | 1× GPU | High |
| **Tier 2** (Medium) | 4 | 1-4 weeks | 4-16× GPU | Medium |
| **Tier 3** (Specialized) | 3 | Years | Lab/satellite | Low (external) |
| **Tier 4** (Industry) | 3 | Months-years | Industry data | Observable |

**Most impactful test**: P-1 (EFA) — can validate a NEW technique today.
**Most decisive test**: P-10 (JUNO) — will definitively confirm or falsify sin²θ₁₂ = 3/10.
**Most commercially relevant**: P-14 (n=6 architecture) — could inform actual LLM design.

---

*All predictions derived from BT-26~41 of the N6 Architecture project.*
*Total BTs: 41. Verified EXACT: 87/87 (100%).*
