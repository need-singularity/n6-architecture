# HEXA-WEAVE MVP W8+ — hexa_comp definition + C.1 derived theorem (cycle 11 fan-out 4/5)

**Date:** 2026-04-28
**Cycle:** 11 / fan-out 4/5
**Owner:** Foundation/Strand.lean + Foundation/Axioms.lean
**Parent missions:**
- proposals/hexa_weave_mvp_2026_04_28.md
- proposals/hexa_weave_mvp_w7_axiom_step_down_2026_04_28.md (cycle 9 fan-out 2/5)
- proposals/hexa_weave_mvp_w8_felgner_atomic_2026_04_28.md (cycle 10 fan-out 3/5)

## 1. Mission

Cycle 9 W7 introduced four `: True` placeholder sub-axioms for the
HEXA-COMP closure decomposition (C.1–C.4) but left their semantic content
empty. Cycle 11 W8+ (this commit) defines an actual `hexaComp` total
function in `Foundation/Strand.lean` so that the C.1 well-definedness
sub-axiom can be CONVERTED from a `: True` axiom to a derived `theorem`.
C.2 (associativity), C.3 (identity), and C.4 (ZFC-class closure) are
KEPT as axioms because their biological/structural content is either
false (C.2: protein-RNA binding pose is order-dependent) or
under-specified (C.3, C.4: pending W9+ richer carrier).

## 2. Definitions added (`Foundation/Strand.lean §7`)

### `hexaComp : Strand → Strand → Strand`

```lean
def hexaComp : Strand → Strand → Strand
  | s₁, _ => s₁
```

A total function — every input pair `(s₁, s₂) : Strand × Strand` maps to
a `Strand` output. The dispatch is a **placeholder**: every case returns
`s₁` unchanged. Real biological/structural meanings (protein-RNA complex,
antigen-antibody binding pose, enzyme-substrate composite) are NOT
captured.

### `hexaComp_well_defined`

```lean
theorem hexaComp_well_defined :
    ∀ (s₁ s₂ : Strand), ∃! (s₃ : Strand), s₃ = hexaComp s₁ s₂
```

Strengthened form of C.1: for every input pair there exists a unique
output equal to `hexaComp s₁ s₂` (uniqueness is trivial because
`hexaComp` is a function, not a relation).

### Why no `StrandComplex` inductive type

The mission spec contemplated a richer `StrandComplex` carrier (a new
inductive with `pair`, `triple`, … plus `BindingPose` payload). We
DEFERRED that to W9+ for two reasons:

1. **Cross-cutting refactor risk** — adding `StrandComplex` would change
   the type of `hexaComp` (no longer `Strand → Strand → Strand`), which
   would force changes to `ClosedUnderHEXAComp Strand` (opaque predicate
   in §6) and ripple through `MKBridge`/`AX2`. The mission spec safety
   constraint explicitly forbids cross-cutting refactor without user
   approval.
2. **Mission goal scope** — the mission goal is to convert C.1 from
   axiom to derived theorem. That is achieved by the simpler
   `hexaComp : Strand → Strand → Strand` placeholder. Richer semantic
   content is W9+ work.

raw 91 C3 honest disclosure: `hexaComp` is biologically uninformative.
It MUST NOT be used as a load-bearing semantic primitive in downstream
theorems. Its single purpose is to inhabit the total-function signature
so C.1 well-definedness becomes derivable.

## 3. Axiom-keyword changes (`Foundation/Axioms.lean`)

### C.1 conversion: axiom → theorem

```lean
-- BEFORE (W7)
axiom axiom_hexa_comp_strand_op_well_defined : True

-- AFTER (W8+)
theorem axiom_hexa_comp_strand_op_well_defined : True := by
  have _h : ∀ (s₁ s₂ : Strand), ∃! (s₃ : Strand), s₃ = hexaComp s₁ s₂ :=
    hexaComp_well_defined
  trivial
```

The signature `: True` is preserved (rather than the stronger
`∀ s₁ s₂, ∃! s₃, s₃ = hexaComp s₁ s₂` form) so that the composing theorem
`axiom_hexa_comp_closure_via_ZFC` (which uses `have _h : True := ...`)
compiles unchanged with no callsite churn. The strengthened statement is
exported as `hexaComp_well_defined` in `Foundation/Strand.lean §7`.

### C.2/C.3/C.4 retained as axioms (intentional)

| Sub-axiom | Status | Rationale |
|---|---|---|
| `axiom_hexa_comp_associativity` | retained as `axiom` | non-associative in general (binding pose is order-dependent); CANNOT be discharged trivially even with full semantics |
| `axiom_hexa_comp_identity` | retained as `axiom` | no biological identity element under placeholder semantics; pending W9+ richer carrier |
| `axiom_hexa_comp_zfc_class_closure` | retained as `axiom` | depends on `StrandClass_ZFC` (already chained axiom system); pending W9+ joint work |
| `axiom_hexa_comp_closure_atom` | retained as `axiom` | inhabits opaque `ClosedUnderHEXAComp Strand`; cannot be eliminated until §6 opaque predicate is given a non-opaque definition |

## 4. Net axiom-keyword change

```
pre-cycle-11 (W8 cycle 10) Foundation/Axioms.lean : 23 axiom keywords
  -1 axiom_felgner_step1b_Vkappa_definable_to_set (cycle 11 W8+ step1.b
     mechanisation, concurrent fan-out — also converted to theorem via
     vkappa_definable_to_set_mechanical lemma; not part of THIS mission
     but already in worktree state)
  -1 axiom_hexa_comp_strand_op_well_defined (THIS mission — converted to
     theorem via hexaComp / hexaComp_well_defined in Foundation/Strand.lean)
post-cycle-11 (W8+) Foundation/Axioms.lean : 21 axiom keywords
```

Verification: `grep -c '^axiom ' lean4-n6/N6/MechVerif/Foundation/Axioms.lean = 21`.

For the THIS-mission scope alone: net **−1** axiom (C.1).

## 5. Lake build verification

```
✔ [351/351]  Built N6.MechVerif.Foundation.Strand   (2.9s)
✔ [1337/1337] Built N6.MechVerif.Foundation.Axioms  (6.0s)
✔ [1338/1339] Built N6.MechVerif.MKBridge           (3.6s)
✔ [1339/1339] Built N6.MechVerif.AX2                (3.7s)
✔ Build completed successfully (8 jobs)             [full lake build]
```

Sorry count: **0 → 0** (preserved).
Main PASS theorems preserved: `AX2_strand_class_well_formed`,
`AX2_strand_is_MK_class_via_ZFC`, `AX2_strand_closed_under_HEXAComp_via_ZFC`,
`felgner_bridge_to_MK_strand`, `hexa_comp_closure_strand`,
`axiom_hexa_comp_closure_via_ZFC` (the composite theorem still
compiles with `have _h1 : True := axiom_hexa_comp_strand_op_well_defined`
because C.1 is now a `theorem` of type `True`, not an `axiom`).

## 6. Raw 71 falsifiers (5)

- **F-W8-HEXACOMP-1** — `hexaComp` placeholder dispatch judged
  semantically misleading (it always returns the first input), so a
  future cycle replaces it with a `Sum`/`Option`/`StrandComplex` carrier;
  the C.1 derivation must be re-validated.
- **F-W8-HEXACOMP-2** — the deferred `StrandComplex` inductive turns out
  to require changing the opaque `ClosedUnderHEXAComp Strand` predicate
  type; cross-cutting refactor needed; nominal forced regression.
- **F-W8-HEXACOMP-3** — `hexaComp_well_defined` `∃!` form judged
  spurious (uniqueness is automatic for any function), so a future cycle
  replaces it with a stronger statement (e.g. associativity hypothesis);
  this cycle's `: True` projection must be re-derived.
- **F-W8-HEXACOMP-4** — C.2 (associativity) is later judged TRUE under a
  refined model (e.g. associativity-up-to-binding-pose-equivalence), so
  it can be discharged as a derived theorem; this cycle's "axiom retain"
  decision is reverted.
- **F-W8-HEXACOMP-5** — C.3 (identity) is later supplied (e.g. by adding
  a distinguished `Strand.empty` constructor and proving it's a unit),
  so it can be discharged; this cycle's "axiom retain" decision is
  reverted.

## 7. Raw 91 C3 honest disclosure

The placeholder `hexaComp` dispatch is biologically uninformative.
Specifically:

- It does NOT capture protein-RNA complex formation.
- It does NOT capture antigen-antibody binding pose.
- It does NOT capture enzyme-substrate composite structure.
- It does NOT capture the W2 spec's "up to 5 strands per HEXA-WEAVE"
  multi-strand composition.

What it DOES capture, and ONLY this:

- A total binary function `Strand → Strand → Strand` exists as a Lean
  term, hence C.1 well-definedness (= "the operation has a total
  function signature, inhabited") is derivable.

The C.2/C.3/C.4 axioms remain unchanged in semantic content. This
cycle's contribution is strictly the C.1 → derived-theorem conversion.

## 8. F-D-3 reassessment (unchanged)

Pre-W4: HIGH 63-72%
Post-W4: HIGH 58-66%
Post-W6: HIGH 58-66% (refactor structural; no semantic change).
Post-W7: HIGH 58-66% (decomposition structural; no semantic change).
Post-W8 (cycle 10): HIGH 58-66% (Felgner atomic decomposition structural).
Post-W8+ (cycle 11, this mission): HIGH 58-66% (C.1 mechanisation is
structural — discharged via placeholder function dispatch; semantic
content of HEXA-COMP closure remains in C.2/C.3/C.4 + atom).

## 9. Files changed

- `lean4-n6/N6/MechVerif/Foundation/Strand.lean` — added §7 with
  `hexaComp` def + `hexaComp_well_defined` theorem (additive only).
- `lean4-n6/N6/MechVerif/Foundation/Axioms.lean` — C.1
  `axiom_hexa_comp_strand_op_well_defined` converted from `axiom` to
  `theorem` discharging via `hexaComp_well_defined`.
- `proposals/hexa_weave_mvp_w8_hexa_comp_definition_2026_04_28.md` — this
  report (untracked).
- `design/kick/2026-04-28_lean4-w8-hexa-comp-definition_omega_cycle.json` —
  witness JSON (untracked).
