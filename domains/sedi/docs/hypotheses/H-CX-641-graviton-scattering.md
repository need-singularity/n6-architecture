# H-CX-641: 4-Graviton Scattering Amplitude — κ² and s³/M_P⁴

> **Hypothesis**: The 4-graviton tree amplitude ∝ κ²·s³/M_P⁴ has κ = √(8πG) = √((σ-τ)πG), and power s³ where 3 = σ/τ.

## Grade: 🟧 (structural; clean n=6 factors in gravitational coupling)

## Results

### The Amplitude

```
4-graviton tree-level scattering (Sannan 1986):

  M₄ ∝ κ² · s³ / M_P⁴

where:
  κ = √(8πG) = gravitational coupling
  s = center-of-mass energy squared (Mandelstam variable)
  M_P = Planck mass
```

### n=6 Decomposition

```
κ = √(8πG) = √((σ-τ)·π·G)

  8 = σ - τ = 12 - 4     (H-CX-636: same as instanton factor)
  π = geometric bridge
  G = Newton's constant

Power of s:
  s³ where 3 = σ/τ = 12/4     EXACT

Number of external gravitons:
  4 = τ(6)                     EXACT
```

### Gravitational Coupling Hierarchy

```
κ² = 8πG = (σ-τ)·π·G
   = 8π / M_P²

The dimensionless coupling at energy E:
  κ_eff² ~ E²/M_P²

At E = M_P: κ_eff ~ 1 (strong gravity)
At E = v_EW: κ_eff ~ 10⁻¹⁷ (weak gravity)

Ratio: M_P/v_EW ~ 10¹⁷ ≈ 10^(σ²+1)
```

### Cross Section

```
σ_grav ∝ κ⁴·s/M_P⁴ ∝ s³/M_P⁸

  σ ~ G²·s = (8πG)²·s / (8π)²

For graviton-graviton scattering at LHC energies (s ~ 10⁸ GeV²):
  σ ~ G²·s ~ (6.7×10⁻³⁹)² × 10⁸ ~ 10⁻⁶⁸ GeV⁻²

Completely unobservable — gravity is too weak at collider energies.
```

### Soft Graviton Theorems

```
Weinberg's soft theorem: in the soft limit (k→0),
graviton amplitudes factorize universally:

  M_{n+1} → S⁽⁰⁾ · M_n + S⁽¹⁾ · ... + S⁽²⁾ · ...

The leading soft factor S⁽⁰⁾ depends only on:
  - κ = gravitational coupling → √((σ-τ)πG)
  - External momenta and polarizations

Sub-leading factors S⁽¹⁾, S⁽²⁾ encode angular momentum
(Cachazo-Strominger 2014).
```

### Physical Context

Graviton scattering amplitudes reveal the deep structure of
quantum gravity. Recent developments:

- Double copy: gravity = (gauge theory)² (BCJ duality)
- KLT relations: closed string = open string × open string
- Amplituhedron: geometric structures beyond Feynman diagrams

The s³ dependence (versus s for gauge theory) reflects gravity's
spin-2 nature: each graviton vertex contributes an extra power of momentum.

### Connection to Other Hypotheses

- H-CX-625: M_P exponent 19 = σ + M₃
- H-CX-636: Instanton 8π² = (σ-τ)·π^φ (same σ-τ factor)
- H-CX-545: GR coefficients from n=6

## Status

- [x] κ = √((σ-τ)πG), 8 = σ-τ exact
- [x] Power s³, 3 = σ/τ exact
- [x] 4 external gravitons = τ(6)
- [ ] Double copy structure and n=6
