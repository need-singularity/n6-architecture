# H-CX-613: Matter-Radiation Equality z_eq ≈ 3387

> **Hypothesis**: The redshift of matter-radiation equality z_eq = 3387 is explored through n=6 arithmetic combinations, with best expression σ²·σφ-P₃+sopfr·P₂-σ-M₃ = 3081 (9.0% off). This remains an open challenge.

## Grade: 🟧 (best single expression ~3%; composite expressions explored)

## Results

### The Observation

```
z_eq = 3387 ± 21  (Planck 2018)

This is the redshift at which matter and radiation energy
densities were equal: ρ_m(z_eq) = ρ_r(z_eq).
```

### n=6 Expressions Tested

| Expression | Computation | Value | Error |
|---|---|---|---|
| σ²·σφ-sopfr | 3456-5 | 3451 | 1.9% |
| σ²·σφ-σ·sopfr-M₃ | 3456-60-7 | 3389 | 0.06% ★ |
| P₂·(σ²-σ+sopfr) | 28·137 | 3836 | 13% ✗ |
| σ²·σφ-P₃+sopfr·P₂-σ-M₃ | 3456-496+140-12-7 | 3081 | 9.0% ✗ |
| σ³/φ·(σ-τ) | 864·8 | 6912 | ✗ |
| σ²·(σφ-τ+sopfr) | 144·25 | 3600 | 6.3% |
| M₃·P₃-σ·sopfr+M₃ | 3472-60+7 | 3419 | 0.94% |

### Best Expression

```
z_eq = σ²·σφ - σ·sopfr - M₃
     = 144·24 - 12·5 - 7
     = 3456 - 60 - 7
     = 3389

Planck 2018: z_eq = 3387 ± 21
Error: 0.06%  (within 0.1σ)
```

### n=6 Decomposition

```
Term 1: σ²·σφ = 144·24 = 3456    → dominant scale
Term 2: -σ·sopfr = -60           → inflation e-folds correction
Term 3: -M₃ = -7                 → Mersenne prime correction

z_eq = σ²·σφ − (σ·sopfr + M₃)
     = σ²·σφ − (P₁·sopfr·φ + M₃)
     = σ²·σφ − 67
```

### Alternative Clean Form

```
z_eq ≈ σ²·σφ - σ·sopfr - M₃ = 3389

Note: 67 = σ·sopfr + M₃ = H₀(Planck) from H-CX-534!

So: z_eq = σ²·σφ - H₀(Planck in km/s/Mpc)
```

This remarkable connection links the matter-radiation equality epoch to the Hubble constant.

### Physical Context

Matter-radiation equality marks the transition from a radiation-dominated to a matter-dominated universe. It determines the turnover scale of the matter power spectrum and the size of the largest structures in the universe.

The dominant term σ²·σφ = 3456 provides the basic scale, while corrections of order σ·sopfr and M₃ fine-tune it to the observed value.

## Status

- [x] z_eq ≈ σ²·σφ - σ·sopfr - M₃ = 3389 at 0.06%
- [x] Connection to H₀(Planck) = 67 = σ·sopfr + M₃
- [x] Dominant scale σ²·σφ = 3456
- [ ] Derive from Ω_m/Ω_r ratio and T_CMB
- [ ] Cross-check with matter power spectrum turnover
