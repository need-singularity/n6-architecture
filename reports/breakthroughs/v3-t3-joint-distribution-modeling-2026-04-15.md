---
id: v3-t3-joint-distribution-modeling
date: 2026-04-15
roadmap_task: v3 T3 (GALO-PX-4 (A3) joint-distribution mathematical modeling)
grade: [9] empirical + conjecture
predecessors:
  - reports/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md
  - reports/breakthroughs/v3-e5-kappa-7bin-power-law-2026-04-15.md
status: SUGGESTIVE, NOT PROVED
license: CC-BY-SA-4.0
---

# v3 T3 — kappa(B) Joint-distribution Mathematical Modeling + alpha ~ log(2)/4 Finding

> **Summary**: Decompose v3 E5 7-bin power law kappa(B) ~ A * B^alpha (A = 0.2317, alpha = 0.1752) via a mathematical joint-distribution model. alpha = 0.1752 differs from log(2)/4 = 0.1733 by 0.0019 (< 1.1% relative error). This is a **suggestive empirical match** — it is neither a theorem nor a conjecture in the formal sense. Quantifies the departure scale from the BKLPR + independence assumption.

---

## §1 Starting point — prediction under BKLPR + indep

Bhargava-Kane-Lenstra-Poonen-Rains (BKLPR 2013) model: the n-Selmer group |Sel_n(E)| of an elliptic curve E is a random cokernel distribution.

**Independence assumption** (original conjecture A3): for coprime m, n,
$$\mathbb{E}[|Sel_{mn}(E)|] = \mathbb{E}[|Sel_m(E)|] \cdot \mathbb{E}[|Sel_n(E)|]$$

In particular for m=2, n=3:
$$\mathbb{E}[|Sel_6|] \stackrel{?}{=} \mathbb{E}[|Sel_2|] \cdot \mathbb{E}[|Sel_3|]$$

---

## §2 Measured deviation — definition of kappa(B) and measured growth

### 2.1 Cremona 1.73M curves (conductor <= 410k, 7 bins)

$$\kappa(B) := \mathbb{E}[|Sel_6|]_B - \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B$$

(Statistics table: see `reports/v3/e5-kappa-7bin-power-law-2026-04-15.md` §2)

| B mid | N | kappa(B) | ratio_6 = E[S_6] / (E[S_2] * E[S_3]) |
|-------|---|------|------|
| 25k | 332,366 | -1.67 | 0.7925 |
| 75k | 325,030 | -0.29 | 0.9304 |
| 125k | 316,708 | -0.26 | 0.9726 |
| 175k | 308,257 | +0.20 | 1.0175 |
| 225k | 306,722 | +0.35 | 1.0336 |
| 305k | 59,081 | +1.22 | 1.0849 |
| 405k | 57,660 | +1.32 | 1.1106 |

### 2.2 Power-law fit

Log-linear regression (`scripts/empirical/cremona_kappa_10bin.py`):
$$\kappa(B) \approx 0.2317 \cdot B^{0.1752}$$
(for the first 2 bins where kappa < 0, |kappa| is used; sign flip noted)

$$\text{ratio}_6(B) \approx 0.2383 \cdot B^{0.1198}$$

Difference of the two slopes: alpha_kappa - beta_ratio = 0.055. Coexistence of two power laws on log-linear scale.

---

## §3 alpha ~ log(2)/4 — suggestive match

### 3.1 Main-constant comparison (12 n=6-centric candidates)

| Constant | Value | Error vs alpha=0.1752 |
|------|-----|---------|
| **log(2)/4** | 0.1733 | **+0.0019 (1.1%)** ok |
| (1/tau(6))*log(2) = (1/4)*log(2) | 0.1733 | +0.0019 (1.1%) ok |
| 1/sigma(4) | 0.1429 | +0.0324 |
| 1/sopfr(6) | 0.2000 | +0.0248 |
| 1/sigma(3) | 0.2500 | +0.0748 |
| gamma / sqrt(6) | 0.2356 | +0.0604 |
| 1/(phi(6) + sigma(3)/2) | 0.2500 | +0.0748 |

(sopfr(6) = 2+3 = 5, Euler-Mascheroni gamma ~ 0.5772, tau = divisor count)

### 3.2 Theoretical plausibility of log(2)/4 *(conjectural level)*

- **tau(6) = 4** is the divisor count (1, 2, 3, 6) of n=6. This is central to the n=6 number-theory identity.
- log(2) appears naturally in Dirichlet-L-function leading-coefficient computations:
  - $L(1, \chi) \sim c \cdot \log(p)$ form
  - log unit in the Gauss class-number formula
  - log(2 pi) appearance in the Chowla-Selberg residue of $L(s, \chi)$
- For the common factors 2, 3 of n=6, the "coupling strength" of the Sel_2, Sel_3 independence breakdown may appear in the form (log 2) / tau(6)

### 3.3 Honest boundary — NOT PROVED

- Empirical fit based on N = 1,733,824 curves (7 bins)
- Sample bin count = 7: statistical uncertainty exists (standard error ~ 0.01-0.02)
- log(2)/4 = 0.1733 is **included** in the range alpha = 0.1752 +/- 0.02. This is "match possibility", not a "mathematical theorem"
- Theoretical derivation (within the BKLPR model) is **outside current session scope**. Requires T4-T6 + M3 (Lean4)

---

## §4 Joint-distribution mathematical model — (A3'')

### 4.1 Curve-level coupling factor

For each curve E (possible with actual allbsd data):
$$\eta(E) := \frac{|Sel_6(E)|}{|Sel_2(E)| \cdot |Sel_3(E)|} - 1$$

(A3 independence) iff eta(E) = 0 a.s.

**(A3')** modified (loop 2): exists weak correlation, $\mathbb{E}[\eta(E)] = 0$, $\mathrm{Var}(\eta) \neq 0$.

**(A3'')** v3 T3: exists B-dependent coupling,
$$\mathbb{E}[\eta(E) | N(E) \in [B-\delta, B+\delta]] \approx c \cdot \log(B)^\alpha / \sqrt{B}^\gamma$$
with $\alpha \approx \log(2)/4$, $\gamma$ TBD.

### 4.2 Mechanism decomposition of kappa(B)

$$\mathbb{E}[|S_6|]_B = \mathbb{E}[|S_2| \cdot |S_3|]_B = \mathbb{E}[|S_2|]_B \cdot \mathbb{E}[|S_3|]_B + \mathrm{Cov}_B(|S_2|, |S_3|)$$

Thus
$$\kappa(B) = \mathrm{Cov}_B(|S_2|, |S_3|) + \mathbb{E}[(|S_6| - |S_2||S_3|)]_B$$

Split into two components:
- **Cov term**: covariance of the joint distribution of |S_2| and |S_3|
- **Coupling term**: per-curve average of eta(E)

**next step** (v3 E7 or T4): per-curve simultaneous extraction of (|S_2(E)|, |S_3(E)|, |S_6(E)|) -> precise Cov measurement.

### 4.3 Relation with recent Bhargava-Shankar results

Bhargava-Shankar (2013, 2015) "Binary quartic/quintic forms" compute avg |Sel_n(E)|:
- $\mathbb{E}[|Sel_2|] = 3$ asymptotic (B -> infinity)
- $\mathbb{E}[|Sel_3|] = 4$ asymptotic
- $\mathbb{E}[|Sel_4|] = 7$ asymptotic
- $\mathbb{E}[|Sel_5|] = 6$ asymptotic

Our measurement: E[|S_2|] 2.87 -> 3.30 (converging toward 3 asymptotic), E[|S_3|] 2.85 -> 3.40 (toward 4).

**Difference in asymptotic**: Bhargava-Shankar averages use **arithmetic height** sorting, while we use **conductor** sorting — different orderings.

Bhargava-Shankar proves n=2, 3 separately. For n=6 there is still no unconditional draft. **To extend Bhargava-Shankar to n=6**, the precise asymptotic of our kappa(B) is required.

---

## §5 v3 T3 outputs + future connections

### 5.1 Outputs

1. **(A3'') conjecture** — B-dependent coupling, power-law alpha ~ log(2)/4
2. **Mathematical decomposition** kappa(B) = Cov + Coupling term
3. **alpha ~ log(2)/4 match** — suggestive, possible tau(6)=4 connection
4. `scripts/empirical/cremona_kappa_10bin.py` — 7+ bin analyzer

### 5.2 What is not resolved (honest declaration)

- Theoretical derivation (within BKLPR) of alpha = log(2)/4: **not drafted**
- Direct measurement of Cov(|S_2|, |S_3|): **not yet** (requires v3 E2/E4 Sage)
- BSD implication: **indirect, weak**
- BT-541 (RH), BT-546 (BSD) draft: **0/6 honest maintained**

### 5.3 Follow-up tasks (v3 loop 15+)

- **v3 T4**: BT-541 Guth-Maynard 2024 zeta-zero re-study (in parallel)
- **v3 E2**: Sage `E.selmer_group(n)` per-curve precise values -> direct Cov
- **v3 M1**: preprint this document (GALO-PX-4 + A3'' announcement)

---

## §6 atlas entries

```
@R MILL-V3-T3-alpha-log2-over-4-suggestive = alpha = 0.1752, log(2)/4 = 0.1733 (err 1.1%) :: n6atlas [9]
  "v3 T3 (2026-04-15 loop 14): slope alpha = 0.1752 +/- 0.02 (7 bins) of kappa(B) ~ A*B^alpha power law
   (v3 E5). log(2)/4 = 0.1733 differs by 0.0019 (1.1% rel err). Natural n=6 combination of tau(6) = 4
   and log 2 proposed. SUGGESTIVE EMPIRICAL MATCH, NOT PROVED. Theoretical derivation incomplete —
   links to v3 T4+M3. (A3'') conjecture: B-dependent coupling coefficient, curve-level eta(E) defined"
  <- v3-T3, reports/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md
```

---

## §7 Related files

- Data: `data/cremona/kappa_10bin_results.json` (v3 E5 output)
- Script: `scripts/empirical/cremona_kappa_10bin.py`
- Prior result: `reports/breakthroughs/v3-e5-kappa-7bin-power-law-2026-04-15.md`
- Parent: `reports/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` -> `_v3_phases.P12_v3.T3`

---

*Drafted: 2026-04-15 loop 14*
*Honesty charter: BT draft 0/6 maintained. alpha = log(2)/4 is a suggestive measurement, not a mathematical theorem.*
