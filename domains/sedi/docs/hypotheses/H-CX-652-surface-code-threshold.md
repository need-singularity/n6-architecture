# H-CX-652: Surface Code Error Threshold — p_th ~ 1.1%

> **Hypothesis**: The surface code error threshold p_th ~ 1.1% resists clean n=6 decomposition, placing quantum error correction in a "dark domain" of the R(n)=1 framework.

## Grade: 🟧 APPROXIMATE (best match ~10% error)

## Results

### Surface Code Threshold

```
Topological surface code (Kitaev / Raussendorf / Fowler):
  Error threshold: p_th ~ 1.1% (phenomenological noise)
  Under circuit-level noise: p_th ~ 0.6-0.7%

This is the maximum physical error rate below which
logical error rate can be made arbitrarily small.
```

### n=6 Approximation Attempts

| Expression | Formula | Value | Error vs 1.1% |
|---|---|---|---|
| 1/(sigma^2-sigma*sopfr+M3) | 1/(144-60+7) | 1/91=1.099% | 0.1% |
| 1/(sigma^2-sopfr^2+phi) | 1/(144-25+2) | 1/121=0.826% | 25% |
| 1/(sigma*(sigma-tau)+sopfr) | 1/(96+5) | 1/101=0.990% | 10% |
| phi/(sigma*sopfr*sigma/tau) | 2/180 | 1.11% | 1% |

### Denominator Analysis

```
Best candidate: sigma^2 - sigma*sopfr + M3 = 144 - 60 + 7 = 91

But 91 = 7 * 13 is not a clean n=6 product.

Note: phi/(sigma*sopfr*sigma/tau) = 2/180 = 1/90
  1/90 = 1.111% matches well, and
  90 = sigma*sopfr*sigma/(tau*phi) = cleaner form
  Actually 90 = sigma*M3 + P1 = 84+6 = 90
  Or 90 = P1*sopfr*sigma/tau = 6*5*3 = 90 ✓
```

### Refined Expression

```
p_th ~ 1/(P1*sopfr*sigma/tau) = 1/(6*5*3) = 1/90 = 1.111%

Target:  1.1%
Result:  1.111%
Error:   ~1%

This is acceptable but not compelling as a structural prediction.
```

### Physical Context

The surface code is the leading candidate for fault-tolerant
quantum computing. Its threshold is determined by the competition
between error accumulation and syndrome-based correction on
a 2D lattice of physical qubits.

The ~1% threshold is a practical target for quantum hardware:
superconducting qubits now achieve ~0.1-0.5% error rates,
approaching the regime where surface codes become effective.

### Connection to Other Hypotheses

- H-CX-651: Topological entanglement entropy (same code family)
- H-CX-653: Shor's algorithm (quantum computing context)
- H-CX-655: Holevo bound (quantum information theory)

## Status

- [x] Best approximation: 1/90 = 1.111% (~1% off)
- [x] Multiple expressions tested
- [x] Threshold is model-dependent (noise model matters)
- [ ] Circuit-level threshold 0.67% vs n=6
- [ ] Concatenated code thresholds for comparison
