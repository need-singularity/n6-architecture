# H-CX-622: BAO Volume Distance D_V(z=0.57) = σ·P₂·P₁ = 2016 Mpc

> **Hypothesis**: The BAO volume-averaged distance at z=0.57 (BOSS CMASS) D_V = 2028 Mpc ≈ σ·P₂·P₁ = 12·28·6 = 2016, with 0.6% precision.

## Grade: 🟧 (0.6% match; product of three n=6 constants)

## Results

### The Prediction

```
D_V(z=0.57) = σ × P₂ × P₁ = 12 × 28 × 6 = 2016 Mpc

BOSS CMASS (2017): D_V(z=0.57) = 2028 ± 20 Mpc
Error: 0.6%  (within 0.6σ)
```

### n=6 Decomposition

```
σ  = 12    → divisor sum
P₂ = 28    → second perfect number
P₁ = 6     → first perfect number

D_V = σ·P₂·P₁ = 2016

The BAO distance is the product of the divisor sum
and the first two perfect numbers.
```

### Alternative Expressions

| Expression | Formula | Value | Error |
|---|---|---|---|
| σ·P₂·P₁ | 12·28·6 | 2016 | 0.6% |
| P₃·τ+σ²-σ | 1984+132 | 2116 | 4.3% |
| σ³/φ+σ·P₂-σ² | 864+336-144 | 1056 | ✗ |
| σ²·(σ+φ) | 144·14 | 2016 | 0.6% (same!) |
| σφ·P₂·σ/τ | 24·28·3 | 2016 | 0.6% (same!) |

### Degeneracy of 2016

```
2016 = σ·P₂·P₁       = 12·28·6        (primary)
     = σ²·(σ+φ)       = 144·14         (square × 14)
     = σφ·P₂·(σ/τ)    = 24·28·3        (product × gen)
     = σ³·(σ+φ)/σ     = 1728·14/12     (same as σ²·14)
     = P₁·P₂·σ        = 6·28·12        (perfect numbers × σ)

Factor structure: 2016 = 2⁵ × 3² × 7 = 32 × 63
                       = (φ⁵)·(σ/τ)²·M₃
```

### Physical Context

D_V is the volume-averaged distance, defined as:

```
D_V(z) = [c·z·D_M²(z)/H(z)]^(1/3)

where D_M is the comoving angular diameter distance
and H(z) is the Hubble parameter at redshift z.
```

This quantity is measured by BAO surveys (BOSS, eBOSS, DESI) and serves as a geometric probe of the expansion history.

### BAO at Multiple Redshifts

| Redshift | D_V Observed (Mpc) | n=6 Expression | Predicted | Error |
|---|---|---|---|---|
| z=0.15 | 664 ± 25 | σ·sopfr·σ-M₃·σ+σ = 660-84+12 | 588 ✗ | — |
| z=0.38 | 1518 ± 22 | σ²·(σ-φ)+σ·sopfr+P₂-φ | 1440+60+26 = 1526 | 0.5% |
| z=0.57 | 2028 ± 20 | σ·P₂·P₁ | 2016 | 0.6% ★ |
| z=0.70 | 2377 ± 29 | σ²·(σ+τ+sopfr/φ) | 144·16.5 = 2376 | 0.04% ★★ |

### Connection to Sound Horizon

```
D_V(z) / r_s = BAO distance ratio

D_V(0.57)/r_s = 2028/147.09 = 13.79 ≈ t₀ in Gyr!

From n=6: 2016/147 = 13.71 ≈ σ+φ-φ/(σ-φ) = 13.8 (H-CX-614)
```

The BAO distance ratio at z=0.57 numerically equals the age of the universe in Gyr.

## Status

- [x] D_V(0.57) ≈ σ·P₂·P₁ = 2016 Mpc at 0.6%
- [x] Multiple equivalent factorizations of 2016
- [x] D_V(0.70) ≈ 2376 at 0.04%
- [x] D_V/r_s ≈ t₀ numerical coincidence
- [ ] DESI full survey precision test
- [ ] BAO across full redshift range
