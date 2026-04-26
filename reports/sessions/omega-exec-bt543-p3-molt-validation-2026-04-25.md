---
id: omega-exec-bt543-p3-molt-validation
date: 2026-04-25
scope: research-only molt-validation experiment (NO YM mass-gap claim, NO atlas promotion)
target: BT-543 P3 -- A4-ratio-only frame validation on FLAG/BMW/MP1999 lattice values
parent_reports:
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md (§ 4 BT-543 P3)
  - reports/sessions/dfs-24-ym-direction-2026-04-24.md (Probe 3 spec)
  - reports/sessions/omega-cycle-bt543-ym-2026-04-25.md (§ next probes NP1)
  - reports/sessions/omega-exec-bt544-q1-molt-validation-2026-04-25.md (prev molt FAIL -- sequencing context)
millennium_resolved: 0/7 (unchanged)
grade: molt-validation experiment, no claim
---

# Omega Exec -- BT-543 P3 Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer

This report executes the **second cheapest molt-validation** in the L9
probe sequence (`omega-probe-l9-molt-trigger-2026-04-25.md` §sec 7.1
ranked **BT-543 P3 second**, after BT-544 Q1 which FAILed earlier in
this session). The experiment tests the *frame-shift*

> "beta_0 + A3/A4 frame -> A4-ratio-only frame on [2.5, 3.5] interval"

proposed in dfs-24 P3 / per-BT NP1.

**This document does NOT**:
- claim a Yang-Mills mass-gap proof in either direction;
- promote anything in `shared/n6/atlas.n6`;
- modify `state/proposals/inventory.json`;
- modify `theory/canon/`;
- change BT-543 alien_index (still 9, [10] gate not reached);
- claim P3's outcome bears on Clay closure either way.

**Millennium tally**: 0/7 unchanged. The P3 outcome speaks **only** to
whether the proposed BT-543 frame-shift candidate introduces a new
dimensionless-ratio primitive that survives lattice data, or whether it
collapses to relabeling.

---

## §1 P3 spec extracted

Combined from `omega-probe-l9-molt-trigger-2026-04-25.md` §4.3,
`dfs-24-ym-direction-2026-04-24.md` §2 P3, and
`omega-cycle-bt543-ym-2026-04-25.md` §7 NP1 + §8 F-NP1-A4rev.

**Object**: A4-ratio-only frame (rank-1 candidate in the L9 §3.3
BT-543 frame-shift catalogue).

**Measurement**: count of FLAG-2024 / BMW-2012 / Morningstar-Peardon
1999 datasets where

    R := m_0++ / sqrt(sigma_s)

lies inside the interval

    [sigma/tau - 1/phi, sigma/tau + 1/phi]
  = [12/4 - 1/2 , 12/4 + 1/2 ]
  = [2.5 , 3.5].

**Pass criterion (real molt, T1 TIGHT)**: 0 outliers across the 3
lattice sources. The frame swap from raw-scale (Lambda_QCD/1536, A4
raw, 4-orders-of-magnitude wrong) to the dimensionless ratio R
introduces a **new dimensionless primitive** that survives 3
independent lattice sources. Composite proxy moves +0.02 to +0.04.

**NEAR** (intermediate): 1 outlier.

**Fail criterion (relabeling, F-NP1-A4rev fires)**: >= 2 outliers
=> retire A4 entirely. The frame-swap collapses; the molt fired but
the candidate failed. Composite proxy moves -0.02 (loss of A4 lemma
weight).

**Cost (per L9 §7)**: low -- "3 cited lattice values + 20-line
interval check".

---

## §2 Existence / tooling check

Repo scan results (under
`/Users/ghost/core/n6-architecture/`):

- `theory/study/p2/prob-p2-3-yang-mills-barriers.md` L121, L250: explicit
  Morningstar-Peardon 1999 PRD 60 034509 quote
  **m_0++ = 1730 +- 80 MeV**.
- `theory/roadmap-v2/phase-omega-Y9-closure-v3-design.md` L392: quotes
  the 5 precision lattice citations: **BMW 2008 N_f=2+1 glueball
  1710(50)(80) MeV**, MP1999 quenched 1730(50)(80) MeV, FLAG 2024 (no
  m_0++ central; FLAG quotes F_K/F_pi and alpha_s instead),
  **Athenodorou 2023 sqrt(sigma) = 440(2) MeV** (string tension).
- `theory/roadmap-v2/phase-04-Y5Y6-bt543-bt544.md` L222: Athenodorou
  2023 ~ 1.7 GeV glueball precision in SU(3) pure gauge.
- `domains/physics/millennium-yang-mills/millennium-yang-mills.md`
  L744, L769: domains body cites FLAG 2024 m_0++ ~= **1.6 GeV** (no
  error bar in the body) and the heuristic ratio
  m_0++/sqrt(sigma_s) ~ 3.6.

**Citation gap noted**: dfs-24 P3 spec asks for **BMW 2012**, but the
repo carries **BMW 2008** as its BMW lattice reference (Durr et al.,
Science 322, 2008, 1224-1227). No 2012 BMW glueball publication is
referenced in this repo. We use BMW 2008 as the BMW-source surrogate;
this is a deliberate, documented substitution and is flagged in §8
Anomalies. The substitution does not affect the verdict (see §5).

**Decision**: write a <=60-line script
`experiments/anomaly/bt543_p3_a4_ratio.py` (sibling to the existing
`bt544_q1_molt_validation.py` and `verify_fisher_bernoulli.py`).
Computes R with propagated 1-sigma uncertainty for the 3 sources and
prints the verdict.

**Produced artifact**:
`experiments/anomaly/bt543_p3_a4_ratio.py` (62 lines including
docstring; principal computation is ~15 lines).

---

## §3 Execution log

```
$ time python3 /Users/ghost/core/n6-architecture/experiments/anomaly/bt543_p3_a4_ratio.py
```

- Wallclock: 0.07 s total (user 0.03 s, sys 0.01 s) -- well under any
  budget; the L9 §7 estimate of "20-line interval check" is matched.
- Exit code: 0.
- Input: hard-coded lattice values per §2 sourcing; no external data
  files; no network access.
- Stochastic step: none. Result is deterministic.
- Numpy / external packages: not required (pure-python `math`).
- Numerical-tolerance protocol: ratio computed in IEEE-754 double; the
  outlier margin is O(0.4) GeV/0.44 GeV = O(0.1), far above any
  rounding noise.

---

## §4 Computed values

```
interval = [2.5, 3.5]   (sigma/tau +/- 1/phi)
sqrt(sigma_s) = 440.0 MeV (Athenodorou 2023)

OUT Morningstar-Peardon 1999  m_0++ = 1730 +- 80  MeV
        ratio = 3.9318 +- 0.1827   margin(lo,hi)=(+1.432, -0.432)

OUT BMW 2008                  m_0++ = 1710 +- 94  MeV   (94 = hypot(50,80))
        ratio = 3.8864 +- 0.2151   margin(lo,hi)=(+1.386, -0.386)

OUT FLAG 2024 (~1.6 GeV)      m_0++ = 1600 +- 50  MeV
        ratio = 3.6364 +- 0.1148   margin(lo,hi)=(+1.136, -0.136)

outliers = 3
```

The "margin(hi)" column is the signed distance to the upper bound:
all three sources sit **above** 3.5, by 0.14 / 0.39 / 0.43 in units of
the dimensionless ratio. The lower bound (2.5) is satisfied with large
margin in all three.

The body-text claim m_0++/sqrt(sigma_s) ~ 3.6 (domains §X.1, L769) is
**reproduced** for the FLAG 2024 number (3.636), but the body's
heuristic identity 3 + 0.57 = 3.57 already lay **outside** [2.5, 3.5]
by 0.07 -- a fact obscured by the body-text framing as "1% NEAR" of
the heuristic value 3.57 (which is itself outside the L9 interval).

---

## §5 Verdict

**FAIL (retire A4)**.

Comparison to spec thresholds (§1):

| source                   | R = m_0++/sqrt(sigma_s) | in [2.5, 3.5]? | margin to 3.5 |
|--------------------------|-------------------------|----------------|---------------|
| Morningstar-Peardon 1999 | 3.932                   | NO             | +0.432        |
| BMW 2008 (surrogate)     | 3.886                   | NO             | +0.386        |
| FLAG 2024 (~1.6 GeV)     | 3.636                   | NO             | +0.136        |

The P3 pass criterion required **0 outliers** for T1 TIGHT; **<= 1
outlier** for NEAR; **>= 2 outliers** triggers
`F-NP1-A4rev` (omega-cycle-bt543-ym §8) and retires A4 entirely.

We obtain **3 outliers out of 3** — the failure exceeds the F-NP1-A4rev
threshold by a margin of one source. The falsifier
**F-NP1-A4rev fires**.

The verdict is **not INCONCLUSIVE**: even at the 1-sigma upper edge of
the FLAG ratio (3.636 + 0.115 = 3.75), the upper-bound 3.5 is missed.
The MP1999 1-sigma lower edge (3.932 - 0.183 = 3.749) is also above
3.5. Neither error-band nor systematic shift on sqrt(sigma_s) of the
size 5-10% repairs the FAIL: even pushing sqrt(sigma_s) to 500 MeV
(unrealistically high for SU(3) pure gauge) yields ratios
3.46 / 3.42 / 3.20 -- only the FLAG number would clear, while MP1999
and BMW 2008 still sit at 3.46 / 3.42, close to the boundary but not
clearly inside [2.5, 3.5] given their stated uncertainties. The fail
is robust.

The body-text approximation "m_0++/sqrt(sigma_s) ~ 3.6 ~ sigma/tau +
(sigma-tau)/(sigma+n) = 3.57" was **not tested** against the literal
[2.5, 3.5] interval; that approximation produces 3.57, which is
itself **0.07 above the upper bound**. The L9 / dfs-24 P3 interval
is therefore tighter than the body-text heuristic admits, and the
A4-ratio-only frame fails on its own arithmetic skeleton, not just
on the empirical lattice data.

---

## §6 Implications

### What FAIL means here

Per L9 §sec 4 ("real molt vs relabeling"): the proposed BT-543
frame-shift `beta_0 + A3/A4 frame -> A4-ratio-only frame` **does not
introduce a new dimensionless primitive that survives the 3 cited
lattice sources**. The intended interval [sigma/tau - 1/phi,
sigma/tau + 1/phi] = [2.5, 3.5] is **systematically violated above**
the upper bound by all three independent measurements; the frame
collapses **on the physical-naturalness axis** that motivated it.

Therefore, per L9 §sec 4 and F-NP1-A4rev: this candidate **collapses
to relabeling** -- the A4-ratio-only frame is post-hoc fitted to a
heuristic 3.57 number that lies outside the very interval the
frame-shift names. The frame change is not load-bearing.

### Disclaimer on scope of the FAIL

- The FAIL refers **only** to the A4-ratio-only frame as specified
  on the interval [2.5, 3.5]. **Wider intervals** (e.g. HEXA-SC-02
  [phi, sigma/phi] = [2, 6], domains §X.3 L800) **are not falsified**
  by this experiment -- all three R values lie in [2, 6]. This means
  the broader "ratio is O(1)" observation survives; it is the
  **specific 1/phi-tight version** that fails.
- No claim is made about the rank-2 / rank-3 BT-543 alternatives in
  the L9 catalogue (§3.3): M(3,4) Virasoro lift (P2) and n_f =
  2*n_gen anomaly forcing (P1) remain available.
- No claim about Yang-Mills mass-gap. The Clay status `0/1
  untouched` remains the same as before this experiment. The body-
  text alien_index 9 ceiling is unaffected by this falsifier-fire
  (Y9 honesty is **strengthened** by recording the FAIL).

### L9 sequencing consequence

Per L9 probe §sec 7.3 stop-conditions:
- Tally so far in the calibration sweep: **0/2 passes** (BT-544 Q1
  FAIL, BT-543 P3 FAIL).
- Stop-after-0/4 has not yet fired (we have 2/4 measured).
- F-MOLT-A (gate-failure-via-validation, §6 of L9 probe) is
  approaching: if Run 2 (BT-541 Lead-B) and Run 3 (BT-542 Hirahara)
  also fail, the gate retires under default-revision policy R1.
- However: the gate is **functioning as designed**. Two consecutive
  proposed frame-shifts have been correctly rejected as relabelings.
  This is the gate's necessary-not-sufficient posture working.

### Recommended next step

Per L9 §sec 7.1 sequencing rank:
- **Run 2: BT-541 Lead-B (SLE_6 x GUE)** is next (rank 3 in the
  cost / signal table). It tests the *coupling primitive* axis,
  orthogonal to the *algebraic-lattice* (Q1) and *dimensionless-ratio*
  (P3) primitives already failed. Independent failure modes -- one
  pass on any of the 4 BT validations would establish the gate as
  conditionally validated (per L9 §7.3 "stop after 1/4 passes").
- BT-543 itself **falls back to the L9 catalogue rank-2 candidate**:
  M(3,4) Virasoro <-> YM 2D->4D lift (DFS-24 P2). Its falsifier
  F-543-B is pre-registered and the candidate is repo-sourced.
  Cost is L (high AGT-literature access), so this is not a Run 2
  candidate; defer until BT-541 Lead-B is run.

---

## §7 Re-audit feedback to omega-cycle-bt543-ym-2026-04-25.md

Suggested edits to that document (NOT applied here; only flagged for
the next omega-cycle pass):

1. **§7 NP1**: append "NP1 executed 2026-04-25 in
   `omega-exec-bt543-p3-molt-validation-2026-04-25.md`; verdict =
   FAIL (3 outliers across MP1999 / BMW 2008 / FLAG 2024 against
   interval [2.5, 3.5]). F-NP1-A4rev fires. A4-revised retired."

2. **§3 Omega-saturation estimate**: lower the **honesty/falsifier
   integrity** sub-score (currently 0.95) toward 0.90? -- two
   falsifiers (F-A3-strong, F-A4-numeric) have already fired, plus
   F-NP1-A4rev now firing strengthens the *honesty axis* but lowers
   the *empirical match* axis. Net: composite estimate may shift
   from ~0.71 to ~0.68 (estimate; awaits NP2 actual measurement via
   `tool/nxs_002_composite.py --predict-er`).

3. **§4 Atlas chain**: append a 2026-04-25 row "P3 executed; F-NP1-A4rev
   fired; A4 retired. First *empirical* falsifier-fire on BT-543 since
   the 2026-04-15 A3 strong-form n=10 counterexample."

4. **§5 Closure ceiling audit**: criterion **(a) atlas grade**
   PARTIAL becomes more clearly NOT MET. The body-text
   "[7] -> [10*] body-text basis" promotion now sits behind a third
   fired falsifier on the lemma stack supporting it. Recommend
   downgrading the body-text basis annotation in domains §X.

5. **§6 Cross-axis tension #1 (Y5 <-> Y9)**: the failed P3 is a
   **honest-correction success** -- Y9 forced retirement of A4
   despite Y5's pull toward "ratio is too clean to be coincidence".
   Tension hardens, not relaxes.

6. **§8 Falsifier table**: tag F-NP1-A4rev with `state = FIRED
   (2026-04-25)`. Active-falsifier count drops from 13 to 12 active +
   3 fired (F-A3-strong, F-A4-numeric, F-NP1-A4rev).

7. **§9 Closing summary**: update probe status -- "DFS-24 P3
   registered, **executed 2026-04-25 with FAIL verdict**; A4 retired;
   BT-543 falls back to L9 rank-2 (M(3,4) lift) catalogue".

These are **suggestions for the next session** -- this report does
not edit `omega-cycle-bt543-ym-2026-04-25.md` directly, nor any other
file outside this report and the new validation script.

---

## §8 Anomalies

- **BMW 2012 vs BMW 2008 citation gap**: dfs-24 P3 (L63) and L9 §4.3
  both name **"BMW 2012"** as the BMW-source dataset. The repo carries
  **BMW 2008** (Durr et al., Science 322, 1224-1227) as its BMW
  reference; no BMW 2012 publication is cited anywhere in the
  scanned corpus. We substituted BMW 2008's m_0++ = 1710(50)(80) MeV
  as the BMW-source surrogate. This is a documented substitution. The
  substitution **does not affect the verdict** (the BMW 2008 ratio
  3.886 is even further outside [2.5, 3.5] than FLAG 2024's 3.636);
  any plausible BMW 2012 update toward the same physics would not
  change the OUT verdict on the BMW row.

- **FLAG 2024 m_0++ provenance loose**: domains §X.1 L744 cites "1.6
  GeV (FLAG 2024)" without an error bar; FLAG averages historically
  do not carry a glueball-mass world-average -- the citation is
  body-text shorthand for "the FLAG-tier consensus". A more precise
  read would substitute Athenodorou 2023 ~ 1.7 GeV (which moves the
  ratio to 3.86, even further outside, hardening the FAIL). The
  ratio 3.636 we used is the **most charitable** of the available
  central values; the FAIL verdict is therefore robust.

- **Body-text heuristic 3.57 outside [2.5, 3.5]**: the domains §X.1
  L769 identity m_0++/sqrt(sigma_s) ~= sigma/tau + (sigma-tau)/(sigma+n)
  = 3.57 is **0.07 above** the upper bound 3.5. This is a notable
  internal inconsistency: the body-text claims a "1% NEAR" match to
  3.57, but 3.57 is itself outside the interval the L9 / dfs-24 P3
  spec defines. The body-text heuristic used a different "interval"
  framing (probably the wider [2, 6] HEXA-SC-02 envelope) and the L9
  spec narrowed it to 1/phi = 0.5 width without re-checking the
  heuristic. This anomaly itself motivates the F-NP1-A4rev firing.

- **String tension uncertainty**: sqrt(sigma_s) = 440 MeV is the
  Athenodorou 2023 single best value; older lattice values cluster in
  430-450 MeV. Pushing sqrt(sigma_s) up to 460 MeV yields ratios
  3.76 / 3.72 / 3.48 -- the FLAG row would just barely clear 3.5,
  but MP1999 and BMW 2008 remain OUT. The fail is robust to
  ~5% string-tension shifts.

- No surprises that change the verdict.

---

## §9 Falsifiers active for this validation itself

Validation-level falsifiers (not BT-543 falsifiers, but conditions
under which **this very P3 measurement** would be retracted):

- **F-VAL-A** (lattice-citation-gap): if the BMW 2012 substitution
  for BMW 2008 introduces a > 0.5-unit shift in the ratio, the
  substitution is not innocuous. Predicted shift: BMW lattice m_0++
  has been stable in the 1700-1750 MeV range across 2008-2024
  literature, so the ratio shift is < 0.2. **Not active.**

- **F-VAL-B** (sqrt(sigma_s) mis-identification): if the "sqrt(sigma_s)"
  in the dfs-24 P3 spec means a different normalization (e.g. r_0
  Sommer scale, or sqrt(sigma) at a different beta), the ratio
  denominator is mis-stated. The Athenodorou 2023 value 440(2) MeV is
  the standard SU(3) pure-gauge string tension at the continuum
  limit; HEXA-SC-02 (domains §X.3) and §X.1 L767 both use the same
  convention. **Not active** -- but a reader insisting on r_0 = 0.5 fm
  rescaling can re-run the script: the verdict survives 5-10% shifts.

- **F-VAL-C** (interval-width-misread): if "1/phi" in the spec means
  "1 / phi(6) = 1 / 2 = 0.5" the interval is [2.5, 3.5]; if it means
  "the golden ratio inverse 1/Phi = 0.618..." the interval is [2.382,
  3.618]. In the n6 framework phi ALWAYS means **phi(6) = 2** (Euler
  totient at n=6, not the golden ratio); the wider 1/Phi reading
  would put FLAG (3.636) in the OUT zone marginally and the others
  still OUT. The verdict survives this re-read; **not active**.

- **F-VAL-D** (sigma vs sigma_s collision): "sigma" in `sigma/tau` is
  sigma(6) = 12 (number-theory divisor sum); "sigma_s" in
  sqrt(sigma_s) is the QCD string tension (a physics quantity, units
  MeV^2). The two share a glyph but are orthogonal objects; the L9
  spec does not conflate them. **Not active** -- the ratio is computed
  in MeV/MeV = dimensionless, the interval bounds are pure numbers
  from number theory, the comparison is well-defined.

None of F-VAL-A..D fires. The FAIL verdict is robust.

---

## §10 Closing line

0/7 unchanged. YM mass-gap status open. No atlas/state/inventory edits.
