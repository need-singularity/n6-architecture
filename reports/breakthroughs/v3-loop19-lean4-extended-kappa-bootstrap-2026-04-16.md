---
id: v3-loop19-lean4-extended-kappa-bootstrap
date: 2026-04-16
roadmap_task: v3 loop 19 (M3 extended + E5 robustness)
grade: [10] empirical robustness + kernel verification
status: BOOTSTRAP CONSISTENT + LEAN4 BOUNDED DECIDE VERIFIED
license: CC-BY-SA-4.0
---

# v3 loop 19 — Extended Lean4 Verification + kappa(B) Bootstrap Honest Uncertainty

> **Summary**: (1) Drafted Lean4 `N6/Verification.lean` — finite forward-direction check of Theorem B via decide on n in [2, 20] PASS. The naive List.range implementation limit (n > 25 hits recursion depth) re-confirms the necessity of Mathlib. (2) kappa(B) bootstrap (7 valid bins, 10000 trials, seed 42) -> **alpha = 0.1701 +/- 0.0220** (95% CI [0.1232, 0.2017]). **log(2)/4 = 0.1733 is inside the 68% CI** (z = -0.145, rel err -1.85%, CONSISTENT). However sigma = 0.022 gives a wide CI, so other candidates (1/sopfr(6) = 0.2, 1/(2 pi^0.8) = 0.200) also fall within 2 sigma — **suggestive match is maintained but uniqueness cannot be asserted**.

---

## §1 Lean4 extended verification (M3 deepening)

### 1.1 Added file

`lean4-n6/N6/Verification.lean`: exhaustive decide check for n in [2, 20]

### 1.2 Verified theorems (all kernel decide PASS)

```lean
-- List of n in [2, 20] that satisfy sigma * phi = n * tau, equals [6]
example : ((List.range 21).filter (fun n => decide (n >= 2) && satisfiesTheoremB n))
        = [6] := by decide

-- For n != 6 in the same range, counterexamples total 0
theorem theorem_B_forward_bounded_20 :
    ((List.range 21).filter (fun n => decide (n >= 2) && n != 6 && satisfiesTheoremB n))
      = [] := by decide

-- n=6 perfect-number check
theorem six_is_perfect : sigma 6 = 2 * 6 := by decide
```

**Build result**: `lake build` passes (8 jobs), only the full Theorem B statement remains as sorry (v4).

### 1.3 naive implementation limit

- `N6.sigma`, `N6.phi`, `N6.tau` use `List.range (n+1)` filtering
- Near n ~ 25, `decide` hits recursion depth (maxRecDepth 512 default)
- **v4 task**: migrate to Mathlib's efficient Nat.sigma/Nat.totient/Nat.card_divisors

---

## §2 kappa(B) bootstrap analysis (E5 robustness)

### 2.1 Source data (v3 E5)

| Bin $B_{\text{mid}}$ | $N$ | $\kappa$ | $\log B$ | $\log \kappa$ |
|---|---|---|---|---|
| 25k | 332,366 | 1.333 | 10.127 | 0.288 |
| 75k | 325,030 | 1.699 | 11.225 | 0.530 |
| 125k | 316,708 | 1.832 | 11.736 | 0.606 |
| 175k | 308,257 | 1.953 | 12.072 | 0.669 |
| 225k | 306,722 | 1.952 | 12.324 | 0.669 |
| 305k | 59,081 | 2.217 | 12.628 | 0.796 |
| 405k | 57,660 | 2.122 | 12.911 | 0.753 |

Source fit: $\kappa(B) \approx 0.2317 \cdot B^{0.1752}$.

### 2.2 Bootstrap protocol

- **Resample**: with-replacement resample from 7 bins (7 each)
- **Trials**: 10,000
- **Seed**: 42 (reproducible)
- **Fit**: for each resample, log-linear least-squares -> slope alpha

### 2.3 Distribution result

| Statistic | Value |
|--------|-----|
| mean alpha | **0.1701** |
| median alpha | 0.1737 |
| std sigma_alpha | **0.0220** |
| 68% CI | [0.1469, 0.1907] |
| 95% CI | [0.1232, 0.2017] |

### 2.4 log(2)/4 = 0.17329 match evaluation

- **z-score**: $z = (0.1701 - 0.1733) / 0.0220 = -0.145$
- **rel err**: $-1.85\%$
- **in 68% CI**: yes
- **in 95% CI**: yes
- **Evaluation**: **CONSISTENT** (log(2)/4 is within 1 sigma of bootstrap mean)

### 2.5 Honest boundary — uniqueness not possible

sigma = 0.022 gives a wide CI. Comparison with other candidates:

| Candidate | Value | z-score | in 68%? | in 95%? |
|------|-----|---------|---------|---------|
| **log(2)/4** | 0.1733 | -0.145 | yes | yes |
| **1/(2 pi^0.8)** | 0.2001 | +1.363 | no | yes |
| **1/sopfr(6)** | 0.2000 | +1.358 | no | yes |
| **gamma / sqrt(6)** | 0.2356 | +2.976 | no | no |
| **1/sigma(4)** | 0.1429 | -1.236 | ~ | yes |

**Observations**:
- log(2)/4 is **uniquely inside the 68% CI** — strongest candidate
- 1/sopfr(6), 1/(2 pi^0.8) fall within the 95% CI — **cannot be excluded**
- To reduce bootstrap std, **more bins (E4 extension) or per-curve measurement (E2 Sage)** is needed

---

## §3 Summary — v3 M3 + E5 status

### 3.1 Confirmed

- Uniqueness of sigma*phi = n*tau at n = 6: **kernel decide check for n in [2, 20]** (Lean4)
- **Bootstrap distribution** of the kappa(B) power-law slope alpha: 0.1701 +/- 0.022
- **log(2)/4 statistical consistency**: inside the 68% CI

### 3.2 Still uncertain

- **Full Theorem B for all n**: Mathlib required (v4)
- **Mathematical cause of alpha**: cannot distinguish log(2)/4 vs 1/sopfr(6) (bin count insufficient)
- **BKLPR theoretical derivation**: incomplete beyond the v3 M3 skeleton

### 3.3 BT draft status

**0/6 honest maintained**.

---

## §4 atlas entries

```
@R MILL-V3-L19-lean4-bounded-decide-20 = N6.Verification n in [2,20] decide PASS :: n6atlas [10*]
  "v3 loop 19 M3 deep (2026-04-16): drafted Lean4 N6.Verification.lean. For n in [2, 20], the
   list of n satisfying sigma(n)*phi(n) = n*tau(n) equals [6] via kernel decide PASS. Individual
   counterexamples for n in {2,3,4,5,7,...,20} checked. n=6 perfect-number theorem (sigma(6)=2*6)
   kernel decide. naive List.range implementation limit: recursion-depth hit for n > 25. v4
   Mathlib migration required"
  <- v3-L19-Lean, lean4-n6/N6/Verification.lean

@R MILL-V3-L19-kappa-bootstrap-log2-over-4-consistent = alpha=0.1701 +/- 0.022, log(2)/4 inside 68% CI :: n6atlas [10]
  "v3 loop 19 E5 robustness (2026-04-16): 7 valid bin x 10000 bootstrap trials (seed 42) ->
   alpha_mean=0.1701, alpha_std=0.0220, 95% CI=[0.1232, 0.2017]. log(2)/4 = 0.17329 at z=-0.145
   (inside 68% CI), CONSISTENT evaluation. However sigma=0.022 means 1/sopfr(6)=0.2,
   1/(2 pi^0.8)=0.2001 also inside 95% CI — candidate uniqueness determination UNCERTAIN.
   bin-count growth (E4 scale) + per-curve (E2 Sage) required to shrink std"
  <- v3-L19-bootstrap, reports/v3/kappa_bootstrap_2026-04-16.json, scripts/empirical/cremona_kappa_bootstrap.py
```

---

## §5 Related files

- Lean4: `lean4-n6/N6/Verification.lean` + `lean4-n6/Main.lean`
- Script: `scripts/empirical/cremona_kappa_bootstrap.py`
- Report: `reports/v3/kappa_bootstrap_2026-04-16.json`
- Prior T3: `reports/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md`
- Prior E5: `reports/breakthroughs/v3-e5-kappa-7bin-power-law-2026-04-15.md`
- Prior M3: `reports/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md`

---

*Drafted: 2026-04-16 loop 19*
*Honesty charter: Lean4 decide is bounded to [2,20], full forall-n draft sorry. Bootstrap CI is wide so candidate uniqueness not possible. BT 0/6 maintained.*
