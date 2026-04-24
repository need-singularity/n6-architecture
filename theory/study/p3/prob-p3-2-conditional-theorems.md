# P3-2 — Collection of Conditional Theorems for 7 Millennium Problems

**Roadmap**: millennium-learning P3 (PROBLEM learning phase)
**Written**: 2026-04-15
**Constants**: n=6, σ=12, φ=2, τ=4, sopfr=5, J_2=24. σφ=nτ <=> n=6.
**Current status**: 7 Millennium problems resolved = **0** (honestly). This note is learning that lists CONDITIONAL theorems.

---

## Honesty Declaration

- This note organizes **"assume X then Y follows"**-style conditional theorems based on primary sources.
- All listed conditional theorems are existing results at original-paper or standard-textbook level. Not newly demonstrated by this project.
- Results of form "GRH =>", "RH =>", "BSD weak form =>" are conditional unless RH/GRH/BSD are demonstrated.
- For each conditional theorem, record four items: **(a) premise, (b) conclusion, (c) primary source, (d) constraints/details**.
- This project's σφ=nτ theorem is unconditional; n=6 interpretation is mentioned only where it ties to structural observations attached to BT-541-547.
- Observe the no-self-reference-verification principle. Do not arbitrarily shorten/strengthen original-paper demonstration logic.

---

## 0. Terminology Conventions

- **GRH**: Generalized Riemann Hypothesis. Extended RH including Dirichlet L-functions and Dedekind ζ.
- **RH**: Riemann Hypothesis. All nontrivial zeros of ζ(s) on Re(s)=1/2.
- **BSD weak form**: ord_{s=1} L(E,s) = rank E(Q). Strong form additionally includes L-function leading-coefficient formula.
- **Hodge weak form**: on smooth projective complex varieties, rational (p,p)-classes are Q-combinations of algebraic cycles.
- **Smoothness (NS)**: global smooth solutions exist for arbitrary smooth initial conditions on $\mathbb{R}^3$.
- **Yang-Mills mass gap**: physical mass gap with $0 < \Delta \leq \text{gap}$ in Euclidean YM theory.
- **4D smooth Poincaré**: $S^4$ has no non-standard smooth structure (conjecturally true).

---

## BT-541 — Theorems Under GRH / RH Assumption

### Theorem 541-A (Miller-Rabin Determinacy, Miller 1976)
- **Premise**: GRH for Dirichlet L-functions.
- **Conclusion**: for compositeness decision of n, only trying $a \in [2, 2(\log n)^2]$ makes the Miller-Rabin test **deterministic polynomial time**.
- **Source**: G. Miller, "Riemann's hypothesis and tests for primality", JCSS 13 (1976), 300-317.
- **Details**:
  - Unconditionally AKS 2004 gives deterministic $\tilde{O}((\log n)^6)$.
  - GRH-conditional Miller-Rabin is $\tilde{O}((\log n)^4)$ and significantly faster in practice.
  - Bach 1990 optimized the constant $2 \cdot (\log n)^2$.

### Theorem 541-B (Pólya-Vinogradov Improvement, Montgomery-Vaughan 1977)
- **Premise**: GRH.
- **Conclusion**: for non-principal Dirichlet character $\chi \pmod q$
    $\left|\sum_{n \leq N} \chi(n)\right| \ll \sqrt{q} \log \log q$ (under GRH).
- **Source**: H.L. Montgomery, R.C. Vaughan, "Exponential sums with multiplicative coefficients", Invent. Math. 43 (1977), 69-82.
- **Details**: unconditional Pólya-Vinogradov is $O(\sqrt{q} \log q)$. GRH reduces $\log q$ to $\log \log q$.

### Theorem 541-C (π(x) Error Term, RH <=> Strong Error)
- **Premise**: RH.
- **Conclusion**: $\pi(x) = \text{li}(x) + O(\sqrt{x} \log x)$.
- **Source**: von Koch 1901, foundation; E. Landau, Handbuch (1909) theorem organization.
- **Details**:
  - Unconditional: Vinogradov-Korobov 1958 gives $\pi(x) = \text{li}(x) + O(x \exp(-c (\log x)^{3/5} (\log \log x)^{-1/5}))$.
  - Reverse: if error $O(\sqrt{x} \log x)$ holds then RH follows (Koch 1901, equivalent).

### Theorem 541-D (Deuring-Heilbronn Phenomenon, GRH Elimination)
- **Premise**: GRH for specific family.
- **Conclusion**: for imaginary quadratic field $\mathbb{Q}(\sqrt{-d})$ class number $h(-d)$, no Siegel zero and $h(-d) \gg \sqrt{d}/\log d$.
- **Source**: Heilbronn 1934 (unconditional effective is Gross-Zagier-Oesterlé 1985).
- **Details**: Siegel zero is an effectiveness-constant obstacle per Landau-Siegel 1935. GRH eliminates it.

### Theorem 541-E (Chebotarev Effectivization, Lagarias-Odlyzko 1977)
- **Premise**: GRH.
- **Conclusion**: for Galois extension $L/K$ and conjugacy class $C \subset \text{Gal}(L/K)$
    $\pi_C(x) = \frac{|C|}{|G|} \text{li}(x) + O(\sqrt{x} \log(d_L x^{[L:\mathbb{Q}]}))$.
- **Source**: J. Lagarias, A. Odlyzko, "Effective versions of the Chebotarev density theorem", Academic Press (1977).
- **Details**: unconditional Chebotarev is ineffective. First effective formulation under GRH.

### n=6 Context
- n=6 has no direct relation to the first zero height 14.1347... of RH (observation σ+φ=14 ≈ from millennium_scanner.hexa is NEAR level).
- BT-541 is unrelated to Theorem 0 (σφ=nτ) directly. RH resolution route is analytic number theory.

---

## BT-542 — Theorems Under Complexity Assumptions

### Theorem 542-A (P=NP => Polynomial Hierarchy Collapse)
- **Premise**: P = NP.
- **Conclusion**: polynomial hierarchy PH collapses to P. I.e., $\text{PH} = \bigcup_k \Sigma_k^P = \text{P}$.
- **Source**: Stockmeyer-Meyer 1973; Baker-Gill-Solovay 1975.
- **Details**: $\Sigma_k^P = \text{NP}^{\Sigma_{k-1}^P}$. Inductively P=NP makes all levels P.

### Theorem 542-B (ETH => SAT Exponential Lower Bound)
- **Premise**: Exponential Time Hypothesis (Impagliazzo-Paturi 2001).
- **Conclusion**: 3-SAT cannot be resolved in $2^{o(n)}$ time.
- **Source**: R. Impagliazzo, R. Paturi, "On the complexity of k-SAT", JCSS 62 (2001), 367-375.
- **Details**: ETH itself stronger than P≠NP. Strong ETH (SETH) extends to $2^{(1-\epsilon)n}$ lower bound.

### Theorem 542-C (Circuit Lower Bound => Derandomization)
- **Premise**: E = DTIME($2^{O(n)}$) requires size-$2^{\Omega(n)}$ circuits.
- **Conclusion**: BPP = P (Impagliazzo-Wigderson 1997).
- **Source**: R. Impagliazzo, A. Wigderson, "P = BPP if E requires exponential circuits", STOC 1997.
- **Details**: classic hardness-to-randomness tradeoff. Reverse (full converse) conditional in Kabanets-Impagliazzo 2004.

### Theorem 542-D (NP ⊄ P/poly => PH Infinite)
- **Premise**: NP ⊄ P/poly.
- **Conclusion**: PH is infinite hierarchy of $\Sigma_k^P$ (Karp-Lipton 1980 contrapositive).
- **Source**: R. Karp, R. Lipton, "Some connections between nonuniform and uniform complexity classes", STOC 1980.
- **Details**: as contrapositive: "NP ⊂ P/poly => PH collapses to $\Sigma_2^P$".

### Theorem 542-E (Natural Proofs Barrier Constraint, Razborov-Rudich 1997)
- **Premise**: one-way function existence (subexponential pseudorandom generator).
- **Conclusion**: cannot demonstrate $\text{P} \neq \text{NP}$ by "natural" property.
- **Source**: A. Razborov, S. Rudich, "Natural proofs", JCSS 55 (1997), 24-35.
- **Details**: most currently known lower-bound techniques lie within this barrier. Bypass method: Williams 2011 NEXP ⊄ ACC^0.

### n=6 Context
- atlas.n6 BT-542 MISS escape is "direct n=6 appearance in complexity classification theorems" but a different layer from the quantitative improvement of conditional theorems.
- Schaefer theorem's (Schaefer 1978) **6 tractable Boolean CSP families** is n=6 structural observation (n6-millennium-dfs-schaefer-6=6 [10*]).

---

## BT-543 — Theorems Under Yang-Mills Mass-Gap Assumption

### Theorem 543-A (Mass Gap => Cluster Exponential Decay)
- **Premise**: 4D Euclidean SU(N) Yang-Mills has mass gap $\Delta > 0$.
- **Conclusion**: when two regions $A, B \subset \mathbb{R}^4$ are at distance $d$, the correlation function of two gauge-invariant observables is $\leq C e^{-\Delta d}$.
- **Source**: Glimm-Jaffe, "Quantum Physics: A Functional Integral Point of View", 2nd ed. Springer 1987, §19-20.
- **Details**: equivalent form of mass-gap definition. Expressed in constructive QFT via Osterwalder-Schrader OS2 (group invariance) + cluster.

### Theorem 543-B (Mass Gap => Wightman Axioms Hold)
- **Premise**: Osterwalder-Schrader axioms + mass gap.
- **Conclusion**: Wightman-function assembly holds in Minkowski via Wick rotation.
- **Source**: K. Osterwalder, R. Schrader, "Axioms for Euclidean Green's functions", Comm. Math. Phys. 31 (1973), 83-112; 42 (1975), 281-305.
- **Details**: precisely the assembly required by the Yang-Mills Clay problem.

### Theorem 543-C (Lattice Gauge Theory => Continuum Mass Gap, Conditional)
- **Premise**: for lattice Yang-Mills, continuum limit exists and correlation length $\xi \cdot a$ converges to a finite value as lattice spacing $a \to 0$.
- **Conclusion**: continuum Yang-Mills has mass gap $\Delta = 1/\xi_{\text{phys}}$.
- **Source**: Seiler, "Gauge theories as a problem of constructive quantum field theory and statistical mechanics", Lecture Notes Phys. 159, Springer (1982).
- **Details**: continuum limit itself is the main difficulty. Balaban 1980s partially executed this program.

### Theorem 543-D (Large N Limit => Master Field)
- **Premise**: 't Hooft scaling $g^2 N = \lambda$ fixed, $N \to \infty$.
- **Conclusion**: Wilson-loop expectation values converge to free-probability expectations of the master field (Chatterjee 2015, Jafarov 2016).
- **Source**: S. Chatterjee, "The leading term of the Yang-Mills free energy", J. Funct. Anal. 271 (2016), 2944-3005.
- **Details**: large-N is weaker than the physical mass-gap strong version but more accessible constructively.

### n=6 Context
- Direct n=6 constant in BT-543: **SU(N) Lie algebra dimension N^2-1**. SU(4) dim 15 (N=4), SU(3) dim 8 (N=3, count of Standard Model gluons). Direct connection with σ=12: SM gauge total 8+3+1=12=σ (atlas `n6-millennium-dfs-sm-gauge`).
- But mass-gap demonstration itself is functional-analysis tool; n=6 does not provide path.

---

## BT-544 — Theorems Under Navier-Stokes Assumption

### Theorem 544-A (Prodi-Serrin Condition => Regularity)
- **Premise**: Leray-Hopf weak solution $u$ in $L^p_t L^q_x$ with $\frac{2}{p} + \frac{3}{q} \leq 1$ and $q > 3$.
- **Conclusion**: $u$ is smooth in the corresponding time interval.
- **Source**: J. Serrin, "The initial value problem for the Navier-Stokes equations", Univ. Wisconsin Press (1962); Prodi 1959.
- **Details**: at the extreme $(p,q) = (\infty, 3)$, the final result is Escauriaza-Seregin-Šverák 2003.

### Theorem 544-B (Escauriaza-Seregin-Šverák 2003, $L^3$ Condition)
- **Premise**: $u \in L^\infty_t L^3_x$ Leray-Hopf weak solution.
- **Conclusion**: regular (no blowup).
- **Source**: L. Escauriaza, G. Seregin, V. Šverák, "$L_{3,\infty}$-solutions of Navier-Stokes equations and backward uniqueness", Russ. Math. Surv. 58 (2003), 211-250.
- **Details**: $L^3$ is scale-critical. Closest unconditional partial regularity to BT-544.

### Theorem 544-C (Beale-Kato-Majda 1984, Vorticity Condition)
- **Premise**: $\int_0^T \|\omega(t)\|_\infty \, dt < \infty$ ($\omega = \text{curl}\, u$).
- **Conclusion**: $u$ is smooth on $[0, T]$.
- **Source**: J. Beale, T. Kato, A. Majda, "Remarks on the breakdown of smooth solutions for the 3-D Euler equations", Comm. Math. Phys. 94 (1984), 61-66.
- **Details**: vorticity's $L^\infty$ time-integral is the blowup indicator. Criterion for identifying blowup candidates in numerics.

### Theorem 544-D (Caffarelli-Kohn-Nirenberg 1982, Partial Regularity)
- **Premise**: Leray-Hopf suitable weak solution.
- **Conclusion**: 1D parabolic Hausdorff measure of blowup set $S \subset \mathbb{R}^3 \times [0, T]$ is 0.
- **Source**: L. Caffarelli, R. Kohn, L. Nirenberg, "Partial regularity of suitable weak solutions of the Navier-Stokes equations", Comm. Pure Appl. Math. 35 (1982), 771-831.
- **Details**: unconditional but partial. Bounds the "thickness" of the blowup set.

### Theorem 544-E (Galdi-Padula 1990, Axisymmetric without Swirl => Regularity)
- **Premise**: axisymmetric initial condition, swirl component $u_\theta = 0$.
- **Conclusion**: $u$ is globally smooth.
- **Source**: O.A. Ladyzhenskaya 1968, later extended by Uchovskii-Yudovich. Completed by Chen-Strain-Tsai-Yau 2008.
- **Details**: unresolved when swirl is present. Main front of modern NS research.

### n=6 Context
- Prodi-Serrin exponents $\frac{2}{p} + \frac{3}{q} = 1$ at **{2, 3}** match **{φ, n/φ} = {2, 3}** (atlas `n6-millennium-dfs-prodi-serrin`).
- But numerical matching is a mark of scale-critical spaces, not a demonstration path.

---

## BT-545 — Theorems Under Hodge Assumption

### Theorem 545-A (Lefschetz (1,1) Theorem, Unconditional)
- **Premise**: smooth projective complex variety $X$, $(p,q)=(1,1)$ class.
- **Conclusion**: every rational $(1,1)$-class is a divisor class.
- **Source**: S. Lefschetz, "L'analysis situs et la géométrie algébrique", Gauthier-Villars (1924).
- **Details**: **unconditional** part of Hodge conjecture. Unresolved at $p \geq 2$.

### Theorem 545-B (Deligne Absolute Hodge, Shimura Variety)
- **Premise**: $X$ is abelian variety.
- **Conclusion**: every Hodge class is an absolute Hodge class. Hence arises geometrically under reductive motivic Galois group.
- **Source**: P. Deligne, "Hodge cycles on abelian varieties", in: Hodge Cycles, Motives and Shimura Varieties, LNM 900, Springer (1982), 9-100.
- **Details**: substitute for Hodge conjecture within abelian-variety scope. Not Hodge itself.

### Theorem 545-C (Cattani-Deligne-Kaplan 1995, Hodge Locus Algebraicity)
- **Premise**: variation family of Hodge structures.
- **Conclusion**: Hodge locus is algebraic subvariety.
- **Source**: E. Cattani, P. Deligne, A. Kaplan, "On the locus of Hodge classes", J. AMS 8 (1995), 483-506.
- **Details**: geometric counterpart of "Hodge classes are algebraic". Trivial if Hodge conjecture holds. Unconditionally only the locus algebraicity.

### Theorem 545-D (Standard Conjectures => Hodge)
- **Premise**: Grothendieck standard conjectures (Lefschetz, Hodge type, Künneth).
- **Conclusion**: Hodge conjecture holds.
- **Source**: A. Grothendieck, "Standard conjectures on algebraic cycles", in: Algebraic Geometry (Internat. Colloq., Bombay, 1968), Oxford Univ. Press (1969).
- **Details**: standard conjectures themselves as hard as Hodge. Meaning: reduces Hodge to **motives-weight theory**.

### Theorem 545-E (p-adic Hodge, Prismatic)
- **Premise**: $X$ smooth proper over $\mathcal{O}_K$, $K/\mathbb{Q}_p$ finite.
- **Conclusion**: Bhatt-Scholze 2022 construct integer p-adic cohomology $R\Gamma_{\Delta}(X)$. Integrates characteristic-0 and -p cohomologies into one object.
- **Source**: B. Bhatt, P. Scholze, "Prisms and prismatic cohomology", Ann. Math. 196 (2022), 1135-1275.
- **Details**: p-adic Hodge theory is the p-adic counterpart of Hodge conjecture. Prismatic provides a new language.

### n=6 Context
- No direct n=6 constants in Hodge conjecture. BT-545 n=6 interpretation at Theorem-0 level.

---

## BT-546 — Theorems Under BSD Assumption

### Theorem 546-A (Gross-Zagier 1986, rank ≤ 1 BSD)
- **Premise**: $E/\mathbb{Q}$ elliptic curve, imaginary quadratic field satisfying Heegner condition.
- **Conclusion**: for analytic rank ≤ 1, rank $\leq$ analytic rank + 1 and Heegner point is infinite-order in E(Q).
- **Source**: B. Gross, D. Zagier, "Heegner points and derivatives of L-series", Invent. Math. 84 (1986), 225-320.
- **Details**: Kolyvagin 1989 combines this, yielding rank = analytic rank and $|Ш|$ finiteness at analytic rank ≤ 1 (with modularity).

### Theorem 546-B (Skinner-Urban 2014, Iwasawa Main Theorem => BSD)
- **Premise**: part of the $E[p^\infty]$ Iwasawa main theorem for p-adic.
- **Conclusion**: $|Ш|[p^\infty]$ finiteness at analytic rank 0.
- **Source**: C. Skinner, E. Urban, "The Iwasawa main conjectures for GL_2", Invent. Math. 195 (2014), 1-277.
- **Details**: controls the Shafarevich-Tate part of BSD prime by prime.

### Theorem 546-C (Wei Zhang 2014, Relaxing Gross-Zagier Conditions)
- **Premise**: $E/\mathbb{Q}$, square-free level, slight ramification condition.
- **Conclusion**: rank $\leq 1$ BSD fully demonstrated.
- **Source**: W. Zhang, "Selmer groups and the indivisibility of Heegner points", Cambridge J. Math. 2 (2014), 191-253.
- **Details**: Liu-Tian et al. further relaxed conditions.

### Theorem 546-D (BSD Weak Form => Birch's Lemma Extension)
- **Premise**: BSD weak form (ord_{s=1} L(E, s) = rank).
- **Conclusion**: asymptotic of $\prod_{p \leq X} \frac{\#E(\mathbb{F}_p)}{p}$ as $X \to \infty$ determined by rank (Birch 1963 analytic BSD premise form).
- **Source**: B. Birch, "Conjectures concerning elliptic curves", Proc. Symp. Pure Math. 8 (1965), 106-112.
- **Details**: numerical observation in BSD's formation era. Rank computable via weak-form theorem's contrapositive.

### Theorem 546-E (BKLPR Model => Selmer Average Size)
- **Premise**: BKLPR 2015 axioms (A1-A3).
- **Conclusion**: for squarefree $n$
    $\mathbb{E}[|\text{Sel}_n(E)|] = \sigma(n)$ (elliptic-curve family average).
- **Source**: M. Bhargava et al., "Modeling the distribution of ranks, Selmer groups, and Shafarevich-Tate groups of elliptic curves", Cambridge J. Math. 3 (2015), 275-321.
- **Details**: cf. pure-p3-1. Specialization at $n=6$: $\mathbb{E}[|\text{Sel}_6|]=\sigma(6)=12$. Conditional point emphasized by this project.

### n=6 Context
- BT-546 is most directly entangled with n=6. Lemma 1 (CRT direct decomposition) is unconditional; Corollary $\sigma(6)=12$ is BKLPR conditional.

---

## BT-547 — Theorems Under 4D Smooth Poincaré Assumption

### Theorem 547-A (Freedman 1982, Topological Poincaré 4D)
- **Premise**: $M^4$ simply connected closed topological 4-manifold with intersection form $E_8 \oplus E_8$ or similar.
- **Conclusion**: topological classification of $M$ determined by intersection form and Kirby-Siebenmann invariant.
- **Source**: M. Freedman, "The topology of four-dimensional manifolds", J. Diff. Geom. 17 (1982), 357-453.
- **Details**: topological Poincaré is an immediate corollary.

### Theorem 547-B (Donaldson 1983, Smooth-Topological Separation)
- **Premise**: $M$ simply connected smooth closed 4-manifold with positive-definite intersection form.
- **Conclusion**: form must be standard diagonal $\oplus_k \langle 1 \rangle$.
- **Source**: S. Donaldson, "An application of gauge theory to four dimensional topology", J. Diff. Geom. 18 (1983), 279-315.
- **Details**: topological allows $E_8$ positive-definite form; smooth forbids. Starting point of smooth-topological separation.

### Theorem 547-C (Smooth 4D Poincaré => Comparison with Non-Standard Smooth $\mathbb{R}^4$)
- **Premise**: unique smooth structure on $S^4$.
- **Conclusion**: constrains compactification structure of currently known infinite family of exotic $\mathbb{R}^4$.
- **Source**: R. Gompf, A. Stipsicz, "4-Manifolds and Kirby Calculus", AMS GSM 20 (1999), ch. 9.
- **Details**: reverse: "smooth 4D Poincaré false => exotic $S^4$ exists", changing topology landscape.

### Theorem 547-D (h-Cobordism Holding Condition, Smale 1962)
- **Premise**: $\dim \geq 5$ simply connected closed smooth manifolds.
- **Conclusion**: if h-cobordism then diffeomorphic.
- **Source**: S. Smale, "On the structure of manifolds", Amer. J. Math. 84 (1962), 387-399.
- **Details**: in 4D smooth h-cobordism does **not** force diffeomorphism (Donaldson). Dim n=6 is "safe lower bound" (atlas `n6-millennium-dfs-h-cobordism = dim ≥ 6`).

### Theorem 547-E (Seiberg-Witten => Smooth Invariants)
- **Premise**: $\text{Spin}^c$ structure on smooth 4-manifold.
- **Conclusion**: Seiberg-Witten invariants are smooth invariants distinguishing smooth structures.
- **Source**: N. Seiberg, E. Witten, "Monopoles, duality and chiral symmetry breaking in N=2 supersymmetric QCD", Nucl. Phys. B 431 (1994), 484-550; Taubes 1994 reformulation.
- **Details**: tool usable in counterexample attempts for 4D smooth Poincaré. So far SW invariants are 0 on $S^4$ (standard smooth structure).

### n=6 Context
- h-cobordism lower bound dim ≥ 6 equals n=6 (atlas). Not coincidence but structural lower bound.
- Demonstration tool for 4D smooth Poincaré is gauge theory; n=6 does not provide direct path.

---

## Conditional Theorems Synthesis Table (BT-541-547, 34 items)

| BT | Theorem | Premise | Conclusion | Source |
|----|------|------|------|------|
| 541-A | Miller-Rabin | GRH | deterministic $\tilde{O}((\log n)^4)$ | Miller 1976 |
| 541-B | Pólya-Vinogradov | GRH | $\sqrt{q}\log\log q$ | Montgomery-Vaughan 1977 |
| 541-C | π(x) error | RH | $O(\sqrt{x}\log x)$ | von Koch 1901 |
| 541-D | Class number | GRH | Siegel zero eliminated | Heilbronn 1934 |
| 541-E | Chebotarev | GRH | effective $\sqrt{x}$ error | Lagarias-Odlyzko 1977 |
| 542-A | PH collapse | P=NP | PH = P | Stockmeyer-Meyer 1973 |
| 542-B | SAT lower | ETH | $2^{o(n)}$ impossible | Impagliazzo-Paturi 2001 |
| 542-C | Derandom | E circuit $2^{\Omega(n)}$ | BPP=P | Impagliazzo-Wigderson 1997 |
| 542-D | PH infinite | NP ⊄ P/poly | PH infinite hierarchy | Karp-Lipton 1980 |
| 542-E | Natural barrier | OWF existence | natural proof impossible | Razborov-Rudich 1997 |
| 543-A | cluster | mass gap | exponential decay | Glimm-Jaffe 1987 |
| 543-B | Wightman | OS + gap | Minkowski axioms hold | Osterwalder-Schrader 1973 |
| 543-C | lattice limit | continuum convergence | continuum mass gap | Seiler 1982 |
| 543-D | large N | 't Hooft scaling | master field | Chatterjee 2015 |
| 544-A | Prodi-Serrin | $L^p_t L^q_x$ condition | smooth | Serrin 1962 |
| 544-B | $L^3$ | $L^\infty_t L^3_x$ | regular | ESŠ 2003 |
| 544-C | BKM | vorticity $L^1_t L^\infty_x$ | smooth | Beale-Kato-Majda 1984 |
| 544-D | CKN | suitable weak | partial regularity | CKN 1982 |
| 544-E | Axisym-no-swirl | $u_\theta=0$ | smooth | Ladyzhenskaya 1968 |
| 545-A | Lefschetz (1,1) | unconditional | divisor class | Lefschetz 1924 |
| 545-B | abelian Hodge | abelian variety | absolute Hodge | Deligne 1982 |
| 545-C | Hodge locus | variation | algebraic | Cattani-Deligne-Kaplan 1995 |
| 545-D | Standard => Hodge | Grothendieck SC | Hodge | Grothendieck 1969 |
| 545-E | prismatic | smooth proper | unified cohomology | Bhatt-Scholze 2022 |
| 546-A | Gross-Zagier | Heegner condition | rank ≤ 1 BSD | Gross-Zagier 1986 |
| 546-B | Iwasawa main | $p$-adic | $\|Ш\|$ finite $p$ | Skinner-Urban 2014 |
| 546-C | Zhang relax | square-free | rank ≤ 1 BSD | W. Zhang 2014 |
| 546-D | weak BSD | analytic rank | Birch asymptotic | Birch 1965 |
| 546-E | BKLPR | axioms A1-A3 | $\mathbb{E}[\|Sel_n\|]=\sigma(n)$ | BKLPR 2015 |
| 547-A | topological | $E_8 \oplus E_8$ etc. | topological classification | Freedman 1982 |
| 547-B | Donaldson | positive def | diagonal form | Donaldson 1983 |
| 547-C | smooth 4D P | $S^4$ smooth unique | exotic $\mathbb{R}^4$ constraint | Gompf-Stipsicz 1999 |
| 547-D | h-cobordism | dim ≥ 5 | diffeo | Smale 1962 |
| 547-E | Seiberg-Witten | $\text{Spin}^c$ | smooth invariant | Seiberg-Witten 1994 |

**Count**: BT-541(5) + BT-542(5) + BT-543(4) + BT-544(5) + BT-545(5) + BT-546(5) + BT-547(5) = **34 conditional theorems**.

---

## n=6 Connection

### Relationship between Conditional Theorems and Theorem 0

- **Theorem 0 (σφ=nτ iff n=6)** is unconditional. Depends on none of this note's conditional theorems.
- However some conditional theorems' conclusions **numerically** match the n=6 structure:
  - 544-A Prodi-Serrin exponents {2, 3} = {φ, n/φ}.
  - 543 lattice-gauge SM total 8+3+1 = 12 = σ.
  - 547-D h-cobordism lower bound dim ≥ 6 = n.
- This matching is **not** a claim that "n=6 provides the premise of conditional theorems". It only records that atlas.n6 DFS observation repeatedly encounters constant sums of each theorem.

### Promotion Path of Conditional Theorems

In this project's roadmap, mapping conditional theorems to atlas.n6 grades:
- Conditional theorem = [5]-[7] EMPIRICAL (premise unresolved, EXACT forbidden).
- If premise resolved = [10] EXACT promotion.
- Ex.: if BKLPR axioms A1-A3 resolved, 546-E goes EMPIRICAL -> EXACT.

---

## Practical Application

### Use as P3 Learning Output

1. **Theorem search index**: quick look-up for each of 7 problems on "what assumes what follows".
2. **atlas promotion candidate identification**: adjust atlas grade when conditional theorems become unconditional by advancing premise demonstration (e.g., partial completion of Iwasawa main theorem).
3. **Research priority**: prioritize "low-level premise -> strong conclusion" pairs (e.g., ETH -> SAT lower bound, premise "plausible").

### Combination with millennium_scanner.hexa

- `n6-architecture/scripts/millennium_scanner.hexa` scans for n=6 basic-function combinations matching Millennium constants.
- Combined with this note's conditional theorems, records **double-conditional** observations "scanner NEAR hit + conditional theorem premise holds" as atlas.n6 `[N?]` (conjecture) grade.
- Example: first Riemann zero 14.1347 ≈ σ+φ=14. If RH holds, first zero near this. Records NEAR hit under RH conditional.

### Textbook Mapping (Subsequent Learning Reference)

| BT | Recommended textbook (for tracking conditional theorems) |
|----|-------------------------------|
| 541 | Iwaniec-Kowalski, "Analytic Number Theory", AMS Colloq. 53 |
| 542 | Arora-Barak, "Computational Complexity" |
| 543 | Glimm-Jaffe, "Quantum Physics: A Functional Integral Point of View" |
| 544 | Robinson-Rodrigo-Sadowski, "The Three-Dimensional Navier-Stokes Equations" |
| 545 | Voisin, "Hodge Theory and Complex Algebraic Geometry" I, II |
| 546 | Silverman, "The Arithmetic of Elliptic Curves", ch. X |
| 547 | Gompf-Stipsicz, "4-Manifolds and Kirby Calculus" |

---

## Next Steps

1. **Citation reinforcement**: cross-confirm arXiv ID or DOI for each of 34 items in subsequent sessions. At this note's time only authors and journals recorded.
2. **Reverse conditionals**: for each conditional theorem, the contrapositive "if conclusion false then premise false". Which are useful counterexample routes?
3. **Premise hierarchy**: GRH < RH weak form < RH. Identify chain giving strongest conclusion from weakest premise.
4. **AI discovery of conditional theorems**: mode in which n6-architecture's DFS discovery engine scans conditional-theorem candidates (e.g., "if sopfr = p_1+p_2+p_3 ≈ π(N), what structure does N have?").
5. **Connection with prob-p3-3**: this note's conditional-theorem list usable as **case set** for hexa-lang verification pipeline. See prob-p3-3.

---

## Primary Source Notes

- Miller 1976: JCSS 13(3), 300-317.
- Montgomery-Vaughan 1977: Invent. Math. 43(1), 69-82.
- von Koch 1901: Sur la distribution des nombres premiers, Acta Math. 24, 159-182.
- Lagarias-Odlyzko 1977: In *Algebraic Number Fields* (Durham), Academic Press, 409-464.
- Stockmeyer-Meyer 1973: STOC '73 proceedings 1-9.
- Impagliazzo-Paturi 2001: JCSS 62(2), 367-375.
- Impagliazzo-Wigderson 1997: STOC '97, 220-229.
- Karp-Lipton 1980: STOC '80, 302-309.
- Razborov-Rudich 1997: JCSS 55(1), 24-35.
- Glimm-Jaffe 1987: 2nd ed., Springer-Verlag.
- Osterwalder-Schrader 1973/1975: Comm. Math. Phys. 31, 83-112 / 42, 281-305.
- Seiler 1982: Lecture Notes in Physics 159, Springer.
- Chatterjee 2015/2016: J. Funct. Anal. 271(10), 2944-3005.
- Serrin 1962: In *Nonlinear Problems* (Madison), 69-98.
- Escauriaza-Seregin-Šverák 2003: Russ. Math. Surv. 58(2), 211-250.
- Beale-Kato-Majda 1984: Comm. Math. Phys. 94(1), 61-66.
- Caffarelli-Kohn-Nirenberg 1982: Comm. Pure Appl. Math. 35(6), 771-831.
- Lefschetz 1924: Gauthier-Villars, Paris.
- Deligne 1982: LNM 900, 9-100.
- Cattani-Deligne-Kaplan 1995: J. AMS 8(2), 483-506.
- Grothendieck 1969: Algebraic Geometry (Bombay 1968), Oxford Univ. Press.
- Bhatt-Scholze 2022: Ann. Math. 196(3), 1135-1275.
- Gross-Zagier 1986: Invent. Math. 84, 225-320.
- Skinner-Urban 2014: Invent. Math. 195, 1-277.
- W. Zhang 2014: Cambridge J. Math. 2(2), 191-253.
- Birch 1965: Proc. Symp. Pure Math. 8, 106-112.
- Bhargava et al. 2015: Cambridge J. Math. 3(3), 275-321.
- Freedman 1982: J. Diff. Geom. 17(3), 357-453.
- Donaldson 1983: J. Diff. Geom. 18(2), 279-315.
- Gompf-Stipsicz 1999: AMS GSM 20.
- Smale 1962: Amer. J. Math. 84(3), 387-399.
- Seiberg-Witten 1994: Nucl. Phys. B 431, 484-550.

**Session author's note**: years/authors are standard-literature based; detailed arXiv ID versions may need reconfirmation in subsequent sessions. Per honesty rule, own-memory-based parts explicit. Prioritize the precise original statements to avoid distorting premise-conclusion structure of conditional theorems.
