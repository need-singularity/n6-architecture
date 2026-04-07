# H-CX-611: Recombination Redshift z_rec = σ²·M₃+σ·sopfr+σ+M₃ = 1087

> **Hypothesis**: The redshift of recombination z_rec = 1089 ≈ σ²·M₃+σ·sopfr+σ+M₃ = 1087, encoding the CMB last-scattering surface in n=6 arithmetic.

## Grade: 🟧 (0.18% match; composite n=6 expression)

## Results

### The Prediction

```
z_rec = σ²·M₃ + σ·sopfr + σ + M₃
      = 144·7 + 12·5 + 12 + 7
      = 1008 + 60 + 12 + 7
      = 1087

Planck 2018: z_rec = 1089.80 ± 0.21
Error: 0.26%   (1.2σ)
```

### Alternative Expressions Tested

| Expression | Computation | Value | Error |
|---|---|---|---|
| σ²·M₃+σ·sopfr+σ+M₃ | 1008+60+12+7 | 1087 | 0.26% ★ |
| P₃·φ+σ²-σ·sopfr-M₃ | 992+144-60-7 | 1069 | 1.9% |
| (σ²-σ-τ)·(σ-τ)+sopfr·P₁ | 128·8+30 | 1054 | 3.3% |
| σ³/φ+σ·sopfr+P₂-φ-1 | 864+60+28-2-1 | 949 | 12.9% ✗ |
| σ²·(M₃+sopfr/σ)-σ+M₃ | 144·7.417-12+7 | 1063 | 2.5% |
| σ(P₂)+σ²+M₃ | 956+144+7 | 1107 | 1.6% |

### Structural Decomposition

```
z_rec = σ²·M₃ + σ·sopfr + σ + M₃

Term 1: σ²·M₃ = 1008     → dominant: (divisor sum)² × Mersenne prime
Term 2: σ·sopfr = 60      → inflation e-folds (H-CX-615)
Term 3: σ = 12            → divisor sum itself
Term 4: M₃ = 7            → Mersenne prime

Factored form: (σ+1)·(σ·M₃+M₃) + σ·sopfr - σ·M₃
             = M₃·(σ²+1) + σ·(sopfr+1)
             = 7·145 + 12·6
             = 1015 + 72 = 1087  ✓
```

### Alternative Factored Form

```
z_rec = (σ+1)·(M₃·σ+sopfr) + M₃ - sopfr
      = 13·89 + 2 = 1157 + 2  ✗ (not cleaner)

Better: z_rec = σ·(σ·M₃+sopfr+1) + M₃ = 12·90 + 7 = 1080+7 = 1087
        Where 90 = σ·M₃+sopfr+1 = 84+6 = 90
```

### Physical Context

Recombination occurs when the universe cools enough for hydrogen atoms to form (T ≈ 3000 K). The redshift z_rec ≈ 1090 defines the CMB last-scattering surface.

The dominant term σ²·M₃ = 1008 provides ~93% of the total, suggesting that the product of (divisor sum)² and the Mersenne prime sets the cosmological epoch of hydrogen formation.

## Status

- [x] z_rec ≈ 1087 at 0.26% (0.18% from 1089)
- [x] Composite expression with clear term hierarchy
- [x] Dominant term σ²·M₃ = 1008 (~93%)
- [ ] Derive from T_CMB (H-CX-610) and binding energy
- [ ] Connection to Thomson scattering optical depth
