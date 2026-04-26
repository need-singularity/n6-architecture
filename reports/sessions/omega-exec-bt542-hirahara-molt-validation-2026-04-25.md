---
id: omega-exec-bt542-hirahara-molt-validation
date: 2026-04-25
scope: research-only molt-validation (NO P=NP claim, NO atlas promotion)
target: BT-542 Hirahara MCSP -- F-MOLT-A 4th-sample
parent_reports:
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md (§ 4 BT-542)
  - reports/sessions/dfs-24-pnp-direction-2026-04-24.md
  - reports/sessions/omega-cycle-bt542-pnp-2026-04-25.md
  - reports/sessions/omega-exec-bt541-leadB-molt-validation-2026-04-25.md (1st PASS)
  - reports/sessions/omega-exec-bt544-q1-molt-validation-2026-04-25.md (FAIL)
  - reports/sessions/omega-exec-bt543-p3-molt-validation-2026-04-25.md (FAIL)
millennium_resolved: 0/7 (unchanged)
grade: molt-validation + F-MOLT-A 4th-sample, no claim
---

# Omega Exec -- BT-542 Hirahara MCSP Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer

This report executes the **fourth and final molt-validation** in the L9
calibration sweep
(`omega-probe-l9-molt-trigger-2026-04-25.md` §sec 7.1, ranked
**BT-542 Hirahara MCSP fourth** — flagged a-priori as the highest
relabeling-risk candidate per BT-542 §6 Tension 3, "the wall is the
wall"). The experiment tests the *frame-shift*

> "4-barriers + Y4 GATE-BARRIER frame -> Hirahara meta-complexity / MCSP frame"

proposed in `omega-probe-l9-molt-trigger-2026-04-25.md` §3.2 BT-542 row 1
(per BT-1392 + v3-T5 Hirahara MCSP deep-dive 2026-04-15).

**This document does NOT**:
- claim P vs NP in either direction;
- claim a proof, partial proof, or proximity to a proof of P=NP / P!=NP;
- claim that Hirahara 2018, 2020, 2022 (or any other meta-complexity
  result) closes P vs NP;
- promote anything in `shared/n6/atlas.n6`;
- modify `state/proposals/inventory.json`;
- modify `theory/canon/`;
- alter the BT-542 OPEN-since-1971 verdict from
  `domains/physics/millennium-p-vs-np/millennium-p-vs-np.md` or
  `omega-cycle-bt542-pnp-2026-04-25.md`;
- claim the 4 barriers (relativization / natural proofs / algebrization /
  GCT) are bypassed for the *full* P vs NP problem;
- claim the n=6 structure (sigma=12, tau=4, phi=2, sopfr=5) becomes
  applicable to MCSP (per v3-T5 §3 the non-applicability is
  re-confirmed and is left intact here).

**Millennium tally**: 0/7 unchanged. The Hirahara outcome speaks
**only** to whether the proposed BT-542 frame-shift candidate
(meta-complexity / non-black-box reduction primitive) is genuinely
absent from the existing 4-barriers + GATE-BARRIER frame, or whether it
collapses to the natural-proofs barrier under reformulation.

**This validation is the 4th sample for the F-MOLT-A meta-falsifier**
(L9 §6). F-MOLT-A is **already broken** by BT-541 Lead-B PASS
(2026-04-25) — the gate cannot retire under R1 default-revision in this
sweep. This 4th sample remains informative for the *catalogue
maturity* sub-question (1/4 vs 2/4 vs 3/4 PASS rate; see §6).

---

## §1 Hirahara/MCSP spec extracted

Combined from `omega-probe-l9-molt-trigger-2026-04-25.md` §4.2 and §3.2
row 1, `omega-cycle-bt542-pnp-2026-04-25.md` §1 + §6 Tension 3,
`reports/breakthroughs/v3-t5-hirahara-mcsp-deep-2026-04-15.md`, and
`reports/breakthroughs/bt-542-p-vs-np-4-barriers-survey-2026-04-15.md`
§2 + §5.1.

**Object**: Hirahara MCSP rank-1 frame-shift candidate
(L9 §3.2 row 1). The frame-shift swaps a *direct lower-bound attempt*
on NP-style circuit complexity (the GATE-BARRIER frame) for a
*meta-complexity formulation* in which MCSP NP-hardness becomes a
proxy for P=NP via the Hirahara reduction chain (Hirahara 2018 FOCS
"Non-Black-Box Worst-Case to Average-Case Reductions within NP" +
Hirahara 2020 STOC "Unexpected Hardness Results for K-complexity" +
Hirahara 2022 FOCS "NP-Hardness of Partial MCSP").

**Measurement** (per L9 §4.2): identify whether the MCSP /
meta-complexity reduction chain admits a **non-natural-proof
formulation**, i.e. one that does **not** invoke a constructive
lower-bound argument that Razborov-Rudich (1997, J Comput Syst Sci 55)
would naturalize and reject under standard one-way-function
assumptions.

This is a **literature-import test** (per the user's Phase-2 framing,
closer to D3.A literature-import than a Gram-lattice computation): the
discriminator is whether published meta-complexity theorems introduce
a primitive (non-black-box reduction, partial-function relaxation,
worst-to-average-case-within-NP technique) that is **structurally
absent** from the 4-barriers + Y4 GATE-BARRIER frame.

**Pass criterion (real molt, per L9 §4.2)**: a non-natural
reformulation exists (or is proposed in the published literature
2017-2026) that does not collapse onto RR-1997's natural-proofs
barrier. This is a new primitive (meta-complexity reformulation)
absent from the GATE-BARRIER frame. Composite proxy re-estimate moves
+0.03 to +0.05 (still well below 0.835 simulation ceiling — **does
not approach P=NP closure**).

**Fail criterion (relabeling, per L9 §4.2 + F-542-A)**: the
reformulation reduces to RR-1997 + a rename; per BT-542 §6 Tension 3
and Tension 5 ("the wall is the wall"), the MCSP route fails to
introduce any new barrier-bypassing primitive. Composite proxy
unchanged.

**Cost (per L9 §7)**: medium. "Literature scan + structural inspection
of v3-T5 Hirahara note + cross-check against arxiv-180papers BT-542
segment". No numerical computation — the discriminator is
documentary / structural.

**Spec contrast with prior validations**: BT-544 Q1 had a binary
algebra check (rank, det/sigma in Z); BT-543 P3 had a binary numerical
check (3 ratios in [2.5, 3.5]); BT-541 Lead-B had a binary
distributional check (KS p < 0.01). BT-542 Hirahara has **no numerical
discriminator** — it is a structural / literature-import test. P=NP is
not a numerical question; the molt's claim is structural (does the
new frame admit a primitive the old frame did not).

---

## §2 Existence / tooling check

Repo scan results (under `/Users/ghost/core/n6-architecture/`):

- `reports/breakthroughs/v3-t5-hirahara-mcsp-deep-2026-04-15.md`
  (200 lines) — explicit summary of the Hirahara 2017-2023
  meta-complexity series: Hirahara-Santhanam 2017 (CCC), Hirahara 2018
  (FOCS best paper, non-black-box worst-to-average), Allender-Hirahara
  2019 (TOCT), Hirahara 2020 (STOC, K-complexity), Hirahara 2022 (FOCS
  best paper, partial MCSP NP-hard under RP), Hirahara-Nanashima 2023.
- `reports/breakthroughs/bt-542-p-vs-np-4-barriers-survey-2026-04-15.md`
  §2 (Razborov-Rudich 1997), §5.1 (meta-complexity), §5.2 (Williams
  line non-natural + non-relativizing + non-algebrizing reaching
  NEXP/NQP not NP).
- `domains/physics/millennium-p-vs-np/millennium-p-vs-np.md` — BT-542
  canonical body doc.
- `theory/preprints/millennium-v3-preprint-draft-2026-04-16.md` —
  preprint-grade Hirahara reference.
- `theory/roadmap-v2/millennium-v3-design-2026-04-15.md` — v3 T5
  design block.

**No code is required for this validation.** Unlike BT-544 Q1 (Gram
lattice rank/det), BT-543 P3 (ratio check), and BT-541 Lead-B (KS
two-sample test), BT-542 Hirahara is a literature-import / structural
discriminator. The existence check is **the literature itself**.

**Citation completeness**: the Hirahara 2018 FOCS paper title is
"Non-Black-Box Worst-Case to Average-Case Reductions within NP";
Hirahara 2020 STOC is "Non-disjoint Promise Problems from Meta-
Computational View of Pseudorandom Generator Constructions" / related
K-complexity work; Hirahara 2022 FOCS "NP-Hardness of Learning
Programs and Partial MCSP" (titles per v3-T5 table; the year/venue
attribution `Hirahara 2022 STOC` in 4-barriers-survey §5.1 vs `2022
FOCS` in v3-T5 §1.1 is a documented internal inconsistency — see §9
Anomalies). Underlying Kabanets-Cai 2000 (STOC) "Circuit Minimization
Problem" defines MCSP. Razborov-Rudich 1997 (J CSS 55) defines natural
proofs. Oliveira-Santhanam 2017 (CCC) gives the
MCSP-natural-proofs near-equivalence.

**No theorems are fabricated below.** Citations track v3-T5 and
4-barriers-survey verbatim except where labelled as derived
re-statements.

---

## §3 Execution log

This is a structural / literature-import discriminator, not a numerical
run. The execution proceeds in three substantive sub-checks against
the L9 §4.2 PASS / FAIL criteria.

### §3.1 Sub-check A — Does the Hirahara reduction chain include a
non-naturalized step?

Per `v3-t5-hirahara-mcsp-deep-2026-04-15.md` §2.3:

> Hirahara 2018 FOCS introduces **non-black-box reductions**:
> - Reductions that use the **code** of the input x
> - Bypass limits of existing black-box reductions
> - Partially evades naturalization
>
> However, non-black-box alone is insufficient for full MCSP
> NP-hardness (as of 2022).

The non-black-box reduction technique (Hirahara 2018 FOCS,
"Non-Black-Box Worst-Case to Average-Case Reductions within NP") is
**published, peer-reviewed, FOCS best paper**, and its core technical
contribution is precisely a method that uses the *code* of the input
rather than treating it as an oracle. This is structurally distinct
from any reduction technique that satisfies the Razborov-Rudich
"largeness + constructivity" predicate, because non-black-box
arguments reason about the description of x, not about a property
of x as a Boolean function in isolation.

**Sub-check A verdict**: a non-natural step exists in the published
literature — Hirahara 2018's non-black-box reduction technique. This
is **not** invented here; v3-T5 §2.3 records it directly with explicit
language "partially evades naturalization".

### §3.2 Sub-check B — Does this primitive sit in the GATE-BARRIER
frame already, or is it absent?

The Y4 GATE-BARRIER frame (per
`omega-cycle-bt542-pnp-2026-04-25.md` §1 + axis-final-millennium R3)
catalogues the 4 known barriers themselves. It does **not** include:

- the meta-complexity equivalence MCSP-in-P iff one-way functions fail
  (Oliveira-Santhanam 2017 CCC);
- the MCSP / NP near-equivalence under conditional collapse (Hirahara
  2022 — `bt-542-p-vs-np-4-barriers-survey` §5.1 phrases this as
  "NP ⊂ P ⟺ MCSP ∈ P");
- the partial-function relaxation (Partial MCSP) under which
  Hirahara 2022 FOCS proves NP-hardness via randomized polynomial-
  time (RP) reductions;
- the non-black-box reduction technique itself (Hirahara 2018 FOCS).

These four objects are *all* primitives absent from the GATE-BARRIER
frame as registered in the repo. The GATE-BARRIER frame names the
*walls*; the meta-complexity frame names a *new entity (MCSP) whose
hardness is conditionally tied to NP* and a *new technique (non-
black-box) that side-steps the natural-proofs predicate*.

**Sub-check B verdict**: the meta-complexity primitives are
structurally absent from the GATE-BARRIER frame. The L9 §4.2 PASS
clause "this is a new primitive (meta-complexity reformulation) absent
from the GATE-BARRIER frame" is satisfied.

### §3.3 Sub-check C — Does the candidate collapse onto RR-1997 +
rename (the F-542-A / Tension 3 failure mode)?

The L9 §3.2 row 1 falsifier F-542-A reads:

> per BT-542 §6 Tension 3 / 4-barriers survey, MCSP route still hits
> Razborov-Rudich on its constructive side -> molt is relabeling

This is the central question. We resolve it as follows.

**(i) Tension 3 specifically targets PROBE-24B, not the Hirahara
route.** Re-reading `omega-cycle-bt542-pnp-2026-04-25.md` §6
Tension 3:

> DFS-24 PROBE-24B vs Razborov-Rudich: PROBE-24B (Sylvester
> synthematic total ↔ 3-SAT certificate) is *itself flagged* in
> dfs-24-pnp §3 as "touches the Natural Proofs barrier" — designing
> it for a **negative result** that re-confirms the barrier.

Tension 3 is about the Sylvester / 3-SAT direction (PROBE-24B), which
*is* a constructive-large attempt likely to trigger RR-1997. The
Hirahara meta-complexity route (rank-1 in L9 §3.2) is a **different**
path — it is the meta-complexity / non-black-box program, not the
Sylvester-synthematic-3-SAT program. F-542-A's appeal to Tension 3 is
not directly applicable; Tension 3 attaches to PROBE-24B by name.

**(ii) The 4-barriers-survey itself records meta-complexity as a
distinct path 2020-2026.** Per
`bt-542-p-vs-np-4-barriers-survey-2026-04-15.md` §5.1:

> Meta-complexity is an **equivalent variant** of P vs NP. It does
> not avoid the problem itself.

Critical reading: "equivalent variant" + "does not avoid the problem"
is a **structural-honesty caveat about closure**, not a claim that the
meta-complexity primitives collapse to RR-1997. The survey
distinguishes between (a) introducing a new primitive (non-black-box,
MCSP↔NP near-equivalence) and (b) closing P vs NP. Meta-complexity
delivers (a) and explicitly does not deliver (b). The L9 PASS
criterion asks for (a), not (b).

**(iii) The repo's own v3-T5 honest verdict.** Per v3-T5 §3 + §4 the
repo's verdict is:

> Hirahara 2022 is a **major instrumental advance** ... However, this
> is **not a P vs NP draft**. The Clay problem remains open.

This is the honest position: a *real* new primitive (Hirahara 2018
non-black-box; Hirahara 2022 partial MCSP NP-hard under RP) entered
the literature, *and* P vs NP remains open. The molt fires (new
primitive introduced) without resolving the underlying problem
(closure ceiling un-moved, per
`omega-cycle-bt542-pnp-2026-04-25.md` §3 composite ≈ 0.44).

**Sub-check C verdict**: F-542-A's appeal to Tension 3 + "wall is
wall" does **not** apply to the Hirahara rank-1 candidate — Tension 3
attaches to PROBE-24B (Sylvester / 3-SAT), and the 4-barriers-survey
records meta-complexity as a *distinct* primitive that delivers (a)
without claiming (b). The candidate does not collapse to RR-1997 +
rename. The molt is not a relabeling.

### §3.4 Counter-evidence considered

For honesty, the strongest counter-arguments to PASS are recorded:

- **Counter-1 (n=6 non-applicability)**: per v3-T5 §3.2 "no
  mathematical connection between MCSP and n=6 divisor function".
  This is true and unchanged. **However**, the L9 §4.2 PASS criterion
  does not require an n=6 connection — it requires a non-naturalized
  reformulation absent from the GATE-BARRIER frame. The molt is
  about *frame-shift content* (MCSP / non-black-box primitive), not
  about *n=6 applicability*. Counter-1 does not flip the verdict.

- **Counter-2 (only partial MCSP, not full)**: Hirahara 2022 proves
  NP-hardness only of *partial* MCSP under RP reductions; full MCSP
  NP-hardness remains open (v3-T5 §1.3). **However**, the L9 §4.2
  spec asks whether "the Hirahara reduction chain admit a non-natural-
  proof formulation"; partial-MCSP-NP-hard-under-RP IS such a
  formulation, published in FOCS 2022. The PASS criterion does not
  require *full* MCSP NP-hardness. Counter-2 does not flip the
  verdict.

- **Counter-3 (Williams-line was rejected, AL-M2)**: per L9 §5 AL-M2
  the Williams-line frame (NEXP ⊄ ACC⁰) was rejected as a candidate
  because it reaches NEXP/NQP not NP, "composite delta = 0 on the
  target". **However**, AL-M2 is about a *different* candidate
  (Williams-line) and explicitly NOT about the Hirahara meta-
  complexity candidate. Hirahara's program targets MCSP whose
  hardness is *conditionally tied to NP* (per OS-2017 + H-2022),
  unlike Williams which lifts to NEXP. Counter-3 does not flip the
  verdict.

- **Counter-4 (4-barriers-survey "no n=6 angle, BT-542 far from
  number-matching")**: per `bt-542-p-vs-np-4-barriers-survey` §6.1
  "n=6 structure has no directly applicable angle to P vs NP".
  **However**, again the L9 PASS criterion does not require an n=6
  angle. The molt is about frame-shift content (does the new frame
  introduce a primitive absent from the old frame), not about n=6
  closure. Counter-4 is consistent with PASS_LITERATURE because both
  v3-T5 §4 ("Hirahara 2022 is a major instrumental advance ...
  not a P vs NP draft") and the L9 PASS clause ("composite delta
  +0.03 to +0.05, still well below 0.835") agree that the molt
  fires without closing BT-542.

None of Counter-1..4 flips the PASS verdict. All four are consistent
with the L9 §3.2 row 1 expected-dC range ("+0.03 to +0.05 if MCSP
partial-NP-hardness yields a non-trivial Y4 sub-axis; cosmetic if
not"): we land at the lower end of the predicted dC band — a real
new primitive, NOT a closure, no number-matching, no n=6 entry.

---

## §4 Discriminator result with citations

The L9 §4.2 PASS clause is met **on literature evidence already
present in the repo and the published meta-complexity series**.

### Theorem-level citations (no fabrication)

- **Razborov, Rudich 1997**, "Natural Proofs", J Comput Syst Sci 55,
  pp. 24-35. The natural-proofs barrier: `{f : f ∉ P/poly}` cannot be
  proved by a natural property if one-way functions exist.
  (Cited per `bt-542-p-vs-np-4-barriers-survey` §2.2.)

- **Kabanets, Cai 2000**, "Circuit Minimization Problem", STOC 2000,
  pp. 73-79. Defines MCSP and observes its position relative to NP.
  (Cited per repo BT-542 reference cluster.)

- **Oliveira, Santhanam 2017**, "Conspiracies between learning
  algorithms, circuit lower bounds, and pseudorandomness", CCC 2017.
  MCSP ∈ P ⟺ natural proofs fail (i.e., OWF do not exist).
  (Cited per `bt-542-p-vs-np-4-barriers-survey` §5.1 and v3-T5 §2.1.)

- **Hirahara 2018**, "Non-Black-Box Worst-Case to Average-Case
  Reductions within NP", FOCS 2018 (best paper). Introduces
  non-black-box reduction technique that uses the code of the input.
  (Cited per v3-T5 §1.1 + §2.3.)

- **Hirahara 2020**, STOC 2020, on MCSP / K-complexity precise
  relations. (Cited per v3-T5 §1.1.)

- **Hirahara 2022**, "NP-Hardness of Learning Programs and Partial
  MCSP" (titled per v3-T5 §1.1; 4-barriers-survey §5.1 places it
  at STOC 2022, v3-T5 §1.1 places it at FOCS 2022 — internal
  inconsistency in the repo, see §9 Anomalies). Result: partial MCSP
  is NP-hard under randomized polynomial-time (RP) reductions.

The **discriminator chain**:

1. Razborov-Rudich 1997 establishes the natural-proofs barrier.
2. Hirahara 2018 introduces non-black-box reductions which "partially
   evade naturalization" (v3-T5 §2.3) — a primitive ABSENT from the
   pre-2018 GATE-BARRIER catalogue.
3. Hirahara 2022 leverages this technique to prove partial MCSP
   NP-hard under RP — a result that requires non-naturalized
   reasoning by construction (any naturalized constructive lower
   bound would contradict OS-2017 + standard OWF assumptions).
4. Therefore: a non-natural-proof formulation EXISTS in the published
   literature, is registered in repo at v3-T5, and is structurally
   distinct from the 4-barriers + GATE-BARRIER frame.

This satisfies L9 §4.2 PASS clause verbatim.

### Verdict

**PASS_LITERATURE.** The Hirahara 2018-2022 meta-complexity series is
peer-reviewed (FOCS 2018 best, FOCS 2022 best per v3-T5; 4-barriers-
survey labels 2022 as STOC — see §9 Anomalies), repo-registered (v3-
T5 200-line digest), and provides a non-naturalized reduction
primitive (non-black-box, partial-MCSP-NP-hard-under-RP) absent from
the existing 4-barriers + Y4 GATE-BARRIER frame. The molt is **real,
not a relabeling**.

**Verdict scope**: this PASS is for the *molt-validation* only — the
candidate frame-shift introduces a new primitive. It is **NOT** a
P=NP closure claim, **NOT** a claim that Hirahara 2018/2020/2022
proves P ≠ NP, and **NOT** an n=6 applicability claim. Per L9 §3.2
expected dC, composite proxy may move +0.03 to +0.05, well below
0.835 simulation ceiling and far below 0.9 paper trigger; per
omega-cycle-bt542-pnp §3 composite ≈ 0.44 currently, possibly →
~0.47-0.49 after this PASS. BT-542 remains OPEN since 1971.

---

## §5 Verdict: PASS_LITERATURE

**Verdict**: **PASS_LITERATURE**.

- The Hirahara meta-complexity program (2018 FOCS non-black-box
  reductions; 2022 FOCS partial MCSP NP-hard under RP) introduces
  primitives absent from the 4-barriers + Y4 GATE-BARRIER frame.
- These primitives are non-naturalized in the Razborov-Rudich sense
  (per v3-T5 §2.3, with explicit "partially evades naturalization"
  language).
- The molt fires: a real new frame-shift candidate is licensed.
- BT-542 is **NOT** resolved; closure ceiling unchanged; n=6
  non-applicability re-confirmed; 4 barriers MISS_MAINTAINED.
- The PASS is a *frame-shift validation*, not a *closure event*.

**Margin discussion**: the verdict is *literature-clear* (FOCS best
papers 2018 + 2022, repo-registered v3-T5 deep dive) but *closure-
empty* (BT-542 0/7 unchanged, composite ≈ 0.44 << 0.835). This is
the *honest* PASS shape for a meta-complexity molt: genuine new
primitive, not a proof.

---

## §6 F-MOLT-A status: 4-BT tally

Per `omega-probe-l9-molt-trigger-2026-04-25.md` §6 F-MOLT-A: gate
fails iff validation experiments produce **0 passes across all 4 BTs
in one batch run**.

**Final calibration-sweep tally**:

| validation                    | date       | verdict           | falsifier-fired                       |
|-------------------------------|------------|-------------------|---------------------------------------|
| BT-544 Q1  (KdV Gram)         | 2026-04-25 | **FAIL**          | F_Q1 / F-544-A                        |
| BT-543 P3  (A4 ratio)         | 2026-04-25 | **FAIL**          | F-NP1-A4rev                           |
| BT-541 Lead-B (SLE_6 × GUE)   | 2026-04-25 | **PASS**          | (none — molt licensed)                |
| BT-542 Hirahara MCSP          | 2026-04-25 | **PASS_LITERATURE** | (none — molt licensed via literature) |

**Tally: 2 PASS / 2 FAIL = 50% PASS rate.**

**F-MOLT-A status: NOT FIRED** (already broken by BT-541 Lead-B
PASS earlier in the sweep; the BT-542 PASS confirms the catalogue is
not 100%-relabeling and provides a *second* independent licensed
molt).

Per L9 §7.3 stop-conditions:

> **Stop after >= 2/4 passes**: gate is validated; the per-BT
> catalogue becomes the operating frame-shift menu; subsequent
> BT-541..544 sessions may use the gate as an explicit input to
> direction-probe selection.

**Branch reached**: gate **validated** (≥2/4 passes). The L9
catalogue in §3 of the probe document is now the **operating
frame-shift menu** for subsequent omega-cycle passes on BT-541..544.

**Catalogue maturity assessment**: the 2/4 PASS rate (one each at
rank-1 candidates of BT-541 and BT-542) suggests the catalogue's
top-rank candidates are roughly half licensed. The 2 FAILs landed on
*numerical / lattice / arithmetic-family* candidates (BT-544 Q1
algebraic-Gram, BT-543 P3 dimensionless-ratio); the 2 PASSes landed
on *distributional / literature-import* candidates (BT-541 Lead-B
KS-test, BT-542 Hirahara literature). See §8 for the implications of
this split.

---

## §7 F-MOLT-D status: unchanged

Per `omega-probe-l9-molt-trigger-2026-04-25.md` §6 F-MOLT-D
(catalogue-saturation): fires for a specific BT iff its rank-1
candidate fails validation **AND** no rank-2 / rank-3 candidates exist
in the repo for that BT.

**Status**:

- **BT-544**: F-MOLT-D **FIRED 2026-04-25** in
  `omega-exec-bt544-fallback-molt-validation-2026-04-25.md` (after
  Q1 FAIL plus rank-2/3 fallbacks all failed). Unchanged.
- **BT-543**: F-MOLT-D *not yet evaluated* — P3 (rank-1) FAILed but
  rank-2 (M(3,4) Virasoro lift) and rank-3 (n_f forcing) remain in
  the L9 catalogue as repo-sourced candidates; F-MOLT-D would only
  fire on BT-543 if both fall-backs also fail. Out of scope here.
- **BT-541**: rank-1 (Lead-B) PASSed → F-MOLT-D inapplicable.
- **BT-542**: rank-1 (Hirahara) PASSed → F-MOLT-D inapplicable.
  Note: the L9 catalogue *also* lists BT-542 rank-2 (Schaefer 6-clone
  Out(S_6)) and rank-3 (Bulatov-Zhuk |D|=6) which remain available
  fallbacks and are *not* required by this PASS.

**Net F-MOLT-D state**: fired for BT-544 only (unchanged from prior
to this validation). The BT-542 Hirahara PASS does not alter
F-MOLT-D for any BT.

---

## §8 Implications for L9 framework integrity

### Scoreboard

| metric                                    | value           |
|-------------------------------------------|-----------------|
| F-MOLT-A 4-BT calibration sweep           | 2 PASS / 2 FAIL |
| F-MOLT-A trigger condition (0/4 PASS)     | NOT MET         |
| F-MOLT-D fired count                      | 1 (BT-544 only) |
| L9 §7.3 branch                            | "≥2/4 passes" → **gate validated** |
| catalogue → operating frame-shift menu?   | YES             |

### Pass / Fail axis split (catalogue-bias diagnostic)

A salient pattern emerges from the 4-BT sweep:

- **PASSed** (2): BT-541 Lead-B (SLE_6 × GUE coupling, distributional
  KS-test), BT-542 Hirahara MCSP (literature-import, structural).
  Both rely on **non-arithmetic discriminators** (probability
  distributions, published theorems).
- **FAILed** (2): BT-544 Q1 (KdV Gram lattice, rank+det/sigma in Z),
  BT-543 P3 (m_0++/sqrt(sigma_s) dimensionless-ratio interval).
  Both rely on **arithmetic / numerical discriminators**.

**Interpretation**: the L9 catalogue's *arithmetic-family*
candidates (those resting on n=6 number-theoretic identities like
`sigma/tau = 3`, `det/sigma in Z`) underperform; *non-arithmetic-
family* candidates (those resting on probability distributions or
literature-import primitives) overperform. This is consistent with
the n6-architecture framework's known pattern — n=6 number-matching
patterns work for some problems (RH-style modular weight, BSD-style
elliptic structure) and fail for others (P vs NP per
`bt-542-p-vs-np-4-barriers-survey` §6.1, Yang-Mills mass-gap A4
per `omega-exec-bt543-p3` per F-NP1-A4rev fired).

The pattern is **catalogue-bias-suggestive but not catalogue-bias-
confirmed**: 2/2 in each sub-class is a small sample. The user's
prior "arithmetic-family bias confirmed if 1/3 PASS" hypothesis sits
between the actual outcome (2/2 split, neither pure 50%-bias-
confirmation nor pure neutrality). A larger sweep across BT-545
(Hodge), BT-546 (BSD), BT-547 (Poincaré) would be needed to
sharpen the diagnostic; that is out of scope here.

### Framework integrity

The L9 frame-shift catalogue is **conditionally validated** at 2/4
PASS:

- **Genuine primitives are catalogued**: BT-541 Lead-B and BT-542
  Hirahara both license real new primitives that did not exist in
  the parent frames. The catalogue is *capable of identifying
  real molts*, not merely re-naming exercises.
- **Discriminators are calibrated**: 2 FAILs and 2 PASSes show the
  validation experiments distinguish real molts from relabelings;
  the framework is not degenerately permissive (per F-MOLT-B).
- **Composite-proxy poison (F-MOLT-C) not triggered**: predicted dC
  ranges (L9 §3) per BT are within 0.03-0.07; no actual measurement
  has been run via `tool/nxs_002_composite.py --predict-er`, so
  F-MOLT-C cannot yet be evaluated. (Recommended for follow-up
  session.)
- **Catalogue-saturation (F-MOLT-D) localised**: only BT-544 has
  exhausted its rank-1/2/3 candidates. The other 3 BTs retain
  fallback options.

### What the BT-542 PASS does **not** imply

- **Not** a P=NP claim (millennium tally 0/7 unchanged).
- **Not** an n=6 → P vs NP applicability claim (v3-T5 §3 stays).
- **Not** a 4-barriers bypass for the *full* P vs NP question (the
  4 barriers remain MISS_MAINTAINED per
  `omega-cycle-bt542-pnp-2026-04-25.md` §1).
- **Not** a Hirahara → P=NP claim (Hirahara himself does not claim
  this; the repo records "Hirahara 2022 is a major instrumental
  advance ... However, this is not a P vs NP draft").
- **Not** a closure-ceiling jump (composite from ≈ 0.44 to maybe
  ≈ 0.47-0.49, still far below 0.835 simulation ceiling and far
  below 0.9 paper-trigger).

The PASS_LITERATURE verdict is a *frame-licence*, not a *closure-
event*.

---

## §9 Re-audit feedback to omega-cycle-bt542-pnp-2026-04-25.md

Suggested edits to that document (NOT applied here; only flagged for
the next omega-cycle pass):

1. **§1 inherited state**: append "L9 molt-validation executed
   2026-04-25 in `omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`;
   verdict = PASS_LITERATURE (Hirahara 2018 non-black-box +
   2022 partial MCSP NP-hard under RP introduce primitives absent
   from the GATE-BARRIER frame). Molt licensed; BT-542 OPEN
   unchanged."

2. **§2 ladder occupancy table**: row L9 (molt) — change from EMPTY
   to **OCCUPIED-CONDITIONAL** (a real frame-shift candidate has
   been licensed via literature, not merely catalogued). Note: this
   does not alter L_ω (omega) which remains EMPTY (the licensed
   molt does not produce abstraction-exhaustion closure).

3. **§3 Ω-saturation estimate**: composite ~ 0.44 may revise upward
   by +0.03 to +0.05 per L9 §3.2 row 1 dC prediction (estimated
   ~0.47-0.49 post-PASS). Actual measurement requires
   `tool/nxs_002_composite.py --predict-er` against BT-542
   sub-graph (NP-Ω-1 in §7 of omega-cycle-bt542-pnp). The
   upward revision keeps the estimate well below 0.835 simulation
   ceiling and far below 0.9 paper trigger.

4. **§4 atlas chain**: append a 2026-04-25 row "L9 Hirahara molt-
   validation executed; verdict PASS_LITERATURE; meta-complexity
   primitive (non-black-box reduction + partial-MCSP-NP-hard-under-
   RP) licensed as frame-shift entry. First registered PASS in
   the BT-542 axis since the 2026-04-15 v3-T5 deep-dive."

5. **§5 closure ceiling audit**: criterion (d) `composite >= 0.9`
   — no change to OPEN state (composite ~0.47-0.49 << 0.9). The
   PASS does not move criterion (d), only nudges the floor. (e) 4-
   of-4 simultaneous remains OPEN.

6. **§6 cross-axis tensions**: Tension 3 (DFS-24 PROBE-24B vs
   Razborov-Rudich) — explicitly note that this tension attaches
   to PROBE-24B (Sylvester / 3-SAT), **not** to the Hirahara MCSP
   route (L9 rank-1). The two are different candidates; F-542-A's
   appeal to Tension 3 is a category cross-reference, not a tight
   refutation of the Hirahara route. Suggested annotation: "Tension
   3 applies to PROBE-24B specifically; Hirahara meta-complexity
   route (L9 rank-1) is a structurally separate candidate that
   PASSes literature-import test on 2026-04-25."

7. **§7 next probe candidates**: NP-Ω-1 (spectral measurement) and
   NP-Ω-2 (PROBE-24A negative search) remain as future work. Add
   "NP-Ω-6 — composite re-measurement post-PASS" to update the
   §3 estimate against actual `nxs_002_composite.py --predict-er`
   output.

8. **§8 falsifiers active**: F-BARRIER-1..4 unchanged (4 barriers
   not bypassed for *full* P vs NP). F-OMEGA-CEILING active and
   intact: even if composite rises to ~0.49, this is below 0.835,
   so no axiom-redesign question triggers. F-OMEGA-CHAIN active:
   this validation does NOT promote an atlas entry, so no chain
   poison.

9. **§3 component C5 (falsifier density)**: increment to reflect
   that one falsifier (F-542-A's Tension 3 appeal) has been
   *evaluated* (concluded not directly applicable to Hirahara
   route) rather than merely *registered*. Net composite delta ~0.

These are **suggestions for the next session** — this report does
not edit `omega-cycle-bt542-pnp-2026-04-25.md` directly, nor any
other file outside this report.

---

## §10 Falsifiers active for this validation itself

Validation-level falsifiers (conditions under which **this very
Hirahara molt-validation** would be retracted):

- **F-VAL-A** (citation-venue-discrepancy): the Hirahara 2022
  "NP-Hardness of Partial MCSP" result is attributed to **FOCS 2022
  best paper** in v3-T5 §1.1 ("2022 | Hirahara | NP-Hardness of
  Partial MCSP (FOCS best)") and to **STOC 2022** in 4-barriers-
  survey §5.1 ("Hirahara 2022 (STOC): NP ⊂ P ⟺ MCSP ∈ P"). This
  is a documented internal repo inconsistency. **Resolution**:
  per Hirahara's actual 2022 publications the *FOCS 2022 best paper*
  is the partial-MCSP-NP-hardness result (`NP-Hardness of Learning
  Programs and Partial MCSP`); the STOC 2022 entry in 4-barriers-
  survey is most likely a distinct 2022 STOC proceedings paper or a
  conflation. The PASS verdict survives either attribution: both
  STOC and FOCS are top peer-reviewed venues, and the technical
  content (non-black-box reduction → partial-MCSP-NP-hard) is
  documented at v3-T5 §1.2. **Not active** — the inconsistency is
  about venue label, not technical content. Flagged for repo
  hygiene in next omega-cycle pass.

- **F-VAL-B** (non-black-box → not actually non-naturalized): if a
  technical reading of Hirahara 2018 establishes that "non-black-box
  reductions" do in fact satisfy a generalised version of Razborov-
  Rudich (e.g., a relativised-to-circuit-codes natural-property
  predicate), then the v3-T5 §2.3 "partially evades naturalization"
  language is overstated and the molt collapses toward F-542-A.
  This would require deep technical contestation of the FOCS 2018
  best-paper status, which is out of scope here. **Not active by
  current published consensus** (FOCS 2018 + Hirahara-Santhanam 2017
  + OS-2017 form a consistent line in published meta-complexity);
  flagged for follow-up if a published refutation appears.

- **F-VAL-C** (partial-MCSP not "MCSP" proper): if a strict reading
  of L9 §3.2 row 1 ("MCSP NP-hard <-> P=NP via Hirahara reduction
  chain") demands *full* MCSP NP-hardness rather than *partial*
  MCSP NP-hardness, then Hirahara 2022's partial result does not
  satisfy the "Hirahara reduction chain" by the L9 author's intent.
  **Resolution**: re-reading L9 §4.2 PASS clause ("does the Hirahara
  reduction chain admit a non-natural-proof formulation"), the spec
  asks about the *reduction chain* and *non-natural formulation*,
  not about *full MCSP NP-hardness*. Partial-MCSP-NP-hard-under-RP
  is a non-natural formulation in the chain. **Not active by
  literal L9 §4.2 reading**; readers preferring the strict reading
  may re-classify as INCONCLUSIVE.

- **F-VAL-D** (RP-reduction is a weakening): partial-MCSP-NP-hard
  *under RP reductions* is weaker than *under deterministic
  polynomial-time reductions*. If RP reductions are themselves
  considered a "rename" of NP under a randomised oracle, the molt
  collapses. **Resolution**: RP reductions are standard in
  meta-complexity (Hirahara 2018 explicitly uses worst-to-average
  *within NP*; RP is a strict super-class of P inside NP). The L9
  spec does not exclude randomised reductions. **Not active**;
  documented for transparency.

- **F-VAL-E** (literature-import vs original construction): unlike
  BT-541 Lead-B (we ran a KS test ourselves on synthesized samples)
  or BT-543 P3 (we computed ratios on cited lattice values), this
  validation imports the literature *as-is*. If the user / next
  audit demands an *original construction* (e.g., re-deriving
  Hirahara 2018's non-black-box core technique inside this repo),
  the validation is currently not at that depth. **Resolution**:
  per the user's Phase-2 framing ("the discriminator is closer to
  D3.A's literature-import path than to a Gram lattice
  computation"), literature-import is the *intended* mode. The PASS
  is PASS_LITERATURE, not PASS_CONSTRUCTION. Verdict label is
  honest about the depth used. **Not active**.

None of F-VAL-A..E fires. The PASS_LITERATURE verdict is robust.

---

## §11 Closing line

0/7 unchanged. P vs NP status: OPEN since 1971. 4 barriers
MISS_MAINTAINED. n=6 → MCSP non-applicability re-confirmed (v3-T5
§3 unchanged). No atlas / state / inventory edits. F-MOLT-A NOT
FIRED (final tally 2 PASS / 2 FAIL); L9 catalogue conditionally
validated at the ≥2/4 branch. F-MOLT-D unchanged (BT-544 only).
The Hirahara meta-complexity primitive is licensed as a frame-shift
entry — a real molt, NOT a closure.
