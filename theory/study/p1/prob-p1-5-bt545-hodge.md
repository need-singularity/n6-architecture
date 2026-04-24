# PROB-P1-5 — BT-545 Hodge Conjecture Advanced (algebraic cycles / (p,p)-forms / Lefschetz theorems)

> Track: P1-PROB / Task 5
> Completion criterion: decompose the Clay official statement (Deligne 2000) sentence by
> sentence and explain Hodge decomposition theorem, (1,1)-Lefschetz theorem, and the current
> structure of positive results and counterexamples to the Hodge conjecture.
> Primary sources: Deligne "The Hodge Conjecture" Clay Millennium official document (2000),
> Griffiths-Harris "Principles of Algebraic Geometry" (Wiley, 1978) ch. 0, 1, 3,
> Voisin "Hodge Theory and Complex Algebraic Geometry" vol. I, II (Cambridge, 2002, 2003),
> Lewis "A Survey of the Hodge Conjecture" (CRM Monograph Ser. 10, 2nd ed. 1999),
> Atiyah-Hirzebruch "Analytic cycles on complex manifolds" (Topology 1, 1962),
> Voisin "A counterexample to the Hodge conjecture extended to Kähler varieties"
> (IMRN 2002).
> **Honesty**: this note is a reconstruction of the Clay official document + standard Hodge
> theory textbooks. There are no new theorems. All statements are reorganized from the six
> references above, and the current state of the Clay problem (some positive results,
> counterexample in the Kähler case) is marked [partial result] / [counterexample] / [unfinished].

---

## 0. Purpose and Scope

Clay BT-545 asks: "If X is a smooth complex projective variety, is every Hodge class
expressible as a rational linear combination of algebraic subvarieties of X?"

Seven topics covered by this note:

1. Hodge decomposition of cohomology H*(X; ℂ) of a complex projective variety X
2. Hodge classes Hdg^k(X) = H^{2k}(X; ℚ) ∩ H^{k,k}(X)
3. Algebraic cycles and the cycle class map
4. (1,1)-Lefschetz theorem (Lefschetz 1924 = Hodge conjecture for k=1)
5. Cases solved so far: abelian varieties, K3, ...
6. Voisin counterexample (2002) to the Kähler extension of the Hodge conjecture
7. Atiyah-Hirzebruch counterexample (integer-coefficient Hodge counterexample, 1962)

---

## 1. Complex Projective Varieties and de Rham Cohomology

### 1.1 Complex projective space ℙ^n_ℂ

ℙ^n_ℂ = (ℂ^{n+1} \ {0}) / ℂ*. Smooth closed complex manifold of dimension n. A complex
projective variety X is a closed complex submanifold of ℙ^n.

### 1.2 de Rham and ∂̄-Dolbeault

If X is a complex manifold, smooth p-forms Ω^p decompose into (p,q)-forms: Ω^p = ⊕_{a+b=p}
Ω^{a,b}. Differential d = ∂ + ∂̄.

Dolbeault cohomology:

```
  H^{p,q}_{∂̄}(X) = ker(∂̄: Ω^{p,q} → Ω^{p,q+1}) / im(∂̄: Ω^{p,q-1} → Ω^{p,q})
```

### 1.3 Hodge decomposition (smooth compact Kähler X)

If X is a smooth compact Kähler manifold, then

```
  H^k(X; ℂ) = ⊕_{p+q=k} H^{p,q}(X),     H^{q,p} = \overline{H^{p,q}}
```

This decomposition is independent of the Kähler metric and depends only on the complex
structure of X.

A smooth complex projective variety is automatically Kähler, so the Hodge decomposition holds
(Fubini-Study metric).

### 1.4 Hodge numbers h^{p,q}

h^{p,q} = dim H^{p,q}(X). Representatives:
- h^{0,0} = 1 (if connected)
- h^{1,0} = q(X) (irregularity)
- h^{2,0} = p_g(X) (geometric genus)
- h^{n,n} = 1 (if X is n-dimensional)
- Hodge diamond: h^{p,q} = h^{n-p, n-q} (Serre duality)

---

## 2. Hodge Classes and Algebraic Cycles

### 2.1 Hodge class definition

```
  Hdg^k(X) = H^{2k}(X; ℚ) ∩ H^{k,k}(X)
```

i.e., ℚ-rational cohomology classes of pure type (k,k). The de Rham representative is a
(k,k)-form with rational coefficients.

### 2.2 Algebraic cycles

**Algebraic k-cycle**: ℤ-linear combination Z = Σ n_i Z_i of codimension-k subvarieties of X,
where each Z_i ⊂ X is an irreducible codimension-k subvariety.

### 2.3 Cycle class map

```
  cl: Z^k(X) → H^{2k}(X; ℤ)
```

Z ↦ Poincaré dual (fundamental class). Extended more finely with ℚ coefficients:

```
  cl_ℚ: Z^k(X) ⊗ ℚ → Hdg^k(X) ⊂ H^{2k}(X; ℚ)
```

The image im(cl_ℚ) ⊆ Hdg^k always holds, but whether equality holds is the central question.

### 2.4 Statement of the Hodge conjecture

**Clay official statement (Deligne 2000, reorganized):**

If X is a smooth complex projective variety, for every α ∈ Hdg^k(X) there exists an algebraic
cycle Z ∈ Z^k(X) ⊗ ℚ such that cl_ℚ(Z) = α.

That is, cl_ℚ: Z^k(X) ⊗ ℚ → Hdg^k(X) is **surjective**.

---

## 3. (1,1)-Lefschetz Theorem

### 3.1 Theorem (Lefschetz 1924)

The Hodge conjecture is fully true in the k=1 case. Every α ∈ Hdg^1(X) = H²(X; ℚ) ∩ H^{1,1}(X)
is expressible as a rational linear combination of divisors.

### 3.2 Argument outline

From the exponential sheaf exact sequence

```
  0 → ℤ → 𝒪_X → 𝒪_X^* → 0
```

the long exact sequence

```
  H^1(X; 𝒪_X^*) → H²(X; ℤ) → H²(X; 𝒪_X)
```

gives H¹(X; 𝒪_X^*) = Pic(X), H²(X; 𝒪_X) ≅ H^{0,2}(X). Hence if α ∈ H²(X;ℤ) maps to 0 in
H^{0,2}, then α ∈ im Pic(X). Elements of Pic(X) can be realized as line bundle → divisor.
Also c_1(L) ∈ H^{1,1} holds.

By the Hodge decomposition, elements of H²(X;ℚ) ∩ H^{1,1} map to zero in both H^{0,2} and
H^{2,0}. ∴ algebraic.

(Source: Griffiths-Harris §1.1, Voisin vol.I §7.2)

### 3.3 Difficulty in the k≥2 case

For k≥2, the (k,k) type extends to (k,k)-wedge structures of order ≥ k!, so not all Hodge
classes can be expressed simply from products of L^k. The isomorphism supplied by the Hard
Lefschetz theorem does not give "surjectivity from the full set of algebraic cycles".

---

## 4. Cases Handled So Far

### 4.1 Dimensionally trivial cases

- X a curve (dim 1): k=1, handled by Lefschetz
- X a surface (dim 2): only k=1 is non-trivial, handled. k=0, k=2 are trivial
- X a threefold (dim 3): k=1 and k=2 are non-trivial. k=2 connects to k=1 via Poincaré duality
  → handled

That is, for **dim X ≤ 3** the Hodge conjecture is fully handled.

### 4.2 Abelian varieties — partially handled

Abelian variety A = ℂ^g / Λ. Using Mumford-Tate group techniques:
- g ≤ 3: fully handled
- g=4: Murty 1984 handled
- General g: [partial result], Deligne's motivic technique

(Source: Deligne "Hodge cycles on abelian varieties" LNM 900, 1982)

### 4.3 Complete intersections

X = V(F_1,...,F_r) ⊂ ℙ^n generic complete intersection. k=1 trivial; k≥2 handled under
specific conditions.

### 4.4 Fano varieties

When dim X = 3 and X is Fano (i.e., -K_X ample), the Hodge conjecture is suggested to hold
by Clemens-Griffiths 1972. Absorbed into the general 3-dimensional theory.

### 4.5 K3 surfaces

dim 2, so trivially handled. Higher-dimensional hyperkähler generalizations are [partial
result].

### 4.6 Representative unsolved cases

- General hypersurfaces X ⊂ ℙ^n with dim X ≥ 4 and k ≥ 2
- Calabi-Yau 4-folds
- General abelian varieties of high dimension with extra endomorphisms

---

## 5. Kähler Extension — Voisin Counterexample (2002)

### 5.1 Kähler extension of the Hodge conjecture

The original Hodge conjecture applies only to projective varieties. The Kähler extension
asserts:
"If X is a compact Kähler manifold, is every element of Hdg^k(X) realized by an analytic
cycle (analytic subvariety)?"

### 5.2 Voisin counterexample (2002)

Voisin constructed a smooth 4-dimensional compact Kähler manifold X and demonstrated the
existence of an element in Hdg² that is **not realizable by any analytic cycle**. That is,
the Kähler extension is **false**.

Construction: generic Kähler deformation of Weil-type abelian 4-fold. For the detailed
construction see IMRN 2002.

This result clarifies that the original Clay conjecture (projective hypothesis) does not
extend directly to the Kähler setting.

### 5.3 Implication

The "projectivity" condition in the Hodge conjecture is essential. The existence of algebraic
cycles depends strongly on the projective structure.

---

## 6. Integer-Coefficient Hodge — Atiyah-Hirzebruch Counterexample (1962)

### 6.1 Integral Hodge conjecture

The original conjecture is with ℚ coefficients. The "ℤ-coefficient Hodge conjecture" asks
whether cl_ℤ: Z^k(X) → Hdg^k_ℤ(X) = H^{2k}(X;ℤ) ∩ (filter on H^{k,k}) is surjective.

### 6.2 Atiyah-Hirzebruch counterexample (1962)

Atiyah-Hirzebruch showed that ℤ-coefficient Hodge is **false**, presenting a counterexample
via torsion elements. Specifically, for certain X, there exist torsion elements in
H^{2k}(X; ℤ)_{tors} ⊂ Hdg^k_ℤ that cannot be represented algebraically.

Argument: uses K-theory and Steenrod operations. For the detailed construction see Topology
1:25, 1962.

### 6.3 Importance of ℚ coefficients

Since this counterexample the Hodge conjecture has been posed only with ℚ coefficients. The
Clay official statement also uses ℚ coefficients. Torsion is a completely different problem.

---

## 7. Related Theory — Motivic Perspective

### 7.1 Grothendieck motives

Category ℳ_{rat}(X) formed by algebraic cycles modulo equivalence. The standard conjectures
and the Hodge conjecture are uniformly described within this framework.

### 7.2 Standard conjectures (Grothendieck 1968)

Five standard conjectures — type A, B, C, D, and Hodge conjecture version. All open, but they
form a structure implying the Hodge conjecture. For details see Kleiman "Algebraic cycles and
the Weil conjectures" 1968.

### 7.3 Absolute Hodge classes (Deligne)

Deligne 1982 introduced an expanded notion "absolute Hodge class" from the motivic viewpoint.
Demonstrated that every Hodge class on an abelian variety is absolute Hodge (Deligne's
theorem). Not the complete Hodge conjecture, but a strong partial result.

---

## 8. n=6 Connection (memo only)

1. At the boundary dim X ≤ 3 of the trivial case, the sum 6 of algebraic-cycle dimension
   coefficients (1,2,3) appears as a numerical match, but there is no direct mathematical
   path between Hodge-conjecture techniques and the σφ=nτ argument ([N?]).
2. In 4-dim Kähler there is the Voisin counterexample. Comparing 4 = 2·2 with 6 = 2·3 one can
   observe "what differs" via the prime-factor structure, but without causal link ([N?]).
3. The double symmetry h^{p,q} = h^{q,p} = h^{n-p, n-q} of the Hodge diamond numerically
   matches the divisor-doubling σ(6)=12 of n=6, but is independent of any argument path
   ([N?]).

Self-reference-verification prohibition: the observations above are kept independent of any
BT-545 resolution strategy.

---

## 9. Practice Problems — 5 Hand Exercises

**P1.** Compute the Hodge numbers h^{p,q} of ℙ^n. Result: h^{p,p} = 1 (0 ≤ p ≤ n), all others
0. Derive directly from de Rham cohomology.

**P2.** For an elliptic curve E, verify h^{0,0} = h^{1,1} = 1, h^{1,0} = h^{0,1} = 1. Write
the Hodge diamond.

**P3.** Reconstruct the (1,1)-Lefschetz argument: exponential sheaf sequence → long exact
sequence → H^{1,1} ∩ H²(X;ℤ) ⊂ im(Pic(X) → H²(X;ℤ)).

**P4.** For the abelian surface A = E × E (E an elliptic curve) compute the Hodge decomposition
of H²(A; ℚ) and Hdg². Verify whether every element of Hdg² is an algebraic cycle.

**P5.** Survey the key components of the Voisin 2002 counterexample. Summarize the definition
of Weil-type abelian 4-fold and the structure producing an analytic non-representable element
in Hdg².

---

## 10. Reading Path

### 10.1 Week 1

- Read the Deligne Clay official document (12 pages)
- Griffiths-Harris §0 review (complex manifolds / Dolbeault)
- Voisin vol.I §1~§3 Hodge decomposition theorem

### 10.2 Week 2

- Voisin vol.I §7 Kähler manifolds / Hodge decomposition
- Voisin vol.II §11 algebraic cycles, cycle class map
- Lewis "A Survey of the Hodge Conjecture" in full

### 10.3 Week 3

- Atiyah-Hirzebruch 1962 original paper
- Deligne "Hodge cycles on abelian varieties" LNM 900, 1982
- Voisin 2002 IMRN counterexample original paper

### 10.4 Week 4

- Kleiman "Algebraic cycles and the Weil conjectures" 1968
- Jannsen "Motives, numerical equivalence..." 1992
- Recent review: Voisin "Hodge loci and absolute Hodge classes" (Compositio 2007)

---

## 11. Source Summary

- Deligne "The Hodge Conjecture" Clay 2000 — official statement
- Lefschetz "L'Analysis Situs et la Géométrie Algébrique" Gauthier-Villars 1924 —
  (1,1)-Lefschetz original paper
- Griffiths-Harris "Principles of Algebraic Geometry" Wiley 1978 — standard textbook
- Voisin "Hodge Theory and Complex Algebraic Geometry" vol. I & II, Cambridge 2002/2003
- Lewis "A Survey of the Hodge Conjecture" CRM Monograph 2nd ed. 1999
- Atiyah-Hirzebruch "Analytic cycles on complex manifolds" Topology 1:25, 1962
- Voisin "A counterexample to the Hodge conjecture extended to Kähler varieties" IMRN 2002
- Deligne "Hodge cycles on abelian varieties" LNM 900, 1982
- Kleiman "Algebraic cycles and the Weil conjectures" Dix exposés sur la cohomologie
  des schémas, 1968

This note is a P1-study-volume reorganization of the 9 primary sources above and does not
claim new results.

---

## 12. Appendix — Hodge Structure Definition

### 12.1 Pure Hodge structure

A decomposition V_ℂ = V_ℤ ⊗ ℂ = ⊕_{p+q=n} V^{p,q} with V^{q,p} = \overline{V^{p,q}} on a
ℤ-lattice V_ℤ. This is a **pure Hodge structure of weight n**.

### 12.2 Mixed Hodge structure (Deligne 1974)

Two filtrations: weight filtration W_• and Hodge filtration F^•. Deligne "Théorie de Hodge"
I, II, III. General (non-compact, singular) algebraic varieties have cohomology with a mixed
Hodge structure.

### 12.3 Period map

For families s parameter, variation of V^{p,q}_s → map into a period domain. The Griffiths
transversality condition (1968) constrains the slope of this map.

---

## 13. Appendix — Mumford-Tate Group and Hodge Loci

### 13.1 Mumford-Tate group

Algebraic group MT(V) ⊂ GL(V) associated to a Hodge structure. The "automorphism" group
preserving Hodge classes.

### 13.2 Hodge loci

In a family of varieties X → S, the set of s where a specific element of Hdg^k(X_s) is found.
Cattani-Deligne-Kaplan 1995: Hodge loci are algebraic subvarieties.

### 13.3 Implication

Algebraicity of Hodge loci → strong structural evidence that Hodge classes forming an
"algebraic family" can be realized as algebraic cycles.

---

## 14. Appendix — Current Main Approaches

### 14.1 Motivic approach

Grothendieck motives and André's absolute Hodge motive. Expresses the Hodge conjecture in
motivic language + integrates with the standard conjectures.

### 14.2 K-theory approach

The Bloch-Beilinson conjecture generalizes Hodge via K-theory and motivic cohomology. Explores
the connection between Chow groups and Hodge classes.

### 14.3 Harmonic analysis approach

L²-Hodge theory (Morrey-Kodaira), Hodge on CR structures. Refining Hodge structures with
complex-geometric tools.

---

## 15. Appendix — Main Hodge-number Computation Examples

### 15.1 ℙ^n

h^{p,p} = 1 (0 ≤ p ≤ n), others 0.

### 15.2 Elliptic curve E

h^{0,0} = h^{1,1} = 1, h^{1,0} = h^{0,1} = 1. Full Hodge diamond (rhombus-like 1 form):
```
       1
     1   1
       1
```

### 15.3 K3 surface

h^{0,0} = h^{2,2} = 1, h^{1,1} = 20, h^{2,0} = h^{0,2} = 1. χ = 1 + 20 + 1 = 24.
Hodge diamond:
```
          1
       0     0
    1    20    1
       0     0
          1
```

### 15.4 Calabi-Yau 3-fold

h^{0,0} = h^{3,3} = 1, h^{3,0} = h^{0,3} = 1, h^{2,0} = h^{0,2} = 0,
h^{1,1} = a, h^{2,1} = h^{1,2} = b. Moduli space dimension b, Kähler class a-dimensional.

### 15.5 Abelian variety A = E × E

h^{p,q}(A) = binomial(2, p)·binomial(2, q). H²(A; ℚ) is 6-dimensional, (1,1)-part h^{1,1} = 4.

---

## 16. Appendix — Reconstruction of the Lefschetz (1,1) Argument

### 16.1 Exponential sheaf sequence

0 → ℤ → 𝒪_X → 𝒪_X^* → 0 (𝒪_X^* = multiplicative group).

### 16.2 Long exact sequence

```
  H^1(X; 𝒪_X) → H^1(X; 𝒪_X^*) = Pic(X) → H²(X; ℤ) → H²(X; 𝒪_X)
```

### 16.3 Application of the Hodge decomposition

H²(X; 𝒪_X) = H^{0,2}(X). Therefore α ∈ H²(X; ℤ) maps to zero in H^{0,2} ⟺ α ∈ im(Pic).
By Hodge decomposition, α ∈ H²(X; ℚ) ∩ H^{1,1} ⟹ α (up to ℤ-coefficient multiple) is a Hdg¹
element with H^{0,2} component zero.

### 16.4 Conclusion

Every element of Hdg¹(X) is realized in Pic(X) ⟹ realized as a (ℚ-coefficient) linear
combination of divisors. ∴ the (1,1) Hodge conjecture is demonstrated.

---

## 17. Next Documents

- PROB-P1-6 : BT-546 BSD advanced
- PROB-P1-7 : BT-547 Poincaré advanced
- N6-P1-3 : n=6 honesty principle

BT-545 is deepened at the P2~P3 stage via motivic unification, the standard conjectures, and
Hodge loci analysis. The aim of this P1 note is "precise decomposition of the Clay statement
+ representative resolution / counterexample map".
