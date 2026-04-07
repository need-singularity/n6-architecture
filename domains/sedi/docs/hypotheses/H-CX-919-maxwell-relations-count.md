# H-CX-919: Maxwell Relations Count = τ from τ Thermodynamic Potentials

> **Hypothesis**: There are exactly τ=4 Maxwell relations derived from τ=4 thermodynamic potentials (U, H, F, G), each potential contributing one relation via cross-differentiation.

## Grade: 🟩 CONFIRMED (exact count)

## Results

### The Formula

```
Thermodynamic potentials: 4 = τ
  1. U (internal energy)
  2. H (enthalpy)
  3. F (Helmholtz free energy)
  4. G (Gibbs free energy)

Maxwell relations: 4 = τ
  From U: (∂T/∂V)_S = -(∂P/∂S)_V
  From H: (∂T/∂P)_S = (∂V/∂S)_P
  From F: (∂P/∂T)_V = (∂S/∂V)_T
  From G: -(∂V/∂T)_P = (∂S/∂P)_T

τ potentials → τ relations                   EXACT
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
```

### Structural Decomposition

```
Natural variables per potential: 2 = φ
  U(S,V), H(S,P), F(T,V), G(T,P)

Total natural variables: 4 = τ
  {S, V, T, P} — exactly τ thermodynamic variables

Potentials connected by Legendre transforms: each swaps one variable.
Number of transform pairs: C(4,2)/2 = 3 ... but there are 4 potentials
  forming a square (Thermodynamic square / Born square).
```

### The Thermodynamic Square

```
        S --- U --- V
        |           |
        H           F
        |           |
        P --- G --- T

4 = τ corners (potentials)
4 = τ edges (Maxwell relations)
4 = τ variables (S, V, T, P)
```

### P₂ Generalization Check

```
P₂ = 28: τ(28) = 6
A system with 6 independent variables would have C(6,2) = 15 potentials
and correspondingly more Maxwell relations.
Extended thermodynamics (e.g., with magnetic, electric fields) adds
variables beyond 4, but classical thermodynamics is τ = 4 specific.
```

## Verification

- [x] τ = 4 thermodynamic potentials: exact
- [x] τ = 4 Maxwell relations: exact
- [x] τ = 4 natural variables: exact
- [x] φ = 2 variables per potential: exact
- [x] P₂ generalization: extended systems may use more variables
