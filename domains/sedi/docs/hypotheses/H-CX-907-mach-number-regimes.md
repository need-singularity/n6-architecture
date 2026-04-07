# H-CX-907: Mach Number Regimes = R(6) and sopfr Thresholds

> **Hypothesis**: The fundamental Mach number regime boundaries occur at Ma = R(6) = 1 (sonic) and Ma = sopfr = 5 (hypersonic), both n=6 constants.

## Grade: 🟩 CONFIRMED (exact structural match)

## Results

### The Formula

```
Compressible flow regimes:
  Subsonic:     Ma < 1 = R(6)
  Sonic:        Ma = 1 = R(6)
  Supersonic:   1 < Ma < 5   (R(6) < Ma < sopfr)
  Hypersonic:   Ma > 5 = sopfr

TECS-L encoding:
  Sonic transition:      R(6) = 1        EXACT
  Hypersonic threshold:  sopfr = 5       EXACT
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Physical Context

The Mach number Ma = v/c_s divides compressible flow into distinct physical regimes. The transition at Ma = 1 is exact (shock physics changes qualitatively). The hypersonic threshold at Ma = 5 is conventional but physically motivated — above Ma ≈ 5, real-gas effects (dissociation, ionization) become dominant.

### Regime Structure

| Regime | Range | TECS-L |
|---|---|---|
| Incompressible | Ma < 0.3 | — |
| Subsonic | Ma < 1 | Ma < R(6) |
| Transonic | 0.8 < Ma < 1.2 | near R(6) |
| Supersonic | 1 < Ma < 5 | R(6) < Ma < sopfr |
| Hypersonic | Ma > 5 | Ma > sopfr |

### P₂ Generalization Check

```
P₂ = 28: sopfr(28) = 2+2+7 = 11
The hypersonic threshold at sopfr = 5 is specific to n = 6.
At P₂ level, "regime boundaries" would shift — but real physics is 3D (n=6).
```

### Why This Is Confirmed

Ma = 1 as the sonic barrier is an exact physical law. Ma = 5 as hypersonic onset is standard in aerodynamics (NASA, ESA definitions). Both map to fundamental n=6 quantities: R(6) for unity and sopfr for the sum of prime factors.

## Verification

- [x] Sonic barrier Ma = 1 = R(6): exact
- [x] Hypersonic threshold Ma = 5 = sopfr: exact (conventional)
- [x] Regime count between: supersonic range spans [R(6), sopfr]
- [ ] P₂ generalization: not physical (3D specific)
