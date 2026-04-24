# DSE_D1 — Cross-DSE 335 x 335 Cross-Resonance Pilot Result

> Generated: 2026-04-09 · Script: `scripts/dse_cross_pilot.py` · Post-processing: top-10 pairs + z-score
> Input SSOT: `docs/dse-map.toml` (335 domains, 17,591 lines)
> Intermediate artifacts: `$NEXUS/shared/dse_cross/` (pair_scores.jsonl, all_pairs_s05.jsonl, top10_result.json)

## 1. Purpose

Statistically detect arithmetic resonance (shared sigma/tau/phi formula matches) among 335 independent DSE TOML files.
To avoid the **missing n6_avg -> 50.0 neutral-value substitution bias** noted in a prior audit (`docs/bt-audit-report.md`), this pilot's `compute_pair_score()` performs only **weight renormalization** on missingness (drop W_PROX and divide by W_JAC+W_BIDIR+W_BAL), and records missing terms explicitly in a `miss_n6avg` field.

## 2. Resonance-score definition

```
S = 0.5 * Jaccard(cross_dse) + 0.2 * n6_prox + 0.2 * bidir + 0.1 * size_balance
  n6_prox   = 1 - |n6_avg_i - n6_avg_j|/100  (weight-renormalize on missingness)
  bidir     = 1.0 bidirectional / 0.5 unidirectional / 0.0 none
  size_balance = min(combos_i,combos_j)/max(...)
For the full 335-domain scan, add W_FJAC=0.15 (formula Jaccard) and renormalize.
```

## 3. Top 10 pairs (combos top-50 domain pool, out of 1,225 pairs)

| # | Domain A | Domain B | S | Jaccard | n6_prox | bidir |
|--:|----------|----------|--:|--------:|--------:|------:|
| 1 | consciousness-scaling | consciousness-training | 0.7109 | 0.429 | 0.983 | 1.0 |
| 2 | corpus-generation | tokenizer-design | 0.6744 | 0.429 | 0.933 | 1.0 |
| 3 | pure-mathematics | cosmology-particle | 0.6624 | 0.400 | 0.992 | 1.0 |
| 4 | consciousness-comm | consciousness-chip | 0.6137 | 0.250 | 0.976 | 1.0 |
| 5 | gpu-lang | embedded-lang | 0.6080 | 0.667 | 0.945 | 0.0 |
| 6 | consciousness-chip | consciousness-scaling | 0.6055 | 0.429 | 0.998 | 0.5 |
| 7 | consciousness-wasm | consciousness-comm | 0.5824 | 0.429 | 0.973 | 0.5 |
| 8 | consciousness-substrate | consciousness-transplant | 0.5792 | 0.429 | 0.957 | 0.5 |
| 9 | consciousness-wasm | consciousness-chip | 0.5728 | 0.429 | 0.949 | 0.5 |
| 10 | (next entry) | | | | | |

### Full 335-domain scan top 10 (with formula Jaccard, S>=0.5 filter, 236 pairs)

| # | Domain A | Domain B | S | Jaccard | formula_Jac |
|--:|----------|----------|--:|--------:|------------:|
| 1 | high-entropy-alloy | steel-metallurgy | 0.7734 | 0.500 | **1.000** |
| 2 | number-theory-deep | elliptic-curves | 0.7245 | 0.667 | 0.000 |
| 3 | hdl | gpu-lang | 0.7048 | 0.667 | 0.000 |
| 4 | cpu-microarchitecture | risc-v-core | 0.6977 | 0.500 | 0.600 |
| 5 | gene-therapy | dna-sequencing | 0.6870 | 0.667 | 0.000 |
| 6 | eda-design-automation | fpga-architecture | 0.6870 | 0.500 | 0.333 |
| 7 | fastener-bolt | welding-technology | 0.6657 | **1.000** | 0.000 |
| 8 | consciousness-comm | consciousness-chip | 0.6641 | 0.250 | **1.000** |
| 9 | asic-design | copper-interconnect | 0.6639 | 0.500 | **1.000** |
| 10 | aluminum-alloy | steel-metallurgy | 0.6511 | 0.200 | **1.000** |

## 4. Statistical significance (z-score)

- **Population**: all 1,225 pair scores in the combos top-50 domain pool
- **Population mean** mu = **0.2755**
- **Population std** sigma = **0.0837**
- **Top-10 mean** = **0.6173**
- **z-score** = (0.6173 - 0.2755) / 0.0837 = **4.08**
- **Empirical p-value** (random 10-pair sampling, N=20,000 resamples): **0.0** (0/20000)
  -> Sampling 10 pairs at random produced a mean >= 0.6173 zero times.

z=4.08 corresponds to two-sided p < 5x10^-5 under normal approximation. The top-10 resonance is drafted as non-coincidental.

## 5. Interpretation

- The **consciousness cluster** (training, scaling, chip, wasm, comm, substrate, transplant) dominates the top of the combos pool — result of cross_dse bidirectional declarations combined with n6_avg proximity.
- **In the full scan, materials/semiconductor/math/biology pairs rise equally**:
  - Materials: high-entropy-alloy <-> steel-metallurgy, aluminum-alloy <-> steel-metallurgy,
    fastener-bolt <-> welding-technology (shared-formula Jaccard 1.0: Z=6, CN=6, hex structure)
  - Semiconductor: hdl <-> gpu-lang, cpu-microarchitecture <-> risc-v-core,
    eda-design-automation <-> fpga-architecture, asic-design <-> copper-interconnect
  - Math: number-theory-deep <-> elliptic-curves (Jaccard 0.667)
  - Life: gene-therapy <-> dna-sequencing
- Pairs with **fjac=1.0** (asic-design x copper-interconnect, aluminum-alloy x steel-metallurgy) share the same n=6 formula set — even when no explicit cross_dse edge is declared, they **converge at the formula level** as a draft. This is the key finding of the 335-domain full-expansion scan.

## 6. Missing-handling bias-avoidance check

- pair_scores.jsonl `miss_n6avg` field: explicit record of domains with n6_avg missing.
- All top-10 pairs **have n6_prox values** (0.933~0.998) — no missingness bias.
- Renormalization fully excludes 50.0 neutral-value substitution.

## 7. Artifacts

| File | Contents |
|------|----------|
| `$NEXUS/shared/dse_cross/pair_scores.jsonl` | full scores of 1,225 pairs in combos top-50 domain pool |
| `$NEXUS/shared/dse_cross/all_pairs_s05.jsonl` | full 335-domain S>=0.5, 236 pairs (with formulas) |
| `$NEXUS/shared/dse_cross/formula_cross.jsonl` | cross-resonance domains per formula pattern |
| `$NEXUS/shared/dse_cross/top10_result.json` | top-10 pairs + z-score + p_empirical |
| `docs/dse-cross-pilot-result.md` | this report (result SSOT) |

## 8. Follow-up tasks

1. 7 domains of the consciousness cluster -> draft target for integration into a single meta-DSE (currently independent runs cause duplicate exploration)
2. fjac=1.0 pairs (materials, semiconductor) -> need PRs to **explicitly add bidirectional cross_dse fields**
3. Pairs with z-score >= 3.0 (estimated top 30) -> draft-candidate BT registration review
