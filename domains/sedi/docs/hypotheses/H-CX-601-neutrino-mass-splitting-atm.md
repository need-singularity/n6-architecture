# H-CX-601: Atmospheric Neutrino Mass Splitting from n=6 Arithmetic

> **Hypothesis**: The atmospheric neutrino mass-squared difference |Δm²₃₂| = 2.510 × 10⁻³ eV² can be expressed using n=6 constants.

## Grade: 🟧 PLAUSIBLE (best fit ~0.4% error)

## Results

### Observed Value

```
|Δm²₃₂| = (2.510 ± 0.027) × 10⁻³ eV² (NuFIT 5.2, normal ordering)
         = 2.510 × 10⁻³ eV²
```

### Coefficient Target

```
|Δm²₃₂| in units of 10⁻³ eV² → target coefficient: 2.510
```

### n=6 Candidate Expressions for 2.510

| Expression | Value | Error |
|---|---|---|
| φ + sopfr/σ | 2 + 0.4167 = 2.4167 | 3.7% |
| sopfr/φ | 5/2 = 2.500 | 0.40% |
| (sopfr·σ-M₃)/(σφ-sopfr/τ) | (60-7)/22.75 = 2.330 | 7.2% |
| P₁·sopfr/(σ-φ) | 30/10 = 3.000 | — |
| σφ·sopfr/(σ·τ) | 120/48 = 2.500 | 0.40% |
| **(σ+sopfr)/(P₁+M₃/σ·τ)** | 17/6.something | — |
| **M₃·τ/(σ-sopfr+τ/φ)** | 28/9 = 3.111 | — |
| **sopfr/φ** | **2.500** | **0.40%** |
| **(P₂+sopfr/τ)/(σ-φ)** | 29.25/10 | — |
| **(σφ+sopfr)/(σ-φ+M₃/τ/sopfr)** | complex | — |

### Best Fit

```
|Δm²₃₂| ≈ (sopfr/φ) × 10⁻³ eV² = (5/2) × 10⁻³ = 2.500 × 10⁻³ eV²

Observed: 2.510 × 10⁻³ eV²
Error: 0.40%
```

### The sopfr/φ = 5/2 Expression

```
sopfr(6)/φ(6) = 5/2

This is the ratio of the sum-of-prime-factors to the Euler totient.
For n=6: the "additive prime weight" per "coprime unit."
```

### Refined Fit

```
Try: sopfr/φ + 1/(σ²·sopfr) = 2.500 + 1/720 = 2.5014 — 0.34%
Try: sopfr/φ + M₃/(M₃·P₃) = 2.500 + 1/496 = 2.5020 — 0.32%
Try: (sopfr·σφ+M₃/τ)/(σ·τ) = (120+1.75)/48 = 121.75/48 = 2.5365 — 1.1%

Best simple: sopfr/φ = 5/2 = 2.500 (0.40%)
```

### Comparison with Solar Splitting

```
|Δm²₃₂|/Δm²₂₁ = 2.510/0.0742 = 33.83  (see H-CX-602)

In n=6: (sopfr/φ)/(M₃+sopfr/σ) = 2.500/7.4167 = 0.3370
Reciprocal: 2.968 — not quite 33.8
This ratio is addressed in H-CX-602.
```

### Interpretation

The atmospheric mass splitting coefficient sopfr/φ = 5/2 is one of the simplest possible n=6 ratios. Combined with H-CX-600 (solar splitting = 89/12 × 10⁻⁵), the two neutrino mass scales are both simple n=6 fractions modulo powers of 10. The ratio of scales (~34) tests whether the power-of-10 gap itself has n=6 structure.

## Verification

```
sopfr = 5, φ = 2                               ✓
sopfr/φ = 5/2 = 2.500                          ✓
2.500 × 10⁻³ eV²                               ✓
|Δm²₃₂| observed = 2.510 × 10⁻³ eV²          ✓
|2.500 - 2.510|/2.510 = 0.40%                  ✓
```

## Status

- [x] |Δm²₃₂| ≈ sopfr/φ × 10⁻³ = 2.500 × 10⁻³ eV² (0.40%)
- [x] Simplest n=6 ratio for atmospheric scale
- [ ] Origin of 10⁻³ suppression from n=6 pending
