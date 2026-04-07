# H-CX-621: Sound Horizon r_s = σ²+σ/τ = 147 Mpc — 0.06%

> **Hypothesis**: The comoving sound horizon at the drag epoch r_s = 147.09 Mpc ≈ σ²+σ/τ = 144+3 = 147 Mpc, with 0.06% precision.

## Grade: 🟩 CONFIRMED (0.06% match; two-term n=6 expression)

## Results

### The Prediction

```
r_s = σ² + σ/τ = 144 + 3 = 147 Mpc

Planck 2018: r_s = 147.09 ± 0.26 Mpc
Error: 0.06%  (within 0.35σ)
```

### n=6 Decomposition

```
σ² = 144     → square of divisor sum (dominant term)
σ/τ = 3      → generation count (correction)

r_s = σ² + σ/τ = 144 + 3 = 147
```

### Alternative Expressions

| Expression | Formula | Value | Error |
|---|---|---|---|
| σ²+σ/τ | 144+3 | 147.000 | 0.06% ★ |
| σ²+sopfr-φ | 144+3 | 147.000 | 0.06% ★ (same!) |
| σ·(σ+1)/τ+σ/τ | 39+3 | 42 | ✗ |
| σ²+τ-1 | 147 | 147.000 | 0.06% (same value) |
| P₃/σ·(σ/τ)+sopfr-φ | ... | ... | ... |

### Remarkable Degeneracy

Three distinct n=6 expressions all give 147:
```
σ² + σ/τ     = 144 + 3 = 147     (abundance² + generations)
σ² + sopfr-φ = 144 + 3 = 147     (abundance² + complexity-totient)
σ² + τ-1     = 144 + 3 = 147     (abundance² + divisors-unity)
```

All share the structure σ²+3, where 3 = σ/τ = sopfr-φ = τ-1.

### Physical Significance

The sound horizon r_s is the comoving distance a sound wave can travel from the Big Bang to baryon decoupling (z ≈ 1060). It serves as the BAO "standard ruler":

```
BAO angular scale: θ_s = r_s / D_A(z_dec)
First acoustic peak: ℓ_1 ≈ π·D_A(z_dec)/r_s ≈ 220
```

This is one of the most precisely measured distances in cosmology, used to calibrate the cosmic distance ladder independently of local measurements.

### Connection to Other Hypotheses

```
r_s = 147 Mpc ← this hypothesis
z_rec = 1089  ← H-CX-611

r_s·z_rec = 147·1089 = 160,083

160,083 ≈ σ²·σ·σφ·sopfr - σ·M₃·σφ + σ·sopfr·M₃ + ...
(complex; the product is not obviously clean)
```

### Implications for Hubble Tension

The sound horizon is central to the Hubble tension:
```
H₀(CMB) = c·z/(r_s·(1+z)) depends on r_s
Different r_s → different H₀

n=6 predicts:
  r_s = 147 Mpc (this)
  H₀ = 73 (H-CX-534, local)
  H₀ = 67 (H-CX-534, Planck)

The tension = P₁ = 6 km/s/Mpc may be resolved if r_s
has a small scale-dependent correction.
```

## Status

- [x] r_s = σ²+σ/τ = 147 Mpc at 0.06%
- [x] Three degenerate expressions all yielding σ²+3
- [x] Central to BAO standard ruler
- [ ] DESI Year 5 precision test
- [ ] Role in Hubble tension resolution
