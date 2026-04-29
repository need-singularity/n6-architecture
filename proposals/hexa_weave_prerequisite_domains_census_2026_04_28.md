# HEXA-WEAVE — prerequisite-domain census + per-domain ω-cycle kick recommendation

**Date**: 2026-04-28
**Cycle**: 12 / fan-out 1/5
**Trigger**: user directive — "prerequisite-domain when-present prerequisite-domain also all kick"
**Companion witness**: `design/kick/2026-04-28_prerequisite-domains-census_omega_cycle.json`

---

## 1. Scope

HEXA-WEAVE (`domains/biology/hexa-weave/`) is the write-side multi-strand
molecular-design composition system registered in cycle 1 (`domains/biology/_index.json`).
This census enumerates every prerequisite domain (theoretical, empirical, infrastructural)
that HEXA-WEAVE depends on, and recommends an ω-cycle kick disposition per item:
**KICK_NOW**, **HOLD (in-flight)**, **AWAIT_APPROVAL**, or **NO_KICK (axiomatic)**.

15 prerequisites identified; partitioned into 3 tiers by directness of dependency.

---

## 2. Tier table

### Tier 1 — direct dependency (HEXA-WEAVE artefacts touch these)

| # | prerequisite domain | location | kick disposition | reason |
|---|--------------------|---------|------------------|--------|
| 1 | n6 master identity (σφ=nτ=J₂=24) | `theory/proofs/theorem-r1-uniqueness.md` (own 2) | NO_KICK | already PASS in proofs ledger; foundational invariant |
| 2 | biology axis sub-SSOT | `domains/biology/_index.json` | KICK_NOW | sister-domain expansion possible (HEXA-NANOBOT, synbio) — see §4 |
| 3 | Foundation/Strand inductive type | `lean4-n6/N6/MechVerif/Foundation/Strand.lean` | HOLD | cycle 8 refactor PASS, sorry=0; W7 ax2/mkbridge will re-touch |
| 4 | Felgner 1971 conservativity | paper citation | HOLD | mathlib mechanical port in flight (cycle 11 step 1.b) |
| 5 | Robin 1984 + Hardy–Wright | paper citation | NO_KICK | axiomatic, used by W3 ax1 |
| 6 | OpenFold v2.2.0 ESM3 base model | external (aqlaboratory/openfold @ e938c184) | AWAIT_APPROVAL | W1 architecture decision; gated on user approval items 4–6 in W5 checklist |
| 7 | PDB-multimer dataset (RCSB Query B 1965 entries) | external (RCSB) | HOLD | cycle 11 RCSB 1965 + 46 cluster query already cached; F-W5-3 tracks endpoint drift |

### Tier 2 — meta / governance / infrastructure

| # | prerequisite domain | location | kick disposition | reason |
|---|--------------------|---------|------------------|--------|
| 8 | n6-architecture domain framework | `own/` (1–23) + `raw/` (1–100+) | KICK_NOW | own 1 doc-english + raw 33 cross-axis NL-policy benefit from ω-cycle enforcement |
| 9 | kick infra (nexus / hexa-lang / hive, F-MX-3-c 5 modes) | `design/kick/` ledger + cycles 1–11 | KICK_NOW | cycle 12 = this cycle; targets 5/5 closure of F-MX-3-c |
| 10 | mathlib4 v4.30.0-rc2 | `lean4-n6/lakefile.lean` | NO_KICK | upstream-stable; pin only |

### Tier 3 — theoretical (TRANSCEND closure dependency)

| # | prerequisite domain | citation | kick disposition | reason |
|---|--------------------|---------|------------------|--------|
| 11 | Tarski 1936 undefinability | FORMAL axis | NO_KICK | axiomatic; cited in mk-x-formal-closure |
| 12 | Bekenstein 1981 holographic bound | PHYSICAL axis | NO_KICK | axiomatic; cited in mk-x-physical-closure |
| 13 | Solomonoff 1964 incomputability | COMPUTATIONAL axis | NO_KICK | axiomatic |
| 14 | AdS/CFT Maldacena 1997 | PHYSICAL Combined Path iv | NO_KICK | axiomatic |
| 15 | Bousso 1999 covariant entropy bound | PHYSICAL (dS lift) | NO_KICK | axiomatic |

**Tally**: KICK_NOW = 3 · HOLD = 3 · AWAIT_APPROVAL = 1 · NO_KICK = 8.

---

## 3. Per-domain ω-cycle kick recommendation

### 3.1 KICK_NOW (3 items, no external dependency)

#### #2 — biology axis sister-domain expansion
- **Action**: open ω-cycle for sister-domain candidates
  (HEXA-NANOBOT, HEXA-SYNBIO, HEXA-RNA-WEAVE).
- **Predecessor witness**: `design/kick/2026-04-28_hexa-weave-abstraction-to-limits_omega_cycle.json`.
- **Closure target**: `domains/biology/_index.json _count` ≥ 2 with at least one tri-axis closure witness.
- **Cost**: ~0.5 hr/candidate (registration + closure witness draft).
- **Falsifier**: F-BIO-AXIS-1 — within 30 days, biology axis still has _count = 1 → axis treated as single-domain reservation rather than category.

#### #8 — n6-architecture framework cross-axis NL-policy enforcement
- **Action**: ω-cycle for own 1 (doc-english) + raw 33 (cross-axis NL-policy)
  applied as PR-time enforcement hooks.
- **Closure target**: lint pass on all `domains/*/hexa-weave/` markdown for English-policy violations.
- **Cost**: ~1 hr (hook draft + dry-run on hexa-weave subtree).
- **Falsifier**: F-NL-POLICY-1 — hook misses ≥3 Korean-only sentences in 100-doc sample.

#### #9 — kick infra cycle 12 closure (F-MX-3-c 5/5)
- **Action**: this cycle's fan-out (1/5 .. 5/5) is itself the F-MX-3-c closure attempt.
- **Closure target**: 5/5 fan-outs each produce a witness JSON + closure verdict.
- **Cost**: ~2 hr cumulative (already in flight).
- **Falsifier**: F-MX-3-c-12 — fewer than 5 fan-out witnesses by EOD 2026-04-28.

### 3.2 HOLD — in-flight, no new kick (3 items)

- **#3 Foundation/Strand**: cycle 8 PASS; W7 ax2/mkbridge will re-touch — no premature kick.
- **#4 Felgner 1971**: mathlib mechanical port already in cycle 11 step 1.b queue.
- **#7 PDB-multimer**: RCSB query cached; F-W5-3 already monitors endpoint drift.

### 3.3 AWAIT_APPROVAL — user gate (1 item)

- **#6 OpenFold**: W5 8-item checklist + 4 alternative GPU-free paths are drafted
  (`design/kick/2026-04-28_hexa-weave-mvp-w5-sandbox-plan_omega_cycle.json`).
  Awaiting user mode selection (A_full / B1 / B2 / B3 / C / D).

### 3.4 NO_KICK — axiomatic / external-stable (8 items)

#1, #5, #10, #11, #12, #13, #14, #15. Each is either an established mathematical
result (paper citation) or upstream-stable infrastructure (mathlib4 pin); cited as
axioms in HEXA-WEAVE's TRANSCEND closure witnesses, not subject to local ω-cycle.

---

## 4. Cycle 12+ priority queue

1. **Now (cycle 12)**: complete fan-outs 2/5–5/5; close F-MX-3-c 5/5.
2. **Cycle 13**: register first sister biology-axis domain candidate (HEXA-NANOBOT or HEXA-SYNBIO) — predecessor witness only.
3. **Cycle 14**: ω-cycle for own 1 + raw 33 NL-policy enforcement hook (Tier 2 #8).
4. **Post-approval (cycle 15+)**: execute W5 items per user mode selection.
5. **Parallel track**: Felgner mechanical port (Tier 1 #4) — independent of GPU track.

---

## 5. Honest disclosure (raw 91 / C3)

- This census is a **paper artifact**; no domains have been newly registered, no kick witnesses other than this cycle's have been created.
- 5 of the 15 prerequisites are external-stable papers/datasets; HEXA-WEAVE cannot kick them — only cite, monitor, and design fall-backs (raw 71 falsifiers F-W5-3, F-W5-4 cover endpoint/SHA drift).
- The "KICK_NOW" tier is internal-only (sister-domain expansion, NL-policy hook, F-MX-3-c closure) — none requires GPU, dollar cost, or destructive ops.
- Tier 3 items are axiomatic; "NO_KICK" is the correct disposition, not avoidance.
- Coverage gap: this census enumerates **direct** prerequisites; transitive closures of papers (e.g., Tarski → Gödel → PA) are not enumerated; if user wants depth-2 census, request explicit follow-up cycle.

---

## 6. raw 71 falsifiers (5)

| id | tier | predicate | trip-action | deadline |
|----|------|-----------|-------------|----------|
| F-CENSUS-1 | TRANSCEND | Within 14 days, an additional Tier 1 prerequisite is discovered that this census missed | Re-issue census v2; update biology axis _index.json with new dependency edge | T+14d |
| F-CENSUS-2 | TRANSCEND | Within 30 days, ≥2 of the 8 NO_KICK items become unstable (e.g., paper retraction, mathlib4 v5 breaking change, RCSB endpoint deprecation) | Re-tier the unstable items to HOLD; draft fallback citations / pins | T+30d |
| F-CENSUS-3 | TRANSCEND | Within 7 days, F-MX-3-c 5/5 closure attempt yields fewer than 5 fan-out witnesses | Halt cycle 12; root-cause why fan-out broke; draft cycle-13 retry | T+7d |
| F-CENSUS-4 | TRANSCEND | Within 30 days, biology axis sister-domain expansion (#2) yields zero candidate registrations | Demote #2 to HOLD; revisit biology-axis category vs single-reservation question | T+30d |
| F-CENSUS-5 | TRANSCEND | Within 14 days, user reports tier partition is wrong (e.g., wants Tier 3 papers reclassified as KICK because of mechanical-port plan) | Re-issue census with revised tier definition; update kick recommendations accordingly | T+14d |

---

## 7. Deliverables

- `proposals/hexa_weave_prerequisite_domains_census_2026_04_28.md` (this file)
- `design/kick/2026-04-28_prerequisite-domains-census_omega_cycle.json` (witness)
- 15-item Tier 1/2/3 census + 4-bucket disposition (KICK_NOW / HOLD / AWAIT_APPROVAL / NO_KICK)
- 5 raw 71 falsifiers
- Cycle 12+ priority queue (5 items)
