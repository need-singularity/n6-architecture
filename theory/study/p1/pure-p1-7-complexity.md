# PURE-P1-7 — Complexity Theory Basics (P/NP/coNP / polynomial reduction / NP-completeness / Cook-Levin)

> Track: P1-PURE / Task 7
> Completion criterion: state precisely P, NP, coNP, NP-completeness, and polynomial reduction ≤_p starting
> from the Turing-machine definition, and reconstruct the skeleton of the Cook-Levin theorem (SAT ∈ NP-complete).
> Source base: Arora-Barak "Computational Complexity: A Modern Approach" (Cambridge, 2009) ch. 1-3, ch. 6,
> Sipser "Introduction to the Theory of Computation" (3rd ed., 2013) ch. 7-8,
> Papadimitriou "Computational Complexity" (Addison-Wesley, 1994) ch. 2-8.
> **Honesty**: this file is a textbook reconstruction. No new arguments or theorems. Every theorem and example
> is organised following the three textbooks above with page references. Numerical comparisons are avoided.

---

## 0. Purpose and Scope

Before tackling Millennium BT-542 (P vs NP) in earnest, the following seven foundations are required.

1. Turing machines (1-tape, multi-tape, non-deterministic) definition and time-complexity measures
2. Complexity classes P, NP, coNP, PSPACE, EXP
3. Polynomial-time reduction ≤_p and the definition of NP-completeness
4. Cook-Levin theorem — SAT ∈ NP-complete
5. Representative NP-complete problem families (3SAT, CLIQUE, HAM-PATH, ...)
6. coNP-completeness and the NP ∩ coNP special family
7. Time- and space-hierarchy theorems

These notes follow an "argument journey" step by step and arrange practical reduction examples. Circuit
complexity, probabilistic complexity, and interactive proofs are deferred to P2.

---

## 1. Turing Machines and Time Complexity

### 1.1 1-Tape Turing Machine

Tuple M = (Q, Σ, Γ, δ, q_0, q_accept, q_reject):

- Q: finite state set
- Σ: input alphabet (∉ ␣)
- Γ ⊇ Σ ∪ {␣}: tape alphabet
- δ: Q × Γ → Q × Γ × {L, R, S}: transition function
- q_0, q_accept, q_reject ∈ Q

A computation is a sequence of triples (state, tape, head). Halts on reaching accept/reject.

### 1.2 Polynomial-Time Class

For a decision problem L ⊆ {0,1}*:

```
  DTIME(T(n)) = {L : a deterministic TM decides L in O(T(n)) time}
  P = ⋃_{k≥1} DTIME(n^k)
```

Multi-tape and 1-tape TMs agree up to polynomial overhead, so the definition of P is robust across models.

### 1.3 Non-deterministic TM and NP

A non-deterministic TM has δ: Q × Γ → 2^{Q × Γ × {L,R,S}}. An input x is accepted if some computation path
reaches accept.

```
  NTIME(T(n)) = {L : a non-deterministic TM decides L in O(T(n)) time}
  NP = ⋃_{k≥1} NTIME(n^k)
```

Equivalent (verifier) definition: L ∈ NP iff there exists a polynomial-time deterministic TM V and a
polynomial p such that x ∈ L ⟺ ∃ y (|y| ≤ p(|x|), V(x, y) accepts). y is called a **witness**/certificate.

### 1.4 coNP and Duality

```
  coNP = {L : {0,1}* \ L ∈ NP}
```

coNP is the class of problems with negative certificates. Example: TAUTOLOGY (a Boolean formula is always
true) ∈ coNP. P ⊆ NP ∩ coNP ⊆ NP ∪ coNP ⊆ PSPACE, and whether NP = coNP is open.

### 1.5 Space Complexity

```
  DSPACE(S(n))           L = DSPACE(log n),   PSPACE = ⋃_k DSPACE(n^k)
  NSPACE(S(n))           NL = NSPACE(log n)
```

Savitch's theorem (1970): NSPACE(S) ⊆ DSPACE(S²) for S(n) ≥ log n. Hence PSPACE = NPSPACE.

---

## 2. Polynomial-Time Reduction and Completeness

### 2.1 many-one Polynomial Reduction

Languages A, B ⊆ {0,1}*. A ≤_p B iff there exists a polynomial-time computable function f such that
x ∈ A ⟺ f(x) ∈ B. f is the **reduction**.

Properties:
- (Reflexivity) A ≤_p A
- (Transitivity) A ≤_p B, B ≤_p C ⟹ A ≤_p C
- (Preservation) B ∈ P ⟹ A ∈ P. Likewise B ∈ NP ⟹ A ∈ NP.

### 2.2 Turing Reduction (Cook Reduction)

A ≤_T^p B : a polynomial-time TM with an oracle for B decides A. Karp reduction (≤_p) is a special case of
Cook reduction (a single oracle call whose output is directly accepted/rejected).

In most subfields ≤_p (Karp) is the default. Using ≤_T^p loosens completeness discussions slightly.

### 2.3 NP-completeness

L is **NP-hard** iff every L' ∈ NP satisfies L' ≤_p L.
L is **NP-complete** iff L ∈ NP and L is NP-hard.

If any NP-complete L is in P, then P = NP. Hence NP-complete problems function as "representatives of NP
problems for which efficient algorithms appear infeasible".

---

## 3. Cook-Levin Theorem

### 3.1 Statement

**SAT** = {φ : Boolean formula φ is satisfiable} ∈ NP-complete.

(Cook 1971, Levin 1973 independently)

### 3.2 SAT ∈ NP

A satisfying assignment is the witness. V(φ, a) evaluates φ on a and checks truth in linear time.

### 3.3 Key Points of the NP-hard Argument

Take an arbitrary L ∈ NP. Let a non-deterministic TM M decide L in polynomial time p(n). For input x,
encode M's computation tableau as a Boolean formula.

- Grid of size (p(n)+1) × (p(n)+1); cell[i,j] encodes (state, symbol) at time i, position j
- Variables x_{i,j,s}: does cell[i,j] equal symbol s?
- Constraints:
  1. Each cell holds exactly one (state, symbol) value (exactly-one encoding)
  2. The initial row (i=0) is fixed by the input x
  3. Transitions (i → i+1) admit only the 2×3 windows permitted by δ
  4. Some row contains a cell in state q_accept

Each constraint scales as O(p(n)²) cells × a constant number of fixed-size windows, so the total Boolean
formula has length O(p(n)² · log n) = poly(n). This formula φ_x satisfies x ∈ L ⟺ φ_x ∈ SAT.

Hence L ≤_p SAT, and since L was arbitrary SAT is NP-hard. Hence NP-complete.

(Source: Arora-Barak Thm 2.10, Sipser Thm 7.37)

### 3.4 Reduction to 3SAT

SAT → 3SAT (exactly 3 literals per clause): decompose a long clause ℓ_1 ∨ ... ∨ ℓ_k into a chain of 3-clauses
using new variables y_i. The resulting formula size is also polynomial.

Hence 3SAT is also NP-complete. 3SAT is a common starting point for reductions.

---

## 4. Representative NP-complete Problems

### 4.1 Karp 1972 — 21 NP-complete Problems

Karp showed the following 21 are NP-complete by reducing from 3SAT:

- CLIQUE, INDEPENDENT-SET, VERTEX-COVER
- 3-DIMENSIONAL-MATCHING, PARTITION, SUBSET-SUM
- HAMILTONIAN-PATH, HAMILTONIAN-CYCLE, TSP
- GRAPH-COLORING, CHROMATIC-NUMBER
- KNAPSACK, BIN-PACKING
- MAX-CUT, MAX-SAT, MAX-3SAT
- others

Today thousands of problems are known to be NP-complete. Standard reference: Garey-Johnson "Computers and
Intractability" (1979).

### 4.2 3SAT → 3COLOR Reduction Example

Input: 3SAT formula φ. Output: a graph G and k=3 colour assignability.

Construction:
- Reference triangle T = {T, F, N} (True, False, Neutral)
- For each variable x_i, pair of nodes x_i, ¬x_i plus N forming a triangle → x_i takes exactly one of T/F
- For each 3-clause (ℓ_1 ∨ ℓ_2 ∨ ℓ_3), a gadget: a small graph formed by composing two OR gates
- Connect so that T and N must receive different colours for the gadget to be satisfied

This reduction is formalised in Garey-Johnson-Stockmeyer (1976); result: 3COLOR is NP-complete.
(Source: Arora-Barak §2.4 example)

### 4.3 HAMILTONIAN-CYCLE Reduction

Karp reduction 3SAT → directed Hamiltonian cycle → undirected. For each variable a "widget" (long chain),
for each clause a "gadget" (structure that crosses over to one of three branches). See Papadimitriou §9.2
for a detailed construction.

### 4.4 NP Problems Believed Not to Be NP-complete

- Integer factoring (factoring) — known to lie in NP ∩ coNP even before Shor's algorithm. Expected not to
  be NP-complete (a leading candidate for an "intermediate" status, neither in P nor NP-complete).
- Graph isomorphism — Babai 2015 gives quasipolynomial time. NP-completeness is structurally unlikely.

---

## 5. coNP, NP ∩ coNP, BPP, and Related Classes

### 5.1 coNP-complete

TAUTOLOGY and UNSAT are coNP-complete. For an NP-complete L, L̄ is coNP-complete.

### 5.2 NP = coNP?

Open. NP = coNP would mean every NP problem has a polynomial-length refutation certificate. For example,
whether TAUTOLOGY has a polynomial-length "certificate of truth" is unclear.

### 5.3 NP ∩ coNP

P ⊆ NP ∩ coNP. Representative candidates:
- Factoring (decision form)
- Parity games
- Nash equilibrium (2-player zero-sum): P

Whether P = NP ∩ coNP is also open.

### 5.4 Probabilistic and Interactive Classes

P ⊆ RP ⊆ BPP ⊆ PSPACE, NP ⊆ MA ⊆ AM ⊆ ... ⊆ IP = PSPACE (Shamir 1992). Outside the scope of this note.
Treated in P2.

---

## 6. Hierarchy Theorems

### 6.1 Time Hierarchy Theorem

(Hartmanis-Stearns 1965) If f is "time-constructible",

```
  DTIME(f(n)) ⊊ DTIME(f(n) · log f(n))
  DTIME(f(n)) ⊊ DTIME(f(n)²)
```

Consequence: P ⊊ EXP. Hence "almost all" functions are unsolvable within any polynomial bound.

### 6.2 Space Hierarchy Theorem

(Stearns-Hartmanis-Lewis 1965) For space-constructible S,

```
  DSPACE(S(n)) ⊊ DSPACE(S(n) · log S(n))
```

Consequence: L ⊊ PSPACE.

### 6.3 P and PSPACE Relation

P ⊆ NP ⊆ PSPACE ⊆ EXP. Whether P = PSPACE is open, but many complexity theorists strongly believe in
separation (time hierarchy already gives P ⊊ EXP).

### 6.4 Natural Proofs Barrier

Razborov-Rudich 1997: "Natural-proof" techniques cannot demonstrate P ≠ NP (assuming pseudorandom generators
exist). A major barrier against P vs NP.

---

## 7. Link to n=6 (memo only)

1. In the SAT → 3SAT reduction, clause length 3 is the minimum NP-hard length. 2SAT is in P. The "3"
   boundary number relates to the NP hierarchy structure but has no direct mathematical connection to n=6
   ([N?]).
2. The observation that τ=4 (divisor count at 1, 2, 3, 6) resembles the literal-count distribution in CNF
   standard form of SAT is recorded in atlas.n6 §L6-complexity as [N?]. An argument path is unconfirmed.
3. The Cook-Levin reduction tableau grid size O(p(n)²) has no special behaviour at n=6 in polynomial
   coefficients. Under the no-pattern-matching-forcing principle this observation remains [N?].

Per the no-self-reference-verification principle, the above three remain observational; the necessity of
n=6 is not used as evidence.

---

## 8. Practice — Five by Hand

**P1.** Directly construct the 3SAT ≤_p CLIQUE reduction. Hint: with m 3-clauses, take 3m graph nodes,
connect two literals in different clauses by an edge iff they are not contradictory. Then φ is satisfiable
⟺ G has an m-clique.

**P2.** Show that 3COLOR ≤_p CLIQUE may not hold. That is, 3COLOR and CLIQUE are both NP-complete but
reductions always go via 3SAT. Think about whether a "direct" ≤_p reduction between the two is possible.

**P3.** Demonstrate that SUBSET-SUM is NP-complete by reducing from 3SAT. (Karp 1972 original.)

**P4.** Directly show TAUTOLOGY is coNP-complete. Hint: construct SAT ≤_p TAUTOLOGY via negation.

**P5.** Show that if L ∈ NP ∩ coNP and L is NP-complete, then NP = coNP.

---

## 9. Reading Path and Next Step

### 9.1 Review Order

Week 1: Sipser §7 (P, NP, polynomial reduction) + §8 (space complexity)
Week 2: Arora-Barak §1-2 (TM, NP definition, Cook-Levin)
Week 3: Arora-Barak §3 + §6 (NP-complete reduction examples, circuits)
Week 4: Papadimitriou §2-8 (completeness extensions, coNP, PSPACE)

### 9.2 Preparation for P2

- Circuit complexity (attempts at lower bounds)
- Interactive proofs (IP = PSPACE)
- PCP theorem and hardness of approximation
- Natural-proofs barrier, relativization barrier

### 9.3 Connections

- PROB-P1-2 (BT-542 P vs NP): discusses barriers and strategies on the basis of this note
- PROB-P1-5 (BT-545 Hodge): Hodge lies at the algebraic-geometry/topology interface, but its computational
  aspects (decidability of the Hodge candidate) indirectly link to complexity

---

## 10. Sources

- Arora-Barak "Computational Complexity: A Modern Approach" Cambridge 2009 — standard modern complexity
  textbook, §1-6
- Sipser "Introduction to the Theory of Computation" 3rd ed. 2013 — undergraduate-level basics, §7-8
- Papadimitriou "Computational Complexity" Addison-Wesley 1994 — structural approach to completeness and
  reductions, §2-§8
- Garey-Johnson "Computers and Intractability" Freeman 1979 — dictionary of NP-complete problems
- Cook "The Complexity of Theorem-Proving Procedures" STOC 1971 — Cook's theorem original paper
- Karp "Reducibility Among Combinatorial Problems" 1972 — list of 21 NP-complete problems
- Razborov-Rudich "Natural Proofs" JCSS 1997 — barrier discussion

This note selects and reorganises the P1-volume portions of these 7 sources.

---

## 11. Further Topic — Overview of Barriers

A summary of the three major barriers that explain why P vs NP is difficult, for P1 study.

### 11.1 Relativization Barrier (Baker-Gill-Solovay 1975)

There exist oracles A with P^A = NP^A and B with P^B ≠ NP^B. Hence "any relativising argument" cannot
decide P vs NP. Standard techniques such as diagonalisation fail.

### 11.2 Natural Proofs Barrier (Razborov-Rudich 1997)

If "natural" arguments (large + constructive + useful) demonstrated strong circuit lower bounds
(super-polynomial), they would contradict the existence of pseudorandom generators. Most existing
circuit-lower-bound techniques are natural.

### 11.3 Algebrization Barrier (Aaronson-Wigderson 2009)

An extension of relativization. Even allowing algebraic oracles, oracles exist on both sides. Combined with
natural proofs, most existing techniques are ruled out.

### 11.4 All Three Barriers Must Be Avoided

A demonstration of P vs NP must avoid all three barriers. Currently known techniques do not satisfy all
three conditions at once. New mathematical ideas are needed.

---

## 12. Further Topic — Fine-Grained Complexity

Even within P there are strong theoretical walls between O(n²) and O(n^{2.37}). Assuming SETH (Strong
Exponential Time Hypothesis), the following relations hold:

- edit distance essentially O(n²)
- 3SUM essentially O(n²)
- APSP essentially O(n³)

These are hypothesis-based classifications for "sub-lower-bounds" within P. Independent of P vs NP.

---

## 13. Further Topic — Overview of Quantum Complexity

BQP is the complexity class of quantum computers. The following relations are known:

- P ⊆ BPP ⊆ BQP ⊆ PSPACE
- Shor's algorithm: factoring ∈ BQP (believed to be in NP but not NP-complete)
- BQP vs NP relation open (no separation in either direction)

Outside the P1 scope but noted briefly since it connects directly to P vs NP discussions.

---

## 14. Appendix — PCP Theorem (reference)

### 14.1 PCP Theorem Statement (Arora-Safra, Arora-Lund-Motwani-Sudan-Szegedy 1992)

```
  NP = PCP[O(log n), O(1)]
```

That is, "witnesses" of NP problems can be probabilistically verified with O(log n) random bits and O(1)
queried bits. A revolutionary theorem.

### 14.2 Hardness of Approximation

Consequence of PCP: lower bounds on approximation algorithms for many optimisation problems. Example:
MAX-3SAT 8/7-approximation is NP-hard (Håstad 2001).

### 14.3 UGC — Unique Games Conjecture

Khot 2002 Unique Games Conjecture: 2-ε approximation of Vertex Cover is NP-hard. Many approximation lower
bounds depend on UGC.

---

## 15. Appendix — Main Complexity-Class Chart

| Class | Definition | Representative complete problem | Inclusion |
|-------|------|---------------|------|
| L | log-space | - | P |
| NL | log-space non-deterministic | s-t reachability | NP |
| P | polynomial-time deterministic | circuit value | NP |
| NP | polynomial-time non-deterministic | SAT | PSPACE |
| coNP | complement of NP | TAUTOLOGY | PSPACE |
| BPP | polynomial-time probabilistic | - | PSPACE |
| PSPACE | polynomial space | TQBF | EXP |
| EXP | exp(poly) time | - | NEXP |

---

## 16. Next Documents

- PROB-P1-2 : BT-542 P vs NP deep dive (uses the foundation of this note)
- PROB-P1-5 : BT-545 Hodge deep dive
- N6-P1-3 : n=6 honesty principle

Completing this document wraps up the P1-PURE track. We next enter the P1-PROB track deep dive (BT-543-547).
