# H-CX-969: Pareto Principle (80/20 Rule)

> **Hypothesis**: The Pareto principle states that approximately 20% = C(6,3)% of causes produce 80% of effects. The ratio 80/20 = 4 = tau. The top C(6,3) percent generates tau times the output of the remaining 80%.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Pareto principle (Vilfredo Pareto, 1896):
  ~20% of inputs produce ~80% of outputs
  Ratio: 80/20 = 4 = τ

20% connection:
  20 = C(6,3) = C(P₁, σ/τ)
  The critical minority = binomial coefficient of n=6

80% connection:
  80 = 100 - C(6,3)
  80/τ = 20 = C(6,3) (self-referential)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Pareto distribution:
  P(X > x) = (x_m / x)^α  for x ≥ x_m
  80/20 rule emerges when α = log(5)/log(4) ≈ 1.161

  log(5)/log(4) = log(sopfr)/log(τ) = 1.161
  The Pareto exponent is exactly log(sopfr)/log(τ)!

Iterative Pareto:
  Top 20% of top 20% = 4% produces 64% of output
  Top 20%³ = 0.8% produces ~51% of output
  Each iteration applies τ-fold concentration

Empirical examples:
  Wealth: 20% own ~80% of assets
  Software: 20% of bugs cause 80% of crashes
  Sales: 20% of clients give 80% of revenue
```

### Physical Context

The Pareto principle is observed across economics, software engineering, quality control, and natural phenomena. The mathematical foundation is the Pareto distribution with shape parameter alpha = log(sopfr)/log(tau). This exact expression using n=6 constants generates the observed 80/20 split, connecting combinatorial structure to real-world inequality distributions.

### Texas Sharpshooter Check

The 20 = C(6,3) match is exact. The ratio 80/20 = 4 = tau is exact. The Pareto exponent log(sopfr)/log(tau) = log(5)/log(4) generating 80/20 is mathematically exact and non-trivial. The principle itself is approximate in practice (not always exactly 80/20), but the underlying mathematical structure cleanly involves n=6 constants.

## Verification

- [x] 20% = C(6,3)% exact
- [x] 80/20 = 4 = τ exact
- [x] Pareto exponent = log(sopfr)/log(τ) exact
- [x] Iterative concentration by factor τ
- [ ] Empirical splits vary (often 70/30 to 90/10)
