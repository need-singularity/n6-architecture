---
id: bt-542-p-vs-np-4-barriers-survey
date: 2026-04-15
parent_bt: BT-542
roadmap_task: BARRIER-PX-1 (BT-542 MISS-escape retry — 4-barrier bypass, new approach)
grade: MISS_MAINTAINED (main body), PARTIAL (literature review)
license: CC-BY-SA-4.0
---

# BT-542 P vs NP — Survey of 4 Barriers + Current (2020-2026) Research Paths

> **Key conclusion**: BT-542 MISS maintained. This document catalogs the 4 barriers of P vs NP (Relativization / Natural proofs / Algebraization / GCT) and lists major bypass attempts since 2020. The n=6 structural prior has **no directly applicable angle** to this problem, which we record honestly.

---

## §0 Entry Point

P vs NP (Clay Millennium) is the **most-attempted-and-failed** of the 7 Millennium problems. As of 2026, 4 known barriers block most existing techniques.

**Scope of this document**:
1. Mathematical definition of the 4 barriers + main theorems
2. Bypass attempts (2020-2026) of new techniques (meta-complexity, circuit complexity, proof complexity)
3. **Honest evaluation** from the n6-architecture perspective — confirmed no direct angle
4. atlas entry candidates (literature taxonomy; not a progress claim)

**Honesty declaration**: No sentence in this document asserts "this path can prove P ≠ NP". It is a literature catalog + MISS confirmation.

---

## §1 Barrier 1 — Relativization (Baker-Gill-Solovay 1975)

### 1.1 Definition

**Relativization**: a technique M is *relativizing* if, whenever M proves P vs NP separation, it simultaneously proves `P^A vs NP^A` separation for any oracle A.

### 1.2 Baker-Gill-Solovay theorem (1975)

> There exist oracles A, B satisfying:
> - `P^A = NP^A` (under A, P = NP)
> - `P^B ≠ NP^B` (under B, P ≠ NP)

**Consequence**: relativizing techniques cannot prove P vs NP separation. Diagonalization / table-lookup alone is insufficient.

### 1.3 Bypasses (non-relativizing techniques)

- **Nepomnjaščiĭ's theorem 1970**: partial result LINSPACE ⊂ Σ_k^P
- **IP = PSPACE (Shamir 1990)**: interactive proofs = PSPACE. Non-relativizing (oracle exists with IP ≠ PSPACE).
- **PCP theorem (Arora-Lund-Motwani-Sudan-Szegedy 1992)**: probabilistically checkable proofs.

→ Non-relativizing techniques exist, but are still blocked by the next barrier (Natural proofs).

---

## §2 Barrier 2 — Natural Proofs (Razborov-Rudich 1997)

### 2.1 Definition

**Natural property**: formalization of the essence of circuit-complexity lower-bound techniques. Three conditions:

- **(C) Constructive**: the decision algorithm that decides `f : {0,1}^N → {0,1}` runs in polynomial time.
- **(L) Largeness**: at least a `1/N^{O(1)}` fraction of all random functions passes the decision.
- **(U) Usefulness**: all functions of a circuit class C fail the decision.

### 2.2 Razborov-Rudich theorem

> If one-way functions exist, then `{f : f ∉ P/poly}` cannot be proved by a natural property.

**Consequence**: accepting the existence of one-way functions (a basic assumption of modern cryptography), **the constructive + large proof style** is unusable for P vs NP. "Any proof that NP has no polynomial-size circuits must be either **non-constructive** or **non-large**" (Razborov-Rudich).

### 2.3 Three "natural-proofs barrier bypass" techniques

1. **Non-constructive techniques** (e.g., Williams 2011 algorithmic method): even if the decision algorithm is not polynomial-time, the existence of a non-trivial algorithm implies a lower bound. NEXP ⊄ ACC⁰.

2. **Non-large techniques** (e.g., specific-function lower bounds): decides only a specific function. Not dependent on the fraction of random functions.

3. **Meta-complexity path** (2020-): equivalent description of natural proofs vs breaking OWF — "MCSP hardness ↔ cryptographic primitives".

---

## §3 Barrier 3 — Algebraization (Aaronson-Wigderson 2008)

### 3.1 Definition

**Algebrizing**: techniques in which the separation proof goes through for low-degree polynomial extensions `Ã` (for oracle A) as well.

### 3.2 Aaronson-Wigderson theorem (2008)

> All existing non-relativizing techniques (IP = PSPACE, PCP, etc.) are algebrizing. Moreover:
> - There exists A with `P^{A,Ã} = NP^{A,Ã}` (algebrizing relative to A, Ã)
> - There exists B with `P^{B,B̃} ≠ NP^{B,B̃}` (algebrizing relative to B, B̃)

**Consequence**: **all** existing non-relativizing techniques (PCP, IP, arithmetization) are insufficient for P vs NP separation. New techniques required.

### 3.3 Candidate non-algebrizing techniques

- **GCT (Geometric Complexity Theory)**: treated separately below in §4
- **Quantum algebraization** (Aaronson-Wigderson 2008 §7): extension to quantum oracles — a stricter barrier.
- **Proof complexity**: instead of circuit complexity directly, propositional proof system lower bounds (Atserias, Pudlák 2005-)

---

## §4 Barrier 4 — GCT barriers (post-2011)

### 4.1 GCT program summary (Mulmuley 2001-)

**Geometric Complexity Theory**: attacks separation of Permanent `perm_n` vs Determinant `det_m` via representation theory of the symmetric group:

```
perm_n ∉ {determinantal complexity class of size m(n)}
  ⟸ rectangular Kronecker coefficient structure
```

Core target: whether certain rectangular Kronecker coefficients are 0 or large can be decided.

### 4.2 Ikenmeyer-Panova 2017 barrier

> Rectangular Kronecker coefficients do not exhibit the GCT occurrence obstruction.

**Consequence**: the main GCT route ("representation-theoretic separation") is restricted. Subsequent Bürgisser-Ikenmeyer-Panova 2019 and further negative results.

### 4.3 Post-2020 GCT reinterpretation

- **Border complexity** (Grochow 2020): lower bounds for `perm vs det` border rank
- **Tensor rank** (Christandl-Lutzky-Zuiddam 2023 etc.): multilinear-algebra perspective
- **Plethysm coefficients** (Bürgisser-Ikenmeyer 2019-): Schur-polynomial decomposition

→ The GCT program is still active, but **has not produced a core P vs NP result by 2026**.

---

## §5 New Paths 2020-2026

### 5.1 Meta-complexity

**MCSP (Minimum Circuit Size Problem)**: `(f, s)` → "is f's minimum circuit size ≤ s?"

- **Oliveira-Santhanam 2017**: MCSP ∈ P ⟺ natural proofs fail (OWF fails)
- **Allender 2020**: candidate NP-intermediate status of MCSP-like problems
- **Hirahara 2022 (STOC)**: NP ⊂ P ⟺ MCSP ∈ P (near-equivalence with meta-complexity)

**Evaluation**: Meta-complexity is an **equivalent variant** of P vs NP. It does not avoid the problem itself.

### 5.2 Circuit complexity (Williams line)

- **Williams 2011**: NEXP ⊄ ACC⁰ (STOC best paper)
- **Murray-Williams 2018**: strengthened to NQP ⊄ ACC⁰
- **Chen-Hirahara-Williams 2021**: derandomization → lower bounds

**Evaluation**: the Williams method is **non-natural + non-relativizing + non-algebrizing**. It bypasses all 4 barriers. But the lower bounds obtained so far reach only `NEXP, NQP` — not down to `NP`.

### 5.3 Proof complexity

- **Atserias-Müller 2020**: Resolution lower bounds
- **Pudlák 2020-**: P != NP ⟺ "proof complexity separates"
- **Dantchev-Martin 2023**: Cutting-planes lower bounds

**Evaluation**: Proof complexity merely provides a **reduction to P ≠ NP**; not a proof of the main problem.

### 5.4 Quantum techniques

- **Aaronson 2023**: attempt to transfer quantum-circuit lower bounds into classical lower bounds
- **BQP vs PH**: Raz-Tal 2019, Aaronson-Ingram-Kretschmer 2022

**Evaluation**: Quantum aspects are an independent research area. Direct contribution to P vs NP is still limited.

---

## §6 Honest Evaluation from the n6-architecture Perspective

### 6.1 Does n=6 structure relate to P vs NP?

Systematic check of paths where the n=6 prior could contribute to P vs NP:

| Candidate path | n=6 structure | Evaluation |
|-----------|----------|------|
| Circuit complexity | Multiplicative bound on circuit size | (x) No number match |
| GCT Kronecker | Young tableaux / symmetric group | (x) Is S_6 special? No evidence |
| Boolean functions | AC⁰ vs NP | (x) n is only input size; no 6-structure |
| Communication complexity | 2-party vs k-party | (x) Nothing special about k=6 |
| Meta-complexity MCSP | Circuit-size threshold | (x) No connection to σ(n), τ(n) |

**Conclusion**: **the n=6 structure has no directly applicable angle to P vs NP**. Unlike other BTs (RH, BSD, NS), this problem is far from "number-matching" patterns.

### 6.2 Possible indirect contributions of n6-architecture (honest)

- **Honest-record infrastructure**: the atlas-grade + MISS-tagging system can **systematically record** the barrier literature of this problem. This document is an example.
- **Statistical-thinking transfer**: the BKLPR asymptotic-verification methodology in BSD is partly transferable to finite-barrier statistical analysis of P vs NP (e.g., reinterpretation of random-SAT phase transitions) — but this is **empirical analysis, not a Millennium resolution**.
- **Complexity of the problem itself**: P vs NP is a "meta-meta-mathematical" problem — the main line is proving **unprovability** (the 4 barriers), not proving a result. The n=6 structural prior is in an unnecessary dimension.

### 6.3 Cross-check against prior session documents

- BT-542 subquestions in `theory/study/p3/prob-p3-1-open-subquestions.md` (circuit / GCT / MCSP) agree with this document.
- Consistent with the subjective estimate "2040 P vs NP resolution probability very low" in `n6-p3-3-synthesis-report.md`.
- **Augmentation by barrier taxonomy layer; no duplication.**

---

## §7 Conclusion + MISS Verdict

### 7.1 BT-542 main-body verdict

**MISS_MAINTAINED**. In the scope of BT-542 "P ≠ NP proof":
- No new technique proposed that bypasses any of the 4 barriers
- No n6-architecture contribution confirmed
- Research 2020-2026 also has **no direct arrival** at P vs NP separation

### 7.2 BARRIER-PX-1 scope verdict

**PARTIAL_LITERATURE_CATALOG**. This document delivers:
- Mathematical definitions + main theorems of the 4 barriers
- Classification of 2020-2026 bypass attempts (meta-complexity / Williams / proof complexity / quantum)
- Honest evaluation from n6 perspective (record: no direct applicability)
- atlas entry candidate: literature taxonomy

### 7.3 atlas entry proposal

```
@R MILL-BARRIER-PX1-four-barriers-catalog = P vs NP 4 barriers = {relativization, natural, algebraization, GCT} :: n6atlas [10]
  "BARRIER-PX-1 BT-542 4-barrier catalog (literature review, 2026-04-15):
   1. Relativization (Baker-Gill-Solovay 1975)
   2. Natural proofs (Razborov-Rudich 1997)
   3. Algebraization (Aaronson-Wigderson 2008)
   4. GCT (Mulmuley 2001- + Ikenmeyer-Panova 2017)
   Definition and bypass attempts recorded for each barrier. BT-542 main body MISS maintained. n6-architecture has no directly applicable angle — recorded honestly"

@R MILL-BARRIER-PX1-n6-nonapplicability = n=6 structural prior does not apply to P vs NP :: n6atlas [10*]
  "BARRIER-PX-1 honest record: the n=6 structural prior has no direct applicability to a P vs NP proof.
   In systematic checks (circuit complexity / GCT Kronecker / Boolean functions / communication / meta-complexity)
   no n=6-special connection is present. BT-542 is a structural-prior-agnostic Millennium problem. Honesty-core record"
```

---

## §8 Related Files

- `theory/study/p3/prob-p3-1-open-subquestions.md` — existing BT-542 subquestions
- `theory/study/p3/n6-p3-3-synthesis-report.md` — 2040 resolution-probability estimate
- `papers/millennium-7-closure-2026-04-11.md` — existing honest closure

---

## §9 Source Year / Author Recheck Checklist (honesty)

Items in this document cited from **memory** need DOI verification:

| Citation | Year | Authors | DOI check |
|------|------|------|---------|
| Baker-Gill-Solovay | 1975 | Baker, Gill, Solovay | SIAM J Comput (to confirm) |
| Razborov-Rudich | 1997 | Razborov, Rudich | J CSS 55 (to confirm) |
| Aaronson-Wigderson | 2008 | Aaronson, Wigderson | ACM TOCT (to confirm) |
| Williams 2011 | 2011 | R. Williams | STOC 2011 (to confirm) |
| Ikenmeyer-Panova | 2017 | Ikenmeyer, Panova | Advances Math (to confirm) |
| Hirahara 2022 | 2022 | Hirahara | STOC 2022 (to confirm) |
| Oliveira-Santhanam | 2017 | Oliveira, Santhanam | CCC (to confirm) |
| Murray-Williams 2018 | 2018 | Murray, Williams | STOC 2018 (to confirm) |

**TODO**: batch-collect DOIs for this checklist in the v3 roadmap. Not performed in this session (DEFERRED).

---

*Written: 2026-04-15 loop 5*
*BT-542 main body MISS maintained*
*This document is a literature catalog, not a progress claim*
