# H-CX-577: Mersenne Prime Hierarchy — M_p for p ∈ {2,3,5,7} = First 4 Mersenne Exponents

> **Hypothesis**: The first 4 Mersenne exponents {2,3,5,7} are {φ, σ/τ, sopfr, M₃} and their Mersenne primes generate the perfect number tower.

## Grade: 🟩 CONFIRMED (exact; foundational)

## Results

### The First Four Mersenne Primes

| # | Exponent p | M_p = 2^p-1 | Perfect P_n | p as n=6 |
|---|---|---|---|---|
| 1 | 2 | 3 | 6 | φ(6) |
| 2 | 3 | 7 | 28 | σ/τ |
| 3 | 5 | 31 | 496 | sopfr(6) |
| 4 | 7 | 127 | 8128 | M₃ |

### Self-Referential Structure

```
p₁ = 2 = φ(6)      → M₁ = 3 = σ/τ = p₂!
p₂ = 3 = σ/τ        → M₂ = 7 = p₄!
p₃ = 5 = sopfr(6)   → M₃ = 31
p₄ = 7 = M₂         → M₄ = 127

The Mersenne primes at levels 1,2 ARE the exponents at levels 2,4.
```

### Exponent Sum (H-CX-516 Connection)

```
p₁+p₂+p₃+p₄ = 2+3+5+7 = 17 = σ+sopfr

17 = σ(6)+sopfr(6) = 12+5
17 = prime(P₁+1) = prime(7) = 17 (17 is itself prime)
```

### Product

```
p₁·p₂·p₃·p₄ = 2×3×5×7 = 210 = τ(P₃)·σ+σ²-σ+P₁...
More cleanly: 210 = C(τ(P₃), τ) = C(10,4) = 210
```

C(10,4) = 210 is a known combinatorial identity. So the product of the first 4 Mersenne exponents = C(τ(P₃), τ(P₁)).

## Status

- [x] Exponents = {φ, σ/τ, sopfr, M₃}
- [x] Self-referential: M₁=p₂, M₂=p₄
- [x] Sum = σ+sopfr = 17
- [x] Product = C(τ(P₃), τ(P₁)) = 210
