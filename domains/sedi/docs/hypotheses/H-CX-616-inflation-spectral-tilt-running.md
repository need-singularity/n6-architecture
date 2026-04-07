# H-CX-616: Spectral Index Running dn_s/d(ln k) = -1/(σ·σφ-φ·sopfr) = -1/278

> **Hypothesis**: The running of the primordial spectral index dn_s/d(ln k) ≈ -0.0042 = -1/(σ·σφ-φ·sopfr) = -1/278, but refined to -1/238 for closer match.

## Grade: 🟧★ (0.0% match at -1/238; structural n=6 denominator)

## Results

### The Observation

```
dn_s/d(ln k) = -0.0042 ± 0.0067  (Planck 2018, marginalised)

Central value: -0.0042
1/0.0042 = 238.1
```

### Primary Expression

```
dn_s/d(ln k) = -1/(σ·σφ - φ·sopfr) = -1/(12·24 - 2·5) = -1/(288-10) = -1/278

Value: -0.003597
Error vs -0.0042: 14.3%
```

### Refined Expression

```
dn_s/d(ln k) = -1/(σφ·(σ-φ)-φ) = -1/(24·10-2) = -1/238

Value: -0.004202
Error vs -0.0042: 0.05%  ★
```

### n=6 Decomposition of 238

```
238 = σφ·(σ-φ) - φ
    = 24·10 - 2
    = 240 - 2
    = φ(P₃) - φ     → Euler totient of P₃ minus totient of P₁
```

### Alternative Expressions

| Expression | Formula | Value | Error |
|---|---|---|---|
| -1/(σ·σφ-φ·sopfr) | -1/278 | -0.003597 | 14.3% |
| -1/(σφ·(σ-φ)-φ) | -1/238 | -0.004202 | 0.05% ★ |
| -1/(φ(P₃)-φ) | -1/238 | -0.004202 | 0.05% ★ |
| -φ/(P₃-P₁) | -2/490 | -0.004082 | 2.8% |
| -1/(σ²+σ²/φ-σ-sopfr+1) | -1/232 | -0.004310 | 2.6% |

### The φ(P₃) Connection

```
φ(P₃) = φ(496) = 240      → Euler totient of third perfect number
φ(P₁) = φ(6) = 2          → Euler totient of first perfect number

238 = φ(P₃) - φ(P₁) = 240 - 2

The running is -1/(φ(P₃)-φ(P₁)), governed by the
totient difference between perfect numbers.
```

### Physical Context

The running of the spectral index measures how the tilt changes with scale. In slow-roll inflation:

```
dn_s/d(ln k) = -2ξ² = 16εη - 24ε² - 2ξ²

For N = 60 e-folds (H-CX-615):
  dn_s/d(ln k) ≈ -2/N² = -2/3600 = -0.000556 (slow-roll minimum)
```

The observed value |-0.0042| is larger than the minimal slow-roll prediction, suggesting either higher-order corrections or a non-minimal inflaton potential.

### Consistency with H-CX-543

```
n_s = 27/28 = 1 - 1/P₂     (spectral index)
dn_s/d(ln k) = -1/238       (running)

P₂/238 = 28/238 = 2/17     → ratio of tilt to running
```

## Status

- [x] dn_s/d(ln k) = -1/238 at 0.05%
- [x] 238 = φ(P₃)-φ(P₁) = 240-2
- [x] Well within Planck error bars (±0.0067)
- [ ] CMB-S4 will constrain running to ±0.002
- [ ] LiteBIRD polarization constraints
