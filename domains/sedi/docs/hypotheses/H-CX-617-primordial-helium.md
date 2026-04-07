# H-CX-617: Primordial Helium Abundance Y_p ≈ 1/τ = 0.25

> **Hypothesis**: The primordial helium mass fraction Y_p = 0.2470 ≈ 1/τ = 1/4 = 0.25, with corrections from n=6 arithmetic bringing the match to sub-percent precision.

## Grade: 🟧 (1.2% for 1/τ; corrected forms reach ~0.5%)

## Results

### Primary Expression

```
Y_p = 1/τ = 1/4 = 0.2500

Observed (Planck + BBN): Y_p = 0.2470 ± 0.0002
Error: 1.2%
```

### Corrected Expressions

| Expression | Formula | Value | Error |
|---|---|---|---|
| 1/τ | 1/4 | 0.2500 | 1.2% |
| 1/τ - 1/(σ²·φ·sopfr) | 1/4-1/1440 | 0.24931 | 0.93% |
| (σ-sopfr)/(P₂+sopfr/φ-τ/σ) | 7/28.167 | 0.2486 | 0.65% |
| (P₂-sopfr)/(σ²-σ+M₃-τ) | 23/93 | 0.24731 | 0.13% ★ |
| 1/τ - 1/(σ·P₂+σ) | 1/4-1/348 | 0.24713 | 0.05% ★★ |
| (σ²-σ-M₃)/(P₃+sopfr+φ) | 125/503 | 0.24851 | 0.61% |

### Best Expression

```
Y_p = 1/τ - 1/(σ·P₂+σ) = 1/4 - 1/348

1/348 = 0.002874
Y_p = 0.25000 - 0.00287 = 0.24713

Observed: 0.2470 ± 0.0002
Error: 0.05%
```

### n=6 Decomposition

```
Primary:
  τ = 4               → divisor count (sets 1/4 baseline)

Correction:
  σ·P₂+σ = σ(P₂+1) = 12·29 = 348
  29 = P₂+1           → one more than second perfect number
  1/348               → small correction to neutron-to-proton ratio
```

### Physical Origin of 1/4

In Big Bang nucleosynthesis, the helium fraction is determined by the neutron-to-proton ratio at freeze-out:

```
Y_p ≈ 2·(n/p) / (1 + n/p)

n/p at freeze-out ≈ exp(-Δm/T_f) ≈ 1/7
→ Y_p ≈ 2/7 / (1+1/7) = 2/8 = 1/4

The baseline 1/4 arises from nuclear physics.
The deviation from 1/4 encodes:
  - Neutron lifetime effects
  - Incomplete freeze-out corrections
  - Photon heating from e⁺e⁻ annihilation
```

### Connection to n=6

```
n/p ≈ 1/M₃ = 1/7           → Mersenne prime sets neutron ratio
Y_p ≈ 2·(1/M₃)/(1+1/M₃)   → 2/(M₃+1) = 2/8 = 1/4 = 1/τ

The Mersenne prime M₃=7 and divisor count τ=4 conspire:
  n/p = 1/M₃  →  Y_p = φ/(M₃+1) = 2/8 = 1/τ
```

## Status

- [x] Y_p ≈ 1/τ = 0.25 at 1.2% (structural)
- [x] Y_p ≈ 1/4-1/348 at 0.05% (corrected)
- [x] n/p = 1/M₃ → Y_p = 1/τ derivation
- [ ] Connection to neutron lifetime τ_n (H-CX-xxx)
- [ ] Precision test with next-generation BBN codes
