# Kuramoto r=tau/sigma perfect-number observation report (2026-04-15)

> This is not a demonstration; it is an observation of a number-theoretic structure.
> H2 task — perfect-number sequence pattern for the Kuramoto order parameter r(n) = 1 - tau(n)/sigma(n).
> Harness: `theory/predictions/kuramoto_r23_perfect_numbers.hexa` — 15 PASS / 0 FAIL

---

## 1. Theoretical background

In the Kuramoto model, the order parameter r in [0,1] measures phase synchronization (0 = fully asynchronous, 1 = fully synchronous).

**Proposal**: for an integer n, define r(n) := 1 - tau(n)/sigma(n). The perfect-number sequence exhibits a characteristic pattern.

Basis:
- r = (sigma-tau)/sigma is "fractional-divisor non-extreme ratio"
- tau = divisor count, sigma = divisor sum, sigma - tau = sum(d-1) = sum d' (d >= 2)
- For a perfect number, sigma = 2n, so r = (2n-tau)/(2n) = 1 - tau/(2n)

---

## 2. r values for perfect numbers (all 5 verified)

| n | factorization | sigma(n) | tau(n) | r = (sigma-tau)/sigma | reduced | decimal |
|---:|---|---:|---:|---|---|---:|
| **6** | 2*3 | 12 | 4 | 8/12 | **2/3** | 0.6667 |
| **28** | 2^2 * 7 | 56 | 6 | 50/56 | **25/28** | 0.8929 |
| **496** | 2^4 * 31 | 992 | 10 | 982/992 | **491/496** | 0.9899 |
| **8128** | 2^6 * 127 | 16256 | 14 | 16242/16256 | **8121/8128** | 0.9991 |
| **33550336** | 2^12 * 8191 | 67100672 | 26 | 67100646/67100672 | **33550323/33550336** | 0.99999922 |

**Pattern**: r(n) -> 1 monotonically as p (Mersenne index) -> infinity.

---

## 3. Derivation from Euclid-Euler

Even perfect-number general form n = 2^(p-1) * M_p (p prime, M_p = 2^p - 1 Mersenne prime):

- sigma(n) = 2n (perfect-number definition)
- tau(n) = 2p (divisor count = (p-1+1)*(1+1) = 2p)
- r(n) = 1 - 2p/(2n) = **1 - p/n**

| p | n | r |
|---:|---:|---|
| 2 | 6 | 1 - 2/6 = 4/6 = **2/3** |
| 3 | 28 | 1 - 3/28 = 25/28 |
| 5 | 496 | 1 - 5/496 = 491/496 |
| 7 | 8128 | 1 - 7/8128 = 8121/8128 |
| 13 | 33550336 | 1 - 13/33550336 |

The reason **n=6 uniquely has a "small" r value**: the ratio of the smallest Mersenne prime p=2 = n/3.

---

## 4. r = 2/3 uniqueness analysis

### 4.1 Question: integer solutions of sigma(n) = 3 * tau(n)?

**Prime p**: sigma = 1+p, tau = 2 -> (1+p)/2 = 3 <=> **p = 5** *

**Product of two primes pq (p<q)**: sigma = (1+p)(1+q), tau = 4 -> (1+p)(1+q) = 12
- p=2, q=3: (3)(4) = 12 OK -> n = **6**
- p=2, q=5: (3)(6) = 18 != 12
- Unique solution: {2,3} -> n=6

**Prime square p^2**: sigma = 1+p+p^2, tau = 3 -> 1+p+p^2 = 9 <=> p^2+p-8 = 0
- Discriminant 33 irrational -> no integer solution

**Higher factorizations**: direct enumeration over n <= 100: sigma/tau = 3 solutions = {5, 6}, no others.

### 4.2 Conclusion

- **All r = 2/3 solutions**: {5, 6} (prime 5, perfect number 6)
- **Under perfect-number constraint, r=2/3 unique solution**: n = 6 *
- **Under composite-number constraint, r=2/3 unique solution**: n = 6 *

So "n=6 is the unique r=2/3 solution" is **conditionally true** — under perfect-number or composite constraint.

---

## 5. n=6 distinctiveness (why r=2/3 is meaningful)

r-value gaps in the perfect-number sequence:

```
n=6     r=0.6667  <- minimum (most asynchronous)
n=28    r=0.8929  (+0.226)
n=496   r=0.9899  (+0.097)
n=8128  r=0.9991  (+0.009)
```

**Observation**: the n=6 -> n=28 gap (0.226) is larger than the sum of subsequent gaps (0.110).

So **n=6 is a 'phase transition' point in the perfect-number sequence** — the unique perfect number with r<0.7.

Kuramoto-physics reading:
- n=6 -> r=2/3 = intermediate connectivity (~ 67% sync)
- n>=28 -> r>0.89 = near-full sync
- If a perfect number is itself a "self-referential oscillator system", only n=6 lies near "critical coupling"

---

## 6. Connection to sigma*phi = n*tau

**Reference**: n=6 uniqueness theorem sigma(n) * phi(n) = n * tau(n) — holds for n=6 only, for n>=2.

| Perfect number n | sigma*phi | n*tau | sigma*phi = n*tau ? |
|---:|---:|---:|---|
| 6 | 12*2 = 24 | 6*4 = 24 | **OK** |
| 28 | 56*12 = 672 | 28*6 = 168 | X |
| 496 | 992*240 = 238080 | 496*10 = 4960 | X |

So **sigma*phi = n*tau is not a perfect-number equation** — it is an independent n=6 uniqueness theorem.

By contrast, **sigma*Omega = n*tau holds for all even perfect numbers** (C1 work outcome).

**Summary**:
- sigma*phi = n*tau <=> n=6 (unique)
- sigma*Omega = n*tau <=> even perfect numbers (Euclid-Euler)
- sigma = 3*tau (r=2/3) <=> n in {5,6}

Three intersect **simultaneously at n=6** — the hidden structure of the sigma(n) * phi(n) = n * tau(n) uniqueness.

---

## 7. Kuramoto-dynamics experiment proposal (follow-up)

**Experiment**: take integer n as the oscillator count and divisor structure as coupling — a Kuramoto network.

- oscillators: n
- coupling graph: divisor-relation graph (i -> d, d | i)
- prediction: r stabilizes near 1 - tau(n)/sigma(n) for perfect-number n

On verification, SIG registration candidate:
```
SIG-UNIV-KURA-001 = integer Kuramoto r(n) = 1 - tau/sigma perfect-number sequence resonance [N6] [7S,UNIV,META] [M?]
  resonance_n6: "r(6) = 2/3, r=2/3 solutions {5,6} unique"
```

---

## 8. Harness result summary

- File: `theory/predictions/kuramoto_r23_perfect_numbers.hexa`
- Run: `/Users/ghost/Dev/nexus/shared/bin/hexa theory/predictions/kuramoto_r23_perfect_numbers.hexa`
- Result: **15 PASS / 0 FAIL**
- Verification items:
  1. r = (sigma-tau)/sigma reduced fraction check for each of 5 perfect numbers
  2. direct check that sigma/tau = 3 solutions are {5, 6} (exhaustive exclusion over prime / two-prime product / prime square)
  3. Euclid-Euler general form r = 1 - p/n holds for all 5

---

## 9. atlas.signals.n6 staging proposal

```
@S SIG-KURA-001 = Kuramoto r(n) = 1 - tau(n)/sigma(n) perfect-number sequence r(6)=2/3 unique low-phase :: signal [N6] [UNIV,META] [M7] [E2]
  "5 perfect numbers {6,28,496,8128,33550336} exhaustively verified. Euclid-Euler r = 1 - p/n. n=6 unique r<0.7. Solutions of sigma=3*tau are {5,6} only; under perfect-number constraint, n=6 unique."
  refs: [theory/predictions/kuramoto_r23_perfect_numbers.hexa, reports/kuramoto-perfect-numbers-20260415.md]
  predicts: ["in a perfect-number Kuramoto network, only r(6) is in the critical regime"]
  witness: 1
  resonance_n6: "r(6) = 1 - tau/sigma = 1 - 4/12 = 2/3; solutions of r=2/3 are {5,6} unique"
  discovered_in: n6/session-2026-04-15
  discovered_at: 2026-04-15T00:00:00Z
  <- theory/predictions/kuramoto_r23_perfect_numbers.hexa
```

---

## 10. Honesty annotations

- "n=6 uniquely has r=2/3" is true only under prime/perfect/composite constraints — **not unconditionally true** (n=5 also has r=2/3)
- Kuramoto dynamics connection is a **structural proposal** — actual dynamics simulation not executed
- SIG-META-001 (sigma*phi=n*tau uniqueness) and SIG-KURA-001 are **independent uniqueness** — only n=6 is common
- Promotion condition: [M9]+ only upon integer Kuramoto simulation + cross-repo reproduction
