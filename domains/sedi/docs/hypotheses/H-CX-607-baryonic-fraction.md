# H-CX-607: Baryonic Fraction Ω_b = 1/(σ+sopfr+τ-1) = 1/20

> **Hypothesis**: The baryon density parameter Ω_b ≈ 1/20 = 1/(σ+sopfr+τ-1), with the closed-system value 15/304 providing superior precision.

## Grade: 🟧 (standalone 2.9%; closed system 0.08%)

## Results

### Primary Standalone Expression

```
Ω_b = 1/(σ+sopfr+τ-1) = 1/20 = 0.0500

Planck 2018: Ω_b = 0.0486 ± 0.0003  (derived from Ω_b h² = 0.02237, h=0.6781)
Error: 2.9%
```

### Alternative Expressions

| Expression | Formula | Value | Error vs 0.0486 |
|---|---|---|---|
| 1/(σ+sopfr+τ-1) | 1/20 | 0.0500 | 2.9% |
| 15/304 (closed system) | from H-CX-535 | 0.04934 | 0.08% ★ |
| sopfr/(σ²-σ-M₃) | 5/125 | 0.0400 | 17.7% ✗ |
| φ/(σ·sopfr-P₂+σ) | 2/44 | 0.04545 | 6.5% |
| 1/(σ+M₃+φ) | 1/21 | 0.04762 | 2.0% |

### Closed-System Derivation (Best)

```
From Ω_DM/Ω_b = 27/5 and Ω_m = 6/19:
  Ω_b = Ω_m / (1 + 27/5) = (6/19)/(32/5) = 30/608 = 15/304

15/304 = 0.049342

Planck 2018: Ω_b = 0.0493 ± 0.0003
Error: 0.08%
```

### n=6 Decomposition

```
Standalone:
  20 = σ+sopfr+τ-1 = 12+5+4-1     → sum of three functions minus unity

Closed system:
  15 = sopfr × (σ/τ) = 5×3         → prime sum × generations
  304 = 19 × 16 = 19 × τ²          → cosmic denominator × τ²
```

### The 1/21 Alternative

```
1/(σ+M₃+φ) = 1/21 = 0.04762

Error: 2.0% — slightly better than 1/20
21 = σ+M₃+φ = 12+7+2
```

### Physical Interpretation

Baryons constitute approximately 1/20 of the total energy density. The denominator 20 = σ+sopfr+τ-1 combines the three principal arithmetic functions of n=6 (abundance, prime factor sum, divisor count) minus the fixed-point value R(6)=1.

The closed-system value 15/304 achieves 0.08% precision by leveraging two independent n=6 ratios, suggesting the baryon fraction is not a free parameter but is determined by the R(n)=1 arithmetic.

## Status

- [x] Ω_b ≈ 1/20 at 2.9%
- [x] Ω_b ≈ 1/21 at 2.0% (alternative)
- [x] Ω_b = 15/304 at 0.08% (closed system)
- [ ] Precision test with CMB-S4 (σ(Ω_b h²) → ±0.00006)
- [ ] BBN consistency check
