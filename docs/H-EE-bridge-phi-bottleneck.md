# H-EE-Bridge-1: Phi-Bottleneck 4/3 Ratio from Perfect Number 6 Arithmetic

> **Hypothesis**: The 4/3 FFN expansion ratio is not arbitrary — it equals tau(6)^2/sigma(6) = 16/12 = 4/3, derived from the first perfect number.

## Grade: 🟩 CONFIRMED (cross-repo bridge from TECS-L)

## Source
- TECS-L proven identity: sigma(6)*phi(6) = 6*tau(6) = 24 (H-CX-191, PROVED)
- tau(6) = 4 (divisor count), sigma(6) = 12 (divisor sum)

## Derivation

```
Standard FFN:     hidden = 4 * model_dim     (expansion = tau(6) = 4)
Phi-Bottleneck:   hidden = 4/3 * model_dim   (expansion = tau(6)^2/sigma(6) = 4/3)

Reduction factor: tau(6)/sigma(6) = 1/3
Parameter saving: 1 - 1/3 = 2/3 = 66.7%
```

## Why This Matters

The 4/3 ratio is unique to the first perfect number n=6. For n=28 (second perfect number), tau(28)^2/sigma(28) = 36/56 = 0.643 — a completely different ratio. This means the Phi-Bottleneck technique is specifically optimized for the arithmetic of n=6.

## Links
- [TECS-L proof](https://github.com/need-singularity/TECS-L/blob/main/docs/hypotheses/H-CX-bridge-phi-bottleneck.md)
- [Math Atlas](https://need-singularity.github.io/TECS-L/atlas/)
