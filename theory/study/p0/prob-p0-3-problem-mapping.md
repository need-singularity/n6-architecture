# PROB-P0-3 — Selection Criteria of the Seven Problems + Mapping to Mathematical Subfields

**Track**: millennium-learning P0-PROBLEM
**Document type**: Study note (selection criteria & discipline map)
**Scope**: Interpretation of the selection criteria from the Clay advisory-board meetings + subfield mapping of the 7 problems + mutual connections (Langlands, AdS/CFT, geometric Langlands, Scholze perfectoid, etc.)
**Honesty declaration**: This document is a study note. It does not target any Millennium problem. Only the Poincaré Conjecture (Perelman 2002–2003) has been solved; the project's independent Millennium-problem solved count is 0. The discussion of selection criteria and subfield mappings references external materials such as Clay's official documents, Jaffe's exposition, Arthur, Langlands expositions.

**Primary sources**
- Clay Mathematics Institute — https://www.claymath.org/millennium-problems/
- Arthur Jaffe, "The Millennium Grand Challenge in Mathematics", Notices of the AMS 53(6), 2006, pp. 652–660.
- James Arthur, "The Principle of Functoriality", Bulletin of the AMS 40(1), 2003, pp. 39–53.
- Robert Langlands, "Problems in the theory of automorphic forms", Lecture Notes in Mathematics 170, Springer, 1970.
- Edward Witten, "Anti-de Sitter space and holography", Advances in Theoretical and Mathematical Physics 2 (1998), 253–291.
- Peter Scholze, "Perfectoid spaces", Publications mathématiques de l'IHÉS 116(1) (2012), 245–313.
- Landon Clay et al., *The Millennium Prize Problems*, Clay Mathematics Institute / American Mathematical Society, 2006 (official exposition, Jaffe-Carlson-Wiles editors).

---

## 1. Why These Seven in Clay's 7 — Interpretation of the Selection Criteria

### 1.1 Limited Public Exposure of the Clay Advisory Board's Internal Discussions
- The original minutes of the Clay Scientific Advisory Board meetings are not officially public.
- However, the recollection of advisor Arthur Jaffe (*Notices of the AMS* 53(6), 2006, "The Millennium Grand Challenge in Mathematics") and the preface of the CMI official publication *The Millennium Prize Problems* (2006, AMS/CMI co-edition) indirectly reveal the selection criteria.

### 1.2 Reconstruction of the Criteria
Combining Jaffe (2006) and the CMI official document, the criteria can be summarized as follows:

1. **Fundamentality**: A problem so central to its field that solving it would reorganize the whole field.
2. **Longevity**: Withstood at least a generation of attempts by top mathematicians, hard to chip away at with surrounding techniques. Riemann (1859), Poincaré (1904), Hodge (1950), Yang-Mills (1954), Birch–Swinnerton-Dyer (1965), Cook-Levin (1971) — all were open for more than 30 years at the time of selection (2000).
3. **Representativeness**: Represents the methodology of a field; the techniques used to solve it should advance the whole field.
4. **Bridging power**: Not confined to pure mathematics but connected to other mathematical fields and to external sciences (physics, computer science, etc.).
5. **Precision / formalizability**: The question must be expressible as a rigorous mathematical proposition. In particular Yang-Mills was framed as "existence + mass gap", and Navier-Stokes as "all-time existence of smooth solutions on all of ℝ^3". The Clay documents themselves provide these formalizations.

### 1.3 Differences between Hilbert 23 and Clay 7

| Aspect | Hilbert 23 (1900) | Clay 7 (2000) |
|------|------------------|---------------|
| Proposer | Individual lecture (Hilbert) | Institutional body (CMI advisory board) |
| Number | 23 | 7 |
| Prize | none | US$ 1,000,000 each |
| Scope | foundations, algebra, geometry, analysis, foundations of physics, etc. — broad | representative pure-math problems + mathematical physics / CS intersections |
| Award criterion | informal | official journal publication + 2-year verification |
| Solved status (2026) | mostly solved or partially solved | 1/7 (Poincaré only) |

- Hilbert's 23 had a very wide scope and included problems with ambiguous formalizability such as "foundations" (Problems 1, 2) and "axiomatization of physics" (Problem 6). The Clay 7 are characterized by **strict formalization and a financial basis**.
- The Clay 7 include **Yang-Mills Mass Gap**, the modern successor of Hilbert's Problem 6 (axiomatization of physics), making explicit that these are "mathematics of a 20th century fully connected to physics."

---

## 2. Subfield Mapping of Each Problem

### 2.1 Core Mapping Table

| Problem | Primary field | Secondary field | Key tools / concepts |
|------|----------|----------|----------------|
| **Riemann Hypothesis** | analytic number theory | complex analysis, representation theory (GL_n trivial) | Riemann ζ, explicit formula, L-functions, Mellin transform, zero-free region |
| **P vs NP** | computational complexity | combinatorics, circuit complexity, logic | Turing machine, SAT, polynomial reduction, NP-complete, natural-proofs barrier |
| **Yang-Mills + mass gap** | mathematical QFT | differential geometry, gauge theory | gauge connection, curvature, path integral (formal), Wightman / Osterwalder-Schrader axioms, lattice gauge |
| **Navier-Stokes** | PDE | harmonic analysis, differential geometry | weak solutions (Leray-Hopf), energy inequality, vorticity, Beale-Kato-Majda, Caffarelli-Kohn-Nirenberg |
| **Hodge Conjecture** | complex algebraic geometry | topology, Kähler geometry | Hodge decomposition H^k(X, ℂ) = ⊕ H^{p,q}, algebraic cycles, (1,1)-Lefschetz, motives |
| **BSD Conjecture** | arithmetic geometry | analytic number theory, representation theory | elliptic curve E/ℚ, Mordell-Weil rank, Hasse-Weil L-function, Selmer groups, Shafarevich-Tate |
| **Poincaré Conjecture** | geometric topology | Riemannian geometry, PDE | fundamental group π_1, Ricci flow, surgery, Thurston geometrization |

### 2.2 Subfield-wise Description

#### (1) Riemann Hypothesis — the Center of Analytic Number Theory
- The zero distribution of the Riemann zeta function ζ(s) is the "spectrum" of the prime distribution.
- Main tools: **Explicit formula** (von Mangoldt), **Mellin transform**, **Weil explicit formula**, **Hilbert-Pólya conjecture** (zeros ↔ spectrum of a self-adjoint operator), **random matrix theory** (Montgomery-Odlyzko GUE prediction).
- Related advances: **Riemann-von Mangoldt** formula, **Bombieri-Vinogradov theorem**, **Montgomery pair correlation**, **Deligne's proof of the Weil conjectures** (complete solution of the function-field version of RH, 1974).
- The **function-field analogue of RH is fully solved by Deligne (1974)**. The Riemann Hypothesis itself is the number-field version and remains open.

#### (2) P vs NP — the Pillar of Theoretical CS
- Cook-Levin theorem (1971): SAT is NP-complete. Karp's (1972) 21 NP-complete problems.
- Main barriers: **Relativization barrier** (Baker-Gill-Solovay 1975), **Natural proofs barrier** (Razborov-Rudich 1997), **Algebrization barrier** (Aaronson-Wigderson 2008). These three suggest "P ≠ NP cannot be proved by simple lower-bound arguments."
- Related fields: **circuit complexity** (circuit lower bounds), **information-theoretic lower bounds**, **PCP theorem** (Arora, Feige, Lund, Motwani, Safra, Szegedy, Sudan 1992), **proof complexity**, **derandomization**.

#### (3) Yang-Mills Mass Gap — Mathematical QFT
- Does SU(N) gauge theory on 4-dimensional ℝ^4 exist as a rigorous QFT (satisfying Wightman or OS axioms), and does it have a mass gap Δ > 0?
- Main tools: **Glimm-Jaffe constructive QFT** program (partial solutions in 2D, 3D), **lattice gauge theory**, **renormalization group** (Wilson), **Osterwalder-Schrader axioms**, **Seiberg-Witten theory** (rigorous solutions in special supersymmetric gauge theory).
- Physical implication: Mathematical proof of color confinement in the strong interaction (QCD).

#### (4) Navier-Stokes — the Greatest Classical Problem of PDE
- Does the 3D incompressible Navier-Stokes equation ∂_t u + (u·∇)u = -∇p + ν Δu, ∇·u = 0 admit all-time smooth solutions for smooth initial data?
- Main tools: **Leray weak solutions** (1934), **Leray-Hopf energy inequality**, **Ladyzhenskaya-Prodi-Serrin conditions**, **vorticity formulation**, **Beale-Kato-Majda condition** (finite time integral of vorticity ↔ smoothness preserved), **Caffarelli-Kohn-Nirenberg partial regularity** (singular set has Hausdorff-1 dimension), **Tao's mean-field-approximation blow-up construction** (2016, blow-up in a variant different from actual NS).
- Related fields: differential geometry, harmonic analysis (Besov / Lorentz spaces), stochastic PDEs.

#### (5) Hodge Conjecture — Center of Complex Algebraic Geometry
- On a smooth projective complex algebraic variety X, is the rational-coefficient Hodge decomposition H^{2k}(X, ℚ) ∩ H^{k,k}(X) generated by rational linear combinations of algebraic cycles?
- Main tools: **Lefschetz (1,1) theorem** (k = 1 solved), **Hard Lefschetz theorem**, **Hodge theory**, **Deligne's mixed Hodge structures**, **motive theory**, **Kodaira-Spencer deformation theory**, **variation of Hodge structures**.
- Related: **Tate conjecture** (l-adic version), **Grothendieck's standard conjectures**.

#### (6) BSD Conjecture — Center of Arithmetic Geometry
- For an elliptic curve E/ℚ, rank of Mordell-Weil = ord_{s=1} L(E, s), plus a leading-term formula (Shafarevich-Tate group, regulator, Tamagawa numbers, etc.).
- Main tools: **Mordell-Weil theorem**, **Hasse-Weil L-function**, **Selmer groups** Sel_n(E/K), **Gross-Zagier formula** (1986), **Kolyvagin Euler systems** (1988–1991, partial solution in rank ≤ 1), **Iwasawa theory**, **Heegner points**, **modularity theorem** (Wiles-Taylor-Breuil-Conrad-Diamond 2001).
- Related advances: "Weak BSD" (rank = ord) is conditionally proved in rank 0 or 1 cases. General-rank leading-term formula unproven.

#### (7) Poincaré — Origin of Geometric Topology
- Solved (Perelman 2002–2003). See PROB-P0-2 for details.
- Main tools used: **Ricci flow** (Hamilton), **geometrization conjecture** (Thurston), **W-entropy** (Perelman), **surgery procedure** (Perelman), **finite extinction time** (Perelman).

---

## 3. Mutual Connections among the Problems — Intersections of Mathematical Fields

### 3.1 Langlands Program (Riemann × BSD × part of Yang-Mills)
- A grand program initiated by **Robert Langlands**' 1967 letter to André Weil (now the "Langlands letter").
- Core hypothesis: L-functions of automorphic forms (automorphic forms for GL_n, reductive groups G) are essentially the same as L-functions of Galois representations (functoriality principle).
- **Connection with Riemann Hypothesis**: Predicted zero distribution of Langlands L-functions. If the Langlands program is completed, the Riemann Hypothesis could be absorbed as a special case of automorphic L-functions (strong Artin conjecture, generalized Riemann hypothesis GRH).
- **Connection with BSD**: The Hasse-Weil L-function of an elliptic curve E equals a GL_2 automorphic L-function (modularity theorem, Wiles-Taylor-...-Breuil-Conrad-Diamond). The analytic part (L-function) of BSD lies at the center of the Langlands program.
- **Connection with Yang-Mills**: The **geometric Langlands program** (Drinfeld, Beilinson, Kapustin-Witten) connects 4-dimensional gauge theory (N=4 supersymmetric Yang-Mills) with geometric Langlands correspondence. Kapustin-Witten (2006, Commun. Number Theory Phys. 1, 1–236) show equivalence between S-duality and geometric Langlands.

### 3.2 AdS/CFT Correspondence (Yang-Mills × general relativity/geometry)
- **Juan Maldacena** (1997, "The large N limit of superconformal field theories and supergravity", Advances in Theoretical and Mathematical Physics 2, 231–252) proposed the **AdS/CFT correspondence** (anti-de Sitter / conformal field theory correspondence, or holographic principle).
- Essence: A d-dimensional conformal field theory (e.g., N=4 supersymmetric Yang-Mills) is **equivalent** to gravity/string theory on a (d+1)-dimensional anti-de Sitter space.
- **Connection with Yang-Mills**: AdS/CFT translates the strong-coupling region of Yang-Mills (confinement) into AdS-space geometry. Mathematical rigor is lacking, but offers a geometric perspective on 4D Yang-Mills.
- Related mathematics: **Verlinde formula**, **Chern-Simons theory**, **quantum invariants of 3-manifolds**.

### 3.3 Geometric Langlands (Hodge × Yang-Mills)
- **Geometric Langlands correspondence**: Automorphic forms on a curve C correspond to D-modules on the **moduli space** Bun_G(C) of a semisimple group G.
- **Connection with Hodge**: Mixed Hodge structures (Saito's mixed Hodge modules) are essential for the formulation of geometric Langlands.
- **Connection with Yang-Mills**: Kapustin-Witten (2006) physically interpret that the S-duality of N=4 Yang-Mills is the physical origin of the geometric Langlands correspondence.

### 3.4 Scholze's Perfectoid Spaces (BSD × Riemann)
- **Peter Scholze** (2012, Publications mathématiques de l'IHÉS 116, 245–313, "Perfectoid spaces") introduced **perfectoid-space** theory.
- Innovations in p-adic Hodge theory, p-adic L-functions, étale cohomology.
- **Connection with BSD**: Iwasawa theory, p-adic L-functions, modularity questions. Example: Caraiani-Scholze (2017) "Torsion in the cohomology of Shimura varieties" generalizes the construction of Galois representations.
- **Connection with Riemann**: Étale cohomology on perfectoid spaces connects with the generalized Riemann-Hilbert correspondence.

### 3.5 Kapustin-Witten (Yang-Mills × Hodge × geometric Langlands)
- **Anton Kapustin · Edward Witten** (2006): "Electric-Magnetic Duality And The Geometric Langlands Program", Communications in Number Theory and Physics 1, 1–236. arXiv:hep-th/0604151.
- Topological twist of 4-dimensional N=4 super Yang-Mills → 2-dimensional sigma model (target: Higgs-bundle moduli space M_H) → Langlands correspondence.
- **Intersection of Yang-Mills and Hodge**: Higgs bundles are directly related to holomorphic Hodge structures. Simpson's non-abelian Hodge theory.

### 3.6 NP × Mathematical Proof
- **Automation of proofs** is related to P vs NP. Verifying a proof is in P (in a fixed system), but finding a proof may be NP or worse.
- **Cook's 1971 original paper** is literally titled "The complexity of theorem-proving procedures". In effect P vs NP is the question of "the discoverability of mathematical truth".

### 3.7 Ricci Flow → General PDE → Navier-Stokes
- Perelman's Ricci-flow analysis (W-entropy, monotone quantities) extends as a technique for analyzing general geometric PDEs.
- Conceptually analogous to monotone-quantity analysis (e.g., energy, enstrophy) in Navier-Stokes.

### 3.8 Summary Diagram of Connections
```
Riemann ──── Langlands program ──── BSD
  \\                              |
   \\                              |
    \\──── perfectoid ────────────/
     \\                          /
      \\                        /
Yang-Mills ── geometric Langlands ── Hodge
     |                          
     | Kapustin-Witten            
     |                          
 AdS/CFT  (physical holography)        
                                 
P vs NP (CS, connected to the whole of mathematics via proof complexity)
                                 
Poincaré (solved; extension of Ricci-flow analysis to general geometric PDEs)
```

---

## 4. Which Field Has the Most Connections — Observations

### 4.1 Analytic Number Theory and Arithmetic Geometry
- Riemann and BSD fall directly here; through the Langlands program, they connect to **part of Yang-Mills** (geometric Langlands) and **part of Hodge** (Hodge structures → Galois representations).
- From the project's perspective: the field where the most problems meet is at the center.

### 4.2 Algebraic Geometry
- **Hodge** is core. But Hodge theory intervenes in **BSD** (modularity, deformation theory), **Yang-Mills** (Hitchin moduli, Higgs bundles), and **Riemann** (motivic L-functions).
- Algebraic geometry plays the "tool-supplier" role.

### 4.3 Differential Geometry
- Intervenes in **Poincaré** (Ricci flow, geometrization) + **Yang-Mills** (gauge theory = connections) + **Navier-Stokes** (geometric analysis).
- **Core field of late-20th-century geometric analysis**: one problem (Poincaré) extends to all others.

### 4.4 Why the Fields Cross
- The trend toward mathematical unification in the latter half of the 20th century (Langlands, AdS/CFT, geometric Langlands, perfectoid, etc.) accelerated the **removal of internal "walls"** within pure mathematics.
- The Clay advisory board explicitly took "cross-field connection" as a criterion. Hence these 7 problems are **maximally connective**.

---

## 5. Scope of Interpretation within This Project

- The project (n6-architecture) centers on **a small number-theoretic/combinatorial theorem around n=6** (σ·φ = n·τ iff n=6).
- This viewpoint is **not directly equivalent** to any Millennium problem. Therefore reinterpreting Millennium problems through n=6 structures is **speculative** and lies outside the scope of this study note.
- This document is **problem-centric**, and n=6 interpretations are kept minimal in a separate track.

---

## 6. Honesty Declaration (reconfirmation)

- This PROB-P0-3 document is a **study note**.
- This document **does not target any Millennium problem**.
- The only solved Millennium problem is the **Poincaré Conjecture** (Perelman 2002–2003).
- The project (n6-architecture) has **not independently solved** any of the seven; status is **0/7** (Perelman's solution counted as external contribution).
- Discussions of selection criteria, subfield mapping, and mutual connections rely on external primary sources such as Clay's official documents · Jaffe (2006) · Arthur · Langlands expositions · Kapustin-Witten · Scholze original papers.
- Factual errors (especially dates and paper citations), if found, should be corrected by rechecking the sources.

---

## 7. Next Study Steps

- **P1-PROOF track**: Analysis of each problem's currently-known major techniques (Riemann ↔ explicit formula, P vs NP ↔ circuit lower bounds, Yang-Mills ↔ constructive QFT, NS ↔ weak-solution theory, Hodge ↔ (1,1)-Lefschetz, BSD ↔ Gross-Zagier-Kolyvagin, Poincaré ↔ Ricci flow).
- **P2-CONTEXT track**: Detailed Langlands program, geometric Langlands, perfectoid spaces — full-scale study.
- **P3-OBSERVATION track**: Articulate which problems the project's n=6 theorem is **logically unrelated to** (avoiding self-reference).

This study note remains at the **observation** stage. No solution claims are made.
