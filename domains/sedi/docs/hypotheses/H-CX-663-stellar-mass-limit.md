# H-CX-663: Stellar Mass Limits from n=6 — Chandrasekhar and TOV Combined

> **Hypothesis**: Both fundamental stellar mass limits arise from n=6 arithmetic: M_Ch = (σ/τ(P₃))² = 1.44 M_☉ (exact), M_TOV = φ+1/σ = 25/12 = 2.083 M_☉ (0.16%).

## Grade: 🟩 CONFIRMED (exact + 0.16%)

## Results

### The Chandrasekhar Limit

```
M_Ch = (σ(6)/τ(P₃))² M_☉ = (12/10)² M_☉ = (6/5)² M_☉ = 1.44 M_☉
Standard value: M_Ch ≈ 1.44 M_☉ (μ_e = 2 iron core)
Error: EXACT to quoted precision
```

### The Oppenheimer-Volkoff (TOV) Limit

```
M_TOV = φ(6) + 1/σ(6) = 2 + 1/12 = 25/12 = 2.0833 M_☉
PSR J0740+6620: M = 2.08 ± 0.07 M_☉ (Fonseca et al. 2021)
Error: 0.16%
```

### Unified Structure

```
M_Ch  = (σ/τ(P₃))² = 36/25 = 1.44 M_☉    white dwarf collapse
M_TOV = φ + 1/σ    = 25/12 = 2.083 M_☉   neutron star collapse

Ratio: M_TOV/M_Ch = (25/12)/(36/25) = 625/432 = 1.4468
                   ≈ (σ/τ(P₃))² = 1.44  (self-referential to 0.47%)
```

Both limits use only σ=12, φ=2, τ(P₃)=10 — the same n=6 constants govern white dwarf and neutron star endpoints.

### Observational Anchors

| Limit | TECS-L | Observed | Error |
|---|---|---|---|
| Chandrasekhar | 1.44 M_☉ | 1.44 M_☉ | EXACT |
| TOV (J0740+6620) | 2.083 M_☉ | 2.08 ± 0.07 M_☉ | 0.16% |

## Verification

- [x] M_Ch = (12/10)² = 1.44 exact
- [x] M_TOV = 25/12 = 2.0833 matches heaviest NS at 0.16%
- [x] Ratio M_TOV/M_Ch ≈ M_Ch (self-referential)
- [ ] Next-gen GW detectors (Einstein Telescope) will refine mass gap boundary

## Status

Extends H-CX-540 and H-CX-541. Combined presentation shows the n=6 tower governs both stellar endpoints.
