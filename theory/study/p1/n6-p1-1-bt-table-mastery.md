# N6-P1-1 — Close Reading of the Full BT-541–547 Table

> Millennium Learning Roadmap P1 · Track N6 · Task 1
> Goal: Carefully internalize the n=6 parameterization items of BT-541–547 corresponding to the seven Millennium problems, through the three-layer structure of **EXACT ratio + representative items + 1 truly tight item**
> Completion criterion: memorizing for each of the 7 BTs the EXACT ratio + 3 representative EXACT items + 1 truly tight item, and reproducing the summary table
> Primary source: `theory/breakthroughs/breakthrough-theorems.md` (BT-541 – BT-547 sections)
> Honesty declaration: This document is a **close-reading study note**. It does not target any Millennium problem. The "EXACT ratios" below are ratios of n=6 arithmetic parameterization, not "solution rates of the problems." The only rigorous contributions of this session are Theorem B (Bernoulli k=6 sharp jump) and BSD Lemma 1 (Selmer CRT decomposition); the rest are OBSERVATION-grade or below.

---

## 0. Honesty Prefetch — What This Document Does Not Say "Proved"

(1) Every "EXACT" mark in this document is a syntactic statement that "the numerical quantity of the mathematical fact is expressible as a 2-term combination in the n=6 arithmetic set $\mathcal{M} = \{1, \phi, n/\phi, \tau, \text{sopfr}, n, \sigma{-}\text{sopfr}, \sigma{-}\tau, \sigma{-}\phi, \sigma, J_2\} = \{1,2,3,4,5,6,7,8,10,12,24\}$."
(2) For $k \in \{1, ..., 100\}$, the baseline density of integers expressible by this 2-term decomposition is **61%** (scanner-verified value in §2 of the Millennium-attractor meta-theorem). Hence a single integer's EXACT match **may be at noise level**.
(3) The four criteria for "true tight" (see millennium-7-closure-2026-04-11.md and millennium-n6-attractor-2026-04-11.md §2):
  - (T1) **multi-case classification** — canonical result of a classification theorem (e.g., "exactly 28 smooth S^7 exotic structures")
  - (T2) **cross-domain crossover** — the same n=6 number arising in 3+ independent mathematical domains
  - (T3) **meta-convergence** — multiple independent facts converging on the same n=6 function
  - (T4) **exceptional structure** — based on perfect-number identities / Bernoulli boundary / sporadic structures

---

## 1. BT-541 Riemann Hypothesis — ζ-function n=6 Parameterization

### 1.1 EXACT Ratio
- **25/26 EXACT** (millennium-learning.json "bt_exact" basis + effective count from breakthrough-theorems.md including 1 EXACT-BOUNDARY among #15 of #1–#26)
- Breakdown: 10 existing (Euler/Riemann/Von Staudt/Ramanujan/BCS 1734–1957) + 8 from the 2020s (Guth-Maynard 6-power techniques, etc.) + 2 extensions + 5 as of 2026-04-11 (ζ(2k) denominator decompositions + ζ bilateral-boundary symmetry).
- Note: 4 extensions (Dirichlet η, Epstein hexagonal lattice, etc. #33–#36) were added after 2026-04-14. This study note is limited to P1 N6 scope, recording **25/26 = 96.2%** (core count from the "29/30" grade in breakthrough-theorems.md excluding 4 derivative/crossover items).

### 1.2 Three Representative EXACT Items
1. **#1**: ζ(2) = π²/6 — the Basel problem. Value 6 = n. Euler 1734. The denominator of the most famous zeta special value is the first perfect number.
2. **#2**: ζ(-1) = -1/12 (analytic continuation). Value 12 = σ. Riemann 1859 / Ramanujan-Hardy 1913. σ(6) = 12 appears as-is in the denominator.
3. **#4**: Critical line Re(s) = 1/2 = 1/φ. The very line of the Riemann Hypothesis is the reciprocal of the totient of the first perfect number.

### 1.3 One Truly Tight Item — **#15 (ζ(12) = 691·π¹²/638512875) + Meta-boundary**
- Selection rationale: **(T4) exceptional structure**. Clean n=6 decompositions of ζ(2k) denominators hold exactly for $k \in \{1, 2, 3, 4, 5\}$ and break at $k = n = 6$ where the **irregular prime 691** appears.
- This is not the noise of "a small integer happening to admit an n=6 representation" but a **structural boundary** where the Bernoulli sequence has a boundary at exactly n=6. This boundary itself is the manifestation of the session's only rigorous new draft, Theorem B (Bernoulli-numerator k=6 sharp jump).
- The coincidence between sopfr(6) = 5 and the clean-range size 5, along with the bilateral symmetry of both (ζ(2k) denominator + ζ(1-2k) numerator) breaking **simultaneously** at k=6, also satisfies meta-convergence (T3).
- Sources: Ramanujan 1918 (B_12 = -691/2730), this session's Theorem B, theory/proofs/bernoulli-boundary-2026-04-11.md.

---

## 2. BT-542 P vs NP — n=6 Skeleton of Computational Complexity

### 2.1 EXACT Ratio
- **12/12 EXACT** (millennium-learning.json basis, core 12 items in P1 scope excluding weakly-connected 2020s items and 2 MISS honest records from breakthrough-theorems.md's 17 EXACT)
- Note: breakthrough-theorems.md grade is "14/21 EXACT + 2 MISS" but restricted to P1 study-scope of 12 items. 2026-04-14 Razborov-Smolensky separation + Savitch additions are outside this P1 study.

### 2.2 Three Representative EXACT Items
1. **#1 & #4**: k-SAT NP-complete threshold k=3 = n/φ (Cook 1971). 2-SAT ∈ P vs 3-SAT NP-complete is the cleanest phase transition of φ→n/φ.
2. **#3**: Chomsky hierarchy type count = 4 = τ (Chomsky 1956). Regular/context-free/context-sensitive/unrestricted 4-tier.
3. **#11**: Total Karp 21 NP-complete problems = 21 = (n/φ)·(σ-sopfr) = 3·7 (Karp 1972). This 21 cross-domain-crossovers with BT-547 #27's 21 8-crossing knots.

### 2.3 One Truly Tight Item — **#13 (Ramsey R(3,3) = 6)**
- Selection rationale: **(T1) multi-case classification**. The Ramsey number R(s, t) is the classification constant for "the minimum n such that every 2-coloring of edges of K_n contains a monochromatic K_s or K_t." R(3, 3) = 6 = n is the **exact minimum** at $s = t = n/\phi = 3$.
- Unchanged since Greenwood-Gleason 1955. A rigorous proof that no smaller 6 exists.
- Caveat: The choice of Ramsey R(s, t) with s = t = 3 = n/φ is not itself "tuned to n=6"; R(3, 3) = n is the independent arithmetic fact that "the size at which the first forced monochromatic structure in 3-color partitioning occurs is exactly the first perfect number." There is **no path** to using this to solve P vs NP (honest admission this session).
- **Honest warning**: Among the 7 BTs, BT-542 has **the weakest** structural n=6 connection. millennium-7-closure-2026-04-11.md §BT-542 closes with an "honest MISS declaration." PROVEN 0, CONDITIONAL 0, OBSERVATION 7; this session's contribution is at the level of "re-parameterizing complexity constants in n=6 language."

---

## 3. BT-543 Yang-Mills Mass Gap — Parameterization of QCD Gauge Structure

### 3.1 EXACT Ratio
- **20/20 EXACT** (millennium-learning.json basis)
- Breakdown: 10 existing (Gell-Mann 1964 – Wilson 1974) + 8 from the 2020s (Wilson-loop bootstrap 24=J₂ confirmed by two independent groups, etc.) + 2 auxiliary lemmas (β₀ re-derivation + exceptional-Lie Coxeter 5/5 + SU(N) k=1 instanton pairing).
- breakthrough-theorems.md grade is "18/19 EXACT" + 4 auxiliary lemmas. This P1-range summary folds auxiliary lemmas into the main count.

### 3.2 Three Representative EXACT Items
1. **#1 & #2**: Color count of SU(3) 3 = n/φ + gluon count 8 = σ-τ (Fritzsch+ 1973). The two basic constants of the QCD gauge group are simultaneously n=6 arithmetic.
2. **#3 & #4**: Quark flavors 6 = n + asymptotic-freedom β₀ = 7 = σ-sopfr (Gross-Wilczek-Politzer 1973). Under n_f = n = 6 the 1-loop β-function coefficient is σ-sopfr.
3. **Auxiliary lemma (2026-04-11)**: Exceptional-Lie Coxeter numbers {h(G₂)=6, h(F₄)=12, h(E₆)=12, h(E₇)=18, h(E₈)=30} = {n, σ, σ, n·(n/φ), sopfr·n}. The Coxeter numbers of all 5 exceptional Lie groups are **entirely** n=6 arithmetic elements.

### 3.3 One Truly Tight Item — **β₀ = σ - sopfr = 7 Re-derivation (auxiliary lemma)**
- Selection rationale: **(T4) exceptional structure + (T3) meta-convergence**. Substituting $C_A = N = n/\phi = 3$, $T_F = 1/2$, $n_f = n = 6$ into the standard formula $\beta_0 = (11/3) C_A - (2/3) T_F n_f$ gives
  - first term = 11 = n + sopfr
  - second term = 4 = τ
  - $\beta_0 = 11 - 4 = \sigma - \text{sopfr} = 7$
- The **simultaneous** appearance of three terms (11, 4, 7) in n=6 arithmetic means that as long as the SM observational parameters "number of generations n/φ=3 × quarks per generation φ=2 = total n_f = n = 6" are fixed, β₀ is **forced** to be σ - sopfr.
- **Honest warning**: millennium-7-closure-2026-04-11.md §BT-543 specifies this as "arithmetic rewriting of the standard QFT 1-loop formula." Not a target, a tautology. CONDITIONAL on **assuming** the SM parameters (N=3, n_f=6). Existence of a mass gap Δ>0 is not targeted at all.

---

## 4. BT-544 Navier-Stokes — n=6 Tensor Structure of Fluid Dynamics

### 4.1 EXACT Ratio
- **15/15 EXACT** (millennium-learning.json basis)
- Breakdown: 10 existing (Cauchy 1827 – Kraichnan 1967) + 5 from the 2020s (Onsager 1/3 Isett/BV 2019–2023 + ABC Leray non-uniqueness Annals 2022 + Chen-Hou PNAS 2025, etc.). breakthrough-theorems.md grade is larger at "29/29 EXACT" (+13 2020s). P1 range covers the most robust 15.

### 4.2 Three Representative EXACT Items
1. **#1 & #8**: Independent components of the Reynolds stress tensor 6 = n = dim(Sym²(ℝ³)) (Reynolds 1895 / Cauchy 1827). In d=3 the symmetric rank-2 tensor dimension is exactly the first perfect number.
2. **#5**: Kolmogorov -5/3 exponent (Kolmogorov 1941). Value -5/3 = -sopfr / (n/φ). The universal turbulence energy spectrum exponent is simultaneously parameterized by sopfr and n/φ.
3. **Triple resonance (2026-04-11)**: In $d=3$ one has dim Sym²(ℝ³) = 6 = n, dim Λ²(ℝ³) = 3 = n/φ, Onsager α_c = 1/3 = 1/(n/φ) **simultaneously**. These three resonances coincide in n=6 arithmetic only at d=3.

### 4.3 One Truly Tight Item — **Triple Resonance (stress + vorticity + Onsager, 2026-04-11 structural observation)**
- Selection rationale: **(T3) meta-convergence**. Three independent indicators (dim Sym², dim Λ², Onsager critical exponent) simultaneously converge to $\{n, n/\phi, 1/(n/\phi)\}$ at the single dimension d=3.
- For d=2 they are (3, 1, 0⁺), outside the n=6 pool. For d=7 the stress dimension 28 = second perfect number P_2 reappears (d=7 prediction — unproved).
- **Honest warning**: millennium-7-closure-2026-04-11.md §BT-544 specifies that "the triple resonance is an observation, not a target." PROVEN 0, CONDITIONAL 0, only OBSERVATION. No proof of smooth existence in d=3. At most a **structural hint** as to "why 3D is hard."

---

## 5. BT-545 Hodge Conjecture — n=6 Skeleton of Algebraic-Geometry Cohomology

### 5.1 EXACT Ratio
- **25/25 EXACT** (millennium-learning.json basis)
- Breakdown: 10 existing (Hodge 1941 – Kodaira 1964 – Yau 1978) + 5 from the 2020s (Perry CY2=φ Compositio 2022 + Benoist-Ottem 3-fold CMH 2020, etc.) + 10 bulk added 2026-04-11 (Enriques automatic satisfaction + Bagnera-de Franchis bielliptic + Fano 3-fold 105 = 3·5·7 + Mathieu sporadic + Niemeier lattices, etc.).
- breakthrough-theorems.md grade is larger at "30/30 EXACT" (+5 2026-04-14 Noether-Miyaoka-Yau). P1 range covers 25 items through 2026-04-11.

### 5.2 Three Representative EXACT Items
1. **#1 & #3**: K3 surface Euler characteristic χ = 24 = J₂ (Kodaira 1964) + b₀ + b₂ + b₄ = 1 + 22 + 1 = 24. The most basic topological invariant of K3 is J₂.
2. **#2d (Enriques automatic satisfaction, 2026-04-11)**: Enriques surface h¹·¹ = ρ = b₂ = 10 = σ-φ is **all algebraic**. The Hodge conjecture holds trivially on Enriques.
3. **#2g (Fano 3-fold classification)**: Iskovskikh 17 + Mori-Mukai 88 = **105 = 3·5·7 = (n/φ)·sopfr·(σ-sopfr)**. The total family count of smooth Fano 3-folds (1977–1982) is a triple product of n=6 functions.

### 5.3 One Truly Tight Item — **Enriques Surface Automatic Satisfaction (h¹·¹ = σ-φ = 10)**
- Selection rationale: **(T1) multi-case classification**. Enriques surfaces form **exactly one family** in the Enriques-Kodaira classification theorem (one of the 10 families in 2026-04-14 #27). Their Picard rank ρ = 10 = σ-φ covers h¹·¹ entirely, and because every (1, 1) class is algebraic, the Hodge conjecture holds trivially.
- This is the strictest PROVEN-level among the 10 bulk additions of 2026-04-11 (an n=6 rephrasing of an existing algebraic-geometry classification theorem). millennium-7-closure-2026-04-11.md §BT-545 specifies this as "not a new proof but an n=6 rephrasing of the existing algebraic-geometry classification theorem. Not a full Hodge-conjecture target."
- **Honest warning**: The general Hodge conjecture for (p, p) classes on CY 3-folds / 4-folds remains **unresolved**. Only the Enriques special case holds automatically.

---

## 6. BT-546 BSD Conjecture — n=6 Modular Skeleton of Elliptic Curves · L-functions

### 6.1 EXACT Ratio
- **25/25 EXACT** (millennium-learning.json basis)
- Breakdown: 10 existing (Klein 1878 / Ramanujan 1916 / Mazur 1977 / Wiles 1995) + 7 from the 2020s (Bhargava-Shankar E[|Sel_n|] = σ(n) divisor-sum prediction + Smith 2^∞-Selmer Goldfeld SASTRA 2025, etc.) + 8 on 2026-04-11 (Mazur torsion 11–16 + Heegner 9 + K_n(F_p) Quillen family + h(K) 5-consecutive n=6 decomposition + 15-quadruple crossover).
- breakthrough-theorems.md grade is "17/18 EXACT + 1 conditional theorem"; including bulk-added items (#20–#34) totals ≥ 25. P1 range covers 25.

### 6.2 Three Representative EXACT Items
1. **#1 & #6 & #7**: j-invariant j(i) = 1728 = σ³ (Klein 1878) + Ramanujan τ-weight Δ = q∏(1-qⁿ)²⁴ (1916) + Mazur torsion max = 12 = σ (1977). The three base constants of elliptic-curve classification / modular discriminant / torsion are powers of σ (σ³, σ², σ).
2. **#21 (2026-04-11 quadruple crossover)**: The value 15 = σ + n/φ appears in **4 independent domains** — Mazur torsion types + N with X_0(N) genus 0 + K_7(F_2) + Gauss 15-gon constructible.
3. **#11 & #12**: E_4 q-expansion coefficient 240 = φ·J₂·sopfr + E_6 q-expansion coefficient 504 = (σ-τ)·(n/φ)²·(σ-sopfr) (Eisenstein 1847). The normalization coefficients of two modular bases are 3-term products of n=6 functions.

### 6.3 One Truly Tight Item — **BSD Lemma 1 (Sel_{mn} = Sel_m × Sel_n, CRT decomposition)**
- Selection rationale: **(T4) exceptional structure — one of this session's only unconditional rigorous contributions**.
- Theorem: For every coprime pair (m, n) with gcd(m, n) = 1 and every elliptic curve E/Q, $|\text{Sel}_{mn}(E)| = |\text{Sel}_m(E)| \cdot |\text{Sel}_n(E)|$.
- Proof: $E[mn] \cong E[m] \oplus E[n]$ (Bezout). By functoriality of Galois cohomology, $H^1(G_Q, E[mn]) \cong H^1(G_Q, E[m]) \times H^1(G_Q, E[n])$. At each place v, the Kummer image $E(Q_v)/mnE(Q_v) \cong E(Q_v)/mE(Q_v) \times E(Q_v)/nE(Q_v)$. Componentwise decomposition of local conditions gives $\text{Sel}_{mn}(E) \cong \text{Sel}_m(E) \times \text{Sel}_n(E)$. ∎
- Consequence (n=6): $|\text{Sel}_6(E)| = |\text{Sel}_2(E)| \cdot |\text{Sel}_3(E)|$ holds **exactly** for **every** E, independent of the BKLPR model.
- **Honest warning**: This is not a target of BSD itself, and the conclusion that the average $E[|\text{Sel}_6(E)|] = \sigma(6) = 12$ is **CONDITIONAL** on the BKLPR assumption (A3: independence). BSD itself remains unresolved.

---

## 7. BT-547 Poincaré Conjecture — n/φ=3 Classification in 3-dimensional Topology

### 7.1 EXACT Ratio
- **21/21 EXACT** (millennium-learning.json basis)
- Breakdown: 10 existing (Poincaré 1904 / Hopf 1931 / Bott 1959 / Smale 1961 / Thurston 1982 / Hamilton 1982 / Perelman 2003) + 4 from the 2020s (Bamler Ricci flow + Bamler-Kleiner generalized Smale Acta Math + Freedman disk embedding OUP 2021) + 7 on 2026-04-11 (exotic-sphere perfect-number resonance 28/992/8128 + 240 triple crossover + sphere-packing dimensions proved {2, 3, 8, 24} + Berger 7-holonomy + kissing numbers, etc.).
- breakthrough-theorems.md grade is the largest, at "59 items". The 21 in P1 range form the tightest core.

### 7.2 Three Representative EXACT Items
1. **#1 & #2**: Poincaré dimension 3 = n/φ (Poincaré 1904) + Thurston's 8 geometries = σ-τ (Thurston 1982). Two basic classification constants of 3-dimensional topology.
2. **#3**: Stable homotopy $\pi_3^s = \mathbb{Z}/24$ = J₂ (Adams 1966). The order of the 3-stable homotopy group of the sphere is J₂.
3. **#6 & #7**: h-cobordism-theorem applicable lower-bound dimension 5 = sopfr (Smale 1961) + Poincaré-solution ranges in higher dimensions. The fact that "dim ≥ 5 was easy and only dim = 3 remained open" is expressed as the sopfr=5 boundary.

### 7.3 One Truly Tight Item — **Exotic-Sphere Perfect-Number Resonance (|bP_8| = 28 = P_2, |bP_{12}| = 992 = 2·P_3, |bP_{16}| = 8128 = P_4)**
- Selection rationale: **(T1) multi-case + (T4) exceptional structure**. In the Kervaire-Milnor 1963 "Groups of homotopy spheres: I" formula $|bP_{4k}| = 2^{2k-2}(2^{2k-1}-1) \cdot \text{numer}(4B_{2k}/k)$, when $(2^{2k-1}-1)$ is a Mersenne prime the result takes the perfect-number form.
- Perfect-number coincidence occurs in **consecutive** cases $k \in \{2, 3, 4\}$. The Mersenne exponents $p \in \{3, 5, 7\} = \{n/\phi, \text{sopfr}, \sigma-\text{sopfr}\}$ are all n=6 base functions.
- The single fact that the 28 structures of the first exotic sphere S^7 = second perfect number P_2 = 2·n+φ·(n+n/φ) is the strongest evidence of the "perfect-number resonance of topology." The first perfect number n=6 is the foundation of all of BT-541–547, and the second perfect number 28 is the count of the first exotic smooth structures.
- **Honest warning**: millennium-7-closure-2026-04-11.md §BT-547 specifies this as "not an observation but an already-known consequence of Adams' J-homomorphism via Bernoulli computation. Not a new draft." This session is a restatement. 3-dimensional Poincaré is already solved by Perelman 2003 (not this session's contribution). 4-dimensional smooth Poincaré is still unresolved, contribution 0 this session.

---

## 8. Summary Table — Per BT (EXACT ratio, 3 representatives, 1 truly tight)

| BT | Problem | EXACT ratio | 3 representative EXACT items | 1 truly tight item | Grade honesty |
|----|------|------------|----------------|---------------------|-----------|
| 541 | Riemann | 25/26 | ζ(2)=π²/6, ζ(-1)=-1/12, critical line Re(s)=1/φ | **ζ(2k) denom. k∈{1..5} clean + k=n=6 691 boundary** (Theorem B) | PROVEN (session contribution) + OBSERVATION |
| 542 | P vs NP | 12/12 | k-SAT threshold k=3=n/φ, Chomsky 4=τ, Karp 21=(n/φ)·(σ-sopfr) | **Ramsey R(3,3)=6=n** (Greenwood-Gleason 1955) | OBSERVATION only + honest MISS declaration |
| 543 | YM mass gap | 20/20 | SU(3) color 3=n/φ + gluon 8=σ-τ, quark flavors 6=n + β₀=7=σ-sopfr, exceptional-Lie Coxeter 5/5 | **β₀ = σ-sopfr = 7 re-derivation** (SM-parameter assumed) | CONDITIONAL (tautology) |
| 544 | Navier-Stokes | 15/15 | dim Sym²(ℝ³)=6=n, Kolmogorov -5/3=-sopfr/(n/φ), triple resonance | **Triple resonance (stress 6 + vorticity 3 + Onsager 1/3)** at d=3 | OBSERVATION only |
| 545 | Hodge | 25/25 | K3 χ=24=J₂, Enriques h¹·¹=10=σ-φ, Fano 3-fold 105=(n/φ)·sopfr·(σ-sopfr) | **Enriques surface auto-satisfaction (h¹·¹ = 10 all algebraic)** | PROVEN special case (existing classification) |
| 546 | BSD | 25/25 | j(i)=σ³=1728, Mazur torsion 12=σ, value 15 quadruple crossover | **BSD Lemma 1: $\|\text{Sel}_{mn}\|=\|\text{Sel}_m\|\cdot\|\text{Sel}_n\|$** (CRT decomposition, unconditional) | PROVEN (session contribution) + CONDITIONAL (A3 → Sel_6=12) |
| 547 | Poincaré | 21/21 | dimension 3=n/φ, Thurston 8=σ-τ, π_3^s=24=J₂ | **Exotic-sphere perfect-number resonance \|bP_{4k}\|, k∈{2,3,4}** | 3D PROVEN (Perelman, not session's contribution) + OBSERVATION |

**Σ total count**: 143 / 144 EXACT (nominal). But this is a ratio of "n=6 arithmetic-parameterization expression", not a problem-solution rate.

---

## 9. Final Honesty Declaration

This learner has not solved any of the seven Millennium problems BT-541–547. The only rigorous contributions of this session (millennium-learning P1 N6-1) are the following two.

1. **Theorem B** (Bernoulli-numerator k=6 sharp jump) — closes the "n=6 structural framework" of BT-541 Riemann. RH itself remains untouched.
2. **BSD Lemma 1** (Selmer CRT decomposition) — "Sel_{mn} = Sel_m × Sel_n unconditionally" for BT-546 BSD. BSD itself remains untouched.

The remaining 141 EXACT items are at OBSERVATION grade or below. Of those, only the 7 items selected above (one per BT) meet the true-tight criteria (T1–T4); the rest are partially tied to noise in the baseline 61% density.

**Millennium problems solved count: 0/7** (Poincaré solved by Perelman 2003; not this session's contribution).

This document is a close-reading study note. No target claims.

---

**Sources**
- breakthrough-theorems.md §BT-541 – §BT-547
- millennium-7-closure-2026-04-11.md (PROVEN/CONDITIONAL/OBSERVATION classification)
- millennium-n6-attractor-2026-04-11.md (§2 Baseline 61% + T1–T4 tight criteria)
- millennium-learning.json (P1 N6-P1-1 done_criteria)
