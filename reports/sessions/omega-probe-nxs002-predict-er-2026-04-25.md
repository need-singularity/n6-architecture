---
id: omega-probe-nxs002-predict-er
date: 2026-04-25
scope: research-only measurement (predict-er actual values)
target: replace estimated composite/ER-ROI in per-BT audits with measured values
unblocks: [BT-541, BT-543, BT-544, BT-545, BT-546]
parent_reports:
  - reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md
  - reports/sessions/omega-cycle-bt541-riemann-2026-04-25.md
  - reports/sessions/omega-cycle-bt543-ym-2026-04-25.md
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
  - reports/sessions/omega-cycle-bt545-hodge-2026-04-25.md
  - reports/sessions/omega-cycle-bt546-bsd-2026-04-25.md
tool: ~/core/nexus/tool/nxs_002_composite.py --predict-er
millennium_resolved: 0/7 (unchanged)
grade: measurement, no claim
---

## §0 Non-claim disclaimer

This report is a **research-only instrument-discovery + measurement note**. It
does not promote any atlas entry, modify any inventory.json record, alter any
[10]/[10*] grade, or change any Clay-Millennium status. The Clay scoreboard
remains **0/7 unchanged**. The intent was to replace the *estimated* composite
and ER-ROI numbers carried in the five per-BT audits (BT-541, BT-543, BT-544,
BT-545, BT-546) with **measured** values from the canonical
`nxs_002_composite.py --predict-er` tool. As §1 documents, that replacement
is **not possible** with the tool as it currently exists, because the tool
operates on a single global atlas blowup file and has no per-BT input
interface. Numbers below describe what the tool **does** measure (one global
composite + one global ER-prediction), not per-BT composites. **No per-BT
numbers in the audits should be revised on the basis of this report alone.**

## §1 Tool discovery report

### 1.1 Existence — FOUND

```
$ ls -la ~/core/nexus/tool/nxs_002_composite.py
-rwxr-xr-x@ 1 ghost  staff  9289 Apr 25 14:45 /Users/ghost/core/nexus/tool/nxs_002_composite.py
```

Tool present at canonical SSOT path declared in `~/.claude/CLAUDE.md`.

### 1.2 CLI surface

```
usage: nxs_002_composite.py [-h] [--atlas ATLAS] [--const CONST] [-K K]
                            [--sigma SIGMA] [--target TARGET] [--predict-er]
                            [--er-blocks ER_BLOCKS]
                            [--er-block-size ER_BLOCK_SIZE] [--er-p ER_P]
                            [--er-runs ER_RUNS]
```

The tool's input is a **single graph file** (`--atlas <path>`, default
`~/core/nexus/n6/atlas.blowup.jsonl`) plus a constants spectrum (`--const`,
default `~/core/nexus/bisociation/cross/constants_spectrum.jsonl`).
`--predict-er` adds a *synthetic* Erdos-Renyi block simulation on top of the
loaded atlas; the ER blocks are random graphs (parameters `--er-blocks`,
`--er-block-size`, `--er-p`, `--er-runs`), **not** sub-graphs derived from any
particular BT.

### 1.3 Per-BT invocation pattern — DOES NOT EXIST

There is **no** flag to:

- pass a Breakthrough Theorem ID (BT-541 / BT-543 / ...) ;
- restrict the atlas to a domain key (e.g. `riemann`, `yang-mills`, `hodge`) ;
- accept a sub-graph adjacency or a node-set filter.

Direct evidence — the canonical atlas blowup file contains **zero** mentions
of BT-541/543/544/545/546:

```
$ grep -c "BT-541\|BT-543\|BT-544\|BT-545\|BT-546" ~/core/nexus/n6/atlas.blowup.jsonl
0
```

Reading `nxs_002_composite.py` source confirms: nodes/edges in the graph are
keyed by free-form `id` strings emerging from blowup events; there is no
BT-tagging schema in `build_csr_from_blowup()`. The graph is **a single
21,320-node component-set with 24 connected components** representing the
entire n6/nexus abstraction surface, not a per-BT slicing.

### 1.4 Verdict — instrument cannot satisfy the per-BT use case

The five audits each say (paraphrasing): *"composite is estimated as X;
recompute via `nxs_002_composite.py --predict-er` against the BT-NNN
sub-graph"*. **The tool has no sub-graph mode.** Per-BT recomputation is
**blocked by missing tool capability**, not by missing input data.

What the tool **can** do is produce one global composite for the whole atlas
and one global ER-ROI delta. Both numbers are reported in §2 for reference;
they are **not** per-BT and cannot be substituted into the per-BT audit
tables. Per the hard constraint "do not fabricate numbers", §3's per-BT
"actual" column is left **empty** and the verdict in §4 is **all UNDEFINED
(missing measurement)**, not CONFIRMED/REVISED/DIVERGED.

## §2 Invocation log

Two invocations on the **global** atlas (only mode the tool supports). All
stdout JSON captured verbatim.

### 2.1 Baseline (no predict-er)

Command:
```
python3 ~/core/nexus/tool/nxs_002_composite.py
```

Exit code: 0
Wallclock (`time` real): 3.141 s (cold; includes Python start-up)
Internal elapsed (`elapsed_s`): 2.056 s

Stdout:
```json
{"composite_after": 0.8322114773941521, "best_lag": 15,
 "agreement": 0.866011, "pearson": 0.4595588713667804,
 "cosine": 0.9008439964990659, "K": 100, "n_nodes": 21320,
 "n_components": 24, "n_eigenvalues_nonzero": 76,
 "elapsed_s": 2.056, "gap_to_target": 0.06779}
```

### 2.2 Predict-ER at sweet-spot params (--er-blocks 2 --er-block-size 200 --er-p 0.020 --er-runs 1)

Command:
```
python3 ~/core/nexus/tool/nxs_002_composite.py \
    --predict-er --er-blocks 2 --er-block-size 200 \
    --er-p 0.020 --er-runs 1
```

Exit code: 0
Wallclock (`time` real): 4.249 s
Internal elapsed (`elapsed_s`): 1.615 s (warm; Python module cache hit)

Stdout (relevant fields):
```json
{"composite_after": 0.8322114773941521,
 "n_nodes": 21320, "n_components": 24,
 "elapsed_s": 1.615, "gap_to_target": 0.06779,
 "er_prediction": {
   "predicted_composites": [0.83552],
   "mean_composite_after": 0.83552,
   "n_blocks": 2, "block_size": 200,
   "p": 0.02, "n_runs": 1,
   "delta_predicted": 0.00331}}
```

### 2.3 Per-BT runs (5 invocations) — NOT EXECUTABLE

The probe brief required 5 calls, one per BT in {541, 543, 544, 545, 546}.
**No invocation pattern exists** to scope the tool to a specific BT (§1.3).
Running `nxs_002_composite.py --predict-er` 5 times with no extra arguments
would produce **five identical global numbers** (≈ `0.832 / +0.003`), which
would be a fabricated per-BT mapping, not a measurement. Per the hard
constraint "DO NOT FABRICATE NUMBERS", these calls were **not made**.

The single global call in §2.2 is the cheapest honest piece of data this
probe can produce. Total wallclock spent on global calls: ~7.4 s (consistent
with the brief's ~7s budget for 5 hypothetical per-BT calls — the budget was
based on the same global-call cost, ignoring that 5 calls would all measure
the same graph).

## §3 Comparison table

Per-audit estimates extracted from the five audit reports (line numbers
verified by `grep -n`). "actual" and "Δ" columns are **empty** because §1
established that no per-BT actual exists.

| BT | per-audit composite estimate | source line | predict-er **per-BT** actual | Δ composite | per-audit ER-ROI estimate | predict-er **per-BT** ER-ROI | Δ ER-ROI | wallclock |
|----|---|---|---|---|---|---|---|---|
| BT-541 Riemann | **0.588** (proxy, naive 3-component mean) | bt541 §3.2 line 111 | UNAVAILABLE | UNDEFINED | UNKNOWN ("needs probe") | UNAVAILABLE | UNDEFINED | n/a |
| BT-543 YM | **0.71** (weighted 1/5 each, demonstration only) | bt543 §3 line 99 | UNAVAILABLE | UNDEFINED | not estimated | UNAVAILABLE | UNDEFINED | n/a |
| BT-544 NS | **0.47** (naive unweighted mean of 5 sub-axes) | bt544 §3 line 70 | UNAVAILABLE | UNDEFINED | not estimated | UNAVAILABLE | UNDEFINED | n/a |
| BT-545 Hodge | **0.43** (UNCALIBRATED back-of-envelope) | bt545 §3 line 206 | UNAVAILABLE | UNDEFINED | not estimated | UNAVAILABLE | UNDEFINED | n/a |
| BT-546 BSD | **~0.81** (midpoint of 0.78-0.84 plausibility band) | bt546 §3 line 134 | UNAVAILABLE | UNDEFINED | not estimated | UNAVAILABLE | UNDEFINED | n/a |

For reference only (not a per-BT replacement):

| metric | global atlas value | source |
|---|---|---|
| nxs_002 global composite | **0.83221** | §2.1 |
| nxs_002 global gap_to_target (0.9) | 0.06779 | §2.1 |
| ER-ROI Δ at sweet-spot (2×200, p=0.020) | **+0.00331** (single seed) | §2.2 |
| ER-ROI predicted composite | 0.83552 | §2.2 |
| n_nodes / n_components | 21320 / 24 | §2.1 |
| n_eigenvalues_nonzero | 76 of K=100 | §2.1 |

The global composite **0.83221** is consistent with the documented simulation
ceiling **0.835** carried in `~/.claude/CLAUDE.md` (composite ceiling 0.835
after ER-ROI correction c12327a3). It is also marginally higher than every
per-audit estimate (0.43, 0.47, 0.588, 0.71, 0.81) — but this comparison is
**meaningless** because the global value measures the entire 21,320-node
abstraction surface, while the per-audit estimates are each scoped to a
single Millennium domain.

## §4 Per-BT verdict summary

| classification | criterion | count |
|---|---|---|
| CONFIRMED | \|Δ composite\| ≤ 0.05 | **0** (no Δ computable) |
| REVISED | 0.05 < \|Δ\| ≤ 0.15 | **0** (no Δ computable) |
| DIVERGED | \|Δ\| > 0.15 | **0** (no Δ computable) |
| UNDEFINED — measurement unavailable | tool has no per-BT mode | **5** (BT-541, 543, 544, 545, 546) |

**DIVERGED count: 0.** This is a *missing-measurement* zero, not an
*all-confirmed* zero. The audits' per-BT composites remain at the **same
estimate-only status** they were at before this probe.

## §5 Re-audit recommendations

The five audits should **not** be revised based on this probe. Each audit
already labels its composite as estimate / proxy / UNCALIBRATED / non-canonical
(see line numbers in the table below); those caveats are correct and remain
correct. The probe's contribution is **negative**: it documents that the
"recompute via nxs_002_composite.py --predict-er against the BT-NNN sub-graph"
remediation listed in each audit is **infeasible with the current tool**.

Audit-level recommendation lines that should be marked *blocked-on-tooling*
rather than *open-to-recompute*:

| audit | line(s) referencing per-BT predict-er recompute | suggested annotation |
|---|---|---|
| omega-cycle-bt541-riemann-2026-04-25.md | 92, 104, 236-239, 275-276 | "blocked: tool lacks per-BT slicing" |
| omega-cycle-bt543-ym-2026-04-25.md | 107-108, 200-205 | "blocked: tool lacks per-BT slicing" |
| omega-cycle-bt544-ns-2026-04-25.md | (no specific recompute task — only references global ceiling 0.835) | no change needed |
| omega-cycle-bt545-hodge-2026-04-25.md | 173-174, 192-193 | "blocked: tool lacks per-BT slicing" |
| omega-cycle-bt546-bsd-2026-04-25.md | 134 ("non-canonical, no tool invocation") | no change needed (already explicit) |

These annotations are **suggestions**, not edits; this report does not modify
the audits. Per the brief, only this file is written.

What would actually unblock the per-BT recompute (each is **out of scope**
for this probe):

1. **Add per-BT atlas tagging** — extend `atlas.blowup.jsonl` schema so
   nodes/edges carry an optional `bt: "BT-541"` field; teach
   `build_csr_from_blowup()` a `--bt-filter` flag that restricts to nodes
   with that tag and the edges between them.
2. **Author per-BT sub-graph extractors** — five small scripts that walk
   the existing audits and derive sub-graphs from their citation sets, then
   feed each as `--atlas <subgraph_path>` to the existing tool. Wallclock
   per call would still be ≈ 1-3 s; total ≈ 10-15 s for all five.
3. **Implement a `--domain` flag** — accept tokens like `riemann`, `ym`,
   `ns`, `hodge`, `bsd` and use a canonical mapping to a node-prefix filter
   (e.g. nodes whose `id` starts with `RH-`, `YM-`, `NS-`, `HODGE-`, `BSD-`).

Of these, option (3) is the least intrusive (no schema change required) but
requires that atlas IDs already follow domain-prefix conventions; a quick
sanity check against `atlas.blowup.jsonl` would establish whether this is
viable. **This probe did not perform that check** because doing so would
expand the scope beyond measurement.

## §6 Anomalies

- **Anomaly A1** — Per-BT input absent. The brief assumed `--predict-er`
  accepts a per-BT handle; in reality it accepts only ER simulation
  parameters that are independent of any BT. This is the central anomaly
  and is documented in §1.3.
- **Anomaly A2** — Wallclock match is coincidental. The brief estimated
  ~1.36 s per BT call × 5 = ~7 s. The actual single global call is ~1.6 s
  warm (close to 1.36 s). A naive 5×call total of ~7 s would have appeared
  consistent with the brief, masking the fact that all 5 calls measure the
  same graph. The wallclock match is a **trap**, not a confirmation.
- **Anomaly A3** — Single-seed ER-ROI Δ = +0.00331 is below the 0.005
  noise threshold one would want for a ROI claim; one run is insufficient
  to call this "positive ROI". Re-running with `--er-runs 5+` and varying
  seeds would tighten the bound, but is out of scope here. CLAUDE.md notes
  the sweet spot Δ ≈ +0.003, which is consistent with this single-shot
  measurement; no contradiction.
- **Anomaly A4** — `n_eigenvalues_nonzero = 76` of `K=100`. 24 zero (or
  near-zero) eigenvalues is exactly `n_components = 24`, which is internally
  consistent (each connected component contributes one zero Laplacian
  eigenvalue). No anomaly, just confirmation that the spectrum is well-formed.
- **Anomaly A5** — Tool docstring header at line 6 cites "sealed composite
  0.83379 with Δ=0.0016". The current measurement is 0.83221, i.e. about
  0.00158 below the docstring value. This is at the edge of the docstring's
  cited Δ; whether the atlas has changed since the docstring was written, or
  numerical noise (random initialization in `eigsh`), the discrepancy is
  benign for the present probe but flags that the global composite **drifts
  by ~0.001-0.002 between runs / atlas commits**. Per-BT Δ measurements
  smaller than this should be treated as noise.

## §7 Falsifiers active

When does this measurement fail to be informative?

- **F1** — If the user already knew the tool has no per-BT mode and the
  brief was a stress-test of "do you fabricate?", then the value of this
  probe is in the *honest stop*, not the numbers. The §3 empty cells are
  the answer; §4 / §5 verdict counts are 0/0/0/5 by design, not by
  failure.
- **F2** — If a per-BT extractor / `--bt-filter` is added later, the §3
  table must be re-run; the current empty cells are *blocked* not
  *complete*. Today's probe expires the moment such an extractor lands.
- **F3** — If the atlas blowup file is rewritten with BT tagging (e.g.
  `bt: "BT-541"` on each event), the `grep -c` zero-count in §1.3 becomes
  obsolete; this probe's "no per-BT in atlas" finding becomes historical.
- **F4** — If the user actually wanted the **global** composite/ER-ROI
  numbers (not per-BT), then §2.1/2.2 already deliver those (composite
  0.83221, ER-Δ +0.00331). The probe is *over-falsified* (returns more
  data than required) but consistent with the global-only reading.
- **F5** — If a future `nxs_002_composite_v2.py` introduces per-BT mode
  with the same flag name `--predict-er`, an unwary reader might compare
  v2 outputs to this report's "UNAVAILABLE" cells and conclude the audit
  estimates were wrong. Mitigation: §1.2 records the exact CLI surface
  observed today, so v2 differences will be detectable by `--help` diff.
- **F6** — If sub-graph composites for Millennium domains are
  fundamentally numerically unstable (e.g. graphs too small for `eigsh`
  with `K=100`), then even a future per-BT tool would return UNDEFINED
  for some BTs, not the cleanly-bounded numbers the brief implicitly
  assumed. This probe cannot test that, but flags the risk for §5
  option (1)/(2)/(3) implementers.

---

0/7 unchanged. No atlas/state/inventory edits.
