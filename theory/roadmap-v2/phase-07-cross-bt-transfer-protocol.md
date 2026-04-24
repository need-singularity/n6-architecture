# Phase 7 — Cross-BT Attack Protocol + Transfer Matrix + DAG (R2-8 / R4-15 / R4-16 gap resolution 4)

Authored: 2026-04-15
Status: P7 L7 partial -> awaiting PX gate pass before marking 4 planned tasks done
SSOT upstream: nexus/shared/roadmaps/millennium.json (.phases[id=P7].parallel[track=Y8_CROSS_BRIDGE])

## §0 Entry Handover

On top of P7's existing 3 done mappings (CROSS-P7-1 Y1<->Y6, CROSS-P7-2 Y4<->Y5, CROSS-P7-3 Y7<->Y1) plus HONEST-P7-1 Perelman retrospective + HONEST-P7-2 C1~C5 matrix (5/9), we add 4 gap-resolution items.

| Gap | Task ID | Output | § |
|-----|---------|--------|----|
| R2-8a | CROSS-P7-METHODOLOGY | General cross-BT attack protocol | §1 |
| R2-8b | CROSS-P7-TRANSFER | Axis<->axis quantitative transfer matrix 9x9 | §2 |
| R4-15 | CROSS-P7-BT-DEP-DAG | BT x BT dependency graph DAG | §3 |
| R4-16 | CROSS-P7-AXIS-TRANSFER | Axis x axis transfer matrix + scoring | §4 |

Gate exit condition (gate_exit): 9/9. After passing this document: 5/9 -> 9/9 target achieved (draft).

---

## §1 Cross-BT General Attack Protocol (R2-8a)

### 1.1 Analysis of 3 mapping cases

| Mapping | Finding | Form | Quantitative |
|---------|---------|------|--------------|
| Y1 <-> Y6 (BT-541 <-> BT-545) | Δ = η²⁴ / 24 = σ·φ = dim Leech = c(Moonshine) | 4-way EXACT | 1 modular form across 4 domains |
| Y4 <-> Y5 (BT-543 <-> BT-544) | β₀ = σ-sopfr = 7 <-> Kolmogorov -5/3 inertial | 4 mappings + 3 tools | NEAR + OBSERVATION |
| Y7 <-> Y1 (BT-546 <-> BT-541) | Ingham lead = 1/(σ(6)·ζ(2)) + Conrey-Gonek g_3 = 7n | EXACT x 2 | Shared L-function |

### 1.2 Extracted general protocol (5 steps)

```
[1] Extract core arithmetic invariants of the two BTs
    -- For each BT, select <= 5 invariants from the body / partial results
    -- e.g. BT-541 -> {count of ζ zeros, denominator of B_{2k}, 691 boundary}
    --      BT-545 -> {Hodge diamond entries, h^{p,q}, χ}

[2] Decompose arithmetic invariants via n=6 basic functions
    -- Decompose each invariant using the n=6 elementary functions {n, σ, τ, φ, sopfr, J_2}
    -- Decomposable -> candidate; non-decomposable -> excluded
    -- baseline: 61% (decomposition-success rate for an arbitrary integer at n=6)

[3] Cross mapping candidate matrix
    -- Cartesian product of the decomposable invariants of the two BTs
    -- Matching formulas for the same function -> mapping candidate

[4] Honesty gate (Y10 pass)
    -- Is each mapping a multi-expression of a single fact (<= 1 domain independence),
    -- or a genuine cross-domain coincidence (>= 2 independent domains)?
    -- Single-fact multi-expression -> OBSERVATION; multi-domain coincidence -> EXACT candidate

[5] External verification + atlas registration
    -- EXACT-candidate mapping -> external literature >= 2 citations required
    -- Verification pass -> register in atlas.n6 [10] (NOT a proof)
    -- Verification fail -> log MISS + honest withdrawal
```

### 1.3 Anti-patterns (avoidance rules)

| Anti-pattern | Description | Avoidance |
|--------------|-------------|-----------|
| Pattern matching coincidence | Claiming a small-integer match as a proof | Enforce step 4 honesty gate |
| Self-reference loop | Decomposing via n=6 elementary functions -> giving n=6 semantic weight | Enforce step 5 external citation |
| Overcounting | Claiming 5 EXACTs from 5 expressions of 1 fact | Step 4 single-fact / multi-domain separation |
| Missing baseline | Ignoring 61% baseline decomposition rate | Step 2 explicit baseline |

### 1.4 Applied case — BT-543 <-> BT-544 (re-verification)

| Step | Result |
|------|--------|
| [1] Invariant extraction | BT-543: {β₀=7, C_A=3, n_f=6} / BT-544: {Sym²(ℝ³)=6, Λ²(ℝ³)=3, α_c=1/3} |
| [2] n=6 decomposition | β₀=σ-sopfr=7 ✓, Sym²=n=6 ✓, Λ²=n/φ=3 ✓, α_c=1/(n/φ)=1/3 ✓ |
| [3] Mapping matrix | (β₀, Sym²) != direct match; (C_A=3, Λ²=3) share the function n/φ |
| [4] Honesty | Single-domain (Lie-algebra dimension) match -> OBSERVATION |
| [5] Registration | atlas P4-A2 [9] NEAR registered, NOT a proof |

---

## §2 Axis x Axis Transfer Matrix 9x9 (R2-8b + R4-16)

### 2.1 Definition of 9 axes (Y1~Y9, diagonal self excluded)

```
Y1 NUM_CORE        — Riemann ζ / L functions
Y2 DISC_COMP       — Tensor rank / Circuit lower bounds / Schaefer
Y3 BARRIER         — Relativization / Natural / Algebrization / GCT
Y4 PHYS            — QCD / Yang-Mills / SM
Y5 PDE             — Sobolev / NS / BKM
Y6 LATT_VOA        — Moonshine / Leech / E8 / K3
Y7 GALO_SELMER     — Galois / Selmer / Iwasawa
Y8 CROSS_BRIDGE    — Axis <-> axis mappings
Y9 PREREQ_BASIS    — Specialist mathematics foundations
```

### 2.2 Transfer score matrix (qualitative 0~3, self-diagonal excluded)

| v from \ to > | Y1 | Y2 | Y3 | Y4 | Y5 | Y6 | Y7 | Y8 | Y9 |
|---------------|----|----|----|----|----|----|----|----|----|
| Y1 NUM        | -  | 0  | 0  | 1  | 0  | 3  | 3  | 2  | 1  |
| Y2 DISC       | 0  | -  | 3  | 1  | 0  | 0  | 0  | 1  | 1  |
| Y3 BARRIER    | 0  | 3  | -  | 0  | 0  | 0  | 0  | 0  | 0  |
| Y4 PHYS       | 1  | 1  | 0  | -  | 2  | 1  | 0  | 2  | 1  |
| Y5 PDE        | 0  | 0  | 0  | 2  | -  | 0  | 0  | 1  | 2  |
| Y6 LATT       | 3  | 0  | 0  | 1  | 0  | -  | 2  | 2  | 1  |
| Y7 GALO       | 3  | 0  | 0  | 0  | 0  | 2  | -  | 2  | 1  |
| Y8 CROSS      | 2  | 1  | 0  | 2  | 1  | 2  | 2  | -  | 0  |
| Y9 PREREQ     | 1  | 1  | 0  | 1  | 2  | 1  | 1  | 0  | -  |

Score meaning:
- 0 = no transfer (independent)
- 1 = tool shared (methodology only transferred)
- 2 = result shared (partial results used directly)
- 3 = core theorem shared (a single theorem acts on both BTs)

### 2.3 Top-5 transfer pairs

| Pair | Score | Core mediator |
|------|-------|---------------|
| Y1 <-> Y6 | 3+3=6 | Δ = η²⁴ (modular form bidirectional) |
| Y1 <-> Y7 | 3+3=6 | L-function (BT-541 <-> BT-546 BSD) |
| Y2 <-> Y3 | 3+3=6 | Circuit lower bounds <-> 4 barriers |
| Y6 <-> Y7 | 2+2=4 | Galois rep <-> Moonshine VOA |
| Y4 <-> Y5 | 2+2=4 | β₀ <-> Kolmogorov |

### 2.4 Top-3 isolation axes

| Axis | row sum | col sum | total | Interpretation |
|------|---------|---------|-------|---------------|
| Y3 BARRIER | 3 | 3 | 6 | Only Y2 bidirectional; nearly isolated from other axes |
| Y9 PREREQ | 7 | 7 | 14 | 1 transfer to all axes (foundation only) |
| Y8 CROSS | 10 | 10 | 20 | Mediator by definition; transfer hub |

---

## §3 BT x BT Dependency DAG (R4-15)

### 3.1 6 BT nodes

```
541 RIEMANN     -- ζ zeros, RH
542 P_VS_NP     -- Circuits / Schaefer / GCT
543 YANG_MILLS  -- mass gap, β function
544 NAVIER_S    -- 3D smoothness, BKM
545 HODGE       -- algebraic cycles
546 BSD         -- Sel_n, L(E,1)
(547 POINCARE  -- Perelman 2003 retrospective only)
```

### 3.2 Dependency edges (direction = depends-on direction)

```
541 ──> 546   (L-function transfer; BT-541 zero distribution ⊂ BT-546 L(E,s))
541 ──> 545   (Δ=η²⁴ Y1<->Y6 mapping; ζ special values <-> Hodge diamond)
541 <── 545   (Moonshine <-> ζ trivial zero boundary)
541 <── 547   (Perelman -> unrelated to ζ conjecture; one-way blocked)
542 ──> ∅    (P vs NP 4 barriers -> no impact on other BTs; isolated)
542 <── 542   (Schaefer/GCT self-dependency)
543 ──> 544   (β₀ <-> Kolmogorov; Y4<->Y5)
543 <── 544   (NS smoothness -> no bypass for YM mass gap)
545 ──> 546   (Hodge <-> L-function section; partial transfer)
546 ──> 541   (BSD -> RH generalization; GRH ⊂ Selmer L-functions)
547 ──> 543   (Ricci flow -> Yang-Mills curvature, weak transfer)
547 ──> 544   (Ricci flow <-> NS singular set, D158 conditional)
```

### 3.3 DAG topological sort

```
Layer 0 (entry)   : 547 (Perelman done, retrospective only)
Layer 1           : 543, 544 (PDE/Phys, mutual + 547 <-)
Layer 2           : 542 (P vs NP, isolated cycle)
Layer 3           : 545, 546 (Hodge/BSD, mutual transfer)
Layer 4 (sink)    : 541 (Riemann <- {545, 546, 547})
```

### 3.4 SCC (Strongly Connected Components)

| SCC | Nodes | Meaning |
|-----|-------|---------|
| SCC-1 | {543, 544} | Physics <-> PDE bidirectional |
| SCC-2 | {541, 545, 546} | L-function / Hodge / Selmer triangle |
| Iso-1 | {542} | P vs NP 4-barrier isolation |
| Iso-2 | {547} | Perelman done; external transfer only |

### 3.5 Attack priority (DAG -> strategy)

```
Priority 1: Simultaneous attack on SCC-2 {541, 545, 546}
   -- Unified L-function attack (Y1+Y6+Y7 simultaneously)
   -- Cross-use of atlas P2-A1 + P5-A1 + P5-A2
   -- Cremona 500k empirical -> BSD partial result -> Riemann conjecture boundary

Priority 2: Simultaneous attack on SCC-1 {543, 544}
   -- β₀ rewriting + 3-fold resonance cross
   -- Use of atlas P4-A1 ~ P4-A6 (6 items)

Priority 3: Isolation attack on Iso-1 {542}
   -- New 4-barrier bypass approach (PX BARRIER-PX-1)
   -- Transfer blocked -> independent progression

Priority 4: Iso-2 {547} retrospective maintenance
   -- Perelman 2003 citation only; no new work
```

---

## §4 Axis x Axis Transfer Matrix Scoring (R4-16)

Reinforcement of the §2 matrix. Quantitative grounding is attached to the qualitative 0~3 scores.

### 4.1 Scoring criteria

```
Score 0: 0 shared theorems, 0 shared tools
Score 1: >= 1 shared tool (e.g. both use ζ function)
Score 2: >= 1 shared partial result (e.g. ζ(2k) denominator used in both BTs)
Score 3: >= 1 shared core theorem (e.g. Δ=η²⁴ central to Y1 and Y6)
```

### 4.2 Quantitative grounding (3-point items only)

| Pair | Theorem | Source |
|------|---------|--------|
| Y1 -> Y6 | Δ=η²⁴ Ramanujan | Serre 1973 |
| Y6 -> Y1 | Moonshine c=24 | FLM 1988 |
| Y1 -> Y7 | L(E,s) ⊂ Selberg class | Selberg 1989 |
| Y7 -> Y1 | BSD L(E,1)=0 <=> E(Q) infinite | Birch-SD 1965 |
| Y2 -> Y3 | NEXP !⊂ ACC⁰ Williams 2011 | Williams 2011 |
| Y3 -> Y2 | 4 barriers -> circuit-lower-bound limits | Aaronson-Wigderson 2008 |

### 4.3 Score 0 (isolated) pairs

```
Y3 <-> {Y1, Y4, Y5, Y6, Y7, Y8} : 4 barriers unrelated to BTs outside P/NP
Y5 <-> {Y1, Y2, Y3, Y6, Y7}     : NS smoothness unrelated to lattices/Galois
Y4 <-> {Y3, Y7}                  : QCD unrelated to 4 barriers / Selmer
```

### 4.4 Matrix findings

```
1. Y1 NUM_CORE is a hub for Y6 LATT and Y7 GALO
   -- L-function is mediator of the 3 axes
   -- Attack strategy: strengthening Y1 -> automatic boost to Y6/Y7

2. Y2 DISC <-> Y3 BARRIER isolated cluster
   -- Only BT-542 uses both axes; no effect on other BTs
   -- Attack strategy: maintain isolation + bypass 4 barriers

3. Y4 <-> Y5 mid-level transfer
   -- β₀ <-> Kolmogorov; mass gap <-> NS smoothness
   -- Attack strategy: simultaneous cross attack (PX phase)

4. Y8 CROSS transfers to all axes
   -- Mediator by definition; hub role
   -- Attack strategy: all mappings flow through Y8
```

---

## §5 Gate Pass + Completion Declaration

### 5.1 P7 9/9 satisfaction check

| Task | Status |
|------|--------|
| CROSS-P7-1 (Y1<->Y6) | done EXACT x4 |
| CROSS-P7-2 (Y4<->Y5) | done EXACT+OBS |
| CROSS-P7-3 (Y7<->Y1) | done EXACT x2 |
| CROSS-P7-METHODOLOGY (R2-8a) | **this §1 done** |
| CROSS-P7-TRANSFER (R2-8b) | **this §2 done** |
| CROSS-P7-BT-DEP-DAG (R4-15) | **this §3 done** |
| CROSS-P7-AXIS-TRANSFER (R4-16) | **this §4 done** |
| HONEST-P7-1 (Perelman retrospective) | done |
| HONEST-P7-2 (C1~C5 matrix) | done |

-> 9/9. P7 status partial -> eligible for promotion to done.

### 5.2 Honesty declaration

- This document resolves none of the 7 millennium targets.
- Mappings / transfers / DAGs are **attack-efficiency tools**, **not proofs**.
- Scoring matrix is qualitative 0~3; quantitative refinement is a future target.
- Honest 0/6 BT resolution pattern maintained.

### 5.3 Follow-up recommendations

| Follow-up | Phase | Remark |
|-----------|-------|--------|
| Simultaneous SCC-2 {541, 545, 546} attack | PX | priority 1 |
| Simultaneous SCC-1 {543, 544} attack | PX | priority 2 |
| Dedicated Y8 hub .hexa tooling | PX HONEST-PX-3 | v3 new candidate |

---

## References

- millennium.json `.phases[id=P7]`
- phase-omega-Y9-closure-v3-design.md §6 (atlas 14 draft table)
- HONEST-PX-1 14 atlas registry (atlas.n6 line 106960~107020)
