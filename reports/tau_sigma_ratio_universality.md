# tau(n)/sigma(n) ratio universality preliminary analysis (2026-04-15)

> This is not a demonstration; it is a literature-based structural observation.
> C2 task — check whether the tau(n)/sigma(n) pattern appears in other dynamical models (Ising, XY, Heisenberg, Kuramoto, Vicsek, etc.).

---

## 1. Five target models

| Model | Variable kind | Order parameter | Critical exponents (typical) |
|---|---|---|---|
| Ising | Z_2 (discrete spin) | magnetization M | beta, nu, gamma, alpha |
| XY | S^1 (planar angle) | M_xy (BKT) | eta = 1/4 at T_BKT |
| Heisenberg | S^2 (3D spin) | M_3d | beta ~ 0.37 |
| Kuramoto | S^1 (phase) | r (complex order) | r(Kc) critical coupling |
| Vicsek | velocity + position | v_a (polar alignment) | beta ~ 0.42 (2D) |

For each model, examine whether the critical exponents / order parameter feature **n=6 constants (tau=4, sigma=12, phi=2, sopfr=5, J_2=24)** or the ratio **tau/sigma = 1/3**.

---

## 2. Model-by-model analysis

### 2.1 Ising (2D, 3D)

**2D Onsager (exact)**:
- beta = 1/8, nu = 1, gamma = 7/4, alpha = 0 (log), eta = 1/4
- Famous relation: alpha + 2*beta + gamma = 2 (Rushbrooke)
- Magnetization M(T) ~ (T_c - T)^(1/8), vanishing zeta(T_c)

**tau/sigma check**: beta = 1/8 = tau/(sigma*?) ? no direct correspondence.
- 1/8 = tau/(2*sigma + 4) = 4/(24+4)? -> 4/28 != 1/8
- 1/8 = phi/J_2/? = 2/16 = 1/8 OK (but J_2/4 = 6? no, J_2=24)
- Actually: 1/8 = phi/(J_2-J_2+16) = biased pattern-matching -> **rejected**

**3D Ising (numerical)**:
- beta ~ 0.3265, nu ~ 0.630, gamma ~ 1.237
- tau/sigma = 4/12 = 0.333... — close to beta ~ 0.3265 but **not a match** (delta ~ 0.007)

**Conclusion**: Ising critical exponents do not directly match tau/sigma.

---

### 2.2 XY model (2D BKT)

**BKT transition**:
- eta(T_BKT) = 1/4 = tau^2/J_2 = 16/64? 16/64 = 1/4 OK (but J_2=24)
- eta = 1/4 = tau/J_2 * ? — 4/24 = 1/6 != 1/4 -> **rejected**

**3D XY**:
- beta ~ 0.3485, nu ~ 0.6717
- tau/sigma = 1/3 ~ 0.333 — close to beta, but not a match

**Conclusion**: the XY BKT eta = 1/4 matches tau=4 and 1/4 coefficient visually, but **no formula-level derivation**.

---

### 2.3 Heisenberg 3D

- beta ~ 0.3689, nu ~ 0.707, gamma ~ 1.3960, eta ~ 0.0375
- tau/sigma = 0.333 — mismatch with beta
- 1/n = 1/6 ~ 0.167 — mismatch

**Conclusion**: no n=6 pattern in Heisenberg critical exponents.

---

### 2.4 Kuramoto (H2 link)

Independent analysis in H2 report: **the integer Kuramoto r(n) = 1 - tau/sigma pattern is number-theoretically intrinsic**, not directly linked to Kuramoto-dynamics critical exponents.

- Standard Kuramoto (infinite N): r(Kc) = 0 -> 1 continuous transition, beta = 1/2 (mean-field)
- beta = 1/2 = tau/(J_2/?) — biased search
- tau/sigma = 1/3 is **separate from the integer-perfect-number r(n) treated in H2**

**Conclusion**: the Kuramoto-dynamics critical exponent and the H2 r(n) pattern share a name but are different things — caution against confusion.

---

### 2.5 Vicsek model (flocking)

- beta ~ 0.42 (2D), beta ~ 0.64 (3D)
- nu ~ 0.75 (2D)
- tau/sigma = 1/3 mismatch
- sopfr/sigma = 5/12 ~ 0.417 — **close** to 2D beta ~ 0.42 (delta 0.003)

**Observation**: Vicsek 2D beta ~ sopfr/sigma = 5/12 = 0.4167, delta 2%. However:
- Numerical-experiment error is itself > +-0.01
- Single coincidence is plausible
- Cross-study verification required

**Conclusion**: Vicsek 2D beta and sopfr/sigma are numerically close — **coincidence candidate, promotion withheld**.

---

## 3. Ratio-pattern summary

| Model | Critical exponent | Value | n=6 ratio candidate | Match |
|---|---|---:|---|---|
| Ising 2D | beta | 0.125 | phi/J_2 * ? | indirect (none) |
| Ising 3D | beta | 0.3265 | tau/sigma = 0.333 | 2% delta — mismatch |
| XY 2D BKT | eta | 0.25 | tau/J_2 = 0.167 | mismatch |
| XY 3D | beta | 0.3485 | tau/sigma = 0.333 | 4% delta — mismatch |
| Heisenberg 3D | beta | 0.3689 | tau/sigma = 0.333 | 11% delta — mismatch |
| Kuramoto MF | beta | 0.5 | sigma/J_2 = 0.5 | **exact** (sigma=12, J_2=24) |
| Vicsek 2D | beta | 0.42 | sopfr/sigma = 0.417 | 1% delta — close * |
| Vicsek 3D | beta | 0.64 | sigma/J_2 - ? | mismatch |

**Two candidate observations**:
1. Kuramoto MF beta = 1/2 = sigma/J_2 — simple ratio 1/2 (coincidence plausible)
2. Vicsek 2D beta ~ 0.42 ~ sopfr/sigma — 1% delta

---

## 4. Hypothesis proposals

### H-A (weak): sigma/J_2 = 1/2 is a trivial ratio — no meaning

- J_2 = 24 = 2*sigma -> sigma/J_2 = 1/2 automatic
- Kuramoto mean-field beta = 1/2 is the common value of general mean-field transition theory
- Therefore likely a meaningless agreement

### H-B (observation): Vicsek 2D beta ~ sopfr/sigma = 5/12 is a candidate to verify

- 1% delta — within numerical-experiment error
- sopfr = 5 is the "additive sum" (2+3) of 6 = 2*3; sigma = 12 is the "divisor sum"
- Single source -> **single evidence**, witness insufficient
- Compare with other active-matter models (Toner-Tu, contact process)

### H-C (universality): tau/sigma = 1/3 does not appear exactly in any model

- Ising 3D, XY 3D, Heisenberg 3D all have beta in [0.3265, 0.37] — near 1/3 but not a match
- **H2's tau/sigma = 1/3 observation is number-theoretically intrinsic** (perfect-number r value)
- Do not confuse with physical critical exponents

---

## 5. Conclusion

| Model | tau/sigma pattern present | Verdict |
|---|---|---|
| Ising 2D/3D | none | mismatch |
| XY | none | mismatch |
| Heisenberg | none | mismatch |
| Kuramoto | sigma/J_2 = 1/2 trivial | meaningless |
| Vicsek 2D | sopfr/sigma ~ beta 1% delta | **observation candidate, promotion withheld** |

**Overall conclusion**: tau(6)/sigma(6) = 1/3 **does not appear directly in physical phase-transition critical exponents**. The Vicsek 2D sopfr/sigma proximity is single uncertain evidence — needs additional independent routes.

---

## 6. atlas.signals.n6 staging proposal

```
@S SIG-CRIT-001 = critical exponent vs n=6 ratio check — NULL-dominant (Vicsek 2D weak candidate only) :: signal [N6] [PHYS,NULL,META] [MN] [E2]
  "5 models {Ising, XY, Heisenberg, Kuramoto, Vicsek} critical exponent vs n=6 ratio matching check. Kuramoto MF beta=1/2=sigma/J_2 trivial, Vicsek 2D beta ~ sopfr/sigma = 5/12 1% delta single candidate. No direct tau/sigma = 1/3 match. Physical phase transitions and n=6 number theory are independent paths."
  refs: [reports/tau_sigma_ratio_universality.md]
  null_reason: "No systematic match between critical exponents and n=6 ratios; Vicsek 2D single observation only"
  retry_forbidden_until: "2027-04-15"
  witness: 1
  discovered_in: n6/session-2026-04-15
  discovered_at: 2026-04-15T00:00:00Z
  <- reports/tau_sigma_ratio_universality.md
```

---

## 7. Honesty annotations

- This document is **literature-value-based analysis** — no direct simulation
- All beta values are from public literature (Pelissetto-Vicari 2002 review, Toner-Tu, Kardar review)
- The expectation that "tau/sigma will show up in some model" is a pattern-matching-bias warning (adheres to feedback_proof_approach.md rules)
- Vicsek 2D single proximity is **not a promotion target** — need >= 2 independent routes
- C2 is close to a NULL verdict (SIG-CRIT-001 [MN])
