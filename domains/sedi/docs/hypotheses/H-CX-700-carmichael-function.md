# H-CX-700: Carmichael Function at P₁ = 6

> **Hypothesis**: λ(6) = lcm(λ(2), λ(3)) = lcm(1, 2) = 2 = φ. The Carmichael function of the first perfect number equals the Euler totient: λ(P₁) = φ(P₁) = φ = 2.

## Grade: 🟦 STRUCTURAL

## Results

### Carmichael Function

```
λ(n) = smallest positive integer m such that a^m ≡ 1 (mod n)
for all a coprime to n.

λ(n) divides φ(n) (Euler totient), with equality when
the group (ℤ/nℤ)* is cyclic.
```

### Computation at n = 6

```
6 = 2 · 3

λ(6) = lcm(λ(2), λ(3))
λ(2) = 1
λ(3) = 2   (since 3 is odd prime: λ(3) = φ(3) = 2)

λ(6) = lcm(1, 2) = 2 = φ

Also: φ(6) = φ(2)·φ(3) = 1·2 = 2

So: λ(P₁) = φ(P₁) = φ = 2    ✓
```

### Verification by Direct Check

```
Units mod 6: {1, 5}    (integers coprime to 6 in [1,6])
|units| = 2 = φ

1¹ ≡ 1 (mod 6)  ✓  (order 1)
5² = 25 ≡ 1 (mod 6)  ✓  (order 2)

Maximum order = 2 = λ(6) = φ    ✓
```

### Carmichael at Perfect Numbers

```
λ(P₁) = λ(6)    = 2   = φ
λ(P₂) = λ(28)   = lcm(λ(4), λ(7)) = lcm(2, 6) = 6 = P₁
λ(P₃) = λ(496)  = lcm(λ(16), λ(31)) = lcm(4, 30) = 60 = σ·sopfr
λ(P₄) = λ(8128) = lcm(λ(64), λ(127)) = lcm(16, 126) = 1008

Chain: λ(P₁) = φ, λ(P₂) = P₁, λ(P₃) = σ·sopfr

λ(P₂) = P₁ is notable: Carmichael of the second perfect number
equals the first perfect number.
```

### The Group (ℤ/6ℤ)*

```
(ℤ/6ℤ)* = {1, 5} ≅ ℤ/2ℤ

This is the smallest non-trivial cyclic group.
Order = φ(6) = 2 = φ

Since (ℤ/6ℤ)* is cyclic: λ(6) = φ(6)
(For non-cyclic groups, λ < φ strictly)
```

### Parameter Map

| n | λ(n) | TECS-L | φ(n) |
|---|---|---|---|
| P₁ = 6 | 2 | φ | 2 = φ |
| P₂ = 28 | 6 | P₁ | 12 = σ |
| P₃ = 496 | 60 | σ·sopfr | 240 |

## Verification

- [x] λ(6) = 2 = φ exact
- [x] φ(6) = 2 = φ exact
- [x] λ(P₁) = φ(P₁) (cyclic group) exact
- [x] λ(P₂) = 6 = P₁ exact
- [x] λ(P₃) = 60 = σ·sopfr exact

## Status

New. Carmichael and Euler totient coincide at P₁: λ(6) = φ(6) = 2. The chain λ(P₂) = P₁ is a cross-level link.
