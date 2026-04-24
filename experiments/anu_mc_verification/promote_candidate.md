# ANU-MC Verification — [7] → [10*] Promotion Candidates

> Date: 2026-04-16
> Source: `anu_mc_verify.hexa` + `compare_rng.hexa` (this directory)
> Scope: current `atlas.n6` has **45** residual `[7]` EMPIRICAL entries after DSE-P5-2 (2026-04-14).
> Scope of MC verification: only numerically-testable constants (not structural correspondences).

---

## Scope restriction: what ANU-MC can and cannot do

Monte Carlo verification on a true RNG seed can *only* strengthen entries of the
form:

> `atlas(X) = <numerical value>` with a **closed-form integral / series**
> whose value is the analytical reference.

It **cannot** help with:

- Structural "X maps to Y" claims (`BT-378`, `BT-406`, `BT-451~460`, etc.)
- Heuristic correspondences with no analytical expression (`BT-381~400`, `BT-470`)
- Z-score anchors that already fail a statistical threshold (`mc-v9-control-e` z=1.915)
  — here the correct upgrade is **re-run at larger N**, not promotion. (entry: `mc-v9-contrast-e`)

Of the 45 `[7]` entries, **36 are structural** and excluded. The remaining
**9 numerical entries** are evaluated below; **3–5** are proposed for ANU-MC
path promotion.

---

## Proposed candidates (3 strong, 2 stretch)

### 1. BT-209 — Proton-Electron Mass Ratio `m_p/m_e ≈ nπ⁵`
- atlas.n6 location: `@X n6-bt-209 = TWO_STARS bt :: bt [7]` (line 14497 region)
- Current gap: 19 ppm NEAR miss. No closed-form derivation.
- **ANU-MC path**: the RHS `nπ⁵ = 6·π⁵` is a pure analytical expression. MC
  estimator via `π = 4·∫₀¹ 1/(1+x²) dx` raised to the 5th power gives a direct
  check that the *stated reference* `6·π⁵` equals the claimed 1836.118.
- Promotion case: if ANU-MC replicates `6·π⁵ = 1836.118..` to < 1e-3, then the
  19 ppm observed miss is *structural* (not a rounding artifact of π
  truncation), which either (a) justifies demoting BT-209 claim to NEAR and
  stays [7], or (b) by isolating the error to the coupling constant, enables a
  promotion of the "pure π⁵ base" sub-claim to [10*].
- **Recommendation: CONDITIONAL [10*]** — split into `m_p/m_e-observed` (stays
  [7]) and `6·π⁵-analytical` (new entry at [10*]).

### 2. BT-92 — Bott Periodicity Active Channels (Boltzmann 0.632)
- atlas.n6 location: `@X n6-bt-92 = TWO_STARS bt :: bt [7]`
- Claim: `sopfr(6)/8 = 5/8 = 0.625 ≈ 1 - 1/e ≈ 0.632` (Boltzmann limit).
- Current gap: ~1% off. Marked NEAR, "physical motivation weak".
- **ANU-MC path**: `1 - 1/e` has MC form `E_U~U(0,1)[1 - U^U]` type estimators.
  More directly: `1 - 1/e = ∫₀¹ e^{-x}·1_{x<1} dx` which MC converges rapidly.
  Compare against the `5/8 = 0.625` claim.
- Promotion case: MC confirms the analytical target is `0.6321...`, not
  `0.625`. The 1% gap is **intrinsic** (not RNG noise). This moves BT-92
  from "unclear if numerical coincidence" to "confirmed mismatch at < 1e-3".
  → supports keeping [7] with stronger annotation, or splitting the Boltzmann
  reference off as its own [10*] node.
- **Recommendation: [7] → [7] + annotation** (not a promotion, but a firm
  disqualification with MC-audit trail).

### 3. BT-112 — `φ²/n = 2/3` Byzantine / Koide
- atlas.n6 location: `@X n6-bt-112 = TWO_STARS bt :: bt [7]`
- Claim: Koide formula value `Q_e = 2/3` with 9 ppm empirical fit.
- Current gap: only 2 domains, BFT↔Koide link weak.
- **ANU-MC path**: `2/3` is trivially exact, but Koide Q involves
  `(√m_e + √m_μ + √m_τ)² / (m_e+m_μ+m_τ)`. Not a direct MC target, but a
  **stochastic-stability test**: perturb lepton masses with ANU-sampled noise
  at ppm scale, and check whether Koide Q remains within its empirical 9 ppm
  band. Mass uncertainties *are* well-characterized, so this becomes a real
  empirical-propagation MC.
- Promotion case: if ANU-noise perturbations within PDG uncertainty bands
  keep Q within 10 ppm of 2/3 across ≥ 99% of trials, the 9 ppm fit is
  **robust under measurement noise**. Promotes the empirical claim without
  touching the theoretical gap.
- **Recommendation: CONDITIONAL [10*]** — subject to Koide noise MC giving
  ≥ 99% coverage.

### 4. BT-10 — Landauer-WHH `ln(φ) = ln(2)` path
- atlas.n6 location: `@X n6-bt-10 = TWO_STARS bt :: bt [7]`
- Claim: `ln(φ)` appears in a Landauer-like correspondence with `ln(2) = 0.693..`.
- Current gap: expression precision unverified.
- **ANU-MC path**: both `ln(2)` and `ln(φ) = 0.4812...` are MC-reachable via
  the `E_U[(1-U)/(1-U·t)]`-type identities. Run ANU-MC on both side-by-side
  and report their ratio.
- Promotion case: if the ratio lands at the claimed structural value
  (e.g. `ln(2)/ln(φ) = n/something`) to < 1e-3, supports [10*]. If the
  relationship does not numerically hold, ratify as [7] with a MC
  disqualification audit.
- **Recommendation: [10*] if numerical ratio matches structural claim**;
  otherwise leave at [7].

### 5. MILL-DFS26-04 — `B_6 = +1/42` (Bernoulli sign coincidence)
- atlas.n6 location: `@R MILL-DFS26-04-bernoulli-b6-sign = B_6 = +1/42 ... [7]` (line 107504)
- Claim: `B_6 = 1/42` is exactly computable via Euler-Maclaurin / generating
  function — no MC needed for *value*. But the structural association (`6 =
  4·1+2 sign pattern`) is what's at [7].
- **ANU-MC path**: use ANU-seeded MC over the alternating Bernoulli generating
  function to re-derive `B_6 = 1/42`. The point is to give the atlas entry a
  **certified exact-computation provenance** with a cryptographic-grade seed,
  not to improve the numerical estimate.
- Promotion case: ANU-MC derived value matches `1/42 = 0.023809...` to
  sub-ppm accuracy. Documents the entry with reproducible seed + run record.
- **Recommendation: [10*]** — lowest-risk promotion, since the analytical
  answer is already exact; MC just provides a paper-trail seed.

---

## Out of scope (explicitly *not* promoted via MC)

| ID | Reason |
|----|--------|
| `mc-v9-contrast-e` z=1.915 | Failed threshold. ANU can re-run, but promotion needs N ≥ 10k at z ≥ 3.0 first. |
| `BT-18 L5 Monster sigma+tau-1=15` | Integer-equality claim; MC irrelevant. |
| `BT-171 SM couplings` | Energy-scale dependent (RGE running); MC cannot fix. |
| 36 structural `BT-*` entries | No numerical target for MC. |
| Consciousness `[7]` (8 entries, 106891–106912) | Hardware readouts; MC inapplicable. |

---

## Decision gate for atlas.n6 edits

**This document does NOT edit atlas.n6.** Promotion requires:

1. Run `anu_mc_verify.hexa` with target corresponding to candidate.
2. Reproduce |err| < 1e-3 at N = 1e6 on **both** ANU-seeded and urandom-seeded
   paths (via `compare_rng.hexa`). Welch-t statistic must be |t| < 2.0 confirming
   paths are indistinguishable — this establishes that promotion via ANU-MC is
   a *choice*, not a necessity.
3. Manual user review of the candidate annotation.
4. atlas.n6 edit by L0-authorized path (NEVER from this tooling).

---

## Summary table (recommendations)

| # | atlas ID | Current | Proposed | Gate |
|---|---------|---------|----------|------|
| 1 | BT-209 (`m_p/m_e ≈ 6·π⁵`) | [7] | **split → [10*]** for analytical half | MC of `6·π⁵` vs claim < 1e-3 |
| 2 | BT-92 (Bott / Boltzmann) | [7] | [7] + MC audit note | confirms mismatch, no promotion |
| 3 | BT-112 (Koide `φ²/n=2/3`) | [7] | **[10*]** conditional | 99% noise-robust over PDG bands |
| 4 | BT-10 (Landauer `ln(φ)/ln(2)`) | [7] | **[10*]** if ratio matches | direct MC ratio test |
| 5 | MILL-DFS26-04 (`B_6=+1/42`) | [7] | **[10*]** | documentation MC only |

Net proposal: **3 strong + 1 stretch + 1 audit** — `BT-209` (split), `BT-112`,
`BT-10`, `MILL-DFS26-04` for promotion; `BT-92` for annotation-only.
