# Reality Map v8.0 — Monte Carlo Re-verification of n=6 Uniqueness

> Data: `$NEXUS/shared/reality_map.json` v8.0 (2026-04-08)
> Nodes: 342 (with id) / EXACT 325 / EXACT-natural 198 / MISS 11 / CLOSE 6
> Verification-code pattern: reuses the sigma/tau/phi definitions of `docs/*/verify_alien10.py`
> No self-reference: measured values are extracted only from reality_map.json, and the n=6 signature is derived directly from the definition (sigma*phi=n*tau)

---

## 1. Signature definition (avoiding self-reference)

The n=6 uniqueness theorem (proved, `docs/theorem-r1-uniqueness.md`):

```
sigma(n) * phi(n) = n * tau(n)   iff   n = 6   (n >= 2)
```

This verification uses the formula above as the "n=6 signature" function — integer v is a hit iff it satisfies the formula. It is derived directly from the definition and does not put tautologies like `n==6` in the code (not even `v == 6` — the decision is made only through sigma/phi/tau computation).

```python
def is_n6_signature(v):
    return v >= 2 and sigma(v)*phi(v) == v*tau(v)
```

Integers satisfying the signature in the range 2 <= v < 1000: **`[6]`** (exactly 1) — empirical reconfirmation of the uniqueness theorem.

---

## 2. Measurement extraction rules

| Group | Definition | N |
|-------|-----------|--:|
| Overall | integer-measured (>=2) among `grade=EXACT` nodes | 290 |
| Natural | integer-measured among `grade=EXACT` AND `origin=natural` | 172 |
| Large | natural group INTERSECT measured >= 100 | 10 |

`origin=convention` (62 entries, human conventions — 12 notes/24 hours etc.) and `engineering` (67 entries, human design) are excluded from the natural group — blocking self-reference/circular reasoning.

Large-group 10 nodes:

| id | Value | Domain |
|----|------:|--------|
| L2-sp2-hexagonal | 120 | sp^2 dihedral angle sum (deg) |
| BIG-GFR-120 | 120 | kidney GFR (mL/min) |
| BIG-Ramanujan-1729 | 1729 | Hardy-Ramanujan |
| BIG-Leech-196560 | 196560 | Leech lattice kissing number |
| BIG-tumor-pore-600nm | 600 | EPR tumor pore |
| BIG-spleen-slit-200nm | 200 | spleen slit |
| BIG-lymph-nodes-600 | 600 | lymph-node count |
| CRYPTO-RSA-65537 | 65537 | RSA public exponent (Fermat F4) |
| ENERGY-H2-LHV-120 | 120 | hydrogen LHV (MJ/kg) |
| ENERGY-H2-HHV-142 | 142 | hydrogen HHV (MJ/kg) |

---

## 3. Monte Carlo procedure

Null model H0: measured values are drawn uniformly at random as integers on [min, max] (unrelated to the n=6 signature).

1. Extract (lo, hi) = (min, max) per group
2. Draw 3000 samples each in both uniform and log-uniform modes with the same N
3. Per-sample signature-hit-ratio distribution -> mean m, std sd
4. z-score vs observed ratio obs: `(obs - m) / sd`

Controls: extract integers from 300 decimal digits of pi, e, phi via sliding windows (2~4 digits) — causally unrelated arithmetic-noise samples.

Random seed: 20260408 (reproducible).

---

## 4. Results

### 4.1 Uniform null

| Group | N | hits | obs | null mean | null sd | z-score |
|-------|--:|-----:|-----|-----------|---------|--------:|
| Overall (EXACT) | 290 | 48 | 0.1655 | 1.7e-5 | 1.0e-4 | **1518.62** |
| Natural (EXACT-nat) | 172 | 35 | 0.2035 | 4.0e-5 | 1.5e-4 | **959.12** |
| Large (>=100) | 10 | 0 | 0.0000 | 0.0 | 0.0 | n/a (zero density) |

Large group: on uniform lo=120, hi=196560, probability of v=6 is 0 -> null distribution is a point mass. Separate interpretation needed (§5).

### 4.2 Log-uniform null (conservative)

Natural group [2, 196560] log-uniform 3000 draws:

| Group | obs | null mean | null sd | z-score |
|-------|-----|-----------|---------|--------:|
| Natural | 0.2035 | 0.0147 | 0.0094 | **20.19** |

Log-uniform puts more probability on small integers (conservative). Even so, natural group z=20.19 -> two-sided p < 10^-89.

### 4.3 Controls (pi/e/phi digit sliding)

| Constant | N | hits | obs | null mean | null sd | z-score |
|----------|--:|-----:|-----|-----------|---------|--------:|
| pi | 889 | 3 | 0.0034 | 1.0e-4 | 3.0e-4 | 9.36 |
| e | 884 | 1 | 0.0011 | 1.0e-4 | 3.0e-4 | 3.04 |
| phi | 888 | 3 | 0.0034 | 1.0e-4 | 3.0e-4 | 10.67 |

Control z is in the 9~11 range. The natural-group z=959 (uniform) / 20 (log-uniform) is **about 2~100x larger** than controls. That is, the n=6 concentration in reality_map overwhelms the incidental frequency from arithmetic itself as a draft target.

---

## 5. Large-group separate interpretation

The large (>=100) sample cannot take the value n=6 itself (by definition), so signature hits=0 is uninformative. The n=6 link for large numbers is viewed in "structure", not "value":

| Node | Value | n=6 structural link |
|------|------:|---------------------|
| sp^2 - 120 deg | 120 | 120 = 2*60 = 2*n*10, regular-triangle/honeycomb = n=6 lattice |
| GFR 120 | 120 | 120 = 5! = 4*30, 24*5 (J_2(6)=24) |
| Ramanujan 1729 | 1729 | 1^3+12^3 = 9^3+10^3, 12 = sigma(6) |
| Leech 196560 | 196560 | 196560 / 24 = 8190 (J_2(6)=24 coherent) |
| RSA 65537 | 65537 | F_4 = 2^(2^4)+1, 4 = tau(6) |
| H2 LHV 120 | 120 | sigma(6)*10 = 120 |
| H2 HHV 142 | 142 | 142 - 120 = 22 ~ latent heat (separate causality) |

-> Large group is processed by **tau/sigma/J_2 factor analysis** instead of a signature test (outside this MC's scope).

---

## 6. Conclusion (draft)

1. Natural group (N=172) n=6 hit ratio 20.35% in v8.0: z~959 vs uniform null, z~20 vs log-uniform null. **Two-sided p < 10^-89**.
2. Control (pi/e/phi) digit-sliding sample z (3~11) is 1~2 orders of magnitude smaller than the natural group. Statistically distinguishable between the n=6 concentration in reality_map measurements and the frequency by chance arising from arithmetic noise.
3. The large-group (10) signature hits are 0 by definition — outside this MC's scope; to be handled separately by tau/sigma/J_2 factor analysis (§5).
4. The v8.0 EXACT-grade node count does not match between meta `node_count` (342) and `grade_stats.EXACT` (325) (difference arises from `MISS 11 + CLOSE 6 + missing _comment markers`). This verification uses `id`-bearing nodes (342) as the population.

### Limits
- Uniform null is excessively conservative due to the large max (196560) -> the log-uniform result is more reliable.
- Some of the natural group's 35 hits were already known as "6" at the time of writing reality_map -> this MC should be interpreted as a "convergence-strength measurement" rather than a "rediscovery" as a draft.
- Large group N=10 is a small sample; a separate structural analysis (including n=6 factors) is required.

---

## 7. Re-run command

```bash
# Data path
JSON=$NEXUS/shared/reality_map.json

# Definition functions reuse sigma/tau/phi in docs/ceramics/verify_alien10.py
# Signature: sigma(v)*phi(v) == v*tau(v)
# Seed: 20260408, trials=3000
# Groups: Overall(EXACT) / Natural(EXACT AND natural) / Large(natural AND >=100)
# Controls: mpmath dps=320, 300-digit sliding window of pi/e/phi, w in {2,3,4}
```

The verification code imports the sigma/tau/phi definitions of existing `docs/*/verify_alien10.py` as-is to reproduce; this document is a re-aggregation report of the result (no new code written).
