# experiments axis — 3rd expansion report: 150 -> 200 (50 new)

Date: 2026-04-11
Session: experiments-expansion-150-200
Operator: Claude Opus 4.6 (1M context)
Basis: user directive — experiments/ axis 3rd expansion (150 -> 200, 50 new .hexa)

## Background

- 1st base 115 -> 2nd 143 (expansion-122-150 agent #7 done, ai-efficiency 10 + anomaly 10 + cross 4 + structural 4 = 28 new)
- 3rd target 50 new -> reach 193 (file-basis 200+), further filling verification gaps
- All are .hexa STUBs; real measurement logic runs in follow-up `nexus verify --hexa` cycles (R18 minimal)

## Generation Principles (compliance rules)

- R1 HEXA-FIRST: zero .py, only .hexa files
- Korean-required-comments policy: 100% Korean comments / print lines in source file headers (legacy policy constraint)
- R14 SSOT: append pending rows to single SSOT `_results.jsonl`; no separate JSON emitted
- R18 minimal: fn main() stub + metadata comment only; real implementation follows
- Follow existing stub pattern: metadata comment header + 3 println lines
- Validation via n6shared/config/absolute_rules.json + n6shared/rules/n6-architecture.json

## Distribution of 50 New Experiments

| Category | Count | Priority key | Purpose |
|---|---|---|---|
| anomaly (BT-381~400) | 20 | bt400-new | verify new breakthroughs: dark matter / dark energy / quantum gravity / consciousness / information theory / complex systems / ecology / social physics / economic physics / network |
| chip-verify | 10 | chip-design | Xn6 ISA 24 ops, RTL 6-stage, NPU 6x6, vector 12 lanes, cache 6 levels, NoC 6 ports, RF 24 entries, issue 6, branch predictor 6, power gating 6 states |
| ai-efficiency (SOTA) | 6 | sota-bench | Mamba-2 2 + Hyena 2 + RWKV-6 2 |
| lens-verify | 10 | lens-verify | causal discovery / topological data / spectral / manifold / Bayesian / optimal transport / attractor / persistent homology / information bottleneck / variational |
| meta | 4 | meta-ops | atlas promotion automation, convergence refresh, DSE v2 regression, MC v9 cross-batch |
| Total | 50 | — | — |

## Per-category Details

### anomaly (20) — BT-381~400 frontier new breakthroughs

BT-381 WIMP ladder / BT-382 axion window / BT-383 Omega_Lambda=24/35 / BT-384 LQG spin foam /
BT-385 AdS6/CFT5 / BT-386 IIT Phi_max / BT-387 GWT broadcast / BT-388 Shannon grand unification /
BT-389 Kolmogorov / BT-390 SOC exponent / BT-391 BA network / BT-392 trophic level /
BT-393 Dunbar / BT-394 Zipf-Pareto / BT-395 market tail / BT-396 May diversity /
BT-397 urban scaling / BT-398 Fisher / BT-399 game theory / BT-400 10-frontier meta

### chip-verify (10) — new category (Xn6 chip-design verification)

1. `verify_xn6_isa_24ops.hexa` — 24-instruction (J2) completeness
2. `verify_rtl_6stage_pipeline.hexa` — 6-stage pipeline PPA
3. `verify_npu_systolic_6x6.hexa` — 6x6 systolic utilization
4. `verify_xn6_vector_width.hexa` — 12-lane Pareto
5. `verify_dse_cache_6level.hexa` — 6-level cache DSE
6. `verify_xn6_interconnect_6port.hexa` — 6-port NoC
7. `verify_xn6_regfile_24_entries.hexa` — 24 RF entries
8. `verify_xn6_issue_width_6.hexa` — 6-issue superscalar
9. `verify_xn6_branch_predictor.hexa` — 6-bit-history BHT
10. `verify_xn6_power_gating_6.hexa` — 6-state power management

### ai-efficiency (6) — SOTA bench, 3 families x 2 each

1. `experiment_mamba2_ssm_bench.hexa` — Mamba-2 LongBench/MMLU
2. `experiment_mamba2_scaling_n6.hexa` — Mamba-2 Chinchilla n=6 family
3. `experiment_hyena_long_conv.hexa` — Hyena order O=6 scan
4. `experiment_hyena_bench_sota.hexa` — Hyena LRA/PILE
5. `experiment_rwkv6_time_mix_n6.hexa` — RWKV-6 time-mix 6 coefficients
6. `experiment_rwkv6_bench_sota.hexa` — RWKV-6 OpenLLM

### lens-verify (10) — new category (representative verification of the 56 secondary lenses)

1. `verify_causal_discovery_lens.hexa` — CausalDiscoveryLens
2. `verify_topological_data_lens.hexa` — TopologicalDataLens
3. `verify_spectral_graph_lens.hexa` — SpectralGraphLens
4. `verify_manifold_learning_lens.hexa` — ManifoldLearningLens
5. `verify_bayesian_inference_lens.hexa` — BayesianInferenceLens
6. `verify_optimal_transport_lens.hexa` — OptimalTransportLens
7. `verify_attractor_basin_lens.hexa` — AttractorBasinLens
8. `verify_persistence_homology_lens.hexa` — PersistenceHomologyLens
9. `verify_information_bottleneck_lens.hexa` — InformationBottleneckLens
10. `verify_variational_inference_lens.hexa` — VariationalInferenceLens

### meta (4) — new category (meta ops)

1. `meta_atlas_promotion_automation.hexa` — atlas.n6 [7]->[10*] auto promotion
2. `meta_convergence_refresh.hexa` — convergence/n6-architecture.json sync
3. `meta_dse_v2_regression.hexa` — DSE v2 Pareto regression
4. `meta_mc_v9_cross_batch.hexa` — MC v9 50-batch confidence estimate

## _results.jsonl Update

Keep existing 143 rows; append 50 pending rows at the end.

Per-row schema:

```json
{"name": "...", "category": "...", "file": "...", "status": "pending", "batch": "expansion-150-200", "priority": "bt400-new | chip-design | sota-bench | lens-verify | meta-ops"}
```

Total rows: 143 + 50 = 193 (verify via `wc -l`).

## New Category Directories

- `experiments/chip-verify/` + CLAUDE.md
- `experiments/lens-verify/` + CLAUDE.md
- `experiments/meta/` + CLAUDE.md

Existing ai-efficiency / anomaly directories receive simple file additions. INDEX.json `experiments.subs` update will be batched later (this session is limited to .hexa + _results.jsonl).

## Forward Path

1. Port top-priority stubs in order (bt400-new -> chip-design -> sota-bench) to real implementations
2. On result availability, `status: pending` -> `passed` / `failed` / `inconclusive`
3. Passed BT-381~400 items become ossification-promotion candidates in `n6shared/convergence/n6-architecture.json` and `atlas.n6` [7]->[10*] edit candidates
4. chip-verify passed -> adopt as `nexus rtl` pipeline passing evidence
5. Fully implement `meta_atlas_promotion_automation.hexa` auto-promotion script to close the loop

## Constraints and Notes

- R5 SSOT: this report lives only in `reports/sessions/`, no duplicates
- no direct atlas.n6 edits (only after verification passes, at promotion stage)
- no actual .hexa changes in techniques/ (this work is experiments-axis only)
- 0 .py files written; all .hexa (R1 passes)
- 100% Korean-comment policy passes (source-file local rule)
- INDEX.json `experiments.subs` ["ai-efficiency","structural","cross","anomaly"] must also receive chip-verify/lens-verify/meta additions (follow-up task)
