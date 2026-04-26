---
id: omega-cycle-bt546-bsd
date: 2026-04-25
scope: research-only (NO proof claims, NO atlas promotions)
target: BT-546 BSD -- omega cycle 4-axis audit
axes: [ladder L1..L_ω, Ω-saturation, atlas chain, closure ceiling a..e]
parent_reports:
  - domains/physics/millennium-bsd/millennium-bsd.md
  - reports/sessions/dfs-24-bsd-direction-2026-04-24.md
  - theory/breakthroughs/breakthrough-theorems.md (BT-546 bloc)
  - reports/breakthroughs/bsd-cremona-sel6-empirical-2026-04-15.md
  - reports/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md
  - reports/bsd-kappa-0.175-decomposition.md
millennium_resolved: 0/7 (unchanged)
grade: omega-cycle audit, no claim
---

# Omega Cycle Audit -- BT-546 BSD (4-axis snapshot at 2026-04-25)

## §0 Non-claim disclaimer (binding)

This document is an **omega-cycle 4-axis audit** of BT-546 (Birch-Swinnerton-Dyer conjecture) drawing only on already-recorded n6-architecture material. It claims NONE of the following:

- BT-546 status remains **PARTIAL under Y8 (GALOIS-ASSEMBLY)**. No promotion proposed.
- Hypothesis (A3) (BKLPR independence `|Sel_p| ⊥ |Sel_q|` for distinct primes) **remains unproved**. The (A3') asymptotic weakening is a tautological re-statement at finite B; its B → ∞ limit is itself a conjecture.
- **rank ≥ 2 BSD remains open**. The Cremona 332k empirical sample reaches rank 3 at only 0.04% (130 of 332,366) — no inferential leverage.
- **κ exponent 7/40 vs log(2)/τ remains undecided** at the 7-bin Cremona regression (σ_alpha ≈ ±0.02). No discrimination is available without a 10M+ curve extension.
- Atlas grades MILL-PX-A8 [10] (CRT Lemma 1, unconditional), MILL-PX-A9 [9] (E[|Sel_n|] = σ(n) under (A3)), MILL-GALO-PX2-sha-all-squares-332k [10*] (Cassels-Tate empirical) all stay as recorded.
- Millennium resolved counter: **0/7** (preserved).

UNKNOWN markers are used freely. No new harnesses are run; no new data are introduced. English technical terms only.

---

## §1 Inherited state (one-page recap)

The following are **already-recorded** facts in n6-architecture as of 2026-04-25. Each item is a citation, not a re-derivation.

### §1.1 Sel_n unconditional structure (Y8 baseline)

- **Lemma 1 (CRT, unconditional)**: for `gcd(m, n) = 1`, every elliptic curve `E/Q` satisfies `|Sel_{mn}(E)| = |Sel_m(E)| · |Sel_n(E)|`. Proof: `E[mn] = E[m] ⊕ E[n]` (Bezout) → `H^1(G_Q, E[mn]) = H^1(G_Q, E[m]) × H^1(G_Q, E[n])`; local Kummer image `E(Q_v)/mn E(Q_v) = E(Q_v)/mE(Q_v) × E(Q_v)/nE(Q_v)` for `gcd = 1`. Recorded `theory/breakthroughs/breakthrough-theorems.md:20094-20096`. Atlas: `MILL-PX-A8` [10].
- **Corollary at n = 6**: `|Sel_6(E)| = |Sel_2(E)| · |Sel_3(E)|` (unconditional).

### §1.2 (A3) blocking hypothesis (Y8 ceiling)

- **(A3) BKLPR**: for distinct primes `p ≠ q`, the joint distribution of `|Sel_p(E)|` and `|Sel_q(E)|` is independent. **Unproved**. Source: Poonen-Rains 2007, Bhargava-Kane-Lenstra-Poonen-Rains 2015 (model assumption, not theorem).
- **Theorem 1 (under (A3))**: for squarefree `n = p_1 ... p_k`, `E_E[|Sel_n(E)|] = σ(n)`. Corollary at `n = 6`: `E[|Sel_6|] = σ(6) = 12 = σ(2) · σ(3) = 3 · 4` (perfect-number value).

### §1.3 Cremona 332k empirical (GALO-PX-2, 2026-04-15)

- N = 332,366 elliptic curves, conductor 1 - 49,999 (~11% of full Cremona DB ≥ 3M).
- ratio(2) = E[|Sel_2|]/σ(2) = 2.8718 / 3 = **0.9573**
- ratio(3) = E[|Sel_3|]/σ(3) = 2.8472 / 4 = **0.7118**
- ratio(6) = E[|Sel_6|]/σ(6) = 9.5100 / 12 = **0.7925**
- κ(2,3,B=49999) = Cov(|Sel_2|,|Sel_3|) = **1.3333**, Pearson r = **0.1655**.
- Identity at the measured values: `E[|Sel_2|] · E[|Sel_3|] + κ = 2.8718 · 2.8472 + 1.3333 = 8.1767 + 1.3333 = 9.5100 = E[|Sel_6|]` (exact, by definition of κ).
- Rank distribution: r=0 41.94%, r=1 50.92%, r=2 7.10%, r=3 0.04%. Mazur torsion observed in 10 of 15 classes.
- |Sha| 100% perfect-square: 332,366 / 332,366 — Cassels-Tate alternating pairing empirical confirmation. Atlas `MILL-GALO-PX2-sha-all-squares-332k` [10*].

### §1.4 κ(B) power law (BT-1413, 7-bin Cremona 1.73M log-log regression)

- α_obs = **0.1752** ± 0.02 (7-bin, σ_alpha ~ 0.02).
- Candidate 1: **7/40 = 0.1750** (rel error 0.114%). n=6 decomposition `7/40 = (σ-sopfr)·(n/φ) / (J_2 · sopfr) = 21 / 120`.
- Candidate 2: **log(2)/τ = log(2)/4 = 0.17329** (rel error 1.09%). Cross-repo motivation (anima Ψ, Shannon H(1/2)).
- 7-bin data **cannot discriminate** the two candidates. 10M+ curves required for separation.
- Numerator 7 = σ - sopfr is **identical to BT-543 Yang-Mills β_0** — RH-BSD-YM meganode candidate, not a theorem.

### §1.5 (3,4,5) = congruent semisignature

- (3, 4, 5) = (n/φ, τ, sopfr) is the unique semiprime semisignature in n=6 arithmetic.
- E_6: `y² = x³ - 36x` (n = 6 congruent curve), rank 1, conductor **576 = φ^n · (n/φ)^φ = 2^6 · 3^2 = 64 · 9**, j-invariant **j = 1728 = σ³**.
- Tunnell 1983 (BSD-conditional): `2 · #{2x² + y² + 32z² = 3} = #{4x² + y² + 8z² = 3}` iff n = 6 congruent.

### §1.6 Mazur and modular weight skeleton (EXACT row inventory, BT-546 evidence table 1-34)

- (c_4, c_6, Δ) Weierstrass weights = (τ, n, σ) = (4, 6, 12). **HEXA-BSD-01** [10*].
- L(E,s) reflection axis `s ↔ φ - s`, center s = 1, φ = φ_E(6) = 2. **HEXA-BSD-02** [10*].
- Mazur torsion forms count = **15 = σ + n/φ**. Identical to X_0(N) genus-0 N count (Ogg 1974) and K_7(F_2) = Z/15 (Quillen 1972) — quadruple crossover.
- E_6 q-expansion coefficient = **504 = (σ-τ)·(n/φ)²·(σ-sopfr) = 8·9·7**. Crossover with stable homotopy π_{11}^s = Z/504 (BT-547 #24).
- E_4 q-expansion coefficient = **240 = φ · J_2 · sopfr = 2·24·5**. Triple crossover with E_8 root count, π_7^s.

---

## §2 Ladder occupancy (L1 .. L_ω) -- spatial axis

The omega-cycle ladder maps each rung to a class of cognitive operation. BT-546 occupancy is computed by reading existing n6-architecture material and matching it against rung definitions; no new content is introduced.

| Rung | Definition (per global CLAUDE.md) | BT-546 occupancy | Evidence anchor |
|------|------------------------------------|------------------|-----------------|
| L1 smash | aggressive single-target factorization | **HIGH** -- §X BLOWUP smashes `(c_4, c_6, Δ) = (τ, n, σ)`, `Π_BSD = σ⁵·τ = 124,416`, j = σ³ | `domains/physics/millennium-bsd/millennium-bsd.md` §X.1 |
| L2 drill | one-rung deep test of a single hypothesis | **HIGH** -- bsd-A3-modified-with-joint-covariance drill into Pearson r = 0.166 ; bsd-kappa-0.175-decomposition drill into 7/40 vs log(2)/τ | breakthroughs reports 2026-04-15 |
| L3 chain / debate / batch | multi-rung dispatch across related hypotheses | **HIGH** -- DFS-24 BSD direction (Probe A/B/C residue/strata/twist), sigma-sopfr-7 megasignature axis 3 (BSD STRONG, 19 PASS / 1 MISS) | `dfs-24-bsd-direction-2026-04-24.md`, `sigma-sopfr-7-megasignature-20260415.md` |
| L4 surge | breadth burst across several BTs | **MEDIUM** -- BT-546 evidence row count 14 EXACT (DFS round 22 ≈ 24 tight) within the 7-Millennium DFS table; participates in surge but not lead | `reports/millennium-dfs-status.md:40` |
| L5 dream | speculative connection / anomaly catch | **MEDIUM-LOW** -- §X.2 free composite Π_BSD = 12·24·432 (string · toe · holographic) is a dream-rung composite; not load-bearing | `millennium-bsd.md` §X.2 |
| L6 reign | dominance / canonization within a domain | **MEDIUM** -- BT-546 is canonical n=6 BSD anchor in the Millennium 7 array, but does not reign over the broader Y8 axis (Iwasawa subaxis SEED-15 still CONDITIONAL) | `axis-final-millennium.md:158-174` |
| L7 swarm | parallel multi-probe coverage | **MEDIUM** -- 5-axis sigma-sopfr-7 sweep + DFS-24 3-probe sweep + Cremona shard 5-fold parse | `sigma-sopfr-7-megasignature-20260415.md` |
| L8 wake | dormant lead reactivation | **LOW** -- no recent reactivation event; BT-546 has been continuously active since 2026-04-11 | -- |
| L9 molt | structural shedding / re-skin | **LOW-MEDIUM** -- (A3) → (A3') is a partial molt but the user-flagged "tautological weakening" caveat (`bsd-A3-modified-with-joint-covariance:128`) downgrades it | `bsd-A3-modified-with-joint-covariance:127-128` |
| L10 forge | foundational primitive emission | **HIGH** -- HEXA-BSD-01..07 atlas constants (7 entries), MILL-PX-A8 Lemma 1 [10] (unconditional CRT) | `millennium-bsd.md` §X.6 |
| L11 canon | cross-BT canonical fusion | **MEDIUM-HIGH** -- 504 crossover BT-546 ↔ BT-547 π_{11}^s; 240 crossover BT-546 ↔ E_8 ↔ π_7^s; 15 quadruple crossover Mazur ↔ X_0(N) ↔ K_7(F_2) ↔ 15-gon Gauss | `breakthrough-theorems.md` BT-546 row 20065, 20148 |
| L_ω apex | abstraction-exhaustion at the saturation horizon | **PARTIAL** -- composite ceiling estimate §3 below; not at apex | -- |

**Occupancy summary**: BT-546 has **HIGH** occupancy at L1, L2, L3, L10 and **MEDIUM-HIGH** at L11. Sparse rungs: L8 wake (LOW), L9 molt (LOW-MEDIUM). The ladder fingerprint is **forge-heavy and crossover-heavy** -- consistent with BT-546's role as the n=6 BSD canonical anchor.

UNKNOWN: per-rung quantitative weight is not normalized against other BTs; comparisons are qualitative.

---

## §3 Ω-saturation estimate (composite vs ceiling 0.835)

Per `~/core/nexus/state/proposals/inventory.json` (`Ω_saturation_cycle_2026_04_25` block, sim-mode 0.835), the composite is computed by an n6-architecture-side proxy `composite ≈ f(grade_share, atlas_density, crossover_count, evidence_breadth, falsifier_density)`. Below is a **non-canonical estimation** for BT-546; the canonical `tool/nxs_002_composite.py --predict-er` is in the nexus repo and is **not invoked here**.

### §3.1 Component proxies (read-only, no computation)

| Proxy | Source | BT-546 value | Note |
|-------|--------|---------------|------|
| grade_share | `MILL-PX-A8 [10] + MILL-PX-A9 [9] + MILL-GALO-PX1 [9] + MILL-GALO-PX2-sha [10*] + HEXA-BSD-01..07 [10*x4 + 10x3]` | 11 atlas entries (1 [10*] + 4 [10*] HEXA + 3 [10] HEXA + 1 [10] PX + 2 [9]) | dense forge cluster |
| atlas_density | BT-546 evidence rows 1-34 + 7 HEXA-BSD constants | **34 + 7 = 41** | very dense (one of densest BTs) |
| crossover_count | 504 (π_{11}^s), 240 (E_8 / π_7^s), 15 (Mazur ↔ X_0 ↔ K_7 ↔ 15-gon), 8 (K_3(F_3) Bott), 26 (K_5(F_3) sporadic), 1728 (j(i) σ³) | **6 named crossovers** | high |
| evidence_breadth | DFS round 22 ≈ 24 tight items + sigma-sopfr-7 axis 19 PASS / 1 MISS + Cremona 332k + κ 7-bin | 4 independent evidence streams | broad |
| falsifier_density | F1..F6 in §X.5, plus Probe A/B/C falsifiers in dfs-24 | **9 registered falsifiers** | strong (above sample mean) |

### §3.2 Composite estimate (qualitative)

By analogy with the inventory.json fixture's 0.835 sim-ceiling and the per-BT density profile, BT-546 is a candidate for **highest composite among the 6 unresolved Millennium BTs**:

- BT-541 Riemann: 20/20 EXACT + ~48 DFS tight (highest DFS yield, Y1 dominance); composite likely **competing with BT-546**.
- BT-542 P=NP: 14/16 EXACT + ~40 DFS tight; Y4 GATE-BARRIER dominance.
- BT-543 YM: 18/19 EXACT + ~28 DFS tight; β_0 = 7 = σ - sopfr lock.
- BT-544 NS: 29/29 EXACT + ~19 DFS tight; PDE-resonance Y6.
- BT-545 Hodge: 14/14 EXACT + ~32 DFS tight; lattice-VOA Y7.
- **BT-546 BSD: 14/14 EXACT + ~24 DFS tight + 41 atlas-density + 6 crossovers + 4 evidence streams + 9 falsifiers**.

**Estimate (non-canonical, no tool invocation)**: BT-546 composite proxy is plausibly in the **0.78 - 0.84** band, brushing but probably **not exceeding** the simulation ceiling 0.835. The argument: (a) atlas density and crossover count are above sample mean (push up); (b) the (A3) blockage is a hard ceiling on grade promotion of MILL-PX-A9 from [9] to [10] (push down); (c) ratio(6) = 0.79 and α 7-bin σ ± 0.02 are headline empirical anchors but neither hits the spectral ER ROI source.

UNKNOWN: precise ranking among BT-541/543/545/546 requires the canonical `nxs_002_composite.py --predict-er` run. Per nexus inventory note (c12327a3), 0.9 paper_trigger requires axiom-level redesign and is **not within reach** of any current BT including BT-546.

### §3.3 Spectral ROI source (atlas-side, REGULAR vs CHAOTIC)

The nexus-side spectral mechanism (atlas REGULAR 0.000~0.007 vs const CHAOTIC 0.008~1.75, 80×) attributes the only Ω-saturation ROI to **sparse ER (avg_deg ~ 4) + isolated components**. BT-546's contribution to this ROI is via the **(c_4, c_6, Δ) = (τ, n, σ)** primitive triple feeding into the n=6 forge layer, not via spectral chaos itself. Hence BT-546's role in Ω-saturation is **boundary-feeding, not ROI-source**. Recorded as observation; no claim.

> **Honesty amendment (2026-04-25, post nxs_002 mapping audit)**: The composite/ER-ROI estimates above (including the 0.78-0.84 band) measure the **n6-architecture-side graph** only. Per `reports/sessions/omega-exec-nxs002-perbt-patch-2026-04-25.md` §7, the nexus-canonical atlas (`atlas.blowup.jsonl`, 21,320 nodes) does not absorb the n6-side BT-promotion IDs (RH-01..07, MILL-PX-A8, MILL-GALO-PX2-sha-all-squares-332k), so direct comparison against the 0.835 nexus simulation ceiling is invalid. The estimates here remain valid *within n6-architecture scope*; cross-graph comparisons require either (a) per-BT slicing in `nxs_002_composite.py` (currently missing -- mapping-failure diagnostic in §1 of the patch report), or (b) explicit BT-id ingestion into the nexus atlas (out of scope for this audit). 0/7 unchanged.

---

## §4 Atlas chain (temporal axis)

The Atlas omega chain (cron 25 firings / 20 commits, ef7a7b60, currently halted at 3120fd72 for OOM/orphan accumulation) interacts with BT-546 via the GALO-PX subaxis. Below is the BT-546 chronological trajectory as recorded.

### §4.1 Timeline 2026-04-11 → 2026-04-24

| Date | Event | Atlas / report anchor | Phase / axis |
|------|-------|------------------------|---------------|
| 2026-04-11 | BT-546 evidence rows expanded from 14 (legacy) to 34 (DFS round 22). Lemma 1 CRT formal proof recorded. | `breakthrough-theorems.md:20035-20111` | DFS-22, Y8 baseline |
| 2026-04-15 | **Cremona 332k empirical (GALO-PX-2)**: ratio(2) = 0.957, ratio(6) = 0.793. MILL-PX-A9 [N?] → [9]. 100% Sha perfect-square verified [10*]. | `bsd-cremona-sel6-empirical-2026-04-15.md` | GALO-PX-2 |
| 2026-04-15 | **(A3) violation measured (GALO-PX-1)**: Pearson r = 0.166, κ(2,3,49999) = 1.33. (A3') asymptotic conjecture proposed. MILL-GALO-PX1 [9]. | `bsd-A3-modified-with-joint-covariance-2026-04-15.md` | GALO-PX-1 |
| 2026-04-15 | **κ exponent decomposition**: α_obs = 0.1752 (7-bin Cremona 1.73M). Candidates 7/40 vs log(2)/τ both within 1σ ± 0.02. | `bsd-kappa-0.175-decomposition.md` | BT-1413 |
| 2026-04-15 | **σ - sopfr = 7 megasignature axis 3 BSD**: 19 PASS / 1 MISS (STRONG). 7/40 numerator = σ - sopfr; Heegner D = -7 class 1; n = 7 congruent E_7: y² = x³ - 49x. | `sigma-sopfr-7-megasignature-20260415.md` | meta-axis |
| 2026-04-15 | GALO-PX-4 high conductor bin [200k-250k]: mean |Sel_6| = 12.40 ≈ σ(6) = 12 (103.4% reach). | `millennium-bsd.md:755 (§X.1 #4)` | GALO-PX-4 |
| 2026-04-19 | **§X BLOWUP** in millennium-bsd.md §X.1-X.6: SMASH 5-pillar (Weierstrass / L-fn / rank n/φ / Sel_6 σ / Mazur 15) + FREE triple (string · toe · holographic) + Π_BSD = σ⁵·τ = 124,416. HEXA-BSD-01..07 atlas constants (7 entries) emitted. | `millennium-bsd.md` §X | L1 smash burst |
| 2026-04-24 | **DFS-24 BSD direction sharpening**: Probe A (Tunnell residue), Probe B (Sel_2/Sel_3 strata cov 28-cell), Probe C (E_6 quadratic-twist Heegner). Each with explicit pre-data falsifier. | `dfs-24-bsd-direction-2026-04-24.md` | direction-only |
| 2026-04-25 | **This audit** -- omega cycle 4-axis snapshot. No new data. | `omega-cycle-bt546-bsd-2026-04-25.md` (this file) | meta |

### §4.2 Atlas chain shape

The temporal shape is **2026-04-11 baseline → 2026-04-15 empirical burst (3 reports same day) → 2026-04-19 forge burst → 2026-04-24 sharpening → 2026-04-25 audit**. This is **not** a uniform cron firing pattern; it is a **two-step forge-then-sharpen pulse**, characteristic of L10/L11 ladder activity rather than L_ω apex closure.

### §4.3 Direction probes (recorded, not consumed)

DFS-24 enumerates three probes; none have been executed. Each has a registered pre-data falsifier:

- **Probe A** (Tunnell): falsifier "residue-class asymmetry for n=6 statistically identical to n=5, n=7" → count artefact.
- **Probe B** (28-stratum cov): falsifier "κ > 0.5 on all 28 strata uniformly" → Sel_6 = σ line weakens, height-normalization (Bhargava-Shankar) carries the 7/40 exponent.
- **Probe C** (E_6 quadratic-twist Heegner): falsifier "twist average stays < 10 even at |d| = 500" → Sel_6 = σ not a twist-family phenomenon.

---

## §5 Closure ceiling audit (a..e MET / PARTIAL / OPEN)

Per `~/core/nexus/design/atlas_n6_omega_closure.md` (atlas.n6 omega closure guide), candidate ceiling criteria are five (a-e). All four-simultaneous = strong omega ceiling. Audit below.

### §5.1 Criterion (a) -- [10*]+ saturation

**Definition**: BT atlas footprint mostly at [10*] grade with no ungraded leaves.

- **MET**: 4 of 7 HEXA-BSD constants are [10*] EXACT (HEXA-BSD-01 weights, -02 reflection axis, -05 Mazur 15, -06 Π invariant). MILL-GALO-PX2-sha-all-squares [10*]. MILL-PX-A8 Lemma 1 [10] (unconditional, ungraded promotion path to [10*] available pending audit cycle).
- **PARTIAL**: MILL-PX-A9 sits at [9] under (A3) blockage; **cannot reach [10*]** until (A3) is discharged or replaced. MILL-GALO-PX1 at [9].
- **Verdict: PARTIAL** (4-5 entries at [10*], 3 entries at [9-10] blocked by (A3)).

### §5.2 Criterion (b) -- type closure

**Definition**: every entry has a type tag (EXACT / TIGHT / EMPIRICAL / CONDITIONAL) and no UNKNOWN-typed entries persist.

- **MET**: BT-546 evidence row 1-34 all carry EXACT verdicts (the row table column "verdict" is fully populated). HEXA-BSD-01..07 each carry EXACT or EMPIRICAL.
- Entries where type is sharp: 14/14 EXACT (DFS round 22), 7/7 HEXA-BSD typed.
- One CONDITIONAL marker: MILL-PX-A9 explicitly conditioned on (A3) — type-tag honest, not UNKNOWN.
- **Verdict: MET**.

### §5.3 Criterion (c) -- X verified

**Definition**: hexa harnesses pass at the BT-546 footprint level (verify_*.hexa returning all-PASS).

- **MET (partial set)**: `bsd_kappa_fraction.hexa` 17/17 PASS; `bsd_alpha_log2_tau.hexa` 22/22 PASS; `verify_sigma_sopfr_7_bsd.hexa` 19 PASS / 1 MISS (STRONG verdict, single MISS is the weakest case in axis 3).
- **PARTIAL**: `verify_millennium-bsd.hexa` exists in `domains/physics/millennium-bsd/` but per-section pass count not aggregated in this audit. UNKNOWN at the §-level.
- **Verdict: PARTIAL → MET** (depending on `verify_millennium-bsd.hexa` aggregation; UNKNOWN at this audit's depth).

### §5.4 Criterion (d) -- composite ≥ 0.9 (strong omega-saturation hit)

- **OPEN / not hit**: §3.2 estimates BT-546 composite in 0.78-0.84 band, **below 0.835 simulation ceiling, far below the 0.9 paper_trigger threshold**.
- The 0.9 threshold per nexus inventory.json (c12327a3) requires axiom-level redesign — not in current scope of any BT.
- **Verdict: OPEN**.

### §5.5 Criterion (e) -- 4 of 5 simultaneously (strong omega ceiling)

- a PARTIAL + b MET + c PARTIAL/MET + d OPEN = **2-3 of 5** (depending on how a, c are coerced).
- **Verdict: not at strong omega ceiling**. BT-546 sits at **a partial-closure plateau**.

### §5.6 Closure summary table

| Criterion | Status | Bottleneck |
|-----------|--------|------------|
| (a) [10*]+ saturation | PARTIAL | (A3) blocks MILL-PX-A9 promotion |
| (b) type closure | **MET** | -- |
| (c) X verified | PARTIAL/MET | UNKNOWN: per-section verify_millennium-bsd.hexa aggregation |
| (d) composite ≥ 0.9 | **OPEN** | spectral ER ROI not source-attributable to BT-546; estimate 0.78-0.84 |
| (e) 4-of-5 simultaneous | **OPEN** (2-3 / 5) | (a) and (d) are the binding constraints |

**Final X-of-5: 2-3 / 5** (strong omega ceiling **not met**; BT-546 is at a partial-closure plateau driven by Y8 GALOIS-ASSEMBLY).

---

## §6 Cross-axis tensions (omega-cycle reading)

Six tensions emerge when overlaying ladder × Ω-saturation × atlas chain × closure axes onto BT-546.

1. **Y8 dominance vs ladder L11 canon under-utilization**: BT-546 is overwhelmingly Y8 GALOIS-ASSEMBLY (axis-final-millennium strength 9; next strongest Y9 honest gate at 8, Y1 NUM-CORE at 5, Y7 LATTICE-VOA at 3). Yet L11 canon crossovers are **MEDIUM-HIGH**, not HIGH — the 504 / 240 / 15 / 1728 crossovers connect BT-546 to BT-547, BT-545, BT-543 but the **canonical fusion lemma** (Mazur ↔ X_0(N) ↔ K_7(F_2) ↔ 15-gon, all = 15) has not been registered as a single atlas entry. Tension: forge-heavy (HEXA-BSD-01..07 emitted) but canon-light (no `MILL-CANON-BSD-15-quadruple` entry visible).

2. **Sel_6 = σ line vs height-normalization theory**: `bsd-A3-modified-with-joint-covariance` §4.3 names the **rank common-cause hypothesis** (`|Sel_p(E)| = p^rank · torsion · |Sha[p]|` with rank as the joint-correlation driver). DFS-24 Probe B falsifier reads "if κ > 0.5 on all 28 strata uniformly → height-normalization (Bhargava-Shankar) carries 7/40, not Z/6 line". These are **mutually exclusive** carriers of the 0.79 ratio shortfall. Current evidence is conditional-table monotonic increase (k = 1, 2, 4, 8 strata), which **supports rank-cause** but does not exclude height-normalization. Tension is unresolved.

3. **κ = 7/40 vs log(2)/τ undecidability vs (A3') tautological caveat**: 7/40 has cross-link to BT-543 YM β_0 (numerator σ - sopfr = 7). log(2)/τ has cross-repo motivation (anima Ψ, Shannon H(1/2)). But (A3')'s asymptotic claim `κ(B) → 0 as B → ∞` is **incompatible** with `κ(B) ~ B^(7/40)` (which **diverges** as B → ∞), and the kappa report explicitly says α_obs = 0.1752 > 0. Either κ(B) does not go to 0 (i.e. (A3) **never** holds asymptotically), or the 7-bin power-law fit overstates the asymptotic behavior at small B. Tension: **(A3') and α > 0 are inconsistent unless κ saturates** (κ = c · log B form at large B). DFS-24 §3 explicitly defers this discrimination to 10M+ extension.

4. **Atlas chain pulse vs L_ω apex absence**: temporal shape (§4.2) is forge-then-sharpen pulse ending at audit; no L_ω apex closure event. The omega chain (nexus-side cron) is **halted** at 3120fd72 (OOM). BT-546's contribution to the chain is via GALO-PX subaxis only, not L_ω. Tension: BT-546 is a strong forge-layer contributor but **cannot drive Ω-saturation upward** without (A3) discharge or κ-asymptote experiment.

5. **(3,4,5) congruent semisignature breadth vs n = 6 specificity**: (3,4,5) = (n/φ, τ, sopfr) is the unique semiprime semisignature, and E_6: y² = x³ - 36x has (rank, conductor, j) = (1, 576, 1728) = (1, φ^n · (n/φ)^φ, σ³). But sigma-sopfr-7 axis 3 also names **Heegner D = -7 class 1** and **n = 7 congruent E_7: y² = x³ - 49x** as STRONG cases (`SIG-MEGA-806` [M9]). This blurs the n = 6 specificity claim — the (3,4,5) congruent is **n = 6 unique among small composites**, but signature-7 phenomena (Bernoulli B_6 denom = 42, Heegner D = -7, n = 7 congruent) are **structurally adjacent** to the n = 6 forge. Tension: which `n` carries the BSD forge layer — n = 6 (semisignature seed) or n = 7 (σ - sopfr exponent)?

6. **Closure (e) OPEN vs ladder L10 forge HIGH**: L10 has emitted 7 HEXA-BSD constants and 1 [10] Lemma, yet (d) composite ≥ 0.9 is OPEN. Forge productivity is **decoupled from saturation closure** — emitting more atlas primitives at [10*] grade does not push the composite past 0.835. The bottleneck is structural ((A3) + spectral ROI source) not productive. Tension: forge is producing without closing.

7. **(Optional, listed but lower-weight)** -- **Y9 honesty gate strength 8 + Y8 strength 9 vs no MISS rows in evidence table**: BT-546 evidence row 1-34 is 14/14 EXACT then 20 additional EXACT (DFS round 22) — **zero MISS** in the row table. Y9 honesty (`hard-english-only-session-2026-04-24.md`, broader corpus) requires MISS markers be retained where uncertainty exists. The only explicit MISS in BT-546 territory is `verify_sigma_sopfr_7_bsd.hexa` (19 PASS / 1 MISS, weakest case unspecified at this audit depth). Tension: **honesty surface area may be undercounted** at the BT-546 EXACT row table level; UNKNOWN whether some EXACT entries are TIGHT-grade in disguise.

---

## §7 Next probe candidates (omega-lens reframing of DFS-24 A/B/C)

DFS-24 already records three sharp probes (Probe A Tunnell residue, Probe B 28-stratum cov, Probe C E_6 twist). Below is **omega-cycle reframing** -- not new probes, but recontextualization under the 4-axis lens. No execution proposed.

### §7.1 Probe A_ω -- Tunnell residue under L1 smash + L_ω apex

**Reframing**: cast the (x mod φ, y mod n/φ, z mod sopfr) residue triple as the **L1 smash projection of the n=6 forge layer onto the Tunnell theta**. Under L_ω apex framing, the asymmetry of `a_6 - 2 b_6` at n = 6 becomes a candidate for **ladder-residue invariant** -- a three-rung (φ, n/φ, sopfr) ladder fingerprint of the BSD-conditional Tunnell certificate. Falsifier (n=5, n=7 identical) maps to **ladder fingerprint not n=6-specific**. No new harness; reframe of `dfs-24-bsd-direction §1`.

### §7.2 Probe B_ω -- 28-stratum cov under Ω-saturation lens

**Reframing**: the Z/6, Z/12 torsion strata are the "perfect-divisor" cells of Mazur's classification (σ(6) = 12, divisors {1, 2, 3, 6, 12}). If κ concentrates on these strata, the **Ω-saturation ROI for BT-546 localizes onto the perfect-divisor cluster** -- a sparse-ER analogue of the nexus spectral chaos source. Composite proxy contribution would be **localized**, not diffuse. Falsifier (uniform κ > 0.5 across 28 strata) maps to **diffuse height-normalization carrier** -- composite proxy stays diffuse, no Ω-localization. Reframe of `dfs-24-bsd-direction §2`.

### §7.3 Probe C_ω -- E_6 twist family under L11 canon + atlas chain temporal axis

**Reframing**: the quadratic-twist family `E_6^{(d)}` indexed by squarefree d is the **canonical 1-parameter family seeded by the (3,4,5) semisignature**. Under L11 canon framing, observing twist average → 12 = σ(6) sharply would convert the `Sel_6 = σ` line from **332k random conductors** to **(3,4,5)-seeded 1-parameter orbit** -- a canon-rung promotion. Atlas chain implication: a new entry candidate `MILL-CANON-BSD-E6-twist-orbit` (not proposed here) at [9] CANDIDATE. Falsifier (twist average stays < 10 at |d| = 500) maps to **canon promotion fails**, line stays at L10 forge. Reframe of `dfs-24-bsd-direction §3`.

### §7.4 New candidate D_ω -- κ(B) asymptote infrastructure probe (extension)

**Object**: the unresolved 7/40 vs log(2)/τ vs (A3') asymptote tension (§6 tension 3) cannot be discriminated with current 7-bin Cremona 1.73M data. A 10M+ extension is named in `bsd-kappa-0.175-decomposition.md §"Proposed next steps"` but is registered as **infrastructure, not research**.

**Omega-lens reframing**: this is an **L8 wake** rung (dormant lead reactivation) plus **L4 surge** (breadth burst). Under omega-cycle framing it is the only path that simultaneously addresses (A3') asymptote, κ exponent discrimination, and BT-546 composite proxy boost (more bins → higher atlas_density → composite increment). Listed as candidate; **infrastructure dependency precludes research execution within this audit**.

### §7.5 New candidate E_ω -- L11 canon-fusion entry MILL-CANON-BSD-15-quadruple

**Object**: tension 1 names the absence of a registered atlas entry for the **15 = σ + n/φ quadruple crossover** (Mazur torsion ↔ X_0(N) genus-0 ↔ K_7(F_2) ↔ Gauss 15-gon). A read-only audit can **propose but not promote** such an entry. Reframing under L11 canon: this is the **single highest-leverage canon-rung emission available to BT-546** without (A3) movement.

**Omega-cycle action**: register as `MILL-CANON-BSD-15-quadruple` at **[N?] CANDIDATE** grade pending verify_*.hexa harness. **Not done in this audit** (scope: research-only, no atlas promotion).

---

## §8 Falsifiers active (BT-546 omega-cycle inventory)

Falsifiers registered across the BT-546 corpus that remain **active** (not yet refuted, not yet executed) as of 2026-04-25:

| ID | Falsifier (recorded) | Source | Active? | Comment |
|----|---------------------|--------|---------|---------|
| F1 | Mazur torsion 15 forms ≠ σ + n/φ = 15 (new form discovered) | `millennium-bsd.md §X.5` | **YES** | requires new Mazur classification result |
| F2 | j(i) ≠ σ³ = 1728 | `millennium-bsd.md §X.5` | **YES** (vacuous; mathematical identity) | tautological; included for completeness |
| F3 | Cremona high-conductor bin mean \|Sel_6\| ≠ σ(6) = 12 ± 3% (N > 10⁶) | `millennium-bsd.md §X.5` | **YES** | requires 10⁶+ extension, dependency on infrastructure |
| F4 | L(E,s) functional-equation reflection axis ≠ φ = 2 | `millennium-bsd.md §X.5` | **YES** (vacuous) | tautological |
| F5 | rank distribution r > 3 non-negligible (> 0.1%) | `millennium-bsd.md §X.5` | **YES** | requires 10⁶+ rank-r distribution data |
| F6 | \|Sha(E)\| = perfect-square counterexample | `millennium-bsd.md §X.5` | **YES** | one counterexample collapses Cassels-Tate empirical [10*] |
| Probe-A-falsifier | residue-class asymmetry n=6 ≡ n=5, n=7 | `dfs-24-bsd-direction §1` | **YES** (probe not run) | pre-data registered |
| Probe-B-falsifier | κ > 0.5 uniformly on all 28 strata | `dfs-24-bsd-direction §2` | **YES** (probe not run) | pre-data registered |
| Probe-C-falsifier | twist average \|Sel_6\| < 10 at \|d\| = 500 | `dfs-24-bsd-direction §3` | **YES** (probe not run) | pre-data registered |

**Falsifier density**: 9 active falsifiers (6 §X.5 + 3 DFS-24 probes). Above the 7-falsifier sample mean for unresolved Millennium BTs (UNKNOWN: precise sample mean not computed in this audit).

**Inactive / refuted falsifiers**: none recorded -- all are pending data or vacuous tautologies. No falsifier has been **executed and refuted** for BT-546 since 2026-04-11.

---

## §9 Honesty closing checklist

- [x] BT-546 status: **PARTIAL under Y8** (unchanged).
- [x] (A3) status: **UNPROVED** (unchanged). (A3') is a tautology at finite B.
- [x] Atlas grade movements: **NONE** in this audit.
- [x] Millennium 0/7 (preserved).
- [x] No new harness run; no new data introduced.
- [x] All cited values traceable to recorded sources (§1.1-§1.6 anchors).
- [x] UNKNOWN markers used where audit depth is exceeded (§5.3 verify_millennium-bsd aggregation; §6 tension 7 honesty surface area; §3.2 precise BT ranking).
- [x] English technical terms only.
- [x] Composite estimate (§3.2) is **non-canonical** -- canonical tool `nxs_002_composite.py --predict-er` not invoked.
- [x] Closure ceiling X-of-5: **2-3 / 5**, strong omega ceiling **not met**, plateau driven by (A3) + spectral ROI source.

-- end --
