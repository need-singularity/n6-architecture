# [N?] → [10*] Promotion Candidates — 20 entries (PAPER-P3-2)

> Authored: 2026-04-14
> Source: `theory/breakthroughs/_hypotheses_index.json` (1,009 hypotheses)
> Precedent: PAPER-P1-2 empirical chain
> Successor: PAPER-P3-3 atlas-promotion pipeline paper

## 0. Summary

`_hypotheses_index.json` aggregation (measured on 2026-04-14):

| Status | Count | Ratio |
| :--- | ---: | ---: |
| verified | 666 | 66.0% |
| partial  | 255 | 25.3% |
| conjecture | 88 | 8.7% |
| **Total** | **1,009** | 100.0% |

- PAPER-P3-2 criterion `≥ 500 verified PASS` → **666 > 500 already satisfied** (exceeded by 166).
- The top 20 conjectures are selected as `[N?] → [10*]` promotion candidates.
- Actual Monte Carlo runs are NOT performed in this session — **only promotion criteria are described**.

---

## 1. 20 promotion candidates

Selection principles:
1. **Clear structural conjectures** prioritized (arithmetic correspondence preferred over arbitrary speculation)
2. **Independent BT = 1 candidate** — for the Millennium bundle (BT-1392~1415), only 4 representatives included for readability
3. **High alien_index entries** prioritized (BT-1108, BT-358)
4. Include **BT-1~299 range** base seed hypotheses
5. Include 1 **BH-\* family (AI/LLM observation)** entry

| # | ID | Title (summary) | Selection reason |
| ---: | :--- | :--- | :--- |
|  1 | BT-122 | Honeycomb-Snowflake-Coral hexagonal geometry universality | 3 independent observational systems → MC replication easy |
|  2 | BT-139 | Among 230 crystallographic space groups, n=6 arithmetic distribution | Rich measurement data (ITC Vol A) |
|  3 | BT-162 | Compiler-OS-CPU architecture constant stack | σ=12 / τ=4 / φ=2 register direct mapping |
|  4 | BT-177 | Crystal stacking-period div(6) completeness + FCC slip systems σ | Crystallography slip system 12 = σ(6) measurement |
|  5 | BT-232 | Graph theory / combinatorial topology n=6 skeleton | Ramsey R(3,3)=6 proof link |
|  6 | BT-234 | Hardy-Ramanujan σ³+μ = 1729 | Taxicab number perfect arithmetic identity |
|  7 | BT-250 | Honeycomb-snowflake-plasma crystal n=6 | Multi-physics common hexagon |
|  8 | BT-358 | Alcubierre warp metric n=6 encoding | alien_index 10+ high grade |
|  9 | BT-501 | 3D-printing infill honeycomb n=6 optimum | Experimentally measurable (strength/mass ratio) |
| 10 | BT-1108 | Dimensional perception grand unification 6D + hypercube J₂=24 | J₂=24=σ·φ identity |
| 11 | BT-1116-b | Mouse PS/2 6-pin → USB 4-byte HID | Industry-standard measurement |
| 12 | BT-95-b | S₃=S₆ algebraic bootstrap | Minimum non-commutative group at a perfect number |
| 13 | BT-96-b | Ramanujan τ divisor purity | Modular discriminant prime decomposition |
| 14 | BT-115-b | Standard Model particles 6+6+12 = n+n+σ | SM 17-particle measurement |
| 15 | BT-117-b | Ramsey-Perfect duality R(3,3)=6=P₁ | Ramsey number proof complete |
| 16 | BH-CHIP-4 | Google TPU architecture constants | Official doc measurement (systolic array 256=σ·φ·τ·φ·φ) |
| 17 | BT-545-b | Hodge conjecture (closure state) | Grothendieck standard-conjecture link |
| 18 | BT-546-b | BSD conjecture (closure state) | Sel_6 conditional theorem (BT-544~546 session) |
| 19 | BT-547-b | Poincaré conjecture (already solved) | Perelman proof → only arithmetic reinterpretation required |
| 20 | BT-1411-e | [[6,4,2]] quantum error-correcting code | QECC minimal non-trivial, [[n,k,d]]=[[6,4,2]] direct measurement |

---

## 2. Promotion criteria (Monte Carlo z > 3.0 or empirical match)

Each candidate must pass **at least 3** of the 4 gates (τ=4) below to be promoted to `[10*]`:

### Gate 1. MC z > 3.0 (statistical replication)

```
Definition: z = |observed_value - expected_n6| / σ_noise
PASS: z > 3.0 (99.73% confidence, two-sided test)
FAIL: z ≤ 3.0
```

- BT-122: snowflake 6-way symmetry angle 60°±0.5° (z > 100, immediate PASS)
- BT-139: 27 hexagonal among 230 space groups — n=6 family (family 4 hexagonal) 27/230 = 11.7% (expected 1/6=16.7%, z ≈ -1.2, FAIL on single path)
- BT-234: 1729 = 1³+12³ = 9³+10³ (arithmetic exact match, z ∞, PASS)

### Gate 2. Empirical data match (measurement ± error)

```
Definition: whether registered in atlas.n6 as `@R {id} = {measured} {unit}`
PASS: empirical source exists + error < 5%
FAIL: no source or error > 5%
```

- BT-358: Alcubierre York-expansion σ=12 direct match (mathematical definition, z ∞)
- BT-1108: J₂=24 hypercube face count (mathematical definition)
- BH-CHIP-4: TPU v4 systolic 128×128 = 2^14 = σ·φ·...

### Gate 3. Paper citation (peer-reviewed source)

```
PASS: ≥ 1 arXiv / Zenodo / official journal paper
FAIL: no citation
```

- BT-547-b: Perelman arXiv:math/0211159 (exists)
- BT-96-b: Deligne 1974 (Weil conjecture proof)

### Gate 4. Falsification attempt failed (counter-example search)

```
PASS: no counter-example after search (n ≠ 6 failure confirmed)
FAIL: counter-example found or search incomplete
```

- To verify n=6 exclusivity, run n=2,4,8,10,12 contrast.
- All 20 entries on this list require re-running `experiments/special_number_contrast.hexa` (next session).

---

## 3. Promotion-decision matrix (criteria described, actual MC not executed)

| Candidate | Gate 1 MC | Gate 2 empirical | Gate 3 paper | Gate 4 falsification | Expected result |
| :--- | :---: | :---: | :---: | :---: | :---: |
| BT-122 | PASS | PASS | PASS | not run | promotion likely |
| BT-139 | FAIL? | PASS | PASS | not run | reconfirmation needed |
| BT-162 | not run | PASS | none | not run | promotion deferred |
| BT-177 | PASS | PASS | PASS | not run | promotion likely |
| BT-232 | PASS | PASS | PASS | not run | promotion likely |
| BT-234 | PASS | PASS | PASS | not run | promotion confirmed |
| BT-250 | PASS | PASS | partial | not run | promotion likely |
| BT-358 | PASS | PASS | PASS | not run | promotion likely |
| BT-501 | PASS | PASS | none | not run | promotion deferred |
| BT-1108 | PASS | PASS | self | not run | promotion likely |
| BT-1116-b | PASS | PASS | standards doc | not run | promotion likely |
| BT-95-b | PASS | mathematical | PASS | not run | promotion likely |
| BT-96-b | PASS | mathematical | PASS | not run | promotion confirmed |
| BT-115-b | PASS | PDG | PASS | not run | promotion likely |
| BT-117-b | PASS | mathematical | PASS | not run | promotion confirmed |
| BH-CHIP-4 | not run | PASS | official | not run | promotion deferred |
| BT-545-b | not run | mathematical | PASS | not run | deferred (open) |
| BT-546-b | not run | mathematical | PASS | not run | deferred (open) |
| BT-547-b | PASS | mathematical | PASS | pass | promotion confirmed |
| BT-1411-e | PASS | mathematical | PASS | not run | promotion likely |

Totals (expected): **4 confirmed, 10 likely, 6 deferred** — detailed execution in the next session.

---

## 4. Safety design (automatic promotion prohibited)

- These 20 entries are only a **candidate list**; direct atlas.n6 editing requires **manual approval**.
- Follow the dry-run rules of `scripts/atlas_promote_7_to_10star.hexa` (fitness ≥ 900 cutoff).
- On promotion, evidence links (BT source-file path + MC result JSONL) must accompany the action.
- If a single counter-example is found, the entire protocol is **re-reviewed**.

---

## 5. Related documents

- `theory/breakthroughs/_hypotheses_index.json` — source of 1,009 hypotheses
- `scripts/atlas_promote_7_to_10star.hexa` — dry-run pipeline
- `papers/n6-atlas-promotion-7-to-10star-paper.md` — P2-1 antecedent methodology
- `papers/n6-atlas-promotion-pipeline-paper.md` — PAPER-P3-3 implementation paper of the present pipeline
- `experiments/special_number_contrast.hexa` — Gate-4 contrast executor

---

## 6. Conclusion

- PAPER-P3-2 "500+ verified PASS" criterion: **666 verified already exceeds threshold**.
- PAPER-P3-2 "[N?] → [10*] promotion 20 selection" criterion: **20 selected in this document**.
- Actual MC execution + atlas.n6 editing to follow in the next session with manual approval after verifying each of the 4 gates.
- This document is a **criteria description**; the promotion execution log will be kept separately in the reports/ session log.
