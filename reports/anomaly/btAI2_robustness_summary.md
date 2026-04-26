---
id: btAI2-robustness-seed-sweep-2026-04-26
date: 2026-04-26
type: anomaly-experiment-extension
parent_cycle: BT-AI2
parent_limitation_addressed: btAI2_summary.md §7 item 4 (single-seed limit)
falsifiers_evaluated: [F-AI2-A, F-AI2-B]
verdict: PARTIAL — F-AI2-A fails under realistic threshold/workload combinations
sweep_grid: 100 seeds × 3 workloads × 3 thresholds = 900 paired runs
wall_seconds: 161
---

# BT-AI2 Robustness Extension — Seed Sweep Results

## §0 Headline finding (no-fabrication)

The single-seed (42) BT-AI2 PASS recorded in `btAI2_summary.md` was a
**lucky-seed event**, not a representative measurement. A 100-seed
sweep across three workloads and three thresholds (900 paired runs)
shows F-AI2-A is **breached in 7 of 9 grid cells**. The original
PASS reproduces only in the most lenient threshold band (threshold=4)
where the write-barrier never stalls.

The honesty-triad write-barrier mechanism itself (F-AI2-B, legitimate-
promotion rejection) is **fully robust** — zero rejections across all
900 runs.

This report supersedes the single-point F-AI2-A claim of the parent
cycle. The parent session's MEDIUM verdict (design-HIGH /
silicon-LOW) is consistent with this finding but the *design*-stage
evidence has now weakened: the design clears F-AI2-A only when the
gate threshold is set so low that the barrier never engages.

---

## §1 Composite robustness verdict

| Falsifier | Single-seed (42) | 900-run sweep | Net |
|-----------|-----------------|---------------|-----|
| F-AI2-A — provenance throughput drop ≤ 5% | PASS (4.00%) | **PARTIAL** — passes only at threshold=4 (3/9 cells, all 0% drop); fails in 7/9 cells with seeds breaching the 5% ceiling 18/100 to 96/100 of the time | **PARTIAL** |
| F-AI2-B — write-barrier rejects ≤ 1% legit | PASS (0.0%) | **PASS** — 0.0% legitimate rejection across all 900 runs (max=0, p99=0, no exceeding seeds in any cell) | **PASS** |
| Composite | PASS | **PARTIAL** | **PARTIAL** |

---

## §2 Per-cell distribution (F-AI2-A throughput drop)

| Workload | Threshold | Drop mean | Drop std | Drop p50 | Drop p90 | Drop p99 | Drop max | Seeds > 5% (of 100) |
|----------|----------:|----------:|---------:|---------:|---------:|---------:|---------:|--------------------:|
| attn     | 4         | 0.000     | 0.000    | 0.000    | 0.000    | 0.000    | **0.000** | **0**  ✅ |
| attn     | 8         | 0.030     | 0.029    | 0.040    | 0.067    | 0.067    | 0.067    | 18  ❌ |
| attn     | 12        | 0.055     | 0.044    | 0.067    | 0.125    | 0.126    | **0.176** | **75**  ❌ |
| ffn      | 4         | 0.000     | 0.000    | 0.000    | 0.000    | 0.000    | **0.000** | **0**  ✅ |
| ffn      | 8         | 0.054     | 0.025    | 0.067    | 0.067    | 0.072    | 0.133    | 66  ❌ |
| ffn      | 12        | 0.128     | 0.038    | 0.125    | 0.188    | 0.188    | **0.188** | **90**  ❌ |
| full     | 4         | 0.000     | 0.000    | 0.000    | 0.000    | 0.000    | **0.000** | **0**  ✅ |
| full     | 8         | 0.058     | 0.025    | 0.062    | 0.080    | 0.111    | 0.111    | 58  ❌ |
| full     | 12        | 0.114     | 0.043    | 0.118    | 0.172    | 0.173    | **0.200** | **96**  ❌ |

Mean values are fractions (0.05 = 5%). The single-seed parent run was
`(workload=full, threshold=8, seed=42)` → 4.00% drop. The same cell's
sweep mean is 5.8%, 99-percentile 11.1%, max 11.1% — i.e. seed 42 is
roughly at the 35th percentile of that cell, *below* the 5% ceiling
while 58/100 of its sibling seeds are *above*. The original PASS was
a one-percentage-point margin event on a below-median seed.

---

## §3 Per-cell distribution (F-AI2-B legit-reject rate)

| Workload | Threshold | Reject mean | Reject max | Reject p99 | Seeds > 1% |
|----------|----------:|------------:|-----------:|-----------:|-----------:|
| All 9 cells | 4 / 8 / 12 | **0.000** | **0.000** | **0.000** | **0** |

The promotion-counter MMU model never spuriously refuses a tensor
whose grade meets or exceeds the threshold. F-AI2-B is robust at the
simulator tier.

---

## §4 Sensitivity ranking

The drop in F-AI2-A throughput is monotonic in threshold:

  threshold = 4   → no stalls, drop = 0%        (always PASS)
  threshold = 8   → 18 to 66 seeds breach 5%   (PARTIAL)
  threshold = 12  → 75 to 96 seeds breach 5%   (FAIL)

And monotonic in workload density at the high-threshold band:

  attn (light)  → max 17.6% drop
  full (mixed)  → max 20.0% drop
  ffn  (heavy)  → max 18.8% drop  (interior peak)

The worst single cell is `(full, 12)`: max drop 20.0%, p99 17.3%,
96/100 seeds breach. This combination of a dense workload and a
strict promotion gate is where the design fails.

---

## §5 Reconciling with the parent single-seed PASS

| Property | Single-seed (42) | Sweep (full, 8) | Sweep (worst: full, 12) |
|----------|-----------------:|----------------:|------------------------:|
| Throughput drop | 4.00% | mean 5.8% / max 11.1% | mean 11.4% / max 20.0% |
| Cycles used (provenance) | 100 | varies | varies |
| Verdict on F-AI2-A | PASS (4.00% ≤ 5%) | PARTIAL (58/100 breach) | FAIL (96/100 breach) |

The 4.00% measured in the parent cycle is **inside the empirical
distribution** of the (full, 8) cell, not an outlier. What was
misleading was treating a single seed as decisive. The sweep
confirms the simulator and the parent run are consistent — but also
confirms that a single seed cannot clear a probabilistic falsifier.

---

## §6 Implications for the parent omega-cycle verdict

The parent session
`reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md`
recorded MEDIUM (design-HIGH / silicon-LOW). After this robustness
extension:

- **Design-stage evidence weakens** from "F-AI2-A clears" to
  "F-AI2-A clears only at threshold=4, where the write-barrier never
  stalls — i.e. clears trivially". A meaningful threshold (8 or 12)
  triggers F-AI2-A breaches in 18-96% of seeds.
- **F-AI2-B robustness strengthens**: the write-barrier really does
  not over-reject under any tested seed.
- **Recommended verdict update**: design-MEDIUM (down from
  design-HIGH). Silicon-LOW unchanged. Composite still MEDIUM but on
  weaker design support.
- **Followup cycle proposed**: **BT-AI2c** — investigate why
  threshold=12 yields ~20% drop. Candidates: (a) the simulator's
  hypothesis-write blocking is too aggressive, (b) the workload DAG
  has too few independent tiles, (c) provenance propagation latency
  saturates at high threshold. A counter-design that keeps F-AI2-A
  ≤ 5% at threshold=8 across all 100 seeds is the new gate to clear
  before claiming design-HIGH again.

---

## §7 Atlas / KG / domain references (read-only)

Identical citation set as the parent BT-AI2 run:

- atlas: `n=6 [11*]`, `tau=4 [11*]`, `meta_fp=phi/n=1/3 [10*!]`,
  `provenance_bit_overhead=phi/sigma_n=1/36 [10*]`
- KG: `silicon:provenance-bit`, `silicon:promotion-counter-mmu`,
  `silicon:bt-id-isa`, `principle:honesty-triad`,
  `principle:write-barrier`, `principle:constraint-honesty`,
  `arch:n6-native-accelerator`
- domain: `ai-native-architecture` (axis: compute)

This experiment writes 0 lines to atlas, KG, or domains.json.

---

## §8 Limitations (no-fabrication)

1. Cycle-level only — same simulator as parent BT-AI2; wall delay,
   area, power not modeled.
2. 100 seeds × 9 cells = 900 runs is a sample, not an exhaustive
   sweep. The reported `max` is the maximum *observed*, not a
   guaranteed bound.
3. The "5% ceiling" of F-AI2-A is the parent contract; this sweep
   does not relax it. Re-evaluating the contract itself (e.g.,
   should the budget be 10% at threshold=12?) is a parent-session
   author decision, not this experiment's.
4. The simulator's hypothesis-write blocking semantics may not be
   the only valid realisation of `principle:write-barrier`. Other
   realisations could yield different F-AI2-A profiles.

---

## §9 Audit close

The single-seed F-AI2-A PASS recorded in
`reports/anomaly/btAI2_summary.md` is not contradicted, but it is no
longer sufficient to clear the falsifier. The design clears F-AI2-A
only at threshold=4, where the barrier never engages. Recommended
parent-session amendment: drop design-HIGH to design-MEDIUM, register
BT-AI2c as a follow-on. F-AI2-B remains cleanly cleared. Atlas
writes 0. KG writes 0. domains.json writes 0. README writes 0.
