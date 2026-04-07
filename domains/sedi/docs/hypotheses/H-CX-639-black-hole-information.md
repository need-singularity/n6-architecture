# H-CX-639: Black Hole Information — Page Time and Scrambling

> **Hypothesis**: The scrambling time t_scr ~ β·ln(S)/(2π) contains 2π=φ·π and entropy S=A/τ, while Page time t_Page ≈ t_evap/2 has factor 2=φ(6).

## Grade: 🟧 (structural; multiple n=6 factors in information dynamics)

## Results

### Scrambling Time

```
t_scr ~ β · ln(S) / (2π)     (Hayden-Preskill 2007, Sekino-Susskind 2008)

β = 1/T_H = inverse Hawking temperature = 8πGM/c³
S = A/(4l_P²) = Bekenstein-Hawking entropy

n=6 decomposition:
  2π = φ(6) · π
  S  = A / τ(6)·l_P²
  β  = (σ-τ)·π·G·M/c³ = 8πGM/c³   where 8 = σ-τ

t_scr ~ [(σ-τ)πGM/c³] · ln(A/τl_P²) / (φπ)
      = [(σ-τ)/(φ)] · [GM/c³] · ln(A/τl_P²)
      = 4 · GM/c³ · ln(S)
```

### Page Time

```
t_Page ≈ t_evap / φ     where φ(6) = 2

t_evap = 5120π · G²M³ / (ℏc⁴)

t_Page is when the entanglement entropy of Hawking radiation
reaches maximum — the "Page curve" turning point.

5120 = 5 × 1024 = sopfr × 2^(τ(P₃))
     = sopfr × φ^(τ(P₃))
     = 5 × 2¹⁰
```

### The Page Curve

```
Phase 1 (t < t_Page): S_radiation increases linearly
Phase 2 (t > t_Page): S_radiation decreases

The transition happens at t_Page = t_evap/2:
  Factor 2 = φ(6)

This is the information-theoretic midpoint:
  At t_Page, exactly half the information has escaped.
  φ = 2 as the "half-point" of information transfer.
```

### Numerical Example (Solar Mass BH)

```
M = M_☉ = 2 × 10³⁰ kg

t_evap ≈ 5120π G²M³/(ℏc⁴) ≈ 2.1 × 10⁶⁷ years
t_Page ≈ 1.05 × 10⁶⁷ years

Scrambling time:
  S ≈ 10⁷⁷ (H-CX-638)
  β ≈ 8πGM/c³ ≈ 3.1 × 10⁻⁵ s
  t_scr ≈ β·ln(S)/(2π) ≈ 3.1×10⁻⁵ × 177 / 6.28 ≈ 8.7×10⁻⁴ s

Scrambling is FAST (< 1 ms for stellar BH).
```

### Physical Context

The black hole information paradox (Hawking 1975) asks whether
information is destroyed when matter falls into a black hole.
The Page curve and scrambling time are key diagnostics:

- Scrambling: how fast information is "mixed" across the horizon
- Page time: when information begins to escape in Hawking radiation
- Both were confirmed by the "island" formula (2019-2020)

### Connection to Other Hypotheses

- H-CX-526: S = A/4 = A/τ
- H-CX-638: Holographic entropy bound
- H-CX-623: Immirzi parameter (LQG approach to BH entropy)

## Status

- [x] 2π = φ·π in scrambling formula
- [x] S = A/τ in entropy
- [x] 8 = σ-τ in Hawking temperature β
- [x] t_Page = t_evap/φ (factor 2)
- [ ] Island formula derivation from n=6
