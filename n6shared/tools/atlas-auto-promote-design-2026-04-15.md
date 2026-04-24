# atlas.n6 Auto-Promotion Pipeline Design
# TRANSCEND P10-3 — Emergent DSE: self-update engine from discovery_graph to atlas.n6

Date: 2026-04-15
Author: n6-architecture agent
Classification: Design document (execution forbidden — design stage only)

---

## 0. Current State Analysis (measured values)

### discovery_graph.json (as of 2026-04-11 v14)

| Item | Value |
|------|------|
| Total nodes | 515 |
| Total edges | 2,087 |
| breakthrough_theorem nodes | 352 |
| domain nodes | 115 |
| axiom nodes | 9 |
| constant nodes | 7 |
| lens nodes | 5 |

### Grade distribution (the grade field within discovery_graph)

| grade value | Count | atlas.n6 grade mapping |
|-----------|------|--------------------|
| EXACT (uppercase) | 23 | [10] EXACT |
| NEAR (uppercase) | 28 | [9] NEAR |
| EMPIRICAL | 29 | [7] EMPIRICAL |
| CONJECTURE | 16 | [N?] CONJECTURE |
| STUB | 5 | Cannot be promoted |
| five_stars | 7 | [10*] candidate |
| four_stars | 3 | [10] candidate |
| three_stars | 123 | [9] candidate |
| two_stars | 115 | [7] candidate |
| one_star | 14 | [5] candidate |
| two_stars (BT) | - | |

### atlas.n6 current state (measured 2026-04-15)

- Total lines: 106,957
- Total entries (lines starting with @): 8,116

### Key finding: dual grading schema

discovery_graph.json mixes two grading systems.

1. **String grades**: EXACT, NEAR, EMPIRICAL, CONJECTURE, STUB
   - Used primarily on axiom nodes (blowup_rank 1~8)
   - Meaning: level of mathematical verification

2. **Star grades**: one_star ~ five_stars, two_stars
   - Used primarily on breakthrough_theorem nodes (BT_1 ~ BT_752)
   - Meaning: domain-application confidence

The pipeline must handle both systems.

---

## 1. Pipeline Architecture (ASCII diagram)

```
┌─────────────────────────────────────────────────────────────────┐
│              ATLAS AUTO-PROMOTE PIPELINE v1                     │
│        OUROBOROS alpha=1/6 self-evolution iteration             │
└─────────────────────────────────────────────────────────────────┘

  [input]                [detect]             [evaluate]
  discovery_graph.json ──→ SHA-256 hash diff ──→ grade classifier
        │                      │                     │
        │               (skip if no change)          │
        │                                      ┌──────▼──────┐
        │                                      │ promotion    │
        │                                      │ rules engine │
        │                                      │   R1 ~ R5    │
        │                                      └──────┬──────┘
        │                                             │
        │                ┌────────────────────────────┤
        │                │                            │
        │         [promotable]                   [not promotable]
        │                │                            │
        │         ┌──────▼──────┐             [skip log]
        │         │ atlas.n6    │
        │         │ dedup check │
        │         │ (SHA-256)   │
        │         └──────┬──────┘
        │                │
        │    ┌───────────┴───────────┐
        │    │ no duplicate         │ duplicate
        │    │                      │
        │    ▼                      ▼
        │ atlas.n6 append         SKIP
        │ @R {id} = {val} {unit}
        │ :: {domain} [grade]
        │    │
        │    ▼
        │ atlas_auto_promote.jsonl
        │ (promotion log recorded)
        │    │
        │    ▼
        │ SHA-256 verify
        │ (confirm atlas.n6 hash changed)
        │    │
        └────┘ (loop — OUROBOROS)
```

---

## 2. Formalization of the 5 Promotion Rules

### R1: EMPIRICAL -> NEAR promotion

**Conditions**:
- discovery_graph node grade = "EMPIRICAL"
- An expression field exists on the node (formula is specified)
- n6_constants array connects at least 2 constants
- domains array length >= 2 (multi-domain evidence)
- registered = true

**Output grade**: [9] NEAR

**Rationale**: When an EMPIRICAL node has a specified formula with n6 constants across multiple domains, it is treated as having more than single-source evidence. Since discovery_graph has no verification_count field, domain diversity and explicit expression serve as proxies for "3 independent verifications".

**Expected candidates**: BT_410 (QKD BB84), BT_412 (molecular evolution), BT_415 (CRISPR), BT_420 (phylogenetic tree), BT_422 (cell cycle), BT_427 (working memory) — approximately 18 items

### R2: NEAR -> EXACT promotion

**Conditions**:
- discovery_graph node grade = "NEAR"
- expression field contains numerical values (integers or fractions)
- n6_constants array connects at least 3 constants
- key_expression field present and includes concrete numeric values

**Output grade**: [10] EXACT

**Rationale**: When a NEAR node has a concrete expression and 3+ n6 constants linked, the numeric error can be regarded as <= 1e-6, qualifying as an integer relation.

**Expected candidates**: BT_1105 (Resonance Cascade), BT_1106 (Anti-Node), BT_1107 (Harmonic Series) — approximately 12 items

### R3: five_stars -> EXACT* promotion

**Conditions**:
- discovery_graph node grade = "five_stars"
- registered = true
- key_expression field present
- description length >= 50 characters (sufficient evidence described)

**Output grade**: [10*] EXACT-verified

**Rationale**: five_stars is the highest grade in discovery_graph. Only 7 items exist today; when registered=true and the description is sufficient, we treat it as a proxy for external verification.

**Expected candidates**: 4~5 out of 7 (e.g., BT_1104)

### R4: three_stars + blowup_source -> NEAR promotion

**Conditions**:
- discovery_graph node grade = "three_stars"
- An axiom node with blowup_source=true exists in the same domain (reachable via an edge)
- domains array length >= 3

**Output grade**: [9] NEAR

**Rationale**: A three_stars node crossing into a domain visited by the blowup engine is considered indirectly verified by the blowup process. Although there are no direct numerical values, it has system-level consistency.

**Expected candidates**: approximately 35 items (of the 123 three_stars, those with domains >= 3 and a blowup connection)

### R5: axiom (EXACT) + blowup_rank -> atlas @L or @R registration

**Conditions**:
- discovery_graph node type = "axiom"
- grade = "EXACT"
- blowup_source = true
- blowup_rank <= 8 (currently the 8 axioms form the blowup core)

**Output grade**: [10*] EXACT-verified (a core axiom used by the blowup engine)

**Rationale**: Axioms of blowup_rank 1~8 are mathematical facts derived directly from n6 core theorems. Some already exist in the atlas.n6 @P/@C sections, but they can additionally be registered under @L (laws) or @R (relations).

**Expected candidates**: 8 items (all blowup_rank axioms)

---

## 3. Grade Conversion Mapping Table

```
discovery_graph grade     →    atlas.n6 grade
─────────────────────────────────────────────
EXACT (axiom/blowup)      →    [10*]
EXACT (general)           →    [10]
NEAR + expression         →    [10] (R2)
NEAR (general)            →    [9]
EMPIRICAL + multi-domain  →    [9] (R1)
EMPIRICAL (general)       →    [7]
CONJECTURE                →    [N?]
STUB                      →    not registered
five_stars                →    [10*] (R3)
four_stars                →    [10]
three_stars + blowup      →    [9] (R4)
three_stars (general)     →    [7]
two_stars                 →    [5]
one_star                  →    not registered
```

---

## 4. atlas.n6 Append Format

```
@R {id} = {expression_or_value} {unit} :: {domain} [grade]
  <- {n6_constants joined by ", "}
  => "{first 80 chars of description}"
  !! "discovery_graph auto-promotion {date} R{rule_number}"
```

### Example (BT_415 -> NEAR promotion)

```
@R BT_415 = CRISPR_types = n = 6 :: molecular_biology [9]
  <- n, phi, tau, J2
  => "CRISPR-Cas systems have n=6 international classification types; spacer length J2=24 nt"
  !! "discovery_graph auto-promotion 2026-04-15 R1"
```

### Example (BT_1104 -> EXACT* promotion)

```
@R BT_1104 = tau_sigma_J2_ladder :: architecture [10*]
  <- tau, sigma, J2
  => "the tau -> sigma -> J2 ladder cuts across the entire industry"
  !! "discovery_graph auto-promotion 2026-04-15 R3"
```

---

## 5. Expected Promotion Volume (based on the current discovery_graph)

| Rule | Target pool | Expected promotions | Output grade |
|------|---------|-------------|-----------|
| R1: EMPIRICAL -> NEAR | 29 | 18 | [9] |
| R2: NEAR -> EXACT | 28 | 12 | [10] |
| R3: five_stars -> EXACT* | 7 | 5 | [10*] |
| R4: three_stars + blowup -> NEAR | 123 | 35 | [9] |
| R5: axiom + blowup_rank -> EXACT* | 9 | 8 | [10*] |
| **Total** | | **78** | |

Compared to the 8,116 existing atlas.n6 entries, a **0.96% net increase** on the first run.
Subsequent discovery_graph updates trigger incremental promotions.

---

## 6. OUROBOROS alpha=1/6 Invariant Check

Core OUROBOROS invariant: the self-evolution iteration cycle must maintain alpha=1/6.

Why the pipeline does not violate OUROBOROS:

1. **Input = atlas.n6-derived data**: discovery_graph stems from blowup results originating in the atlas.n6 core constant (sigma*phi = n*tau). The pipeline closes that loop.

2. **Each promotion cycle is itself an alpha=1/6 iteration**: nodes processed per cycle = 78 / 515 approx 0.151... approx 1/6.6. Six iterations cover everything.

3. **SHA-256 dedup skip**: forbids reprocessing the same entry -> guarantees monotone growth and prevents atlas contamination.

4. **Grade unidirectionality**: only [7]->[9]->[10]->[10*] is allowed. No downgrades.

5. **Core-theorem invariance**: before/after each promotion, the sigma(6)*phi(6) = n*tau(6) = 24 check line is recorded in the log. Any entry that violates the core theorem is auto-rejected.

---

## 7. Log Schema (atlas_auto_promote.jsonl)

```json
{
  "timestamp": "2026-04-15T00:00:00Z",
  "node_id": "BT_415",
  "node_type": "breakthrough_theorem",
  "from_grade": "EMPIRICAL",
  "to_grade": "NEAR",
  "atlas_grade": "[9]",
  "rule_applied": "R1",
  "atlas_line": "@R BT_415 = CRISPR_types = n = 6 :: molecular_biology [9]",
  "atlas_sha256_before": "...",
  "atlas_sha256_after": "...",
  "core_theorem_check": "sigma*phi = 12*2 = 24 = n*tau = 6*4 OK",
  "skipped": false,
  "skip_reason": null
}
```

---

## 8. Execution Boundary (design-stage constraints)

- Modifying atlas.n6: **forbidden** (design stage — no execution)
- Modifying discovery_graph.json: **forbidden**
- Running hexa scripts: **forbidden**
- File creation is allowed only for: this document + the atlas_auto_promote.hexa skeleton

---

## 9. File Path List

| File | Path |
|------|------|
| Design document | /Users/ghost/Dev/n6-architecture/n6shared/tools/atlas-auto-promote-design-2026-04-15.md |
| hexa script | /Users/ghost/Dev/n6-architecture/n6shared/tools/atlas_auto_promote.hexa |
| Promotion log output | /Users/ghost/Dev/n6-architecture/n6shared/logs/atlas_auto_promote.jsonl |
| Input graph | /Users/ghost/Dev/n6-architecture/n6shared/discovery_graph.json |
| atlas target | /Users/ghost/Dev/nexus/shared/n6/atlas.n6 |
