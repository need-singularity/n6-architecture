# H-CX-618: Primordial Deuterium Abundance D/H ≈ 2.527×10⁻⁵

> **Hypothesis**: The primordial deuterium-to-hydrogen ratio D/H = 2.527×10⁻⁵ is explored through n=6 arithmetic. Coefficient ≈ φ+sopfr/σ = 2.417 (4.4% off); exponent -5 = -sopfr. Open challenge for precise coefficient.

## Grade: 🟧 (exponent exact; coefficient approximate)

## Results

### The Observation

```
D/H = (2.527 ± 0.030) × 10⁻⁵  (Cooke et al. 2018, primordial)
```

### Exponent Analysis

```
-5 = -sopfr(6)

The exponent is exactly the negative of the prime factor sum.
This parallels H-CX-603 where σ_v has exponent -(σφ+φ) = -26.
```

### Coefficient Expressions

| Expression | Formula | Value | Error vs 2.527 |
|---|---|---|---|
| φ+sopfr/σ | 2+5/12 | 2.417 | 4.4% |
| sopfr/φ | 5/2 | 2.500 | 1.1% |
| σφ·sopfr/(σ·τ) | 120/48 | 2.500 | 1.1% |
| (P₂-sopfr·sopfr)/φ | 3/2 | 1.500 | ✗ |
| (σ·sopfr+P₁)/(σφ+φ) | 66/26 | 2.538 | 0.44% ★ |
| (σ-τ+sopfr·φ/σ)/(τ-φ/sopfr) | ... | ... | ... |
| M₃·τ/(σ-φ-sopfr/φ) | 28/11.5 | ... | ... |

### Best Expression

```
D/H = (σ·sopfr+P₁)/(σφ+φ) × 10⁻sopfr
    = 66/26 × 10⁻⁵
    = 2.538 × 10⁻⁵

Error: 0.44%

Numerator:   σ·sopfr+P₁ = 60+6 = 66
Denominator: σφ+φ = 24+2 = 26 = d_bosonic
```

### Alternative: sopfr/φ Form

```
D/H = (sopfr/φ) × 10⁻sopfr = 2.500 × 10⁻⁵

Error: 1.1%

This is the cleanest form: prime sum over totient, times
10 to the minus prime sum.
```

### n=6 Structure

```
D/H ≈ (sopfr/φ) × 10⁻sopfr

The abundance of deuterium (the simplest composite nucleus)
is controlled entirely by sopfr(6) = 5:
  - Scale: 10⁻⁵ (five orders of magnitude below hydrogen)
  - Coefficient: 5/2 (prime sum over totient)
```

### Physical Context

Primordial deuterium is the most sensitive BBN barometer because its abundance depends strongly on the baryon-to-photon ratio η. The observed D/H constrains:

```
η = n_b/n_γ = (6.09 ± 0.06) × 10⁻¹⁰  (Planck 2018)
Ω_b h² = 0.02237 ± 0.00015
```

The deuterium bottleneck in BBN makes D/H a precision probe of baryonic content.

### Connection to Other BBN Hypotheses

- Y_p ≈ 1/τ = 0.25 (H-CX-617)
- Ω_b ≈ 1/20 (H-CX-607)
- n_γ/n_b ≈ 10^(τ+P₁)/P₁ (H-CX-619)

## Status

- [x] Exponent -5 = -sopfr exact
- [x] Coefficient ≈ sopfr/φ = 2.5 at 1.1%
- [x] Coefficient ≈ 66/26 = 2.538 at 0.44%
- [ ] Derive from η and BBN network
- [ ] Connection to lithium problem
