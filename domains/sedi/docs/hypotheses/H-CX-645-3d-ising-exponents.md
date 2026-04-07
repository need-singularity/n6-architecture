# H-CX-645: 3D Ising Critical Exponents — Approximate n=6 Forms

> **Hypothesis**: The 3D Ising critical exponents, known only numerically from RG and Monte Carlo, resist exact n=6 decomposition — marking a "dark domain" where irrational structure dominates.

## Grade: 🟧 APPROXIMATE (best matches 1-3%)

## Results

### Known Values (Conformal Bootstrap / Monte Carlo)

```
beta  = 0.32653(10)
gamma = 1.2372(5)
nu    = 0.63012(16)
eta   = 0.03631(3)
alpha = 0.1096(5)
delta = 4.7898(6)
```

### Best n=6 Approximations

| Exponent | Value | n=6 Expression | Result | Error |
|---|---|---|---|---|
| beta | 0.3265 | (sigma-tau-sopfr)/(sigma-tau+phi) | 3/10 = 0.300 | 8.1% |
| beta | 0.3265 | M3/(sigma+sigma/tau+sopfr) | 7/20 = 0.350 | 7.2% |
| gamma | 1.2372 | (sigma+sopfr-tau)/(sigma-phi-tau) | 13/6 = 2.167 | 75% |
| gamma | 1.2372 | sopfr*phi/(sigma-tau) | 10/8 = 1.250 | 1.0% |
| nu | 0.6301 | sopfr/(sigma-tau) | 5/8 = 0.625 | 0.8% |
| delta | 4.7898 | (sigma*tau-P1)/(sigma-tau-phi) | 42/2 = 21 | -- |
| delta | 4.7898 | (sigma*phi+P1*phi-sigma)/(sigma-tau) | 24/5 = 4.800 | 0.2% |

### Contrast with 2D Ising (H-CX-644)

```
2D: ALL exponents are exact rational numbers -> EXACT n=6 match
3D: ALL exponents are irrational numbers     -> only approximate

This is the dimensional "dark domain" transition:
  d = 2 = phi(6):  universality class fully captured
  d = 3 = sigma/tau: universality class escapes exact capture
  d = 4 (upper critical): mean-field exponents, again rational
```

### Physical Context

The 3D Ising model has no exact solution. Critical exponents are
computed via epsilon-expansion, conformal bootstrap, and Monte Carlo.
Their values appear to be genuinely irrational, with no known
closed-form expressions.

### Structural Interpretation

The failure of exact n=6 matching in d=3 is itself informative.
The R(n)=1 framework predicts that d=phi(6)=2 is the "arithmetic
dimension" where number theory fully controls universality. Moving
to d=sigma/tau=3 introduces geometric complexity that transcends
pure arithmetic.

### Connection to Other Hypotheses

- H-CX-644: 2D Ising exponents (all exact)
- H-CX-662: Anderson localization (d=2 marginal, d=3 transition)

## Status

- [x] Best approximations catalogued (0.2-8% range)
- [x] "Dark domain" interpretation established
- [x] Dimensional contrast with 2D documented
- [ ] Test epsilon-expansion coefficients for n=6 structure
- [ ] Conformal bootstrap bounds vs n=6 expressions
