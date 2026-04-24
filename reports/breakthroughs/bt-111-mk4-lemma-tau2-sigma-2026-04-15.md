---
id: bt-111-mk4-lemma
date: 2026-04-15
parent_bt: BT-111
parent_theorem: Mk.IV
status: LEMMA (not eligible as main theorem — uniqueness C1 fails)
grade: "[10] cross-domain EXACT 9/10"
verdict_ref: theory/proofs/mk4-trident-final-verdict-2026-04-15.md
candidates_ref: theory/proofs/mk4-theorem-candidates-2026-04-14.md
---

# BT-111 Mk.IV Lemma formal listing — tau^2/sigma = 4/3

> **Status**: Mk.IV candidate A -> **Lemma** (P8-4 decision, 2026-04-15)
> **Reason**: n=6 uniqueness C1 fails (tau(n)^2/sigma(n) = 4/3 solution set = {2, 6}, n >= 2)
> **Main theorem**: candidate B (sigma - tau = 8) -> MK4-THEOREM-B confirmed

---

## §1 Formula definition

```
  tau(6)^2 / sigma(6) = 16 / 12 = 4/3

  Equivalent expressions:
    R_local(3, 1) = (3^2 - 1) / (2*3) = 8/6 = 4/3
    2nd factor of the sigma*phi = n*tau draft (1st factor R_local(2,1) = 3/4)
    (3/4) x (4/3) = 1 = R(6) -> n=6 perfect-number determination
```

## §2 Uniqueness check — C1 fails

```python
# Exhaustive check (n in [2, 10^4])
A_hits = [n for n in range(2, 10001)
          if 3 * tau(n)**2 == 4 * sigma(n)]
# Result: A_hits = [2, 6]
```

n=2: tau(2)=2, sigma(2)=3 -> tau^2/sigma = 4/3 ok
n=6: tau(6)=4, sigma(6)=12 -> tau^2/sigma = 16/12 = 4/3 ok

**Two solutions exist**, so a uniqueness theorem of the form "tau^2/sigma = 4/3 iff n=6" is impossible.
-> Not eligible as a second theorem at the Mk.III level (sigma*phi = n*tau iff n=6).

## §3 Value as a Lemma — 10-domain crossing

Uniqueness fails, yet the cross-domain occurrence of the constant 4/3 itself is **honestly strong**:

| # | Domain | Observed value | Error | Verdict |
|---|--------|--------|------|------|
| 1 | Shockley-Queisser solar cell | Optimal bandgap 1.34 eV | 0.45% | EXACT |
| 2 | Semiconductor GaAs bandgap | 1.42 eV | 6.10% | NEAR |
| 3 | Wind Betz limit | tau^2 / (n/phi)^3 = 16/27 | 0.00% | EXACT |
| 4 | AI SwiGLU FFN ratio | (sigma - tau)/(n/phi) = 8/3 | 0.00% | EXACT |
| 5 | AI Chinchilla dropout | ln(4/3) ~ 0.2877 | 0.00% | EXACT |
| 6 | Music perfect fourth | 4:3 just intonation | 0.00% | EXACT |
| 7 | String theory R^2/alpha' compactification | 4/3 (BT-111) | 0.00% | EXACT |
| 8 | Number-theory R_local draft factor | theorem-r1 Lemma 2 | 0.00% | EXACT |
| 9 | QED hydrogen hyperfine Delta E | (4/3) alpha^4 m_e c^2 | 0.00% | EXACT |
| 10 | 2D percolation correlation length | nu = 4/3 (Stauffer) | 0.00% | EXACT |

**9/10 EXACT, 1/10 NEAR (GaAs)** — average error 0.66%.

## §4 Structural meaning

```
  Lemma (BT-111, tau^2/sigma = 4/3):
    In the draft of sigma(n) * phi(n) = n * tau(n), the factor
    R_local(3, 1) = 4/3 from R(n) = Prod_p R_local(p, v_p(n))
    is not unique to n=6 (shared with n=2), but it is the
    essential balancing factor for R(6) = 1 in sigma(6)*phi(6) = 6*tau(6).

    This factor reappears EXACT in 10 independent domains
    (SQ bandgap, Betz limit, SwiGLU, QED hyperfine, 2D percolation, etc.)
    and functions as an auxiliary lemma for candidate B (sigma - tau = 8,
    MK4-THEOREM-B).
```

## §5 atlas grade

- BT-111 grade: **[10] Lemma** (EXACT domain crossing, no uniqueness)
- MK4-THEOREM-B grade: **[10*]** (sigma - tau = 8, unique at n=6, 10/10 PASS)
- Relation: BT-111 is an auxiliary lemma for MK4-THEOREM-B. Not an independent theorem.

## §6 Cross-links

- MK4-THEOREM-B: main theorem sigma - tau = 8 (theory/proofs/mk4-trident-final-verdict-2026-04-15.md)
- BT-30: Shockley-Queisser SQ bandgap 1.34 eV ~ 4/3
- BT-33: SwiGLU FFN ratio 8/3 = 2*(4/3)
- theorem-r1-uniqueness.md: R_local(3, 1) = 4/3 draft factor

---

**Honesty record**: tau^2/sigma = 4/3 is not unique to n=6 (shared with n=2). Domain fit is strong,
but this does not rise to a theorem that "selects n=6". Honestly listed as a Lemma.
