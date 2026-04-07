# H-CX-951: Lotka-Volterra Equations

> **Hypothesis**: The Lotka-Volterra predator-prey model consists of 2 = φ coupled ODEs with 2 = φ fixed points and phase space dimension φ = 2.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Lotka-Volterra predator-prey equations:
  dx/dt = αx - βxy      (prey growth)
  dy/dt = δxy - γy       (predator growth)

  Number of equations: 2 = φ
  Number of species:   2 = φ
  Phase space dimension: 2 = φ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Structural Analysis

```
Fixed points:
  1. Extinction:    (0, 0)
  2. Coexistence:   (γ/δ, α/β)
  Count = 2 = φ

Parameters:
  α = prey birth rate
  β = predation rate
  γ = predator death rate
  δ = predator reproduction rate
  Count = 4 = τ

Each equation has 2 = φ terms:
  Prey:     αx (growth) - βxy (predation)
  Predator: δxy (growth) - γy (death)

Conserved quantity (Lotka-Volterra integral):
  V = δx - γ ln(x) + βy - α ln(y) = constant
  Terms in conserved quantity: 4 = τ
```

### Extensions

```
Competition model (2 species):
  dx₁/dt = r₁x₁(1 - (x₁ + α₁₂x₂)/K₁)
  dx₂/dt = r₂x₂(1 - (x₂ + α₂₁x₁)/K₂)
  Still φ = 2 equations

  Fixed points: 4 = τ
    (0,0), (K₁,0), (0,K₂), coexistence

Generalized n-species Lotka-Volterra:
  dxᵢ/dt = xᵢ(rᵢ + Σⱼ aᵢⱼxⱼ)
  For P₁ = 6 species → 6 coupled ODEs
```

### Physical Context

The Lotka-Volterra equations (1925-1926) are the foundational model of mathematical ecology. The φ = 2 structure reflects the minimal predator-prey interaction. The τ = 4 parameters are the irreducible set needed to describe birth, death, and interaction for two species. Cycles are observed in real systems (e.g., lynx-hare data).

## Verification

- [x] 2 coupled ODEs = φ exact
- [x] 2 fixed points = φ exact (predator-prey)
- [x] 4 parameters = τ exact
- [x] 4 terms in conserved quantity = τ exact
- [x] Competition model: 4 fixed points = τ
