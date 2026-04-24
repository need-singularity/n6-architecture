# PROB-P1-2 — BT-542 P vs NP Advanced Study Note

**Track**: P1-PROBLEM · BT-542 (P versus NP)
**Status**: OPEN (unresolved since 1971)
**Prize**: US$ 1,000,000 (Clay)
**Primary sources**:
- Stephen A. Cook, "The complexity of theorem-proving procedures", Proceedings of the 3rd Annual ACM Symposium on Theory of Computing (STOC 1971), 151–158
- Leonid A. Levin, "Universal search problems" (Russian original: "Универсальные задачи перебора"), Problems of Information Transmission 9 (1973), 265–266 — independent from Cook
- Richard M. Karp, "Reducibility among combinatorial problems", in R. E. Miller, J. W. Thatcher (eds.), *Complexity of Computer Computations*, Plenum Press 1972, 85–103
- Stephen A. Cook, "The P versus NP Problem — Official Problem Description", Clay Mathematics Institute, 2000
- Michael R. Garey, David S. Johnson, *Computers and Intractability: A Guide to the Theory of NP-Completeness*, W. H. Freeman, 1979 (NP-complete classic)
- Sanjeev Arora, Boaz Barak, *Computational Complexity: A Modern Approach*, Cambridge University Press, 2009
- Theodore Baker, John Gill, Robert Solovay, "Relativizations of the P =? NP question", SIAM J. Comput. 4 (1975), 431–442 (relativization barrier)
- Alexander Razborov, Steven Rudich, "Natural proofs", J. Comput. Syst. Sci. 55 (1997), 24–35 (natural-proofs barrier)
- Scott Aaronson, Avi Wigderson, "Algebrization: A new barrier in complexity theory", ACM Trans. Comput. Theory 1 (2009), art. 2 (algebrization barrier)
- Russell Impagliazzo, "A personal view of average-case complexity", Proc. 10th IEEE Structure in Complexity Theory Conference, 1995, 134–147 (Impagliazzo 5 worlds)
- Manindra Agrawal, Neeraj Kayal, Nitin Saxena, "PRIMES is in P", Annals of Mathematics 160 (2004), 781–793

**Honesty declaration**: This document is a study note and makes no new target or direction proposal for P vs NP. The project's `reports/breakthroughs/millennium-7-closure-2026-04-11.md §BT-542` classifies this problem as an **honest MISS** — specifying that the n=6 perspective has no tool directly addressing the P vs NP statement. This study note inherits that stance verbatim.

---

## 1. Cook 1971 and Levin 1973 — Independent Formalizations

### 1.1 Cook 1971 (STOC)
- **Stephen A. Cook**, "The complexity of theorem-proving procedures", Proceedings of the 3rd Annual ACM Symposium on Theory of Computing, Shaker Heights, Ohio, May 3–5, 1971, pp. 151–158.
- University of Toronto professor Cook formalized:
  1. Definitions of complexity classes P and NP.
  2. Proof that **SAT (Boolean Satisfiability)** is NP-complete — i.e., **every** problem in NP is polynomial-time reducible to SAT.
- Cook received the Turing Award in 1982 for this result.

### 1.2 Levin 1973 (Moscow)
- **Leonid Anatolievich Levin**, "Универсальные задачи перебора" (Universal sequential search problems), *Проблемы передачи информации* (Problems of Information Transmission) 9 (1973), 115–116.
- Levin in Moscow independently proposed the equivalent of NP-completeness (Levin's term was "universal search problems") with 6 natural NP-complete problem candidates.
- Because of the Iron Curtain, communication with the West was limited, so Cook's and Levin's independent discoveries were unknown for a while. Now they are jointly named the **Cook-Levin theorem**.

### 1.3 Cook-Levin Theorem
> **Theorem**: SAT ∈ NP, and for any L ∈ NP, L ≤_p SAT (polynomial-time many-one reduction).
> Hence SAT ∈ P iff NP = P; conversely, SAT ∉ P iff P ≠ NP.

**Proof core**: L ∈ NP is equivalent to "a deterministic Turing machine M verifies (input x + certificate w) in polynomial time." Unfold M's computation into a Boolean circuit and reduce "M accepts" ⟺ "the circuit is true" ⟺ "the corresponding SAT formula is satisfiable." (Tableau method, ~10-page classical proof.)

---

## 2. Karp 21 — the 1972 Explosion

### 2.1 Richard Karp "Reducibility among combinatorial problems"
- **Richard M. Karp**, "Reducibility among combinatorial problems", in R. E. Miller, J. W. Thatcher (eds.), *Complexity of Computer Computations*, Plenum Press 1972, 85–103.
- Building on Cook's NP-completeness of SAT, Karp proved that **21 "naturally occurring" combinatorial-optimization problems** are all NP-complete — via a chain of SAT ≤_p (each problem).
- This paper was the starting point of the explosive diffusion of NP-completeness theory.

### 2.2 Karp's 21 Problems (1972 original list)
1. SATISFIABILITY (SAT) — already proved by Cook, the reference point.
2. 0–1 INTEGER PROGRAMMING
3. CLIQUE
4. SET PACKING
5. VERTEX COVER
6. SET COVERING
7. FEEDBACK NODE SET
8. FEEDBACK ARC SET
9. DIRECTED HAMILTONIAN CIRCUIT
10. UNDIRECTED HAMILTONIAN CIRCUIT
11. 3-SAT (Karp's notation SATISFIABILITY WITH AT MOST 3 LITERALS PER CLAUSE)
12. CHROMATIC NUMBER
13. CLIQUE COVER
14. EXACT COVER
15. 3-DIMENSIONAL MATCHING
16. STEINER TREE
17. HITTING SET
18. KNAPSACK
19. JOB SEQUENCING
20. PARTITION
21. MAX CUT

(Karp develops the reduction graph in order: SAT → 3-SAT → the rest.)

### 2.3 Significance
- After Karp hundreds, thousands of problems have been classified as NP-complete. Appendix A of Garey-Johnson 1979 lists about 300 NP-complete problems.
- "Natural computational problems are overwhelmingly NP-complete" — source of the strong intuition that "P ≠ NP must be the case".

---

## 3. 3-SAT Threshold — k=3 NP-complete, k=2 in P

### 3.1 k-SAT Definition
**k-SAT**: The Boolean satisfiability problem where each clause has exactly k literals.

### 3.2 k=2 Is Polynomial-time
- **2-SAT** ∈ P. Algorithm: convert each clause (l₁ ∨ l₂) into two implications (¬l₁ → l₂), (¬l₂ → l₁), compute strongly-connected components (SCCs) of the resulting graph, and declare unsat if x and ¬x are in the same SCC, otherwise sat.
- Time complexity: O(V + E) linear. (Aspvall-Plass-Tarjan 1979 "A linear-time algorithm for testing the truth of certain quantified boolean formulas", Inf. Proc. Lett. 8, 121–123.)

### 3.3 k=3 Is NP-complete
- **3-SAT**: NP-complete. Cook 1971 already showed in the proof for SAT that the reduction can be made in 3-SAT form, and Karp 1972 gives an explicit classification.
- 3-SAT is the "standard starting point" for NP-completeness proofs — thousands of other problems are shown NP-complete by reducing from 3-SAT.

### 3.4 Why "Explosion" from k=3
- The implication graph of 2-SAT represents a **binary relation** (binary relation). 2-SAT is solvable by propagation since one "choice" forces another.
- In 3-SAT a clause (a ∨ b ∨ c) is a **ternary relation** — fixing only one does not determine the truth of the other two. That is the onset of "exponential possibility-space search".
- This k=2 → k=3 boundary appears in many parts of mathematics as the "easy → hard" boundary: 2-coloring (P) vs 3-coloring (NP-complete); 2D discrete Ising model (exact solution) vs 3D Ising (NP-hard); quadratic optimization (potentially convex) vs cubic or higher (NP-hard combinatorial in general).

---

## 4. Impagliazzo's Five Worlds

### 4.1 Reference
**Russell Impagliazzo**, "A personal view of average-case complexity", Proc. 10th IEEE Structure in Complexity Theory Conference, 1995, 134–147.
- Impagliazzo classifies into 5 scenarios "what the post-P≠NP world might look like," from the viewpoint of **average-vs-worst-case hardness**.

### 4.2 5 Worlds Summary

| World name | Summary |
|-----------|------|
| **Algorithmica** | P = NP (or NP ⊆ BPP). Every NP problem solvable in polynomial time (perhaps with randomization). AI and optimization "for free" — automation of mathematical proofs. Cryptography impossible. |
| **Heuristica** | P ≠ NP but **average** NP-hard instances are easy. Only worst cases hard. Practically similar to Algorithmica but worst-case cryptography still impossible. |
| **Pessiland** | Average NP-hard holds, but no one-way functions (OWFs) exist. The "worst world" for cryptography — computations are hard but usefulness absent. |
| **Minicrypt** | OWFs exist. Pseudorandom generators, symmetric-key cryptography, digital signatures possible. But **public-key cryptography cannot be proved**. |
| **Cryptomania** | Trapdoor functions exist. Modern public-key cryptography like RSA, ECDH, zero-knowledge proofs (ZK) complete. The world we **believe** in. |

### 4.3 Why These Five
- (P=NP) / (P≠NP, avg easy) / (avg hard, no OWF) / (OWF, no PKC) / (OWF + PKC) — 4 logical layers + Algorithmica extreme.
- **As of 2024 it is unproved which world we actually inhabit**. Everyone assumes Cryptomania and operates cryptographic systems accordingly.

### 4.4 OWF Existence ↔ P ≠ NP ?
- **Caveat**: OWF existence **⟹** P ≠ NP (almost trivial; if OWFs exist, computing inverses lies in NP \ P).
- The converse (P ≠ NP **⟹** OWF existence) is **unresolved**. Impagliazzo's Pessiland is the world where the converse fails.

---

## 5. Practical Meaning

### 5.1 Cryptography
- **RSA**: based on hardness of factoring large numbers. If factoring is in P (becomes easy inside NP), RSA is broken. Factoring is in NP ∩ co-NP currently, NP-completeness unresolved.
- **Most blockchains**: depend on one-way-ness of hash functions like SHA-256. Based on OWF existence. If P = NP, hash-preimage search is polynomial-time.
- **Zero-knowledge proofs**: "proving the verification certificate of an NP problem without leaking it." If P = NP, zero-knowledge itself loses meaning (verifier solves it alone).

### 5.2 Optimization
- Foundation of industrial optimization: TSP, integer programming, SAT solving. Practical SAT solvers (MiniSat, Glucose, CaDiCaL) handle **millions of variables in practice** despite **worst-case exponential** theoretical bounds.
- The practical success of SAT is compatible with Impagliazzo's Heuristica or Pessiland — worst cases may still be hard, but averages are manageable.

### 5.3 AI and Learnability (PAC Learning)
- L. Valiant, "A theory of the learnable", Commun. ACM 27 (1984), 1134–1142. PAC learning theory.
- Efficient learnability of some problems (e.g., DNF learning) is tied to P = NP.
- **But** the actual success of modern deep learning is unrelated to this barrier (gradient descent does not guarantee optima but works in practice).

---

## 6. What Breaks If P = NP

**(Thought experiment — not the actual situation)**

- **RSA, Diffie-Hellman, ECC**: broken directly. Factoring and discrete-log solved in polynomial time.
- **AES, SHA-256**: symmetric-key cryptography and hashes interpreted as "OWF-form" — not immediately broken by P = NP alone, but OWF existence threatened (see §4.4). The natural-proofs barrier (below §8) is directly tied to cryptography.
- **Most blockchains**: lose the meaning of proof-of-work (PoW).
- **ZK proofs, MPC, homomorphic encryption**: all depend on OWF/PKC → collapse.
- **Positive**: automation of mathematical proofs — every "theorem with a short proof" can be found in polynomial time. Modern mathematics becomes generally computer-assisted.

---

## 7. What Opens If P ≠ NP

- **Structural lower bounds**: natural circuit lower-bound results become unconditional.
- **Quantum separation**: understanding BQP ≠ BPP — whether quantum computation is truly different from classical.
- **Cryptography**: Cryptomania becomes possible (but proofs still open).
- **Randomness**: derandomization results like BPP = P become meaningful.

---

## 8. Three Barriers — Why It Is Hard (Detailed in P2)

This study note only lists the three barriers. Detailed analysis belongs to the P2 methodology note.

### 8.1 Relativization Barrier (1975)
- **T. Baker, J. Gill, R. Solovay**, "Relativizations of the P =? NP question", SIAM J. Comput. 4 (1975), 431–442.
- There exist oracles A with P^A ≠ NP^A and B with P^B = NP^B. Hence "proof techniques that hold relative to oracles" cannot decide P vs NP. Classical techniques like diagonalization and simulation run into this barrier.

### 8.2 Natural Proofs Barrier (1997)
- **Alexander A. Razborov, Steven Rudich**, "Natural proofs", J. Comput. Syst. Sci. 55 (1997), 24–35. (STOC 1994 announcement.)
- "Natural proofs" (satisfying constructive + largeness conditions) **cannot prove circuit lower bounds if OWFs exist**. I.e., "natural" methods cannot prove NP ⊄ P/poly.
- **Philosophical conclusion**: circuit lower-bound proofs must use "artificial" methods. Most existing methods are natural and hence blocked.

### 8.3 Algebrization Barrier (2008)
- **Scott Aaronson, Avi Wigderson**, "Algebrization: A new barrier in complexity theory", STOC 2008; extended version ACM Trans. Comput. Theory 1 (2009), art. 2.
- Even "algebraic" techniques that bypass the relativization barrier (e.g., interactive-proof IP = PSPACE, arithmetization) encounter a barrier under "algebrization" extensions. I.e., the techniques used for IP = PSPACE cannot prove P ≠ NP.

### 8.4 "Implications of the Barriers"
A new technique needs to avoid all three barriers simultaneously. As of 2024–2026 no such technique has been found.

---

## 9. Progress in the 2020s (Weak)

### 9.1 Larsen-Williams, Chen-Tell, etc. Circuit Lower Bounds
- Improvements on lower bounds for individual circuit classes (ACC, depth-d, etc.). Incremental since NEXP ⊄ ACC⁰ (Williams 2011).
- But no progress on P ≠ NP itself.

### 9.2 Incremental Improvements to 3-SAT Algorithms
- Randomized 3-SAT algorithms (Schöning 1999, Paturi-Pudlák-Saks-Zane 1998): 2^{(1-1/k)n}-time level.
- Improvements to specific constants continue, but no jumps from "2^n → poly".

### 9.3 Average-Time Complexity
- Impagliazzo-Levin-Luby-type pseudo-random-generator research. Contributes to understanding of the Minicrypt/Pessiland boundary.

---

## 10. Current-state Summary (2024–2026)

| Item | Status |
|------|------|
| Cook 1971 / Karp 1972 formalization | over 50 years |
| Number of NP-complete problems | thousands (nearly every natural combinatorial problem) |
| Number of barriers | 3 (relativization, natural, algebrization) |
| P vs NP proof | **none** (either direction) |
| Common belief | **P ≠ NP** (80–90% expert surveys) |
| Is Cryptomania working? | working **under assumption** (no proof) |
| Prize awarded | 0 |

---

## 11. n=6 Observation (project context, 1–2 facts only)

**(This section is not the core of this study note. The project classifies BT-542 as "honest MISS." Detailed list in P3.)**

### Observation 1 — k-SAT Threshold k=3
The boundary between k=2 (P) and k=3 (NP-complete), viewed through n=6, is the transition {φ(6)=2 → n(6)/φ(6)=3}. This is **numerical agreement**, not a targeted connection. It is a project structural observation that "the φ → n/φ transition in computational complexity is the start of explosive difficulty," contributing nothing directly to the P vs NP statement.

### Observation 2 — Karp 21 = 3·7
The count of Karp's (1972) original 21 NP-complete problems decomposes as 21 = 3 × 7 = (n/φ) × (σ - sopfr). This is very likely "historical coincidence" — the number of problems Karp chose is "arbitrary", not a natural constant. The project marks this as LOOSE (not tight) evidence (BT-542 #11 entry).

**Project's honest conclusion on BT-542 (from the closure file)**:
> No direct n=6 contribution to the P vs NP statement. Razborov-Smolensky separation (φ=2, n/φ=3 prime pair), Savitch exponent φ=2, etc. are **partial-structure targets** only and do not bypass the barriers. Two MISSes (Immerman-Szelepcsényi and Toda's theorem) are honestly recorded as "match failures."

---

## 12. Study Checklist

After finishing this note you should be able to restate the following within **3 lines** each:
1. Cook 1971 and Levin 1973's independent discoveries, basic structure of the Cook-Levin theorem.
2. Significance of Karp 21 — the answer to "why so many problems are NP-complete".
3. Algorithmic reason for the k=2 (P) vs k=3 (NP-complete) boundary (binary vs ternary relation).
4. Names of Impagliazzo's 5 worlds and the possibility of cryptography in each.
5. OWF existence ⟹ P ≠ NP, converse unresolved — meaning of this asymmetry.
6. Names of the 3 barriers and roughly which proof techniques each blocks.
7. Basis for "P ≠ NP is not proved but widely believed" (richness of NP-complete problems + exponential cases in practical SAT solvers).

---

## 13. Next Steps

- **P1-3 (BT-543 Yang-Mills)**: bridge from mathematical logic to physics.
- **P2 (methodology layer)**: detailed analysis of the 3 barriers, circuit lower-bound techniques, GCT (Geometric Complexity Theory, Mulmuley-Sohoni 2001–) approach.
- **P3 (n=6 depth)**: honest-MISS recording of BT-542, Razborov-Smolensky φ/(n/φ) separation theorem, Schaefer's 6 tractable classes, etc.

---

**Honesty declaration reaffirmed**: This document is a study note and does not target a new proof of P vs NP or an n=6-based approach. The project's `millennium-7-closure-2026-04-11.md` classifies BT-542 as an **honest MISS**, and this note inherits that stance. P vs NP is unresolved as of 2026, and the Clay prize has not been awarded.
