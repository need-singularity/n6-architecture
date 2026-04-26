---
id: omega-cycle-bt541-riemann
date: 2026-04-25
scope: research-only (NO proof claims, NO atlas promotions)
target: BT-541 Riemann hypothesis -- omega cycle 4-axis audit
axes: [ladder L1..L_ω, Ω-saturation, atlas chain, closure ceiling a..e]
parent_reports:
  - domains/physics/millennium-riemann/millennium-riemann.md
  - reports/sessions/dfs-24-riemann-direction-2026-04-24.md
  - theory/roadmap-v2/n6arch-axes/axis-final-millennium.md
  - theory/roadmap-v2/phase-omega-Y9-closure-v3-design.md
millennium_resolved: 0/7 (unchanged)
grade: omega-cycle audit, no claim
---

# Omega-Cycle Audit -- BT-541 Riemann Hypothesis (2026-04-25)

## §0 Non-claim disclaimer

This is a **structural audit only**. No theorem is proven, no [10*] atlas promotion
is recommended, no resolution claim is registered. The 7-millennium tally remains
**0/7** (BT-547 Poincare excluded as Perelman-resolved upstream of this project).
After this audit, BT-541 status is **unchanged**: the Riemann hypothesis body is
untouched, and the only outputs of this pass are (i) a per-rung occupancy map of
the omega ladder, (ii) a saturation estimate against the 0.835 simulation ceiling,
(iii) a temporal chain reconstruction with explicit gaps, and (iv) a closure-criteria
audit. Nothing here promotes any atlas entry from `[10]` to `[10*]`.

---

## §1 Inherited state (5 lines)

- `phase-02-Y1-bt541-riemann.md` final verdict on BT-541: **PARTIAL** (EXACT 10,
  PARTIAL 1, MISS 5, OBSERVATION 2 across 18 sub-items). RH body explicitly
  untouched.
- Strongest peripheral results in scope: Ingham 4th moment (EXACT, formal-p11-1),
  Conrey-Gonek 6th moment with `g_3 = 42` (EXACT, formal-p12-1), Theorem B
  (Bernoulli numerator boundary at k=6, irregular prime 691) -- CANDIDATE for
  `[10] -> [10*]` promotion, currently held.
- Last MISS surface: zero-density on critical line is not bounded by any
  monotone invariant produced by Y1; GRH reduction is MISS
  (`phase-02-Y1-bt541-riemann.md` §3.2).
- Last direction-probe (dfs-24-riemann-direction-2026-04-24): three falsifiable
  probes registered (Lead-A 691 tower, Lead-B SLE_6 x GUE independence,
  Lead-C M-set noise-floor on Bilateral Thm B). **Execution deferred to a separate
  session**, suggested order C -> B -> A. None executed as of this audit.
- `millennium-riemann.md` §X BLOWUP entries RH-01..RH-07 are **arithmetic
  observations**, not RH content; specifically `Re(rho_k) = 1/phi = 1/2` is
  re-stated as an algebraic identity, not derived from zero distribution.

---

## §2 Axis A -- Ladder occupancy

Each rung is marked OCCUPIED / EMPTY / N/A with a single evidence pointer.
"OCCUPIED" means real artefacts exist that match the rung's intended abstraction
level for BT-541; it does **not** mean the rung discharges any RH obligation.

| Rung | State | Evidence pointer (BT-541 specific) |
|------|-------|------------------------------------|
| L1 smash | OCCUPIED | `millennium-riemann.md` §X.1 (critical-line 1/phi, Polya log, GUE Dyson beta=phi=2) |
| L2 drill | OCCUPIED | `theory/study/p1/prob-p1-1-bt541-riemann.md` 279 lines + `theory/study/p2/prob-p2-1-riemann-barriers.md` |
| L3 chain/debate/batch | OCCUPIED | `dfs-24-riemann-direction-2026-04-24.md` (3 leads, all falsifiable, batch-style) |
| L4 surge | PARTIAL-OCCUPIED | `bt-1392-millennium-7-breakthrough-ideas-2026-04-12.md` §1.1 Bilateral Thm B surge; not iterated |
| L5 dream | EMPTY | no speculative Riemann program file under any "dream/" subtree -- needs probe X (search `**/dream*riemann*`) |
| L6 reign | OCCUPIED | `theory/roadmap-v2/n6arch-axes/axis-final-millennium.md` Y1 main + Y7/Y8 sub (governance frame) |
| L7 swarm | EMPTY | no parallel-agent / multi-axis-swarm record exists for BT-541 specifically (axis-final lists axes, no swarm logs) |
| L8 wake | OCCUPIED | `phase-omega-Y9-closure-v3-design.md` S5 (cross-Phase wake of Y1 attack into closure) |
| L9 molt | EMPTY | no "molt" event (deep restructuring of Y1 axis after a failure) registered for BT-541 -- only PARTIAL hold |
| L10 forge | PARTIAL-OCCUPIED | `phase-02-Y1-bt541-riemann.md` §2 Theorem B [10]->[10*] forging attempt, held as CANDIDATE not promoted |
| L11 canon | EMPTY | no canonised lemma under `theory/canon/` mentions BT-541 -- needs probe Y (search canonical lemma index) |
| L_omega | OCCUPIED (this file) | this audit itself is the L_omega entry; prior `nexus omega` invocations on BT-541 UNKNOWN -- needs probe Z |
| --- | --- | --- |
| L1..L11 occupancy | 6 OCCUPIED / 2 PARTIAL-OCCUPIED / 3 EMPTY | -- |

**Reading**: lower rungs (L1..L4, L6) are heavily filled; the **mid-to-upper
abstraction band (L5, L7, L9) is genuinely EMPTY**, and the canon rung (L11) is
EMPTY despite multiple `[10]` candidates around BT-541. This is a structural
imbalance, not a defect that this audit can fix.

---

## §3 Axis B -- Omega-saturation estimate

Reference ceiling (per global instructions, corrected 2026-04-25):
- simulation ceiling: **0.835** (atlas REGULAR regime)
- chaotic regime threshold: 0.008..1.75 (const, 80x atlas)
- 0.9 paper_trigger: requires axiom-level redesign (out of scope for BT-541 alone)

### 3.1 Component decomposition (estimates, not measurements)

BT-541 has not been run through `tool/nxs_002_composite.py --predict-er`; the
following are **structural estimates** based on artefact count and verdict
distribution. Each is annotated UNKNOWN where data is missing.

| Component | Value | Basis |
|-----------|------:|-------|
| ladder occupancy | 6/11 = 0.545 | §2 above (PARTIAL counted as 0.5) -> `(6 + 2*0.5) / 11 = 0.636` |
| ladder occupancy (PARTIAL-weighted) | **0.636** | -- |
| EXACT verdict density (Y1 BT-541) | 10/18 = **0.556** | `phase-02-Y1-bt541-riemann.md` §7.4 |
| MISS verdict density | 5/18 = 0.278 | same source |
| atlas RH-prefix [10*] forged | 4 of 7 (RH-01, RH-05, RH-06, RH-07) | `millennium-riemann.md` §X.6, atlas not edited |
| atlas RH-prefix forging fraction | 4/7 = **0.571** | -- |
| ER ROI estimate | UNKNOWN | needs probe `nxs_002_composite.py --predict-er` against BT-541 sub-graph |
| spectral entropy | UNKNOWN | needs probe -- BT-541 sub-graph not extracted from atlas |
| regular/chaotic split | likely REGULAR | n6 atlas = REGULAR regime per global instructions; BT-541 is fully inside it |

### 3.2 Composite

Naive composite (mean of three available components, each in [0,1]):
`(0.636 + 0.556 + 0.571) / 3 = 0.588`.

Compared to the **0.835** simulation ceiling, BT-541 sits at roughly
**0.59 / 0.835 = 70%** of the simulation ceiling -- i.e. **not saturated**.
This is consistent with the PARTIAL verdict: peripheral artefacts exist but the
chain has not exhausted the available abstraction surface.

### 3.3 Caveats

- This composite is a **proxy**, not the official `nxs_002_composite` score.
  It is reported here only to give a relative position; replacing it with the
  real measurement is itself a future probe.
- The 0.59 estimate ignores chain-temporal saturation (§4) and is therefore
  optimistic; weighting in chain gaps would push the estimate down.
- No claim is made that crossing 0.835 would imply RH progress. The ceiling
  governs **abstraction exhaustion**, not zero-distribution truth.

> **Honesty amendment (2026-04-25, post nxs_002 mapping audit)**: The composite/ER-ROI estimates above measure the **n6-architecture-side graph** only. Per `reports/sessions/omega-exec-nxs002-perbt-patch-2026-04-25.md` §7, the nexus-canonical atlas (`atlas.blowup.jsonl`, 21,320 nodes) does not absorb the n6-side BT-promotion IDs (RH-01..07, MILL-PX-A8, MILL-GALO-PX2-sha-all-squares-332k), so direct comparison against the 0.835 nexus simulation ceiling is invalid. The estimates here remain valid *within n6-architecture scope*; cross-graph comparisons require either (a) per-BT slicing in `nxs_002_composite.py` (currently missing -- mapping-failure diagnostic in §1 of the patch report), or (b) explicit BT-id ingestion into the nexus atlas (out of scope for this audit). 0/7 unchanged.

---

## §4 Axis C -- Atlas chain (chronological)

Reconstructed from file dates and explicit inter-file references. Each row is
(date, axis-rung, verdict, primary artefact). Chain begins with the project's
seed and ends at the present audit. "verdict" here means how the artefact
landed against its own falsifier, **not** RH itself.

| # | date | rung | verdict | artefact |
|---|------|------|---------|----------|
| 1 | 2026-04-08 | L4 | OBSERVATION | `breakthrough-theorems-extension-2026-04-08.md` (early BT scaffolding around Riemann) |
| 2 | 2026-04-11 | L6 | EXACT (boundary record) | `millennium-7-closure-2026-04-11.md` §BT-541 closure -- 7-problem first close |
| 3 | 2026-04-11 | L4 | OBSERVATION | `millennium-n6-attractor-2026-04-11.md` (n=6 attractor, RH peripheral) |
| 4 | 2026-04-11 | L4 | EXACT | `theory/proofs/bernoulli-boundary-2026-04-11.md` (Theorem B original) |
| 5 | 2026-04-12 | L3 | OBSERVATION | `bt-1392-millennium-7-breakthrough-ideas-2026-04-12.md` (Bilateral Thm B as a "breakthrough idea") |
| 6 | 2026-04-12 | L3 | OBSERVATION | DFS rounds 3..18 (`bt-1394` ... `bt-1410`) -- BT-541 contributions accumulate per round |
| 7 | 2026-04-12 | L3 | OBSERVATION | `bt-1409-millennium-dfs-round17-2026-04-12.md` -- SLE_6 locality entry |
| 8 | 2026-04-14..15 | L3 | OBSERVATION | DFS rounds 19..26 (`bt-1411` ... `bt-1420`) -- continued accumulation |
| 9 | 2026-04-15 | L4 | EXACT | `formal-p11-1-selberg-ingham-2026-04-15.md` (Ingham 4th moment) |
| 10 | 2026-04-15 | L4 | EXACT | `formal-p12-1-conrey-gonek-6th-moment-2026-04-15.md` (Conrey-Gonek g_3=42) |
| 11 | 2026-04-15 | L4 | MISS | `formal-p10-1-riemann-sigma-tau-2026-04-15.md` (zeta-zero <-> sigma-tau=8 correspondence MISS) |
| 12 | 2026-04-15 | L6 | governance | `theory/roadmap-v2/n6arch-axes/axis-final-millennium.md` Y1..Y9 finalised |
| 13 | 2026-04-15 | L8 | governance | `phase-omega-Y9-closure-v3-design.md` Phase Omega declared |
| 14 | 2026-04-15 | L10 | CANDIDATE (held) | Theorem B `[10] -> [10*]` promotion attempt (`phase-02-Y1-bt541-riemann.md` §2) |
| 15 | 2026-04-19 | L1 | OBSERVATION | `millennium-riemann.md` §X BLOWUP -- RH-01..RH-07 arithmetic observations |
| 16 | 2026-04-24 | L3 | direction-probe registered | `dfs-24-riemann-direction-2026-04-24.md` -- 3 falsifiable leads (A, B, C) |
| 17 | 2026-04-25 | L_omega | audit (this file) | this report |

### Chain gaps (explicit, EMPTY rungs intersected with temporal flow)

- **G1 (L5 dream)**: no speculative-program artefact between row 14 (Theorem B
  promotion held) and row 15 (RH-01..07 BLOWUP). The BLOWUP is `smash` engine
  output (L1 re-smash), not L5 dream. EMPTY between 2026-04-15 and 2026-04-19.
- **G2 (L7 swarm)**: no multi-axis swarm log; Y1 has been single-leading
  throughout. Y7/Y8 sub-axes are documented as participants but no concurrent
  parallel-attack record exists for BT-541.
- **G3 (L9 molt)**: PARTIAL verdict at row 14 was not followed by a Y1 axis
  redesign. The "molt" expected after PARTIAL did not happen -- the chain
  jumped from PARTIAL directly to BLOWUP re-smash (row 15) and then to a
  direction memo (row 16).
- **G4 (probe execution)**: row 16 leads C/B/A explicitly state "execution in
  a separate session". As of 2026-04-25 (this audit), no execution log appears
  under `reports/breakthroughs/bt-14XX-dfs24-riemann-probes-*`. UNKNOWN whether
  any probe has been started -- needs probe `find reports/breakthroughs/ -name
  '*dfs24-riemann-probes*'` (returned empty in this session).

---

## §5 Axis D -- Closure ceiling audit

Criteria definitions (per global instructions, abstraction_ceiling.md §a..e):

- (a) **[10*]+** -- all relevant atlas entries promoted to `[10*]` or higher
- (b) **type closure** -- the BT's Y-axis classification is complete and frozen
- (c) **X verified** -- `verify_*` harness for the BT passes against external SSOT
- (d) **composite >= 0.9** -- nxs_002 composite score crosses paper_trigger
- (e) **4-of-4 simultaneous** -- (a) AND (b) AND (c) AND (d) all hold at once

Audit:

| # | criterion | state | one-line evidence |
|---|-----------|-------|-------------------|
| a | [10*]+ atlas | **PARTIAL** | `millennium-riemann.md` §X.6 lists 4 of 7 RH-* at [10*]; Theorem B held as CANDIDATE not promoted; 3 of 7 are [10] or [9] |
| b | type closure | **PARTIAL** | Y1 NUM-CORE finalised at utility 9.5 (axis-final-millennium.md §1) but Y8 / Y7 sub contributions remain "interface" not "frozen"; Y9 meta-gate audit not yet sealed for BT-541 |
| c | X verified | **PARTIAL** | `domains/physics/millennium-riemann/verify_millennium-riemann.hexa` exists and asserts zeta(2)=pi^2/n, zeta(-1)=-1/sigma, zeta(0)=-1/phi -- but these are arithmetic identities, not RH content |
| d | composite >= 0.9 | **OPEN** | proxy estimate 0.59 (§3) is below 0.835 simulation ceiling, well below 0.9 paper_trigger; real `nxs_002_composite --predict-er` against BT-541 not run |
| e | 4-of-4 simultaneous | **OPEN** | requires (d) MET, currently OPEN |

**BT-541 closure status: 0-of-5 MET, 3-of-5 PARTIAL, 2-of-5 OPEN.**

(No criterion is fully MET. Three are PARTIAL with concrete pending items;
two are OPEN pending measurement that has not been attempted.)

---

## §6 Cross-axis tensions (5 bullets)

- **Over-built lower ladder, empty upper ladder**: rungs L1..L4 + L6 are dense
  with artefacts (millennium-riemann.md §X has 7 RH-* entries alone), but L5
  dream / L7 swarm / L9 molt are EMPTY. The composite proxy 0.59 is dragged
  down primarily by the upper-rung emptiness, not by lower-rung quality.
- **Closure (a) blocked by chain gap G3**: Theorem B is the obvious `[10*]`
  candidate, but the L9 molt that PARTIAL verdicts usually trigger never
  happened. Without a Y1 redesign post-PARTIAL, the promotion stays held.
- **Closure (c) is shallow**: `verify_millennium-riemann.hexa` checks zeta(2),
  zeta(-1), zeta(0) -- all arithmetic identities. None of them probes
  zero-distribution. (c) being PARTIAL is structural, not fixable by adding
  more arithmetic asserts.
- **Closure (d) blocked by chain gap G4**: the dfs-24 direction memo (row 16)
  registered three falsifiable probes specifically aimed at the noise-floor /
  independence questions that feed the composite, but none has been executed.
  Composite cannot move without those.
- **Closure (e) cannot be reached by current axis stack**: even if (a)..(d)
  are individually closed, the atlas REGULAR regime caps the simulation
  composite at 0.835 (per global instructions), short of the 0.9
  paper_trigger. Reaching (e) for BT-541 would require either (i) finding
  ER/sparse pockets within the BT-541 sub-graph or (ii) admitting that the
  closure ceiling for this BT is structurally below 0.9 and reframing the
  criterion. No such reframing has happened.

---

## §7 Next probe candidates (research-only, falsifier-registered)

Each probe follows the template `(object, tight measurement, falsifier)`.
None of these is a proof attempt.

1. **Probe-1: composite-score measurement**
   - object: BT-541 sub-graph extracted from atlas.n6 (RH-01..RH-07 + Y1 lemmas)
   - measurement: `tool/nxs_002_composite.py --predict-er` run, target wall <= 5s
   - falsifier: if measured composite >= 0.835 simulation ceiling **without**
     any `[10*]` promotion, then either the proxy in §3 was wrong or the
     atlas REGULAR regime assumption is locally violated -- both invalidate
     §3 and §6.
2. **Probe-2: Lead-C M-set noise-floor (already registered, dfs-24)**
   - object: |num(B_{2k})| for k=1..20 versus M-set product representations
   - measurement: fraction r of k in M-set products; threshold r <= 0.10 or r >= 0.30
   - falsifier: r >= 0.30 demotes Bilateral Thm B "sharp boundary" reading
     and reduces Y1 BT-541 EXACT count below 10. **This would change §1
     inherited state.**
3. **Probe-3: chain-gap G3 audit**
   - object: any post-PARTIAL Y1 redesign log between 2026-04-15 and 2026-04-19
   - measurement: `git log --grep='Y1.*redesign\|molt' theory/roadmap-v2/`
   - falsifier: if a redesign log exists, §4 G3 is wrong and the L9 rung is
     OCCUPIED, raising ladder occupancy from 0.636 to ~0.682.
4. **Probe-4: ladder rung L11 canon**
   - object: any canonised lemma referencing BT-541 under `theory/canon/`
   - measurement: `find theory/ -path '*canon*' -type f` then grep "541|Riemann"
   - falsifier: presence of even one canonised lemma toggles L11 from EMPTY
     to OCCUPIED and removes one of the structural-imbalance flags in §6.
5. **Probe-5: Lead-B SLE_6 x GUE independence (already registered, dfs-24)**
   - object: Cardy-formula kappa=6 boundary-crossing distribution vs Odlyzko
     normalised gap distribution
   - measurement: KS p-value
   - falsifier: KS p < 0.01 reconfirms independence and the "11..16
     independent Bernoulli findings" count holds; KS large -> the
     independence count must be reweighted (impacts §3 EXACT density).

---

## §8 Falsifiers active for this audit

- **F1**: if `find reports/breakthroughs/ -name '*dfs24-riemann-probes*'`
  returns one or more files, §4 G4 is wrong and the chain row 16->17 has
  intermediate execution rows that this audit missed; §3 composite proxy must
  be re-estimated.
- **F2**: if `tool/nxs_002_composite.py --predict-er` against the BT-541
  sub-graph returns a composite >= 0.835, §3.2 is wrong (proxy under-estimated)
  and §5 (d) status moves from OPEN towards PARTIAL. The "70% of ceiling"
  reading collapses.
- **F3**: if any of the [10*] RH-* entries in `millennium-riemann.md` §X.6 are
  actually atlas-promoted (i.e. the file's grade column reflects an atlas-edit
  that this audit did not check), §5 (a) state moves from PARTIAL towards MET
  and the closure count rises. (Per global instructions atlas.n6 was not read
  in this audit, so this is the most likely failure mode.)

---

## §9 Audit close

BT-541 omega-cycle audit complete. Resolution count **0/7** maintained. No
atlas file edited, no proposal modified, no inventory touched. The PARTIAL
verdict from `phase-02-Y1-bt541-riemann.md` carries forward unchanged.
Recommended next action is **Probe-1 then Probe-2** (lowest cost, highest
information for §3 and §5(d)). Any subsequent session that executes Probe-2
should also update §1 of this file in place if the M-set noise-floor falsifies
Bilateral Thm B's "sharp boundary" reading.

End of audit.
