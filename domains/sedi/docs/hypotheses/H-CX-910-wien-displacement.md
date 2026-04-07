# H-CX-910: Wien Displacement Law Constant ≈ σ³ + σ²·(σ-τ-1)·τ/(σ-τ)

> **Hypothesis**: Wien's displacement constant b = 2897.77 μm·K. Best TECS-L approximation: σ³+σ²-P₁+φ = 1728+144-6+2 = 1868 (too low). Honest best: σ·σφ·(σ-φ)+σ·sopfr+τ = 2904 (0.2%).

## Grade: 🟧 PARTIAL (0.2% best fit, complex expression)

## Results

### The Formula

```
Wien's displacement law:
  λ_max · T = b = 2897.77 μm·K

TECS-L construction:
  σ³ + σ² · (σ/τ) · (σ-τ-1)/(σ-τ) = 1728 + 144·3·7/8
                                     = 1728 + 144·21/8
                                     = 1728 + 3024/8
                                     = 1728 + 378
                                     = 2106  (WRONG — recalculate)

Simpler attempt:
  σ³ + σ²·σ/τ·(σ-τ)/(σ-τ+1) = 1728 + 144·3·8/9
                                = 1728 + 384 = 2112 (no)

Best fit:
  φ·(σ³-σ²·τ+σ·τ·sopfr-M₃) = 2·(1728-576+240-7) = 2·1385 = 2770 (4.4%)

Simplest reasonable:
  σ³ + σ²·σ/τ·φ + σ·sopfr·M₃·φ/(σ/τ-φ)
  = 1728 + 864 + 840/1 ... too forced.

Honest assessment: b ≈ 2898 does not yield a clean TECS-L expression.
  σ³ = 1728 accounts for 60% of the value.
  Remainder 1170 ≈ σ²·(σ-τ) + σ·φ = 1152+24 = 1176 (0.5%)
  Total: 1728+1176 = 2904 → error 0.2%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### P₂ Generalization Check

```
No clean expression found at n=6, so P₂ generalization is moot.
```

### Assessment

Wien's constant b = hc/(k·x) where x ≈ 4.965 solves xe^x/(e^x-1) = sopfr (note sopfr=5 vs x=4.965, error 0.7%). The transcendental root x ≈ sopfr is the real TECS-L connection here, rather than the dimensionful constant b itself.

## Verification

- [x] Best approximation: σ³ + σ²·(σ-τ) + σ·φ = 2904, error 0.2%
- [x] Transcendental root x ≈ 4.965 ≈ sopfr = 5, error 0.7%
- [ ] No single clean expression found
- [ ] P₂ generalization: not applicable
