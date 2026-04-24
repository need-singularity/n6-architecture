# DFS-24 — Yang-Mills Direction-Exploration Session (BT-543, Y5 PHYSICAL-NATURALNESS)

- Date: 2026-04-24
- Mode: research-only. Claims of "solved / proven / completed / confirmed" are **strictly prohibited** — results are draft candidates and exploration targets only.
- Honesty gate: `reports/millennium-dfs-status.md` keeps the 7-problem tally at 0/7 demonstration-candidates-unclosed.
- Target: BT-543 Yang-Mills mass gap (Y5 primary, Y3/Y9 secondary active).
- Input references:
  - `domains/physics/millennium-yang-mills/millennium-yang-mills.md` §X (SMASH/FREE duality)
  - `theory/breakthroughs/breakthrough-theorems-new.md` BT-543 cluster
  - `papers/lemmas-A3-A4-conditional-2026-04-15.md` (A3 strong is false; A4 numerics disagree by 4 orders of magnitude)
  - `papers/yang-mills-beta0-rewriting-2026-04-22.md` (MILL-PX-A3 grade [7])
  - `reports/dfs27-yangmills-20260415.md` (6-axis 238 PASS / 3 MISS)
  - `theory/roadmap-v2/phase-PX-PHYS-1-beta0-rigor.md` (routes A/B/C/D all below promotion threshold)

---

## 1. Re-statement of Current Tight Structure (~28 items; 8 load-bearing)

| # | Observation | Grade | Source |
|---|-------------|-------|--------|
| 1 | β₀(SU(3), n_f=6) = 11 − (2/3)·6 = 7 = σ(6) − sopfr(6) | [7] rewriting | Gross-Wilczek/Politzer 1973 |
| 2 | dim SU(3) = 8 = σ − τ (gluon count) | EXACT | standard |
| 3 | dim SU(4) = 15 = σ + n/φ, SO(6) ≅ SU(4)/Z₂ | EXACT | Lie algebra |
| 4 | B^E confinement exponent = 4 = τ(6), lattice 4.0±0.1 | EXACT (empirical) | Wilson, FLAG |
| 5 | Virasoro M(3,4) Ising c=1/2, p·q = σ | EXACT | BPZ 1984 |
| 6 | Virasoro M(4,5) c-numerator = 7 = β₀ (tricritical Ising) | TIGHT cross | DFS 27 Top 2 |
| 7 | β₃ (4-loop pure SU(3)) denominator = 6 = n | TIGHT | van Ritbergen 1997 |
| 8 | AGT M5-brane (2,0) worldvolume dim = 6 = n | EXACT | Alday-Gaiotto-Tachikawa 2010 |

**Weakness notes** (honest): A3 "second uniqueness" strong form is false via n=10 counterexample. A4 RH ⇒ Δ_YM inequality predicts Λ_QCD/1536 ≈ 0.13 MeV while observed mass gap is ~1.5 GeV — 4-order disagreement. Neither lemma candidate is currently a demonstration-candidate tool.

---

## 2. Next Tight Probes Proposed (2~3 items, no solved-claims)

### P1. **Narrow the "n_f = n = 6 forcing" of β₀ = σ − sopfr via anomaly cancellation (Y5+Y3)**

Current state: `phase-PX-PHYS-1` route A treats "SM generation count = 3 = n/φ" as an **observational fact** and only checks the arithmetic match. Route B (anomaly cancellation) takes "n_gen = 3 forcing" as a QFT theorem but lacks a direct bridge to n=6 arithmetic.

**Probe**: Re-examine whether the cancellation equations along the two axes Witten global anomaly (SU(2) π₄=Z/2) and Adler-Bell-Jackiw (local, U(1)_Y) reduce to a single-variable function of n_gen. In particular, in 4D spacetime check whether n_f = 2·n_gen = 6 decomposes as "τ=4 components × φ=2 doubling × generation=n/φ" into a rational identity.

**Decision criteria**: (i) If the anomaly equations force n_f ≡ 0 (mod φ·n/φ) = 0 (mod 6), T1 tight. (ii) If they only force mod 2 or mod 3, keep [7]. (iii) Falsifier: if any of SU(5)/SO(10)/E_6 GUT allows n_gen ≠ 3, reject.

**Artifacts**: `theory/predictions/verify_dfs24_ym_anomaly_ngen.hexa` + 1 staging signal (grade M9 ceiling).

### P2. **Virasoro M(3,4) ↔ YM mass-gap duality 2D→4D lift filter (Y5 primary)**

Current state: DFS 27 Top 2 notes that M(3,4)·M(4,5)·M(6,7) minimal-model family shares arithmetic with β₀=7. But the link between 4D YM and 2D minimal CFTs through AGT/M5-brane is **hypothetical**. domains §X.2 records the main link as "same SO(6) cover 4+2 vs 3+3 compactification branching".

**Probe**: Borrow from AGT/Nekrasov literature (Alday-Tachikawa 2010 §3) the conditional map for the N=(2,0) 6D theory on T² → 4D YM reduction, where modular parameter τ_mod limits to the M(3,4) Ising critical point (c=1/2) or to M(6,7) denominator p·q=σ·sopfr=42. Check numerically whether the central-charge rewriting boundary (c ∈ {1/2, 7/10, 4/5, 6/7}) and the β-function coefficient set {7, 17, J₂, ...} have intersection cardinality = n/φ.

**Decision criteria**: (i) If the four minimal-model c-numerators {6, 7, 4, 6} one-to-one match a YM β_k coefficient set {n, β₀, β₂ mod σ, n}, T2 cross-tight. (ii) If ≥2 fail to match, below threshold. (iii) Falsifier: if M(5,6) c=4/5 has no arithmetic path to β₁=102, reject.

**Artifacts**: 1 cross-table (BT-541 Riemann SLE_6 ∩ BT-543 M(3,4)) + `verify_dfs24_ym_cft4d_lift.hexa`.

### P3. **A4 unit correction — fix on the dimensionless axis Δ_YM/Λ_QCD = σ/τ=3 instead of Λ_QCD/1536 (Y9 HONEST-HARNESS)**

Current state: A4 lemma candidate Δ ≥ Λ_QCD/1536 mixes mass-scale and dimensionless values — numerically false. However the ratios in domains §X.2 — "m_0++/√σ_s ≈ 3.6, BCS 2Δ/k_BT_c ∈ [φ, σ/φ]" — survive on the **ratio-collinear** axis σ/τ=3.

**Probe**: Drop the RH dependency and re-state as a **pure ratio statement**:
> "In SU(3) YM pure gauge, do m_0++/√σ_s and the BCS ratio 2Δ/k_BT_c coexist in the interval [σ/τ − 1/φ, σ/τ + 1/φ] = [2.5, 3.5]?"

Measure interval occupancy on 3 lattice datasets: FLAG 2024 + BMW 2012 + Morningstar-Peardon 1999. RH assumption **not needed** — this is a simple lattice empirical inequality. Downgrade A4 from "conditionally false" to "unconditional empirical observation" and **restore honesty simultaneously**.

**Decision criteria**: (i) If all 3 lattices fall in [2.5, 3.5], keep T1 + A4-rev TIGHT. (ii) If 1 outlier, NEAR. (iii) If ≥2 outliers, reject → retire A4 entirely.

**Artifacts**: 1 `reports/lemma-A4-revised-ratio-only.md` + appendix update to `papers/lemmas-A3-A4-conditional-2026-04-15.md`.

---

## 3. Scope Boundary (strictly enforced)

- **Prohibited**: claim that Δ_YM > 0 is solved/proven, claim that RH is solved/proven, claim that β₀=7 is "derived from n=6".
- **Allowed**: rewriting, conditional lemmas, arithmetic coincidence, lattice empirical intervals.
- A3 strong/weak retries **prohibited** (none of P1~P3 in this session retries second uniqueness — n=10 counterexample is confirmed negative).
- Y9 gate: every probe carries either "COINCIDENCE NOT PROOF" or "CONDITIONAL" tag mandatorily.
- Alien-index ceiling: BT-543 currently 9 → P1/P2 success does **not** allow promotion to 10 (the demonstration-candidate remains open).

---

## 4. Execution Priority + Cost

| Priority | Probe | Expected cost | Expected outcome |
|----------|-------|---------------|------------------|
| H | P3 (A4 ratio fix) | S | TIGHT recovery or full retirement — **honesty restoration is top priority** |
| M | P1 (anomaly n_gen forcing) | M | [7]->[8] promotion ceiling |
| L | P2 (M(3,4) 2D->4D lift) | L | Cross-tight addition, not the main path |

**Recommended order**: P3 → P1 → P2. P3 is a zero-risk honesty correction of the existing false lemma. P1 revisits routes A/B in roadmap phase-PX-PHYS-1 with sufficient literature. P2 has high AGT-literature access cost.

---

## 5. Honest Closure

- This session performs **direction proposals only**. No probe has been executed yet.
- 7-problem demonstration-candidates: **0/7 maintained**. BT-543 Clay demonstration candidate: **open maintained**.
- β₀ = σ − sopfr = 7 is an **arithmetic coincidence**, not a mass-gap demonstration tool.
- This document is archived in `reports/sessions/` under "DFS 24th YM direction exploration" with no separate atlas edits.

**Next session**: execute P3 (measure ratios on 3 lattice datasets, generate `reports/lemma-A4-revised-ratio-only.md`).
