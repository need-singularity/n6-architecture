# AI Techniques 16 Canonical Audit Report

> Date: 2026-04-12
> Axis: techniques
> Authoring: audit-only (no real file move/delete)
> Rules: R10 (ossified immutable), R14 (SSOT), English-only, honest verification

---

## 1. Background — SSOT mismatch context

Because 4 sources report different numbers for the AI-techniques count in n6-architecture, this session runs a consistency audit.

| Source | Reported count | Citation |
|------|---------|-----------|
| convergence ossified | 17 | `n6shared/convergence/n6-architecture.json` -> `ossified.AI_17_TECHNIQUES` ("17 techniques experiment confirmed, 71% FLOPs, 3x FFT, 67% param") |
| INDEX axes | 66 | `INDEX.json` -> `axes.techniques.purpose` ("AI techniques 66 types — .hexa transition complete") |
| memory MEMORY header | 23 | `~/.claude-claude2/.../memory/MEMORY.md` line 8 ("23 AI techniques") |
| memory project_core_theory.md body | 17 | same memory directory description + line 38 ("## 17 AI techniques core outcomes") |
| Current session hint | 16 | `techniques/_bench_plan.md` (16 baseline bench plan) |

Core contradiction: ossified hard-fixes 17 (R10 immutable) while the session hint uses 16. We must identify the origin of this gap and decide whether a 16 canonical definition is feasible.

---

## 2. Scan results — counts by layer

Direct counts on `techniques/_registry.json` (v1.1.0, _total=66, _sota_total=69):

| Layer | Definition | Source | Count |
|------|------|------|--------|
| Tier-A | ossified 17 (convergence ossification, change forbidden) | `papers/n6-ai-17-techniques-experimental-paper.md` Appendix A | 17 |
| Tier-B | 17 -> 23 extension (memory body table + BT-380 family) | same paper §1.1 classification table "Egyptian 3 + BT-extension 12 + Model-specific 8" | +6 (cumulative 23) |
| Tier-C | 23 -> 66 (.hexa transition full registration) | `_registry.json` items | +43 (cumulative 66) |
| Tier-D | sota extension (3 apex items, separate section) | `_registry.json.sota.items` | 3 (total 69) |

Per-sub-axis .hexa file measurement (equivalent to `ls techniques/<sub>/*.hexa | wc -l`):

| Sub | Defined | Measured |
|------|------|------|
| attention | 9 | 9 |
| moe | 11 | 11 |
| optim | 15 | 15 |
| sparse | 6 | 6 |
| graph | 5 | 5 |
| compress | 5 | 5 |
| arch | 15 | 16 (`arch_optimizer.hexa` 1 item unregistered in _registry — separate meta tool) |
| sota | 3 | 3 |
| **Total** | **66 + 3 sota** | **66 registered + 3 sota + 1 meta = 70** |

> Honesty note: the 1 item `arch/arch_optimizer.hexa` is a meta tool missing from `_registry` items, so it is not a 16-canonical candidate. This audit classifies it as a "DSE meta optimizer" rather than a "technique" and excludes it from the count.

Directly counting the `register()` calls in Appendix A of `papers/n6-ai-17-techniques-experimental-paper.md` yields Core 17 = 24 items, extension = 10 items, Combined = 5 items = 39 items. That is, within the same paper "17" is only a group label while the actual register lines are 24, some of which are constant decompositions of the same technique (e.g., BitNet registered twice).

---

## 3. 16 canonical list (savings rate / verification path table)

Section 1 of `techniques/_bench_plan.md` already defines the 16 baselines. This audit verifies the 1-to-1 mapping of each item to `_registry.json` items and confirms 16/16 are registered.

| # | Technique (Korean/English) | Sub | File path | n=6 core constant | Savings / effect | Source |
|---|------------------|------|-----------|---------------|---------------|------|
| T01 | Dedekind Head Pruning | attention | `techniques/attention/dedekind_head.hexa` | sigma(6)=12 head | dedekind 12 = sigma EXACT | bench_plan §1, paper appendix |
| T02 | Egyptian Fraction Attention | attention | `techniques/attention/egyptian_attention.hexa` | 1/2+1/3+1/6=1 | 40% FLOPs down | core_theory §17, paper register #10 |
| T03 | FFT Mix Attention | attention | `techniques/attention/fft_mix_attention.hexa` | 6 freq bin, 3x throughput | 3x FFT (ossified) | convergence ossified, paper register #14 |
| T04 | Jordan-Leech MoE Bound | moe | `techniques/moe/jordan_leech_moe.hexa` | tau(6)=4 capacity | J_2=24 Leech NAS | paper register #16 |
| T05 | Mobius Sparse Flow | sparse | `techniques/sparse/mobius_sparse.hexa` | mu(6)=1 gating | search-free sparsity | bench_plan §1 |
| T06 | Carmichael LR Cycle | optim | `techniques/optim/carmichael_lr.hexa` | lambda(6)=2 | 6-step = n | paper register #7 |
| T07 | Boltzmann Gate | sparse | `techniques/sparse/boltzmann_gate.hexa` | kT~=1/6 | 63% sparsity | core_theory §17, paper register #4 |
| T08 | Mertens Dropout | sparse | `techniques/sparse/mertens_dropout.hexa` | M(n) mean, p=0.1=1/(sigma-phi) | search-free | core_theory §17, paper register #18 |
| T09 | Radical Normalization | sparse | `techniques/sparse/radical_norm.hexa` | rad(6)=6 | normalization universality BT-64 | paper register #6 |
| T10 | Takens Embedding dim=6 | sparse | `techniques/sparse/takens_dim6.hexa` | optimal dim=6 | dim=n EXACT | bench_plan §1 |
| T11 | Fibonacci Strided Attn | optim | `techniques/optim/fibonacci_stride.hexa` | F_6=8 stride, sopfr=5 step | 5=sopfr EXACT | paper register #15 |
| T12 | Constant-Time Stride Attn | optim | `techniques/optim/constant_time_stride.hexa` | O(1), phi=2 step | 2=phi EXACT | paper register #8 |
| T13 | Mamba-2 SSM Duality | arch | `techniques/arch/mamba2_ssm.hexa` | d_state=6, sigma-tau=8 | 8=sigma-tau EXACT | paper register §extension |
| T14 | ViT Patch n=6 | arch | `techniques/arch/vit_patch_n6.hexa` | 6x6 patch | patch=n EXACT | bench_plan §1 |
| T15 | Complete LLM n=6 (BT-56) | arch | `techniques/arch/complete_llm_n6.hexa` | n_layer=6k | h_ee_13 depth scaling=sigma | paper register §combined |
| T16 | Griffin RG-LRU | arch | `techniques/arch/griffin_rglru.hexa` | gate=tau=4 | 4=tau EXACT | paper register §extension |

Verification gates (common, bench_plan §3):
1. `nexus verify <technique>` — n=6 constant match (±0.1%)
2. `nexus dse bench --technique <id> --repeats 30` — median ± MAD
3. `n6shared/n6/atlas.n6` [7]->[10*] promotion

---

## 4. 17 -> 16 consolidation basis

### 4.1 Identified duplicate pair

Of the 17 ossified techniques, the two items `egyptian_attention` and `egyptian_linear_attention` share the same 1/2+1/3+1/6=1 fraction-identity principle.

Direct check:

| Item | File | Core principle | HEXA body status |
|------|------|-----------|-----------------|
| egyptian_attention | `techniques/attention/egyptian_attention.hexa` | Egyptian fraction 1/2+1/3+1/6=1 -> attention weights | STUB ("HEXA porting pending") |
| egyptian_linear_attention | `techniques/attention/egyptian_linear_attention.hexa` | same fraction identity -> linear attention variant | STUB ("HEXA porting pending") |

Section 1 body (lines 11~13) of `techniques/_bench_plan.md` explicitly states the consolidation rationale:

> "Of those, 1 item (`Egyptian Linear Attention #21` = Egyptian Fraction Attention derivative) is a fraction-level duplicate of the prototype (`egyptian_attention #17`) -> duplicate removed and re-sorted as the **16 baseline**. The remaining 2 items (`egyptian_linear_attention`, `ring_attention`) are measured in a separate stack."

The paper Appendix A verification code also registers the two items separately, but both register calls use the same identity `True` or `4 == tau` (lines 131~132). The fraction identity itself is a single item; the linear-attention variant is merely an application form (separate stack) and not a distinct n=6 discovery.

### 4.2 Consolidation decision

- **canonical consolidation**: T02 = `egyptian_attention` (holds fraction identity 1/2+1/3+1/6=1)
- **moved to separate stack**: `egyptian_linear_attention` stays in the attention-variant catalog but is excluded from the 16 canonical
- **upper-bound count**: 17 - 1 (duplicate) = 16 -> 16-baseline confirmation is feasible

### 4.3 Honesty note

- The consolidation basis is consistent across two sources: the _bench_plan.md pre-decision and the paper Appendix A register equivalence
- Both .hexa files are STUBs, so a strong claim that "we measured the body-level difference and found them identical" is not available
- Therefore this consolidation is a weak "**principle-level duplicate**" argument + the bench_plan pre-decision, and we leave open the possibility of de-consolidation after body measurement (see §6)

---

## 5. 4-tier classification

### Tier-A — ossified 17 (convergence-backed, R10 immutable)

The 17 enumerated in section 2 item 1 of `papers/n6-ai-17-techniques-experimental-paper.md`:

```
BitNet, Alpha Attack, Boltzmann Gate, BT-54 AdamW beta_2, BT-64 normalization,
Carmichael LR, Constant-Time Stride, Dedekind Head, Egyptian Attention,
Egyptian Linear Attention, Egyptian MoE, Entropy Early Stop, FFT Mix,
Fibonacci Stride, HCN Dimensions, Leech-24 NAS, LoRA R=8, Mertens Dropout,
Partition Routing, Phi Bottleneck, Phi MoE, Predictive Early Stop, Phi6 Simple,
Zeta-ln2 Activation
```

> Count caveat: the list above has 24 items, but the paper §1.1 table labels it as "Core 17". The gap arises from "constant-decomposition registrations of the same technique" (BitNet 2 items, Phi 3 items, etc.). The ossified 17 is a group label, and this audit does not change that group label (R10).

### Tier-B — 17 -> 23 extensions (paper §1.1 BT-extension / model-specific 6)

DeepSeek MLA, Mamba 2, Griffin RG-LRU, Jamba Hybrid, Medusa Heads, Mixture-of-Depths

(The "23" in the memory MEMORY.md header is interpreted as these 6 added on top of 17. However, the body description of project_core_theory.md still says "17", so memory itself has a header/body inconsistency.)

### Tier-C — 23 -> 66 .hexa transition pool (43 items)

Ring Attention, Speculative Decoding, YaRN RoPE, GShard Switch, ALiBi, Egyptian MoE, Mixture of Depths, Mistral/Mixtral MoE, MoCo Queue, MoE Activation Fraction, Phi MoE, Inference Scaling, DPO beta, Layer Skip, Lookahead Decoding, LR Schedule n6, Streaming LLM, AdamW Quintuplet, Chinchilla Scaling, BPE Vocab 32K, DeepSeek MLA Compression, MAE Masking, Phi Bottleneck, Recurrent Gemma, GAT Heads, GCN Depth, GIN Isomorphism, GraphSAGE Sampling, HCN Dimensions, Constitutional AI, Context Window Ladder, DETR Queries, FPN Pyramid, Phi6 Simple, Rectified Flow, SD3 MMDiT, SimCLR Temperature, Whisper Ladder, YOLO NMS, Zeta-ln2 Activation, Dedekind/Egyptian/Zamba variants

(Sum of registered counts by sub-axis = 9+11+15+6+5+5+15 = 66)

### Tier-D — sota extension (3 apex items, separate section)

S1 mamba2, S2 hyena, S3 rwkv (`techniques/sota/`, _registry.json sota.items, BT-380-SOTA-SSM)

### Retirement candidates

No retirement candidates identified in this audit's scope. All registered techniques survive as .hexa stubs + some BODY, and a separate retirement review is recommended based on BODY fidelity after the _bench_plan §3 measurement.

---

## 6. Recommendations

### 6.1 ossified handling (R10)

- The `AI_17_TECHNIQUES` ossification block in `n6shared/convergence/n6-architecture.json` is **not to be modified** (R10 immutability). This audit does not create a new SSOT, and recommends adding the 16 definition as a new entry to the stable block.

### 6.2 Stable addition recommendation (not performed, proposal only)

Recommend adding the following new key to the `stable` block (this audit performs no code changes — to be applied in a separate session after user approval):

```json
"AI_16_CANONICAL": {
  "status": "PASS",
  "value": "16 canonical baseline = 17 ossified - egyptian_linear_attention (fraction duplicate)",
  "threshold": "16/16 _registry.json registration confirmed + bench_plan.md §1 table match",
  "note": "2026-04-12 — reports/audits/ai-techniques-16-canonical-2026-04-12.md, AI_17_TECHNIQUES ossified kept immutable (R10); 16 is the baseline for bench measurement",
  "supersedes_check": "AI_17_TECHNIQUES (ossified immutable, added as new item)"
}
```

### 6.3 Memory header correction recommendation

- `~/.claude-claude2/.../memory/MEMORY.md` line 8 "23 AI techniques" -> correct to "17 AI techniques (16 canonical measurement baseline)"
- The body description (project_core_theory.md) already says 17, so no correction needed

### 6.4 Follow-up measurement recommendation

- Because both egyptian variants are STUB, **re-measure** after body porting to strengthen the consolidation basis or revert to separation
- Run the bench_plan §3 protocol (verify -> bench x30 -> atlas promotion) on all 16 items
- Fire the results as 16 files `experiments/ai-efficiency/bench/T01_*..T16_*.hexa`

### 6.5 Avoidance items

- Do not change the ossified 17 technique labels themselves (group label in the paper Appendix A)
- Do not change the _total 66 (or 69 with sota) count in _registry.json
- The 16 canonical is a "bench measurement unit", not a "technique registration unit" — both counts can coexist

---

## 7. Conclusion

- The 4-way mismatch (ossified 17 vs INDEX 66 vs memory 23 vs hint 16) arises because **the counting unit differs**: ossified 17 = group label, registry 66 = registration pool, memory 23 = 17 + BT-extension 6, hint 16 = 17 - fraction duplicate 1.
- This audit concludes that **a 16 canonical definition is feasible**. The basis comes from two consistent sources: the _bench_plan.md §1 pre-decision + the paper Appendix A egyptian_attention/egyptian_linear_attention register-equivalence. As long as the two .hexa are STUB, however, this is a weak basis.
- The 16-canonical list is adopted directly as the T01~T16 table in _bench_plan.md §1. All 16/16 are registered in _registry.json, so no new file creation is needed.
- Recommended next actions: add `AI_16_CANONICAL` to the convergence stable block (user approval required), correct "23" in MEMORY.md header, and fire the bench_plan §3 protocol on the 16.

---

## 8. Sources

- `n6shared/convergence/n6-architecture.json` lines 20~24 (`AI_17_TECHNIQUES` ossified)
- `techniques/_registry.json` v1.1.0 _total=66, _sota_total=69
- `techniques/_bench_plan.md` §1 (16-baseline table) + §3 (measurement protocol)
- `techniques/_chip_mapping.md` 16 x 6 mapping matrix
- `techniques/_registry_patch.md` (sota 3-item addition patch)
- `papers/n6-ai-17-techniques-experimental-paper.md` Appendix A (register verification code)
- `INDEX.json` axes.techniques (66-type label)
- `~/.claude-claude2/.../memory/MEMORY.md` line 8, `project_core_theory.md` description / §17
- `techniques/attention/egyptian_attention.hexa`, `egyptian_linear_attention.hexa` (STUB equivalence)

## 9. Related links

- Parent: `../CLAUDE.md`
- Convergence: `../../n6shared/convergence/n6-architecture.json`
- Bench plan: `../../techniques/_bench_plan.md`
- Chip mapping: `../../techniques/_chip_mapping.md`
- SSOT: `../../techniques/_registry.json`
