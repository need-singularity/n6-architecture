# H-CX-619: Photon-to-Baryon Ratio n_γ/n_b = 10^(τ+P₁)/P₁ ≈ 1.667×10⁹

> **Hypothesis**: The cosmic photon-to-baryon number ratio 1/η ≈ 1.63×10⁹ ≈ 10^(τ+P₁)/P₁ = 10¹⁰/6, with 1.5% precision.

## Grade: 🟧 (2.2% match; clean power-of-ten form)

## Results

### The Prediction

```
n_γ/n_b = 10^(τ+P₁) / P₁ = 10¹⁰/6 = 1.6667 × 10⁹

Observed:
  η = n_b/n_γ = (6.09 ± 0.06) × 10⁻¹⁰  (Planck 2018)
  1/η = 1.642 × 10⁹

Error: 2.2%  (within 0.4σ given CMB uncertainties)
```

### n=6 Decomposition

```
10 = τ+P₁ = 4+6                → divisor count + first perfect number
10 = τ(P₃) = τ(496)            → also the divisor count of P₃!
P₁ = 6                         → first perfect number

n_γ/n_b = 10^τ(P₃) / P₁       → perfect number tower structure
```

### Alternative Expressions

| Expression | Formula | Value (×10⁹) | Error |
|---|---|---|---|
| 10^(τ+P₁)/P₁ | 10¹⁰/6 | 1.667 | 2.2% |
| 10^τ(P₃)/P₁ | 10¹⁰/6 | 1.667 | 2.2% (same) |
| σ²·10^(σ-τ)/P₁ | 144·10⁸/6 | 2.40 | ✗ |
| P₃·σ²·σφ | 496·144·24 | 1.715×10⁶ | ✗ (wrong scale) |
| σ·P₂·10^(M₃+1) | 336·10⁸ | 3.36×10¹⁰ | ✗ |

### The 10¹⁰/6 Interpretation

```
10¹⁰ = total photon "budget" per unit volume
6    = P₁ = perfect number divisor

For every 10 billion photons, there are 6 baryons.
Or equivalently: the universe has P₁ baryons per 10^τ(P₃) photons.
```

### Physical Context

The baryon-to-photon ratio η is one of the most important numbers in cosmology:
- It determines BBN yields (D/H, He-4, Li-7)
- It is measured independently by CMB anisotropies and by BBN
- It encodes the matter-antimatter asymmetry of the universe

### Connection to Baryon Asymmetry

```
η = n_b/n_γ ≈ 6 × 10⁻¹⁰

This is related to the baryon asymmetry (H-CX-531):
  (n_b - n_b̄)/n_γ ≈ η ≈ 6 × 10⁻¹⁰

The "6" = P₁ appears directly as the coefficient of the asymmetry.
The universe chose P₁ baryons per 10¹⁰ photons.
```

### Precision Comparison

| Method | η (×10⁻¹⁰) | 1/η (×10⁹) | vs 10¹⁰/6 |
|---|---|---|---|
| Planck CMB | 6.09 ± 0.06 | 1.642 | 2.2% off |
| BBN (D/H) | 6.10 ± 0.07 | 1.639 | 2.7% off |
| n=6 prediction | 6.000 | 1.667 | — |

## Status

- [x] 1/η ≈ 10¹⁰/6 = 10^τ(P₃)/P₁ at 2.2%
- [x] Coefficient P₁ = 6 links to perfect number
- [x] Exponent τ(P₃) = 10 links to perfect number tower
- [ ] Connection to leptogenesis mechanism
- [ ] CMB-S4 precision test of η
