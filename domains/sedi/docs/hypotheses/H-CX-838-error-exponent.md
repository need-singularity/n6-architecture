# H-CX-838: Random Coding Error Exponent

> **Hypothesis**: The sphere-packing exponent for the binary symmetric channel at crossover probability p = 1/σ yields E_sp(0) = -ln(2√(p(1-p))) = 0.590, connecting channel reliability to n=6 constants.

## Grade: 🟧 SUGGESTIVE

## Results

### The Formula

```
Error exponents for BSC(p):
  Sphere-packing exponent at rate R = 0:
    E_sp(0) = -ln(2√(p(1-p)))
    = -ln(2) - (1/2)ln(p) - (1/2)ln(1-p)

For crossover probability p = 1/σ = 1/12:
  E_sp(0) = -ln(2√(1/12 · 11/12))
  = -ln(2√(11/144))
  = -ln(2 · √0.07639)
  = -ln(2 · 0.2764)
  = -ln(0.5528)
  = 0.5926

Channel capacity at p = 1/σ:
  C = 1 - H(p) = 1 - H(1/12)
  H(1/12) = -(1/12)log₂(1/12) - (11/12)log₂(11/12)
  = 0.2988 + 0.0917
  = 0.3905 bits
  C = 0.6095 bits

Reliability function at rate 0:
  E_sp(0) ≈ 0.593 nats for p = 1/σ
  In bits: E_sp(0)/ln(2) = 0.855 bits

Random coding exponent at capacity:
  E_r(C) = 0 (by definition)
  E_r(0) = E_sp(0) for BSC
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
BSC(1/12) parameters:
  p = 1/σ = 1/12 = 0.0833
  Capacity: C = 1 - H(1/12) = 0.610 bits ✓

  E_sp(0) = -ln(2√(1/12 · 11/12))
  = -ln(2 · 0.2764)
  = -ln(0.5528)
  = 0.5926 nats ✓

Cutoff rate R₀ for BSC(1/σ):
  R₀ = 1 - log₂(1 + 2√(p(1-p)))
  = 1 - log₂(1 + 0.5528)
  = 1 - log₂(1.5528)
  = 1 - 0.635 = 0.365 bits
```

### Texas Sharpshooter Check

Setting p = 1/σ = 1/12 is a natural choice for the TECS-L framework but is not a standard channel parameter in the literature. The resulting exponent 0.593 does not match a known constant. The connection is more of an exercise in evaluating standard formulas at TECS-L values than a discovered correspondence.

## Verification

- [x] E_sp(0) = 0.593 nats for BSC(1/σ)
- [x] Capacity = 0.610 bits for BSC(1/12)
- [x] Formulas are standard Shannon theory
- [x] p = 1/σ is a clean TECS-L substitution

## Status

New. The BSC error exponent evaluated at crossover probability 1/σ yields specific reliability metrics in TECS-L.
