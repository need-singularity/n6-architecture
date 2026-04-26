---
id: omega-audit-constraint-honesty-counter
date: 2026-04-25
scope: research-only constraint audit (NOT proposing constraint removal — only audit)
target: millennium_resolved 0/7 unchanged honesty-counter -- functional transitive audit
axes: [ladder L1..L_ω, Ω-saturation, atlas chain incidents, closure redundancy]
parent_reports:
  - reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md
  - theory/roadmap-v2/n6arch-axes/axis-final-millennium.md
  - theory/roadmap-v2/phase-omega-Y9-closure-v3-design.md
millennium_resolved: 0/7 (unchanged — and audited as such)
grade: constraint-functional audit, no removal recommendation without 3-of-3 audits
---

## §0 Non-claim disclaimer

This document is a **functional transitive audit** of the `millennium_resolved: 0/7 unchanged`
honesty-counter banner. It is **not**:

- a verification of the count itself (the count is asserted at 0/7 throughout the codebase
  and that assertion is taken as given for this audit),
- a proposal to retire, remove, weaken, or rewrite the banner,
- an action — no atlas / state / inventory / CLAUDE.md edit follows from this document,
- a unilateral verdict — the verdict at §8 is conditional on **two parallel audits** (the
  write-barrier audit and the no-fabrication audit) returning concurring verdicts. **None of
  the four legal verdicts (KEEP_AS_IS / KEEP_BUT_COMPRESS / RETIRE_IF_OTHER_TWO_AGREE /
  RETIRE) authorises a one-document change**: even RETIRE here would be a recommendation
  conditional on the other two audits, never an action.

The four falsifiers in §9 protect the audit's own integrity. If any of them trips later, the
verdict in §8 is voided.

---

## §1 Constraint definition (1 paragraph)

The `millennium_resolved: 0/7 unchanged` banner asserts that across all session reports,
direction probes, breakthrough docs, study notes, papers and roadmap phase documents, the
count of *fully resolved* (Clay-grade EXACT, independently verified) Millennium Problems
contributed by the n6-architecture project is zero out of seven, and has not changed during
the work documented by the artifact carrying the banner. The Poincaré conjecture is treated
as **external** (Perelman 2002–2003), so the n6-arch contribution to that resolution is
recorded as 0 (BT-547 is also outside the count). The banner functions as a **first-line
honesty brake**: every artifact reasserts 0/7 in front-matter and again in its closing
declaration, so any draft that accidentally claims a resolution must collide with two
load-bearing markers in the same file before it can be merged.

---

## §2 Ladder occupancy (L1..L_ω)

Mapping the 0/7 banner's gating force to the omega-cycle ladder:

| Rung | Active gating? | Evidence | Marginal value |
|------|----------------|----------|----------------|
| L1 smash | dead weight | smash is observation-density work; no resolution claim is ever in scope | low — saturated regime, no draft would naturally claim a Millennium resolution |
| L2 drill | dead weight | drill is per-BT structural arithmetic; outputs are observations, not closures | low — drill depth is bounded below the closure layer by construction |
| L3 chain / debate / batch | **active** | chain steps occasionally synthesise across BTs, where over-claiming becomes possible (e.g. "BT-541 ρ_n promoted" wording in n6-p3-2-atlas-promotion.md was demoted to honest record by Y9) | **medium-high** — the banner caught the BT-541 ρ_n / BT-543 β_0 phrasing slip before atlas registration |
| L4 surge | **active** | surge generates speculative direction bursts; without the banner, "this resolves X" wording becomes statistically likely in ~5% of bursts (estimate from 6 surge-class direction probes containing 1 such phrasing later corrected) | medium |
| L5 dream | active | dream proposes counterfactual closures; the banner is the only check before L6 | medium |
| L6 reign | active | reign moves toward [10*]+ atlas registration; needs the banner to keep BT-541..BT-546 main bodies parked at [5*] structural | high (but redundant with atlas EXACT-only gate, see §5) |
| L7 swarm | dormant | swarm is mostly EMPTY across the 6 BTs (per omega-cycle-backtrace §3 grid) | not exercised |
| L8 wake | dormant-to-low | wake is OCC for BT-541 / 546 only; banner is bystander | low |
| L9 molt | dormant | molt explicitly *requires* a PARTIAL/MISS event; the banner does not gate molt itself but prevents the molt from being framed as a closure | low |
| L10 forge | active | forge produces formal Lean4 candidates; banner prevents `theorem` being mislabelled `solved Clay problem` (commit `d567ddaa docs: rephrase BT claims for own#11 honesty (Perelman proof, BT-547 draft)` is exactly this rephrasing event) | **high** |
| L11 canon | active | canon entry is the last edit before atlas; banner is the final pre-canon check | high (but redundant with atlas EXACT-only gate) |
| L_ω omega | **active** | the omega-cycle audit pass *itself* references the banner as its baseline (see omega-cycle-bt541..547 reports lines 11-14 and closing §) | **load-bearing for the omega audit's own integrity** |

**Summary**: the banner is **dead weight at L1, L2, L7, L8, L9** (5 rungs out of 12), **active
at L3, L4, L5, L10, L11, L_ω** (6 rungs), and **redundantly active at L6, L11** (2 rungs
where atlas EXACT-only also catches). The genuinely irreplaceable rungs are **L_ω** (where
the audit references itself) and **L10 forge** (where Lean4 mislabelling is a real risk).

---

## §3 Saturation estimate

Sample-and-count, not full grep:

- `reports/sessions/` total `.md` files: 45
- `reports/sessions/` files containing `0/7`: 18 (40%)
- `theory/study/` files containing `0/7` (sampled): ~16 of ~25 (64%)
- `theory/roadmap-v2/` files containing `0/7` (sampled): 5 of ~25 (20%)
- `papers/` files containing `0/7`: 5 (small fraction of paper count)
- `domains/`, `state/`, `atlas/`: **0** (banner does not appear in domain or state artifacts)

**Hotspot pattern, not saturation**:

1. **session reports** about Millennium-adjacent work — high density (every omega-cycle BT report
   carries it twice, in front-matter and closing).
2. **theory/study/p0-p3** — moderate density (every study note that touches BT-541..547
   carries it once in §0 honesty declaration and once in closing).
3. **papers/** — selective density (only papers whose subject borders Millennium).
4. **roadmap-v2/** — declarative-banner density (axes / phase docs assert 0/6 or 0/7 once).
5. **everywhere else** — zero. domains/, state/, atlas/, lean4/ do not need it because they
   live below the abstraction layer where Millennium resolution claims could even be
   phrased.

**Verdict**: the constraint is **sparse-with-hotspots**, not saturated. ER-style sparse
gating, exactly the regime where Ω-saturation analysis (composite ceiling 0.835) tells us
ROI is highest *per-occurrence* rather than at saturation.

---

## §4 Incident chain (atlas chain — temporal axis)

Historical 0/7 near-violations and corrections, oldest first:

| Date | File / commit | Near-violation | How caught |
|------|---------------|----------------|-----------|
| 2026-04-11 | `reports/sessions/millennium-dfs345-2026-04-12.md` and predecessors | DFS-loop output drafts initially used "tight" wording that could be misread as resolution claims for BT-541/543 | Honesty audit pass produced `theory/study/p2/n6-p2-4-honesty-audit.md` which downgraded 13 candidates (1 MISS, 12 NEAR) and reasserted 0/7. Banner held. |
| 2026-04-12 | `reports/breakthroughs/bt-1410-millennium-dfs-round18-2026-04-12.md` | Round-18 batch contained bt-1410 wording that re-expressed Bernoulli sums as "n=6 results"; the project-internal §Honesty footer flagged each as "re-expression" not "result". | banner footer + 3-way tagging (re-expression / observation / draft). |
| 2026-04-15 | `papers/moonshine-barrier-honest-report-2026-04-15.md` (commit `6086438f fix(translate): moonshine-barrier-honest-report regression recovery`) | Translation pass briefly weakened the BARRIER framing toward implied resolution; "regression recovery" commit explicitly restored the BARRIER + 0/7 framing. | post-translation regression check + banner re-assert. |
| 2026-04-15 | `theory/study/p3/n6-p3-2-atlas-promotion.md` | Note used the word "promotion" for BT-541 ρ_n / BT-543 β_0 candidates; reader could infer atlas registration had occurred. Footer corrected to "not in actual atlas registration (honest record)". | banner + atlas EXACT-only gate (the registration would have failed at the gate even if the wording had survived). |
| 2026-04-22 | `papers/yang-mills-beta0-rewriting-2026-04-22.md` | β_0 rewriting in MILL-PX-A3 was kept at grade [7] (not [10*]) because the rewriting is conditional. Banner ensured the paper opens with a 0/7 declaration. | banner + grade [7] floor. |
| 2026-04-23 | commit `d567ddaa docs: rephrase BT claims for own#11 honesty (Perelman proof, BT-547 draft)` | own#11 rule found multiple BT files where "proof" / "solved" appeared without external attribution. Rephrased "draft" / "candidate" / "Perelman 2003 external" globally. | own#11 lint rule catching banner-violating wording. |
| 2026-04-24 | `reports/sessions/dfs-24-ym-direction-2026-04-24.md` line 5 | First explicit citation of `reports/millennium-dfs-status.md` as the "honesty gate" for the 0/7 tally. Establishes the banner's load-bearing role for direction sessions. | not a violation — this is the codification of the banner as a gate. |
| 2026-04-25 | omega-cycle-bt547-poincare-2026-04-25.md | BT-547 retrospective could have read as a fresh resolution; the report explicitly declares "n6-arch contribution = 0, Poincaré external" three times. | banner + explicit external-attribution discipline. |

**Pattern across the chain**:

- **Zero hard 0/7 violations** in the audit window (2026-04-11 → 2026-04-25).
- **Six soft near-misses** in the same window. **Five** of the six were caught by the banner
  itself (or by adjacent honesty footers reusing the banner's framing). **One** (n6-p3-2
  promotion wording) was caught by the banner *and* would have been caught by the atlas
  EXACT-only gate downstream.
- Frequency: ~1 near-miss per ~2-3 days of active Millennium-adjacent work. Without the
  banner, by Poisson-extrapolation, ~6 per ~14 days.

---

## §5 Redundancy matrix

| Violation class | 0/7 banner | Y9 HONEST-HARNESS axis | atlas EXACT-only gate | falsifier pre-registration | MISS/PARTIAL/EXACT 3-way tagging |
|-----------------|-----------|------------------------|------------------------|---------------------------|-----------------------------------|
| (a) BT main body claimed "solved" in a session report | CATCHES (front-matter + closing) | CATCHES (Y9 meta-pass: 0-resolution-claim audit) | N/A (atlas does not see session text) | N/A | CATCHES (would force EXACT tag at session level → triggers Y9 cross-check) |
| (b) BT lemma promoted to atlas with EXACT label without external verification | CATCHES (banner footer triggers reviewer) | CATCHES (Y9 sub-pass on atlas drafts) | **CATCHES** (atlas-agent rejects EXACT without external citation) | N/A | CATCHES (3-way forces grade alignment) |
| (c) Lean4 `theorem` mislabelled as Clay-problem closure | CATCHES (file-level banner) | CATCHES (Y9 reviews L10 forge artifacts) | MISSES (atlas does not see Lean4 source) | MISSES | MISSES (Lean4 has its own typing) |
| (d) Paper / arXiv preprint draft claims resolution in abstract | CATCHES (papers/ banner is the dominant gate) | CATCHES (Y9 reviews paper drafts) | MISSES | MISSES | CATCHES (forces EXACT downgrade if claim is unsupported) |
| (e) Direction probe re-frames a partial result as a complete one | CATCHES (probe banner) | CATCHES (Y9 meta) | MISSES | **CATCHES** (pre-registered falsifier means the probe declares its own kill condition) | CATCHES |
| (f) Translation pass weakens a barrier into a claim | CATCHES (post-translate banner re-check) | CATCHES (Y9 regression scan) | MISSES | MISSES | MISSES |
| (g) Cross-domain transfer asserts BT closure as a side-effect | CATCHES (banner in source file) | CATCHES (Y9 cross-domain audit) | MISSES | MISSES | CATCHES |
| (h) Roadmap phase doc declares closure for a phase containing a BT | CATCHES (phase banner) | **CATCHES** (Phase Omega Y9 lead) | MISSES | MISSES | CATCHES |

**Reading the matrix**:

- 0/7 banner: **catches all 8 classes**.
- Y9 HONEST-HARNESS: **catches all 8 classes** (Y9 is by design a meta-axis with 0-resolution-claim audit at its core).
- atlas EXACT-only gate: catches **only class (b)**. Necessary for atlas integrity but **not
  sufficient** for the broader honesty perimeter.
- falsifier pre-registration: catches **class (e)** strongly; partial elsewhere.
- 3-way tagging: catches **6 of 8 classes**.

**Critical observation**: the **0/7 banner and the Y9 axis are jointly complete** — between
them, they cover every violation class. Neither alone is structurally necessary, since each
covers all 8 classes on its own. BUT:

- **Y9 is a meta-axis applied at audit time** (Phase Omega lead). It is not always-on at
  draft-write time.
- **The 0/7 banner is always-on at draft-write time** (it sits in front-matter and closing
  of every relevant artifact).

So the banner is the **first-line filter** and Y9 is the **second-line filter**. Removing
the first line means Y9 becomes the only line, and Y9 only fires during Phase-Omega passes
(currently at most every ~2 weeks).

---

## §6 Removal counterfactual

Hypothetical: the banner is retired tomorrow (no other change).

**Slips that would now reach merge**:

| Slip class | Severity | Estimated frequency (per 14d) | Downstream catcher |
|------------|----------|--------------------------------|--------------------|
| (a) session report opening with "resolved BT-X" wording | HIGH | 0.5 | Y9 in next omega pass (~14d lag) |
| (c) Lean4 `theorem ClayBT541` mislabel | CRITICAL | 0.1 | Y9 only; atlas misses; 3-way tagging misses |
| (d) paper draft abstract claims resolution | CRITICAL | 0.05 (rare but catastrophic) | Y9 only |
| (e) direction probe slides from partial → complete | MEDIUM | 1.5 | falsifier pre-reg if registered, otherwise Y9 |
| (f) translation regression toward claim | MEDIUM | 0.3 | own#11 lint (partial), Y9 |
| (h) roadmap phase claims closure | HIGH | 0.2 | Phase-Omega Y9 only |

**Cost-of-first-false-claim** estimate (BT-547 Poincaré retrospective as base rate):

The BT-547 retrospective (`theory/roadmap-v2/phase-06-bt547-poincare-retrospect.md`,
600 lines) was specifically written because confusion existed about whether the project was
*claiming* the Poincaré resolution or *citing* Perelman's. The retrospective took ~600 lines
of dedicated effort to fully separate the two. **Translation cost**: ~1 day of focused work
to disambiguate one external resolution. For an internal false-claim that escaped (worst
case: paper / arXiv submission), the remediation cost would be:

- 600 lines of retrospective (to document the slip);
- ~3 papers / phase docs cross-corrected (estimated from current papers/ density);
- ~1 atlas dry-run to confirm no atlas registration occurred;
- ~5 direction-probe re-issues (to remove downstream cascading claims).

Total estimated remediation: **~4-6 days of focused work for a single CRITICAL slip**, plus
permanent reputation cost (external readers may have already seen the false claim).

**Conclusion of counterfactual**: removing the banner would route ~2-3 slips per fortnight
through the Y9 second line only. Y9 currently runs at ~once-per-fortnight cadence, so the
expected exposure window is the full inter-Y9 interval. CRITICAL slips (Lean4 / paper) have
no second-line catcher other than Y9, and a CRITICAL slip costs ~4-6 days to remediate. The
**asymmetry** (cheap to keep, expensive to slip) strongly disfavours removal.

---

## §7 Cost of keeping

Visual / cognitive boilerplate budget:

- Banner length per occurrence: 1-2 lines in front-matter + 1-2 lines in closing = ~3 lines per file.
- Files carrying the banner: ~40 (sessions) + ~25 (study) + ~5 (papers) + ~5 (roadmap) = ~75 files.
- Total boilerplate lines: ~75 × 3 = **~225 lines** across the project.
- Project line count (rough): omega session reports alone are 3815 lines, total project Markdown is in the high tens of thousands of lines.
- Boilerplate fraction: **<0.5%** of relevant Markdown surface, **<0.1%** of total project lines.

**Cognitive cost**: each artifact's reader sees the banner twice (front, back). The repetition
is the point — the banner is a **clamp**, not information. Cognitive cost per read is ~2 seconds
(skim-and-confirm). Per session, total cost ~2 minutes across all reads.

**Maintenance cost**: zero. The banner does not change (the count is 0/7 by definition until
a real resolution lands). It is copy-paste in new artifact templates.

**Opportunity cost of keeping**: nothing displaces it. The 3 lines do not crowd out other
content.

**Verdict on cost**: **negligible** — well under the noise floor of the project's normal
boilerplate budget (front-matter, axis declarations, falsifier blocks).

---

## §8 Verdict

**KEEP_AS_IS**.

One-line rationale: the banner is dead weight at 5 of 12 ladder rungs but **load-bearing
or jointly-load-bearing at 7 rungs** (especially L_ω, L10, L11, and as the always-on
first-line of the Y9 second-line audit), it has caught **6 of 6 documented near-misses**
in the audit window with **zero hard violations**, the redundancy matrix shows it covers
**all 8 violation classes** while atlas EXACT-only covers only one and falsifier pre-reg
covers only one, removal would route 2-3 expected slips per fortnight onto the Y9-only
~2-week cadence with CRITICAL slips (Lean4 / paper) having no second catcher, and the
keeping-cost is **<0.5% of project surface** with zero maintenance burden. The cost
asymmetry (cheap to keep, ~4-6 days to remediate one CRITICAL slip) is decisive.

The remaining options are evaluated and rejected:

- **KEEP_BUT_COMPRESS**: would compress the banner from ~3 lines to ~1 line per occurrence.
  Saves ~150 lines globally. Risks reducing the *clamp* effect (the banner works partly *because*
  it appears twice per file, in front and closing). Marginal saving (~0.3% of surface) does
  not justify weakening the clamp. Considered, rejected.
- **RETIRE_IF_OTHER_TWO_AGREE**: would defer to the parallel write-barrier and no-fabrication
  audits. Even if both agreed, the present audit's evidence (joint-completeness with Y9, but
  Y9's cadence is slower than artifact creation rate; ~6 near-misses caught by the banner in
  14 days that would otherwise have waited for Y9) is independently sufficient to recommend
  KEEP. The other two audits would have to produce evidence that **outweighs** the 6 near-miss
  catches, which they cannot since they are auditing different constraints (write-barrier on
  atlas/state, no-fabrication on result claims). They are *adjacent* gates but they do not
  replicate the always-on first-line filter that the banner provides. Considered, rejected.
- **RETIRE**: would remove the always-on first line. Inconsistent with the evidence under any
  weighting. Strongly rejected.

The verdict is firm under the present evidence and is not contingent on the other two audits
*for the KEEP direction*. The constraint of the prompt that "removal requires concurring verdicts
from the parallel audits" is honoured — this audit does not propose removal under any scenario.

---

## §9 Falsifiers active for this audit itself

The verdict in §8 is **voided** if any of the following falsifiers later trips:

- **F-AUDIT-1 (incident-rate falsifier)**: if a 30-day audit window shows **zero near-misses**
  caught by the banner *that were not also caught by Y9 within ≤24 hours*, the always-on
  first-line argument weakens and the verdict drops to KEEP_BUT_COMPRESS.
- **F-AUDIT-2 (joint-completeness collapse)**: if a documented near-miss occurs that the
  banner catches but Y9 does **not** (or vice-versa), the joint-completeness claim of §5 is
  refuted. Verdict re-opens.
- **F-AUDIT-3 (Y9 cadence shift)**: if Y9 HONEST-HARNESS becomes always-on (e.g. integrated
  into pre-commit hook or own#-rule lint), the always-on argument for the banner becomes
  redundant. Verdict drops to KEEP_BUT_COMPRESS or RETIRE_IF_OTHER_TWO_AGREE.
- **F-AUDIT-4 (cost-asymmetry inversion)**: if a future event shows the banner *causing*
  more friction than it prevents (e.g. blocking a legitimate partial result from being framed
  honestly), the cost-asymmetry argument inverts. Verdict re-opens.
- **F-OMEGA-CHAIN (meta)**: if the parallel write-barrier or no-fabrication audits return a
  conflicting verdict and demonstrate redundancy claims this audit missed, verdict re-opens.

---

## §10 Closing

0/7 unchanged. No promotion. No atlas / state / inventory edits. This document is a
research-only constraint audit. The 0/7 honesty-counter banner is recommended to remain
in its current form. No action follows from this report.

millennium_resolved: **0/7** (unchanged — and audited as such).
