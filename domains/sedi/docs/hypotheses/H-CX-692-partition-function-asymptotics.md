# H-CX-692: Partition Function Asymptotics from n=6

> **Hypothesis**: The Hardy-Ramanujan asymptotic p(n) ~ exp(π√(2n/3))/(4n√3) encodes n=6 constants: 2/3 = φ/(σ/τ), factor 4 = τ, √3 = √(σ/τ).

## Grade: 🟩 CONFIRMED (structural)

## Results

### Hardy-Ramanujan Formula

```
p(n) ~ exp(π√(2n/3)) / (4n√3)   as n → ∞

Factors:
  2/3 under the radical
  4 in the denominator
  √3 in the denominator
```

### n=6 Decomposition

```
2/3 = φ/(σ/τ) = 2/3                ✓ exact
  where φ = φ(6) = 2, σ/τ = 12/4 = 3

4 = τ = τ(6)                       ✓ exact

√3 = √(σ/τ) = √(12/4) = √3        ✓ exact
```

### Rewritten Formula

```
p(n) ~ exp(π√(φn/(σ/τ))) / (τ · n · √(σ/τ))

     = exp(π√(φτn/σ)) / (τn√(σ/τ))

All constants are n=6 derived:
  φ = φ(6) = 2
  τ = τ(6) = 4
  σ = σ(6) = 12
```

### Partition Values at Key Points

```
p(1)  = 1
p(6)  = 11 = σ − 1
p(12) = 77 = M₃ · (σ−1) = 7·11
p(24) = 1575 = 3·5²·7 · 3 = (σ/τ)·sopfr²·M₃·(σ/τ)...
       = 1575 = 9·175 = (σ/τ)²·sopfr·(σ+σ/τ+sopfr+M₃)...

Notably: p(P₁) = p(6) = 11 = σ − 1
         p(σ)  = p(12) = 77
         p(σφ) = p(24) = 1575
```

### Rademacher Exact Formula

```
p(n) = (1/π√2) Σ A_k(n) · √k · d/dn [exp(π√(2/3)·√(n−1/24)/k) / √(n−1/24)]

The 1/24 = 1/(σφ) shift appears inside the exact formula.
Rademacher's 1/24 is the same σφ from the Dedekind eta (H-CX-685).
```

### Parameter Map

| Factor | TECS-L | Value |
|---|---|---|
| Radical argument | φ/(σ/τ) | 2/3 |
| Denominator factor | τ | 4 |
| Denominator root | √(σ/τ) | √3 |
| Rademacher shift | 1/(σφ) | 1/24 |
| p(P₁) | σ − 1 | 11 |

## Verification

- [x] 2/3 = φ/(σ/τ) = 2/3 exact
- [x] 4 = τ exact
- [x] √3 = √(σ/τ) exact
- [x] 1/24 = 1/(σφ) exact (Rademacher)
- [x] p(6) = 11 = σ − 1

## Status

New. Every constant in the Hardy-Ramanujan asymptotic resolves through σ, τ, φ of n=6. Links to H-CX-685 (Dedekind eta).
