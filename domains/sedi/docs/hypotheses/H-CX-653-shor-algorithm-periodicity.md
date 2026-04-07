# H-CX-653: Shor's Algorithm — Qubit Count and phi^sigma

> **Hypothesis**: Shor's algorithm for factoring N requires 2*ceil(log2(N)) qubits, where the factor 2 = phi(6). For RSA-2048, this gives 4096 = 2^12 = phi^sigma qubits.

## Grade: 🟧★ NOTABLE (structural + numerical coincidence)

## Results

### Shor's Algorithm Resource Requirements

```
To factor an n-bit integer N:
  - Quantum register: 2n qubits (for period-finding)
  - Factor 2 = phi(6) — quantum parallelism overhead
  - Classical post-processing: continued fractions

For RSA-2048 (n = 2048 bits):
  Qubits = 2 * 2048 = 4096 = 2^12 = phi^sigma
```

### n=6 Coincidence

```
4096 = 2^12 = phi(6)^sigma(6)

sigma(6) = 12 = number of bits in the exponent
phi(6) = 2 = base of binary representation

RSA-2048 qubit count = phi^sigma

This is a genuine numerical coincidence:
  2048 = 2^11 (chosen for security)
  2 * 2048 = 2^12 = phi^sigma
```

### Deeper Structure

```
Period-finding requires phi = 2 register copies:
  |x⟩|f(x)⟩ -> entangle input with output

The quantum Fourier transform acts on phi*n qubits
to extract the period r of f(x) = a^x mod N.

Exponent sigma = 12 appears because:
  RSA key size 2048 = 2^(sigma-1)
  So qubits = phi * 2^(sigma-1) = 2^sigma = phi^sigma
```

### Algorithmic Complexity

```
Shor's algorithm:
  Quantum: O((log N)^2 * log(log N)) gates
  Classical: O((log N)^3) for continued fractions

vs classical factoring:
  Number field sieve: exp(O(n^(1/3) * (log n)^(2/3)))
  Exponent 1/3 = tau/(sigma-tau+tau) = tau/sigma? No, 1/3 = tau/sigma.
```

### Physical Context

Shor's algorithm (1994) is the primary motivation for
large-scale quantum computing. Breaking RSA-2048 requires
thousands of logical qubits, each built from thousands of
physical qubits (via error correction, see H-CX-652).

### Connection to Other Hypotheses

- H-CX-652: Surface code threshold (error correction for Shor)
- H-CX-654: Grover's algorithm (quadratic vs exponential)
- H-CX-648: Quantum conductance (phi factor in transport)

## Status

- [x] Qubit overhead factor = phi(6) = 2
- [x] RSA-2048 qubits = phi^sigma coincidence noted
- [x] RSA key size 2048 = 2^(sigma-1) observed
- [ ] Actual physical qubit count with error correction
- [ ] Post-quantum cryptography parameters vs n=6
