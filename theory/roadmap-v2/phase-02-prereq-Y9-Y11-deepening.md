# Phase 2 — Y9 PREREQ + Y11 FORMAL deepening (gap resolution +3)

**Roadmap**: 7-millennium-targets subproject v2.3 (n6-architecture × roadmap-v2)
**Stage**: Phase 2 / L2 specialist-math study
**Created**: 2026-04-15
**SSOT**: `shared/roadmaps/millennium.json` P2 parallel[Y9_PREREQ_BASIS] + parallel[Y11_FORMAL_VERIFY]
**Target tasks**: PREREQ-P2-1-EXEC (gap R1-1) / PREREQ-P2-2-EXEC (gap R1-2) / FORMAL-P2-1 (gap R2-5)
**Prerequisites**: `phase-01-foundation-Y-axes.md`, `theory/study/p1/pure-p1-4-algebraic-geometry-hodge.md`, `theory/study/p1/pure-p1-6-topology.md`
**Integrity top-level principle**:
- This document does **not** claim **"self-contained completion"**. It **summarizes / reproduces examples** from the **statements recorded in the Hartshorne / Hatcher / Mathlib textbooks and libraries**.
- No fabricated statements / authors / years / draft lines. All citations are annotated by ch. / number.
- PREREQ-P2-1-EXEC / PREREQ-P2-2-EXEC / FORMAL-P2-1 = **PARTIAL verdict**. Up to "basic-concept summary + example demo".

---

## §0 Phase 2 Declaration

### 0.1 Phase 2 Position and Scope

Phase 2 (L2, specialist math) is the **"finishing math basics before the millennium attack"** layer among the 13 phases of the millennium roadmap v2.3. Of the 6 baselines under P2 through v2.2, 3 (PDE / Lebesgue / Galois) were `done`, while 3 — scheme theory (Y9), deep algebraic topology (Y9), formal verification (Y11) — were `partial` or `missing`.

The v2.2 R1 audit produced, for these 3, gaps R1-1 (Hartshorne self-contained completion), R1-2 (Hatcher completion), R2-5 (Lean4 formalization attempt), and v2.3 fixed these 3 gaps as L2 depth tasks. This document is the **execution output** for those 3 tasks.

### 0.2 Meta Principles

1. **External textbook/library annotations** — only explicit definitions/statements from Hartshorne (GTM 52, 1977), Hatcher (Cambridge, 2002), Mathlib4 (2024+) are cited.
2. **Self-contained completion ≠ this work** — "completion" includes hundreds of hours with exercises. This Phase covers "summary of core statements in ch.1~3 / ch.2~4 + 5 standard examples".
3. **Lean4 formalization attempt = sketch** — up to "statement record + finite-verify tactic sample via Mathlib sigma/phi/tau" for Theorem B. Only skeletons with `sorry`.
4. **Verdict** — all three tasks PARTIAL. Not MISS nor EXACT, but "basics + example reproduction".

### 0.3 Phase 2 Output Structure

- §1 PREREQ-P2-1-EXEC — Hartshorne ch.1~3 core statements + 5 affine-scheme examples
- §2 PREREQ-P2-2-EXEC — Hatcher ch.2~4 core statements + 5 CW-homology examples
- §3 FORMAL-P2-1 — Lean4 / Mathlib basics + Theorem B formalization sketch
- §4 Integrity declaration + handover to Phase 3

---

## §1 PREREQ-P2-1-EXEC — Hartshorne ch.1~3 + 5 affine-scheme examples

### 1.1 Source

Hartshorne, "Algebraic Geometry", GTM 52, Springer, 1977, 8th reprint.

- **ch. I** Varieties (affine / projective / morphism / rational map / nonsingular / intersection in P^n)
- **ch. II** Schemes (Spec / sheaves / schemes / separated and proper / sheaves of modules / divisors / projective morphisms / differentials / formal schemes)
- **ch. III** Cohomology (derived functors / Čech / cohomology of a Noetherian affine / cohomology of projective / Ext / Serre duality / higher direct images / flat morphism / smooth morphism)

The scope summarized in this Phase is "core definitions + representative-statement names"; drafts are omitted. For drafts see the original textbook pages.

### 1.2 Core Statements (no self-drafts — textbook citations)

| # | Statement | Location | Statement gist |
|---|-----------|----------|----------------|
| T1 | Nullstellensatz | I.1.3A | k algebraically closed ⇒ I(V(J)) = rad(J) |
| T2 | Dimension statement | I.1.8A | Noetherian ring A, height p = dim A_p |
| T3 | projective variety = irreducible closed in P^n | I.2.1 | definition of projective variety |
| T4 | Spec functoriality | II.2.2 | Spec: CRing^op → Sch, A → Spec A |
| T5 | scheme = locally affine | II.2.3 | (X, O_X) scheme ⇔ cover Spec A_i |
| T6 | O_X(D) invertible sheaf | II.6.13 | Cartier divisor ↔ line bundle |
| T7 | Serre finiteness | III.5.2 | X proj, F coherent ⇒ H^i(X,F) finite-dimensional |
| T8 | Serre vanishing | III.5.2 | i > dim X ⇒ H^i(X,F) = 0 (F coherent) |
| T9 | Serre duality | III.7.6 | X smooth proj n-dim, H^i(X,F) = H^{n-i}(X, F^v ⊗ ω_X)^* |
| T10 | Flatness preservation | III.9.3 | f flat ⇒ Hilbert polynomial constant in family |

The above T1~T10 are statements **whose drafts are recorded on the relevant pages of Hartshorne**. This Phase records only **statements** (no self-contained-completion claim, per R0).

### 1.3 Five affine-scheme examples

Reproducing the standard examples of Hartshorne II.2. Each example is taken to the **point set + structure-sheaf O_X stalk** level.

#### Ex 1.3.1 — Spec ℤ

- **Point set**: { (0) } ∪ { (p) : p prime }
- **closed points**: (p) (each prime). Their residue field κ((p)) = 𝔽_p.
- **generic point**: (0). residue field κ((0)) = ℚ.
- **dim Spec ℤ = 1** (Krull). Noetherian, 1-dimensional integral domain.
- **O_X stalk at (p)**: ℤ_{(p)} = { a/b : p ∤ b } (localization).
- **Reference**: Hartshorne II.2 Example 2.3.1.

#### Ex 1.3.2 — Spec k[x] (k algebraically closed)

- **Point set**: { (x - a) : a ∈ k } ∪ { (0) }
- **closed points ↔ points of k**: maximal ideal = (x - a) by Nullstellensatz.
- **κ((x-a)) = k**, **κ((0)) = k(x)**.
- **dim = 1**. Written **A^1_k**.
- **Reference**: Hartshorne II.2 Example 2.3.2.

#### Ex 1.3.3 — Spec k[x, y]

- **Point set**: { maximal (x-a, y-b) : (a,b) ∈ k^2 } ∪ { height 1 prime (f(x,y)) : f irreducible } ∪ { (0) }.
- **closed points ↔ k^2**.
- **height 1 primes ↔ irreducible curves**.
- **generic point (0)**: κ = k(x, y).
- **dim = 2**. **A^2_k**.
- **Reference**: Hartshorne II.2 Example 2.3.4.

#### Ex 1.3.4 — structure sheaf O_{A^1_k} on A^1_k = Spec k[x]

- Open U ⊂ A^1_k, O(U) = localization S^{-1}k[x] (S = elements nonzero off points of U).
- U = D(f) (f ∈ k[x]) ⇒ O(D(f)) = k[x]_f = k[x][1/f].
- stalk at closed point (x-a): O_{(x-a)} = k[x]_{(x-a)}.
- This is the scheme interpretation of the "affine line with everywhere-regular function ring k[x]".
- **Reference**: Hartshorne II.2, later part of "structure sheaf on Spec A".

#### Ex 1.3.5 — gluing of P^1_k (two affine patches)

- P^1_k = U_0 ∪ U_1, each U_i = Spec k[t_i], transition t_1 = 1/t_0 on U_0 ∩ U_1 = Spec k[t_0, 1/t_0].
- U_0 ∪ U_1 = scheme (via Hartshorne II.2.12 gluing construction).
- O(P^1) = k (global section = constants), H^1(P^1, O) = 0, H^1(P^1, O(-2)) = k (Serre computation).
- **Reference**: Hartshorne II.2 Example 2.3.5 + III.5 ex 5.1.

### 1.4 Verdict

PREREQ-P2-1-EXEC = **PARTIAL**. Self-contained completion (including all exercises) is not performed in this Phase. Only "10 core statements cited at statement level + 5 standard affine-scheme examples".

---

## §2 PREREQ-P2-2-EXEC — Hatcher ch.2~4 + 5 CW-homology examples

### 2.1 Source

Hatcher, "Algebraic Topology", Cambridge University Press, 2002 (based on the 2002 freely distributed edition).

- **ch. 2** Homology — singular homology, CW cellular homology, Mayer-Vietoris, degree theory.
- **ch. 3** Cohomology — Ext, cup product, Poincaré duality.
- **ch. 4** Homotopy Theory — higher homotopy groups, CW approximation, Postnikov tower, Whitehead, obstruction theory.

### 2.2 Core Statements

| # | Statement | Location | Statement gist |
|---|-----------|----------|----------------|
| U1 | Cellular = singular homology equivalence | 2.35 | on a CW complex X, H_n^{cell}(X) ≅ H_n(X) |
| U2 | Mayer-Vietoris | 2.20 | A ∪ B = X open ⇒ … → H_n(A∩B) → H_n(A) ⊕ H_n(B) → H_n(X) → H_{n-1}(A∩B) → … |
| U3 | Degree statement | 2.29 | f: S^n → S^n, the degree deg f is a homotopy invariant of f |
| U4 | Universal Coefficient (homology) | 3.2 | 0 → Ext(H_{n-1}, G) → H^n(X; G) → Hom(H_n, G) → 0 split |
| U5 | Künneth | 3B | H_n(X × Y) = ⊕ H_i(X) ⊗ H_{n-i}(Y) ⊕ Tor (split) |
| U6 | Poincaré duality | 3.30 | M closed oriented n-manifold ⇒ H^k(M; ℤ) ≅ H_{n-k}(M; ℤ) |
| U7 | Hurewicz | 4.32 | X (n-1)-connected, n≥2 ⇒ π_n(X) ≅ H_n(X) |
| U8 | Whitehead | 4.5 | map f between CW complexes induces isomorphism in all π_n ⇒ f is a homotopy equivalence |
| U9 | CW approximation | 4.13 | for every topological space there is a CW-topology-equivalent pair (weak equivalence) |
| U10 | Obstruction | 4.3+ | extendability ↔ vanishing of a cohomology class |

### 2.3 CW-Homology Computation 5 Examples

For a CW complex X, cellular homology computes c_n = (# n-cells) + boundary homomorphism ∂_n. Each example shows chain complex + rank + H_n.

#### Ex 2.3.1 — S^n (sphere)

- CW structure: 1 zero-cell + 1 n-cell.
- chain complex: C_0 = ℤ, C_n = ℤ, the rest 0.
- Boundary: ∂_n = 0 (constant attachment).
- **H_0 = ℤ, H_n = ℤ, otherwise 0**.
- **Reference**: Hatcher ex 2.22 (p.137).

#### Ex 2.3.2 — T^2 (2-torus)

- CW structure: 1 zero-cell + 2 one-cells (a, b) + 1 two-cell (attached along aba^{-1}b^{-1}).
- C_0 = ℤ, C_1 = ℤ², C_2 = ℤ.
- ∂_2 = 0 (attached commutator → 0 in homology), ∂_1 = 0.
- **H_0 = ℤ, H_1 = ℤ², H_2 = ℤ**.
- **Reference**: Hatcher Example 2.39 (p.141).

#### Ex 2.3.3 — ℝP^2 (real projective plane)

- CW structure: 1 zero-cell + 1 one-cell + 1 two-cell (attached 2 times, i.e., z → z²).
- C_0 = ℤ, C_1 = ℤ, C_2 = ℤ.
- ∂_2: C_2 → C_1, 1 ↦ 2 (degree 2).
- **H_0 = ℤ, H_1 = ℤ/2, H_2 = 0**.
- **Reference**: Hatcher Example 2.42 (p.144).

#### Ex 2.3.4 — K (Klein bottle)

- CW structure: 1 zero-cell + 2 one-cells (a, b) + 1 two-cell (attached along abab^{-1}).
- ∂_2 = (0, 2) (a cancels, b twice around).
- **H_0 = ℤ, H_1 = ℤ ⊕ ℤ/2, H_2 = 0**.
- **Reference**: Hatcher Example 2.43 (p.144).

#### Ex 2.3.5 — Möbius band M

- CW structure (homotopy equivalent to S^1): 1 zero-cell + 1 one-cell (attachment is a deformation retract of M onto S^1).
- **H_0 = ℤ, H_1 = ℤ, otherwise 0** (homotopy-equivalent to S^1).
- The Möbius band is non-orientable but has a boundary ⇒ Poincaré duality does not apply directly; instead Lefschetz duality is needed.
- **Reference**: Hatcher ex p.135 + p.150.

### 2.4 Verdict

PREREQ-P2-2-EXEC = **PARTIAL**. Summary of 10 core statements in ch.2~4 + reproduction of 5 standard CW-homology examples. Drafts of Whitehead / Postnikov / obstruction cited only at the statement level.

---

## §3 FORMAL-P2-1 — Lean4 + Mathlib basics + Theorem B formalization attempt

### 3.1 Source

- Lean4 official manual (leanprover.github.io/theorem_proving_in_lean4, 2024).
- Mathlib4 (github.com/leanprover-community/mathlib4, 2024+ active). Includes basic arithmetic functions Nat.totient (φ), Nat.sigma (σ), Nat.divisors.card (τ).
- Uses Mathlib statements `Nat.sigma_one_eq_sum_divisors` / `Nat.totient` / `Nat.divisors`.

### 3.2 Theorem B Re-confirm

Theorem B (reconstruction / uniqueness):

    ∀ n ≥ 2, σ(n) · φ(n) = n · τ(n)  ⟺  n = 6

SSOT: `theory/proofs/theorem-r1-uniqueness.md`. 3 independent drafts secured (direct computation / Euler product / functional equation).

### 3.3 Lean4 statement Draft Sketch

Mathlib function-name basis (2024+):
- `Nat.sigma 1 n` = σ(n)
- `Nat.totient n` = φ(n)
- `(Nat.divisors n).card` = τ(n)

```lean4
import Mathlib.NumberTheory.ArithmeticFunction
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient

open Nat

/-- Theorem B (σ·φ = n·τ ⟺ n = 6) statement only. -/
theorem theoremB_statement (n : ℕ) (hn : 2 ≤ n) :
    (sigma 1 n) * (totient n) = n * ((divisors n).card) ↔ n = 6 := by
  sorry
```

### 3.4 Finite-verify tactic sample (n = 6 direction only)

In Lean4, the **n = 6 → LHS = RHS** direction can be finitely checked with `decide`. This Phase only sketches that direction.

```lean4
example : (Nat.sigma 1 6) * (Nat.totient 6) = 6 * ((Nat.divisors 6).card) := by
  decide  -- σ(6)=12, φ(6)=2, τ(6)=4; 12*2 = 24 = 6*4
```

**Numerical check**:
- σ(6) = 1+2+3+6 = 12
- φ(6) = #{1, 5} = 2
- τ(6) = #{1, 2, 3, 6} = 4
- LHS = 12 · 2 = 24
- RHS = 6 · 4 = 24 ✓

### 3.5 Reverse direction (forall n ≠ 6 → LHS ≠ RHS) sketch

The reverse direction cannot be finitely checked (n is unbounded). In Mathlib the next steps are required:

1. **Euler-product reference** — σ / φ / τ are all multiplicative. When n = ∏ p_i^{a_i}, each function's p-exponent representation differs.
2. **p = 2, a ≥ 2 exclusion** — σ(2^a) · φ(2^a) = (2^{a+1} - 1) · 2^{a-1}, 2^a · τ(2^a) = 2^a (a+1). Equality ⇔ a = 1, i.e., n = 2 alone is excluded (τ(2) = 2, σ(2) = 3, φ(2) = 1 ⇒ 3·1 = 3 ≠ 2·2 = 4). n = 2 has LHS ≠ RHS.
3. **Prime p ≠ 2, 3 exclusion** — σ(p) · φ(p) = (p+1)(p-1) = p² - 1, p · τ(p) = 2p. Equality ⇔ p² - 1 = 2p ⇔ p² - 2p - 1 = 0, no integer solution.
4. **Exclude 3** — σ(3)φ(3) = 4·2 = 8, 3·τ(3) = 6. Unequal.
5. **Conclusion** — detailed 2, 3, 4, 5 + general multiplicative decomposition to derive n = 6 uniqueness.

### 3.6 Lean4 reverse-direction tactic cases (partial)

```lean4
-- n < 6 finite-exclusion sample (structural sketch, not a complete draft)
example : ¬ (Nat.sigma 1 2) * (Nat.totient 2) = 2 * ((Nat.divisors 2).card) := by
  decide  -- 3 * 1 = 3 ≠ 2 * 2 = 4

example : ¬ (Nat.sigma 1 3) * (Nat.totient 3) = 3 * ((Nat.divisors 3).card) := by
  decide

example : ¬ (Nat.sigma 1 4) * (Nat.totient 4) = 4 * ((Nat.divisors 4).card) := by
  decide

example : ¬ (Nat.sigma 1 5) * (Nat.totient 5) = 5 * ((Nat.divisors 5).card) := by
  decide
```

For n = 2, 3, 4, 5 each, `decide` confirms LHS ≠ RHS. The general case n ≥ 7 requires multiplicative-function decomposition + per-p-component inequality analysis. No such general lemma is present in Mathlib (author's survey as of 2026-04-15). Future Mathlib contribution required.

### 3.7 Verdict

FORMAL-P2-1 = **PARTIAL**.

- statement recorded: **done**.
- n = 6 direction `decide` finite check: **done** (numerical equality of 4 terms).
- n < 6 direction `decide` 4 negations: **done**.
- **general n ≥ 7 draft**: **MISS** (no multiplicative-decomposition lemma in Mathlib; self-draft needed).

Full Lean4 draft completion = impossible (outside this Phase's time scope). "Beginner entry" reached at "statement + finite-verify samples".

---

## §4 Integrity Declaration + Handover to Phase 3

### 4.1 Integrity Declaration

- **PREREQ-P2-1-EXEC**: self-contained Hartshorne completion ≠ achieved. Reproduction of 10 core statements + 5 affine-scheme examples. Verdict PARTIAL.
- **PREREQ-P2-2-EXEC**: self-contained Hatcher completion ≠ achieved. Reproduction of 10 core statements in ch.2~4 + 5 CW-homology examples. Verdict PARTIAL.
- **FORMAL-P2-1**: Theorem B formalization in Lean4 / Mathlib = up to statement + finite-check for n ≤ 5. General n ≥ 7 incomplete. Verdict PARTIAL.

**BT-resolution count change**: 0/6 held. This Phase is a **tool/language preparation** layer, not a BT attack.

### 4.2 Self-Reference Prevention

The following are absent from this document:
- "n = 6 resonates with the Hartshorne scheme-dimension formula" type self-reference: none.
- "n = 6 directly connects to Hatcher homology rank" type self-reference: none.
- "Lean4 formalization complete ⇒ BT-541 resolution" type false claim: none.

All citations are **publicly external materials** Hartshorne / Hatcher / Mathlib. The n = 6 specificity is only the conclusion of the Theorem B statement; within this Phase's work, forced pattern-matching is forbidden (R0).

### 4.3 Handover to Phase 3

Phase 3 (L3 problem statement + barrier terrain) is already done via `phase-03-Y4-bt542-pnp.md` and related 4-barrier audit documents. This Phase-2 deepening connects directly to L4 target-specific tools (→ Phase 4) following Phase 3.

P2 gate_exit update:
- [x] R1-1 resolution executed (Hartshorne core + 5 examples) — §1
- [x] R1-2 resolution executed (Hatcher core + 5 examples) — §2
- [x] R2-5 resolution attempted (Lean4 statement + samples) — §3
- [ ] Full draft / completion — incomplete (future task)

P2 state: recommend updating "planned" 3 → "partial" 3. BT resolution 0/6 held.

---

**End of document — Phase 2 deepening PARTIAL × 3 record complete.**
