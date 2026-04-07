# H-CX-687: Bernoulli Number Denominators and n=6 Divisibility

> **Hypothesis**: All Bernoulli number denominators B₂ₖ for small k are products of n=6 constants. Every even Bernoulli denominator is divisible by P₁=6 (von Staudt-Clausen).

## Grade: 🟩 CONFIRMED

## Results

### Bernoulli Numbers and n=6 Denominators

```
B₂  =  1/6    = 1/P₁
B₄  = −1/30   = −1/(sopfr·P₁)         30 = 5·6
B₆  =  1/42   = 1/(P₁·M₃)             42 = 6·7
B₈  = −1/30   = −1/(sopfr·P₁)         30 = 5·6
B₁₀ =  5/66   = sopfr/(P₁·11)         66 = 6·11
B₁₂ = −691/2730                        2730 = 6·5·7·13
```

### Denominator Factorizations

```
denom(B₂)  =   6 = P₁
denom(B₄)  =  30 = P₁ · sopfr       = 6·5
denom(B₆)  =  42 = P₁ · M₃          = 6·7
denom(B₈)  =  30 = P₁ · sopfr       = 6·5
denom(B₁₀) =  66 = P₁ · (σ−1)       = 6·11
denom(B₁₂) = 2730 = P₁ · sopfr · M₃ · 13 = 6·455

All divisible by P₁ = 6  ✓
```

### Von Staudt-Clausen Theorem

```
B₂ₖ = A₂ₖ − Σ 1/p   (sum over primes p where (p−1)|2k)

For B₂: primes with (p−1)|2 → p = 2, 3
  denom = lcm(1, 2, 3) = 6 = P₁

For B₄: primes with (p−1)|4 → p = 2, 3, 5
  denom = lcm(1, 2, 3, 5) = 30 = P₁·sopfr

For B₆: primes with (p−1)|6 → p = 2, 3, 7
  denom = lcm(1, 2, 3, 7) = 42 = P₁·M₃
```

### Why P₁=6 Always Divides

```
For any even index 2k ≥ 2:
  (p−1)|2k is satisfied by p = 2 (since 1|2k) and p = 3 (since 2|2k)
  So 2 and 3 always contribute to the sum
  ⟹ denom(B₂ₖ) is always divisible by lcm(2,3) = 6 = P₁

This is a theorem, not a coincidence: the prime factors of
P₁ = 6 = 2·3 are the smallest primes, guaranteeing 6 | denom(B₂ₖ).
```

### Parameter Map

| Bernoulli | Denominator | TECS-L factorization |
|---|---|---|
| B₂ | 6 | P₁ |
| B₄ | 30 | P₁·sopfr |
| B₆ | 42 | P₁·M₃ |
| B₈ | 30 | P₁·sopfr |
| B₁₀ | 66 | P₁·(σ−1) |
| B₁₂ | 2730 | P₁·sopfr·M₃·13 |

## Verification

- [x] B₂ = 1/6 = 1/P₁ exact
- [x] B₄ = −1/30 = −1/(P₁·sopfr) exact
- [x] B₆ = 1/42 = 1/(P₁·M₃) exact
- [x] Von Staudt-Clausen guarantees 6 | denom(B₂ₖ) for all k ≥ 1
- [x] Primes 2, 3 of P₁ = 6 are minimal primes → always divide

## Status

New. The universality of P₁=6 in Bernoulli denominators follows from von Staudt-Clausen and the minimality of 2, 3.
