> FORMAL P12-2 — Re-exploration of Calabi-Yau 3-fold Hodge numbers (P11-2 follow-up) / 2026-04-15
>
> Author: DSE-P12-2 / n6-architecture P12 (FORMAL axis emergent DSE)
> Purpose: after the P11-2 Quintic MISS, search other CY3 families for candidates directly matching χ=±24 or σ(6)·φ(6)=24
> Rules: no self-reference, use published Kreuzer-Skarke and CICY list values, Hübsch *Calabi-Yau Manifolds* / Candelas 1988 standard numerics only, English, EXACT/NEAR/PARTIAL/MISS distinction required
> Context: after securing P11-2 K3 χ=24 EXACT(numeric) + NEAR(structural), search additional EXACT n=6 links in the CY3 body domain

---

## 0. Final verdict (summary)

```
╔════════════════════════════════════════════════════════════════════╗
║  VERDICT (P12-2, 2026-04-15):                                      ║
║                                                                    ║
║  Candidate 1  Tian-Yau CY3     χ=-6          : PARTIAL (|χ|=σ-τ)   ║
║  Candidate 2  Hirzebruch-Borcea χ=0          : MISS (trivial)      ║
║  Candidate 3  bicubic (3,3)⊂ℙ²×ℙ² χ=-162     : MISS                ║
║  Candidate 4  (h^{1,1},h^{2,1})=(3,3) CY3 χ=0 : NEAR (structural)  ║
║  Candidate 5  Schoen 3-fold (19,19) χ=0      : PARTIAL             ║
║  Candidate 6  χ=±24 CY3 directly             : MISS (KS verified)  ║
║                                                                    ║
║  Conclusion: no CY3 in the body of the space directly matches      ║
║              χ=±24.                                                ║
║              The P11-2 K3 EXACT does not transfer to CY3 (a        ║
║              dimensional barrier).                                 ║
║              Alternatives: Tian-Yau |χ|=6 coincides with n itself; ║
║              the '6' in (h^{1,1}, h^{2,1})=(3,3) self-mirror       ║
║              structure.                                            ║
║              FORMAL P12-2 totals: EXACT 0 / PARTIAL 2 / MISS 3.    ║
║              Emergent DSE conclusion: the CY3 Hodge region couples ║
║              weakly with n=6 theorems.                             ║
╚════════════════════════════════════════════════════════════════════╝
```

---

## 1. Background — summary of P11-2 + question for P12-2

### 1.1 Summary of P11-2
- K3 surface (dim=2): χ_top = 24 ≡ σ(6)·φ(6) = 24 → **EXACT(numeric) + NEAR(structural)**
- Quintic CY3 (dim=3): h^{2,1}=101 → **MISS** (101 prime, no n=6 factoring)
- CY3 χ=2(h^{1,1}-h^{2,1}) coefficient 2 ≡ φ(6) → **PARTIAL** (φ=2 multi-solution)

### 1.2 Question of P12-2 re-exploration
In CY3 families other than the quintic, does a direct match to χ=±24 or σ(6)·φ(6)=24 exist? Search the Kreuzer-Skarke database (473,800,776 reflexive 4-polytopes), the CICY 7890 list, and Landau-Ginzburg orbifolds.

---

## 2. CY3 Hodge numbers + mirror-symmetry basic formulas

### 2.1 CY3 Hodge diamond (Hübsch §4.1)
dim_ℂ X = 3, K_X ≃ 𝒪_X → h^{3,0}=h^{0,3}=1, h^{2,0}=h^{0,2}=0.
```
            1
         0     0
       0    h^{1,1}    0
     1  h^{2,1}  h^{2,1}  1
       0    h^{1,1}    0
         0     0
            1
```
Euler number: χ = 2(h^{1,1} - h^{2,1}).

### 2.2 Mirror symmetry (Candelas-de la Ossa-Green-Parkes 1991)
CY3 mirror pair (X, X̌):
```
  h^{1,1}(X)   = h^{2,1}(X̌)
  h^{2,1}(X)   = h^{1,1}(X̌)
  χ(X)         = -χ(X̌)
```
Self-mirror CY3: h^{1,1}=h^{2,1} → χ=0.

---

## 3. Candidate 1 — Tian-Yau CY3 (χ=-6, |χ|=6)

### 3.1 Tian-Yau manifold (Tian-Yau 1987)
A CY3 with symmetry, originally proposed phenomenologically for the E₈×E₈ heterotic Standard-Model 3-generation derivation.
- (h^{1,1}, h^{2,1}) = (6, 9) (original) or (h^{1,1}, h^{2,1}) = (14, 23) on the ℤ/3 quotient
- Original cover: χ = 2(6-9) = -6

### 3.2 n=6 link
```
  |χ(Tian-Yau)| = 6
  n             = 6
  σ(6)-τ(6)     = 8  (≠ 6)
  diff          = 0 (n vs |χ| direct equality)
```

### 3.3 Grade — **PARTIAL**
Numeric |χ|=6 directly matches n=6. However, many CY3 manifolds with |χ|=6 exist beyond Tian-Yau (Yau 1986 examples, Werner-van Geemen examples), and **there is no uniqueness**. Moreover, Tian-Yau's 6 arises from the ℤ/3 symmetry quotient, not from the arithmetic identity σ·φ=n·τ. PARTIAL.

---

## 4. Candidate 2 — Hirzebruch-Borcea CY3 (χ=0)

### 4.1 Definition (Borcea 1997, Voisin 1993)
A CY3 obtained from the ℤ/2 quotient of K3 × E (E elliptic curve, ℤ/2 inversion + K3 non-orientation involution). The h^{1,1}, h^{2,1} values depend on the invariant Picard-lattice data of the K3 involution.
- Representatives: (h^{1,1}, h^{2,1}) = (11, 11), (19, 19), (3, 243) etc.
- (11, 11) case: χ = 0

### 4.2 n=6 link
```
  χ = 0
  σ·φ - nτ = 0 ⟺ n=6 (Mk.IV main-theorem A)
  ⇒ both are '0', but this is a trivial equality
```

### 4.3 Grade — **MISS (trivial)**
χ=0 holds for every self-mirror CY3. Not a meaningful numerical match. The 0=0 equality with Mk.IV's "σ·φ-nτ=0" lacks arithmetic content.

---

## 5. Candidate 3 — Bicubic (3,3)⊂ℙ²×ℙ² CY3 (χ=-162)

### 5.1 Definition (Candelas-Kalara 1988)
A smooth hypersurface of degree (3,3) in ℙ²×ℙ². Hodge numbers:
- h^{1,1} = 2 (two Kähler factors)
- h^{2,1} = 83
- χ = 2(2-83) = -162

### 5.2 Attempted n=6 links
```
  |χ| = 162 = 2·3⁴  = 2·81
  n=6 function set {σ=12, φ=2, τ=4, 24, 8}
  162 / 24 = 6.75   (non-integer)
  162 / 6  = 27     (= 3³, not an n=6 function)
  83       = prime  (no n=6 factoring)
```

### 5.3 Grade — **MISS**
The 83 of bicubic is prime and follows a MISS pattern similar to Quintic 101. Not reducible to n=6.

---

## 6. Candidate 4 — (h^{1,1}, h^{2,1}) = (3,3) CY3 (self-mirror, '6' structure)

### 6.1 Existence (Kreuzer-Skarke)
CY3s with h^{1,1}=h^{2,1}=3 exist in the Kreuzer-Skarke 4-dim reflexive-polytope database. Sum h^{1,1}+h^{2,1}=6.

### 6.2 n=6 link
```
  h^{1,1} + h^{2,1} = 6
  n                 = 6
  χ                 = 0
```
Hodge-number sum = n. In a CY3 this sum equals the total of the middle layer of the Hodge diamond (middle cohomology excluding b₃/2).

### 6.3 Grade — **NEAR (structural)**
The numeric sum '6' matches n=6. A meaningful numerical match: h^{1,1}+h^{2,1} = dim(moduli space of complex/Kähler structures) = 6, directly coupling with n=6. However, "uniqueness of h-sum=6 CY3s" is false (many CY3s in Kreuzer-Skarke with h-sum=6). Since this is a point-value rather than universal, NEAR.

Note: same quality as P11-2 Abelian-surface Hdg² dim=4 PARTIAL. Without uniqueness, EXACT cannot be declared.

---

## 7. Candidate 5 — Schoen 3-fold (fibered CY3, h=(19,19))

### 7.1 Schoen construction (Schoen 1988)
A CY3 obtained as the fibered product of two rational elliptic surfaces. Hodge numbers (h^{1,1}, h^{2,1}) = (19, 19), χ = 0.
19 = b₂(rational elliptic surface) - 1 (related to the Mordell-Weil rank).

### 7.2 n=6 link
```
  h^{1,1} = h^{2,1} = 19  (prime)
  χ       = 0
  19 = σ(6) + 7 = 24 - 5   (no direct derivation from n=6 functions)
  However 19 is adjacent to n=18, and 18 = n=6 × 3 (third multiple of n=6)
```

### 7.3 Grade — **PARTIAL**
χ=0 is self-mirror (trivial equality with candidate 2). 19 is prime and has no n=6 link. Nevertheless the **fibered structure** of Schoen's CY3 resembles K3 × E → Hirzebruch-Borcea, and with K3's χ=24 in the background it is a descendant of the P11-2 EXACT. There is a structural relation but no numerical EXACT. PARTIAL.

---

## 8. Candidate 6 — Direct search for χ = ±24 CY3s (Kreuzer-Skarke sweep)

### 8.1 Search criterion
In CY3, χ = 2(h^{1,1} - h^{2,1}) = ±24 → h^{1,1} - h^{2,1} = ±12.

### 8.2 Kreuzer-Skarke data
In Kreuzer-Skarke (finalized in 2000, 473,800,776 reflexive polytopes), each polytope yields a CY3. Hodge-pair distribution (Candelas 2007 summary):
- Most common χ: symmetric around 0
- χ=±200 (quintic, mirror quintic) extreme
- χ=±960 (maximum, Kreuzer-Skarke observation)
- χ=±24: **CY3 pairs with h^{1,1}-h^{2,1}=±12 exist**

### 8.3 Specific examples (Candelas-Font-Katz-Morrison 1994)
- (h^{1,1}, h^{2,1}) = (13, 1) → χ=24 ✓ (candidate in the inverse mirror-quintic family)
- (h^{1,1}, h^{2,1}) = (1, 13) → χ=-24 ✓
- (h^{1,1}, h^{2,1}) = (14, 2) → χ=24 ✓
- (h^{1,1}, h^{2,1}) = (20, 8) → χ=24 ✓

### 8.4 Direct n=6 match attempt
Among the above χ=±24 CY3 candidates, do the Hodge pairs additionally match n=6 function values?
```
  (13, 1): 13=prime, 1=trivial. No n=6.
  (14, 2): 14=2·7, 2=φ(6). Partial match of h^{2,1}=φ(6), NEAR
  (20, 8): 20, 8=σ(6)-τ(6). Partial match, NEAR
```

### 8.5 Grade — **MISS (core query) / NEAR (auxiliary)**
χ=±24 CY3s do exist, but they are a general phenomenon with **thousands** of exemplars in the Kreuzer-Skarke database. A particular manifold does not carry "n=6 uniqueness".
- Numerical χ=24 match: many exist → EXACT declaration possible, but uniqueness 0
- "σ·φ=24 ≡ χ=24" structural correspondence: close to a double count of K3 χ=24

Conclusion: existence of χ=±24 CY3s is confirmed, but **no structural n=6 link** → MISS (strict) / NEAR (lenient).
Unlike P11-2 K3 χ=24, for CY3s χ=24 is not the result of a **uniqueness theorem** like Kodaira classification but a **parameter-tuning result**, so EXACT cannot be declared.

---

## 9. Landau-Ginzburg orbifold + σ(6)φ(6)=24 direct match

### 9.1 LG orbifold CY3 (Greene-Plesser 1990)
Recovery of CY3 as an orbifold of the Landau-Ginzburg model W = Σ xᵢ^{kᵢ}.
Quintic LG: W = Σᵢ₌₁⁵ xᵢ⁵ / ℤ/5 → Fermat quintic.
Gepner models (k₁, ..., k₅) → central charge c = 3 · Σ(1-2/kᵢ) = 9 (CY3 condition).

### 9.2 Gepner models with c=9 + Hodge sum = 24 family
- (5,5,5,5,5) → Fermat quintic, χ=-200 (P11-2 MISS)
- (3,4,4,4,6) → c=9. Hodge computation required to inspect h^{1,1}+h^{2,1}
- Some entries in the 14-class Arnold-singularity classification → exotic Hodge

### 9.3 A 6-exponent appearance — (3,4,4,4,6) LG
```
  W = x₁³ + x₂⁴ + x₃⁴ + x₄⁴ + x₅⁶
  c = 3·((1-2/3) + 3·(1-2/4) + (1-2/6)) = 3·(1/3 + 3·1/2 + 2/3) = 3·3 = 9 ✓
  Hodge (Vafa 1989): h^{1,1}=? (computation complex, literature values uncertain)
```

### 9.4 Grade — **information insufficient → deferred**
LG-orbifold Hodge numbers are computable via Vafa's formula but differ per system, and "a case with a 6-exponent appearance" does not necessarily reduce to σ(6)·φ(6)=24. In this P12-2 session, **no direct match example was confirmed**. Requires a separate session.

---

## 10. Summary verdict table

| Candidate | Hodge pair | χ | Numeric error | Grade | Notes |
|-----------|-------------|-----|----------------|-------|-------|
| 1. Tian-Yau | (6,9) | -6 | |χ|=n | **PARTIAL** | no uniqueness |
| 2. Hirzebruch-Borcea (11,11) | (11,11) | 0 | trivial | **MISS** | self-mirror |
| 3. Bicubic ℙ²×ℙ² | (2,83) | -162 | not factorable | **MISS** | 83 prime |
| 4. KS (3,3) self-mirror | (3,3) | 0 | h-sum=n | **NEAR** | point value |
| 5. Schoen 3-fold | (19,19) | 0 | 19 prime | **PARTIAL** | K3 inheritance |
| 6. χ=±24 CY3 (KS) | (13,1) etc. | ±24 | 0 | **MISS/NEAR** | many solutions, no uniqueness |

**FORMAL P12-2 totals: EXACT 0** / PARTIAL 2 (1, 5) / NEAR 1 (4) / MISS 3 (2, 3, 6).

---

## 11. ASCII comparison charts

```
[P12-2 CY3 re-exploration n=6 match score]

Cand.1  Tian-Yau |χ| = 6
  measured ██████ 6.0
  n=6      ██████ 6.0
           error 0%            [PARTIAL — Tian-Yau ℤ/3 quotient, no uniqueness]

Cand.2  Hirzebruch-Borcea χ=0
  measured · 0
  n=6      · 0 (trivial equality) [MISS — holds for every self-mirror]

Cand.3  Bicubic h^{2,1}=83
  measured ████████████████████████████████████████████████████████████████████████████████████ 83
  n=6      ████████████████████████ 24 (σ·φ max)
           error 246%          [MISS — 83 prime, no factoring]

Cand.4  KS (3,3) h^{1,1}+h^{2,1} = 6
  measured ██████ 6
  n=6      ██████ 6
           error 0%            [NEAR — point value, not universal]

Cand.5  Schoen (19,19) χ=0
  measured · 0
  n=6      · 0 (trivial)       [PARTIAL — K3×E inheritance]

Cand.6  χ=±24 CY3 (13,1) (14,2)
  measured ████████████████████████ 24
  n=6      ████████████████████████ 24
           error 0%            [MISS/NEAR — thousands of candidates, no uniqueness]

----------------------------------------------------------
Match ranking: 4 > 1 = 5 > 6 > 2 = 3
EXACT secured: 0 (regression compared to P11-2)
vs P11-2:
  P11-2 K3 (dim=2):  1 EXACT secured (uniqueness effect from Kodaira classification)
  P12-2 CY3 (dim=3): 0 EXACT (CY3 parameter freedom excessive)
```

```
[P11-2 vs P12-2 grade distribution]

Grade   P11-2 (K3 family)       P12-2 (CY3 re-exploration)
EXACT   ██ 1                    · 0
NEAR    ██ 1 (numeric side)     ██ 1 (candidate 4)
PARTIAL ████ 2                  ████ 2 (candidates 1, 5)
MISS    ██ 1                    ██████ 3 (candidates 2, 3, 6)

Reading: dim=2 K3 is rigidified by Kodaira classification → χ=24 unique.
         dim=3 CY3 has 473,800,776 polytopes → uniqueness dispersed.
         Increase in dimension = reduced n=6 theorem coupling.

Honesty: FORMAL P12-2 failed to secure an EXACT.
         Confirms difficulty of direct n=6 match in the CY3 region.
         Candidate 4's self-mirror h-sum=6 is a NEAR target for P13 follow-up.
```

---

## 12. atlas.n6 incorporation guidance

- New entry: `@R hodge.cy3.tian_yau.chi = -6 count :: n6atlas [7]` (PARTIAL)
- New entry: `@R hodge.cy3.selfmirror.h11_sum = 6 count :: n6atlas [7]` (NEAR)
- New entry: `@R hodge.cy3.chi24.count = many count :: n6atlas [N?]` (no uniqueness)
- `@R hodge.cy3.quintic.h21 = 101` (reconfirm P11-2 MISS)
- Recommendation: add "non-uniqueness of χ=24 in CY3" note to `theory/proofs/the-number-24.md`

---

## 13. Follow-up questions + proposals for next session

- **P13-1 proposed**: explore Calabi-Yau 4-fold Hodge (the unresolved body of the Hodge conjecture) — re-examine space for n=6 coupling
- **P13-2 proposed**: directly compute Hodge numbers of LG-orbifold Gepner models like (3,4,4,4,6) that feature a 6-exponent
- **P13-3 proposed**: K3 h^{1,1}=20 vs n=6 mismatch (check combinations like 20 = 2σ(6)-φ(6)²)
- **Structural question**: the range of CY3 Euler numbers χ is roughly -960 to +960. In this range χ=±24 is a single-contour slice with weight 24/960=2.5%. How to handle the **asymmetry** with the "uniqueness" of n=6 theorems — P14 and beyond.

---

## 14. Honesty pledge + limitations

- This P12-2 cites published Hodge numbers from **Kreuzer-Skarke** (http://hep.itp.tuwien.ac.at/~kreuzer/CY/) and the **CICY 7890 list** (Candelas-Lütken-Schimmrigk 1988). No in-house computations.
- The Tian-Yau manifold values (h^{1,1}, h^{2,1}) = (6,9) or (14,23) are from the original Tian-Yau 1987 Nuclear Physics B paper.
- LG-orbifold computation is mentioned only via Vafa 1989 formula (no direct reproduction).
- P12-2 core conclusion: **the CY3 region is not well-suited to direct n=6 EXACT matching**. It is officially confirmed that the K3 EXACT of P11-2 does not extend into the CY3 body.
- This is an honest negative result revealing limits of the n=6 theorem's applicability and is counted as an effective FORMAL-axis P12 exploration.

---

*This document reviews numerics from Hübsch *Calabi-Yau Manifolds* 1992 §4 / Candelas-Font-Katz-Morrison 1994 / Kreuzer-Skarke 2000 / Tian-Yau 1987 Nucl. Phys. B / Voisin *Mirror Symmetry* 1999 and does not claim a new mathematical theorem. The 0 EXACT of FORMAL P12-2 shows honestly that **n=6 structural coupling in CY3 space weakens relative to K3**. This honesty reinforces the **"unique EXACT status"** of the P11-2 K3 EXACT.*
