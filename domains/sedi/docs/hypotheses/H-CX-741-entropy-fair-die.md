# H-CX-741: Entropy of Fair Die -- ln(P_1) as Information Unit

> **Hypothesis**: The entropy of a fair six-sided die is H(X) = ln(6) = ln(P_1) nats = log_2(6) = log_2(P_1) bits ~ 2.585 bits. The fair die is the maximum-entropy device for P_1 outcomes.

## Grade: 🟧 SPECULATIVE

## Results

### The Formula

```
Fair die: X ~ Uniform({1, 2, 3, 4, 5, 6})
  p_i = 1/6 = 1/P_1  for all i

Shannon entropy:
  H(X) = -sum p_i * log(p_i) = log(6)

In nats:  H = ln(6) = ln(P_1) = 1.7918 nats
In bits:  H = log_2(6) = log_2(P_1) = 2.5850 bits
```

### n=6 Prediction

```
P_1 = 6

H(fair die) = ln(P_1) nats

In bits:
  log_2(P_1) = log_2(6) = log_2(2) + log_2(3) = 1 + log_2(3)
             = 1 + 1.585 = 2.585

The die encodes 1 bit (even/odd = Z/phi) + 1.585 bits (which third = Z/(sigma/tau))
```

### Verification

```
Predicted:  H = ln(P_1) = 1.7918 nats
Observed:   H = ln(6) = 1.7918 nats (information theory)
Error:      0.00%
p-value:    ~0.30 (tautological for a 6-sided die)
```

### Texas Sharpshooter Check

The entropy of a fair N-sided die is always ln(N). Choosing N = 6 = P_1 is simply noting that the die has 6 faces. The content is limited unless we ask: why is the standard die 6-sided? Historical reasons (cubic symmetry, oldest known dice). The structural reading: a die embodies the P_1 = 6 structure, and its entropy decomposes via the Chinese Remainder Theorem.

### Information Decomposition

```
Z/6Z = Z/2Z x Z/3Z  (Chinese Remainder Theorem)

H(X mod 2) = log_2(2) = 1 bit         (even/odd, Z/phi)
H(X mod 3) = log_2(3) = 1.585 bits    (residue mod sigma/tau)

Total: H(X) = H(X mod 2) + H(X mod 3) = 2.585 bits
  (independent because gcd(2,3) = 1)

Mutual information: I(X mod 2; X mod 3) = 0
  Perfect independence of the phi and sigma/tau channels.
```

### Comparison with Other Dice

```
d4  (tetrahedron):  H = log_2(4)  = 2.000 bits = log_2(tau)
d6  (cube):         H = log_2(6)  = 2.585 bits = log_2(P_1)
d8  (octahedron):   H = log_2(8)  = 3.000 bits = log_2(sigma-tau)
d12 (dodecahedron): H = log_2(12) = 3.585 bits = log_2(sigma)
d20 (icosahedron):  H = log_2(20) = 4.322 bits

Platonic solid dice entropies in TECS-L:
  d4 = tau, d6 = P_1, d8 = sigma-tau, d12 = sigma
```

### P_2=28 Generalization

```
H(fair d28) = ln(28) = ln(P_2) = 3.332 nats = 4.807 bits

ln(P_2) = ln(4) + ln(7) = ln(tau) + ln(M_3)
         = 2*ln(phi) + ln(M_3)

Decomposition: Z/28Z = Z/4Z x Z/7Z
  H = log_2(4) + log_2(7) = 2 + 2.807 = 4.807 bits

P_2 generalization: EXTENDS (entropy decomposes via CRT for any P_k)
```

## Verification

- [x] H(fair die) = ln(6) is standard
- [x] CRT decomposition Z/6Z = Z/2Z x Z/3Z gives information split
- [ ] Primary claim is tautological

## Status

New. The fair die entropy ln(P_1) decomposes via CRT into independent phi-channel (1 bit) and sigma/tau-channel (1.585 bits). The Platonic solid dice series (d4, d6, d8, d12) maps to (tau, P_1, sigma-tau, sigma).
