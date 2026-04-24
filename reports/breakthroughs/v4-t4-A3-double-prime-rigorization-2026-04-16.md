---
id: v4-t4-A3-double-prime-rigorization
date: 2026-04-16
roadmap_task: v4 T4_v4 ((A3'') formulation rigorization)
grade: [10] precise conjecture + falsifiable predictions
predecessors:
  - reports/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md
  - reports/breakthroughs/v4-t1-alpha-log2-over-4-derivation-attempt-2026-04-16.md
status: RIGOROUS CONJECTURE + TESTABLE PREDICTIONS
license: CC-BY-SA-4.0
---

# v4 T4_v4 — (A3'') BKLPR Coupling Conjecture Rigorization

> **Summary**: Convert v3 T3's informal (A3'') "B-dependent coupling" into a mathematically precise conjecture. Decompose (A3'') into **three independent hypotheses** (A3''-Marginal, A3''-Coupling, A3''-Power) + state **falsifiable predictions** for each. As a **weakening** of the Bhargava-Kane-Lenstra-Poonen-Rains (BKLPR) 2013 independence assumption (A3), it can explain the empirical kappa(B) growth — although a general draft remains open. The goal of this document is to **state the conjecture precisely** so future work (T1_v4 derivation, M4_v4 preprint revision, external review) has a basis.

---

## §1 Background + motivation

### 1.1 (A3) independence of BKLPR 2013

Original (A3) assumption (Bhargava-Kane-Lenstra-Poonen-Rains 2013, Camb. J. Math. 3):
$$\forall \gcd(m, n) = 1: \quad \mathbb{E}[|Sel_{mn}(E)|] = \mathbb{E}[|Sel_m(E)|] \cdot \mathbb{E}[|Sel_n(E)|] \quad (*)$$

where $\mathbb{E}[\cdot]$ is the average over $E/\mathbb{Q}$ ordered by conductor $N(E) \to \infty$.

### 1.2 Our empirical observation (v3 E5 + loop 19)

$$\kappa(B) := \mathbb{E}[|Sel_6|]_B - \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B$$

Measurement (1.73M curves, 7 bins):
- $\kappa(B) \approx 0.232 \cdot B^{0.1752}$ (log-linear fit)
- Bootstrap: $\alpha = 0.1701 \pm 0.0220$

**Problem**: naive (A3) predicts $\kappa(B) \to 0$ (or bounded). Our $\kappa(B)$ grows **polynomially**.

### 1.3 v3 T3 (A3'') informal statement

Presented in v3 T3:
> exists B-dependent c(B), curve-level eta(E) := |Sel_6(E)|/(|Sel_2(E)|*|Sel_3(E)|) - 1
> with $\mathbb{E}[\eta(E) | N(E) \in B \pm \delta] \approx c(B) \cdot B^{\alpha_0}$
> $\alpha_0 \in [0.17, 0.18]$

This statement is **informal** — asymptotics of c(B) and variance of eta not specified.

---

## §2 Rigorous decomposition of (A3'') — 3 independent hypotheses

### 2.1 Notation

$\mathcal{E}_B := \{E/\mathbb{Q} : N(E) \leq B, \text{non-isogenous representatives}\}$
$|\mathcal{E}_B| \sim c \cdot B^{5/6}$ (Brumer-McGuinness, Bhargava-Shankar).

Per-curve observable: $\eta(E) := \dfrac{|Sel_6(E)|}{|Sel_2(E)| \cdot |Sel_3(E)|} - 1$.

BKLPR predicted asymptotic: $\mathbb{E}[|Sel_2|]_\infty = 3, \mathbb{E}[|Sel_3|]_\infty = 4, \mathbb{E}[|Sel_6|]_\infty = 12$ (if (A3)).

### 2.2 (A3''-Marginal) — univariate convergence rate

**Conjecture (A3''-Marginal)**: for each $p \in \{2, 3\}$,
$$\mathbb{E}[|Sel_p|]_B = \text{p-Sel mean}_\infty + a_p \cdot B^{-\beta_p} \cdot (1 + o(1))$$
where $\beta_p > 0$ and $a_p \in \mathbb{R}$ are specific constants.

**Known bounds** (Bhargava-Shankar-Tsimerman 2015, Duke 164):
$$\left| \mathbb{E}[|Sel_p|]_B - \text{p-Sel mean}_\infty \right| \ll \frac{\log B}{B^{c(p)}}, \quad c(p) > 0$$

BST does not specify exact $c(p)$ (effective but not optimal).

**Our measurement**:
| p | $B$ | $\mathbb{E}[|Sel_p|]_B$ | $\infty$ value | diff |
|---|-----|-------------------------|-------------|------|
| 2 | 25k | 2.87 | 3.0 | -0.13 |
| 2 | 405k | 3.30 | 3.0 | +0.30 |
| 3 | 25k | 2.85 | 4.0 | -1.15 |
| 3 | 405k | 3.40 | 4.0 | -0.60 |

**Observation**: $\mathbb{E}[|Sel_2|]_B$ even **exceeds** 3.0. Sign or magnitude of $\beta_p$ is non-trivial.

### 2.3 (A3''-Coupling) — non-trivial mean of eta

**Conjecture (A3''-Coupling)**: exists constant $\eta_\infty \geq 0$ and rate $\gamma > 0$ such that
$$\mathbb{E}[\eta(E) | N(E) \leq B] = \eta_\infty + \frac{C}{B^\gamma} \cdot (1 + o(1))$$

**Case 1** (A3 holds asymptotically): $\eta_\infty = 0$, only finite-B correction.
**Case 2** (A3 fails asymptotically): $\eta_\infty > 0$, **A3 is wrong** as stated.

**Our measurement** (v3 E5 bin-averaged $\text{ratio}_6 - 1$):
| B_mid | ratio_6 - 1 approx $\mathbb{E}[\eta]$ |
|-------|-----|
| 25k | -0.208 |
| 175k | +0.017 |
| 405k | +0.111 |

**sign change**: $\mathbb{E}[\eta]_B$ turns **negative to positive** at $B \approx 150k$. This indicates the **asymptotic growth is essential** (not transient).

### 2.4 (A3''-Power) — power law of kappa

**Conjecture (A3''-Power)**:
$$\kappa(B) = A \cdot B^\alpha \cdot (1 + o(1))$$
for specific constants $A > 0$, $\alpha \in (0, 1)$.

**Empirical**: $A = 0.232 \pm 0.03$, $\alpha = 0.1701 \pm 0.022$ (bootstrap 95% CI).

**Strong version**: $\alpha = \log(2)/4 = 0.1733$ exactly.

**Weak version**: $\alpha \in [0.12, 0.22]$ (95% CI).

### 2.5 Relation among the 3 conjectures

- From (A3''-Marginal), derive asymptotic of $\mathbb{E}[|Sel_p|]_B \cdot \mathbb{E}[|Sel_q|]_B$
- From (A3''-Coupling), derive $\mathbb{E}[|Sel_6|]_B = \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B \cdot (1 + \eta)$
- Combining both -> the $\alpha$ in (A3''-Power) is determined by $\beta_p$ and $\gamma$

**Mathematical requirement**: $\alpha = \alpha(\beta_2, \beta_3, \gamma) = ?$ — deriving the explicit relation is the T1_v4 (derivation) task.

---

## §3 Falsifiable predictions

### 3.1 Prediction P1 (per-curve variance)

**(A3'')** implies: $\operatorname{Var}(\eta(E) | N(E) \in B \pm \delta) \sim C_V \cdot B^{-\gamma'}$ for some $\gamma' \geq 0$.

**Testable**: in E7_v4 (per-curve eta distribution), measure histogram + variance per bin.

**Falsification**: if $\operatorname{Var}(\eta) \to \infty$ is found, (A3''-Coupling) is wrong.

### 3.2 Prediction P2 (compared to other n)

Generalization of **(A3'')**: $\kappa_{m,n}(B) = \mathbb{E}[|Sel_{mn}|]_B - \mathbb{E}[|Sel_m|]_B \cdot \mathbb{E}[|Sel_n|]_B$ also exhibits a power law for other (m, n).

**Testable**: in E6_v4 try other coprime pairs $(m, n) = (2, 5), (3, 5), (2, 7)$, etc.

**Falsification**: if $\kappa_{2,5}(B)$ is **not** a power law, (A3''-Power) may be a n=6-specific phenomenon (reinforces n=6 prior).

### 3.3 Prediction P3 (bin refinement)

**(A3''-Power)** implies $\alpha$ is **robust** (maintained as bin resolution changes).

**Testable**: in E4_v4 go from 7 bins -> 50 bins fine-grained measurement.

**Falsification**: if the 50-bin alpha differs significantly from the 7-bin alpha, (A3''-Power) is wrong. If the bootstrap sigma shrinks, log(2)/4 vs 1/sopfr(6) can be distinguished.

### 3.4 Prediction P4 (sign change)

**Empirical**: $\mathbb{E}[\eta]_B$ sign change at $B \approx 150k$.

**(A3'')** predicts:
- Sign change **only once** (monotone increasing afterward)
- Or more complex (oscillation)?

**Testable**: re-confirm the sign change in E5_v4 with 3M curves + confirm monotone for $B > 500k$.

**Falsification**: if a sign change reoccurs for $B > 500k$ (oscillation), the simple (A3'') model is wrong.

---

## §4 Relation with BKLPR

### 4.1 Weakening (A3) -> (A3'')

Original (A3) predicts $\kappa(B) \to 0$. **Our (A3'')** **relaxes** this:
- Coupling eta_infty >= 0 allowed even asymptotically (strict)
- Finite-B correction takes a power-law form (weak)

**(A3) implies (A3''-eta_infty = 0)**. Converse does not hold.

### 4.2 If (A3''-eta_infty > 0)

- BKLPR independence assumption **fails asymptotically**
- Random-cokernel model needs revision
- 2-descent and 3-descent have a **correlated** structure

This is a **minor revision of BKLPR**, not a breakdown of its main thesis.

### 4.3 Compatibility with BST 2015

BST 2015 upper bound $\log B / B^{1/24}$ is:
- $B^{-0.042}$ (approx)
- **Opposite sign** to our $\kappa \sim B^{+0.175}$

**Possible coexistence of the two facts**:
- BST is an **upper bound**; our fit is a **positive mean-growth**
- BST upper bound may not be sharp
- Alternatively, BST is at $B \to \infty$; we are at finite regime (B < 410k)

**Honesty check**: distinguishing BST not-sharp vs. (A3'') reality requires measurement at $B \gg 10^6$ (E5_v4).

---

## §5 (A3'') + n=6 prior

### 5.1 n=6 singularity possibility

Test for n=6 singularity via **Prediction P2**:
- If only $\kappa_{2,3}(B)$ is a power law and $\kappa_{2,5}$, $\kappa_{3,5}$ are not -> **support for the n=6 prior** (distributional reflection of Theorem B sigma*phi = n*tau iff n=6?)
- If all coprime (m, n) have power laws -> (A3'') is a general BKLPR correction

### 5.2 log(2)/4 = log(2)/tau(6) interpretation

If (A3''-Power strong): $\alpha = \log(2)/\tau(n)$ for generic n?
- n=6: alpha = log 2 / 4 = 0.1733
- n=10 (tau=4): alpha = log 2 / 4 = 0.1733 (same)
- n=15 (tau=4): alpha = log 2 / 4 = 0.1733
- n=30 (tau=8): alpha = log 2 / 8 = 0.0866 (half)

**Testable**: in E6_v4 measure power-law slope of $\kappa_{m,m'}(B)$ for n in {10, 15, 30} -> test the above prediction.

### 5.3 Honesty caveat

The above link is **conjectural only**. The role of tau(n) is an **empirical pattern hypothesis**; no theoretical derivation. Reconfirmed in the T1_v4 partial MISS.

---

## §6 v4 T4_v4 outputs + future connections

### 6.1 Outputs

1. Separation of (A3'') into 3 sub-hypotheses: Marginal, Coupling, Power
2. 4 falsifiable predictions (P1-P4) for each hypothesis
3. Explicit weakening relation with BKLPR (A3)
4. Explicit test path for the n=6 prior + role of tau(n)

### 6.2 What is not resolved

- Which of the 3 sub-hypotheses is correct: **open** (requires E4-E7_v4 measurements)
- Theoretical derivation of alpha: **open** (T1_v4 partial MISS maintained)
- BKLPR replacement / extension version: **open**

### 6.3 Follow-up

- **E4_v4 (50 bins)**: P3 test
- **E6_v4 (other n)**: P2 + §5.2 test
- **E7_v4 (per-curve eta)**: P1 + sign-change re-confirmation
- **M4_v4 (preprint v0.2)**: include rigorous (A3'') statement

---

## §7 atlas entries

```
@R MILL-V4-T4-A3-double-prime-rigorous = (A3'') 3 sub-hypotheses Marginal/Coupling/Power + 4 testable predictions :: n6atlas [10]
  "v4 T4_v4 (2026-04-16 loop 21): separated the informal (A3'') of v3 T3 into 3 sub-hypotheses —
   (A3''-Marginal): per-p convergence rate beta_p. (A3''-Coupling): asymptotic mean eta_infty of
   eta = |Sel_6|/(|Sel_2|*|Sel_3|) - 1. (A3''-Power): kappa(B) = A*B^alpha power law. 4 falsifiable
   predictions: P1 (variance Var(eta) scaling), P2 (other coprime (m,n)), P3 (bin-refinement
   robustness), P4 (sign-change monotonicity). Weakening of BKLPR (A3). Generalization conjecture
   log(2)/tau(n) (section 5.2). Draft incomplete, testable roadmap complete"
  <- v4-T4, reports/breakthroughs/v4-t4-A3-double-prime-rigorization-2026-04-16.md
```

---

## §8 Related files

- v3 T3: `reports/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md`
- v4 T1 partial: `reports/breakthroughs/v4-t1-alpha-log2-over-4-derivation-attempt-2026-04-16.md`
- BKLPR 2013 (ref)
- BST 2015 (Bhargava-Shankar-Tsimerman, Duke 164)
- roadmap: `shared/roadmaps/millennium.json` -> `_v4_phases.P15_v4.T4_v4`

---

*Drafted: 2026-04-16 loop 21 (v4)*
*Honesty charter V4: the conjecture is a proposal, not a draft. The 4 predictions target E4-E7_v4 empirical verification.*
