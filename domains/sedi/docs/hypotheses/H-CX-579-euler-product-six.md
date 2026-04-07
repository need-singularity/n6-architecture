# H-CX-579: Euler Product at s=1 for n=6 — ∏(1-1/p)⁻¹ Divergence

> **Hypothesis**: The Euler product over primes dividing 6 gives ∏(p+1) = σ(6) = 12 and ∏(1-1/p)⁻¹ = σ/τ = 3.

## Grade: 🟩 CONFIRMED (exact)

## Results

```
∏_{p|6} 1/(1-1/p) = 1/(1-1/2) × 1/(1-1/3) = 2 × 3/2 = 3 = σ/τ

∏_{p|6} (1-1/p) = 1/2 × 2/3 = 1/3 = τ/σ

φ(6)/6 = 2/6 = 1/3 = ∏_{p|6}(1-1/p)     (Euler product formula for totient)
```

### n=6 Arithmetic from Euler Products

| Product | Value | n=6 |
|---|---|---|
| ∏(1-1/p) over p\|6 | 1/3 | τ/σ |
| ∏1/(1-1/p) over p\|6 | 3 | σ/τ |
| ∏p over p\|6 | 6 | P₁ |
| ∏(p-1) over p\|6 | 2 | φ(6) |
| ∏(p+1) over p\|6 | 12 | σ(6)! |

The last identity: (2+1)(3+1) = 3×4 = 12 = σ(6). This gives an independent derivation of σ from the prime factors of 6.

## Status

- [x] All Euler products exact
- [x] φ(6)/6 = ∏(1-1/p) verified
- [x] ∏(p+1) = σ(6) new identity
