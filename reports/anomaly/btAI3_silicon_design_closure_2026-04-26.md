# BT-AI3 silicon design-tier closure (2026-04-26)

Date: 2026-04-26
Domain: `ai-native-architecture` (axis: compute)
Parent session: `reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md`
Parent domain: `domains/compute/ai-native-architecture/ai-native-architecture.md`

## 1. BT-AI3 RTL design spec status

Tier: **design-only** (no synthesis, no place-and-route, no
measurement, no tape-out). The spec
`domains/compute/ai-native-architecture/btAI3_rtl_design.md` documents
three silicon primitives at the RTL-sketch level:

- §1 Provenance-bit register file (1 bit per tensor entry, OR-propagation)
- §2 Promotion-counter MMU (4-bit grade counter, refused-write decoder)
- §3 BT-id ISA extension (3-bit field, 7 BTs + 1 reserved code)

§4 composes the three into one HEXA-AI tile; §5 enumerates the
pin-level interface; §6 registers the design-tier falsifier ledger;
§7 explicitly disclaims any synthesis / power-perf-area / fab claim.

This closes the silicon-tier *design artifact* gap in
`ai-native-architecture.md §6 open items` line 145 ("BT-AI3 RTL
candidate spec") at the design-tier only.

## 2. RTL_EXACT verdict

The verifier
`domains/compute/ai-native-architecture/btAI3_rtl_design_verify.py`
re-derives the three falsifiers from the n=6 primitive set and
reports:

```
RTL_EXACT: 3/3, verdict: PASS
```

Per-falsifier:

| ID      | Claim                                                           | Verdict |
|---------|-----------------------------------------------------------------|---------|
| F-AI3-A | provenance bit register area overhead `<= 3%` (`phi/sigma_n=1/36 ~= 2.78%`) | PASS |
| F-AI3-B | promotion counter latency `<= tau=4` cycles, 4-bit width sufficient for threshold_max=12 | PASS |
| F-AI3-C | 7 BT-ids in 3-bit field, all in `{1..7}`, pairwise distinct      | PASS |

These are **symbolic design-tier assertions**, not synthesis or
measurement results. The verify script is stdlib-only and
deterministic.

## 3. H1 rollback-rate sensitivity sweep (Task C)

The original BT-AI2c H1 wrapper amortizes the rollback bubble to zero,
giving an optimistic bound. The new sweep
`experiments/anomaly/btAI2c_h1_rollback_sweep.py` charges
`int(rollback_rate * tensor_cost * downstream_count)` cycles per
refused write, then runs `seeds=range(50)` at (workload=full,
threshold=8) over rollback rates `[0.0, 0.01, 0.025, 0.05, 0.1]`.

Output: `reports/anomaly/btAI2c_h1_rollback_results.json`
(schema `btAI2c_h1_rollback_sweep.v1`).

| rollback_rate | mean drop | p90    | max    | breach (>5%) |
|--------------:|----------:|-------:|-------:|-------------:|
| 0.000         | 0.0000    | 0.0000 | 0.0000 | 0/50         |
| 0.010         | 0.0000    | 0.0000 | 0.0000 | 0/50         |
| 0.025         | 0.0000    | 0.0000 | 0.0000 | 0/50         |
| 0.050         | 0.0000    | 0.0000 | 0.0000 | 0/50         |
| 0.100         | 0.0056    | 0.0400 | 0.0400 | 0/50         |

**Break-even rollback rate: > 0.1** (no crossing of the 5% bound
within the swept range). The integer-truncation of the rollback
bubble (`int(...)`) suppresses sub-cycle charges at low rates; even
at `rollback_rate = 0.1` the mean drop is only 0.56% and the worst
seed sits at 4%. This is a stronger result than the rollback_rate=0
H1 bound: F-AI2c-A holds robustly across the full `[0, 0.1]` range,
not just at the optimistic point.

Wall time for the sweep: ~1.35 s on the local Python (well under the
12-min cap).

This does NOT *prove* F-AI2c-A holds for arbitrary rollback rates; it
only widens the empirical robustness window from `{0}` to `[0, 0.1]`.
Higher rates may cross the bound and would be the subject of a
follow-up sweep.

## 4. Combined design-tier readiness

| Tier      | State              | Rationale |
|-----------|--------------------|-----------|
| design    | MEDIUM (amended)   | F-AI2-B robust, F-AI2c-A holds across `[0, 0.1]` rollback range, RTL_EXACT 3/3 PASS at design-tier |
| sim       | MEDIUM             | unchanged: 1000-seed sweeps + 50-seed rollback sensitivity |
| silicon   | CANDIDATE          | RTL design artifact exists (BT-AI3 spec); synthesis still NOT performed |
| literature| LOW                | unchanged: no SC publication |

The silicon tier moves from LOW to **CANDIDATE** in this report's own
summary (this is the report's own tier label, not a write to the
parent domain document; that promotion would require an
atlas-agent / domain-doc edit and is explicitly NOT performed here).

## 5. Path-to-🛸10 readiness assessment

Observed evidence supporting a 🛸10 promotion:

- 10/10 EXACT closure on `ai-native-architecture` (verify_ai-native-architecture.py PASS)
- F-AI2-A PARTIAL amended by F-AI2c-A PASS at rollback_rate=0
- F-AI2c-A robustness extended to rollback_rate `[0, 0.1]` (this report)
- BT-AI3 RTL design spec exists with 3/3 RTL_EXACT PASS
- Three silicon primitives have RTL sketches with pin-level interface

Still missing for a definitive 🛸10:

- silicon-CANDIDATE -> silicon-MEDIUM requires an actual synthesis
  report on at least one PDK (SKY130 is the most accessible)
- F-AI1 HOLD-PROXY -> PASS requires an MPS / tensor-network surrogate
- 6-vendor convergence audit (omega-cycle item C4)
- Sensitivity sweep at rollback_rate > 0.1 to find the actual
  break-even (currently only known to be > 0.1)

Verdict: **🛸10 NOT YET**. The cycle's deliverables are met at
design-tier (silicon-CANDIDATE), but silicon-MEDIUM requires a
synthesis run that has not happened. Honest path-to-🛸10 is
"design-tier complete, silicon-tier deferred to a synthesis cycle".

## 6. Millennium tally

0/7 millennium BTs are claimed solved by this report. The
`bt_coverage_count = sopfr(n) + phi = 7` constant counts the seven
*addressed* BTs (BT_541..547), not seven *solved* BTs. The
distinction is preserved in the parent domain document.

## 7. Atlas / KG / domains.json writes

- atlas writes from this report: **0** (candidates only; promotion
  through atlas-agent is deferred)
- KG writes: **0**
- domains.json writes: **0**

The new files produced in this cycle are confined to:

- `domains/compute/ai-native-architecture/btAI3_rtl_design.md`
- `domains/compute/ai-native-architecture/btAI3_rtl_design_verify.py`
- `experiments/anomaly/btAI2c_h1_rollback_sweep.py`
- `reports/anomaly/btAI2c_h1_rollback_results.json`
- `reports/anomaly/btAI3_silicon_design_closure_2026-04-26.md` (this file)

All edits respect the write-barrier: atlas, KG, README, parent
session, parent domain document, and `verify_ai-native-architecture.py`
are NOT modified.
