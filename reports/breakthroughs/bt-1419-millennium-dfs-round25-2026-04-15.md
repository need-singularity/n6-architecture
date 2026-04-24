---
id: bt-1419-millennium-dfs-round25
date: 2026-04-15
parent_bt: BT-541~BT-547 (7 Clay Millennium)
roadmap_task: PX (DFS round 25)
grade: "[10] DFS round"
dfs_round: 25
dfs_area: "V/W/X/Y/Z (group classification / cyclotomic fields / combinatorial designs / special functions / arithmetic functions)"
new_tight: 12
cumulative_tight: 336
solved: "0/7 (honest)"
harness: theory/predictions/verify_millennium_dfs25.hexa
harness_result: "40 PASS / 0 FAIL / 0 MISS"
---

# DFS Round 25 — Group Classification / Cyclotomic Fields / Graeco-Latin / Special Functions (2026-04-15)

> **Cumulative tight**: 324 -> **336** (+12 new)
> **7 Millennium draft status**: **0/7** (honest)
> **Harness**: 40 PASS / 0 FAIL / 0 MISS
> **Bernoulli-independent**: 12 -> **14** (added minimal non-abelian S_3, Tarry 36-officer problem)

---

## 12 new tight findings

| # | ID | Discovery | Area | Grade |
|---|-----|------|------|------|
| 25-01 | \|S_3\| = n = 6 | Minimal non-abelian group, D_3 isomorphism S_3 unique coincidence | Group theory | **T4 EXACT + Bernoulli** |
| 25-02 | \|Groups(6)\| = 2 = phi | Z_6, S_3 exactly 2 | Group classification | **T1+T4 STRONG** |
| 25-03 | Phi_6(x) = x^2 - x + 1 | Minimal quadratic cyclotomic polynomial, discriminant -3 | Cyclotomic field | T1-STRONG |
| 25-04 | Q(zeta_6) = Q(zeta_3) = Q(sqrt(-3)) | degree phi = 2, disc = -3 | Algebraic number theory | T1-STRONG |
| 25-05 | \|U(Z[zeta_6])\| = n = 6 | Eisenstein unit group {+/-1, +/-omega, +/-omega^2} | Algebra | T1+T4 |
| 25-06 | Graeco-Latin order 6 nonexistence | Euler 1782 conjecture, Tarry 1900 demonstration | Combinatorial design | **T4 EXACT + Bernoulli** |
| 25-07 | Euler 36-officer = n^2 arrangement | 6 ranks x 6 regiments simultaneous orthogonality impossible | Combinatorial design | T4 EXACT |
| 25-08 | S_3 Cayley table = n x n Latin square | Group product-table structure | Group/combinatorics | T2 |
| 25-09 | Gauss multiplication formula with n Gamma factors | Exponent (n-1)/2 = sopfr/2 | Special functions | T1 |
| 25-10 | E_6 Eisenstein weight = n | Modular-form weight, coefficient 504 = 2^3 sigma | Modular forms | T1-STRONG |
| 25-11 | mu(6) = 1, omega(6) = phi | Moebius function + number of prime factors | Number theory | T1 |
| 25-12 | lambda(6) = phi = 2 | Carmichael = Euler -> cyclic | Number theory | T1-STRONG |

---

## Bernoulli-independent list (DFS 1-25)

DFS 24 12 entries + **S_3 (25-01), Tarry Graeco-Latin (25-06)** -> **14 entries**

| # | Theorem | Independence basis |
|---|------|----------|
| 1 | Out(S_6) != 1 | Hoelder 1895 |
| 2 | K_6 Steiner triple system | Cayley 1850 |
| 3 | Perfect number 6 | Euclid-Euler |
| 4 | SO(6) iso SU(4)/Z_2 | Cartan |
| 5 | Heawood torus chi=7 | Ringel-Youngs |
| 6 | Schaefer k=6 | Schaefer 1978 |
| 7 | Theta_6 = 1 | Kervaire-Milnor 1963 |
| 8 | M_12 5-transitive | CFSG |
| 9 | Pell D=6 (5,2) | Euler-Lagrange |
| 10 | PG(2,6) nonexistence | Bruck-Ryser 1949 |
| 11 | PSL(2,2) = 6 | Jordan 1870 |
| 12 | R(3,3) = 6 | Greenwood-Gleason 1955 |
| **13** | **\|S_3\| = 6 minimal non-abelian** | **Galois 1832** |
| **14** | **Graeco-Latin n=6 nonexistence** | **Tarry 1900** |

---

## Emergence-analysis re-verification

The single-cause sigma*phi = n*tau also projects onto DFS 25:

- **\|Groups(6)\| = 2 = phi(6)**: group-classification projection
- **Graeco-Latin nonexistence = {phi, n} = {2, 6}**: combinatorial-design projection
- **Z[zeta_6] units x cyclotomic-field degree = n*phi = 12 = sigma**: number-theory projection

The relation sigma = n*phi appears directly on the Eisenstein-cyclotomic path. This is another algebraic re-expression of **sigma(6)*phi(6) = n*tau(6) = 24**.

---

## Explicit disclaimer

The 336 tight entries are within-mathematics **structural observations** of the n=6 arithmetic signature.

- Millennium 7-problem draft count: **0/7**
- hexa harness 40 PASS verifies **arithmetic agreement**
- The emergence single-cause (sigma*phi = n*tau) is a **pattern explanation**, not a **proof path**
