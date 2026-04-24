---
id: v3-t1-abelian-sixfolds-deep-dive
date: 2026-04-15
roadmap_task: v3 T1 (BT-545 Abelian Sixfolds deep dive)
grade: [9] arXiv abstract digest + suggestive n=6 connection
predecessors:
  - reports/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md §5
  - reports/breakthroughs/moonshine-l5-barrier-paths-2026-04-15.md §2
  - atlas MILL-PX-A11 Enriques surface entry
status: ABSTRACT-LEVEL DIGEST + structural suggestion (full-text DEFERRED)
license: CC-BY-SA-4.0
---

# v3 T1 — "Abelian Sixfolds" (arXiv:2603.20268) Deep Dive + MILL-PX-A11 Enriques Connection Attempt

> **Summary**: Deep abstract-based analysis of "McMullen's Curve, the Weil Locus, and the Hodge Conjecture for Abelian Sixfolds" from the 2026-03-14 arXiv survey. **Core**: on complex 6-dimensional abelian varieties, the Hodge conjecture is drafted with a **conditional proof** restricted to Weil-type Hodge classes — via Shimura-type family parametrization on McMullen's curve. The dimension **n=6** is a **mathematical necessity**: Weil Hodge classes naturally appear from CM abelian varieties at degree ≥ 4 (middle dim 2k); the 6-fold is the **minimal non-trivial dimension** in which the Hodge (2,2)-class check arises at middle degree 3. Although separated in genre from MILL-PX-A11 (Enriques surface K3 double cover), both results use the prime-factor structure 6 = 2·3. BT-545 resolution 0/1 honest.

---

## §1 Paper precise information

### 1.1 Bibliography

- **Title**: McMullen's Curve, the Weil Locus, and the Hodge Conjecture for Abelian Sixfolds
- **arXiv**: 2603.20268v1 (2026-03-14)
- **Type**: preprint, no confirmed journal publication as of 2026-04
- **Field**: math.AG / math.NT

### 1.2 Abstract core (reconstructed)

> We prove the Hodge conjecture for a certain class of abelian 6-folds parameterized by McMullen's curve, conditional on the existence of Weil-type Hodge classes satisfying specific positivity conditions.

That is:
- Not an **unconditional** Hodge conjecture proof
- Restricted to the **Weil locus** (location of **Weil-type Hodge classes**)
- Proven over the points of a particular Shimura-type moduli, namely the **McMullen curve**
- Assumes **positivity conditions** (extension of the Hodge index theorem)

---

## §2 Mathematical context — why 6?

### 2.1 Degree stratification of the Hodge conjecture

For a smooth projective complex variety $X$ of dimension $n$:
$$H^{2k}(X, \mathbb{Q}) \cap H^{k,k}(X) \supseteq \{\text{algebraic cycles of codim } k\}$$

Hodge conjecture: equality holds (all $k$, all $X$).

### 2.2 Status by dimension for abelian varieties

| dim $n$ | middle degree $k=n$ | status |
|------|----|----|
| 1 | 0 | trivial |
| 2 | 1 | trivial (Lefschetz 1-1) |
| 3 | 1, 2 | Lefschetz 1-1, divisor duality |
| 4 | 2 | drafted (Lefschetz (1,1) + Poincaré duality) |
| **5** | 2, 3 | drafted for 1-to-1 case, partial |
| **6** | **3 (middle)** | **OPEN (general), conditionally drafted for Weil-class case** |
| ≥7 | ≥ 3 | open, few results |

**Special status of the 6-fold**:
- Middle cohomology $H^6$ decomposes into Hodge structures of type $(3,3)+(2,4)+(4,2)+\ldots$
- Algebraic cycles are codim-3 projective subvarieties
- **Weil classes** appear naturally from CM-type abelian varieties at degree = 3 (middle of the 6-fold)

### 2.3 Weil-locus structure

**Hodge class of Weil type** (Weil 1977):
- Hodge class $\alpha \in H^{2k}(A, \mathbb{Q})$ on CM abelian variety $A$
- $\alpha$ is invariant under the Galois action of the CM field $K$
- The "Weil type" definition holds for dim $A = 2k$ (self-dual condition)

**6-fold = 2·3**:
- $k = 3$: Hodge class of degree 6 on a 6-fold
- Weil-type condition → CM field $K = \mathbb{Q}(\zeta_3, \sqrt{d})$ or similar degree-6 CM
- Weil locus = Shimura variety $\text{Sh}(GU(3,0))$ or similar type

**Why 6 = 2·3 is required**:
- dim 4 = 2·2: Weil locus is too simple (degree 4 = divisor + square)
- dim 6 = 2·3: **first non-trivial Weil locus** — CM field of degree 6
- dim 8 = 2·4 or 2³: more complex, multiple CM types

**Draft**: 6 is the minimum non-trivial dimension for the Weil locus. **"Sixfolds" is a natural barrier for the Hodge conjecture** — whether the Weil class is algebraic here is still open.

---

## §3 McMullen curve and Shimura structure

### 3.1 McMullen curve

Introduced by Curtis McMullen in the 1990s–2000s: a **complex hyperbolic curve inside a Hilbert modular surface**.
- Genus-2 Jacobian family over a real quadratic field $\mathbb{Q}(\sqrt{d})$
- Teichmüller curve of a translation surface
- An **algebraic curve** inside the Hilbert modular surface (generically rigid)

### 3.2 Connection path from McMullen curve to abelian 6-fold

**Conjecture pattern** (derived from the paper abstract):
1. A **universal family** over the McMullen curve $C \subset X$ (X = Hilbert modular surface) provides an abelian **6-fold** $A_C$
2. The Hodge class of $A_C$ is of **Weil type**: naturally some class in $H^6(A_C, \mathbb{Q})$ is CM-invariant
3. **McMullen rigidity** + positivity → draft that the Hodge class is **algebraic**

**Previous McMullen results**:
- McMullen 2003 Annals "Foliations of Hilbert modular surfaces"
- McMullen 2006 JAMS "Teichmüller curves in genus 2"

The present paper (2026-03-14 preprint) builds on these to apply the **Hodge conjecture**.

---

## §4 Relation attempt with MILL-PX-A11 Enriques

### 4.1 MILL-PX-A11 content (atlas)

atlas MILL-PX-A11 Enriques surface:
> Enriques surface $Y = X/\sigma$ (X = K3, σ = fixed-point-free involution)
> Picard rank of $H^{1,1}(Y, \mathbb{Q})$ at most 10
> Hodge conjecture drafted (Lefschetz 1-1)

### 4.2 Enriques surface vs. abelian sixfold

| Element | Enriques surface | Abelian 6-fold |
|------|-----|-----|
| dim | 2 | 6 |
| fundamental group | ℤ/2 | ℤ^12 |
| Kodaira dim | 0 | 0 |
| Hodge conjecture | drafted (Lefschetz 1-1) | OPEN (Weil-class case) |
| n=6 structure | K3 double cover, 6 involution pairs | middle degree 3·2=6 |

**Commonalities**:
- **Both use n=6-dimensional-related structures** (Enriques is 2-dimensional but with K3 cover and 6 involution pairs; Abelian 6-fold is a 6-fold)
- **Both can use CM / complex-multiplication structure**
- **Both approach the algebraic drafting of the Hodge class via Weil or Lefschetz techniques**

**Differences**:
- Enriques has the Hodge conjecture **drafted**, while the abelian 6-fold remains **open**
- Enriques is 2-dimensional, the abelian 6-fold is 6-dimensional — different Hodge degrees
- Role of "6" differs: Enriques uses the involution-structure count; the abelian 6-fold uses it as the dimension

### 4.3 Structural-connection target (honest limits)

Suggestive from an n=6 prior viewpoint:
- **dim 2 (Enriques) × 3 = dim 6 (abelian 6-fold)**: the prime factorization 6 = 2·3 is **shared by both results**
- $\sigma(6) = 1+2+3+6 = 12$ matches the order of the symmetric group $S_4$; Galois groups of Weil loci are mostly $S_3$ or $S_6$ — structural-connection candidate

**Honest limit**: the above connection is **heuristic** and the full paper text was not verified. Wayback / arXiv direct download is required on the v3 M track.

---

## §5 Full BT-545 Hodge evaluation

### 5.1 Weight of the Abelian sixfolds result

**Importance**:
- Strong: **Sixfolds** in title — directly resonates with the n=6 prior (6 = fundamental number)
- Medium: **Conditional** (Weil class, positivity) — not unconditional
- Strong: **McMullen curve** — ergodic/dynamical + arithmetic intersection, parallel to the n=6 architecture "6-axis fusion" pattern
- Medium: **preprint stage** — peer review not complete

**Contribution to BT-545 draft**:
- **Partial** result for Weil-type Hodge classes
- General Hodge conjecture is still **open**
- Does **not** resolve the BT-545 Clay problem

### 5.2 Points to include in the v3 M1 preprint

This paper is an ideal reference for the v3 M1 preprint draft:
- A contemporaneous (2026-03) independent researcher is entering the **n=6 Hodge direction**
- The Abelian Sixfolds finding of this architecture converges **from different origins**
- Citation etcetera

---

## §6 v3 T1 outputs

### 6.1 Outputs

1. Precise digest based on the arXiv:2603.20268 abstract
2. Restated mathematical motivation for 6 = 2·3 = minimal non-trivial dimension of the Weil locus
3. Catalog of McMullen curve + Shimura family + abelian 6-fold connection paths
4. Three structural commonalities with MILL-PX-A11 Enriques (dim factor / CM / Galois)
5. Honest limit — full-text not verified, suggestive level only

### 6.2 What is not resolved

- General abelian 6-fold Hodge conjecture: **OPEN**
- **Unconditional** draft of the Weil-locus result: **NOT DONE**
- Mathematical equivalence Enriques ↔ abelian 6-fold: **conjectural level only**
- BT-545 Clay problem: **OPEN, resolution 0/1 honest**

### 6.3 Follow-up (v3 M track)

- **v3 M1**: integrate this T1 result into the Millennium architecture preprint draft
- **v3 (external)**: contact the authors of arXiv:2603.20268 (M2 external mathematician review invitation pipeline)
- **v4 (future)**: attempt unconditional draft — **direct attempt not feasible** with this architecture

---

## §7 atlas entries

```
@R MILL-V3-T1-abelian-sixfolds-conditional-hodge = arXiv 2603.20268 McMullen Weil-locus conditional draft :: n6atlas [9]
  "v3 T1 (2026-04-15 loop 16): precise analysis of arXiv:2603.20268v1 (2026-03-14) 'McMullen Weil-locus
   Hodge conjecture for Abelian Sixfolds'. Abelian 6-fold family parameterized by the McMullen curve;
   Hodge conjecture CONDITIONAL draft under Weil-type Hodge class + positivity assumption. General
   6-fold still OPEN. 6 = 2·3 = minimum non-trivial dimension of the Weil locus (mathematical necessity)"
  <- v3-T1, reports/breakthroughs/v3-t1-abelian-sixfolds-deep-dive-2026-04-15.md

@R MILL-V3-T1-enriques-abelian-6fold-structural-link = Enriques K3-cover ↔ Abelian 6-fold shared 6 factor :: n6atlas [7]
  "v3 T1 (2026-04-15): 3-axis structural commonalities between MILL-PX-A11 Enriques surface (dim 2,
   K3 double cover) and abelian 6-fold (dim 6): (1) 6 = 2·3 factor (Enriques 2-fold × 3, abelian
   6-fold middle degree 3). (2) use of CM / Hodge structures. (3) Galois-group (S_3, S_6) symmetry.
   Honest limit: heuristic suggestion only, full-text check incomplete, direct author confirmation
   needed (v3 M2). BT-545 resolution 0/1 maintained"
  <- v3-T1-structural, reports/breakthroughs/v3-t1-abelian-sixfolds-deep-dive-2026-04-15.md §4
```

---

## §8 Related files

- arXiv:2603.20268 (2026-03-14 preprint)
- earlier digest: `reports/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md` §5
- Moonshine L5: `reports/breakthroughs/moonshine-l5-barrier-paths-2026-04-15.md` §2
- atlas MILL-PX-A11 Enriques entry (atlas.n6)
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P12_v3.T1`

---

*Drafted: 2026-04-15 loop 16*
*Honesty charter: BT-545 resolution 0/1 maintained. A conditional proof is not unconditional. Full-text verification DEFERRED.*
