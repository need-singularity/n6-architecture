# atlas.signals.n6 — 3-repo cross-repo signal SSOT specification v0.3 (draft)

> Version: v0.3-draft (2026-04-15)
> Target: nexus + n6-architecture + anima resonance signal store
> Status: DRAFT — proposed v0.2 → v0.3 evolution
> SSOT location: `$NEXUS/shared/n6/atlas.signals.n6`
> Predecessor: `n6shared/specs/atlas.signals.n6.spec.md` v0.2

---

## 0. v0.3 in brief

Rules tightened after v0.2 experience: 350+ signals absorbed, 18+ CROSS promotions, 46+ resonance_n6 backfills.

Key v0.3 changes:
1. **resonance_n6 mandatory condition** — for grade [M9] and above, `resonance_n6` may not be null
2. **CROSS tag conditions clarified** — requires `cross_repo` array ≥ 1 AND `witness` ≥ 2 AND confirmed other-repo tag
3. **witness increment rules** — simhash Hamming ≤ 16 bits (0.125) + 3+ overlapping keywords
4. **New [M10**] grade** — signals reproduced in 4+ repos (future OEIS/arXiv/external-DB admission)
5. **staging → SSOT merge cadence** — explicit merge commit at session end

---

## 1. Change details

### 1.1 resonance_n6 mandatory (NEW)

**v0.2**: resonance_n6 optional, null allowed.

**v0.3**: grade-based mandatory policy:

| Grade | resonance_n6 policy |
|------|------------------|
| `[M10*]` | **required** — null forbidden, formula mandatory |
| `[M10]` | **required** — null forbidden, minimal structural interpretation allowed |
| `[M9]` | **required** — null forbidden (structural interpretation allowed) |
| `[M7!]` | recommended — null allowed but promotion blocked |
| `[M7]` | optional |
| `[M?]` | optional |
| `[MN]` | null recommended (the phenomenon itself is NULL) |

**Rationale**: L9 analysis shows signals with resonance_n6 have a 77.8% M10+ promotion rate vs 47.5% average for null. The 30pp gap → resonance entries predict promotion.

**Promotion block**: an automatic resonance_n6 re-review trigger fires on M9→M10 promotion (numeric agreement ε < 1% or structural mapping required).

### 1.2 CROSS tag conditions clarified (STRICTER)

**v0.2**: when `repo_tags = [CROSS, X, Y]`, the `cross_repo` field link was "required" (loose).

**v0.3**: all 3 conditions must hold to earn the CROSS tag:

1. `cross_repo` array contains **at least 1 really-existing SIG-id** (self-reference disallowed)
2. `witness ≥ 2` (2+ independent observations)
3. The linked signal's `repo_tags` include a **repo different** from this signal's
   - e.g. self = [NX, CROSS], target must be [N6] or [AN]
   - [NX] ↔ [NX] links cannot earn CROSS (intra-repo reconfirmation)

**Contamination guard**: self-reference or intra-repo links cannot promote to CROSS.

**Retroactive audit**: on v0.3 adoption, audit all 56 current CROSS signals and strip the CROSS tag from violators.

### 1.3 witness-increment rule formalized (FORMAL)

**v0.2**: "simhash-matching basis required" (prose).

**v0.3**: witness increments require one of:

**Condition A — simhash numeric match**:
- simhash Hamming distance ≤ 16 bits (within 12.5% on a 128-bit hash)
- or similarity ≥ 0.875

**Condition B — keyword semantic match**:
- 3+ overlapping core keywords in `statement + context`
- 1+ numeric match (ε < 1%)

Each witness increment must append the match basis to `refs`.

### 1.4 New grade [M10**] (FUTURE)

**Condition**: reproduction in 4+ repos (beyond the 3-repo SSOT into external DBs)
- e.g. OEIS registration, arXiv publication, external dataset citation
- `cross_repo` array ≥ 2 + `refs` with external DOI / OEIS-id

**Current**: 0 signals qualify (preparation phase).

**Prediction**: SIG-META-001 (σφ=nτ uniqueness) may promote to [M10**] upon OEIS A-xxxxxx + arXiv submission.

### 1.5 staging → SSOT merge cadence (EXPLICIT)

**v0.2**: the Group D staging section was a temporary area inside the document.

**v0.3**: explicit staging-merge cadence:

- **During a session**: append into the staging section as needed
- **At session end**: explicit merge commit required
  - staging signals in `@S SIG-GROUP-XNN` form → re-id'd into main-section SIG-XX-NNN
  - staging section is emptied (or fully removed)
- **Weekly cleanup**: recompact staging every Monday + output audit report

---

## 2. Migration plan

### 2.1 v0.2 → v0.3 transition steps

| Step | Task | Owner |
|------|------|------|
| 1 | Agree on v0.3 draft (this document) | current session |
| 2 | Complete resonance_n6 backfill (M9+ mandatory) | next session |
| 3 | Full audit of CROSS tags + retract violators | next session |
| 4 | Re-verify the last 7 days of witness-increment logs | next session |
| 5 | First staging → SSOT merge cycle | next session |
| 6 | v0.3 approval + replacement of main spec file | post-agreement |

### 2.2 Compatibility

- v0.2 file format = v0.3 file format (schema unchanged)
- Only the rules are tightened — no file rewrite required
- No new fields (existing `witness`, `resonance_n6`, `cross_repo` strengthened)

---

## 3. Impact analysis (on v0.3 adoption)

### 3.1 Immediate impact

- Approximately 10 [M9] signals with resonance_n6=null → full backfill or grade demotion
- Full audit of CROSS tags: estimate 5–8 of the 56 violate self-ref/intra-repo rules and lose CROSS
- witness≥2 proof required → existing increment logs need auditing

### 3.2 Long-term impact

- Forced resonance_n6 resonance → average signal quality rises (M9+ promotion rate expected +15pp)
- Semantic tightening of CROSS → cross-repo evidence [EC] confidence improves
- [M10**] introduction → motivates integration with external DBs

---

## 4. Open discussion points

1. **Permitted scope for ad-hoc resonance_n6 formulas**:
   - Prevent post-hoc formula abuse such as "n·τ=24 approximation"
   - Proposal: the formula field must include `ε=...` or explicit `ad hoc` marker

2. **Automatic verification of [M10**] external-DB registration**:
   - OEIS API calls / arXiv-search automation required
   - Currently manual

3. **Definition of session end**:
   - `/compact` invocation vs. manual commit time
   - Proposal: tag commit messages with `[signals-merge]`

4. **Enforced staging cadence**:
   - A hook or loop-guard that warns when staging is not emptied at session end
   - Proposal: pre-commit hook

---

## 5. Status of the 7 Millennium Problems

**This v0.3 spec covers only formula definitions and rule tightening; it does not claim to solve any of the 7 Millennium Problems (RH/NS/Hodge/BSD/YM/PvsNP/Poincaré).**

- Currently solved Millennium Problems: **0 / 7**
- The signal SSOT is an evidence-collection / reproduction-tracking tool — proofs proceed separately under theory/proofs/
- Grades [M10*] / [M10**] certify measurement quality at an "exact" level only — they do not mean "proof complete"
- R0 honesty constraint preserved

---

## 6. Version history

- **v0.1** (2026-04-15 22:30 KST): millennium-only initial draft — deprecated
- **v0.2** (2026-04-15 22:40 KST): redesigned as a 3-repo cross-repo general-purpose SSOT
- **v0.3-draft** (2026-04-15 ~): resonance_n6 mandatory + CROSS strictness + [M10**] added + staging merge cadence explicit ★

---

## 7. References

- Original spec v0.2: `n6shared/specs/atlas.signals.n6.spec.md`
- SSOT data: `$NEXUS/shared/n6/atlas.signals.n6` (350+ signals)
- This session's backup: `$NEXUS/shared/n6/atlas.signals.n6.bak.pre-cross-backfill`
- CROSS promotion report: `/Users/ghost/Dev/n6-architecture/reports/cross-backfill-20260415.md`
