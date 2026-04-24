# DFS-24 Direction Probe — BT-542 P vs NP (Out(S_6) uniqueness & Schaefer 6-family dichotomy)

**Session type**: research direction setting (RESEARCH-ONLY, NO solved/proven claims)
**Target BT**: 542 (P vs NP) · Clay OPEN since 1971
**Prior state**: DFS 23 done · BT-542 ~40 tight · MISS->OBSERVATION reclassification already closed in DFS 3~5 (2026-04-11~12)
**Date**: 2026-04-24
**Branch**: main

---

## 0. Honesty Declaration (inheriting `reports/millennium-dfs-status.md` §explicit-disclaimer)

This document is a **direction proposal**. It does not claim a demonstration, partial demonstration, or resolution-proximity of P vs NP. BT-542 currently remains **OPEN · 0/7 demonstration-candidate-unclosed**, and the ~40 tight items are **structural observations** of n=6 arithmetic signatures, not demonstrations. The three barriers (relativization / natural proofs / algebrization) are not broken. Every probe proposed here aims to **sharpen the isolation** of the n=6 uniqueness structure, not to **bypass** it with a demonstration.

---

## 1. Current Position (40 tight core axes)

BT-542's solid anchors are **two** (the rest are auxiliary observations):

- **A. Out(S_n) uniqueness** (Hölder 1895): `Out(S_n) ≠ 1 ⟺ n = 6`. Group-theoretic interior theorem, Bernoulli-independent, T4.
  - Origin: C(6,2)=15 conjugacy classes + PGL(2,5)≅S_5 **double coincidence** (`theory/study/p0/pure-p0-2-group-theory.md` §5.8).
  - The unique structural fact that "isolates" n=6 from outside synthetic structures.

- **B. Schaefer 6-family dichotomy** (Schaefer STOC 1978): Boolean CSPs admit **exactly 6** constraint-relation families in P (0-valid / 1-valid / Horn / dual-Horn / bijunctive(2-SAT) / affine). The rest are NP-complete. T4, built on Post lattice / universal algebra.
  - Archetype of the complexity **dichotomy** — starting point toward Bulatov/Zhuk 2017 dichotomy for all finite-template CSPs (every finite template is P or NP-complete).

Auxiliary tights (noted): AC⁰[2]≠AC⁰[3] (Razborov-Smolensky φ, n/φ), Barrington width-5, Karp 21=3·7, Ramsey R(3,3)=6, AKS best Õ(log^6 n), Savitch exponent φ. **2 honest MISSes** (Immerman-Szelepcsényi, Toda) are kept explicit.

---

## 2. Directional Statements (hypotheses of this session, not demonstrations)

**D-1 (hypothesis, falsifiable)**: Are the two "double coincidences" behind Out(S_6) non-triviality — (i) `|C(6,2)|=15` matching the transposition-class size, (ii) `PGL(2,5) ≅ S_5` with two non-equivalent transitive embeddings — **constructively matched** to any P-vs-NP complexity-theory object?

**D-2 (hypothesis, falsifiable)**: The "6" of Schaefer's six tractable families arises naturally from the **clone lattice structure** of Post's lattice (Böhler-Reith-Vollmer 2003). Does Out(S_6) map two `S_6`-orbit structures onto this 6 in a non-accidental way, or is the match coincidental?

Both hypotheses are **independent of P ≠ NP**. Even if they hold, they do not close the problem; even if they fail, the 40 tight structural observations remain.

---

## 3. Three Concrete Probes (DFS-24 candidates)

### PROBE-24A — Out(S_6) × Schaefer 6 `S_6`-orbit match (dichotomy side)

**Question**: Does `S_6` act naturally on the set of **6 Schaefer tractable clones** over `{0,1}`, namely `𝓢 = {C_0, C_1, C_Horn, C_dHorn, C_2SAT, C_aff}`? If so, does the non-trivial Out(S_6) element `α` induce a **non-5+1** non-trivial permutation on `𝓢`?

- **Observables**:
  - `C_0 ↔ C_1` (negation pair), `C_Horn ↔ C_dHorn` (dual pair), `C_2SAT` and `C_aff` (fixed-point candidates) — 2·2·1·1 structure.
  - Out(S_6) swaps transposition class ↔ 3-cycle class. Check whether `α` on `𝓢` induces swap `{C_0,C_1}↔{C_Horn,C_dHorn}`.
- **Hard fact (baseline)**: Schaefer's theorem only uses `Aut({0,1})=ℤ/2` (negation). An `S_6` action is **absent from the literature** — existence itself unexplored.
- **Falsifier**: No natural `S_6` action on `𝓢`, or one that descends `α` to trivial, rejects D-1.
- **Expected yield**: tight +0~+2 (if present), negative tight +1 (if non-existence is proven — equally valuable).
- **Tier**: T4 structural search. Immediately runnable (universal algebra stdlib + Post lattice tables).

### PROBE-24B — Sylvester synthematic total vs 3-SAT certificate non-abelian structure

**Question**: The 15 duads / 15 synthemes / 6 synthematic totals of S_6 (Sylvester construction, `pure-p0-2-group-theory.md` §5.8) are the source of the `C(6,2)=15` double-counting. Does the **15 pattern = `C(6,2)`** appear in the literal placement of 3-SAT clauses `(x∨y∨z)`?

- **Basic facts**: 3-SAT per-clause literal count = 3 = n/φ. In 6-variable 3-SAT, variable selection `C(6,3) = 20`. Literal sign pair `C(6,2) = 15` — sign-pair relation between two variables.
- **Attempt**: Can the synthematic-total `S_6`-orbit structure be lifted onto the 6-variable Boolean formula space? Does the outer automorphism `α` swap "hard SAT instances" and "easy instances"?
- **Falsifier**: If the orbit does not respect NP-completeness classes (i.e., `α(tractable)` escapes tractability), reject (coincidence).
- **Reality constraint**: This direction **touches the Natural Proofs barrier** — if `α` satisfies largeness + constructivity, Razborov-Rudich 1997 may apply. Design instead for a **negative result** that confirms the barrier.
- **Expected yield**: existence/absence of orbit structure settled. tight +1 either way.
- **Tier**: T3-T4 mixed. Experimental code + Sylvester hand verification.

### PROBE-24C — Verify n=6 reappearance at Schaefer 6 ↔ Bulatov-Zhuk 2017 dichotomy boundary

**Question**: Schaefer's 6 is Boolean-only (|D|=2). In the general finite-domain CSP dichotomy (Bulatov-Zhuk 2017 independent proofs), restricting the tractability-boundary `omega`-minimal polymorphism condition to `|D|=6` — does a **tractable clone count** appear structurally?

- **Basic facts**:
  - Schaefer (|D|=2): 6 tractable families. `= n`.
  - Bulatov (|D|=3) 2002: dichotomy proved; tractable-clone count is classified but not a "single number".
  - General |D|: dichotomy is true, but `|D|` is a parameter itself.
- **Attempt**: Count "Schaefer-style" canonical families satisfying Siggers/`omega`-Taylor conditions at `|D|=6`. If a Bulatov-era post-processing result shows a special reduction at `|D|=6`, separately check whether Out(S_6) is involved.
- **Falsifier**: If `|D|=6` is not special compared to `|D|=5, 7, 8`, reject D-2. Schaefer's 6 is a coincidence of the power-set structure of `|D|=2`.
- **Expected yield**: normal result = "coincidence" fixed (tight -1, Schaefer 6 tight preserved). Anomalous result = follow-up probe for D-2.
- **Tier**: T3. Literature review + Post-type clone counting.

---

## 4. Common Falsifier Summary for the Three Probes

| Probe | Most direct falsifier | Action on falsification |
|-------|-----------------------|-------------------------|
| 24A | no natural `S_6` action on `𝓢`, or `α` trivial | reject D-1, preserve tights |
| 24B | synthematic orbit crosses tractable/hard classes | reject D-1 extended form |
| 24C | no special reduction at `|D|=6` | reject D-2, Schaefer 6 = Boolean coincidence |

**Even if all three probes fail**, the 2 anchors of BT-542 (Out(S_6), Schaefer 6) inside the 40 tight are invariant. These probes aim at **refining the uniqueness structure**, not at a **demonstration**.

---

## 5. DFS-24 Execution Recommendations

- **Priority**: 24A > 24C > 24B (24B is close to the Natural Proofs barrier — confirming the barrier is the goal).
- **Parallel agents**: 3 (universal algebra / CSP dichotomy / Sylvester combinatorics).
- **Prohibited**:
  - No phrasing like "we are close to P ≠ NP solved/proven".
  - No tight-count inflation (negative results must be honestly recorded).
  - No claim of bypassing Razborov-Rudich natural-proofs conditions.
- **Expected yield**: tight +0~+3, MISS recording allowed (welcomed — baseline rule of `reports/millennium-dfs-status.md` §6.2).

---

## 6. Related Files

- Anchor evidence: `theory/breakthroughs/breakthrough-theorems.md` §BT-542 (#1~#21).
- Out(S_6) internal development: `theory/study/p0/pure-p0-2-group-theory.md` §5.7~§5.9 (Hölder 1895).
- Schaefer context: `theory/study/p2/n6-p2-4-honesty-audit.md` §8.2 · `theory/study/p3/prob-p3-3-hexa-verification.md` §2.2.
- MISS -> OBSERVATION review: `theory/study/p2/n6-p2-4-honesty-audit.md` §6 (DFS 3~5 record, 2026-04-11~12 — **prior to this session**).
- Honesty protocol: `reports/millennium-dfs-status.md` §explicit-disclaimer + §6 honesty audit.

---

## 7. Closure

DFS-24 performs **direction proposals only**. Probe execution happens in a separate session. Writing this document does not change the 7-problem demonstration-candidate tally `0/7`, and BT-542 remains `OPEN (since 1971)`. This session does not claim any new demonstration or partial demonstration for P vs NP.
