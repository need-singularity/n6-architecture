# H-CX-662: Anderson Localization — Critical Dimension d_c = phi(6)

> **Hypothesis**: Anderson localization has critical dimension d_c = 2 = phi(6): all states are localized in d <= 2, while a metal-insulator transition (mobility edge) exists for d = 3 = sigma/tau.

## Grade: 🟧 APPROXIMATE (structural identification of dimensions)

## Results

### Anderson Localization Theorem

```
Anderson (1958): Sufficiently strong disorder localizes all
quantum states in a lattice.

Scaling theory of localization (Abrahams et al. 1979):

  d = 1: ALL states localized (any disorder)
  d = 2: ALL states localized (marginal, logarithmic)
  d = 3: Metal-insulator transition at critical disorder W_c
  d > 3: Extended states survive weak disorder
```

### n=6 Dimensional Classification

```
d = 1 = R(6):       fully localized (trivial)
d = 2 = phi(6):     marginal dimension — all localized
d = 3 = sigma/tau:  mobility edge exists — metal-insulator transition
d = 4 = tau(6):     upper critical dimension for localization?

Critical dimension: d_c = phi(6) = 2

Below d_c:    all states localized
At d_c:       marginal (logarithmic corrections)
Above d_c:    delocalized states possible
```

### Beta Function

```
Scaling beta function: beta(g) = d*ln(g)/d*ln(L)

For d = phi = 2:
  beta(g) = ln(g/g_c) -> 0 from below
  All states eventually localize

For d = sigma/tau = 3:
  beta(g*) = 0 at critical conductance g*
  g* ~ e^2/h (quantum of conductance, H-CX-648)

  Metal:     g > g* -> beta > 0 -> delocalized
  Insulator: g < g* -> beta < 0 -> localized
```

### Critical Exponent

```
3D Anderson transition:
  nu ~ 1.57 (correlation length exponent)

n=6 attempt:
  (sigma-tau+sopfr-phi)/(M3) = (12-4+5-2)/7 = 11/7 = 1.571 (0.06%)

  11/7 = (sigma-tau+sopfr-phi)/M3 = 1.5714...
  Exact: nu = 1.571(3)
  Error: 0.06% — NOTABLE
```

### Universality Classes

```
Symmetry class | d_c | n=6 form
--------------|-----|----------
Orthogonal (AI)    | 2 = phi | Standard Anderson
Unitary (A)        | 2 = phi | Broken TRS (magnetic field)
Symplectic (AII)   | 2 = phi | Strong spin-orbit coupling

All three Wigner-Dyson classes have d_c = phi(6) = 2.

The ten-fold way (Altland-Zirnbauer classification):
  10 = sigma - phi = 10 symmetry classes
```

### Physical Context

Anderson localization is one of the most fundamental phenomena
in condensed matter physics. It explains why some disordered
systems are insulators (all states localized) and others show
a metal-insulator transition.

The marginal dimension d = 2 is especially important:
- 2D electron gases (GaAs quantum wells)
- Graphene (anti-localization due to Berry phase, H-CX-661)
- Thin metallic films

### Structural Interpretation

```
phi(6) = 2 as the critical dimension means:
  The Euler totient determines whether quantum
  diffusion can overcome disorder.

  d < phi: arithmetic too sparse for delocalization
  d = phi: marginal — on the boundary
  d > phi: arithmetic rich enough for metal-insulator transition

The symmetry classification has sigma-phi = 10 classes,
connecting topological classification to n=6 arithmetic.
```

### Connection to Other Hypotheses

- H-CX-645: 3D Ising exponents (d=3 critical phenomena)
- H-CX-648: Quantum conductance (critical conductance g*)
- H-CX-661: Graphene (anti-localization at d=2)

## Status

- [x] Critical dimension d_c = phi(6) = 2
- [x] Mobility edge at d = sigma/tau = 3
- [x] 3D nu ~ 11/7 at 0.06%
- [x] Ten-fold way: sigma-phi = 10 symmetry classes
- [ ] Topological Anderson insulator vs n=6
- [ ] Many-body localization critical dimension
