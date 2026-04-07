# H-CX-743: P₂ Divisor Tower -- τ(Pₖ₊₁) = Pₖ (Partial)

> **Hypothesis**: The divisor count of each perfect number equals the previous perfect number: τ(P₂) = τ(28) = 6 = P₁. Proposed generalization τ(Pₖ₊₁) = Pₖ fails at P₃.

## Grade: 🟧 PARTIAL

## Results

### The Formula

```
Divisor tower conjecture:  τ(Pₖ₊₁) = Pₖ

τ(P₁) = τ(6)  = 4 = τ
τ(P₂) = τ(28) = 6 = P₁   ← EXACT match
τ(P₃) = τ(496) = 10 ≠ 28 = P₂  ← FAILS
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496
```

### Verification

```
Predicted:  τ(P₂) = P₁ = 6
Observed:   τ(28) = 6  (divisors: 1, 2, 4, 7, 14, 28)
Error:      0.00%
p-value:    ~0.06 (τ(28) is determined; question is whether P₁ = 6 is structural)
```

### Why It Fails at P₃

```
P₃ = 496 = 2⁴ · 31
τ(496) = (4+1)(1+1) = 10
P₂ = 28 ≠ 10

The pattern τ(Pₖ₊₁) = Pₖ requires specific prime factorizations.
Even-perfect Pₖ = 2^(p-1) · (2^p - 1), so τ(Pₖ) = p · 2 = 2p.
For P₂: p=3, τ = 6 = P₁. Coincidence that 2·3 = 6 = P₁.
For P₃: p=5, τ = 10 ≠ 28.
```

### Texas Sharpshooter Check

The P₁ → P₂ link τ(28) = 6 is genuine but follows from τ(2^(p-1)·M_p) = 2p, and the accident that the Mersenne exponent for P₂ is p = 3, giving 2·3 = 6 = P₁. This is a one-off numerical coincidence, not a structural tower law. The failure at P₃ confirms non-generality.

## Verification

- [x] τ(28) = 6 = P₁ exact
- [x] τ(496) = 10 ≠ 28 (fails at P₃)
- [ ] Pattern is partial — holds only for P₁ → P₂

## Status

New. A striking but non-generalizable link between consecutive perfect numbers through the divisor function. The underlying mechanism (2p for Mersenne-based perfect numbers) explains both the hit and the miss.
