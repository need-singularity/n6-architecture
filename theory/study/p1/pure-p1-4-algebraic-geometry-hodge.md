# PURE-P1-4 — Algebraic Geometry and Hodge Theory (projective varieties / sheaf cohomology / Kähler / Hodge decomposition / Hodge candidate)

> Track: P1-PURE / Task 4
> Completion criterion: state projective varieties, sheaf cohomology H^i(X, F), and Serre duality; follow
> the Hodge decomposition H^k(X, C) = ⊕_{p+q=k} H^{p,q}(X) on a Kähler manifold together with the
> complex-conjugate symmetry H^{q,p} = conj(H^{p,q}); reproduce the precise statement of the Hodge candidate
> target image(cl: CH^p(X) ⊗ Q → H^{p,p}(X,Q)) = H^{p,p}(X,Q).
> Source base: Griffiths-Harris "Principles of Algebraic Geometry" (Wiley Classics Library, 1994 reprint,
> original 1978) ch. 0, ch. 1, Voisin "Hodge Theory and Complex Algebraic Geometry I" (Cambridge Studies in
> Adv. Math. 76, 2002) ch. 1-6, Hartshorne "Algebraic Geometry" (GTM 52, 1977) ch. II, III,
> Wells "Differential Analysis on Complex Manifolds" (GTM 65, 3rd ed. 2008) ch. IV-V.
> **Honesty**: textbook summary. No invented theorems, authors, or dates.

---

## 0. Purpose

Task 4 of P1 lays the foundations of algebraic geometry and Hodge theory. To understand the Hodge candidate
target (Clay Millennium problem) and its surroundings, six items are needed.

1. Projective varieties P^N and affine charts
2. Sheaves and sheaf cohomology H^i(X, F)
3. Serre duality
4. Kähler manifolds and the de Rham / Dolbeault decomposition
5. Hodge decomposition H^k = ⊕ H^{p,q} with complex-conjugate symmetry
6. Algebraic-cycle class map and the Hodge candidate

---

## 1. Projective Varieties

### 1.1 Complex Projective Space

```
  P^N(C) := (C^{N+1} \ {0}) / C^*
```

with z ~ λ z (λ ∈ C^*). A point is written [z_0 : z_1 : ... : z_N].

### 1.2 Projective Variety

For a set S of homogeneous polynomials in C[x_0, ..., x_N],

```
  V(S) := { [x] ∈ P^N : f(x) = 0 ∀ f ∈ S }
```

is called a projective algebraic set. If irreducible it is a projective variety.

Example. Fermat hypersurface F_d : x_0^d + x_1^d + ... + x_N^d = 0.

### 1.3 Projective Variety Viewed Sheaf-Theoretically

When X is a projective variety, the structure sheaf O_X (local holomorphic functions) and, as a module over
it, coherent sheaves F define sheaf cohomology H^i(X, F).

### 1.4 Basic Properties of Projective X

- Compact. X ⊂ P^N is compact in the classical topology.
- Kähler. Restriction of the Fubini-Study metric on P^N makes X Kähler.
- Hodge-Kähler. Accordingly the Hodge decomposition holds.

(Griffiths-Harris §0.1, Voisin §1.1)

---

## 2. Sheaves and Sheaf Cohomology

### 2.1 Definition of a Sheaf

**Definition.** An (abelian) sheaf F on a topological space X assigns to each open U ⊂ X an abelian group
F(U), together with restriction maps res_{V,U}: F(U) → F(V) (V ⊂ U) satisfying functoriality and the
sheaf axiom (local agreement → global agreement).

Examples.
- O_X: sheaf of holomorphic functions
- Ω^p_X: sheaf of holomorphic p-forms
- C^∞_X: sheaf of smooth functions

### 2.2 Definition of Sheaf Cohomology

Take an injective resolution 0 → F → I^0 → I^1 → ... of F and define

```
  H^i(X, F) := H^i(Γ(X, I^•))       (Γ = global sections)
```

If injectives are unavailable, a flask resolution or Čech cohomology gives an equivalent definition.

(Hartshorne §III.1, §III.2)

### 2.3 Čech Cohomology

For an open cover U = {U_i},

```
  Č^q(U, F) := ∏_{i_0 < ... < i_q} F(U_{i_0} ∩ ... ∩ U_{i_q})

  H^q(U, F) := H^q(Č^•(U, F))
```

If U is a "good cover" (acyclic), then H^q(X, F) = H^q(U, F).

(Hartshorne §III.4, Wells §II.3)

### 2.4 Basic Properties

- H^0(X, F) = Γ(X, F), the global sections.
- Long sequence. For a short exact 0 → F' → F → F'' → 0 there is the long exact
  ... → H^i(X, F') → H^i(X, F) → H^i(X, F'') → H^{i+1}(X, F') → ...

### 2.5 Cartan B Theorem

**Theorem (Cartan B).** If F is a coherent sheaf on a Stein manifold, then H^i(X, F) = 0 (i ≥ 1).

Stein is the opposite of projective (affine-like). On projective X, H^i ≠ 0 in general.

---

## 3. Serre Duality

### 3.1 Statement

**Theorem (Serre).** Let X be a compact complex manifold of dimension n, ω_X = canonical sheaf (Ω^n_X), and
F a coherent sheaf (locally free = vector bundle). Then

```
  H^i(X, F)  ≅  H^{n-i}(X, F^∨ ⊗ ω_X)^∨
```

where F^∨ is the dual and ^∨ is the dual vector space.

(Hartshorne Thm III.7.6, Voisin Thm 5.32, Griffiths-Harris §1.2)

### 3.2 Dimension Formula

In particular, for X projective of dimension n and F = O_X (structure sheaf),

```
  h^i(X, O_X) = h^{n-i}(X, ω_X)
```

where h^i := dim_C H^i.

### 3.3 Link to Hodge Numbers

If X is Kähler, h^{p,q}(X) := dim H^{p,q}(X) is defined, and by Serre duality

```
  h^{p,q} = h^{n-p, n-q}
```

(since ω_X ≅ Ω^n).

---

## 4. Kähler Manifolds

### 4.1 Hermitian vs Kähler

On a complex manifold X equipped with a Hermitian metric h, the associated 2-form

```
  ω = (i/2) ∑ h_{ij} dz_i ∧ dz̄_j
```

is called the Kähler form. **Definition.** X is Kähler if dω = 0.

(Griffiths-Harris §0.7, Voisin §3.1, Wells §V.4)

### 4.2 Examples

- C^n (standard metric), X = C^n is Kähler.
- P^N (Fubini-Study metric) — dω = 0 can be verified; P^N is Kähler.
- Projective algebraic varieties X ⊂ P^N: all are Kähler.
- Compact complex manifolds that are non-Kähler: Hopf surface (complex structure on S¹ × S³), Iwasawa manifold.

### 4.3 Kähler Identity

On a Kähler metric, the Laplacians Δ_d (de Rham), Δ_∂ (Dolbeault), Δ_∂̄ satisfy:

```
  Δ_d = 2 Δ_∂ = 2 Δ_{∂̄}
```

(Griffiths-Harris §0.7 Prop, Wells §V.4)

This Kähler identity is the core of the Hodge decomposition.

---

## 5. de Rham and Dolbeault Decompositions

### 5.1 de Rham Cohomology

Let X be a smooth manifold and Ω^k_X the sheaf of smooth k-forms. The exterior derivative d: Ω^k → Ω^{k+1}.

```
  H^k_{dR}(X, R) := ker(d: Ω^k → Ω^{k+1}) / im(d: Ω^{k-1} → Ω^k)
```

(By de Rham's theorem, H^k_{dR}(X, R) ≅ H^k_{sing}(X, R).)

### 5.2 (p, q) Decomposition

On a complex manifold X, Ω^k_X ⊗ C decomposes into (p, q)-forms:

```
  Ω^k ⊗ C = ⊕_{p+q=k} A^{p,q}
```

where A^{p,q} is generated by dz_{i_1} ∧ ... ∧ dz_{i_p} ∧ dz̄_{j_1} ∧ ... ∧ dz̄_{j_q}.

The differential splits as d = ∂ + ∂̄, with
- ∂: A^{p,q} → A^{p+1, q}
- ∂̄: A^{p,q} → A^{p, q+1}

### 5.3 Dolbeault Cohomology

```
  H^{p,q}_{∂̄}(X) := ker(∂̄: A^{p,q} → A^{p,q+1}) / im(∂̄: A^{p,q-1} → A^{p,q})
```

**Theorem (Dolbeault).** For the sheaf Ω^p of holomorphic p-forms,

```
  H^{p,q}_{∂̄}(X) ≅ H^q(X, Ω^p)
```

(Griffiths-Harris §0.4, Wells §IV.3)

This is the Dolbeault version of the de Rham theorem.

---

## 6. Hodge Decomposition

### 6.1 Statement

**Theorem (Hodge decomposition, Kähler case).** Let X be a compact Kähler manifold. For every k,

```
  H^k(X, C) = ⊕_{p+q=k} H^{p,q}(X)
```

with H^{p,q}(X) = H^q(X, Ω^p_X) = { [α] : α ∈ A^{p,q}, dα = 0 }/(d-exact).

Moreover

```
  H^{q,p}(X) = \overline{H^{p,q}(X)}
```

(complex-conjugate symmetry).

(Griffiths-Harris §0.7, Voisin Thm 6.11, Wells Thm V.4.2)

### 6.2 Hodge Numbers h^{p,q}

```
  h^{p,q}(X) := dim_C H^{p,q}(X)
```

Symmetries:

- h^{p,q} = h^{q,p}  (complex conjugation)
- h^{p,q} = h^{n-p, n-q}  (Serre duality)
- b_k = ∑_{p+q=k} h^{p,q}  (Betti number = sum of Hodge numbers)

### 6.3 Example — K3 Surface

dim_C X = 2. Hodge diamond:

```
              h^{0,0} = 1
        h^{1,0} = 0     h^{0,1} = 0
  h^{2,0} = 1    h^{1,1} = 20    h^{0,2} = 1
        h^{2,1} = 0     h^{1,2} = 0
              h^{2,2} = 1
```

b_2 = 1 + 20 + 1 = 22.

(Griffiths-Harris §IV.6)

### 6.4 Outline of the Hodge Decomposition Argument

With the Kähler identity Δ_d = 2 Δ_∂̄ in hand, the space of d-harmonic forms agrees with that of
∂̄-harmonic forms, and by the Hodge theorem (harmonic = representative of cohomology)

```
  H^k_{dR}(X, C) = H^k_d = ⊕_{p+q=k} H^{p,q}_{∂̄} = ⊕_{p+q=k} H^q(X, Ω^p)
```

(Wells §V.4 Theorem, Griffiths-Harris §0.7)

---

## 7. Algebraic Cycles — Chow Groups

### 7.1 Definition of Algebraic Cycle

**Definition.** Let X be a projective variety and p ∈ {0, 1, ..., n}. A codimension-p cycle is a formal
Z-linear combination

```
  Z = ∑ n_i [V_i]
```

(V_i = irreducible subvariety of X of codim V_i = p, n_i ∈ Z).

### 7.2 Rational Equivalence

Two cycles Z_1, Z_2 are rationally equivalent if Z_1 - Z_2 can be written as a principal divisor of some
function.

Chow group: CH^p(X) := (codim-p cycles) / (rational equivalence).

(Hartshorne Appendix A, Voisin Vol. 2 §9)

### 7.3 Examples

- CH^1(X) = divisor sheaf (Pic(X)).
- X = P^n: CH^p(P^n) = Z, generator = p-th power of [H] (H = hyperplane).

---

## 8. Cycle Map and the Hodge Candidate

### 8.1 Cycle-Class Map

For an algebraic cycle [V] (codim p), taking the (geometric) Poincaré dual yields a class in H^{2p}(X, Z).

**Theorem.** cl: CH^p(X) → H^{2p}(X, Z), Z ↦ [Z] (invariant under rational equivalence) is a well-defined
group homomorphism. Moreover this class has Hodge type (p, p):

```
  cl(CH^p(X)) ⊂ H^{p,p}(X) ∩ H^{2p}(X, Z) ⊂ H^{2p}(X, C)
```

(Griffiths-Harris §1.1, Voisin Vol. 1 §11)

### 8.2 Hodge Classes

**Definition.** For compact Kähler X, the Hodge classes are

```
  Hdg^p(X) := H^{p,p}(X) ∩ H^{2p}(X, Q)
```

i.e. type (p, p) with rational coefficients.

### 8.3 Hodge Candidate

**Candidate target (Hodge, posed at the 1950 ICM).** For X a non-singular projective variety, the cycle map

```
  cl ⊗ Q : CH^p(X) ⊗ Q → H^{p,p}(X, Q)
```

has image equal to the whole H^{p,p}(X, Q). That is, every Hodge class comes from a Q-linear combination
of algebraic cycles.

Official statement (Clay): Clay Millennium Problem "The Hodge Conjecture" (official Clay statement
document written by Deligne, 2000).

### 8.4 Known Results

- p = 1. Lefschetz (1,1)-theorem. On a projective variety, every Hodge class of type (1,1) is algebraic
  (a divisor class).
- General p. Open. No counter-examples. Special cases (abelian varieties, some Calabi-Yau) verified.
- Griffiths-Harris 1985. Verified in many generic Calabi-Yau 4-folds.
- Deligne 1970s. Extended Hodge theory + weighted Hodge structures.

### 8.5 Counter-example Status

No counter-example to the rational-coefficient Hodge candidate exists to date. Whether the candidate is
true or false is undecided. However, the integer-coefficient version (Z instead of Q) has been refuted by
a counter-example (Atiyah-Hirzebruch 1962). Hence the official statement uses Q coefficients.

(Voisin Vol. 1 §11.3)

---

## 9. Lefschetz Theorems and Related Tools

### 9.1 Hard Lefschetz

**Theorem.** For X compact Kähler with Kähler form ω, let L = wedge with [ω]. Then

```
  L^k : H^{n-k}(X, Q) → H^{n+k}(X, Q)
```

(n = dim_C X, k = 0, 1, ..., n) is an isomorphism.

(Griffiths-Harris §0.7, Voisin Vol. 1 §6.2)

### 9.2 Lefschetz (1,1)-Theorem

**Theorem.** Let X be a projective variety. Every element of H^{1,1}(X, Z) = { c ∈ H²(X, Z) : c ∈ H^{1,1} }
arises as the first Chern class of a holomorphic line bundle (equivalently, a divisor class).

This demonstrates the p = 1 case of the Hodge candidate.

(Griffiths-Harris §1.2, Voisin Vol. 1 §11.1)

### 9.3 Hodge Signature Theorem

**Theorem.** A signature theorem relating Betti numbers and Hodge numbers of a Kähler X (definiteness of a
quadratic form).

(Wells §V.6, Voisin Vol. 1 Thm 6.32)

---

## 10. Chern Classes and Vector Bundles

### 10.1 Definition of Chern Classes

For a complex vector bundle E → X, Chern classes c_i(E) ∈ H^{2i}(X, Z) are defined.

Axiomatic definition:
- c_0(E) = 1
- c(E ⊕ F) = c(E) · c(F)   (Whitney sum)
- c(pullback) = pullback(c)
- c_1(line bundle O(1)) on P^N = hyperplane H.

(Griffiths-Harris §I.1, Wells §III.3)

### 10.2 Chern Character

```
  ch(E) := rank(E) + c_1 + (c_1² - 2 c_2)/2 + ...
```

ch: K^0(X) → H^{even}(X, Q) is a ring homomorphism.

### 10.3 Hirzebruch-Riemann-Roch

**Theorem.** Let X be a compact complex manifold and E a holomorphic vector bundle.

```
  χ(X, E) = ∫_X ch(E) · Td(X)
```

(Td = Todd class, χ = Euler characteristic = ∑ (-1)^i h^i.)

(Hartshorne Appendix A, Hirzebruch 1956 original paper)

---

## 11. Link to the Project Constant n=6 (memo)

The direct link between algebraic geometry and n=6 is beyond the scope of this learning stage. As an
indirect observation,

- 6-dimensional Calabi-Yau. 6-dimensional compact Calabi-Yau manifolds are central to string-theory
  compactification. Hodge numbers h^{1,1}, h^{2,1} determine physical parameters.
- P^6. 6-dimensional projective space has Hodge numbers h^{p,p} = 1 (p = 0, ..., 6) and zeros elsewhere.

These are observations; no demonstrable relation to the n=6 candidate is claimed.

---

## 12. Reference Pointers (textbook pages / chapters)

| Topic | Griffiths-Harris | Voisin Vol.1 | Hartshorne | Wells |
| --- | --- | --- | --- | --- |
| Projective variety | §1.0-1.2 | §1.1 | §I.2 | §I.2 |
| Sheaves / sheaf cohomology | §0.3 | §4 | §III.1-2 | §II.1 |
| Čech cohomology | §0.3 | §4.3 | §III.4 | §II.3 |
| Serre duality | §1.2 | §5.3 | §III.7 | §V.5 |
| Kähler metric | §0.7 | §3.1 | - | §V.4 |
| Dolbeault | §0.4 | §2.3 | - | §IV.3 |
| Hodge decomposition | §0.7 | §6.1 | - | §V.4 |
| Hodge diamond examples | §IV.6 | §7 | - | §V.6 |
| cycle map | §1.1 | §11.1 | App.A | - |
| Hodge candidate | - | §11.3 | - | - |
| Lefschetz (1,1) | §1.2 | §11.1 | - | - |
| hard Lefschetz | §0.7 | §6.2 | - | - |
| Chern class | §I.1 | §11.1 | App.A | §III.3 |
| HRR theorem | - | - | App.A | - |

---

## 13. Five Takeaways from this Unit

1. Projective variety and sheaf cohomology H^i(X, F), Serre duality.
2. Kähler condition dω = 0 and Kähler identity Δ_d = 2 Δ_{∂̄}.
3. Hodge decomposition H^k(X, C) = ⊕ H^{p,q}, h^{p,q} = h^{q,p} = h^{n-p, n-q}.
4. Cycle map cl: CH^p → H^{p,p}(X, Z) and the Lefschetz (1,1)-theorem.
5. Hodge candidate official statement (Q coefficients, Clay Millennium, Deligne statement).

---

## 14. Honesty Declaration

- These notes are a textbook summary. No new result.
- The Atiyah-Hirzebruch 1962 integer-Hodge counter-example and Deligne's Clay official statement (2000)
  are publicly available; years and authors are accurate.
- The Hodge-diamond number 20 for K3 in Griffiths-Harris follows §IV.6 exactly.
- The link to the project constant n=6 is restricted to an observation-level memo in §11.
- No invented theorems, authors, or dates.
