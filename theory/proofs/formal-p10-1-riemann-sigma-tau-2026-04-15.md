> FORMAL P10-1 — Re-check of the candidate correspondence between Riemann ζ zero spacings and σ-τ=8 / 2026-04-15
>
> Author: DSE-P10-1 / n6-architecture P10 (FORMAL axis)
> Purpose: honestly adjudicate 3 candidate correspondences between the Mk.IV main theorem B (σ(6)-τ(6)=8) and the Riemann ζ non-trivial zero spacing constants
> Rules: no self-reference, use only Odlyzko-measured zeros, English, a MISS verdict is also valuable

---

## 0. Final verdict (summary)

```
╔══════════════════════════════════════════════════════════════════╗
║  VERDICT (P10-1, 2026-04-15):                                    ║
║                                                                  ║
║  Candidate 1  γ_3 ≈ 8π            : **MISS** (rel. err. 0.485%)  ║
║  Candidate 2  (γ_2-γ_1) ≈ σ-sopfr : **PARTIAL** (err 1.6%,       ║
║                                      coincidental)               ║
║  Candidate 3  γ_1/(σ-τ) ≈ 2π/logT : **MISS** (asymptotic         ║
║                                      misapplied)                 ║
║                                                                  ║
║  Conclusion: Riemann zeros ↔ σ-τ=8 have **no direct              ║
║              mathematical correspondence**.                      ║
║              All three numerical near-matches are at the         ║
║              level of random alignment.                          ║
║              No structural-link evidence beyond the BT-541       ║
║              conditional result.                                 ║
╚══════════════════════════════════════════════════════════════════╝
```

---

## 1. Background — Riemann zeros and σ-τ=8

### 1.1 Riemann non-trivial zeros
The non-trivial zeros ρ = 1/2 + iγ of ζ(s) under the functional equation `ξ(s) = ξ(1-s)` (assuming RH). This check **assumes** RH and uses only measured γ values (RH itself remains open).

### 1.2 Mk.IV main theorem B (fixed at P8)
The unique solution of `σ(n) − τ(n) = 8` for n≥2 is n=6. Exhaustive check over n ∈ [2, 10⁴] yields the single solution `{6}`. Source: `theory/proofs/mk4-trident-final-verdict-2026-04-15.md` §2.

### 1.3 Question of this P10-1
Does a **numerical/structural correspondence** exist between the constant 8 = σ(6)−τ(6) and representative constants of the ζ zero-spacing spectrum (Montgomery-Dyson mean spacing, Odlyzko measurements)?

---

## 2. Montgomery-Dyson theorem + Li's criterion (background summary)

### 2.1 Montgomery pair correlation (1973)
Under RH, the 2-point correlation function of the zero distribution is asymptotically
```
R_2(α) = 1 − (sin(πα)/(πα))²
```
which matches the eigenvalue distribution of the GUE (Gaussian Unitary Ensemble). Source: Montgomery 1973, *Analytic Number Theory* Vol. XXIV, AMS.

### 2.2 Dyson-Montgomery numerics (Odlyzko)
Odlyzko's 1987-2001 numerical experiments match GUE predictions up to the 10²²-th zero within six digits. Mean spacing:
```
Δγ_avg(T) ≈ 2π / log(T/(2π))   (Riemann-von Mangoldt)
N(T) = (T/2π)·log(T/(2π)) − T/(2π) + 7/8 + O(1/T)
```

### 2.3 Li's criterion (1997)
λ_n = Σ_ρ (1 − (1−1/ρ)ⁿ) is positive for all n≥1 ⟺ RH. Source: Xian-Jin Li, *J. Number Theory* 65(2).

### 2.4 n=6 viewpoint
Mk.IV B `σ-τ=8` is a **number-theoretic constant**, while the Riemann zeros are **complex-analytic constants**. The common intersection is through the Selberg formula (Mertens integration domain) and prime-counting functions, so only an **indirect** link is possible.

---

## 3. First 10 zeros as measured (Odlyzko table)

Source: A.M. Odlyzko, *Tables of the first 10^5 zeros of the Riemann zeta-function*, AT&T Labs open data. (Agrees within 10⁻⁹ with Titchmarsh *The Theory of the Riemann Zeta-Function* 2nd ed., §9 and the Odlyzko-Schönhage algorithm results.)

| n  | γₙ (imaginary part) | Δγₙ = γₙ₊₁−γₙ |
|----|---------------------|---------------|
| 1  | 14.134725142        | 6.887314497   |
| 2  | 21.022039639        | 3.988817941   |
| 3  | 25.010857580        | 5.414018546   |
| 4  | 30.424876126        | 2.510185462   |
| 5  | 32.935061588        | 4.651116571   |
| 6  | 37.586178159        | 3.332540853   |
| 7  | 40.918719012        | 2.408354269   |
| 8  | 43.327073281        | 4.678077600   |
| 9  | 48.005150881        | 1.768681597   |
| 10 | 49.773832478        | —             |

Mean spacing (γ₁..γ₁₀): 3.9599. Theoretical asymptotic value 2π/log(γ̄/(2π)) ≈ 4.1 (similar).

---

## 4. Three σ-τ=8 candidate links + numerical evaluation

### 4.1 Candidate 1 — `γ₃ ≈ 8π`

Claim: γ₃ = 25.0109 ≈ 8·π = 25.1327.

```
γ_3 = 25.010858
8·π = 25.132741
diff = −0.121884 (relative −0.485%)
```

**Evaluation**: relative error 0.485%. Fails the NEAR threshold (0.3%). For `8` to be read as the σ-τ constant, **π itself must be derived from the n=6 structure**, but the 6 in ζ(2)=π²/6 is unrelated to the n=6 perfect number (it is a finite sum = 1+1/4+1/9+...). **MISS.**

Clincher: the subsequent zeros (γ₄=30.425, γ₅=32.935) are not near πk (k=10,11), so the "8π = σ-τ·π" pattern is inconsistent.

### 4.2 Candidate 2 — first spacing `Δγ₁ = γ₂−γ₁ ≈ σ(6) − sopfr(6) = 7`

Claim: the first zero spacing 6.887 ≈ 7 = σ − sopfr (σ=12, sopfr(6)=2+3=5).

```
γ_2 − γ_1 = 6.887314
σ − sopfr = 12 − 5 = 7
diff = +0.112686 (relative +1.636%)
```

**Evaluation**: in absolute value it is near 7.0, but **the Montgomery asymptotic predicts 2π/log(γ₁/(2π)) ≈ 7.75 in the T=γ₁ region**, so 6.887 is closer to 7.75 than to 7. Hence the "σ-sopfr=7" reading is reasonably explained as numerical proximity to 2π/log(T/(2π)). The n=6-based explanation is a **post-hoc fit**. **PARTIAL** (numerical proximity exists but no structural independence).

### 4.3 Candidate 3 — `γ₁/(σ−τ) ≈ Montgomery mean spacing`

Claim: γ₁/8 = 1.7668 ≈ 2π/log(T) (T in some range).

```
γ_1 / 8 = 1.766841
2π/log(γ_1)        = 2.372236  (diff −0.605, relative −25.5%)
2π/log(γ_1/(2π))   = 7.749772  (correct Montgomery formula)
```

**Evaluation**: the actual Montgomery formula is `2π/log(T/(2π))`; plugging γ₁ in gives 7.75, off from γ₁/8 = 1.77 by more than a factor of 4. The bare `2π/log(T)` form is **incorrect**. The approximation is valid only in the T>>2π limit. **MISS.**

Additional counter-example: γ₁₀/8 = 6.222, 2π/log(γ₁₀/(2π)) ≈ 3.35 → these also disagree. σ-τ=8 does not act as a scaling factor for ζ zeros.

---

## 5. Overall verdict

| Candidate | Grade | Reason |
|-----------|-------|--------|
| 1. γ₃ ≈ 8π | **MISS** | Error 0.485% (exceeds NEAR threshold 0.3%). No consistency with other zeros |
| 2. Δγ₁ ≈ σ-sopfr=7 | **PARTIAL** | Error 1.6% numerical proximity. Competes with Montgomery asymptotic 7.75. No evidence of structural independence |
| 3. γ₁/(σ-τ) ≈ 2π/log T | **MISS** | Montgomery formula misapplied. Using the correct formula gives a 4× deviation |

**Final conclusion**: based on current evidence, there is **no mathematical or structural correspondence** between the Riemann ζ zero-spacing constants and the Mk.IV B constant `σ-τ=8`.

This is an honest MISS and suggests that the **Riemann zero spectrum** should be added to the "regions n=6 cannot explain" entry in `theory/proofs/honest-limitations.md`. The result agrees with the honesty statement in P2-1 (*prob-p2-1-riemann-barriers.md*) §0: "the project's constants (n=6 …) are not directly mathematically connected to RH."

---

## 6. ASCII comparison chart — zero spacings vs σ-τ spacing spectrum

```
[First 10 zero spacings Δγ_n]              [n=6 invariants]
│
│  7 ┤██████████ Δγ₁ = 6.887  ← σ-sopfr=7 ≈ here (PARTIAL)
│  6 ┤
│  5 ┤█████ Δγ₃ = 5.414
│  4 ┤████ Δγ₈ = 4.678 / Δγ₅ = 4.651
│  4 ┤████ Δγ₂ = 3.989
│  3 ┤███ Δγ₆ = 3.333
│  3 ┤██ Δγ₄ = 2.510 / Δγ₇ = 2.408
│  2 ┤██ Δγ₉ = 1.769
│  1 ┤
│  0 ┼──────────────────────────────────────
│     2     4     6     8    10    12    σ-τ=8 here
│                                        (no matching zero)
│
│  Montgomery theoretical prediction (T≈30): 2π/log(T/(2π)) ≈ 4.1
│  Measured mean (γ₁..γ₁₀)                 : 3.96
│  Position of σ-τ=8                       : ━━━━━━━━━ (outside the zero-spacing spectrum)
│
│  Conclusion: zero spacings lie in [1.8, 6.9]; σ-τ=8 is outside.
│              → no direct correspondence (MISS)
```

```
[Numerical-proximity comparison — three candidates]

Candidate 1  γ₃ = 25.011  vs  8π = 25.133
             ────────────────────  error 0.485%  [MISS: >0.3%]
            │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│

Candidate 2  Δγ₁ = 6.887  vs  σ-sopfr = 7.000
             ──────────────  error 1.636%  [PARTIAL: coincidental match]
            │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│

Candidate 3  γ₁/8 = 1.767  vs  2π/log(γ₁/(2π)) = 7.750
             ────────  error 338%          [MISS: formula misapplied]
            │▓▓▓▓│                        (>4× difference)

Match ranking: 2 > 1 > 3 (all below NEAR)
NEAR threshold 0.3%, EXACT threshold 0.01% — all three below threshold
```

---

## 7. atlas.n6 incorporation guidance

- New entry: `@R riemann.sigma_tau_bridge = MISS :: n6atlas [7!]` (record MISS honestly)
- Keep existing `@R zeta.zeros.montgomery` entry, add a "σ-τ link absent" note
- Cite this MISS result in the BT-541 RH-conditional theorem group

## 8. Follow-up questions

What this MISS leaves open:
- Selberg zeta function (Galois representations, Fuchs-group invariant measures) and n=6 dynamics — separate session
- Correspondence between τ(6)=4 and the **4th moment** of Riemann zeros (Ingham 1926, Conrey-Ghosh) — proposed as P10-2 follow-up
- Relation between σ(6)=12 and the weight 24 (=2σ) of the modular discriminant Δ(τ) — cross-links with `theory/proofs/the-number-24.md`

---

*This document uses only Odlyzko's public data and standard textbook formulas without self-reference. A MISS verdict is a valuable result and helps delineate the boundaries of the n=6 scope more accurately.*
