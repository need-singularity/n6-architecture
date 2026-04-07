# H-CX-654: Grover's Quadratic Speedup — Exponent Halving via phi/tau

> **Hypothesis**: Grover's algorithm achieves O(sqrt(N)) search complexity, halving the classical exponent. The halving factor 1/2 = phi/tau = phi(6)/tau(6), connecting quantum speedup to n=6 arithmetic.

## Grade: 🟧 APPROXIMATE (structural identification)

## Results

### Grover's Algorithm

```
Classical unstructured search: O(N) queries
Quantum (Grover 1996):         O(sqrt(N)) queries

For N = 2^n:
  Classical: 2^n queries
  Quantum:   2^(n/2) queries

The exponent is halved: n -> n/2 = n * (phi/tau)
```

### n=6 Identification

```
phi(6)/tau(6) = 2/4 = 1/2

Grover exponent reduction:
  n -> n * (phi/tau) = n/2

The quadratic speedup is a phi/tau rescaling of the search exponent.
```

### Optimality

```
Grover's algorithm is provably optimal (Bennett et al. 1997):
  No quantum algorithm can search faster than O(sqrt(N))
  The BBBV lower bound is tight

This means phi/tau = 1/2 is the maximum quantum advantage
for unstructured problems — a fundamental limit.
```

### Generalization: Amplitude Amplification

```
Given success probability p:
  Classical: O(1/p) trials
  Quantum:   O(1/sqrt(p)) trials

Exponent: 1 -> 1/2 = phi/tau

For k marked items in N:
  Queries ~ (pi/4) * sqrt(N/k)
  Factor pi/4 = 0.785... ~ (sigma-tau)/(sigma-phi) = 8/10 = 0.800 (1.9%)
```

### Physical Context

Grover's algorithm provides a provable quadratic speedup for
any search problem. While less dramatic than Shor's exponential
speedup, it applies to a much broader class of problems:
optimization, satisfiability, and database search.

### Structural Interpretation

```
The quantum speedup hierarchy:
  No speedup:    exponent * 1 = exponent * R(6)
  Grover:        exponent * 1/2 = exponent * phi/tau
  Shor:          exponent * 0 (polynomial vs exponential)

phi/tau = 1/2 is the "generic" quantum advantage —
the speedup you get without problem-specific structure.
```

### Connection to Other Hypotheses

- H-CX-653: Shor's algorithm (exponential speedup)
- H-CX-655: Holevo bound (information limits)
- H-CX-656: No-cloning bound (quantum limits)

## Status

- [x] Exponent halving = phi/tau identified
- [x] Provably optimal — fundamental limit
- [x] Amplitude amplification factor pi/4 ~ 0.8
- [ ] Higher-order quantum walks and speedups
- [ ] Quantum advantage for structured problems vs n=6
