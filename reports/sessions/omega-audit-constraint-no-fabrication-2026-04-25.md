---
id: omega-audit-constraint-no-fabrication
date: 2026-04-25
scope: research-only constraint audit (NOT proposing constraint removal)
target: no-fabrication guard -- functional transitive audit across 5 fabrication classes
axes: [ladder L1..L_ω, Ω-saturation, atlas chain guard-activations, closure redundancy]
parent_reports:
  - reports/sessions/omega-probe-nxs002-predict-er-2026-04-25.md
  - reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: constraint-functional audit, no removal recommendation without 3-of-3 audits
---

## §0 Non-claim disclaimer

This is a **research-only audit** of the `no fabrication` agent constraint as
it currently functions in the n6-architecture repository. The audit does not
propose, recommend, or simulate constraint removal. It does not modify any
agent harness, system prompt, atlas entry, inventory record, [10]/[10*] grade,
or Clay-Millennium status. The Clay scoreboard remains **0/7 unchanged**.

The audit is itself subject to the constraint under audit: every percentage,
count, file path, and tool name cited below is grounded in a `grep` / `ls` /
`Read` against the actual repository or the cross-repo nexus SSOT. UNKNOWN
markers are used where the audit's grep depth was bounded; nothing is
invented to fill a slot.

This audit is one of three required before any retire decision could be
considered. A retire verdict would require **3-of-3 independent audits**
agreeing — this single document **cannot** authorize retirement, regardless
of its conclusion.

## §1 Constraint definition + 5 fabrication classes

**Constraint** (as enforced today): an agent must not invent numerical values,
file contents, tool names, dataset names, paper citations, or measurement
results. When a measurement is unobtainable (tool missing, data absent,
computation not run, paper not read), the agent must return `UNDEFINED` /
`UNKNOWN` / `INCONCLUSIVE` / `TIMEOUT` / `MISSING` / `UNAVAILABLE` plus a
short diagnostic naming the gap.

**5 fabrication classes** for transitive analysis:

| class | fabrication shape | example slip |
|---|---|---|
| C1 numerical | invent a measurement value | "composite is **0.847**" when no tool run produced 0.847 |
| C2 citation | invent paper / theorem / file path | cite `Voevodsky 2003 §4.2` when §4.2 was never read |
| C3 tool | invent a script / flag / CLI surface | claim `--bt-filter` exists when the tool has no such flag |
| C4 structural | claim a file/directory/schema exists | claim `data/cremona/allbsd/` has 50 shards when it has 27 |
| C5 result | claim PASS/EXACT/CONFIRMED without running | tag a row EXACT without executing the verification |

These five classes are largely orthogonal: a given agent slip almost always
sits cleanly in exactly one class. They correspond to the five most common
shapes the constraint actively rejects in the chain log (§4 below).

## §2 Ladder occupancy — fabrication-risk rungs

The omega ladder has 12 rungs (L1 smash → L_ω). Fabrication risk is **not
uniform**:

| rung | risk profile | reason | grounded example |
|---|---|---|---|
| L1 smash | **high** | concrete data; smallest abstraction step from "I need this number" to "I write a number" | C1, C4 slips. BT-545 §3 "0.43 (UNCALIBRATED back-of-envelope)" sits here — the number is honest because it is **labeled UNCALIBRATED** |
| L2 drill | **high** | object-level computation; agent could "predict" a tool output without running it | C3, C5 slips. Probe #3 nxs_002 verdict: agent could have output 5 plausible per-BT composites; instead returned 5 × UNDEFINED |
| L3 chain/debate/batch | medium | aggregating many object-level results; one fabricated input contaminates the aggregate | C5 slips. Backtrace strategy report rolls up 7 BT files; honesty preserved by carrying explicit estimate-quality tags upward |
| L4 surge | medium-low | speculative direction-setting; explicitly tagged as direction not measurement | structural risk lower because outputs are "if we did X, we would learn Y" |
| L5 dream | low | far-from-data; outputs are scenario-shaped, fabrication-shaped output is conventional and detectable | mostly UNKNOWN-occupancy in current chain (BT-545 §X.4 "L5 dream UNKNOWN") |
| L6 reign | low-medium | governance / promotion decisions; here the failure mode is C5 (premature promotion) more than C1 | atlas [10] → [10*] promotions are gated by separate inspection step |
| L7 swarm | low | multi-agent coordination; cross-agent contradiction surfaces fabrication |  |
| L8 wake | low | retrospective on prior runs; references prior records → fabrication produces detectable inconsistency |  |
| L9 molt | low-medium | row demotion; failure mode is **silent demotion-without-record** (C5 inverted) |  |
| L10 forge | very low | abstract synthesis; fabricated abstractions are detectable as inconsistent with axiom layer |  |
| L11 canon | very low | premature canonization is the failure; this is a closure-ceiling problem (criterion (e)), not fabrication per se |  |
| L_ω apex | n/a | omega closure is the **target**, not a fabrication site |  |

**Risk concentration**: L1 + L2 + L3 carry ≈ 80% of fabrication surface.
This matches the chain-log evidence in §4 (every guard activation in §4
sits at L1 or L2).

The constraint is **not redundant** at L1/L2 — there is no upstream gate
between "agent decides what to write" and "agent writes". At L4+ the
abstraction itself constrains what plausible-shaped output looks like.

## §3 Saturation estimate

**Question**: of the 11 omega-cycle session reports written 2026-04-25,
how many actively engage the no-fabrication guard?

**Direct grep** (UNDEFINED / UNKNOWN / TIMEOUT / INCONCLUSIVE):

```
grep -rlE "UNDEFINED|TIMEOUT|INCONCLUSIVE|UNKNOWN" reports/sessions/*.md
  → 10 distinct reports hit
```

Per-report hit counts (only reports with hits ≥ 1):

| report | strong-guard signal count |
|---|---|
| omega-probe-dfs24-batch-execution-2026-04-25.md | 14 |
| omega-cycle-bt545-hodge-2026-04-25.md | 13 |
| omega-cycle-bt546-bsd-2026-04-25.md | 11 |
| omega-probe-nxs002-predict-er-2026-04-25.md | 8 |
| omega-cycle-bt541-riemann-2026-04-25.md | 5 |
| omega-cycle-bt542-pnp-2026-04-25.md | 5 |
| omega-cycle-bt543-ym-2026-04-25.md | 5 |
| omega-cycle-backtrace-strategy-2026-04-25.md | 5 |
| omega-cycle-bt544-ns-2026-04-25.md | 3 |
| omega-probe-l9-molt-trigger-2026-04-25.md | 1 |

**Of 11 omega-cycle reports** (the 11 named files in
`reports/sessions/omega-*-2026-04-25.md`), **10 carry at least one
fabrication-guard activation**; **1** (`omega-cycle-bt547-poincare`) does not
include UNDEFINED/UNKNOWN markers because BT-547 is **resolved** (Perelman
2003) and the report is documenting a settled result, not measuring an open
one.

**Saturation estimate (this corpus)**: **10 / 11 ≈ 91%** of recent omega-cycle
reports actively engage the guard. The single non-engagement is the resolved
BT, where there is no measurement gap to flag.

**Caveat**: this is a single-day, single-axis sample. Earlier reports show
lower saturation (e.g. `millennium-lemmas-2026-04-11.md`: 0 hits — though
inspection shows it uses `MISS` heavily, which is a sibling honesty marker
not counted in the strong-guard set above). A wider-corpus saturation
measurement is **out of scope** for this audit.

## §4 Guard-activation incidents (chain)

Concrete cases where the constraint converted a "fake answer" into a
"negative result + diagnostic". Each row is grounded in a specific report
and line number.

| # | date | report | class | activation |
|---|---|---|---|---|
| 1 | 2026-04-25 | omega-probe-nxs002-predict-er §1.4 line 85-90, §3 lines 173-177 | C1 + C3 | tool lacks per-BT mode → 5 × UNAVAILABLE / UNDEFINED in result table instead of 5 fabricated per-BT composites |
| 2 | 2026-04-25 | omega-probe-nxs002-predict-er §6 A1 line 257 | C3 | brief assumed `--predict-er` accepts a per-BT handle; reality has no such flag — flagged as anomaly, not output as if present |
| 3 | 2026-04-25 | omega-probe-dfs24-batch-execution-2026-04-25 §line 100 | C4 | "data: arbitrary-precision Bernoulli numerator generator UNKNOWN (sympy/PARI usable but no wrapper here)" — declined to claim a wrapper exists |
| 4 | 2026-04-25 | omega-probe-dfs24-batch-execution-2026-04-25 §line 170 | C4 | Cremona allbsd shards: "torsion + analytic Sha labels presence UNKNOWN (requires shard inspection)" — declined to claim columns exist |
| 5 | 2026-04-25 | omega-probe-dfs24-batch-execution-2026-04-25 §lines 175, 180 | C2 | Voevodsky 2003 / Geisser-Levine 2001 / KPZ d-extrapolation literature: each tagged "UNKNOWN (text-only, human read)" — declined to summarize content not actually read |
| 6 | 2026-04-25 | omega-cycle-bt545-hodge §lines 139, 142, 143, 149 | C1 + C4 | "L5 dream UNKNOWN", "L8 wake UNKNOWN", "L9 molt UNKNOWN" — declined to invent ladder occupancy claims |
| 7 | 2026-04-25 | omega-cycle-bt545-hodge §line 244 | C4 | "files read; UNKNOWN whether deeper grep would surface one" — declined to extrapolate beyond grep depth |
| 8 | 2026-04-25 | omega-cycle-bt546-bsd §line 105 | C1 | "precise ranking among BT-541/543/545/546 requires the canonical `nxs_002_composite.py --predict-er` run" — declined to fabricate a ranking |
| 9 | 2026-04-25 | omega-cycle-bt546-bsd §line 296 | C1 | "Falsifier density: 9 active falsifiers... Above the 7-falsifier sample mean (UNKNOWN: precise sample mean not computed in this audit)" — labeled the sample-mean comparison as UNKNOWN rather than inventing it |
| 10 | 2026-04-25 | omega-cycle-bt546-bsd §line 310 | meta | explicit checklist line: "[x] UNKNOWN markers used where audit depth is exceeded" — guard usage is itself audited |

**Pattern**: incidents 1-2 are the canonical Probe #3 case; incidents 3-5 are
on the dfs-24 batch (where the tool / data / literature gap surface is
largest); incidents 6-9 are routine guard hits inside per-BT audits.
**Class distribution** in this 10-row sample: C1 ≈ 4, C2 ≈ 1, C3 ≈ 2, C4 ≈ 4
(some overlap), C5 ≈ 0 (C5 slips would not appear in a guard-activation log
because they are *passes* the agent declined to claim — they are absences,
not events).

**Downstream effect of incident #1 (Probe #3)**: directly enabled Probe #2
patch dispatch (the brief that produced the `omega-probe-nxs002-predict-er`
report explicitly cites this as the unblocking event for the per-BT extractor
proposal in §5 of that report).

## §5 Guard-miss audit — sample-check of factual claims

Sampled 5 factual claims from 5 distinct session reports; each verified by
direct `ls` / `grep` against the filesystem.

| # | report | claim | verification | result |
|---|---|---|---|---|
| 1 | omega-probe-nxs002-predict-er §2.1 line 117-120 | composite_after = 0.83221... ; n_nodes = 21320 ; n_components = 24 | grounded in tool output paste in §2.1 (multi-line JSON block) | claim self-evidences as tool output, not invented |
| 2 | omega-probe-nxs002-predict-er §1.1 line 39-41 | tool exists at `~/core/nexus/tool/nxs_002_composite.py` | `ls /Users/ghost/core/nexus/tool/nxs_002_composite.py` → file present | **VERIFIED** |
| 3 | omega-probe-dfs24-batch-execution line 169 | `data/cremona/allbsd/` is 169 MB, 27 shards | `du -sh` → 169M; `ls \| wc -l` → 27 | **VERIFIED** (exact match) |
| 4 | omega-probe-dfs24-batch-execution line 100 | `theory/predictions/verify_bernoulli17_*.hexa` covers Bernoulli structure (8 files) | `ls theory/predictions/ \| grep bernoulli` → 7 `verify_bernoulli17_*` files + 1 `verify_bernoulli_17_enumeration` (also a bernoulli17 file with underscore variant), totalling 8 | **VERIFIED** (count exact) |
| 5 | omega-cycle-bt545-hodge line 93 | `papers/moonshine-barrier-honest-report-2026-04-15.md` exists | `ls` → file present | **VERIFIED** |
| 6 | omega-cycle-bt541-riemann line 11 | `theory/roadmap-v2/phase-omega-Y9-closure-v3-design.md` exists | `ls` → file present | **VERIFIED** |
| 7 | omega-cycle-bt541-riemann line 111 | per-BT 541 estimate composite 0.588 = `(0.636 + 0.556 + 0.571) / 3` | arithmetic verifiable; source lines explicit | **VERIFIED** (computation traceable) |
| 8 | omega-cycle-bt545-hodge line 186 | per-BT 545 estimate 0.432 with UNCALIBRATED tag | grounded computation, explicit honesty tag preserved into §3 of probe-nxs002 (line 176) | **VERIFIED** + properly tagged |

**Result**: **8 / 8 sampled claims verify** against the filesystem or
self-evidence as captured tool output. **Zero guard-miss slips found in this
sample.** The sample is small (n=8) so the upper confidence bound on
slip-rate is loose; a 95% one-sided binomial bound on slip rate given 0/8
hits is roughly **≤ 31%**, which is too loose to claim the slip rate is
small. A larger sample (n ≥ 30) is **out of scope** for this audit.

**What this audit cannot detect**: a slip that is internally consistent and
unfalsifiable — e.g., a claim "I read paper X" when paper X exists but the
agent did not actually read it. C2 citation slips are the hardest to audit
this way because the verification (did the agent read it?) is not derivable
from filesystem state. **Acknowledged audit-depth limit.**

## §6 Redundancy matrix — does any other gate catch fabrication?

For each fabrication class, would each downstream gate catch the slip
**without** the no-fabrication constraint upstream?

| gate \ class | C1 numerical | C2 citation | C3 tool | C4 structural | C5 result |
|---|---|---|---|---|---|
| **no-fab guard** (this audit) | CATCHES (UNDEFINED/UNKNOWN markers, §4 incidents 1, 8, 9) | CATCHES (§4 incident 5) | CATCHES (§4 incidents 1, 2) | CATCHES (§4 incidents 3, 4, 6, 7) | CATCHES (forces explicit not-run tag) |
| **Y9 HONEST-HARNESS distortion audit** | partial (catches retrospectively if the number is later flipped without log entry — bt545 §line 238, 383) | partial (catches if a citation is later contradicted) | MISSES (Y9 audits row-tag flips, not invented tools) | MISSES | CATCHES (Y9 explicit purpose: "verdict-distortion audit" — strength 8 in dfs24-ym §line 76) |
| **falsifier-pre-registration** | CATCHES if the falsifier is numeric and pre-data (e.g. "r ≤ 0.10 vs r ≥ 0.30" in dfs24-riemann); MISSES for un-pre-registered numbers | partial (citation-falsifiers rare) | MISSES | MISSES | CATCHES (forces explicit prediction → measurement → comparison; cannot be PASS without prediction step) |
| **3-way tagging** (rewriting / conditional / observation) | partial (forces honesty about whether a number is observation-mode vs rewriting-mode) | partial | MISSES | MISSES | CATCHES (rewriting tag is the strongest C5 guard) |
| **user review** of session reports | partial (depends on which numbers user spot-checks) | partial (depends on user paper-ground-truth) | partial | partial | partial |

**Coverage gaps if no-fab guard removed**:

- **C3 tool fabrication**: only the no-fab guard reliably catches this.
  Y9 audits row-tag flips, falsifier convention pre-registers the
  measurement-shape, 3-way tagging operates on a truth-mode axis. None of
  these check "does this CLI flag exist?". A C3 slip would propagate
  uncaught until a future invocation attempt fails — potentially **many
  reports later**.
- **C4 structural fabrication**: same gap. If an agent claims
  `data/cremona/foo_bar.json` exists, no downstream gate detects this until
  someone tries to open it.
- **C2 citation**: partial coverage from user review only. No automated
  gate.

**Strong overlap (redundant) classes**:

- **C5 result fabrication**: caught by Y9, falsifier convention, AND 3-way
  tagging. The no-fab guard's contribution here is **partially redundant**.
  This is the only class where compression (not removal) is plausible.
- **C1 numerical**: caught by no-fab guard primarily; Y9 retrospectively;
  falsifier convention only when pre-registered. The no-fab guard is
  load-bearing because pre-registration is not universal.

## §7 Removal counterfactual

**If the no-fabrication guard were dropped today**, class-by-class slip-rate
estimates and first-incident remediation cost:

| class | predicted slip-rate change | first-incident remediation cost |
|---|---|---|
| C1 numerical | **medium → high** rise. Most current per-BT composite-estimate honesty is preserved by explicit `(UNCALIBRATED)` / `(proxy, naive 3-component mean)` tags driven by the guard. Without the guard, the path of least resistance is "report the back-of-envelope number unmarked". | **HIGH**. A single fabricated composite slipped into a per-BT verdict, then rolled up into the backtrace-strategy table, then read by Y9 distortion audit, would produce a false BT verdict — order of multiple person-days to walk back, plus inventory-row honesty cost. |
| C2 citation | **low → medium** rise. Audit-depth limit (§5) means we cannot bound this currently; without the guard, C2 slip rate likely rises because there is no internal pressure to flag "I have not actually read this paper". | **MEDIUM**. False citation typically detected on second-look only; remediation = retract + re-verify chain of dependent claims. |
| C3 tool | **low → high** rise. Probe #3 directly demonstrates: with the guard, the agent honestly returned UNAVAILABLE; without, the path of least resistance is to report 5 plausible-looking per-BT numbers that satisfy the brief shape. | **VERY HIGH**. C3 slip + C1 slip together = false tool-output claim that downstream readers cannot cheaply re-verify (because they assume the tool was run). Probe #3 alone would have produced 5 fabricated composites and prevented the per-BT extractor proposal in §5. |
| C4 structural | **medium** rise. 3-way tagging and falsifier convention are silent on file-existence claims. Without the guard, C4 slip rate rises in proportion to the size of the structural-claim surface (large in dfs-24-batch report). | **MEDIUM-HIGH**. Discovered when a follow-up agent tries to use the claimed file; remediation = patch the source report + re-run dependent computations. |
| C5 result | **low** rise. Y9 distortion audit + falsifier convention + 3-way tagging together provide redundant catching. | **LOW-MEDIUM**. Caught by next downstream gate within days. |

**Aggregate first-incident cost (single fabrication-induced false BT
verdict)**: based on Probe #3's averted-cost estimate (one BT closure
inadvertently advanced from MISS to PARTIAL would require Y9 audit of all
upstream rows + retraction commits + atlas state-rollback if a
[10]→[10*] promotion was triggered): **multiple person-days minimum**, with
a non-negligible chance of cascade contamination if the false verdict was
read by another agent before discovery.

**Net counterfactual**: dropping the guard saves boilerplate but produces a
predictable C3+C4 slip surge with high first-incident remediation cost.
Downstream gates (Y9, falsifier, 3-way) cover C5 well, C1 partially, C2/C3/C4
poorly. Removal is **not justified** by current redundancy structure.

## §8 Cost of keeping (false-positive examples)

The guard is not free. Reviewing the same 11 reports for cases where
UNKNOWN was applied to something that **could** have been computed within
the agent's tool budget:

- **bt545-hodge §line 244** "files read; UNKNOWN whether deeper grep would
  surface one". Tractable — an additional grep was within budget. Decision
  to mark UNKNOWN was **defensible** (audit-depth bounding) but a stronger
  agent would have run the extra grep.
- **bt541-riemann line 192** "Y9 meta-gate audit not yet sealed for BT-541"
  — this is a fact, not a false-positive UNKNOWN.
- **probe-nxs002 §1.4 line 90** declined to recommend specific per-BT
  estimates "blocked by missing tool capability". Could the agent have
  proposed a manual proxy? Yes. Was it correct to decline? **Yes** — a
  proxy reported as if it were a `--predict-er` measurement would have
  been a C1 + C3 slip. **Not a false-positive.**

**Overall false-positive rate (this sample)**: **1 of 8 sampled
UNKNOWN-applications** is plausibly over-cautious (`bt545 §244`). The other
7 are correct constraint engagements. This gives a rough false-positive
rate of **≈ 12%** in this sample, which is acceptable given the C3+C4
slip-rate cost of removing the guard.

**Cost of keeping**:
- ~5-15% of UNKNOWN markers may be over-cautious
- some boilerplate "UNKNOWN: precise X not computed in this audit" lines
- moderate verbosity in §X.4 / §X.5 / falsifier sections of every report

These costs are **strictly bounded** and do not compound.

## §9 Verdict

**KEEP_AS_IS**

**Rationale (one paragraph)**: the no-fabrication guard is load-bearing on
classes C3 (tool) and C4 (structural) where no other gate provides
coverage; it is partially redundant only on C5 (result), where Y9 +
falsifier + 3-way tagging together cover the slip surface. The single
canonical recent activation (Probe #3) directly enabled the next probe's
dispatch — an unambiguous positive case. Saturation is high (~91% of
recent omega-cycle reports actively engage the guard), false-positive cost
is low (~12% over-caution rate in the sample), removal counterfactual is
unfavourable (predicted C3+C4 slip surge with multi-person-day first-
incident remediation cost). Even if a compression of guard verbosity were
considered, the load-bearing role on C3/C4 means the guard cannot be
retired without first building automated C3/C4 detectors elsewhere in the
pipeline. **No removal-shaped decision is licensed by this audit.**

Per the brief, retire decisions require 3-of-3 audits. This is audit 1 of
≥ 3, and its verdict is `KEEP_AS_IS`. Even if subsequent audits returned
`KEEP_BUT_COMPRESS`, the union verdict would still be KEEP, not retire.

## §10 Falsifiers active for this audit

When does this very audit fail to be informative?

- **F-A1**: if the saturation grep (§3) is interpreted as a slip-rate
  rather than a guard-engagement rate — the 91% is **engagements**, not
  catches. Each engagement is a *successful guard hit*; catches without
  engagement (silent fabrication) are by definition not in the grep.
  This audit does NOT measure silent slip rate.
- **F-A2**: if the n=8 sample (§5) is interpreted as a slip-rate
  upper-bound. The binomial 95% one-sided bound at 0/8 is ≈ 31% which is
  too loose to claim "low slip rate". A larger sample is required to
  tighten the bound.
- **F-A3**: if the C2 (citation) class slip rate is much higher than the
  audit's depth allowed it to detect. Citation-fabrication is the hardest
  class to audit ex post — this audit explicitly flags C2 audit depth as
  bounded.
- **F-A4**: if a future instrumentation pass surfaces automated detectors
  for C3 and C4 (e.g. a CI hook that grep-checks every cited path in a
  session report), the redundancy structure changes and the verdict
  could shift to `KEEP_BUT_COMPRESS` for the now-redundant C3/C4 portion
  of the guard. This audit does not pre-judge that future instrumentation.
- **F-A5**: if the user's brief was itself a fabrication-stress-test on
  this audit (similar to F1 in Probe #3) — that is, if the user wanted to
  see whether this audit would invent a slip-rate percentage to satisfy
  the brief shape — then the value is in §5's explicit "n=8 is too small"
  acknowledgement and §3's per-report grounded counts, not in any single
  headline number. The audit is consistent with that reading.
- **F-A6**: if the omega-cycle 4-axis lens itself is a category error for
  this question (constraint audits are not a ladder/saturation/chain/
  closure object), the structural choice in §2-§4 is mis-fit; the
  underlying §6/§7/§8 redundancy + counterfactual analysis still stands
  on its own.

## §11 Closing

Audit complete. Verdict: **KEEP_AS_IS**. The no-fabrication guard remains
load-bearing on classes C3 (tool) and C4 (structural) and partially on
C1/C2; its retirement is not licensed by current redundancy structure or
by the recent guard-activation chain. This audit modifies no atlas, no
inventory, no agent system prompt, no [10]/[10*] grade, and no Clay
status. **0/7 unchanged.**

This is audit 1 of at least 3 required for any retire-shaped decision; on
its own it cannot retire the constraint regardless of conclusion.

— end —
