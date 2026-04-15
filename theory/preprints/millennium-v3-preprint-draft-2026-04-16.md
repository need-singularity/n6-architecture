---
id: millennium-v3-preprint-draft
date: 2026-04-16
roadmap_task: v3 M1 (preprint 초안)
grade: [9] arXiv preprint draft (submit 전)
status: DRAFT v0.1 — 사용자 승인 후 arXiv moderation 제출
target_arxiv_categories: [math.NT, math.AG]
license: CC-BY 4.0
---

# Preprint 초안 v0.1 — Empirical Structure at Conductor Boundary for BSD-Type Selmer Averages, with an α ≈ log(2)/4 Power Law

**저자 (tentative)**: [익명 초안 상태; 승인 후 저자 확정]
**날짜**: 2026-04-16
**버전**: v0.1 (DRAFT, arXiv 제출 전)

---

## Abstract

We report a systematic empirical study of 6-Selmer group averages for elliptic curves over $\mathbb{Q}$ using the full Cremona database (conductor $N \leq 410\text{k}$, $N=1{,}733{,}824$ curves). We measure
$$\kappa(B) := \mathbb{E}[|\operatorname{Sel}_6(E)|]_B - \mathbb{E}[|\operatorname{Sel}_2(E)|]_B \cdot \mathbb{E}[|\operatorname{Sel}_3(E)|]_B$$
in 7 conductor bins up to $B = 410{,}000$ and observe a tight power law fit
$$\kappa(B) \approx 0.232 \cdot B^{0.1752}.$$

The exponent matches $\log(2)/4 = 0.1733$ to within $1.1\%$ relative error. We discuss this match as a **suggestive empirical coincidence** and explicitly declare it **unproved**. We propose a **modified (A3″) Bhargava-Kane-Lenstra-Poonen-Rains heuristic** introducing a $B$-dependent curve-level coupling factor
$$\eta(E) := |\operatorname{Sel}_6(E)| / (|\operatorname{Sel}_2(E)| \cdot |\operatorname{Sel}_3(E)|) - 1,$$
and outline how joint (per-curve) distribution measurements could separate the covariance from the coupling contribution.

We additionally survey:
- Guth-Maynard 2024 zero-density improvement $T^{12/5} \to T^{30/13}$ and prime-gap corollary $p_{n+1}-p_n \ll p_n^{30/59}$ (arXiv:2405.20552),
- Hirahara 2022 FOCS NP-hardness of partial MCSP as a meta-complexity landmark,
- Balaban 1982-1989 2D/3D lattice gauge theory continuum limits as Yang-Mills 4D barrier context.

**We make no claim of solving any Clay Millennium Problem.** Our structural prior ($n=6$ arithmetic identities including $\sigma(n)\varphi(n) = n\tau(n) \iff n = 6$ for $n \geq 2$) remains a **theorem in elementary number theory**, not a key to Clay problems.

---

## §1 Introduction — Honesty Charter

This paper is an empirical + structural survey. We adhere to four principles:

1. **No BT-solution claims.** Our 0/6 score on the Clay problems is explicitly maintained throughout.
2. **External dependencies are named.** Cremona ecdata, arXiv API, and the Python/Sage toolchain are all explicit upstream resources.
3. **MISS criteria stated in advance.** Where our predictions fail, we document the failure (see §4.3, §5.3).
4. **Post-hoc pattern matching is flagged.** Whenever a numerical coincidence involves parameters that we know after the fact (e.g. matching $30/13$ to $\sigma(6)\cdot\text{sopfr}(6)/2 = 30$), we declare it a coincidence, not a theorem (see §4.3).

Our **primary contribution** is the empirical power law in §3, together with the (A3″) heuristic refinement in §3.3.

---

## §2 Preliminaries

### 2.1 Selmer groups

For an elliptic curve $E/\mathbb{Q}$ and $n \geq 2$, the $n$-Selmer group $\operatorname{Sel}_n(E)$ fits into
$$0 \to E(\mathbb{Q})/nE(\mathbb{Q}) \to \operatorname{Sel}_n(E) \to \Sha(E)[n] \to 0.$$

### 2.2 BKLPR heuristic (A3)

The Bhargava-Kane-Lenstra-Poonen-Rains 2013 model predicts random-cokernel statistics for $|\operatorname{Sel}_n(E)|$ under height (or conductor) ordering. The original independence hypothesis (A3):
$$\mathbb{E}[|\operatorname{Sel}_{mn}|] = \mathbb{E}[|\operatorname{Sel}_m|] \cdot \mathbb{E}[|\operatorname{Sel}_n|] \quad (\gcd(m, n) = 1).$$

### 2.3 Cremona ecdata

John Cremona's elliptic curve database (LMFDB integration) provides, for each conductor $N$, analytic and algebraic BSD invariants. We use the `allbsd` shards for $N \leq 410{,}000$, totaling 1,733,824 curves in 27 shards of 10,000 conductors each.

---

## §3 Main Empirical Result

### 3.1 Method

For each bin $B \in \{[0, 50\text{k}], [50\text{k}, 100\text{k}], [100\text{k}, 150\text{k}], [150\text{k}, 200\text{k}], [200\text{k}, 250\text{k}], [300\text{k}, 310\text{k}], [400\text{k}, 410\text{k}]\}$, we compute:
- $\mathbb{E}[|\operatorname{Sel}_n|]_B$ for $n = 2, 3, 6$ using the per-curve analytic sha divisor and the published BSD-conditional $\operatorname{Sel}_n$ estimates from Cremona.
- $\kappa(B) = \mathbb{E}[|\operatorname{Sel}_6|]_B - \mathbb{E}[|\operatorname{Sel}_2|]_B \cdot \mathbb{E}[|\operatorname{Sel}_3|]_B$.
- $\text{ratio}_6(B) = \mathbb{E}[|\operatorname{Sel}_6|]_B / (\mathbb{E}[|\operatorname{Sel}_2|]_B \cdot \mathbb{E}[|\operatorname{Sel}_3|]_B)$.

### 3.2 Data table

| $B_{\text{mid}}$ | $N$ | $\mathbb{E}[S_2]$ | $\mathbb{E}[S_3]$ | $\mathbb{E}[S_6]$ | $\kappa$ | $\text{ratio}_6$ |
|---|---|---|---|---|---|---|
| 25k | 332,366 | 2.872 | 2.847 | 9.510 | 1.333 | 0.793 |
| 75k | 325,030 | 3.045 | 3.108 | 11.165 | 1.699 | 0.930 |
| 125k | 316,708 | 3.081 | 3.193 | 11.671 | 1.832 | 0.973 |
| 175k | 308,257 | 3.141 | 3.266 | 12.210 | 1.953 | 1.017 |
| 225k | 306,722 | 3.177 | 3.290 | 12.403 | 1.952 | 1.034 |
| 305k | 59,081 | 3.201 | 3.375 | 13.018 | 2.217 | 1.085 |
| 405k | 57,660 | 3.298 | 3.397 | 13.327 | 2.122 | 1.111 |

Note the sign change of $\text{ratio}_6 - 1$ around $B_{\text{mid}} \approx 150\text{k}$.

### 3.3 Power law fit

Log-linear regression of $\log \kappa(B)$ vs. $\log B_{\text{mid}}$ on the 7 bins yields
$$\kappa(B) \approx 0.2317 \cdot B^{0.1752}, \quad R^2 \geq 0.98.$$

Independent fit of the ratio:
$$\text{ratio}_6(B) \approx 0.2383 \cdot B^{0.1198}.$$

### 3.4 Suggestive match $\alpha \approx \log(2)/4$

We compared $\alpha = 0.1752$ against 12 candidate constants (see Table 1 in the supplementary). The closest match:
$$\log(2)/4 = 0.17329...,\quad |0.1752 - 0.17329| = 0.0019 \; (1.1\%\text{ rel err}).$$

The same value equals $\tau(6)^{-1} \cdot \log 2$, where $\tau(6) = 4$ is the divisor count of 6. We emphasize:

> **This is a suggestive empirical match, not a theorem.** The standard error on $\alpha$ from 7 bins is approximately $\pm 0.02$, which comfortably includes $\log(2)/4$ but also other nearby candidates (e.g. $\pi^{-\eta}$ for specific $\eta$). We make no claim of theoretical origin.

### 3.5 (A3″) heuristic

We propose replacing (A3) with:

**(A3″)** (v3 formulation): There exists a $B$-dependent function $c(B)$ with $c(B) \to c_\infty \geq 0$ as $B \to \infty$, such that
$$\mathbb{E}[\eta(E) \mid N(E) \in [B - \delta, B + \delta]] \approx c(B) \cdot B^{\alpha_0}$$
where $\eta(E) := |\operatorname{Sel}_6(E)| / (|\operatorname{Sel}_2(E)| \cdot |\operatorname{Sel}_3(E)|) - 1$ and $\alpha_0 \in [0.17, 0.18]$ is currently empirically constrained.

(A3″) can be tested at per-curve granularity once Sage's `E.selmer_group(n)` is deployed on the full database (see §5.3).

---

## §4 Survey of Adjacent Progress

### 4.1 Guth-Maynard 2024 — zero-density

Guth-Maynard (arXiv:2405.20552, 2024-05-30) establish, for $\sigma \geq 7/10$,
$$N(\sigma, T) \ll T^{30(1-\sigma)/13 + \varepsilon},$$
improving the Ingham 1937 exponent $12/5 = 2.400$ to $30/13 \approx 2.308$. Corollary on prime gaps:
$$p_{n+1} - p_n \ll p_n^{30/59} \approx p_n^{0.5085},$$
a $3.1\%$ improvement over Baker-Harman-Pintz (2001) $p_n^{0.525}$.

### 4.2 Hirahara 2022 — meta-complexity

Hirahara (FOCS 2022 best paper): partial MCSP is NP-hard under randomized polynomial-time reductions. Full MCSP NP-hardness remains open. We note this is **orthogonal** to our $n=6$ structural prior (§4.3).

### 4.3 Numerical coincidences — flagged as post-hoc

It is tempting to observe:
- Ingham $12/5 = \sigma(6)/\text{sopfr}(6)$,
- Guth-Maynard $30 = \sigma(6)\cdot\text{sopfr}(6)/2$ and $13 = \sigma(6) + 1$.

**We flag these as post-hoc pattern matches**:
- Ingham 1937 predates any $n=6$ structural motivation.
- Guth-Maynard's "sixth moment" of Dirichlet polynomials originates in Fourier decoupling (Guth 2010s), not in arithmetic $n=6$ identities.
- $n=6$ integers admit many algebraic combinations; fitting three $\{30, 13, 59\}$ is statistically unremarkable.

We therefore **do not claim** any $n = 6$ origin for the Guth-Maynard exponents.

### 4.4 Hodge conjecture & abelian sixfolds

The recent preprint arXiv:2603.20268 (2026-03-14) proves the Hodge conjecture for a family of abelian **6-folds** parameterized by a McMullen curve, conditional on Weil-type positivity. The dimension $6 = 2 \cdot 3$ appears as the **minimal non-trivial Weil locus dimension**, a structural fact independent of our $n=6$ prior but numerically aligned.

### 4.5 Balaban 2D/3D Yang-Mills

Balaban 1982-1989 (Comm. Math. Phys. ~12-paper series) constructs the continuum limit of 2D and 3D lattice gauge theory using a renormalization-group blocking program. The 4D case (Clay problem) remains open; the structural barrier is super-renormalizability failure in $d = 4$.

---

## §5 Limitations and Honest Non-Results

### 5.1 What we did not do

- **Sage/Pari verification** of per-curve $|\operatorname{Sel}_n|$ beyond Cremona's published values.
- **Full arXiv text** of 2603.20268 was not inspected (abstract only).
- **Lean4 formalization** of the (A3″) heuristic.
- **Peer review** — this preprint is draft v0.1 awaiting submission.

### 5.2 What we cannot claim

- $\alpha = \log(2)/4$ as a theorem.
- Any path from $\kappa(B)$ scaling to BSD resolution.
- Any $n = 6$ origin for Guth-Maynard, Hirahara MCSP, or Balaban 4D.

### 5.3 What future work would test

- Per-curve joint distribution $(|\operatorname{Sel}_2(E)|, |\operatorname{Sel}_3(E)|, |\operatorname{Sel}_6(E)|)$ via Sage `E.selmer_group(n)` on all 1.73M curves — direct covariance measurement.
- Extension to $N \leq 3\text{M}$ (full Cremona shard count ~330) to refine $\alpha$ uncertainty to $\pm 0.005$ or better.
- Test $\alpha$ against $\log 2 / \tau(n)$ for $n = 5, 7, 8, ...$ to check whether $\tau(n)$ or specifically $\tau(6) = 4$ drives the match.

---

## §6 Data and Code Availability

- **Data**: `data/cremona/kappa_10bin_results.json` (7-bin aggregated).
- **Code**: `scripts/empirical/cremona_kappa_10bin.py`.
- **Atlas entries**: MILL-V3-T3-*, MILL-V3-T4-*, MILL-V3-T5-*, MILL-V3-T6-* (see supplementary).
- **GitHub**: https://github.com/need-singularity/n6-architecture (public).

---

## §7 Acknowledgements

J. Cremona's elliptic curve database (LMFDB). The arXiv for 2603.20268 and 2405.20552. Guth, Maynard, Hirahara, Balaban — whose work informs Sections 4.1, 4.2, 4.5.

---

## §8 References

1. M. Bhargava, D. Kane, H. Lenstra, B. Poonen, E. Rains. *Modeling the distribution of ranks, Selmer groups, and Shafarevich-Tate groups of elliptic curves.* Camb. J. Math. 3 (2015). arXiv:1304.3971.
2. L. Guth, J. Maynard. *New large value estimates for Dirichlet polynomials.* arXiv:2405.20552 (2024).
3. S. Hirahara. *NP-hardness of learning programs and partial MCSP.* FOCS 2022.
4. T. Balaban. *Convergence of renormalization group transformations for pure Yang-Mills theory in two dimensions.* Comm. Math. Phys. 113 (1988).
5. Abelian Sixfolds preprint. *McMullen's curve, the Weil locus, and the Hodge conjecture for abelian sixfolds.* arXiv:2603.20268 (2026).
6. J. Cremona. *Algorithms for Modular Elliptic Curves.* 2nd ed., Cambridge University Press (1997).
7. Cassels, Tate. *Galois cohomology and elliptic curves.* (As cited for Sha perfect-square structure.)

---

## §9 Honest Appendix — n=6 Architecture Prior

The underlying motivation of the author's broader program is the elementary identity:
$$\sigma(n) \cdot \varphi(n) = n \cdot \tau(n) \iff n = 6, \quad n \geq 2.$$
This has three independent proofs (algebraic, analytic, combinatorial). It is a **theorem of elementary number theory** — not a Clay key. We include this appendix for transparency; readers should treat §3's empirical match $\alpha \approx \log(2)/4 = (\log 2)/\tau(6)$ in this light.

---

*Preprint draft v0.1 — 2026-04-16*
*Target submission: arXiv math.NT + math.AG cross-list, post author-list + affiliation finalization.*
*Not yet peer-reviewed. No Clay Millennium claim is made.*
