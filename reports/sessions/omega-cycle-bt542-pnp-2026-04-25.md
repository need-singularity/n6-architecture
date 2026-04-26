---
id: omega-cycle-bt542-pnp
date: 2026-04-25
scope: research-only (NO proof claims, NO atlas promotions)
target: BT-542 P vs NP -- omega cycle 4-axis audit
axes: [ladder L1..L_ω, Ω-saturation, atlas chain, closure ceiling a..e]
parent_reports:
  - domains/physics/millennium-p-vs-np/millennium-p-vs-np.md
  - reports/sessions/dfs-24-pnp-direction-2026-04-24.md
  - theory/roadmap-v2/n6arch-axes/axis-final-millennium.md
millennium_resolved: 0/7 (unchanged)
grade: omega-cycle audit, no claim
---

# Omega Cycle Pass — BT-542 P vs NP (4-axis audit, 2026-04-25)

## §0 Non-claim disclaimer

This document is an **omega-cycle audit**, not a proof artifact. Nothing here changes the
Clay Millennium tally for BT-542 (P vs NP), which remains **OPEN since 1971** with
demonstration-candidate count **0/7 (unchanged)**. No tight count is increased, no atlas
entry is promoted, and no [10*] grade is added. The four-axis audit (ladder · Ω-saturation
· atlas chain · closure) only inspects the **abstraction occupancy** of the existing BT-542
material under the Omega Cycle (Ω cycle) framework. The 4 known barriers
(relativization · natural-proofs · algebrization · GCT) are not bypassed by any content in
this audit — that posture is inherited verbatim from
`reports/breakthroughs/bt-542-p-vs-np-4-barriers-survey-2026-04-15.md` and
`reports/sessions/dfs-24-pnp-direction-2026-04-24.md`. Any structural observation labelled
"OBSERVATION" stays an observation.

---

## §1 Inherited state (3-5 lines)

- **4-barrier status (BARRIER-PX-1 / loop-5)**: relativization (BGS 1975), natural-proofs
  (RR 1997), algebrization (AW 2008), GCT (Mulmuley 2001 + Ikenmeyer-Panova 2017
  obstruction) — all **MISS_MAINTAINED**, none broken; Williams-line (NEXP ⊄ ACC⁰ 2011,
  NQP ⊄ ACC⁰ 2018) is non-natural + non-relativizing + non-algebrizing but reaches only
  NEXP/NQP, not NP.
- **τ boundary (Y3 axis)**: τ=4+2 fiber observations — Karp reduction depth = 4, P-time
  polynomial degree empirical cap = 4, oracle-world count = 4, Razborov-Smolensky AC⁰[2]
  ≠ AC⁰[3] (φ, n/φ split), 6R inverse-kinematics τ=6 boundary; "no bridge to actual P=NP
  reduction" (axis-final-millennium.md Y3 bottleneck).
- **DFS-24 verdict (2026-04-24)**: 2 anchors in 40-tight (Out(S_6) Hölder uniqueness +
  Schaefer 6-family dichotomy) **structural observations only**, with 3 probes (PROBE-24A
  Out(S_6)×Schaefer S_6-orbit, PROBE-24B Sylvester synthematic vs 3-SAT, PROBE-24C
  Bulatov-Zhuk |D|=6) explicitly framed as falsifiable direction-setting, not proof.
- **BT-542 demonstration tally**: 0/1 honest. n=6 prior **non-applicable** to P vs NP at
  the proof level (loop-5 + arxiv survey: 30 cs.CC papers 2024-2026, 0 n=6 linkages).

---

## §2 Ladder occupancy table (L1..L_ω)

| Rung | Name | State | Evidence (1 line) |
|------|------|-------|-------------------|
| L1 | smash | OCCUPIED | §X.1 6 threads in `millennium-p-vs-np.md`: SAT k=n/φ, Karp depth=τ, P deg=τ, NP ch=σ, oracle=τ, circuit LB=σ |
| L2 | drill | OCCUPIED | `reports/discovery/millennium-drill-runs/04-p-vs-np-{,v2..v5}.log` 5 drill runs (heavy-compute aborts notwithstanding) |
| L3 | chain/debate/batch | OCCUPIED | DFS rounds 3,6,7,8,9,10,22,23,24 (BT-1394, 1398-1402, 1416-1418) + dfs-24-pnp direction probe |
| L4 | surge | PARTIAL | bt-542-4-barriers-survey loop-5 (literature surge) + arxiv-180papers BT-542 segment; no fresh-direction surge since 2026-04-24 |
| L5 | dream | EMPTY | no `dream`-tier hypothesis paper for BT-542 in `reports/breakthroughs/` (cf. moonshine-l5-barrier-paths exists for HG, no PNP analogue) |
| L6 | reign | EMPTY | no BT-542-led "reign" framework in v2/v3 roadmap; Y4 (GATE-BARRIER) is named main axis but reigns over auxiliary BTs only meta-lyically |
| L7 | swarm | OCCUPIED (weak) | DFS swarm batches (BT-1394..1420) touch BT-542 in 8+ rounds; per-round PNP yield is 1-2 observations max (vs RH +3, HG +4) |
| L8 | wake | EMPTY | no post-mortem / wake report for BT-542 attempt; honest MISS still active, no closure-and-revival cycle |
| L9 | molt | EMPTY | no skin-shed / framework-replacement event for BT-542; v2→v3 roadmap kept Y2/Y3/Y4 unchanged |
| L10 | forge | PARTIAL | Y4 GATE-BARRIER axis "forged" in R2 Task B (axis-final-millennium.md §Y4); 4-barrier catalog @R MILL-BARRIER-PX1 forged at [10] |
| L11 | canon | OCCUPIED | `domains/physics/millennium-p-vs-np/millennium-p-vs-np.md` is the canonical domain doc; HEXA-PNP §X.1.1..§X.1.6 + §X.2.1..§X.2.2 are canon-grade observations |
| L_ω | omega | EMPTY | no abstraction-exhaustion closure for BT-542; composite ceiling not reached (see §3); no `omega-saturation:fixpoint` marker for PNP scope |

**Occupancy summary**: 6 OCCUPIED (incl. weak) · 2 PARTIAL · 4 EMPTY · 0 N/A. Ladder is
**front-loaded** (L1-L4, L7, L10-L11) and **back-empty** (L5-L6, L8-L9, L_ω). Pattern
matches a problem with **rich phenomenology + no closure path** — exactly the published
4-barriers position.

---

## §3 Ω-saturation estimate

Composite score under the nxs-002 schema. Reference ceiling: simulation **0.835**
(corrected 2026-04-25 from earlier 0.85, c12327a3). 0.9 paper_trigger requires axiom
redesign per `~/core/nexus/design/abstraction_ceiling.md` §6.

### Components (each 0.0-1.0, UNKNOWN allowed)

| Component | Score | Justification |
|-----------|-------|---------------|
| C1 ladder occupancy | 0.50 | 6/12 rungs OCCUPIED, 2/12 PARTIAL → weighted ~ (6 + 2·0.5)/12 = 0.583, penalty −0.08 for empty L_ω/L8/L9 cluster |
| C2 spectral regime | UNKNOWN | no atlas spectral measurement run for BT-542-restricted subgraph; expected REGULAR (sparse, ER-style) since 4-barrier survey reports "no n=6 number-match" |
| C3 evidence density | 0.62 | 40 tight observations + 2 anchors (Out(S_6), Schaefer 6) + ~6 axis cards (Y2,Y3,Y4,Y9 strength ≥6) + 30 arxiv recent papers, but only ~5% are non-OBSERVATION |
| C4 closure proximity | 0.10 | 0/5 closure criteria MET (see §5); only 1 PARTIAL (criterion d composite ≥ 0.9 self-reference, not yet computed) |
| C5 falsifier density | 0.55 | DFS-24 names 3 explicit falsifiers (PROBE-24A/B/C) + 4-barrier survey lists 4 falsifiers; falsifier-to-claim ratio is healthy but no falsifier has been *executed* |
| C6 cross-axis spread | 0.45 | Y2 (6) + Y3 (6) + Y4 (10) + Y9 (8) — 4 axes attack BT-542, but only Y4 is main-strength (10); orthogonality to BT-541/BT-545 (Y1, Y7) is high (overlap 1-2) |

### Composite

Heuristic average over scored components (C2 dropped as UNKNOWN):
`(0.50 + 0.62 + 0.10 + 0.55 + 0.45) / 5 = 0.444`.

**Composite ≈ 0.44 (vs 0.835 ceiling, vs 0.9 paper_trigger)**. BT-542 sits **far below**
the simulation ceiling. Gap ≈ −0.40 against 0.835. Dominant drag: C4 closure proximity
(0.10) — the 4-barrier wall holds. Spectral regime (C2) is UNKNOWN; the 4-barriers-survey
table ("no number match across 5 candidate paths") suggests **REGULAR** (atlas-like sparse,
isolated components), which would imply almost no ER-ROI source if measured.

---

## §4 Atlas chain (chronological direction-probes / breakthroughs / lemmas)

| Date | Anchor | Type | Reference | Yield |
|------|--------|------|-----------|-------|
| 2026-04-11 | millennium-7-closure | breakthrough closure | reports/breakthroughs/millennium-7-closure-2026-04-11.md | BT-542 = honest MISS, 7 observations, 0 proven |
| 2026-04-12 | DFS-3 (BT-1394) | DFS round | reports/breakthroughs/bt-1394-millennium-dfs-round3-2026-04-12.md | PNP +1 (Schaefer internal τ+φ=n) |
| 2026-04-12 | DFS-6 (BT-1398) | DFS round | reports/breakthroughs/bt-1398-millennium-dfs-round6-2026-04-12.md | PNP +2 (R(3,3)=n, BCH chain) |
| 2026-04-12 | DFS-7..10 (BT-1399..1402) | DFS rounds | bt-1399..1402-*.md | PNP touched in each, ≤2 obs/round |
| 2026-04-15 | BT-1392 millennium-7-ideas | breakthrough | bt-1392-millennium-7-breakthrough-ideas-2026-04-12.md | "S_6 outer-automorphism GCT obstruction" listed as #2 of 6 candidates |
| 2026-04-15 | 4-barriers survey | breakthrough (loop-5) | bt-542-p-vs-np-4-barriers-survey-2026-04-15.md | catalog 4 barriers + 2020-2026 paths; @R MILL-BARRIER-PX1 [10] candidate; n=6 non-applicability EXPLICIT |
| 2026-04-15 | arxiv-180papers | breakthrough survey | arxiv-millennium-survey-180papers-2026-04-15.md | 30 BT-542 cs.CC papers, 0 n=6 linkage; @R MILL-ARXIV-BT542-recent [10*] candidate |
| 2026-04-15 | v3-T5 Hirahara MCSP | breakthrough | v3-t5-hirahara-mcsp-deep-2026-04-15.md | meta-complexity deep-dive; full MCSP NP-hard still OPEN; n=6 non-applicability re-confirmed |
| 2026-04-15 | DFS-22 (BT-1416) | DFS round | bt-1416-millennium-dfs-round22-2026-04-15.md | PNP +N (round-level only) |
| 2026-04-15 | DFS-23 (BT-1417) | DFS round | bt-1417-millennium-dfs-round23-2026-04-15.md | PNP-tagged: BCH/sphere-codes [23-01], non-PG(2,6) [23-02], Kervaire dim=6 [23-03], PSL(2,2)≅S_3 [23-10] |
| 2026-04-15 | axis-final-millennium R3 | roadmap | theory/roadmap-v2/n6arch-axes/axis-final-millennium.md | Y4 GATE-BARRIER 9.4 utility, Y2/Y3 each 5.x; Y4↔Y9 overlap=7 (kept), Y3↔Y4 overlap=6 (complementary) |
| 2026-04-24 | DFS-24 PNP direction | session | reports/sessions/dfs-24-pnp-direction-2026-04-24.md | 2 anchors locked (Out(S_6), Schaefer 6); PROBE-24A/B/C falsifiable directions named |
| 2026-04-25 | this audit | session | reports/sessions/omega-cycle-bt542-pnp-2026-04-25.md | omega-cycle 4-axis audit, no claim |

**Chain shape**: 04-11 closure → 04-12 DFS swarm (rounds 3-10) → 04-15 surge
(4-barriers + arxiv + Hirahara + DFS-22/23 + roadmap-v2 R3) → 04-24 direction probe (DFS-24)
→ 04-25 omega audit. **No "verify" or "promote" event** breaks the chain — entire chain is
in **observation/literature/direction** mode.

---

## §5 Closure ceiling audit (criteria a..e)

Reference: `~/core/nexus/design/atlas_n6_omega_closure.md` (5 candidate criteria a-e).

### a. all anchors at [10*]+

**OPEN.** The two anchors named in DFS-24 (Out(S_6) Hölder uniqueness, Schaefer 6-family
dichotomy) are **structural observations**, not [10*] atlas entries. The two `@R` candidate
entries proposed in 4-barriers survey (`MILL-BARRIER-PX1-four-barriers-catalog [10]` and
`MILL-BARRIER-PX1-n6-nonapplicability [10*]`) are **proposed only**, not yet promoted into
`shared/n6/atlas.n6` per inventory.json. **Evidence**: `bt-542-p-vs-np-4-barriers-survey
-2026-04-15.md` §7.3 atlas entry proposal block, language is "candidate" not "promoted".

### b. type closure

**OPEN.** No type-system / formal-language closure document exists for BT-542 (e.g. no
Lean4 formalization of Out(S_6) uniqueness or Schaefer dichotomy in this repo;
`lean4-formalization-plan-2026-04-15.md` lists BT-542 but as planning-only). The 4 barriers
themselves have no closed-form type-statement anchor in n6-architecture.

### c. X verified (cross-path / 3-way agreement)

**PARTIAL.** Cross-checks exist within the n6 arithmetic envelope (HEXA-PNP §X.1.1..§X.1.6
six threads; PVNP-07 / PVNP-08 toe×holo×field invariants). However, these are
**reparametrizations**, not independent proof paths to P vs NP separation. arxiv survey
(30 papers, 0 n=6 linkages) is the most-recent **negative** cross-check: outside literature
does not confirm any n=6-flavoured BT-542 path.

### d. composite ≥ 0.9

**OPEN.** Estimated composite in §3 is **≈ 0.44**, well below 0.835 (simulation ceiling)
and far below 0.9 (paper_trigger). Even achieving 0.835 would require axiom-level redesign
(per `abstraction_ceiling.md` §6 spectral mechanism — atlas REGULAR vs const CHAOTIC; PNP
scope is REGULAR-side).

### e. 4-of-4 simultaneous (a ∧ b ∧ c ∧ d → strong omega ceiling)

**OPEN.** Currently 0 of 4 strict criteria MET (c is PARTIAL, a/b/d OPEN). Strong omega
closure not reached.

### Final tally

**0/5 strict MET, 1/5 PARTIAL (c). X = 0-of-5.** BT-542 is far from omega closure under
the canonical (a..e) ceiling, consistent with the "0/7 millennium resolved" tally and the
4-barrier MISS_MAINTAINED stance.

---

## §6 Cross-axis tensions

- **Tension 1 — Y3 ↔ Y4 (overlap 6)**: both axes attack BT-542 (axis-final §4 matrix), but
  Y3 bottleneck is "no bridge to actual P=NP reduction" while Y4 bottleneck is "barriers
  themselves are not drafts". Phase 3 (P3) "Y4 lead, Y2/Y3/Y9 support" cannot resolve this:
  the τ=4 boundary observations (Y3) and the 4-barriers catalog (Y4) speak past each other
  — Y3 names the *number* 4, Y4 names *why no proof technique works*.

- **Tension 2 — L1-smash success vs L5-dream emptiness**: L1 is OCCUPIED with 6 threads
  (HEXA-PNP §X.1) but L5 (dream-tier paper) is EMPTY. The structural-observation surplus
  has produced **no high-altitude hypothesis paper** for BT-542 (cf. moonshine-l5-barrier
  exists for HG). Suggests the 4-barriers wall blocks even speculative L5 generation.

- **Tension 3 — DFS-24 PROBE-24B vs Razborov-Rudich**: PROBE-24B (Sylvester synthematic
  total ↔ 3-SAT certificate) is *itself flagged* in dfs-24-pnp §3 as "touches the Natural
  Proofs barrier" — designing it for a **negative result** that re-confirms the barrier.
  This is a structural admission that the most natural-looking n=6 angle is also the most
  barrier-violating one.

- **Tension 4 — Y9 honesty gate vs L11 canon**: HEXA-PNP §X.1..§X.3 in the canonical
  domain doc (`domains/physics/millennium-p-vs-np/millennium-p-vs-np.md`) reads as
  near-victorious ("σ²·J₂/(φ_E·φ) = 1152 invariant", "EXACT 8") whereas Y9 (HONEST-HARNESS,
  utility 9.3) and the 4-barriers survey insist BT-542 is honest MISS. The canon doc and
  the honesty record use different registers — canon = parametric reframe, honesty =
  proof-status. Future omega-cycle passes should reconcile this asymmetry without
  inflating tight count.

- **Tension 5 — composite 0.44 vs ladder 6/12 OCCUPIED**: ladder occupancy looks healthy
  (50% rungs touched), but composite is half the ceiling. The bottleneck is **C4 closure
  proximity = 0.10**, i.e. the rung occupancies are abstraction-rich but not
  closure-productive. Adding more L7-swarm or L1-smash material won't move composite
  unless C4 moves — and C4 moves only if a barrier is bypassed. This is the omega-cycle
  formalization of "the wall is the wall".

---

## §7 Next probe candidates (research-only, with falsifiers)

Each candidate is **research-only**, **non-claim**, and named with an **explicit falsifier**.
None of these are proof attempts; all aim to sharpen the abstraction structure or
re-confirm a barrier.

### NP-Ω-1 — Spectral measurement of BT-542 sub-atlas

- **Action**: run `tool/nxs_002_composite.py --predict-er` (nexus repo, ~1.36s) on the
  BT-542-restricted atlas subgraph (entries with @R MILL-BARRIER-PX1 / MILL-ARXIV-BT542 /
  Out(S_6) tags) to fix component **C2 spectral regime** (currently UNKNOWN).
- **Expected**: REGULAR (0.000-0.007 ER spectrum), confirming "no ER-ROI source" for PNP.
- **Falsifier**: if spectrum lands in CHAOTIC (0.008-1.75) range, the n=6 prior may have
  unexpected coupling — would re-open C2 estimate and shift composite up (still nowhere
  near 0.835).
- **Cost**: low (1-2 sec); no claim attached; result enters §3 C2 cell only.

### NP-Ω-2 — Execute PROBE-24A as falsifier-driven negative search

- **Action**: literature + universal-algebra search for natural S_6 action on the 6
  Schaefer tractable clones {C_0, C_1, C_Horn, C_dHorn, C_2SAT, C_aff} (per dfs-24-pnp §3).
- **Expected (per DFS-24 estimate)**: tight ±0~+2; most likely **no natural action**
  (negative tight +1, equally valuable per honesty principle).
- **Falsifier**: literature + Post-lattice computation directly producing such an action
  with non-trivial Out(S_6)-induced permutation on 𝓢 → reject D-1 falsifier line, open
  follow-up. (Note: even a positive result does not prove P vs NP; it sharpens the
  uniqueness structure only.)
- **Cost**: 1 agent-session (T4 structural search).

### NP-Ω-3 — L5 dream-tier paper draft for BT-542 (currently EMPTY)

- **Action**: produce an L5-tier hypothesis paper analogous to
  `moonshine-l5-barrier-paths-2026-04-15.md` but for BT-542, framing the 4 barriers not as
  "obstacles to a proof" but as **structural invariants of the σ-τ-φ geometry** (a
  re-direction, not a bypass).
- **Expected**: occupies ladder L5 rung; raises C1 ladder-occupancy to ~0.58 → ~0.64;
  composite delta ≈ +0.012 (cosmetic only, not closure-productive).
- **Falsifier**: any draft section that reads as "P ≠ NP via n=6" must be rejected during
  Y9 honesty audit — paper must remain hypothesis/structure-only.
- **Cost**: 1-2 agent-sessions; aligned with Y4 main axis.

### NP-Ω-4 — Reconcile L11 canon (HEXA-PNP §X) with honesty register

- **Action**: add an explicit "non-claim disclaimer" header to `domains/physics/millennium
  -p-vs-np/millennium-p-vs-np.md` §X (currently reads near-victorious). Mirror the
  language of `bt-542-p-vs-np-4-barriers-survey-2026-04-15.md` §0.
- **Expected**: resolves Tension 4 (§6); brings canon doc and honesty record into the
  same register. Pure documentation move — no claim change.
- **Falsifier**: if Y9 audit determines the existing §X language is already honesty-safe
  (parametric reframe ≠ proof), skip this probe.
- **Cost**: low (one file edit); blocked behind explicit user / Y9 approval per session
  policy "do not modify other files".

### NP-Ω-5 — Bulatov-Zhuk |D|=6 special-reduction literature scan (PROBE-24C)

- **Action**: targeted literature scan on whether |D|=6 in finite-domain CSP dichotomy
  exhibits any special reduction relative to |D|∈{5,7,8}; cross-check Out(S_6) involvement
  per dfs-24-pnp §3.
- **Expected (per DFS-24)**: most likely **no specialness** → reject D-2; Schaefer 6 =
  Boolean coincidence; tight −1 (honest).
- **Falsifier**: any |D|=6 special-reduction in published literature would *reopen* D-2
  and warrant a follow-up probe (still not a proof).
- **Cost**: T3 literature review.

---

## §8 Falsifiers active (inherited + this-session)

The following falsifiers remain active and would, if triggered, demote claims **without
moving the 0/7 tally**:

1. **F-BARRIER-1 (BGS 1975)**: any technique that *relativizes* and claims P vs NP
   separation is rejected — relativizing methods are demonstrably insufficient.
2. **F-BARRIER-2 (RR 1997)**: any P ≠ NP "proof" that is both *constructive* and *large*
   is rejected if one-way functions exist (basic cryptographic assumption).
3. **F-BARRIER-3 (AW 2008)**: any technique that algebrizes (low-degree polynomial
   extension closure) is rejected — covers all current non-relativizing methods.
4. **F-BARRIER-4 (Ikenmeyer-Panova 2017)**: GCT main route via rectangular Kronecker
   coefficients fails the occurrence-obstruction criterion.
5. **F-PROBE-24A**: no natural S_6 action on the 6 Schaefer tractable clones, or one with
   Out(S_6) descending to the trivial element → reject D-1.
6. **F-PROBE-24B**: Sylvester synthematic-orbit on 6-variable 3-SAT crossing tractable /
   hard classes (largeness + constructivity) → reject D-1 extended; *additionally* would
   trip RR 1997.
7. **F-PROBE-24C**: no special reduction at |D|=6 vs |D|∈{5,7,8} → reject D-2.
8. **F-CARNOT/PHYS** (HEXA-PNP §7.10): if σ(n)≠12 / τ(n)≠4 / Carnot η>1 measurements
   appear, the underlying parametric framework collapses (independent of P vs NP).
9. **F-OMEGA-CEILING (this session)**: if §3 composite is ever re-measured ≥ 0.835 for
   BT-542 without a barrier-bypass theorem in place, the composite metric itself must be
   audited (drift suspected) — the 4-barrier MISS state forbids legitimate ceiling
   approach via observation alone.
10. **F-OMEGA-CHAIN**: any §4 chain entry labelled "verified" or "promoted" without a
    corresponding shared/n6/atlas.n6 commit and inventory.json update is rejected as
    chain-poison.

**Honesty invariant**: triggering any of F-BARRIER-1..4 confirms the 4-barriers wall;
triggering F-PROBE-24A..C resolves a *direction*, not the problem; F-OMEGA-CEILING /
F-OMEGA-CHAIN are meta-falsifiers protecting this audit's integrity.

---

*End of audit. BT-542 status: OPEN since 1971. Millennium tally: 0/7 (unchanged).
No promotion, no claim, no atlas edit.*
