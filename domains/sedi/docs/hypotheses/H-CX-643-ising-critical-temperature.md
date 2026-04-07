# H-CX-643: 2D Ising Critical Temperature — Onsager Exact Solution

> **Hypothesis**: The 2D Ising model critical temperature T_c = 2J/ln(1+sqrt(2)) = 2.2692J can be approximated by n=6 arithmetic as sigma*sopfr/(sigma*phi+sopfr-phi) = 60/27 = 2.222J (2.1% error).

## Grade: 🟧 APPROXIMATE (2.1% match)

## Results

### Exact Value (Onsager 1944)

```
T_c / J = 2 / ln(1 + sqrt(2)) = 2 / 0.88137... = 2.26919...

Onsager's analytic solution of the 2D square-lattice Ising model
gives the exact critical temperature for the order-disorder transition.
```

### n=6 Expression

```
sigma*sopfr / (sigma*phi + sopfr - phi)
= 12*5 / (12*2 + 5 - 2)
= 60 / 27
= 2.2222...

Target:   2.2692
Result:   2.2222
Error:    2.1%
```

### Alternative Expressions Tested

| Expression | Formula | Value | Error |
|---|---|---|---|
| sigma*sopfr/(sigma*phi+sopfr-phi) | 60/27 | 2.222 | 2.1% |
| sigma/(sopfr+phi/tau) | 12/5.5 | 2.182 | 3.8% |
| (sigma-tau)/(sigma/tau+phi/sigma) | 8/3.167 | 2.526 | 11% |
| phi*M3/(P1+phi/tau) | 14/6.5 | 2.154 | 5.1% |

### Physical Context

The 2D Ising model is the simplest statistical-mechanics model exhibiting
a phase transition. Onsager's 1944 exact solution was a landmark result,
providing the only analytically solvable model in d >= 2.

The critical temperature marks the transition between:
- T < T_c: ordered ferromagnetic phase (spontaneous magnetization)
- T > T_c: disordered paramagnetic phase

### Structural Interpretation

```
60 = sigma * sopfr    -> divisor-sum times prime complexity
27 = (sigma/tau)^3    -> cube of the generation count
T_c ~ 60/27           -> complexity / generations^3
```

The ln(1+sqrt(2)) denominator resists clean n=6 decomposition, placing
this constant in a "dark domain" where irrational structure dominates.

## Status

- [x] Best n=6 approximation at 2.1%
- [x] Multiple alternatives tested, none sub-1%
- [ ] Investigate if ln(1+sqrt(2)) has deeper n=6 decomposition
- [ ] Compare 3D Ising critical temperature (see H-CX-645)
