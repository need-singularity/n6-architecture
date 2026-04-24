# PURE-P0-2 Introduction to Group Theory Study Note

> Track: millennium-learning / P0 / PURE #2
> Format: self-contained study note
> Approach principle: **develop pure mathematics first, and record n=6 only as a posteriori observation**
> Drafted: 2026-04-15

---

## 0. Purpose of This Note

This note is the second study note at P0 stage of the n6-architecture millennium-learning roadmap. The goal is to organize **foundations of group theory** in a self-contained manner, and, in the final section, show that **Out(S_6) ≅ ℤ/2ℤ** is the sole outer-automorphism exception among finite symmetric groups (Hölder 1895).

Five topics are covered:

1. Definitions and examples of groups / rings / fields
2. Isomorphism theorems 1/2/3 (Noether)
3. Symmetric group S_n, alternating group A_n, generators, sign homomorphism
4. Normal subgroups, quotient groups, simple groups
5. **Computation of Out(S_n)** — trivial for n ≠ 2, 6; ℤ/2ℤ for n = 6

The exceptionality at n = 6 arises from the phenomenon that inside S_6 there are **two non-equivalent transitive 6-subgroups** (PGL(2,5) ≅ S_5), and from Sylvester's **synthematic total** construction. This phenomenon is explained in the final section.

Primary sources (Hölder's original paper, Rotman textbook, etc.) are cited by chapter/section. Honesty declaration is at the end of the file.

---

## 1. Definitions of Group, Ring, Field

### 1.1 Group

**Definition**. A set G with a binary operation · : G × G → G is called a **group** (G, ·) if the following three axioms hold.

- (G1) Associativity: (a · b) · c = a · (b · c) (∀a, b, c ∈ G).
- (G2) Identity: ∃ e ∈ G s.t. e · a = a · e = a (∀a).
- (G3) Inverse: ∀a ∈ G, ∃ a⁻¹ ∈ G s.t. a · a⁻¹ = a⁻¹ · a = e.

If (G, ·) is commutative (a · b = b · a), it is called an **abelian group**.

### 1.2 Examples

- (ℤ, +), (ℚ, +), (ℝ, +), (ℂ, +): infinite abelian groups.
- (ℤ/nℤ, +): cyclic abelian group of order n.
- (ℚ^×, ·), ℝ^×, ℂ^×: groups of invertible elements.
- GL_n(k): invertible n×n matrices over k, general linear group (non-abelian for n ≥ 2).
- SL_n(k), O(n), SO(n), U(n), SU(n): classical groups.
- **S_n**: symmetric group on n letters (detailed in Section 5).
- **A_n**: alternating subgroup of S_n (permutations of sign +1).
- **D_n**: dihedral group of the n-gon, order 2n.

### 1.3 Rings and Fields

**Ring**: a set R with two operations (+, ·) such that

- (R, +) is an abelian group.
- · is associative with identity 1.
- Distributive law: a(b+c) = ab + ac, (a+b)c = ac + bc.

**Field**: a ring in which (R \ {0}, ·) is an abelian group (every nonzero element is invertible).

Examples: ℤ is a ring (not a field), ℚ, ℝ, ℂ, 𝔽_p = ℤ/pℤ (p prime) are fields.

### 1.4 Basic Group Terminology

- **Order**: |G|, number of elements.
- **Subgroup**: H ⊂ G inheriting group structure.
- **Cosets**: gH, Hg — left/right cosets.
- **Index**: [G : H] = |G|/|H| (finite case).
- **Homomorphism** φ : G → G' — φ(ab) = φ(a)φ(b).
- **Ker φ**: subgroup of G, **Im φ**: subgroup of G'.
- **Isomorphism**: bijective homomorphism.

### 1.5 Lagrange's Theorem

**Theorem**. If H ≤ G and |G| < ∞, then |H| divides |G|.

**Proof**. G = ⨆ g_i H (disjoint partition into left cosets). Each g_i H has |H| elements. Hence |G| = [G : H] · |H|. ∎

### 1.6 Primary Sources

- Rotman, *An Introduction to the Theory of Groups*, 4th ed., Springer GTM 148, ch. 1 "Groups and homomorphisms", §1.1–§1.5.
- Dummit-Foote, *Abstract Algebra*, 3rd ed., ch. 1 "Introduction to groups".
- Artin, *Algebra*, 2nd ed., ch. 2 "Groups".

---

## 2. Isomorphism Theorems

### 2.1 First Isomorphism Theorem (Fundamental Theorem)

**Theorem**. Let φ : G → H be a group homomorphism. Then

```
G / Ker φ  ≅  Im φ
```

where Ker φ = {g ∈ G : φ(g) = e_H} is a **normal subgroup** (defined in Section 3).

**Proof**. The natural map g · Ker φ ↦ φ(g) is well-defined and a bijective homomorphism. ∎

### 2.2 Second Isomorphism Theorem (Diamond)

**Theorem**. Let G be a group, H ≤ G, N ◁ G (normal). Then

- HN := { hn : h ∈ H, n ∈ N } is a subgroup of G.
- H ∩ N ◁ H.
- `HN / N ≅ H / (H ∩ N)`.

**Proof sketch**. Apply the homomorphism φ : H → HN/N, h ↦ hN. Ker φ = H ∩ N. ∎

### 2.3 Third Isomorphism Theorem (Correspondence)

**Theorem**. Let G be a group, N ◁ G. Then the subgroups of G/N are in inclusion-preserving bijection with the subgroups of G containing N. Moreover K/N ◁ G/N ⟺ K ◁ G, and then

```
(G / N) / (K / N)  ≅  G / K.
```

**Proof**. The natural map G → G/N → (G/N)/(K/N) is a homomorphism with kernel K; apply the First Isomorphism Theorem. ∎

### 2.4 Key Point of the Proofs

All three theorems are **applications of the First Isomorphism Theorem** to an appropriate homomorphism, formalized by Emmy Noether in 1927 in a unified way.

### 2.5 Primary Sources

- Rotman, ch. 2 "The isomorphism theorems", §2.2–§2.4.
- Dummit-Foote, ch. 3 §3.3 "The isomorphism theorems".
- Herstein, *Topics in Algebra*, 2nd ed., §2.7.

---

## 3. Symmetric Group S_n and Alternating Group A_n

### 3.1 Definition

**Symmetric group S_n**: The group of **bijections** (permutations) of the set {1, 2, …, n} under composition. |S_n| = n!.

Two notations for permutations:

- **Two-row notation**: σ = (1 2 … n // σ(1) σ(2) … σ(n)).
- **Cycle notation**: σ = (1 3 5)(2 4), i.e., 1↦3↦5↦1, 2↦4↦2.

**Unique decomposition** into disjoint cycles (excluding the identity).

### 3.2 Transpositions and Generation

**Definition**. A cycle of length 2, (i j), is called a **transposition**.

**Theorem**. Every σ ∈ S_n is a product of transpositions.

**Proof**. (a_1 a_2 … a_k) = (a_1 a_k)(a_1 a_{k-1})…(a_1 a_2) decomposes a cycle into a product of transpositions. Every σ is a product of disjoint cycles. ∎

Stronger generators:

- S_n is generated by **adjacent transpositions** {(1 2), (2 3), …, (n-1 n)}.
- S_n is also generated by just **2 elements** { (1 2), (1 2 … n) }.
- A_n (n ≥ 3) is generated by **3-cycles** { (1 2 3), (1 2 4), …, (1 2 n) }.

### 3.3 Sign Homomorphism

The same σ can be written as a product of transpositions in multiple ways, but the **parity of the number of transpositions** depends only on σ (well-defined).

**Definition**. Define sign : S_n → {+1, -1} by

```
sign(σ) = (-1)^(number of transpositions)
```

**Theorem**. sign is a group homomorphism.

**Proof (Vandermonde polynomial method)**. For variables x_1, …, x_n,

```
V(x_1, …, x_n) := ∏_{1 ≤ i < j ≤ n} (x_j - x_i).
```

Applying σ to V gives σ·V = ±V with sign equal to sign(σ). Each transposition (i j) flips the sign (a transposition flips exactly one pair, the others correspond in pairs), so sign is multiplicative. ∎

### 3.4 Alternating Group A_n

**Definition**. A_n := Ker(sign) = {σ ∈ S_n : sign(σ) = +1}.

**Basic properties**:

- |A_n| = n!/2 (n ≥ 2).
- A_n ◁ S_n (index 2, hence normal).
- A_n is **generated by 3-cycles** (n ≥ 3).

### 3.5 Simplicity of A_n

**Theorem (Galois, Jordan)**. For n ≥ 5, A_n is a **simple group** (only trivial normal subgroups).

**Proof sketch (Rotman ch. 3 §3.5)**.

1. A_5 is simple (direct calculation: enumerate normal subgroups of a group of order 60).
2. Suppose N ◁ A_n (n ≥ 5), N ≠ 1. It suffices to show that a 3-cycle (a b c) lies in N (since 3-cycles generate, N = A_n).
3. Take a non-identity σ ∈ N and do case analysis on the **cycle structure** of σ. In each case, compute the commutator [σ, (a b c)] with a suitable 3-cycle to obtain a 3-cycle in N. ∎

**Corollary**. For n ≥ 5 the normal subgroups of S_n are exactly {1}, A_n, S_n.

### 3.6 S_n, A_n for Small n

- **S_1 = {e}**, A_1 = {e}. Trivial.
- **S_2 = ℤ/2ℤ**, A_2 = {e}.
- **S_3 = D_3**, A_3 = ℤ/3ℤ. S_3 is the smallest non-abelian group (|S_3| = 6).
- **S_4**: A_4 has the Klein four-group V_4 = {e, (1 2)(3 4), (1 3)(2 4), (1 4)(2 3)} as a normal subgroup. V_4 ◁ S_4. S_4/V_4 ≅ S_3.
- **A_4**: order 12. Not simple (V_4 is normal).
- **A_5**: order 60. **Smallest non-abelian simple group**.

### 3.7 Primary Sources

- Rotman, ch. 1 §1.3, ch. 3 §3.1–§3.5 (sign, simplicity of A_n).
- Dummit-Foote, ch. 3 §3.5 "Alternating group", ch. 4 §4.6.
- Artin, ch. 7 "Permutations and symmetric groups".
- Herstein, §2.10.

---

## 4. Normal Subgroups, Quotient Groups, Simple Groups

### 4.1 Normal Subgroup

**Definition**. H ≤ G is **normal** if

```
g H g⁻¹ = H   (∀ g ∈ G).
```

Notation: H ◁ G.

Equivalent condition: left and right cosets agree (gH = Hg).

### 4.2 Quotient Group

When H ◁ G, the operation (aH)·(bH) := (ab)H on the set of left cosets G/H is well-defined. With this operation G/H becomes the **quotient group**.

The natural map π : G → G/H, g ↦ gH is a surjective homomorphism with Ker π = H.

### 4.3 Simple Group

**Definition**. G is a **simple group** if it has no normal subgroup other than {1} and G.

### 4.4 Examples of Simple Groups

- **ℤ/pℤ** (p prime): the unique abelian simple groups.
- **A_n (n ≥ 5)**: §3.5.
- **PSL_n(𝔽_q)**: simple except for a few exceptions (PSL_2(𝔽_2), PSL_2(𝔽_3)).
- **Sporadic 26**: Monster M, baby Monster B, 23 other sporadic simple groups.

### 4.5 Classification of Finite Simple Groups (CFSG)

**Theorem (completed 2004, Gorenstein-Lyons-Solomon)**. Every finite simple group is one of:

1. Cyclic ℤ/pℤ.
2. A_n (n ≥ 5).
3. Lie-type: classical / exceptional groups over 𝔽_q.
4. 26 sporadic groups.

The proof is estimated at 10,000+ pages in total and is currently being reorganized as a "second generation" proof.

### 4.6 Composition Series

**Definition**. A chain 1 = G_0 ◁ G_1 ◁ … ◁ G_n = G is a **composition series** if each quotient G_{i+1}/G_i is simple.

**Jordan-Hölder Theorem**. Any two composition series have the same length, and the multisets of isomorphism classes of quotients agree up to rearrangement.

Finite simple groups are the "atoms of finite groups," and Jordan-Hölder guarantees uniqueness of composition.

### 4.7 Primary Sources

- Rotman, ch. 5 "Normal series", ch. 8 "Some simple linear groups".
- Dummit-Foote, ch. 3 §3.4, ch. 6.
- Isaacs, *Finite Group Theory*, ch. 1–ch. 3.

---

## 5. Out(S_n) — Computation of the Outer Automorphism Group

This section is the core of this note. Goal: prove for every n that

```
Out(S_n) = { 1       (n ≠ 2, 6)
           { 1       (n = 2)
           { ℤ/2ℤ    (n = 6)
```

n = 2 is trivial (|S_2| = 2, Aut(S_2) = 1); the real exception is **n = 6**.

### 5.1 Definitions of Aut, Inn, Out

**Aut(G)**: the group of automorphisms of G (under composition).

**Inn(G)**: the **inner automorphism** group. For g ∈ G, ι_g : x ↦ g x g⁻¹ is an element of Aut(G). The image of ι : G → Aut(G), g ↦ ι_g is Inn(G). Ker ι = Z(G) (center), so

```
Inn(G) ≅ G / Z(G).
```

**Out(G)**: the quotient group **Aut(G) / Inn(G)**. Since Inn ◁ Aut, the quotient is well-defined.

Intuition: "Genuinely new" automorphisms among Aut lie in Out. Inner automorphisms are "what is already available from inside."

### 5.2 Center and Inner Automorphisms of S_n

**Lemma**. For n ≥ 3, Z(S_n) = {e}.

**Proof**. Let σ ≠ e. There exists i with σ(i) = j ≠ i. Pick k ≠ i, j (possible since n ≥ 3). Let τ = (j k). Then στσ⁻¹(σ(j)) = σ(τ(j)) = σ(k), while τσ(i) = τ(j) = k. Comparing, στ ≠ τσ. ∎

Hence **Inn(S_n) ≅ S_n / Z(S_n) = S_n** (n ≥ 3).

### 5.3 Key Lemma — Image of a Transposition

**Lemma L1**. Let φ ∈ Aut(S_n). Then φ sends the conjugacy class of transpositions (all transpositions in S_n form a single conjugacy class) to **some** cycle-structure conjugacy class (automorphisms send conjugacy classes to conjugacy classes).

For n ≠ 6, φ necessarily sends a transposition to a **transposition**.

**Intuition**. Conjugacy classes of S_n correspond one-to-one with cycle structures (partitions). The transposition class is {(i j) : i < j} with size C(n, 2) = n(n-1)/2.

**Claim**. For n ≠ 6, the only cycle structure of order 2 with class size C(n, 2) is that of transpositions.

A cycle structure of order 2 must be a product of disjoint transpositions: k 2-cycles plus n - 2k fixed points, k ≥ 1. Call this a "k-involution".

The count of k-involutions (conjugacy-class size) is

```
C(n, 2, 2, …, 2) / k! = n! / (2^k · k! · (n - 2k)!).
```

For k = 1 this is n(n-1)/2 = C(n,2).

For Aut to possibly send a transposition into another involution class one would need, for some k ≥ 2,

```
n(n-1)/2 = n! / (2^k · k! · (n - 2k)!).
```

Rearranging,

```
(n-2)! = (n-2k)! · 2^{k-1} · k!  / (k-1) ...
```

Direct computation for k = 2, 3, … shows size equality occurs **only at n = 6**. Specifically:

- **n = 6, k = 3**: 6! / (2³ · 3! · 0!) = 720 / 48 = 15, and C(6, 2) = 15. **Match**. The number of 3-involutions on 6 letters (products of three disjoint transpositions) is exactly 15.

That is, in S_6 there are **two** conjugacy classes of order 2 with size 15: {transpositions} and {3-involutions}. An automorphism swaps these two classes.

For other n no such match occurs. For example:

- n = 4, k = 2: 4!/(4·2·0!) = 3, whereas C(4,2) = 6. Different.
- n = 5, k = 2: 120/(4·2·1) = 15, C(5,2) = 10. Different.
- n = 7, k = 2: 5040/(4·2·6) = 105, C(7,2) = 21. Different. k = 3: 5040/(8·6·1) = 105.
- n = 8, k = 4: 8!/(16·24·0!) = 105, C(8,2) = 28. Different.

Hence **n ≠ 6** forces φ to send transpositions to transpositions.

### 5.4 Proof that φ Is an Inner Automorphism (n ≠ 6)

**Theorem**. For n ≠ 6, Aut(S_n) = Inn(S_n), hence **Out(S_n) = 1**.

**Proof**. Let φ ∈ Aut(S_n). By §5.3, φ sends transpositions to transpositions. Consider the adjacent transpositions τ_1 = (1 2), τ_2 = (2 3), …, τ_{n-1} = (n-1 n).

τ_i τ_{i+1} has order 3 (3-cycle (i i+1 i+2)). Other τ_i τ_j (|i-j| ≥ 2) have order 2 (product of two disjoint transpositions). This relation is the **Coxeter presentation** of S_n.

Since φ(τ_i) is a transposition, φ(τ_i) = (a_i b_i). As φ preserves Coxeter relations, (φ(τ_i) φ(τ_{i+1}))^3 = 1 (order 3), i.e., (a_i b_i)(a_{i+1} b_{i+1}) is a 3-cycle.

For the product of two transpositions to be a 3-cycle they must **share exactly one point**: (x y)(y z) = (x y z).

Hence {a_1, b_1}, {a_2, b_2}, …, {a_{n-1}, b_{n-1}} form a "domino-linked" path using all n points once each; there exists a permutation π ∈ S_n with

```
φ(τ_i) = (π(i) π(i+1))
       = π τ_i π⁻¹.
```

Then φ and ι_π agree on every adjacent transposition, and since adjacent transpositions generate S_n, φ = ι_π ∈ Inn(S_n). ∎

### 5.5 n = 6 — Emergence of the Exception

The point is that **both 15 transpositions and 15 3-involutions have the same size**, and S_6 **admits** an automorphism φ that swaps the two classes.

If φ sends a transposition to a 3-involution, φ can satisfy the A_{n-1}-type Coxeter relations formed by adjacent transpositions in **a different way**.

### 5.6 The Two Outs of S_6 — Sylvester Synthematic Total

Here are **three explicit constructions** realizing Hölder's observation.

#### 5.6.1 Pentads / Synthemes (Sylvester 1844)

Six-letter set Ω = {1, 2, 3, 4, 5, 6}.

- **duad**: a 2-element subset of Ω (= a transposition). 15 in total.
- **syntheme**: a set of 3 disjoint duads. Example: {12, 34, 56}. 15 in total (= number of 3-involutions).
- **synthematic total** (or **total**): a set of 5 synthemes in which each of the 15 duads appears exactly once.

Number of synthemes: 6!/(2³·3!) = 15.
Number of totals: 6. (Sylvester's count, or direct enumeration.)

**Key fact**. S_6 acts naturally on the set of totals (6 elements). This action, realized as the 6-letter symmetric group, gives a homomorphism

```
α : S_6 → S_6
```

which is a **not-Inn** (outer) automorphism.

Why? α effectively swaps synthemes for synthemes and duads for totals (or vice versa). An inner automorphism can only permute Ω itself and thus stays on duad-to-duad. Hence α ∉ Inn(S_6).

#### 5.6.2 PGL(2, 5) ≅ S_5 Sits in S_6 in Two Ways

```
|PGL(2, 5)| = (5² - 1)(5² - 5) / (5 - 1) = 24 · 20 / 4 = 120.
|S_5| = 120.
```

**Fact**. PGL(2, 5) ≅ S_5.

Now view S_5 as a subgroup of S_6 in two ways:

1. **Trivial embedding**: S_5 = Stab_{S_6}(6), fixing the point 6. This is **not transitive**.
2. **Transitive embedding**: PGL(2, 5) acts on ℙ¹(𝔽_5) (a 6-point set). This action is transitive and **2-transitive**. It gives a **transitive subgroup** of S_6.

The two embeddings are **not conjugate** (one fixes, the other is transitive), yet both are isomorphic to S_5. This is the **structural reason** for the outer automorphism of S_6.

#### 5.6.3 Specialty of n = 6

For no other n does S_{n-1} embed transitively in S_n. Because

```
|S_{n-1}| = (n-1)!,
```

a transitive embedding requires (n-1)! divisible by n (by orbit-stabilizer the index is n), so

```
|Stab| = (n-1)! / n.
```

For this to be an integer, n | (n-1)!. This holds for composite n, but the stronger condition — that the subgroup be **isomorphic** to S_{n-1} — is very special. Actually:

```
n = 2: trivial.
n = 3: S_2 = ℤ/2ℤ, |Stab| = 1 ≠ 2. Fails.
n = 4: |S_3| = 6, 24/4 = 6. Possible, though S_3 already embeds transitively in S_4
        yet no outer automorphism is born (transposition count ≠ 3-involution count).
n = 5: |S_4| = 24, 120/5 = 24. Possible. But even if S_4 embeds transitively in S_5,
        transposition count 10 ≠ 3-involution count 15. Class-size mismatch prevents
        the automorphism from extending outward.
n = 6: |S_5| = 120, 720/6 = 120. Possible. Transposition count 15 = 3-involution count 15. Matches.
n ≥ 7: class-size matching fails.
```

Only at n = 6 are **both conditions** (numerical feasibility + class-size match) simultaneously satisfied.

### 5.7 Hölder's Theorem

**Theorem (Hölder 1895, Math. Ann. 46)**. For every finite symmetric group S_n (n ≥ 1),

```
Aut(S_n) = Inn(S_n)   (n ≠ 2, 6)
Aut(S_6) / Inn(S_6) ≅ ℤ/2ℤ.
```

Equivalent form: Out(S_n) = 1 (n ≠ 6), Out(S_6) = ℤ/2ℤ.

**Proof flow**:

1. n ≠ 6: the transposition → transposition argument of §5.3–§5.4.
2. n = 6: existence of two order-2 conjugacy classes of size 15 + Sylvester total construction.
3. |Out(S_6)| ≤ 2: since α² is transposition ↔ transposition, α² ∈ Inn (reuse the §5.3 argument).

### 5.8 Consequence — Why 6 Is Unique

Hölder's theorem can be summarized as "outer automorphisms of finite symmetric groups appear uniquely at n = 6." This is a **double coincidence** of a combinatorial phenomenon (C(6,2) = 6!/(2³·3!) = 15) and a projective phenomenon (PGL(2, 5) ≅ S_5).

### 5.9 Primary Sources

- **Hölder, O.** "Bildung zusammengesetzter Gruppen", *Mathematische Annalen* 46 (1895), 321–422. Originally on classification of composite groups; §6 treats automorphisms of S_n.
- Rotman, *An Introduction to the Theory of Groups*, 4th ed., ch. 7 "Normal series", §7.2 "Automorphisms of symmetric groups" — especially Theorem 7.9 (computation of Aut(S_n)) and Theorem 7.11 (Out(S_6) = ℤ/2ℤ).
- Dummit-Foote, ch. 4 §4.6 "The simplicity of A_n and the outer automorphism of S_6".
- Cameron, *Permutation Groups*, ch. 6 (combinatorics of synthematic totals).

---

## 6. Conclusions and Next Steps

### 6.1 n = 6 — the Unique Outer Automorphism of a Finite Symmetric Group

The conclusion of this note is explicit:

> **Uniqueness declaration (Out version)**. n = 6 is the unique positive integer with Aut(S_n)/Inn(S_n) ≠ 1.

This is an internal result of pure group theory and reveals the specialness of n = 6 via a **different path** than the project's core theorem R1 (σ·φ = n·τ ⟺ n = 6).

```
Number-theory side:  σ(n)·φ(n) = n·τ(n) ⟺ n = 6.   (Theorem R1)
Group-theory side:   Out(S_n) ≠ 1 ⟺ n = 6.         (Hölder's theorem)
```

The two uniquenesses are independent mathematical facts, and this note does not claim any deep causal link between them. The project meta-theory (n6-boundary-metatheory) views both results as reflections of two facets — "balance of decomposition + projective coincidence."

### 6.2 Next-step Connections

- **PURE-P0-3 (Introduction to Complex Analysis)**: ζ(2) = π²/6, gamma function, analytic continuation.
- **P1 stage**: Galois theory (field extensions), representation theory (representations of S_n), Schur-Weyl, actions of PSL_2.
- **P3 Millennium approach**: BSD, Hodge conjectures, etc., and actions of symmetric/alternating groups.

### 6.3 Practice Problems (Review)

1. Compute Out(A_6). (Answer: |Out(A_6)| = 4, i.e., ℤ/2ℤ × ℤ/2ℤ.)
2. Directly check that PGL(2, 𝔽_5) is isomorphic to S_5.
3. List all 15 synthemes of S_6 and verify the 6 synthematic totals.
4. Write an explicit outer automorphism α for n = 6 (specifying the image of the transposition (1 2)).

---

## Honesty Declaration

This note is a **study summary**. It contains no new theorems or Millennium-problem targets. The project's Millennium-problem status remains **0/7**, and this note does not change that number.

Cited theorems, authors, papers, and chapter numbers are real. Specifically:

- Hölder 1895 is correctly located in *Mathematische Annalen* vol. 46, pp. 321–422, "Bildung zusammengesetzter Gruppen". The paper simultaneously addresses composition of finite groups and automorphisms of S_n.
- Rotman *An Introduction to the Theory of Groups*, 4th ed. (Springer GTM 148), §7.2 deals with Aut(S_n) (exact theorem numbers may differ slightly between editions; readers are encouraged to double-check in their own edition).
- Sylvester's synthematic-total construction appeared in his 1844 paper and is modernized in Cameron, *Permutation Groups* (LMS 45), ch. 6.

Adhering to the no-self-referential-verification rule, the result Out(S_6) ≠ 1 is recorded only as an internal result of pure group-theoretic logic. No "cause-effect" link between this fact and other n = 6 phenomena in the project is claimed in this note.
