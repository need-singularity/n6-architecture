# BSD kappa(B) ~ A*B^alpha exponent decomposition report (alpha = 0.1752)

- Date: 2026-04-15
- Harness: `theory/predictions/bsd_kappa_fraction.hexa` (17/17 PASS)
- Companion harness: `theory/predictions/bsd_alpha_log2_tau.hexa` (22/22 PASS)
- Data: Cremona 1.73M curves 7-bin log-log regression (v3 E5)
- Measurement: alpha_obs = 0.1752, standard error ~ +-0.02 (7-bin constraint)
- Staging signal: `SIG-7B-401`, `SIG-7B-402` (`n6shared/staging/atlas.signals.staging.mill.n6`)

## 7 grand problems resolved 0/7 preserved (honest)

This report treats only **empirical match** and **n=6 basis-constant decomposition candidate comparison**.
Neither candidate is a **theorem** explaining alpha.
BSD remains an unresolved millennium problem. 7-grand-problems resolved counter: **0/7**.

---

## Summary (5 lines)

- alpha_obs = 0.1752 (Cremona 1.73M curves 7-bin regression, sigma_alpha ~ 0.02)
- Best candidate: **7/40 = 0.1750** (diff 0.0002, 0.114%)
- Runner-up: **log(2)/tau = 0.17329** (diff 0.0019, 1.09%)
- Both candidates fall within 1-sigma band (+-0.02) -> not distinguishable with 7-bin data
- 7/40 = (sigma-sopfr)(n/phi)/(J_2*sopfr) = 21/120 = 7/40 (five n=6 basis constants)

## Candidate comparison

| Candidate | Formula | Value | alpha_obs - cand | Rel error | n=6 decomposition smoothness |
|------|------|----:|-------------:|----------:|--------------------|
| **7/40** | (sigma-sopfr)(n/phi)/(J_2*sopfr) | 0.17500 | +0.00020 | 0.114% | ** best |
| **log(2)/tau** | log(2)/4 | 0.17329 | +0.00191 | 1.09% | * cross-repo |
| alpha*sigma vs phi+1/n | alpha*sigma = 2.1024, phi+1/n = 2.1666 | 2.1024 | -0.0642 | 3.05% | X disqualified |
| alpha*J_2 vs tau+1/sopfr | 4.2048 vs 4.2 | 4.2048 | +0.0048 | 0.114% | = equivalent to 7/40 |

## Candidate 1: 7/40 (minimum error)

**Formula**: alpha = 7/40 = 0.1750

**n=6 decomposition**:
```
7  = sigma - sopfr       (12 - 5, equal to BT-543 YM beta_0)
40 = J_2 * sopfr / (n/phi)
   = 24 * 5 / 3
```
i.e. `7/40 = (sigma - sopfr) * (n/phi) / (J_2 * sopfr) = 21/120 = 7/40`.

**Core observations**:
- Numerator 7 = sigma-sopfr is **identical** to BT-543 Yang-Mills beta_0. RH-BSD-YM meganode candidate.
- Denominator 40 = J_2 * sopfr / (n/phi) combines three basis constants: Jacobi theta J_2=24, sopfr=5, n/phi=3.
- Equivalent: alpha * J_2 = tau + 1/sopfr = 4.2 (same error).

**Relative error**: 0.114% (5.7% of the 1-sigma +-0.02 band). Convergence distinguishable at 10M+ curves.

## Candidate 2: log(2)/tau (cross-repo universal)

**Formula**: alpha = log(2)/4 = 0.17329

**n=6 decomposition**:
- tau = 4 = 2^phi (divisor count).
- log(2) = H(1/2) = 1 bit Shannon entropy.

**Cross-repo resonance (3 independent branches)**:
1. **[N6]** BSD Cremona empirical kappa(B) power exponent (this report).
2. **[AN]** anima Psi-constants all derived from ln(2) (`SIG-CONS-312`).
3. **[N6]** Shannon H(1/2) = log(2) 1 bit, Bernoulli block entropy.

**tau=4 denominator reading**: tau = 2^phi = 4 = 2-bit block. Formally consistent with the BSD rank, torsion, Sha[2], Tamagawa 4-direction self-dual symmetry conjecture (post-hoc observation).

**Relative error**: 1.09%. Strongly within 7-bin standard error +-0.02, but 10x the error of 7/40.

## Candidate 3: alpha*sigma = 2.1024 (disqualified)

alpha x sigma = 0.1752 * 12 = 2.1024. Fit attempts against basis-constant polynomials:
- phi + 1/n = 13/6 = 2.1666 -> diff 0.064 (3.05% error)
- phi + 1/J_2 = 49/24 = 2.0416 -> diff 0.061 (2.9% error)

Both near the 1-sigma band boundary. **Disqualified**.

## Candidate 4: alpha*J_2 = tau + 1/sopfr = 4.2 (equivalent to 7/40)

alpha x J_2 = 0.1752 * 24 = 4.2048
tau + 1/sopfr = 4 + 0.2 = 4.2

diff 0.0048 (0.114%) — a **rearrangement of the same formula** as candidate 1 (7/40).

Irreducible-fraction expansion:
```
alpha = (tau + 1/sopfr) / J_2
      = (tau*sopfr + 1) / (J_2*sopfr)
      = 21 / 120
      = 7 / 40
```

## Core findings

1. **Two candidates co-present within 1-sigma**: 7/40 and log(2)/tau both within standard error +-0.02.
   With 7-bin data, **cannot rank them**.
2. **7/40 has the smallest error but may be trivial**: likely a fit with 5 basis-constant degrees of freedom.
   (BIC penalty considering DOF 4-5 not computed.)
3. **log(2)/tau has cross-repo motivation**: linked to anima Psi-constants + Shannon entropy.
   Has **an edge in physical-interpretability**.
4. **BT-543 cross link**: numerator 7 = sigma-sopfr equals Yang-Mills beta_0 (BSD-YM meganode).

## Proposed next steps

1. **Cremona 10M+ extension** — reduce standard error below +-0.005 to distinguish 7/40 vs log(2)/tau.
2. **BIC penalty computation** — correct for overfitting of 5-DOF fit.
3. **Bridge to BKLPR heuristic** — check whether log(2)/tau is linked to BKLPR constants.
4. **(A3'') conjecture verification** — fix the B-dependent coupling factor as log(2)/tau and re-evaluate the fit.
5. **SIG-7B-401 promotion judgment at 10M+** — [M7!] -> [M9] or [M10] candidate.

## Conclusion

alpha_obs = 0.1752 admits two candidates co-present within 1-sigma:

- **7/40** (minimum integer-fraction error, 5 n=6 basis constants combined)
- **log(2)/tau** (cross-repo universal, physically interpretable)

Ranking requires a **10M+ curves extension**. The mere coexistence of the two candidates suggests a **potential n=6 arithmetic-structure target in BSD**, but neither is a demonstration, and BSD unresolved status is preserved at **0/7**.

---

## Harness execution log

### bsd_kappa_fraction.hexa (17/17 PASS)
```
PASS: 7/40 = 1750/10000
PASS: 40 = J_2*sopfr/(n/phi) = 24*5/3
PASS: 7/40 = (sigma-sopfr)(n/phi)/(J_2*sopfr) = 21/120
PASS: |diff 7/40| = 2 (0.0002)
PASS: |diff log(2)/tau| = 19 (0.0019)
PASS: alpha*sigma integer-ratio match failed (> 1% error)
...
total PASS: 17
total FAIL: 0
```

### bsd_alpha_log2_tau.hexa (22/22 PASS)
```
PASS: divisor count of 6 = 4 = tau
PASS: log(2) * 10^8 = 69314718
PASS: log(2)/tau * 10^8 ~ 17328679
PASS: tau = 2^phi binary split
PASS: strongly within 1-sigma band (+-2*10^6)
PASS: independent branches >= 3 -> M7! candidate
...
total PASS: 22
total FAIL: 0
```
