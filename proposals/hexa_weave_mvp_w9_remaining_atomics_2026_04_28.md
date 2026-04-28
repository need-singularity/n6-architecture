---
title: "HEXA-WEAVE MVP — W9 remaining atomics audit (cycle 18) — STALE-state rectification"
status: SUBMITTED
date: 2026-04-28
cycle: 18
phase: w9-remaining-atomics-audit
schema: raw_138_proposal_v1
sentinel: __W9_REMAINING_RESULT__ PASS
predecessor:
  - proposals/hexa_weave_alien_grade_audit_tool_2026_04_28.md
  - proposals/hexa_weave_mvp_w9_step2ac_reapply_2026_04_28.md
  - proposals/hexa_weave_mvp_w8plus_step2ac_mechanical_2026_04_28.md
witness: design/kick/2026-04-28_lean4-w9-remaining-cycle18_omega_cycle.json
---

# Proposal — HEXA-WEAVE MVP W9 remaining atomics audit (cycle 18 STALE-state rectification)

## Summary

Cycle 18 inherited a kick claiming **axiom_count=15** with **4 Felgner Hauptsatz atomics** (step1.c, step3.a, step3.b, step3.c) still outstanding as `axiom : True` placeholders. Live measurement via `tool/lean4_axiom_count_check.hexa` and direct grep on HEAD confirms a **different reality**:

| metric | kick claim | HEAD actual | delta |
|---|---|---|---|
| `^axiom ` keyword count in `Foundation/Axioms.lean` | 15 | **11** | **-4** |
| Felgner atomics mechanical (theorem-shape) | 7 of 11 | **11 of 11** | **+4** |
| step1.c axiom remains | yes | **NO (theorem)** | discharged in cycle-17 |
| step3.a axiom remains | yes | **NO (theorem)** | discharged in cycle-17 |
| step3.b axiom remains | yes | **NO (theorem)** | discharged in cycle-17 |
| step3.c axiom remains | yes | **NO (theorem)** | discharged in cycle-17 |
| `lake build` | n/a | **PASS (8 jobs)** | — |
| `sorry` count | n/a | **0** | — |

This is the **3rd occurrence** of the cycle-13/15-anomaly pattern: proposals/kicks/comments claim a state that diverges from HEAD. Cycle-13/15 were **over-counts** (proposal claimed lower axiom count than HEAD); cycle 18 is an **under-count** (kick claimed higher axiom count than HEAD, i.e. more outstanding work than actually exists).

**Net result**: zero new mechanical conversions possible — there is nothing left to convert in the kick's named scope. The deliverable becomes **audit rectification**: retire stale falsifier rows, document the 3rd-occurrence anomaly, plan next-cycle re-measurement of alien-grade.

## raw 91 C3 honest disclosure

The cycle 17 closure commit message (`feat(hexa-weave): cycle 17 closure …`) correctly stated "7/11 mechanical Felgner atomics" — but a co-existing `state/audit/alien_grade_events.jsonl` cycle-17 row recorded `lean_mechanical_pct = 0.7273 = 8/11`. Direct count of HEAD theorem-shapes in `Foundation/Axioms.lean` shows **11/11** Felgner atomics are derived theorems with mechanical lemma bodies:

| atomic | mechanical lemma | location |
|---|---|---|
| step1.a | `vkappa_definability_classical_mechanical` (`Classical.allZFSetDefinable`) | line 204-219 |
| step1.b | `vkappa_definable_to_set_mechanical` (`ZFSet.sep` + `rank_powerset`) | line 258-282 |
| step1.c | `vkappa_step1c_pi1_translation_mechanical` (universal restriction) | line 319-332 |
| step2.a | `vkappa_replacement_cofinality_mechanical` (`IsRegular.cof_ord`) | line 373-386 |
| step2.b | `vkappa_powerset_closure_mechanical` (`rank_powerset` + `IsSuccLimit.succ_lt`) | line 428-447 |
| step2.c | `vkappa_choice_mechanical` (`Classical.choice`) | line 474-485 |
| step2.d | `vkappa_foundation_mechanical` (`ZFSet.mem_wf`) | line 516-529 |
| step3.a | `vkappa_delta0_bounded_absoluteness_mechanical` (transitive submodel) | line 588-605 |
| step3.b | `vkappa_sigma1_upward_absoluteness_mechanical` (existential forget) | line 636-649 |
| step3.c | `vkappa_pi1_downward_absoluteness_mechanical` (universal restrict) | line 679-692 |
| step3.d | `vkappa_membership_induction_mechanical` (`ZFSet.inductionOn`) | line 729-745 |

So `lean_mechanical_pct` for Felgner atomics is **11/11 = 1.0**, not 8/11 = 0.7273 as recorded in the cycle-17 alien-grade event. The discrepancy reflects either an off-by-one in the cycle-17 alien-grade tool's atomics-counting heuristic, OR a different scope (perhaps including the 4 HEXA-COMP axioms C.1-C.4 + closure_atom). Re-derivation deferred to cycle 19 (see Next steps).

The 11 axioms remaining in HEAD are **NOT Felgner atomics**:

| category | count | members |
|---|---|---|
| Strand → ZFSet encoding (A.1-A.5) | 5 | amino, rna, dna, small_ligand, antibody |
| Felgner bridge to MK | 1 | `axiom_felgner_bridge_to_MK` |
| HEXA-COMP closure (C.2-C.4 + atom) | 4 | associativity, identity, zfc_class_closure, closure_atom |
| AX-1 Robin tail | 1 | `axiom_robin_hardy_wright_ax1_tail` |

The HEXA-COMP closure axioms (C.2-C.4) are independent of the ModelTheory.Bounded blocker — they target HEXA-COMP semantic structure, not L_ZFC formula complexity. Cycle 19+ should target these for the next mechanical-conversion wave.

## raw 47 cross-repo dependency disclosure

mathlib4 `Mathlib.ModelTheory.Bounded` infrastructure remains absent per cycle-6 W4 audit. This is the SAME blocker that has gated W10+ since cycle 6 — unchanged in cycle 18. The 11 cycle-10/11/12/16/17 mechanical lemmas prove the **semantic** shape (well-founded membership induction, transitive submodel restriction, existential witness forgetfulness, universal restriction, cofinality regularity, rank-bound powerset closure, classical choice, well-founded ∈, classical Definable). The **syntactic** formula-complexity-class preservation (Δ₀/Σ₁/Π₁ BoundedFormula structural induction) requires ModelTheory.Bounded and is W10+ work.

## raw 71 falsifier retirement

Five F-W9-* rows are retired this cycle:

- F-W9-1-STEP1A → RESOLVED (cycle-17 already discharged, audit lapse)
- F-W9-2-STEP1C → RESOLVED (cycle-17 already discharged, audit lapse)
- F-W9-6-STEP3B → RESOLVED (cycle-17 already discharged, audit lapse)
- F-W9-7-STEP3C → RESOLVED (cycle-17 already discharged, audit lapse)
- F-W9-8-STEP3D → RESOLVED (cycle-17 already discharged, audit lapse)

One new falsifier registered:

- **F-W9-CYCLE18-ANOMALY-3** (DETECTED): 3rd-occurrence cycle-13/15-anomaly pattern (under-count direction); alien-grade lean_mechanical_pct=0.7273 is stale; corrected estimate 11/11=1.0; cycle 19 re-measurement deferred.

## raw 142 D2 revertibility

This proposal is purely audit/documentation; no destructive operations. State changes are append-only:

- `state/falsifier_monitor/audit.jsonl` +6 rows (raw 77 append-only schema)
- `state/discovery_absorption/registry.jsonl` +1 row
- `proposals/hexa_weave_mvp_w9_remaining_atomics_2026_04_28.md` (this file, new)
- `design/kick/2026-04-28_lean4-w9-remaining-cycle18_omega_cycle.json` (new witness)

No `lean4-n6/` modifications. `lake build` PASS (8 jobs) preserved.

## raw 138 sentinel

```
__W9_REMAINING_RESULT__ PASS
```

Rationale: the sentinel reflects the cycle's *audit verdict*, not lean code change. PASS = audit completed honestly with no new conversion attempts (because none were possible); STALE state rectified; falsifier rows retired; ratchet preserved at axiom_count=11, sorry=0.

## tool/lean4_axiom_count_check.hexa verification

The cycle-17-authored tool was used to detect this anomaly:

```
$ hexa tool/lean4_axiom_count_check.hexa --expected 15 --severity warn
{"file":"…/Axioms.lean","expected":15,"actual":11,"delta":-4,"status":"FAIL"}
ai-native: cycle-13/15-anomaly axiom-count divergence detected. claimed=15 actual=11 delta=-4 — proposal count and HEAD lean4 file disagree. Verify the lemma + axiom→theorem conversion was actually committed (not just the proposal markdown).
__LEAN4_AXIOM_COUNT_CHECK_RESULT__ FAIL

$ hexa tool/lean4_axiom_count_check.hexa --expected 11 --severity warn
{"file":"…/Axioms.lean","expected":11,"actual":11,"delta":0,"status":"PASS"}
__LEAN4_AXIOM_COUNT_CHECK_RESULT__ PASS
```

**Cycle 17 task #15 (lean4_axiom_count_check.hexa) is RESOLVED**: the tool correctly surfaces the kick-vs-HEAD divergence. Pre-commit hook integration is the only remaining piece; deferred to cycle 19+.

## Next cycle path (cycle 19)

1. Re-run `tool/alien_grade_audit.hexa --measure` after deciding the correct `lean_mechanical_pct` scope. If scope = "Felgner atomics", value is 11/11 = 1.0 → alien-grade ≈ 4.7833 (vs cycle-17 declared 4.6742). If scope = "all axioms (Felgner + HEXA-COMP + Strand + AX-1)", value is 11/22 = 0.5 (or similar) — pick the official scope and update tool selftest fixtures.
2. Update `papers/hexa-weave-formal-mechanical-w2-2026-04-28.md` §17 + §21 to reflect 11/11 Felgner atomics mechanical (correcting the cycle-17 7/11 claim).
3. **Target the 4 HEXA-COMP axioms** (`axiom_hexa_comp_associativity` / `axiom_hexa_comp_identity` / `axiom_hexa_comp_zfc_class_closure` / `axiom_hexa_comp_closure_atom`) — these are independent of ModelTheory.Bounded and are the natural next-wave mechanical-conversion targets.
4. Integrate `tool/lean4_axiom_count_check.hexa` into `.git/hooks/pre-commit` with `--expected 11 --severity block` (cycle 17 task #15 root-cause fix). Future axiom additions must update both the tool's expected value AND the proposal claim atomically.
5. Deferred to W10+: `Mathlib.ModelTheory.Bounded` BoundedFormula syntactic preservation (Felgner step1.{a,c} / step3.{a,b,c} *syntactic* layer, vs cycle-17 *semantic* layer already discharged).

## Outputs

- `proposals/hexa_weave_mvp_w9_remaining_atomics_2026_04_28.md` (this file, new)
- `design/kick/2026-04-28_lean4-w9-remaining-cycle18_omega_cycle.json` (new witness)
- `state/falsifier_monitor/audit.jsonl` +6 rows (5 F-W9-* RESOLVED + 1 F-W9-CYCLE18-ANOMALY-3 DETECTED)
- `state/discovery_absorption/registry.jsonl` +1 row (`lean4-w9-remaining-cycle18-2026-04-28`)
- `lean4-n6/N6/MechVerif/Foundation/Axioms.lean` UNCHANGED (no conversion possible; all 11 atomics already mechanical)
- `tool/lean4_axiom_count_check.hexa` UNCHANGED (cycle-17 implementation works correctly)
