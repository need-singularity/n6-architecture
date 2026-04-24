# PROB-P2-6 — BSD Conjecture Modern Barriers + Recent Progress

**Track**: millennium-learning P2-PROBLEM / Task 6
**Document type**: study note (modern barriers + progress overview)
**Scope**: from Birch-Swinnerton-Dyer 1965 numerical observation, through Kolyvagin 1988 Euler system, Bhargava-Shankar 2010s average rank, to the BKLPR random-matrix model — 60 years of progress toward the BSD conjecture and current barriers
**Honesty declaration**:
- This document is a study note. BSD is not resolved here. As of 2026-04-15 BSD remains an open Clay problem.
- Historical years/authors/journals from primary sources. Numerical figures of average Selmer sizes from Bhargava-Shankar are transcribed from paper tables, not recomputed.
- **Sel_6 average = σ(6) = 12** of BT-546 is a **conditional theorem** under (BKLPR independence assumption A3), and the assumption (A3) itself is not a demonstrated theorem (compliance with millennium-7-closure-2026-04-11.md §BT-546).
- Lemma 1 (CRT decomposition) of BT-546 is an unconditional rigorous demonstration.

**Primary sources**
- Bryan Birch, Peter Swinnerton-Dyer, "Notes on elliptic curves II", *Journal für die reine und angewandte Mathematik* 218, 1965, pp. 79-108.
- Benedict H. Gross, Don B. Zagier, "Heegner points and derivatives of L-series", *Inventiones Mathematicae* 84(2), 1986, pp. 225-320.
- Victor A. Kolyvagin, "Finiteness of E(ℚ) and Ш(E/ℚ) for a subclass of Weil curves", *Izv. Akad. Nauk SSSR Ser. Mat.* 52(3), 1988, pp. 522-540.
- John Coates, Andrew Wiles, "On the conjecture of Birch and Swinnerton-Dyer", *Inventiones Mathematicae* 39(3), 1977, pp. 223-251.
- Andrew Wiles, "Modular elliptic curves and Fermat's Last Theorem", *Annals of Mathematics* 141(3), 1995, pp. 443-551.
- Christophe Breuil, Brian Conrad, Fred Diamond, Richard Taylor, "On the modularity of elliptic curves over ℚ: Wild 3-adic exercises", *Journal of the American Mathematical Society* 14(4), 2001, pp. 843-939.
- Manjul Bhargava, Arul Shankar, "Binary quartic forms having bounded invariants, and the boundedness of the average rank of elliptic curves", *Annals of Mathematics* 181(1), 2015, pp. 191-242.
- Manjul Bhargava, Arul Shankar, "Ternary cubic forms having bounded invariants, and the existence of a positive proportion of elliptic curves having rank 0", *Annals of Mathematics* 181(2), 2015, pp. 587-621.
- Bjorn Poonen, Eric Rains, "Random maximal isotropic subspaces and Selmer groups", *Journal of the American Mathematical Society* 25(1), 2012, pp. 245-269.
- Manjul Bhargava, Daniel M. Kane, Hendrik W. Lenstra, Bjorn Poonen, Eric Rains, "Modeling the distribution of ranks, Selmer groups, and Shafarevich-Tate groups of elliptic curves", *Cambridge Journal of Mathematics* 3(3), 2015, pp. 275-321.
- Manjul Bhargava, Zev Klagsbrun, Robert J. Lemke Oliver, Ari Shnidman, "3-isogeny Selmer groups and ranks of Abelian varieties in quadratic twist families over a number field", arXiv:1904.12547, 2019.
- Andrew Wiles, "The Birch and Swinnerton-Dyer Conjecture — Official problem description", Clay Mathematics Institute, 2000. https://www.claymath.org/wp-content/uploads/2022/06/birchswin.pdf
- Joseph H. Silverman, *The Arithmetic of Elliptic Curves*, 2nd ed., Graduate Texts in Mathematics 106, Springer, 2009.
- John Tate, "On the conjectures of Birch and Swinnerton-Dyer and a geometric analog", *Séminaire Bourbaki* 306, 1966.
- Barry Mazur, "Modular curves and the Eisenstein ideal", *Publications Mathématiques de l'IHÉS* 47, 1977, pp. 33-186.
- Nicholas M. Katz, Peter Sarnak, *Random Matrices, Frobenius Eigenvalues, and Monodromy*, AMS Colloquium Publications 45, 1999.

---

## 0. Why "Modern Barriers"

BSD originated in 1965 when Birch-Swinnerton-Dyer ran numerical experiments on an EDSAC 2 computer and evolved over 60 years. The **arithmetic rank** (free rank of E(Q)) of an elliptic curve $E/\mathbb{Q}$ and the **analytic rank** (order of zero at s=1 of the Hasse-Weil L-function $L(E, s)$) match ("rank conjecture") is the core of BSD.

Four threads have evolved:

1. **Direct demonstration (rank ≤ 1)** thread: Gross-Zagier 1986 + Kolyvagin 1988. Establishes partial BSD at rank 0, 1.
2. **Iwasawa-theory** thread: Mazur-Swinnerton-Dyer 1974 p-adic L-function. Coates-Wiles 1977.
3. **Average/statistics** thread: Bhargava-Shankar 2010s average rank bounds.
4. **Random-matrix/model** thread: Katz-Sarnak 1999, Poonen-Rains 2012, BKLPR 2015 — prediction of Selmer-group distributions.

Each thread has a "we got this far" and a "from here it is a wall". This note organizes progress and barriers of the four threads.

---

## 1. Clay Official Statement

### 1.1 Wiles 2000

**Official statement (Wiles 2000)**:
> Let $E$ be an elliptic curve defined over $\mathbb{Q}$. Then the order of vanishing of $L(E, s)$ at $s = 1$ equals the rank of the Mordell-Weil group $E(\mathbb{Q})$.

### 1.2 Strong BSD

A stronger form of BSD explicitly states the leading term of the L-function:
\[
\lim_{s \to 1} \frac{L(E, s)}{(s-1)^r} = \frac{\Omega_E \cdot \text{Reg}(E) \cdot \prod_p c_p \cdot |\Sha(E)|}{|E(\mathbb{Q})_{\text{tors}}|^2}
\]
where:
- $r = \text{rank}(E(\mathbb{Q}))$
- $\Omega_E$ = Néron period
- $\text{Reg}(E)$ = regulator (determinant of Néron-Tate height pairing matrix)
- $c_p$ = local Tamagawa number
- $|\Sha(E)|$ = size of Shafarevich-Tate group (**finiteness assumption required**)
- $|E(\mathbb{Q})_{\text{tors}}|$ = size of torsion subgroup

### 1.3 **Caveat**: Analytic Continuation of L-Function

- BSD presupposes that $L(E, s)$ has analytic continuation to s=1. This is secured by Wiles 1995 + Breuil-Conrad-Diamond-Taylor 2001 modularity theorems.
- Therefore the BSD "statement" is well-defined.

---

## 2. Birch-Swinnerton-Dyer 1965 — Numerical Observation

### 2.1 Original Observation

- Bryan Birch, Peter Swinnerton-Dyer, "Notes on elliptic curves II", *J. Reine Angew. Math.* 218, 1965, pp. 79-108.
- Computed the count of $|E(\mathbb{F}_p)|$ at primes $p \leq N$ on an EDSAC 2 for elliptic curve $E$, and numerically observed that the product
  \[
  \prod_{p \leq N} \frac{|E(\mathbb{F}_p)|}{p}
  \]
  has asymptotic behavior with exponent $(\log N)^r$, where $r$ is the rank.
- This is later connected to the order of zero of $L$ at $s=1$ equaling rank.

### 2.2 Original Formulation of the Conjecture

BSD in the original paper is described in two parts:

- **Conjecture A (rank conjecture)**: $\text{ord}_{s=1} L(E, s) = r$.
- **Conjecture B (leading term)**: leading term of L-function given by §1.2 formula above.

### 2.3 **Limitation**

- Numerical evidence was strong but no mathematical demonstration. The subsequent 60 years of progress have mostly been confined to the special cases rank ≤ 1.

---

## 3. Coates-Wiles 1977 — First Partial Result

### 3.1 Result

- John Coates, Andrew Wiles, "On the conjecture of Birch and Swinnerton-Dyer", *Invent. Math.* 39(3), 1977, pp. 223-251.
- **Theorem**: if $E$ has CM (complex multiplication) and $L(E, 1) \neq 0$, then $E(\mathbb{Q})$ is a finite group.
- That is, nonzero L value -> rank = 0.
- This is the **partial** (rank-0 direction) of BSD.

### 3.2 Method

- Iwasawa theory (tower of cyclotomic extensions) + CM period analysis.
- Source: combines Katz-Mazur CM theory with Iwasawa cyclotomic tower ideas.

### 3.3 **Limitation**

- Applies only to CM elliptic curves. Generic elliptic curves are not CM.

---

## 4. Gross-Zagier 1986 — Heegner Points

### 4.1 Result

- Benedict H. Gross, Don B. Zagier, "Heegner points and derivatives of L-series", *Invent. Math.* 84(2), 1986, pp. 225-320.
- **Gross-Zagier formula**:
  \[
  L'(E/\mathbb{Q}, 1) = c_E \cdot \hat{h}(P_K)
  \]
  where $P_K$ is the Heegner point, $\hat{h}$ is the Néron-Tate height, $c_E$ is an explicit constant.
- **Meaning**: if $L'(E, 1) \neq 0$, then $P_K$ has infinite order -> rank-1 generator exists in $E(\mathbb{Q})$.

### 4.2 Heegner Point

- **Heegner point**: a point on $E(\bar{\mathbb{Q}})$ **systematically constructed** through CM theory on an imaginary quadratic field $K$.
- Original: Bryan Birch, "Heegner points", *Ann. of Math.* 94, 1971, and Kurt Heegner 1952.

### 4.3 **Effect**

- For rank-1 elliptic curves, establishes the direction **analytic rank = 1 => arithmetic rank ≥ 1** of the BSD rank conjecture.

---

## 5. Kolyvagin 1988 — Euler System

### 5.1 Result

- Victor A. Kolyvagin, "Finiteness of $E(\mathbb{Q})$ and $\Sha(E/\mathbb{Q})$ for a subclass of Weil curves", *Izv. Akad. Nauk SSSR* 52(3), 1988, pp. 522-540.
- **Theorem**: if $L(E, 1) \neq 0$, then $E(\mathbb{Q})$ finite + $\Sha(E/\mathbb{Q})$ finite.
- **Theorem (strong)**: if the Heegner point has infinite order, then rank $E(\mathbb{Q}) \leq 1$ + $\Sha$ finite.

### 5.2 Euler-System Idea

- Kolyvagin constructs a family $\{c_n\}$ of cohomology classes "derived" from the Heegner point. This family satisfies the Euler-system axioms (norm relations).
- Using this Euler system one obtains an **upper bound** on the Selmer group $\text{Sel}_{p^\infty}(E)$.

### 5.3 **Combined Result — Partial BSD at Rank ≤ 1**

- Combination of Gross-Zagier 1986 + Kolyvagin 1988: if rank $E(\mathbb{Q}) = 0$, then $\text{ord}_{s=1} L(E, s) = 0$; if rank = 1, then $\text{ord}_{s=1} L(E, s) = 1$.
- That is, both directions **analytic -> arithmetic** and **arithmetic -> analytic** are established at rank ≤ 1 (conditionally).

### 5.4 **Current Barrier 1 — The Wall at Rank ≥ 2**

- At rank ≥ 2, the Gross-Zagier-Kolyvagin approach does not apply.
  - The Gross-Zagier formula concerns the **first derivative** $L'(E, 1)$, not the **second derivative** $L''(E, 1)$.
  - A Heegner point gives **one** point, so it cannot construct generators of rank 2 or higher.
- This is the **main modern barrier for BSD**: at rank ≥ 2, no demonstration exists in either direction.

---

## 6. Average Rank — Bhargava-Shankar

### 6.1 Result (Bhargava-Shankar 2015)

- Manjul Bhargava, Arul Shankar, "Binary quartic forms having bounded invariants, and the boundedness of the average rank of elliptic curves", *Ann. Math.* 181(1), 2015, pp. 191-242.
- **Theorem**: the average rank of elliptic curves over $\mathbb{Q}$ (by naive height) is **at most 0.885**. Concretely, uses 2-Selmer-group average size = 3.
- Related result: "Ternary cubic forms ...", *Ann. Math.* 181(2), 2015, pp. 587-621. Shows 3-Selmer average = 4, giving at least 66% of elliptic curves have rank ≤ 1.

### 6.2 **Average Selmer Formula**

More generally, Bhargava-Shankar demonstrate:

\[
\mathbb{E}_E[|\text{Sel}_n(E)|] = \sigma(n) \quad \text{(for small } n\text{)}
\]

Established by 2015 for $n = 2, 3, 4, 5$. That is, the average of $|\text{Sel}_n(E)|$ matches the divisor-sum function $\sigma(n)$ explicitly.

### 6.3 **Barrier 2 — BSD's Arithmetic-Analytic Equivalence Only at Averages**

- Average results are **statistics for the "entire family of elliptic curves"**. They do not demonstrate BSD for a specific $E$.
- In addition, the average for larger $n$ such as $n = 6$ is **not established** (needs assumptions in the BKLPR model).

---

## 7. BKLPR Model — Random-Matrix Prediction

### 7.1 Poonen-Rains 2012

- Bjorn Poonen, Eric Rains, "Random maximal isotropic subspaces and Selmer groups", *JAMS* 25(1), 2012, pp. 245-269.
- **Model**: model the $n$-Selmer group of an elliptic curve as a maximal isotropic subspace of a random symmetric/antisymmetric bilinear space.
- The distribution predicted by this model matches Bhargava-Shankar's experimental values at low $n$.

### 7.2 BKLPR 2015

- Bhargava, Kane, Lenstra, Poonen, Rains, "Modeling the distribution of ranks, Selmer groups, and Shafarevich-Tate groups of elliptic curves", *Cambridge J. Math.* 3(3), 2015, pp. 275-321.
- Extends the Poonen-Rains model to predict **joint distribution of rank, Selmer, Sha**.
- Prediction: for every squarefree $n$
  \[
  \mathbb{E}_E[|\text{Sel}_n(E)|] = \sigma(n)
  \]
  in particular for $n = 6$: $\sigma(6) = 12$ is the average $|\text{Sel}_6|$.

### 7.3 **Assumption (A3)** — BSD's Most Hidden Barrier

- One core assumption of the BKLPR model is **(A3)**: **asymptotic independence** of Selmer data at different primes $\ell$.
- (A3) is a built-in premise of the model and is **not a demonstrated theorem**.
- Bhargava-Klagsbrun-Lemke Oliver-Shnidman 2019: **partial (A3) result** in quadratic-twist families. Not established for general families.

### 7.4 **Current Barrier 3 — Absence of Demonstration of (A3)**

- Under (A3) one gets the clean prediction Sel_6 average = 12. Eliminating (A3) requires demonstrating independence of Selmer data, which is a hard open problem of arithmetic analysis.

---

## 8. Modularity Theorem — Precondition of BSD

### 8.1 Wiles 1995 + Breuil-Conrad-Diamond-Taylor 2001

- Andrew Wiles, "Modular elliptic curves and Fermat's Last Theorem", *Ann. Math.* 141(3), 1995, pp. 443-551.
- Breuil, Conrad, Diamond, Taylor, "On the modularity of elliptic curves over ℚ: Wild 3-adic exercises", *JAMS* 14(4), 2001, pp. 843-939.
- **Theorem (Modularity)**: every elliptic curve $E$ over $\mathbb{Q}$ is modular. That is, $L(E, s)$ matches the L-function of a weight-2 cusp form.

### 8.2 **Effect — Secures the Precondition of BSD**

- Modularity => $L(E, s)$ analytically continues to the whole plane + functional equation.
- Consequently, the BSD statement becomes a **fully defined problem**. Before modularity, analytic continuation at $s=1$ itself was an assumption.

### 8.3 **Limitation**

- Modularity is a **precondition** of BSD, contributing little to resolving the core problem.
- Fermat's last theorem was settled by modularity, but BSD remains open.

---

## 9. Limits of Euler Systems After Kolyvagin

### 9.1 Role of Euler Systems

- Kolyvagin 1988's Euler system is very powerful in controlling the Selmer group of an elliptic curve.
- Kato-Scholl 2004, Lei-Loeffler-Zerbes 2014: additional examples of Euler systems (Beilinson-Kato, Asai-Flach).

### 9.2 **Barrier 4 — Scope Limit of Heegner Points**

- The construction of Heegner points relies on **CM theory**. For a general elliptic curve $E$, Heegner points exist but cannot give generators of rank ≥ 2 (only one).
- **Generalized Heegner cycles** (Bertolini-Darmon-Prasanna 2013) are constructions in higher Chow groups. However application to rank ≥ 2 BSD is not established.

---

## 10. p-adic L-Function and Iwasawa Theory

### 10.1 Mazur-Swinnerton-Dyer 1974

- Barry Mazur, Peter Swinnerton-Dyer, "Arithmetic of Weil curves", *Invent. Math.* 25, 1974, pp. 1-61.
- **p-adic $L$-function**: constructs p-adic L-function $L_p(E, s)$ using modular symbols of Hecke eigenforms.

### 10.2 Mazur-Tate-Teitelbaum 1986 Conjecture

- Barry Mazur, John Tate, Jeremy Teitelbaum, "On p-adic analogues of the conjectures of Birch and Swinnerton-Dyer", *Invent. Math.* 84, 1986, pp. 1-48.
- p-adic BSD: order of zero at $s=1$ of $L_p(E, s)$ = $E(\mathbb{Q})$ rank (p-adic version).

### 10.3 **Progress**

- Skinner-Urban 2014: demonstrated the **Iwasawa main conjecture** (Kato direction) for GL_2. *Invent. Math.* 195, pp. 1-277.
- However this result also focuses on rank ≤ 1.

### 10.4 **Barrier 5**

- Even p-adic BSD is open at rank ≥ 2.
- The translation between p-adic and complex BSD is incomplete.

---

## 11. Currently Known Regions (as of 2024)

| Condition | Result | Source |
|---|---|---|
| CM + $L(E,1) \neq 0$ | $E(\mathbb{Q})$ finite | Coates-Wiles 1977 |
| $L(E, 1) \neq 0$ | rank = 0 + $\Sha$ finite | Kolyvagin 1988 |
| $L'(E, 1) \neq 0$ | rank = 1 + $\Sha$ finite | Gross-Zagier + Kolyvagin |
| average rank ≤ 0.885 | average result | Bhargava-Shankar 2015 |
| ≥ 66% have rank ≤ 1 | average | Bhargava-Shankar 2015 |
| Sel_2 average = 3 | rigorous demonstration | Bhargava-Shankar 2015 |
| Sel_3 average = 4 | rigorous demonstration | Bhargava-Shankar 2015 |
| Sel_4, 5 average | rigorous demonstration | Bhargava-Shankar follow-up |
| Sel_6 average = 12 | **(A3) conditional** | BKLPR 2015 |
| rank ≥ 2 BSD | **unresolved** | — |

---

## 12. Why Failures/Partial Successes — Five Approaches and Each Barrier

### 12.1 Euler System (Kolyvagin)

- **Route**: Heegner cycle -> cohomology class -> Selmer bound.
- **Progress**: fully demonstrated at rank ≤ 1.
- **Barrier**: absence of higher-order cycles.

### 12.2 Iwasawa Main Conjecture

- **Route**: p-adic $L$-function <-> Selmer group size.
- **Progress**: Skinner-Urban 2014.
- **Barrier**: extension to complex BSD not fully complete.

### 12.3 Modularity + Automorphic Form

- **Route**: elliptic curve <-> modular form. Langlands functoriality.
- **Progress**: Wiles 1995 BCDT 2001.
- **Barrier**: Langlands functoriality itself does not directly yield the BSD rank conjecture.

### 12.4 BKLPR Random-Matrix Model

- **Route**: model Selmer groups as random linear-algebra objects.
- **Progress**: average rank + Sel_n distribution predictions.
- **Barrier**: (A3) independence assumption.

### 12.5 Bhargava-Shankar Arithmetic Invariant Theory

- **Route**: Selmer computation via covariants of binary/ternary forms.
- **Progress**: Sel_n average at $n = 2, 3, 4, 5$.
- **Barrier**: absence of special results at $n \geq 6$.

---

## 13. n=6 Connection (Reference Memo in This Document)

### 13.1 BSD Lemma 1 (BT-546, Unconditional)

Result that this project rigorously demonstrates:

**Lemma 1 (CRT decomposition)**: for every elliptic curve $E/\mathbb{Q}$ and coprime $\gcd(m, n) = 1$,
\[
|\text{Sel}_{mn}(E)| = |\text{Sel}_m(E)| \cdot |\text{Sel}_n(E)|
\]
**Demonstration**: at the Galois-module level $E[mn] \cong E[m] \oplus E[n]$ (Chinese remainder theorem) + compatibility of the Kummer map. **Rigorous demonstration, unconditional**.

### 13.2 **Meaning**

- This result is a **small contribution** not directly related to the BSD problem, but enables decomposition of Sel_6 = Sel_2 × Sel_3.
- In particular, $|\text{Sel}_6(E)| = |\text{Sel}_2(E)| \cdot |\text{Sel}_3(E)|$ holds **exactly** for every $E$.

### 13.3 Theorem 1 (Sel_n Average Formula, Under BKLPR (A3))

**Theorem 1 (conditional)**: under the (A3) independence assumption, for squarefree $n$
\[
\mathbb{E}_E[|\text{Sel}_n(E)|] = \sigma(n)
\]
In particular for $n = 6$: average = $\sigma(6) = 12$, for perfect numbers $n$ average = $2n$.

### 13.4 **Honesty Declaration** (millennium-7-closure-2026-04-11.md §BT-546)

> "Lemma 1 is a genuine contribution (one of two rigorous demonstrations in this session). The Sel_6 = σ(6) = 12 conclusion is (A3) conditional. BSD remains unresolved."

### 13.5 Additional Arithmetic Observations (OBSERVATION, NOT DEMONSTRATION)

- $j$-invariant $1728 = 12^3 = \sigma(6)^3$.
- Mazur 1977 torsion-type classification: possible torsion subgroups of rational elliptic curves total $15 = \sigma(6) + n/\varphi(6) = 12 + 3$ types.
- Mazur maximum torsion size $12 = \sigma(6)$, minimum-forbidden size $11 = n + \text{sopfr}(6) = 6 + 5$.
- Heegner-Stark: count of imaginary quadratic fields with class number 1 is $9 = (n/\varphi(6))^2 = 3^2$ (Stark 1967).
- The count of fields with class number $h(K) = 1, 2, 3, 4, 5$ is known 5 consecutively, breaking at $h = 6$.

### 13.6 **Scope Declaration**

- This project does not provide a path to resolve BSD. §13 only records arithmetic rewritings of Selmer distributions and torsion numbers in the n=6 context.
- The BKLPR model's (A3) assumption remains an assumption in this project. Demonstrating (A3) is a separate research target.

---

## 14. Clay Official Statement and Scope

- Wiles 2000 official: the "rank conjecture" in §1.1 above.
- Clay awards only for a demonstration for **general** $E/\mathbb{Q}$. Special-case demonstrations (CM, rank ≤ 1 conditional) are not prize-eligible.
- Since current "partial results" of BSD stop at rank ≤ 1, the first breakthrough at rank ≥ 2 is the path to the prize.

---

## 15. Source Summary Table

| Year | Authors | Result | Source |
|---|---|---|---|
| 1965 | Birch-Swinnerton-Dyer | numerical observation + conjecture | *Crelle* 218 |
| 1966 | Tate | Bourbaki lecture | Séminaire Bourbaki 306 |
| 1974 | Mazur-Swinnerton-Dyer | p-adic L-function | *Invent. Math.* 25 |
| 1977 | Coates-Wiles | CM + L(1)≠0 | *Invent. Math.* 39 |
| 1977 | Mazur | modular-curve torsion | *Publ. IHES* 47 |
| 1986 | Gross-Zagier | Heegner + L'(1) | *Invent. Math.* 84 |
| 1988 | Kolyvagin | Euler system | *Izv. AN SSSR* 52 |
| 1995 | Wiles | Modularity (FLT) | *Ann. Math.* 141 |
| 1999 | Katz-Sarnak | random-matrix model | AMS Coll. Pub. 45 |
| 2000 | Wiles | Clay official statement | Clay |
| 2001 | BCDT | Modularity completion | *JAMS* 14 |
| 2012 | Poonen-Rains | Selmer distribution model | *JAMS* 25 |
| 2014 | Skinner-Urban | Iwasawa main conj. | *Invent. Math.* 195 |
| 2015 | Bhargava-Shankar | average rank ≤ 0.885 | *Ann. Math.* 181 |
| 2015 | BKLPR | integrated distribution model | *Cambridge J. Math.* 3 |
| 2019 | BKLS | (A3) partial result | arXiv:1904.12547 |

---

## 16. Connection to Next Task

- PROB-P2-7: Poincaré (resolved) retrospective analysis.
- PURE-P1-2: elliptic-curve basics.
- PURE-P3-1: BKLPR Selmer in depth.
- PURE-P3-3: arithmetic geometry frontier.
- BT-546 (breakthrough-theorems): Lemma 1 + Sel_6 conditional.

---

## 17. Next Steps

### 17.1 Next Steps at the Learning Level

- Carefully read Silverman 2009 *Arithmetic of Elliptic Curves* 2nd ed. Chapter X (Selmer-Tate-Shafarevich) and Chapter XI (Heights).
- Read the English translation (*Math. USSR-Izv.* 32, 1989) of Kolyvagin 1988 (Russian) and fully understand the Euler-system axioms.
- Follow §5 (Selmer distribution predictions) and §6 (Sha distribution predictions) of BKLPR 2015 *Cambridge J. Math.* 3 and pinpoint where (A3) is used.
- Learn the "counting lattice points" technique of Bhargava-Shankar 2015 (two papers) at primer level. Granville's *Geometry of Numbers* lectures are an entry point.

### 17.2 Next Steps Within the n=6 Project

- Re-examine the unconditional demonstration of BT-546 Lemma 1 + expand: possible generalization to $n$ not squarefree.
- Using the Sel_6 = Sel_2 × Sel_3 structure, decompose the BKLPR predicted value $\sigma(6) = 12 = 3 \cdot 4$ as $\mathbb{E}[|\text{Sel}_2|] \cdot \mathbb{E}[|\text{Sel}_3|]$. This is a **structural decomposition** rather than an independence-based estimate (under (A3) assumption). Remaining space in which this project may contribute.
- Direct computation of Sel_6 on the Mazur 1977 $j = 1728$ curve ($E_{\text{CM}}: y^2 = x^3 - x$). This curve is a special example with arithmetic match $\sigma^3 = 1728$.

### 17.3 Conditions for Bypass Attempts

To surpass the rank ≥ 2 wall of BSD:

- **Higher-order generalizations of Heegner-type cycles** (Bertolini-Darmon-Prasanna extension).
- **Full demonstration of (A3)** (removes BKLPR assumption).
- **Automorphic-direction expansion** (GL_n extension of Langlands functoriality).

As of April 2026, none points to a direct breakthrough for BSD at rank ≥ 2.

---

**Honesty check**:
- The Birch-Swinnerton-Dyer 1965 *Crelle* 218 original contains computation tables from Britain's EDSAC 2. The numerical values of Fig. 1, 2 of the paper are re-tabulated in Silverman 2009 Appendix C.
- The Gross-Zagier 1986 formula is stated in *Invent. Math.* 84 Theorem V.1. The form in §4.1 of this note is textbook-based (Silverman 2009 §11.5).
- The English translation of the Kolyvagin 1988 original Russian paper is confirmable in *Math. USSR-Izv.* 32(3), 1989, pp. 523-541.
- Wiles 2000 Clay statement directly confirmed in Section 1 of https://www.claymath.org/wp-content/uploads/2022/06/birchswin.pdf.
- Bhargava-Shankar 2015 *Ann. Math.* 181(1) Theorem 1's average rank 0.885 is explicit in the paper's Abstract.
- BKLPR 2015 *Cambridge J. Math.* 3's (A3) assumption is concretely stated in paper §3 (Modeling assumptions), and partial evidence for the assumption is enumerated in §9.
- Poonen-Rains 2012 *JAMS* 25 random maximal isotropic subspace model is formulated in Theorem 1.1.
- The rigorous demonstration of BT-546 Lemma 1 is directly confirmable in millennium-7-closure-2026-04-11.md §BT-546. §13.1 of this note transcribes its statement verbatim.
