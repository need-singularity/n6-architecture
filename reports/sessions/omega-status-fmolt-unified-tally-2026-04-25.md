---
id: omega-status-fmolt-unified-tally
date: 2026-04-25
scope: status synthesis (NOT producing new claims; consolidating existing verdicts)
target: F-MOLT-A / F-MOLT-D / D-tier / EXT-tier / discriminator-type 2×2 -- unified dashboard
parent_reports:
  - reports/sessions/omega-exec-bt541-leadB-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt542-hirahara-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt543-p3-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-q1-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-fallback-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md
  - reports/sessions/omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md
  - reports/sessions/omega-exec-bt544-r5-lemma1-strict-gap-2026-04-25.md
  - reports/sessions/omega-exec-bt544-r5-low-mach-dispatch-2026-04-25.md
  - reports/sessions/omega-exec-bt545-hodge-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt546-bsd-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt546-probeB-28stratum-2026-04-25.md
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md
  - reports/sessions/omega-meta-audit-l9-catalogue-design-2026-04-25.md
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md
  - reports/sessions/omega-meta-audit-cross-bt-n-phi-pattern-2026-04-25.md
  - reports/sessions/omega-amend-confounded-correction-2026-04-25.md
  - reports/sessions/omega-amend-perbt-honesty-2026-04-25.md
  - reports/sessions/omega-decide-bt544-d3-strategy-postC-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: status synthesis, no claim
---

# Omega Status -- F-MOLT Unified Tally Dashboard (2026-04-25)

## §0 Non-claim disclaimer

This document is a **synthesis-only status dashboard** consolidating
the molt-validation verdicts produced across the 2026-04-25 session.

This document does **NOT**:
- produce any new molt-validation, mathematical claim, or sketch;
- re-judge, retract, override, or upgrade any prior verdict;
- modify the atlas (`design/atlas.json` or HEXA tables);
- modify `state/*` or `state/proposals/inventory.json`;
- alter the Millennium tally (remains **0/7 unchanged**);
- alter the BT-545/BT-546 main-claim status (remain MISS / PARTIAL
  per their respective `millennium-*.md`).

All cell counts, verdict strings, and falsifier-status tags below
are **transcribed verbatim** from the parent reports listed in the
front-matter; nothing is fabricated. Where a count is computed from
a sub-tally, the contributing rows are listed.

---

## §1 Master inventory (all molt-validation reports, 2026-04-25)

The 2026-04-25 session executed the following molt-validation runs
across 4 active Millennium BTs (541, 543, 544 with all D-tier
extensions, plus 545 / 546 catalogue extensions) and the BT-547
Perelman retrospective. BT-541 / BT-542 are the F-MOLT-A 3rd / 4th
samples; BT-545 / BT-546 are the catalogue extensions; BT-547 is
external/Perelman-solved (excluded from active tally per Anti-E).

### §1.1 BT-541..547 base catalogue (F-MOLT-A line)

| BT  | frame-shift / candidate              | verdict        | discriminator type           | family                  | 1-line key finding                                                                          |
|-----|--------------------------------------|----------------|------------------------------|-------------------------|---------------------------------------------------------------------------------------------|
| 541 | Lead-B SLE_6 × GUE coupling          | PASS           | distributional               | original (rank-1)       | KS distinguishability p < 0.01 vs both Brownian and Cardy; double-margin                    |
| 542 | Hirahara MCSP non-natural            | PASS_LITERATURE | structural-literature        | original (rank-1)       | L9 §4.2 met by Hirahara 2018-2022 series; molt licensed via published literature            |
| 543 | P3 A4-ratio-only on [2.5, 3.5]       | FAIL           | numerical-interval           | original (rank-1)       | 3 outliers across MP1999 / BMW 2008 / FLAG 2024; F-NP1-A4rev fires; A4 retired              |
| 544 | Q1 KdV 6-soliton Gram-lattice        | FAIL           | discrete-equality            | original (rank-1)       | rank=6 ≠ 3 AND det/σ ∉ ℤ on both κ families; F-Q1 fires; double-margin                       |
| 544 | Q5 Sobolev/Besov mechanism (rank-2)  | FAIL           | discrete-equality            | fallback (rank-2)       | "arithmetic without analysis"; F-Q5 / F-544-B fire                                          |
| 544 | KPZ d=7 lift (rank-3)                | FAIL           | discrete-equality            | fallback (rank-3)       | no literature attestation; F-KPZ / F-Q3 / F-544-C fire                                      |
| 545 | Hodge IHC failure-locus (n/φ=3)      | PASS_LITERATURE | structural-literature        | catalogue extension     | IHC verdict map split: success at φ=2, n/φ=3 (de Gaay Fortman); failure at n/φ=3 (Benoist–Ottem) |
| 546 | BSD A3D 28-stratum κ                 | PASS_DISTRIBUTIONAL | distributional           | catalogue extension     | KS p = 4e-13 vs A3D-α; max\|z\|=276.4 vs A3D-β; concentration on Z/6+Z/12 strata           |
| 546 | (Probe B 28-stratum diagnostic)      | concentration_partial | (empirical, non-fixed)  | empirical sub-probe     | Z/6 stratum largest per-cell κ=+6.30; F-A3D-uniform refuted on 6 of 19 cells                |
| 547 | M1 Ricci-flow-as-variational         | PASS retro     | structural-literature        | Perelman retrospective  | variational-derivation; closest L9 fit                                                      |
| 547 | M2 W-entropy + κ-noncollapsing       | PASS retro     | OTHER (analytic-inequality)  | Perelman retrospective  | no clean L9 vocabulary match; recommends EXT-B (analytic-Lyapunov)                          |
| 547 | M3 surgery + finite extinction       | PASS retro     | OTHER (procedure-class)      | Perelman retrospective  | parameter-bounded procedure; recommends EXT-C (procedure-class)                             |

### §1.2 BT-544 D-tier deep-dive

| sub-id   | frame-shift / candidate                              | verdict                                       | discriminator type      | tier  | 1-line key finding                                                                |
|----------|------------------------------------------------------|-----------------------------------------------|-------------------------|-------|-----------------------------------------------------------------------------------|
| D1.1     | HVC Bekenstein cap on enstrophy                      | FAIL (vacuous)                                | vacuous-magnitude       | D1    | max Φ/cap = 0.000 / 0.004 ≪ 0.1; F-D1.1-B fires (cap decorative, primitive vacuous) |
| D1.3     | NS↔MHD duality (R_m = 48 = J_2)                      | FAIL_NO_DUALITY (+ FAIL_RELABELING active)    | cross-PDE family        | D1    | MHD-→-NS reduction one-way only; "duality" relabels well-known Elsasser-frame fact |
| D1.4     | She-Leveque residual −2/9 = ζ_6 inner-product        | PASS                                          | discrete-equality       | D1    | exact arithmetic identity within 2% interval; first cross-cell PASS (discrete row) |
| D1.5     | axisym-no-swirl group-symmetry                       | catalogued, not dispatched (in-flight)        | (TBD)                   | D1    | seed registered; D1.4 §6.4 lists as remaining D1-catalogue item                   |
| D2 R1    | Onsager-Hölder threshold acceptability               | ACCEPTABLE → C3 conjecture                    | axiom-recast (L_ω)      | D2    | acceptability check passed; α*_NS ∈ (α_BV, 1/3) registered as conjecture          |
| D2 R1 L1 | BV-2019 iteration α*_NS lower bound (Lemma 1)        | OBSTRUCTION_DOCUMENTED                        | sketch                  | D2    | Phase 3 schematic too coarse; intermittency-aware retry queued                    |
| D2 R5    | Low-Mach uniform-in-ε acceptability                  | ACCEPTABLE → D2 conjecture                    | axiom-recast (L_ω)      | D2    | acceptability check passed; rate ε^α(s) registered as conjecture                  |
| D2 R5 L1 | ε-power strict-gap α(0) − α(1)                       | PASS_LITERATURE (Δα = 1 − 1/2 = 1/2)          | structural-literature   | D2    | Desjardins–Grenier 1999 / Danchin 2002 ε-bookkeeping; lemma closed via literature |
| D3.A     | uniform Sobolev on 2.5D non-local-pressure           | PASS_LITERATURE (Path P)                      | structural-literature   | D3    | F-D3-A does not fire; A-axis remains intact under literature import               |
| D3.B'    | merged stretching + intermittency (B/B fusion)       | FAIL_NO_LITERATURE_PATH + FAIL_BLOWUP_PRECEDENT | structural-literature | D3    | F-D3-B' fires; B' inherits both obstructions (Constantin–Fefferman partial; Hou–Luo blowup precedent) |
| D3.C     | Onsager/Kraichnan two-sided S_6 ∼ ℓ² bound           | FAIL_INTERMITTENCY + FAIL_RELABELING          | discrete-equality       | D3    | F-D3-META-A fires (D3 mechanism-decouple breaks); C-axis exhausted by relabeling  |
| EXT-B    | analytic-Lyapunov construction (per BT-547 retro)    | seeded, in-flight                             | analytic-inequality (OTHER) | EXT | seed pulled from BT-547 §5.2 + l9-generation-pipeline §6.2; not yet executed       |

---

## §2 F-MOLT-A tallies

### §2.1 Original 4-BT historical anchor

Per `omega-meta-audit-l9-catalogue-design-2026-04-25.md` §6, F-MOLT-A
is defined on the original 4 active BTs (541, 542, 543, 544 rank-1)
and fires iff **0 PASS in one batch run**.

| #   | BT  | rank-1 candidate          | verdict          |
|-----|-----|---------------------------|------------------|
| 1   | 541 | Lead-B SLE_6 × GUE        | PASS             |
| 2   | 542 | Hirahara MCSP             | PASS_LITERATURE  |
| 3   | 543 | P3 A4-ratio-only          | FAIL             |
| 4   | 544 | Q1 KdV Gram-lattice       | FAIL             |

**4-BT tally: 2 PASS / 2 FAIL** (50% PASS).
F-MOLT-A: **NOT FIRED** (gate fails iff 0/4 PASS; observed 2/4).

### §2.2 6-active-BT extended tally (post-545 / 546)

Catalogue extensions (BT-545, BT-546) add two more active samples
to the F-MOLT-A line. BT-547 Perelman PASSes are excluded per Anti-E
(`omega-exec-bt547-poincare-L9-retro-2026-04-25.md` §7).

| #   | BT  | candidate                       | verdict             | discriminator type            |
|-----|-----|---------------------------------|---------------------|-------------------------------|
| 1   | 541 | Lead-B SLE_6 × GUE              | PASS                | distributional                |
| 2   | 542 | Hirahara MCSP                   | PASS_LITERATURE     | structural-literature         |
| 3   | 543 | P3 A4-ratio-only                | FAIL                | numerical-interval            |
| 4   | 544 | Q1 KdV Gram-lattice             | FAIL                | discrete-equality             |
| 5   | 545 | IHC failure-locus               | PASS_LITERATURE     | structural-literature         |
| 6   | 546 | A3D 28-stratum κ                | PASS_DISTRIBUTIONAL | distributional                |

**6-BT tally: 4 PASS / 2 FAIL** (66.7% PASS).
F-MOLT-A: **NOT FIRED** (gate fails iff 0/N PASS; observed 4/6).

Note: per `omega-amend-confounded-correction-2026-04-25.md` §5.1
the F-MOLT-A tally at n=5 was 3 PASS / 2 FAIL; BT-546 PASS adds a
clean 4th PASS, sharpening the gate further from firing.

---

## §3 BT-544 D-tier + EXT-tier sub-tally

### §3.1 D-tier counts (terminal verdicts only)

Counting only **terminal** verdicts (PASS variants, FAIL variants,
ACCEPTABLE — excluding in-flight catalogued items):

| tier  | sub-id     | terminal verdict                                 |
|-------|------------|--------------------------------------------------|
| D1    | D1.1       | FAIL (vacuous)                                   |
| D1    | D1.3       | FAIL_NO_DUALITY (+ FAIL_RELABELING)              |
| D1    | D1.4       | PASS                                             |
| D2    | R1 accept. | ACCEPTABLE                                       |
| D2    | R1 L1      | OBSTRUCTION_DOCUMENTED                           |
| D2    | R5 accept. | ACCEPTABLE                                       |
| D2    | R5 L1      | PASS_LITERATURE (Δα = 1/2)                       |
| D3    | D3.A       | PASS_LITERATURE                                  |
| D3    | D3.B'      | FAIL_NO_LITERATURE_PATH + FAIL_BLOWUP_PRECEDENT  |
| D3    | D3.C       | FAIL_INTERMITTENCY + FAIL_RELABELING             |

**D-tier terminal-verdict counts:**

- D1: **1 PASS (D1.4) / 2 FAIL (D1.1, D1.3) / 1 in-flight (D1.5)**
- D2: **2 ACCEPTABLE (R1, R5)**, lemma-level: **1 PASS_LITERATURE (R5 L1) / 1 OBSTRUCTION_DOCUMENTED (R1 L1)**
- D3: **1 PASS_LITERATURE (A) / 2 FAIL (B', C)** — D3 mechanism-decouple program closed; F-D3-META-A fires

**D-tier aggregate (PASS / FAIL only, lemma + axis):**
- PASS / PASS_LITERATURE: 3 (D1.4, D2 R5 L1, D3.A)
- FAIL variants: 5 (D1.1, D1.3, D3.B' x 2 string, D3.C x 2 string — 3 distinct sub-ids in FAIL)
- ACCEPTABLE / OBSTRUCTION: 3 non-PASS / non-FAIL (D2 R1 accept., D2 R5 accept., D2 R1 L1)

Distinct-sub-id FAIL count: **3** (D1.1, D1.3, D3.B', D3.C — 4 distinct);
distinct-sub-id PASS count: **3** (D1.4, D2 R5 L1, D3.A).

### §3.2 EXT-tier sub-tally

| sub-id  | source                                              | status     |
|---------|-----------------------------------------------------|------------|
| EXT-B   | analytic-Lyapunov for BT-544 (BT-547 retro §5.2)    | in-flight  |

Per `omega-exec-bt547-poincare-L9-retro-2026-04-25.md` §5, EXT-B
"has highest urgency for BT-544". Not yet dispatched in this session.

EXT-tier terminal-verdict count: **0 PASS / 0 FAIL** (none terminal).

---

## §4 Discriminator-type 2×2 contingency

### §4.1 Pre-D1.4 table (n=8, from meta-audit §2)

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §2:

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 0    | 5    |

Fisher exact two-sided **p ≈ 0.036** (one-sided ≈ 0.018);
parent verdict: **CONFOUNDED** (per audit §9, lines 572–615; not
retracted).

### §4.2 Post-D1.4 table (n=9, from D1.4 §5.2)

Per `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md`
§5.2 — D1.4 PASS becomes the first cross-cell entry in the
discrete-equality row:

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | **1**| 5    |

Fisher exact two-sided **p ≈ 0.095** (computation in §5.3 of D1.4
report: P((3,0;1,5)) under hypergeometric on (3,6),(4,5) marginals
= 4/84 ≈ 0.0476; doubled for two-sided ≈ 0.095).

### §4.3 Effect on parent verdict

- Pre-D1.4: p ≈ 0.036 (significant at α = 0.05).
- Post-D1.4: p ≈ 0.095 (no longer significant at α = 0.05).
- **CONFOUNDED verdict UNCHANGED** (per D1.4 §5.4): the meta-audit
  already noted the data are consistent with both
  discriminator-type and candidate-validity hypotheses; D1.4
  cross-cell entry weakens the bias hypothesis but does not
  distinguish the two explanations.
- Per `omega-amend-confounded-correction-2026-04-25.md` §4.3, the
  parent CONFOUNDED verdict (audit §9) was not touched.

---

## §5 Active falsifiers

### §5.1 Meta-falsifiers

| falsifier      | scope                                            | status        | rationale                                                                                                                                                |
|----------------|--------------------------------------------------|---------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| F-MOLT-A       | gate-failure-via-validation across 4 active BTs  | NOT FIRED     | original 4-BT: 2 PASS / 2 FAIL (needs 0 PASS); 6-BT extended: 4 PASS / 2 FAIL — gate sharpens further from firing                                          |
| F-MOLT-D       | catalogue-saturation on a single BT              | FIRED on BT-544 | per `omega-exec-bt544-fallback-molt-validation-2026-04-25.md` §7 (Q1+Q5+KPZ d=7 all FAIL → top-3 catalogue exhausted on BT-544); D-tier extensions did not retract this firing — D1.4 PASS / D3.A PASS_LITERATURE are *cross-cell* / *L_ω-axiom-recast* hits, not retraction of the original rank-1/2/3 catalogue saturation. **Status preserved**. |
| F-D3-META-A    | D3 mechanism-decouple breaks                     | FIRED         | per `omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md` §4: D3.C FAIL means per-axis decoupling does not deliver content                              |
| F-D3-B'        | B' merger inherits both obstructions             | FIRED         | per `omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md` §3.1: FAIL_NO_LITERATURE_PATH + FAIL_BLOWUP_PRECEDENT both active                       |

### §5.2 Per-trial falsifiers (FIRED set)

| falsifier             | parent report                                                | status |
|-----------------------|--------------------------------------------------------------|--------|
| F-Q1                  | bt544-q1                                                     | FIRED  |
| F-P3 / F-NP1-A4rev    | bt543-p3                                                     | FIRED  |
| F-Q5 / F-544-B        | bt544-fallback                                               | FIRED  |
| F-KPZ / F-Q3 / F-544-C | bt544-fallback                                              | FIRED  |
| F-D1.1-B              | bt544-d1-1-hvc                                               | FIRED  |
| F-D3-A (negation)     | bt544-d3-A                                                   | NOT FIRED (PASS via Path P)   |
| F-D3-C                | bt544-d3-C                                                   | FIRED  |
| F-D3-B'               | bt544-d3-Bprime                                              | FIRED  |
| F-545-IHC             | bt545-hodge                                                  | NOT FIRED |
| F-545-IHC-DIM         | bt545-hodge                                                  | NOT FIRED |
| F-545-IHC-NOVEL       | bt545-hodge                                                  | NOT FIRED |
| F-NP1 (Hirahara)      | bt542-hirahara                                               | NOT FIRED |
| F-D1.4 falsifiers     | bt544-d1-4 (F-VACUOUS, F-D1.4-novelty)                       | NOT FIRED |
| F-Disp-1..6           | r1 / r5 dispatch                                             | NOT FIRED (per dispatch §8)   |
| F-VAL-A..D / F-VAL-E  | leadB validation                                             | NOT FIRED |
| F-Acc-1..n            | d2-r1r5 acceptability                                        | NOT FIRED |
| F-Sk-1..6             | r1 lemma1 BV-adapt sketch                                    | NOT FIRED |
| F-AMEND-V2            | confounded-correction self-falsifier                         | armed (NOT FIRED) |
| F-RETRO-A..G          | bt547 retro self-falsifiers                                  | NOT FIRED (F-RETRO-B partially active, conditional on M3 independence) |

---

## §6 Cross-BT n/φ pattern audit verdict (reference)

Per `omega-meta-audit-cross-bt-n-phi-pattern-2026-04-25.md` §6:
**PARTIAL_STRUCTURAL** (chosen from {REAL_STRUCTURAL,
PARTIAL_STRUCTURAL, NUMEROLOGY, CONFOUNDED}).

Recorded here as a **reference verdict** — orthogonal to the 2×2
discriminator-type axis. The PARTIAL_STRUCTURAL verdict notes that
n/φ = 3 mentions across BT-544/545/546 carry *some* structural
weight (e.g. IHC verdict split at dim 3 in BT-545) but do not
support a uniform H_real predictor; this is consistent with — and
neither strengthens nor weakens — the CONFOUNDED 2×2 verdict.

---

## §7 In-flight tasks

Tasks that are seeded / dispatched but have no terminal verdict as
of 2026-04-25:

1. **D1.3 NS↔MHD duality** — terminal FAIL_NO_DUALITY recorded;
   per parent report §3.2, no `PASS_LITERATURE` path is reachable
   without new transfer theorem; further attempts on D1.3 not
   queued. (treated as TERMINATED FAIL, not in-flight)
2. **D1.5 axisym-no-swirl group-symmetry** — catalogued in D1.4
   §6.4 / §6.4 list; not dispatched. **In-flight (catalogued)**.
3. **D2 R1 Lemma 1 retry** (intermittency-aware adaptation) —
   per `omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md` §7.2 #1
   and §11 closing: "next-session task". **In-flight**.
4. **EXT-B analytic-Lyapunov for BT-544** — per BT-547 retro §5
   "highest urgency for BT-544"; seed in
   `omega-meta-audit-l9-generation-pipeline-2026-04-25.md` §6.2.
   **In-flight (seeded, not dispatched)**.
5. **D2 R1 / R5 conjecture-level work** (C3 Onsager-Hölder window;
   D2 ε^α(s) low-Mach rate) — registered as conjectures only; no
   proof attempt scheduled. **In-flight (open research)**.
6. **n ≥ 16 native-pairing resample** for 2×2 confounding — per
   `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §5.5.
   Not executed; would supersede CONFOUNDED if cleanly separating.
   **In-flight (not started)**.
7. **F-MOLT-A 4-BT firing-condition retire** (per FIX-D in
   l9-catalogue-design audit §5.4) — open governance question on
   whether to retire the original 4-BT pre-condition now that the
   6-BT extended tally is in. **In-flight (governance)**.

(7 in-flight tasks; none terminal.)

---

## §8 Status verdict

**L9-framework state at 2026-04-25 EOD: F-MOLT-A NOT FIRED on either
the 4-BT (2/2) or 6-BT (4/2) tally; F-MOLT-D FIRED on BT-544 only;
D-tier closure on BT-544 = 1 cross-cell PASS (D1.4) + 1 axis
PASS_LITERATURE (D3.A) + 1 lemma PASS_LITERATURE (D2 R5 L1) +
3 FAILs (D1.1, D1.3, D3.C) + 1 obstructed-FAIL merger (D3.B') +
1 sketch OBSTRUCTION (D2 R1 L1); discriminator-type 2×2 post-D1.4
= [3,0; 1,5] with Fisher two-sided p ≈ 0.095 (CONFOUNDED parent
verdict preserved); cross-BT n/φ audit = PARTIAL_STRUCTURAL; 7
in-flight tasks; Millennium tally 0/7 unchanged.**

---

## §9 Anti-list — validations not yet attempted

The following BT × tier combinations are **not** validated as of
2026-04-25; this dashboard records absence, not failure:

- **BT-544 D1.2** (Pólya recurrence): catalogued (D1.4 §6.4),
  not dispatched.
- **BT-544 D1.5** (axisym-no-swirl): catalogued, not dispatched
  (also listed in §7).
- **BT-544 Q2** (per D1.4 §6.4 "next-cheapest dispatch"):
  catalogued, not dispatched.
- **BT-545 D-tier** (Hodge D-tier extensions): not designed; only
  the IHC catalogue extension (PASS_LITERATURE) is on record.
- **BT-546 D-tier** (BSD D-tier extensions beyond Probe B
  empirical): not designed.
- **BT-541 / BT-542 / BT-543 D-tier**: not designed; D-tier
  expansion was BT-544-specific in this session.
- **EXT-A** (variational re-interpretation; per BT-547 retro §5):
  not seeded.
- **EXT-B** (analytic-Lyapunov for BT-543 chromomagnetic; per
  l9-generation-pipeline §6.2): seeded, not dispatched.
- **EXT-C** (procedure-class with parameter bounds; per BT-547
  retro §5): not seeded.
- **EXT-D** (vocabulary extension OTHER → analytic-inequality /
  procedure-class; per BT-547 retro §5): not seeded.
- **n ≥ 16 native-pairing resample** for 2×2 (per audit §5.5):
  not started.
- **R1 Lemma 1 intermittency-aware retry** (per R1 BV-adapt §7.2
  #1): queued, not started.
- **D-tier on BT-547**: out of scope (Perelman-solved).

---

## §10 Closing

- Millennium tally: **0/7 unchanged**.
- No `design/atlas.json` edits.
- No `state/*` edits.
- No `state/proposals/inventory.json` edits.
- This is a status synthesis only; all verdicts trace back to the
  parent reports listed in front-matter and remain authoritative
  there.
- For machine-readable consumption, the master inventory in §1.1
  and §1.2 is the canonical join key; the 2×2 in §4.2 is the
  current contingency state; the in-flight list in §7 is the
  current open-task queue.
