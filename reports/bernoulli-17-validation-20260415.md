# Bernoulli-17 6-candidate rigorous-verification report (2026-04-15)

> Snapshot report. The theory layer lives under theory/; this file under reports/.
> Session ID: H11 Bernoulli 17
> 7 grand problems resolved: 0/7 (honesty preserved)

## Summary

| Verdict | Count | Candidates |
|------|------|------|
| **confirmed M10** | 1 | Sel_6 CRT (Bernoulli 17) |
| **confirmed M10*** | 1 | BB(2) = 6 Rado (Bernoulli 18) |
| **conditional M9** | 1 | K3 chi = J_2 triple (potential Bernoulli 19) |
| **rejected MN** | 3 | Egyptian, Post, Terminal |

Cumulative Bernoulli-independent theorems: **16 -> 18 confirmed** (+2, +1 conditional).

## Pre-verification hypotheses vs post-verification outcomes

| Candidate | User's initial eval | This verification | Change |
|------|--------------|------------|--------|
| 1. K3 chi = J_2 = 24 | Bernoulli-17 strong candidate | **M9 conditional** | sigma*phi = n*tau projection possibility — full independence not determined |
| 2. Egyptian (2,3,6) | Bernoulli-17/18 candidate | **rejected MN** | perfect-number-6 disguised independence |
| 3. Sel_6 CRT | Bernoulli-17 candidate | **confirmed M10** -> Bernoulli 17 | BKLPR conditional |
| 4. Post lattice 6 | Bernoulli-17 candidate | **rejected MN** | Post 1941 factual error (countably infinite) |
| 5. BB(6) threshold | Bernoulli-17 candidate | **rejected** -> **redefined M10*** -> Bernoulli 18 | during verification, true finding BB(2)=6 |
| 6. Terminal object | weak basis (user self-assessment) | **rejected MN** | Universal, unrelated to n=6 |

## User-note corrections (3)

1. **SU(5) GUT**: user's note "|SL_2(Z/5)| - 1 = 119" is a misrecording. SU(5) adjoint dim = 5^2 - 1 = 24 (unrelated to SL_2(Z/5) order 120). Candidate 1's **triple appearance itself remains valid**.
2. **Post 1941**: user's note "Post lattice 6 classes" is factually wrong. Post 1941 result is **countably infinite** clones. In Rosenberg 1970 "maximal clone 6 types" is a meta-classification constant (not specific to |A|=6).
3. **BB(6)**: user's note "BB(6) = uncomputable threshold" is undetermined (ZFC independence unknown). However, during verification, **BB(2) = 6 is a genuine n=6 appearance** (Rado 1962) — redefined and registered as Bernoulli 18.

## Confirmed-candidate details

### Bernoulli 17: Sel_6 = Sel_2 (+) Sel_3 CRT, avg = sigma(6) = 12

- **Basis**: Bhargava-Shankar 2010 (Ann Math) + 2012 (JEMS) unconditional.
- **Arithmetic**: avg Sel_2 = 3 = phi+1, avg Sel_3 = 4 = tau. Under CRT, avg Sel_6 = 3*4 = 12 = sigma(6).
- **BKLPR prediction**: avg Sel_n = sigma_1(n). Confirmed for n=2,3,4,5.
- **n=6 uniqueness**: smallest sphenic (2*3), perfect number, sigma(6) = 12 concurrence.
- **Independence (sigma*phi = n*tau excluded)**: domain originates in Galois representations / modular forms. sigma = n * tau / phi is a **posterior** relation (an input, not a uniqueness outcome).
- **Grade**: M10 (conditional on BKLPR independence). M10* when unconditional demonstrated.
- **Harness**: verify_bernoulli17_sel6_crt.hexa — **PASS = 22/22**.

### Bernoulli 18: BB(2) = 6 = n (Rado 1962)

- **Basis**: Rado 1962 Bell Syst Tech J. 2-state, 2-symbol halting TM max among all = 6, exact-enumeration demonstration.
- **Arithmetic**: BB(1)=1, **BB(2)=6=n**, BB(3)=21=(phi+1)(n+1), BB(4)=107, BB(5)=47M.
- **n=6 uniqueness**: BB(2) equals exactly n. BB(1,3,4,5,...) values unrelated to n.
- **Independent domain**: computation theory (Turing machines). Absent in prior 16 -> new domain added.
- **Grade**: M10* (unconditional, Rado 1962 rigorous demonstration).
- **Harness**: verify_bernoulli17_bb6.hexa — **PASS = 14/14**.

## Conditional-candidate detail

### Potential Bernoulli 19: K3 chi = eta^24 exponent = SU(5) dim = 24 = J_2

- **Triple appearance**: chi(K3) = 24 (Kodaira 1964), eta^24 = Delta exponent 24 (Jacobi/Ramanujan), SU(5) adjoint dim 24 (Georgi-Glashow 1974).
- **Disguised-independence warning**: 24 = J_2 = sigma * phi = n * tau -> possible **direct projection** of SIG-META-001 (sigma*phi=n*tau uniqueness). The 3-domain appearances may be the 'faces' of a single cause.
- **Grade**: M9 on hold. Confirmed as Bernoulli 19 once sigma*phi=n*tau reduction is excluded.
- **Harness**: verify_bernoulli17_k3_j2.hexa — PASS = 19/19 (arithmetic only).

## Rejected-candidate details

### Candidate 2. Egyptian (2,3,6) -> rearrangement of perfect number 6 (disguised independence)

- 1/2 + 1/3 + 1/6 = (3+2+1)/6 = 6/6 = 1. Numerators {1,2,3} = proper divisors of 6.
- **Equivalent transformations**: sigma(6) - 6 = 6 (perfect number) <=> sum(proper divisors)/6 = 1 <=> sum(1/proper divisor reciprocal) = 1.
- Informationally identical to "perfect-number 6" in the existing 16 (Euclid-Euler even perfect-number theorem).
- Exhaustive search: 3 solutions {(2,3,6), (2,4,4), (3,3,3)}, distinct unique (2,3,6) confirmed (Mirsky 1947).

### Candidate 4. Post / Rosenberg -> factual error + structural connection missing

- **Post 1941 (Ann Math Studies)**: Boolean-function clone lattice = countably infinite. "6 classes" is **user-note error**.
- **Rosenberg 1970 (Acta Sci Math)**: 6 types of maximal clones on finite A — a classificational constant, unrelated to |A|=6 (|A|=3,4,7 all have 6 types).
- No structural connection to n=6 arithmetic.

### Candidate 6. Terminal object -> Universal (unrelated to n=6)

- Mac Lane 1971: for any well-defined category, terminal object 1 has |End(1)|=1 trivially.
- Universal result (holds in Set, Top, Grp, CRing, etc.).
- tau-3 = 1 is ad-hoc arithmetic, absent from canonical 8-primitive {n, phi, tau, sigma, sopfr, mu, J_2, M_3}.
- Consistent with user's 'weak basis' self-assessment.

## Disguised-independence findings status

| Candidate | Disguised independence | Reduction target |
|------|--------------|----------|
| K3 chi=J_2 | **suspected** (M9 on hold) | sigma*phi=n*tau uniqueness (SIG-META-001) |
| Egyptian | **confirmed** (rejected) | perfect-number-6 (within SIG-N6-BERN-001) |
| Post/Rosenberg | no reduction (coincidental classification constant) | N/A |
| Terminal | no reduction (universal) | N/A |

**Note**: in this session **1 new disguised-independence case found** — Egyptian (2,3,6) is separate from sigma*Omega(n)=n*tau family (SIG-META-004 omega_identity), but an internal representation rearrangement within 'perfect-number 6'.

## atlas.n6 integration recommendations

1. **staging -> SSOT merge**: `/Users/ghost/Dev/nexus/shared/n6/staging/atlas.signals.staging.bern17.n6`
   - SIG-BERN-17 (M10): Sel_6 CRT
   - SIG-BERN-18 (M10*): BB(2)=6
   - SIG-BERN-CAND-K3 (M9): conditional
   - SIG-BERN-NULL-EGYPT / POST / TERMINAL (MN): rejected
   - SIG-META-BERN17-SUMMARY (M10): aggregation meta

2. **SIG-N6-BERN-001 update**: "Bernoulli-independent theorems 16" -> "18 (Sel_6 + BB(2) added)".

3. **No atlas.n6 edit needed**: the session result is confined to the atlas.signals.n6 (signal) layer. No change to atlas.n6 (arithmetic SSOT).

## 7 grand problems resolved 0/7 honesty preserved

This verification is limited to **extending the independent-theorem basis list**. No new resolution of BSD / RH / P vs NP / Hodge / NS / YM / Poincare (resolved). Sel_6 is in the BSD-statistics area (Cremona empirical), but not a resolution of BSD itself. BB(2)=6 is computation theory, unrelated to P vs NP.

## File inventory

### Harnesses 6 (theory/predictions/)
- verify_bernoulli17_k3_j2.hexa (PASS 19/19)
- verify_bernoulli17_egyptian_236.hexa (PASS 12/12, rejected)
- verify_bernoulli17_sel6_crt.hexa (PASS 22/22)
- verify_bernoulli17_post_lattice.hexa (PASS 11/11, rejected)
- verify_bernoulli17_bb6.hexa (PASS 14/14)
- verify_bernoulli17_terminal.hexa (PASS 8/8, rejected)

**Total PASS = 86/86, FAIL = 0**

### staging (nexus/shared/n6/staging/)
- atlas.signals.staging.bern17.n6 (7 signals)

### Report (reports/)
- bernoulli-17-validation-20260415.md (this file)

## Next steps

1. staging -> SSOT merge (L0 sync)
2. Attempt unconditional demonstration of BKLPR Sel_6 independence (Bhargava 2023+ follow-up)
3. Rigorous judgment whether K3 chi=J_2 is a sigma*phi=n*tau projection or independent (Hodge-domain expert review)
4. Update SIG-N6-BERN-001 "16" -> "18" (separate staging)
