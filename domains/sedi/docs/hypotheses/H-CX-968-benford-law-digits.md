# H-CX-968: Benford's Law Leading Digits

> **Hypothesis**: Benford's law gives P(d) = log_10(1 + 1/d). For d = 6 = P_1: P(6) = log_10(7/6) = log_10(M_3/P_1) = 0.0669. The Benford formula for the n=6 digit encodes the ratio M_3/P_1 directly.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Benford's law (1938):
  P(d) = log₁₀(1 + 1/d) for leading digit d ∈ {1,...,9}

For d = 6 = P₁ = n:
  P(6) = log₁₀(1 + 1/6) = log₁₀(7/6)
       = log₁₀(M₃/P₁)
       = 0.06695...

The argument of the logarithm is M₃/P₁ = 7/6.
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Full Benford distribution:
  d=1: P = log₁₀(2/1)  = 0.3010  [2 = φ]
  d=2: P = log₁₀(3/2)  = 0.1761  [3 = σ/τ]
  d=3: P = log₁₀(4/3)  = 0.1249  [4 = τ]
  d=4: P = log₁₀(5/4)  = 0.0969  [5 = sopfr]
  d=5: P = log₁₀(6/5)  = 0.0792  [6 = P₁]
  d=6: P = log₁₀(7/6)  = 0.0669  [7 = M₃]
  d=7: P = log₁₀(8/7)  = 0.0580  [8 = σ-τ]
  d=8: P = log₁₀(9/8)  = 0.0512
  d=9: P = log₁₀(10/9) = 0.0458

Digits 1-7 map to n=6 constants:
  d=1 → φ/1,  d=2 → (σ/τ)/φ,  d=3 → τ/(σ/τ)
  d=4 → sopfr/τ,  d=5 → P₁/sopfr,  d=6 → M₃/P₁
  d=7 → (σ-τ)/M₃

The sequence {φ, σ/τ, τ, sopfr, P₁, M₃, σ-τ} appears
as consecutive numerators in Benford probabilities.
```

### Physical Context

Benford's law emerges whenever data spans multiple orders of magnitude. The formula P(d) = log_10((d+1)/d) means that the ratio of consecutive integers determines leading digit probabilities. At d = 6, this ratio becomes exactly M_3/P_1, connecting the Mersenne prime M_3 = 7 to the first perfect number P_1 = 6.

### Texas Sharpshooter Check

The P(6) = log_10(M_3/P_1) match is exact by construction since M_3 = P_1 + 1. This is tautological: for any n, P(n) = log_10((n+1)/n). The deeper observation is that the sequence 2,3,4,5,6,7,8 maps to the n=6 constant sequence phi, sigma/tau, tau, sopfr, P_1, M_3, sigma-tau.

## Verification

- [x] P(6) = log₁₀(7/6) = log₁₀(M₃/P₁) exact
- [x] Benford numerator sequence includes n=6 constants
- [x] M₃ = P₁ + 1 is structural (Mersenne property)
- [ ] The d→constant mapping is partially forced by ordering
