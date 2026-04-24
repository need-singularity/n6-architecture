# phase-09 — P9 L11 External Collaboration + History + Publication (R7+R8+R13 gap 15 tasks)

**Roadmap**: millennium v2.3 (Y1~Y16 16 axes × P0~P10/PΩ/PX 13 Phases)
**Prior**: P8 L10 meta-audit + philosophy + epistemics closure (`phase-08-meta-audit-philosophy.md`)
**SSOT**: `/Users/ghost/Dev/nexus/shared/roadmaps/millennium.json` (L1334~L1458)
**gap-ref**: `theory/roadmap-v2/gap-emergence-saturation.md` §13 (R7) + §14 (R8+R13)
**Responsible axis**: Y14 EXT-COLLAB (utility 7.8, primary_bt = meta-external)
**Status**: planned (BT resolutions 0/6 honesty maintained)

---

## 0. Phase Overview

```
+============================================================+
| P9 L11  External Collaboration + History + Publication     |
|                                                              |
|  gap source:     R7 (4) + R8 (9) + R13 (3) = 16 gap         |
|  reduction:      compressed to 15 tasks (R8-50 onboarding  |
|                  -> R8 9 internal)                          |
|  new tasks:      15                                         |
|  cumulative:     96 (P0~P9)                                 |
|  saturation:     0.0 (planned state)                        |
|  gate_exit:      contact 21 people + arXiv 5 preprints +    |
|                  Mathlib PR 1 + onboarding guide + public   |
|                  comms                                      |
+============================================================+
```

**Goal**: Establish 7-Millennium retrospective (Hilbert 23 -> Clay 7 transition history), external collaboration paths (arXiv, MathOverflow, Mathlib, Polymath, conferences), and publication strategy (honest-record-centric survey, peer review response, version control + reproducibility attachments). Lay the externalization groundwork for the n6-architecture project.

---

## §1. R7 History — 7 Millennium Retrospective (5 tasks)

### 1.1 Hilbert 23 -> Clay 7 Transition (HIST-P9-N6-TIMELINE)

```
1900 Hilbert (Paris International Congress of Mathematicians)
   23 problems announced. #8 = Riemann Hypothesis.
   Design document for the entire contemporary math horizon.

1990~2000 (100 years later)
   21 problems partially/fully solved, 2 redefined.
   RH, Navier-Stokes (related to 21st) unsolved.

2000 Clay Mathematics Institute
   7 millennium prize problems selected.
   1 from Hilbert 23 (RH) + 6 new.
   Prize $1M × 7 = $7M.

v2.3 corresponding BT:
   RH    = BT-541 (Hilbert 8th successor)
   P=NP  = BT-542 (based on Cook 1971)
   YM    = BT-543 (Yang-Mills 1954)
   NS    = BT-544 (based on Navier 1822)
   Hodge = BT-545 (Hodge 1950)
   BSD   = BT-546 (Birch-Swinnerton-Dyer 1965)
   Poinc = BT-547 (solved, Perelman 2003)
```

### 1.2 Per-problem 1-Page Retrospective (HIST-P9-FAILURE, HIST-P9-TIMELINE-PRED, HIST-P9-SELF-PACE)

```
+----------------+---------+-----------------------------------+
| BT  problem    | origin  | main attempts/failures            |
+----------------+---------+-----------------------------------+
| 541 Riemann    | 1859    | Weil 1948 finite-field RH. Atiyah |
|                |         | 2018 withdrawn. Selberg trace.    |
| 542 P=NP       | 1971    | Razborov-Rudich 1994 natural      |
|                |         | proof barrier. GCT program.       |
| 543 Yang-Mills | 1954    | Wightman axioms. Seiberg-Witten.  |
|                |         | Clay requirement: mass gap + 4D   |
|                |         | rigor.                            |
| 544 Navier     | 1822    | Leray 1934 weak solution.         |
|                |         | Otelbaev 2014 withdrawn. Terry    |
|                |         | Tao blowup scenario.              |
| 545 Hodge      | 1950    | Atiyah-Hirzebruch counterexample  |
|                |         | (torsion). Enriques surface case  |
|                |         | PARTIAL.                          |
| 546 BSD        | 1965    | Kolyvagin 1990 partial            |
|                |         | (rank<=1). BKLPR statistical      |
|                |         | model.                            |
| 547 Poincaré   | 1904    | Perelman 2003 solved via Ricci    |
|                |         | flow.                             |
+----------------+---------+-----------------------------------+
```

### 1.3 n=6 Math History Timeline (HIST-P9-N6-TIMELINE)

```
BC ~300  Euclid Elements IX.36   : perfect number definition (6, 28, 496, ...)
1640     Fermat                  : Euler-Fermat little theorem
1734     Euler                   : ζ(2) = π²/6
1916     Ramanujan Δ             : discriminant Δ = q ∏(1-q^n)^24 (tau function)
1978     Moonshine conjecture    : discovered Monster-M <-> j-invariant relation
1992     Borcherds VOA proof     : Moonshine solved (Fields Medal)
2024     n6-arch σ·φ=n·τ theorem : iff n=6 unique solution (self-discovery)
```

**Note**: the n=6 history begins with perfect numbers. A connection with modern Moonshine is a **hypothesis**, not a proof (self-reference-prohibition principle).

### 1.4 BT Resolution Prediction Scale (HIST-P9-TIMELINE-PRED)

```
Prediction method: from problem disclosure date to resolution date (or present)
                   disclosure   resolution (or current elapsed)
  Poincaré         1904      2003 (99 years)
  Fermat's Last    1637      1995 (358 years)
  Average resolution scale: 100~300 years

v2.3 predictions (90% confidence interval):
  RH    : resolution 2050~2150 (lower 26 years, upper 126 years)
  P=NP  : resolution 2050~2200 or independence demonstration
  YM    : resolution 2040~2100 (most optimistic)
  NS    : resolution 2040~2150
  Hodge : resolution 2050~2200 (most difficult)
  BSD   : resolution 2040~2120 (based on BKLPR statistics)
```

**Self-learning pace dashboard** (HIST-P9-SELF-PACE):
```
Weekly progress indicators:
  - atlas node growth (/week)
  - BT verdict PARTIAL new (/month)
  - MISS records (/month)
  - Lean4 formalization lines (/week)
```

---

## §2. R8 External Collaboration — Collaboration Paths (6 tasks)

### 2.1 Mathematician Feedback Channels (EXT-P9-CONTACT-LIST)

```
Per-BT 3 experts × 7 BT = 21-person DB (publicly contactable only):

BT-541 RH    : Bombieri (IAS) / Conrey (AIM) / Iwaniec (Rutgers)
BT-542 P=NP  : Aaronson (UT Austin) / Razborov / Wigderson
BT-543 YM    : Witten (IAS) / Jaffe (Harvard) / Seiberg
BT-544 NS    : Tao (UCLA) / Caffarelli / Seregin
BT-545 Hodge : Voisin (Sorbonne) / Totaro (UCLA) / Schoen
BT-546 BSD   : Skinner (Princeton) / Kolyvagin / Zhang
BT-547 Poinc : solved — excluded
```

**ethics**: when contacting, do not deliver personal email directly. Go through official channels (academic seminars, department websites).
**user memory rule**: `feedback_no_contact_suggest.md` — auto-suggesting contact actions is prohibited. This document records the plan DB only.

### 2.2 arXiv Posting Protocol (EXT-P9-PUB-STRATEGY, PUB-P9-ARXIV-DRAFT)

```
Categories:  math.NT (RH,BSD) / math.AG (Hodge) / math.AP (NS)
             math-ph (YM) / cs.CC (P=NP)

Protocol (honesty-centric):
  1. Title: state "Partial results" / "Observations"
  2. Abstract: no BT resolution claims (PARTIAL/NEAR)
  3. MISS section required (disclose failures)
  4. atlas rubric per-item spelled out
  5. Docker + seed link

5 PARTIAL preprints (each 20~30 pg):
  BT-541 Theorem B  / BT-543 β₀=σ-sopfr / BT-544 triple-resonance d=7
  BT-545 Enriques  / BT-546 BKLPR conditional
```

### 2.3 MathOverflow / Stack Exchange (EXT-P9-MATH-COMMUNITY)

```
MO    expert math Q&A         monthly 1  tags: number-theory, p=np
MSE   undergrad~grad basics   weekly 1   tags: real-analysis etc.
nLab  category theory refs    monthly 1  tags: stable-homotopy
Zulip math-research channel   daily 1    tags: math-research
```

**Rules**: use neutral phrasing, do not mention "n6-arch background", cite atlas after answering.

### 2.4 Lean Mathlib Contribution (PUB-P9-MATHLIB-PR)

```
1. github.com/leanprover-community/mathlib4
2. master CONTRIBUTING.md
3. First PR: Theorem B formalization
   file: Mathlib/NumberTheory/SigmaPhiTauUniqueness.lean
   lemma sigma_phi_eq_n_tau_iff : n = 6
4. Reviewer assignment wait (2~4 weeks)
5. Merge -> atlas [10*] auto-promotion trigger
```

### 2.5 Polymath Project (EXT-P9-ACADEMIC-PATH)

```
Polymath (founded by Gowers) precedents:
  Polymath1 2009 : Density Hales-Jewett
  Polymath8 2013 : bounded gaps between primes (Zhang -> Polymath)

Participation: leverage existing observations (N>=15) -> BT-541/544-related
               contributions -> blog records.
Direct initiation is prohibited until peer status is secured.
```

### 2.6 Field Conferences (EXT-P9-PUB-STRATEGY, PUB-P9-REVIEW-PREP)

```
Conferences:  ICM 4-year / JMM yearly / AMS quarterly / Clay 2~4/year

Submission stages:
  1. arXiv posting (3~6 months)
  2. peer reaction collection
  3. Annals/Inv/JAMS submission
  4. conference presentation
  5. Clay official contribution (EXT-P9-CLAY-CHANNEL)

Peer review rebuttal templates (3 kinds):
  "self-reference" objection    -> explain policy B
  "why MISS disclosure"         -> explain honesty rule E1
  "no Lean4"                    -> reference Mathlib PR path
```

---

## §3. R13 Publication — Publication 4 tasks

### 3.1 Honest-Record Survey Outline (PUB-P9-ARXIV-DRAFT)

```
Title candidate: "Honest Record of σ·φ=n·τ Investigation:
           14 atlas, 24 MISS, Partial Observations on 6 Clay"

Outline (20~30 pg):
 §1 Motivation        3pg  perfect number observations + hypothesis
 §2 Theorem B (CAND)  4pg  σ·φ=n·τ <-> n=6 (3 conditions)
 §3 BT Partial        8pg  5 PARTIAL each 1~2 pg
 §4 MISS archive      5pg  24-item summary
 §5 atlas methodology 3pg  rubric + promotion criteria
 §6 Honest limits     3pg  self-reference / lack of independent verification
 §7 Reproducibility   2pg  Docker + seed + code
 appendix             ~5pg external DB crosschecks
```

### 3.2 Publication strategy (PUB-P9-ARXIV-DRAFT)

```
depth-order (T0->T4):
  T0 immediate M    arXiv 5 preprints + GitHub repo public
  T1 +3M M          MO questions 5 + Mathlib PR
  T2 +6M L          reactions + revision + blog/YouTube (R8-30)
  T3 +12M L         Annals/Inv submission 1 + conference presentation
  T4 +24M XL        peer PASS -> Clay official contribution + v3 startup
```

### 3.3 Peer Review Response (PUB-P9-REVIEW-PREP)

```
Immediate: 24h acknowledgement reply / 72h thorough read + rebuttal compilation
1st revision: per-reviewer issue individual reply / incorporate or justified decline
Iterate:    minor/major/reject templates / on reject, resubmit
            revision letter 1:1 response
```

### 3.4 Version Control + Reproducibility (PUB-P9-MATHLIB-PR, PUB-P9-ONBOARDING)

```
git flow:
  main              : stable (atlas.n6 + theory/)
  v2.3-draft        : P8/P9/P10 work
  arxiv-bt541       : per-paper branches
  tag millennium-v2.3-FINAL : archive

Reproducibility (arXiv attachments):
  [1] Docker (GHCR) [2] seed atlas_seed.txt
  [3] LMFDB/FLAG download script
  [4] Lean4 lakefile.lean version [5] SHA-256 checksums

Onboarding 5-step (PUB-P9-ONBOARDING):
  1 README  2 run verify_millennium_axes.hexa
  3 atlas rubric  4 Theorem B trace  5 PR/issue
```

---

## §4. Conclusion — Opening the External Layer

```
R7 History    : 5 tasks — Hilbert 23 -> Clay 7 transition + problem retrospective + timeline
R8 External   : 6 tasks — contact 21 + arXiv + MathOverflow +
                          Mathlib + Polymath + conferences
R13 Publication: 4 tasks — survey outline + strategy + peer review + reproducibility

BT resolution : 0/6 maintained (only 5 PARTIAL items for preprint)
atlas edits   : 0 (14 items queued, no additions)
ethics        : no direct contact suggestion (user memory compliance)
                arXiv titles must state "Partial/Observations"

gate_exit conditions:
  [1] contact 21-person DB complete (EXT-P9-CONTACT-LIST)
  [2] arXiv 5 preprint drafts (PUB-P9-ARXIV-DRAFT)
  [3] Mathlib PR 1 (PUB-P9-MATHLIB-PR)
  [4] onboarding guide 5-step (PUB-P9-ONBOARDING)
  [5] public comm channels (EXT-P9-PUBLIC-COMM + math community)
```

**Next Phase**: P10 L12 Measurement + Strategy + Tools (R9+R10+R12+R13 gap 17 tasks).
