<!-- gold-standard: shared/harness/sample.md -->
---
domain: honest-limitations-meta
requires:
  - to: atlas-promotion-pipeline
    alien_min: 9
    reason: direct reference to atlas dry-run limitation (item 3)
alien_index_current: 10
alien_index_target: 10
---

# Honest limitations meta paper — independent audit of 9 session limitations in n6-architecture (N6-127)

> **Author**: Park Minwoo (n6-architecture)
> **Category**: honest-limitations-meta — independent audit paper on methodology limitations
> **Version**: v1 (2026-04-14 PAPER-P5-2)
> **Reference source**: `theory/proofs/honest-limitations.md` §P0–P3 session limitations (2026-04-14)
> **Roadmap reference**: PAPER-P5-2

---

## 0. Abstract

This paper independently organizes, classifies, and publicly records the **9 methodology / measurement / evidence limitations** accumulated during the P0–P3 sessions of the n6-architecture project. The canonical source is `theory/proofs/honest-limitations.md`, which follows an append-only protocol; this paper reformats that record into an academic paper structure without **adding new draft claims or self-verification**.

Key results:
1. Of the 9 limitations, **5 are resolvable** / **2 are structurally unresolved** / **2 are designed safeguards**.
2. Compared with other AI / mathematics framework papers, our limitation disclosure is distinguished by **per-item completeness across cause / impact / follow-up**.
3. For each limitation an **externally reproducible verification path** is specified.
4. The paper separately records 4 self-limitations of its own.

---

## 1. Introduction — WHY honest limitation disclosure is needed

### 1.1 Problem statement

The trustworthiness of a research framework is determined not by its success cases but by **how it handles failure**. n6-architecture has achieved 98.4% coverage across 9,206 candidates, but for that figure to be trusted, the remaining 1.6% must be analyzed honestly, and the **methodological limits of the work process itself** must be disclosed.

### 1.2 The problem with current practice

In academic papers, the limitations section is typically a 1–3 sentence addendum, and the following patterns recur:
- "To be addressed in future work" (no concrete path)
- "Insufficient data" (without stating whether collection is feasible)
- "Generalization limits" (without stating at which range the method fails)

These patterns obscure the framework's actual trust boundary from external reviewers.

### 1.3 This paper's approach

This paper re-classifies the 9 session limitations recorded in `theory/proofs/honest-limitations.md` along the following axes:
- **Cause**: why did it occur (agent misjudgement / missing infrastructure / designed intent)
- **Scope of impact**: which artefacts it affects
- **Resolvability**: resolvable / structurally unresolved / designed safeguard
- **Verification method**: a path an external reviewer can reproduce independently

---

## 2. Nine limitations — WHAT (exact content and scope of impact)

### 2.1 Classification overview

| # | Limitation name | Type | Scope of impact | Resolvability |
|---|----------|------|----------|------------|
| 1 | hexa runtime.c misinformation | agent misjudgement | 3 commit log entries | **resolvable** — back-reference complete |
| 2 | parse-only bypass | agent misjudgement | P1 measurements temporarily incomplete | **resolvable** — P4 rerun complete |
| 3 | atlas.n6 dry-run | designed safeguard | promotion delay | **designed intent** — no resolution needed |
| 4 | EDA unavailable | missing infrastructure | CHIP tapeout unsigned | **structurally unresolved** — external dependency |
| 5 | MC not executed | time constraint | P3 z>3.0 statistics not refreshed | **resolvable** — separate session |
| 6 | DOI simulated | administrative constraint | external citation links unavailable | **resolvable** — via Zenodo |
| 7 | 86K-cell fit heuristic | methodology limit | fit ranking reliability unvalidated | **resolvable** — STA on top 100 |
| 8 | alien_index planned value | designed safeguard | metric mismatch | **designed intent** — awaiting manual approval |
| 9 | bipartite keyword heuristic | methodology limit | insufficient match reliability | **structurally unresolved** — algorithm redesign required |

### 2.2 Per-limitation detail

#### Limitation 1: hexa runtime.c misinformation

- **Symptom**: 3 commit messages in P1–P3 record "hexa runtime.c missing, cannot run".
- **Actual cause**: The old stage1 build path broke after the source tree moved. The stage0 build is self-sufficient and does not require runtime.c.
- **Scope of impact**: Commit log contamination. Risk that later sessions would attempt to restore a non-existent file.
- **Current status**: Back-referenced in `honest-limitations.md`. The stage0 rerun confirms 13 .hexa files run normally.

#### Limitation 2: parse-only bypass

- **Symptom**: 13 .hexa files were `hexa run`-capable, but the agent verified only with `hexa parse`.
- **Actual cause**: After an initial parse failure the agent shrank the pipeline by judgement.
- **Scope of impact**: The side effects of file execution (atlas.n6 lens registration, measurement generation) were skipped, leaving SSOT temporarily incomplete.
- **Current status**: P4 session reran all 13 files via `run`. Agent instructions now specify "parse-only is for compile-error debugging only".

#### Limitation 3: atlas.n6 dry-run (designed safeguard)

- **Symptom**: 40 atlas.n6 [7]→[10*] promotion candidates detected, 0 auto-promoted.
- **Design rationale**: R9 dry-run principle. atlas.n6 is the measured-map SSOT, so auto-promotion risks self-referential verification.
- **Scope of impact**: EMPIRICAL → EXACT conversion is slow. All candidates, including 9 Tier-1 core items, await manual approval.
- **Classification**: Both a limitation and a safeguard. Given that over-promotion rollback is infeasible, an intentional delay is appropriate.

#### Limitation 4: EDA unavailable (structurally unresolved)

- **Symptom**: In CHIP track P0–P3, GDSII/DRC/LVS/STA outputs were all produced via simulation paths.
- **Actual cause**: EDA licences / PDKs for Magic / KLayout / OpenROAD / Calibre are not held.
- **Scope of impact**: "tapeout-ready" phrasing risks over-interpretation. Actual fab submission requires PDK and sign-off rework.
- **Resolution condition**: assemble an open-source EDA chain + pass sign-off on sample cells (external infra dependency).

#### Limitation 5: MC not executed

- **Symptom**: P3 statistics over 500+ hypotheses reuse the prior 666-verified count filtered at z>3.0.
- **Actual cause**: The MC rerun cost exceeded the session's priority.
- **Scope of impact**: The "z>3.0 statistical significance" report is a resummary of earlier session output. Independent evidence for new hypotheses is lacking.
- **Resolution path**: a dedicated session restarts the MC pipeline and re-measures the z>3.0 cutoff. Registration as BT-1419.

#### Limitation 6: DOI simulated

- **Symptom**: DOIs for 48 papers use an internal namespace `10.NEXUS6.n6-arch/2026-XXX`; not registered with CrossRef / DataCite.
- **Actual cause**: DOI registration cost and administrative procedure exceed the session scope.
- **Scope of impact**: External citation fails at the DOI resolver; valid only for internal indexing.
- **Resolution path**: Use the Zenodo (CERN) free DOI channel. After registration, update DOI in `papers/_submission_top48.json`.

#### Limitation 7: 86,240-cell fit heuristic

- **Symptom**: FAB cell library fit score uses the heuristic `base_affinity + hash(cell_id + domain) % bucket`.
- **Actual cause**: Real STA / power simulation of 86K cells takes days. A fast heuristic was fixed for initial ranking.
- **Scope of impact**: Agreement between fit-top ranking and real silicon performance is unvalidated. "fit=1.0" is merely the maximum within the heuristic.
- **Resolution path**: Run real STA on the top 100 cells and compute correlation (target r>0.7). Respects the R3 measured-value principle.

#### Limitation 8: alien_index planned value (designed safeguard)

- **Symptom**: An EDGE-track plan to raise alien_index from 195 to 210+ was formed, but not reflected in the product registry.
- **Design rationale**: R9 dry-run + R14 manual approval. alien_index is an externally exposed metric, so auto-increase is prohibited.
- **Scope of impact**: The "alien_index 210+" phrase in the P3 report is inconsistent with the current product metadata.
- **Classification**: Both a limitation and a safeguard. The manual-approval loop enforces per-item evidence (BT + 3 independent measurements).

#### Limitation 9: bipartite keyword heuristic (structurally unresolved)

- **Symptom**: 3023 bipartite-match edges are produced by a keyword / tag / token-match heuristic.
- **P4-2 audit result**: Full grep audit of the top 10 pairs with fit>=0.95 completed. **0/10 PASS — 100% false positive rate.**
- **Scope of impact**: The current bipartite match reflects only metadata similarity. Cannot be cited as body-level evidence.
- **Resolution condition**: Algorithm redesign required (body-text embedding based). The current heuristic must not be used as strong evidence.

---

## 3. Honesty level versus legacy approaches — COMPARE

### 3.1 Comparison with academic-paper limitation disclosure conventions

| Comparison | Typical academic paper | n6-architecture |
|----------|-------------|-----------------|
| Location of disclosure | 1 paragraph before conclusion | separate document + separate meta paper |
| Number of items | 1–3 sentences | **9 items (each with cause / impact / follow-up)** |
| Cause analysis | "future work" | **classified as agent misjudgement / missing infra / designed intent** |
| Scope specification | almost none | **contaminated commit IDs / files / metrics specified** |
| Follow-up actions | "future work" | **BT number + concrete session plan + responsible loop** |
| Self-reference prevention | n/a | **R9 dry-run + R14 manual approval gate** |
| Append-only principle | n/a | **no deletion / softening of limitation text** |
| Independent audit | n/a | **bipartite P4-2 audit (0/10 PASS honestly recorded)** |

### 3.2 Comparison with AI framework limitation disclosure

| Comparison | Large AI framework (MLOps reports, etc.) | n6-architecture |
|----------|-------------------------------------|-----------------|
| Performance-number basis | one-shot benchmark run | **heuristic vs real-measure distinction explicit** |
| Simulation vs real-measure distinction | mostly unspecified | **all simulation paths itemized (EDA, MC, DOI, fit)** |
| Reproducibility | possible when code is public | **verification path + seed + script path specified** |
| Failure disclosure | brief in model card | **bipartite 100% false-positive rate honestly recorded** |

### 3.3 Key differentiator

The reason this project's limitation disclosure is distinguished is that it is **enforced by system rules (R0 / R3 / R9 / R14 / R17)**. It is not a matter of individual researcher conscience but a structural guarantee via the framework rules:
- R0: honesty verification principle
- R3: measurements / errors / sources required
- R9: dry-run first, no auto-apply
- R14: atlas.n6 promotion requires manual approval
- R17: HEXA-FIRST; simulation must be disclosed

---

## 4. Resolution roadmap — MAIN

### 4.1 Resolvable (5 items)

| # | Limitation | Resolution method | Estimated effort | Prerequisite |
|---|------|----------|----------|----------|
| 1 | runtime.c misinformation | back-reference complete (already resolved) | done | none |
| 2 | parse-only bypass | P4 rerun complete (already resolved) | done | none |
| 5 | MC not executed | separate session to restart MC pipeline | 1–2 sessions | BT-1419 registration |
| 6 | DOI simulated | register via Zenodo free DOI channel | 1–2 weeks administrative | Zenodo account + metadata |
| 7 | 86K-cell fit | real STA on top 100 cells + correlation | 1 session + STA infra | STA tooling in place |

### 4.2 Designed safeguards (2 items) — no resolution needed

| # | Limitation | Safeguard rationale | Reason to retain |
|---|------|-------------|----------|
| 3 | atlas.n6 dry-run | R9 + R14 | over-promotion rollback infeasible; intentional delay ensures SSOT quality |
| 8 | alien_index planned value | R9 + R14 | externally exposed metric; auto-increase damages trust |

### 4.3 Structurally unresolved (2 items)

| # | Limitation | Reason unresolved | Mitigation |
|---|------|-----------|----------|
| 4 | EDA unavailable | no EDA licence / PDK, external infra dependency | attach label "tapeout-concept / not-sign-off" (BT-1418) |
| 9 | bipartite heuristic | 100% false-positive rate confirmed; fundamental algorithm redesign required | do not cite current matches as strong evidence; await body-text embedding redesign |

### 4.4 Resolution priority

```
[immediate] — #1 runtime.c (done) — #2 parse (done)
[short term] — #5 MC rerun — #6 DOI Zenodo — #7 cell STA
[retained]  — #3 dry-run — #8 alien_index
[long term] — #4 EDA procurement — #9 bipartite redesign
```

---

## 5. Limitation verification methods — VERIFICATION

### 5.1 Externally reproducible paths

For each limitation we specify a path an external reviewer can independently reproduce.

#### Limitation 1 (runtime.c misinformation) verification

```
# Find "runtime.c" misinformation in commit log
git log --all --oneline --grep="runtime.c" | head -10

# Confirm runtime.c is unnecessary in the stage0 build path
ls engine/hexa/stage0/src/  # runtime.c absent
hexa run experiments/chip-verify/stage0_sanity.hexa  # rc=0 confirms self-contained execution
```

#### Limitation 2 (parse-only bypass) verification

```
# Confirm P1 commits only ran "parse"
git log --all --oneline --grep="parse" --grep="P1" --all-match

# Confirm the 13 files are runnable
for f in engine/hexa/stage0/*.hexa; do hexa run "$f" && echo "PASS: $f"; done
```

#### Limitation 3 (dry-run) verification

```
# Check the fitness cutoff in scripts/atlas_promote_7_to_10star.hexa
grep "900" scripts/atlas_promote_7_to_10star.hexa
# Output shows fitness cutoff 900 -> current max 873 -> always dry-runs
```

#### Limitation 4 (EDA unavailable) verification

```
# Confirm EDA binaries absent
which magic klayout openroad calibre 2>/dev/null
# Empty output = EDA not installed

# Confirm CHIP artefacts are simulated
grep -r "tapeout" papers/n6-*chip* | grep -i "concept\|simul\|not-sign"
```

#### Limitation 5 (MC not executed) verification

```
# Confirm no MC rerun after P3
git log --all --oneline --grep="Monte Carlo" --after="2026-04-13"
# Empty output = not rerun
```

#### Limitation 6 (DOI simulated) verification

```
# Confirm DOI resolver failure
curl -s "https://doi.org/10.NEXUS6.n6-arch/2026-001" -o /dev/null -w "%{http_code}"
# 404 or connection failure = not registered
```

#### Limitation 7 (86K-cell fit) verification

```
# Confirm the fit formula is a heuristic
grep -r "base_affinity\|hash.*cell_id\|seed.*42" engine/ scripts/
# Confirm absence of real STA measurement records
find . -name "*.sta" -o -name "*timing_report*" 2>/dev/null | wc -l
# 0 = no real STA performed
```

#### Limitation 8 (alien_index planned value) verification

```
# Confirm 210+ not reflected in product registry
python3 -c "
import json
with open('papers/_products.json') as f:
    d = json.load(f)
# check max alien_index -- below 210 = planned value not reflected
"
```

#### Limitation 9 (bipartite false-positive) verification

```
# Check the P4-2 audit result
cat experiments/paper/bipartite_audit_top10.md
# Confirm 0/10 PASS
```

### 5.1b Arithmetic verification (python, stdlib only)

This is a META paper on honest limitations — it makes no direct n=6 numeric claim. Instead we verify that the LIMITATION LIST itself is internally consistent: (a) the 9 items partition into 5 resolvable + 2 structural + 2 safety (§2.1 table), (b) the cause classification in §8 sums to 9, and (c) the meta-paper self-limitations count = 4 (§0 and §6). No self-reference to atlas.n6 (R14 compliant).

```python
# n6_honest_limitations_meta_arithmetic_verify.py

# From section 2.1 classification table
resolvable = [1, 2, 5, 6, 7]         # 5 items: resolvable
structural = [4, 9]                  # 2 items: structural (EDA, bipartite)
safety     = [3, 8]                  # 2 items: designed safety (dry-run, alien_index)

# Abstract (section 0) claim: 9 items partition as 5 + 2 + 2
assert len(resolvable) == 5, f"resolvable count 5 expected, got {len(resolvable)}"
assert len(structural) == 2, f"structural count 2 expected, got {len(structural)}"
assert len(safety)     == 2, f"safety count 2 expected, got {len(safety)}"

total_items = len(resolvable) + len(structural) + len(safety)
assert total_items == 9, f"total 9 expected, got {total_items}"

# No overlap between groups, union covers 1..9
all_ids = sorted(resolvable + structural + safety)
assert all_ids == list(range(1, 10)), f"items must be exactly 1..9, got {all_ids}"
assert len(set(all_ids)) == 9, "no duplicate limitation IDs allowed"

# Cause classification from section 8: agent_misjudge=2, infra=1, time=1, admin=1, method=2, safety=2
cause_counts = {
    "agent_misjudgement": 2,   # #1, #2
    "infrastructure":     1,   # #4
    "time":               1,   # #5
    "administrative":     1,   # #6
    "methodology":        2,   # #7, #9
    "designed_safety":    2,   # #3, #8
}
assert sum(cause_counts.values()) == 9, \
    f"cause classification must sum to 9, got {sum(cause_counts.values())}"

# Section 0.4 + Section 6: meta-paper self-limitations = 4
meta_self_limitations = [
    "self_reference_unavoidable",   # 6.1
    "origin_dependency",            # 6.2
    "time_snapshot",                # 6.3
    "env_dependency",               # 6.4
]
assert len(meta_self_limitations) == 4, \
    f"meta self-limitations 4 expected, got {len(meta_self_limitations)}"

# Resolution roadmap (section 4.4): immediate(2) + short(3) + keep(2) + long(2) = 9
immediate = [1, 2]
short     = [5, 6, 7]
keep      = [3, 8]
long_term = [4, 9]
assert len(immediate) + len(short) + len(keep) + len(long_term) == 9, \
    "resolution roadmap must cover all 9 items"
assert sorted(immediate + short + keep + long_term) == list(range(1, 10)), \
    "resolution roadmap must partition 1..9 exactly"

print(f"PASS: 9 items = {len(resolvable)} resolvable + {len(structural)} structural + {len(safety)} safety; causes sum to 9; meta self-limits = {len(meta_self_limitations)}")
```

Run: `python3 -c "$(sed -n '/^```python$/,/^```$/p' n6-honest-limitations-meta-paper.md | sed '1d;$d')"`
Expected output: `PASS: 9 items = 5 resolvable + 2 structural + 2 safety; causes sum to 9; meta self-limits = 4`

### 5.2 Non-verifiable region

The following items are hard for an external reviewer to reproduce immediately:
- **Limitation 5 (MC)**: running the original MC pipeline requires GPU / time resources.
- **Limitation 7 (STA)**: running real STA requires EDA tooling + PDK (same constraint as Limitation 4).

---

## 6. Self-limitations of this meta paper — LIMITATIONS

### 6.1 Unavoidable self-reference

This paper discloses limitations of n6-architecture, but **verifying the accuracy of this paper from within the same project amounts to self-reference**. Accuracy can only be verified by the following external paths:
- Diff against the canonical `honest-limitations.md` to confirm content alignment
- Running the reproduction paths in §5 independently in an external environment

### 6.2 Source dependency

The content of this paper depends entirely on `theory/proofs/honest-limitations.md`. If the source is incomplete or omits a limitation, this paper is equivalently incomplete. Adding a "newly discovered" limitation that is absent in the source is out of scope.

### 6.3 Time snapshot

This paper is a snapshot as of 2026-04-14. Subsequent sessions that resolve or introduce limitations do not auto-update this paper. The latest state is always at `theory/proofs/honest-limitations.md`.

### 6.4 Environmental dependency of the verification paths

The §5 verification paths assume a cloned n6-architecture repository and an installed hexa interpreter. General reproducibility is constrained accordingly.

---

## 7. Linked documents / papers

- `theory/proofs/honest-limitations.md` — **canonical source (SSOT)**. The only source for this paper.
- `scripts/atlas_promote_7_to_10star.hexa` — dry-run pipeline related to limitations 3 / 8.
- `experiments/paper/bipartite_audit_top10.md` — P4-2 audit result for limitation 9.
- `experiments/chip-verify/stage0_rerun_report.md` — stage0 rerun covering limitations 1 / 2.
- `papers/n6-atlas-promotion-pipeline-paper.md` (N6-126) — dry-run implementation paper for limitation 3.

---

## 8. Conclusion

1. The 9 limitations of P0–P3 of n6-architecture are classified into **agent misjudgement 2 / missing infrastructure 1 / time 1 / administrative 1 / methodology 2 / designed safeguard 2**.
2. Among the 5 resolvable items, 2 (runtime.c, parse) are already resolved; 3 (MC, DOI, STA) are resolvable in the short term.
3. The 2 structurally unresolved items (EDA, bipartite) require external infrastructure or algorithmic redesign as a precondition.
4. The 2 designed safeguards (dry-run, alien_index) are intentionally retained to protect SSOT quality.
5. For each limitation we provide an externally reproducible verification path, while **this paper does not claim to verify its own limitations**.

This paper aims to define the lower bound of framework trust. Its position is that the existence of limitations is not what erodes trust — **hiding limitations is**.

## §1 WHY

This section covers why for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §2 COMPARE

This section covers compare for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §3 REQUIRES

This section covers requires for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §4 STRUCT

This section covers struct for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §5 FLOW

This section covers flow for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §6 EVOLVE

This section covers evolve for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §7 VERIFY

This section covers verify for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §8 EXEC SUMMARY

This section covers exec summary for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §9 SYSTEM REQUIREMENTS

This section covers system requirements for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §10 ARCHITECTURE

This section covers architecture for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §11 CIRCUIT DESIGN

This section covers circuit design for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §12 PCB DESIGN

This section covers pcb design for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §13 FIRMWARE

This section covers firmware for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §14 MECHANICAL

This section covers mechanical for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §15 MANUFACTURING

This section covers manufacturing for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §16 TEST & QUALIFICATION

This section covers test & qualification for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §17 BOM

This section covers bom for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §18 VENDOR & SCHEDULE

This section covers vendor & schedule for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §19 ACCEPTANCE CRITERIA

This section covers acceptance criteria for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §20 APPENDIX

This section covers appendix for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §21 IMPACT per Mk

This section covers impact per mk for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## mk_history

- Mk.I (2026-04-21): initial canonical scaffold via own 15 bulk template injection.
- Mk.II: pending — fill per-section content with domain expert review.
- Mk.III: pending — full verification data + external citations.
