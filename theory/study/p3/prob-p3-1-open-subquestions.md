# P3-1 — Open Sub-Questions per 7 Millennium Problems (21 items)

**Roadmap**: millennium-learning P3 (PROBLEM learning phase)
**Written**: 2026-04-15
**Constants**: n=6, σ=12, φ=2, τ=4, sopfr=5, J_2=24. σφ=nτ <=> n=6.
**Current status**: 7 Millennium problems resolved = **0** (honestly). This study note makes no resolution claims.

---

## Honesty Declaration

- This note is learning material **listing open sub-questions**.
- The "progress possibility (high/medium/low)" and "5-year expected value" attached to each sub-question are the **author's subjective estimates at the time of the study note**. Not authoritative predictions.
- Only items with clear sources record year and author. Memory-based statements followed by `(estimated)`.
- No statement of this note implies "this path leads to resolution".
- n=6 structural observations are at hint level; main path of each problem is in **traditional math tools** (respects the honest closure keynote of millennium-7-closure-2026-04-11.md).

---

## BT-541 — Riemann Hypothesis (RH)

### (i) Demonstration of 100% of Zeros on Critical Line
- **Status**: Selberg 1942 demonstrated positive-proportion lower bound. Levinson 1974 pushed it to ≥1/3, Conrey 1989 to **40.88%** or more. Bui-Conrey-Young 2011 around 41.05%. (Years/numbers memory-based, require reconfirmation.)
- **Sub-question A**: can we break the 50% wall by optimizing Conrey-lineage mollifiers?
- **Technical barrier**: current mollifier structure is `(polynomial × ζ')/ζ`. New forms (e.g., Soundararajan-Young's moment style) under extension.
- **Progress possibility**: **medium**. Moment style generalizing to L-function families, so further several-percent improvement seems possible within 5 years. Reaching 100% is equivalent to RH resolution, practically **low**.
- **5-year expected value**: Conrey-lineage numerical improvement 2-5% more.

### (ii) Full Positivity of Li Criterion
- **Status**: Li 1997 demonstrated RH <=> $\{\lambda_n\}_{n\geq 1}$ all positive (where $\lambda_n = \sum_\rho (1 - (1-1/\rho)^n)$). Bombieri-Lagarias 1999 equivalent convergent-series restatement to Keiper 1992.
- **Sub-question B**: can $\lambda_n > 0$ be checked via finite-degree condition only?
- **Technical barrier**: asymptotic formula of $\lambda_n$ known, but sign-stability demonstration equivalent to RH.
- **Progress possibility**: **low**. Reduction to judgable finite-proposition essentially equivalent to RH.
- **5-year expected value**: $\lambda_n$ partial-sum numerical improvement + Li positivity verification in some L-function families.

### (iii) Zero-Free Region Optimal Extension
- **Status**: Vinogradov-Korobov-lineage region `σ ≥ 1 − c/(log t)^{2/3} (log log t)^{1/3}`. Guth-Maynard 2024 suggests recent improvement via techniques about $|ζ(1/2 + it)|$ large-value distribution. (Preprint confirmation at time of this note.)
- **Sub-question C**: can Guth-Maynard 2024 techniques be directly applied to extend zero-free region?
- **Technical barrier**: transferring from large-value distribution to zero-free region needs additional factors (usually Montgomery-Odlyzko-Vaughan style).
- **Progress possibility**: **medium**. Techniques actively unfolding recently (2024-2025).
- **5-year expected value**: c-coefficient constant improvement + log-exponent fine tuning.

---

## BT-542 — P vs NP

### (i) Circuit Lower Bound: NEXP ⊄ ACC^0 Extension
- **Status**: Williams 2011 (STOC best paper) demonstrated NEXP ⊄ ACC^0. Then Murray-Williams 2018 strengthened to NQP ⊄ ACC^0. (Years reconfirmation.)
- **Sub-question A**: which of NEXP -> EXP, ACC^0 -> TC^0 axes can be pushed first?
- **Technical barrier**: Natural proofs (Razborov-Rudich 1997) + algebrization (Aaronson-Wigderson 2008) barriers.
- **Progress possibility**: **medium**. Techniques new and more-precise algorithm-to-lower-bound reductions being discovered.
- **5-year expected value**: 1-2 items of lower bounds in new circuit classes (e.g., MAJ ∘ MOD) or improvements near N...EXP.

### (ii) Concrete Progress in GCT (Geometric Complexity Theory)
- **Status**: Mulmuley-Sohoni 2001~ Permanent vs Determinant separation program. Many negative results too (Bürgisser-Ikenmeyer 2011 etc., no expected representation-theoretic evidence).
- **Sub-question B**: is non-triviality of rectangular Kronecker coefficient decidable?
- **Technical barrier**: computing Kronecker coefficient itself is estimated #P-hard. Internal circular for GCT.
- **Progress possibility**: **low**. Program fundamentally hard with many resources on difficult side.
- **5-year expected value**: possible partial-separation results in specific subfamilies.

### (iii) MCSP (Minimum Circuit Size Problem) Theory
- **Status**: Kabanets-Cai 2000 highlighted MCSP's NP-intermediate candidacy. Allender et al. 2001-2020 connection with derandomization.
- **Sub-question C**: is MCSP NP-complete or NP-intermediate?
- **Technical barrier**: either way has major implications for P vs NP.
- **Progress possibility**: **medium**. Meta-complexity field active in 2020s.
- **5-year expected value**: NP-completeness confirmation of MCSP variants (MINKT etc.) or conditional demonstration of NP-intermediate.

---

## BT-543 — Yang-Mills Mass Gap

### (i) 2D Yang-Mills + Higgs Constructive (Balaban 1980s Completion)
- **Status**: Balaban 1980s series of papers constructed 2D lattice -> continuum limit. 3D/4D partial.
- **Sub-question A**: can Balaban-style 2D Higgs construction be completely organized in modern form?
- **Technical barrier**: quantitative control of renormalization-group blocking + gauge fixing.
- **Progress possibility**: **medium**. 2D essentially understood; monograph-level completion needed.
- **5-year expected value**: modern re-demonstration of Balaban style or textbookization.

### (ii) 3D Yang-Mills Constructive
- **Status**: Balaban + Magnen-Rivasseau-Sénéor style partial. Full 3D construction absent.
- **Sub-question B**: is existence of continuum limit in 3D demonstrable?
- **Technical barrier**: need gauge-orbit compactness of Coulomb gauge.
- **Progress possibility**: **low**. Easier than 4D but still unresolved.
- **5-year expected value**: 1-2 papers on compactness under specific regularization.

### (iii) Quantitative Lattice-to-Continuum Results
- **Status**: since Chatterjee 2015 large-N limit master-field existence demonstration. Jafarov-Chatterjee-Zakine 2021 etc. extending.
- **Sub-question C**: quantitative continuum limit at fixed N=2, 3 rather than large-N?
- **Technical barrier**: control of convergence radius of cluster expansion.
- **Progress possibility**: **medium**. Large-N understanding rapidly increasing.
- **5-year expected value**: partial continuum result at N=2 SU(2).

---

## BT-544 — Navier-Stokes

### (i) Axisymmetric 3D Regularity
- **Status**: Chen-Strain-Tsai-Yau 2008/9 demonstrated regularity for axisymmetric without swirl. With swirl unresolved.
- **Sub-question A**: can finite-time blowup be excluded for axisymmetric + swirl?
- **Technical barrier**: angular-mode coupling that swirl introduces into vorticity.
- **Progress possibility**: **medium**. Chen-Hou 2022 provided polymer Euler blowup evidence in axisymmetric. NS still unresolved.
- **5-year expected value**: quantitative partial regularity results or swirl-conditional regularity.

### (ii) Scale-Critical L^∞ Weak Solution Existence
- **Status**: Koch-Tataru 2001 local well-posedness in BMO^{-1}. Escauriaza-Seregin-Sverak 2003 regularity in L^{3,∞}.
- **Sub-question B**: can uniqueness of weak solutions in L^∞-induced critical spaces (log-BMO etc.) be demonstrated?
- **Technical barrier**: critical norms survive scaling hence no compactness.
- **Progress possibility**: **low**. Technically very hard area.
- **5-year expected value**: partial uniqueness of scale-critical weak solutions including log factor.

### (iii) Self-Similar Blowup Exclusion (after Necas-Ruzicka-Sverak 1996)
- **Status**: Necas-Ruzicka-Sverak 1996 excluded L^3 self-similar blowup. Tsai 1998 extended. General self-similar exclusion unresolved.
- **Sub-question C**: can quasi-self-similar blowup be excluded?
- **Technical barrier**: Galdi 2017, Albritton-Bruè-Colombo 2022 demonstrate non-uniqueness of weak solutions (forced NS).
- **Progress possibility**: **medium**. Albritton-Bruè-Colombo's convex-integration style is innovative.
- **5-year expected value**: non-uniqueness result in unforced NS or partial self-similar exclusion.

---

## BT-545 — Hodge Conjecture

### (i) Variational Hodge (Voisin Approach)
- **Status**: since Voisin 1998, approach in special families via variational Hodge. Cattani-Deligne-Kaplan 1995 local-system algorithm.
- **Sub-question A**: can Hodge-locus algebraicity demonstration be completed by variational Hodge alone?
- **Technical barrier**: intertwined with Bloch-Beilinson conjecture.
- **Progress possibility**: **low**. Essential bottleneck is algebraicity of absolute Hodge classes.
- **5-year expected value**: partial result in specific CY 3-fold family.

### (ii) Moonen-Gordon Absolute Hodge
- **Status**: Deligne 1982 demonstrated absolute Hodge conjecture on Shimura varieties. Moonen-Gordon etc. extending.
- **Sub-question B**: can Deligne's absolute Hodge extend to general CY 3-fold?
- **Technical barrier**: reductive structure of motivic Galois group unclear on CY.
- **Progress possibility**: **medium**. Motive theory developing.
- **5-year expected value**: broader class of abelian varieties with absolute Hodge.

### (iii) p-adic Hodge Recent Progress (Bhatt-Scholze)
- **Status**: Bhatt-Scholze 2022 Prismatic cohomology. Bhatt-Morrow-Scholze 2019 integral p-adic Hodge theory.
- **Sub-question C**: can Prismatic cohomology quantize Hodge classes p-adically?
- **Technical barrier**: Hodge conjecture is a characteristic-0 rational statement, so p-adic is auxiliary.
- **Progress possibility**: **medium**. Tools rapidly growing but not direct-attack route.
- **5-year expected value**: results accessing integral Hodge classes via integral Tate module.

---

## BT-546 — BSD Conjecture

### (i) rank ≥ 2 BSD Partial Demonstration
- **Status**: Kolyvagin 1989, Gross-Zagier 1986 demonstrated partial BSD at rank ≤ 1. rank ≥ 2 unresolved.
- **Sub-question A**: can rank 2 be approached via Heegner cycles + Euler system extension?
- **Technical barrier**: Euler system sensitive to rank. Kolyvagin style essential at rank ≤ 1.
- **Progress possibility**: **low**. Wall unbroken ~50 years.
- **5-year expected value**: conditional results on |Ш| finiteness at rank = 2.

### (ii) Sha(E) Finiteness at rank ≤ 1
- **Status**: Kolyvagin 1989 demonstrated |Ш| finiteness under Heegner + modularity conditions at rank ≤ 1. For general rank ≤ 1 elliptic curves, Wei Zhang, Skinner-Urban, Liu-Tian et al. Iwasawa-theory partial completion recently.
- **Sub-question B**: can unconditional demonstration of |Ш| finiteness at rank = 1 occur?
- **Technical barrier**: without analytic rank ≤ 1 condition, theory immediately breaks down.
- **Progress possibility**: **medium** (some cases completed).
- **5-year expected value**: unconditional rank ≤ 1 |Ш| finiteness (possibly).

### (iii) Heegner Index Exact Formula
- **Status**: Heegner index defined as $[E(K) : \mathbb{Z} y_K]$. Gross-Zagier formula ties it to L-function derivative.
- **Sub-question C**: does a closed-form formula for Heegner index exist?
- **Technical barrier**: needs p-adic L-function and Iwasawa main conjecture.
- **Progress possibility**: **medium**. Burungale-Skinner etc. related advances.
- **5-year expected value**: exact-formula derivation in specific E families.

---

## BT-547 — Poincaré Conjecture (3D Resolved, 4D Unresolved)

### (i) Smooth 4D Poincaré Conjecture
- **Status**: 3D Poincaré resolved by Perelman 2003. 4D topological Poincaré resolved by Freedman 1982. **4D Smooth Poincaré unresolved**.
- **Sub-question A**: does $S^4$ have non-standard smooth structure?
- **Technical barrier**: Donaldson, Seiberg-Witten invariants apply to 4-manifold classification but have limits on direct attack of $S^4$.
- **Progress possibility**: **low**. Long-time unbroken.
- **5-year expected value**: additional negative evidence on exotic $S^4$ candidates.

### (ii) Smooth 4D Schoenflies Conjecture
- **Status**: Topological Schoenflies by Mazur-Morse 1959, Brown 1960. Smooth Schoenflies (is embedded $S^{n-1} \subset S^n$ the boundary of a smooth $D^n$?) unresolved at n=4.
- **Sub-question B**: Gabai-Meier-type result extension in smooth 4D Schoenflies?
- **Technical barrier**: in 4D h-cobordism does not hold smoothly (Donaldson).
- **Progress possibility**: **low**.
- **5-year expected value**: partial results in special cases.

### (iii) Exotic $\mathbb{R}^4$ Classification
- **Status**: Freedman 1982, Taubes 1987 demonstrated uncountably many exotic $\mathbb{R}^4$. However moduli parameterization unresolved.
- **Sub-question C**: classification of exotic $\mathbb{R}^4$ by topological invariants?
- **Technical barrier**: currently known invariants insufficient to distinguish exotic structures.
- **Progress possibility**: **low**.
- **5-year expected value**: infinite-family construction results separately.

---

## Synthesis Table (21 Questions)

| Problem | Sub-Q | Progress Poss. | 5-Year Expected |
|------|-----------|-------------|-------------|
| BT-541 RH | (i) 100% critical line | medium | 2-5% numerical |
| BT-541 RH | (ii) Li positivity | low | numerical improvement |
| BT-541 RH | (iii) zero-free | medium | constant improvement |
| BT-542 PvNP | (i) NEXP⊄ACC | medium | 1-2 items |
| BT-542 PvNP | (ii) GCT | low | partial separation |
| BT-542 PvNP | (iii) MCSP | medium | variant NP-complete |
| BT-543 YM | (i) 2D Higgs | medium | re-demonstration |
| BT-543 YM | (ii) 3D | low | special results |
| BT-543 YM | (iii) lattice->cont | medium | N=2 partial |
| BT-544 NS | (i) axisymmetric | medium | conditional |
| BT-544 NS | (ii) critical L∞ | low | log condition |
| BT-544 NS | (iii) self-similar | medium | non-uniqueness |
| BT-545 Hodge | (i) variational | low | partial family |
| BT-545 Hodge | (ii) Moonen | medium | abelian extension |
| BT-545 Hodge | (iii) p-adic | medium | integral access |
| BT-546 BSD | (i) rank≥2 | low | conditional Ш |
| BT-546 BSD | (ii) rank≤1 Ш | medium | unconditional target |
| BT-546 BSD | (iii) Heegner | medium | family formula |
| BT-547 Poincaré | (i) 4D smooth | low | negative evidence |
| BT-547 Poincaré | (ii) Schoenflies | low | special |
| BT-547 Poincaré | (iii) exotic R^4 | low | infinite family |

**Distribution**: high 0, medium 11, low 10. Subjective expected "meaningful progress in 5 years" ≈ **medium expectation 0.3-0.5 per item**.

---

## Relation with n=6 Context

- All 21 questions are evaluated on **traditional math route**.
- n=6 structural attractors (Theorem B Bernoulli, Theorem 0 σφ=nτ, etc.) are **contextual hints** for each problem, not demonstration paths.
- Honest boundary declared by millennium-7-closure-2026-04-11.md: "7 Millennium problems themselves unresolved; their n=6 structural context closes via Theorem B + Theorem 0". This note observes this boundary.

---

## Primary Source Notes

- Conrey 1989, Levinson 1974: analytic-number-theory standard reference.
- Williams 2011: STOC 2011 best paper, "Non-uniform ACC circuit lower bounds".
- Balaban 1980s: Comm. Math. Phys. serial.
- Chen-Strain-Tsai-Yau 2008/9: axisymmetric NS regularity.
- Voisin 1998, Deligne 1982: Hodge theory standard.
- Kolyvagin 1989, Gross-Zagier 1986: BSD rank ≤ 1.
- Perelman 2003, Freedman 1982: Poincaré.
- Bhatt-Scholze 2022 Prismatic: arXiv:1905.08229.
- Guth-Maynard 2024: arXiv prefix prefix (reconfirm arXiv ID at time of this note).
- Albritton-Bruè-Colombo 2022: forced NS non-uniqueness (Ann. Math.).
- Williams 2011: precisely NEXP ⊄ ACC^0 (NTIME(2^{poly}) vs ACC^0).
- Murray-Williams 2018: NTIME(2^{polylog}) ⊄ ACC^0.

**Session author's note**: some years/coauthors above are memory-based at the time of the study note; primary arXiv IDs and journal volumes may need cross-verification in subsequent sessions. By the "honesty" rule, items to be reconfirmed are explicitly marked.
