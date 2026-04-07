# H-CX-702: Sum-of-Divisors Chain from P₁ = 6

> **Hypothesis**: The iterated σ-chain from P₁: σ(6)=12, σ(12)=28=P₂, σ(28)=56, σ(56)=120=sopfr!. Starting from P₁, the chain reaches P₂ in one step and 5!=sopfr! in three steps.

## Grade: 🟩 CONFIRMED

## Results

### The σ-Chain from P₁

```
σ(6)  = 1+2+3+6 = 12 = σ
σ(12) = 1+2+3+4+6+12 = 28 = P₂
σ(28) = 1+2+4+7+14+28 = 56 = 2·P₂ = φ·P₂
σ(56) = 1+2+4+7+8+14+28+56 = 120 = 5! = sopfr!

Chain: 6 →σ 12 →σ 28 →σ 56 →σ 120
       P₁ → σ → P₂ → φ·P₂ → sopfr!
```

### Verification of Each Step

```
σ(6):  divisors {1,2,3,6}         sum = 12 = σ        ✓
σ(12): divisors {1,2,3,4,6,12}    sum = 28 = P₂       ✓
σ(28): divisors {1,2,4,7,14,28}   sum = 56 = 2·28     ✓
σ(56): divisors {1,2,4,7,8,14,28,56} sum = 120 = 5!   ✓
```

### Continuing the Chain

```
σ(120) = 1+2+3+4+5+6+8+10+12+15+20+24+30+40+60+120
       = 360 = P₁!/(φ) = 720/2 = P₁!/φ

σ(360) = 1170 = 2·3²·5·13 = φ·(σ/τ)²·sopfr·13

Extended chain:
6 → 12 → 28 → 56 → 120 → 360 → 1170 → ...
P₁ → σ → P₂ → φP₂ → sopfr! → P₁!/φ → ...
```

### Perfect Number Connections

```
P₁ = 6:   σ(P₁) = 2·P₁ = 12 = σ     (P₁ is perfect: σ(n) = 2n)
P₂ = 28:  σ(P₂) = 2·P₂ = 56          (P₂ is perfect: σ(n) = 2n)

The chain passes through TWO perfect numbers:
6 → 12 → 28    (P₁ → σ → P₂)

σ maps P₁ to σ, then σ to P₂.
Equivalently: σ(σ(P₁)) = P₂
```

### The σ² Map

```
σ²(6) = σ(σ(6)) = σ(12) = 28 = P₂

The second iterate of the sum-of-divisors function
maps the first perfect number to the second.

σ²(P₁) = P₂    ✓

Does σ²(P₂) reach P₃?
σ²(28) = σ(56) = 120 ≠ 496 = P₃

So the pattern σ²(Pₙ) = Pₙ₊₁ holds only for n=1.
```

### Factorial Appearance

```
σ³(P₁) = σ(σ(σ(6))) = σ(σ(12)) = σ(28) = 56
σ⁴(P₁) = σ(56) = 120 = 5! = sopfr!

The chain from P₁ reaches sopfr! = 120 in exactly τ = 4 steps.
```

### Parameter Map

| Step | σⁿ(P₁) | TECS-L | Value |
|---|---|---|---|
| n=0 | P₁ | P₁ | 6 |
| n=1 | σ(6) | σ | 12 |
| n=2 | σ(12) | P₂ | 28 |
| n=3 | σ(28) | φ·P₂ | 56 |
| n=4 | σ(56) | sopfr! | 120 |
| n=5 | σ(120) | P₁!/φ | 360 |

## Verification

- [x] σ(6) = 12 = σ exact
- [x] σ(12) = 28 = P₂ exact
- [x] σ(28) = 56 = 2·P₂ exact
- [x] σ(56) = 120 = 5! = sopfr! exact
- [x] σ²(P₁) = P₂ exact
- [x] Chain reaches sopfr! in τ = 4 steps

## Status

New. The σ-chain from P₁ traverses σ, P₂, and sopfr! in succession. σ²(P₁) = P₂ links the first two perfect numbers.
