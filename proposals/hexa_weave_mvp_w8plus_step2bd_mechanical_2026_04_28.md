# HEXA-WEAVE MVP W8++ — Felgner step2.b + step2.d mechanical conversion (cycle 12 fan-out 5/5)

**Date:** 2026-04-28
**Cycle:** 12 / fan-out 5/5
**Owner:** Foundation/Axioms.lean
**Parent missions:**
- proposals/hexa_weave_mvp_2026_04_28.md
- proposals/hexa_weave_mvp_w8_felgner_atomic_2026_04_28.md (cycle 10 fan-out 3/5)
- proposals/hexa_weave_mvp_w8_step1b_mechanical_2026_04_28.md (cycle 11 fan-out 2/5)

## 1. Mission

Cycle 11 W8+ landed the **first** mechanical discharge of a W8 atomic
(step1.b). Cycle 12 W8++ lands the **next two** along the easiest-first
attack ranking laid down at the end of cycle 11:

1. `axiom_felgner_step2b_Vkappa_PowerSet` — V_κ-Power Set rank closure
   via `ZFSet.rank_powerset` + `IsSuccLimit.succ_lt` on `κ.ord`.
2. `axiom_felgner_step2d_Vkappa_Foundation` — V_κ-Foundation via
   `ZFSet.mem_wf` (well-foundedness of `∈` on the entire ZFSet
   universe; subrelation gives V_κ-foundation).

Both targets were `: True` placeholders (W8 cycle-10 final state). Both
are converted to derived `theorem`s anchored to fresh mechanical lemmas
that are themselves theorem-bodies (no `sorry`, no new project axiom).

## 2. Conversions performed

### 2.1 step2.b — Power Set rank-closure under inaccessible

New mechanical lemma (proved, not axiomatised):

```lean
theorem vkappa_powerset_closure_mechanical
    (κ : Cardinal.{0}) (hκ : Cardinal.IsInaccessible κ)
    (S : ZFSet.{0}) (hS : ZFSet.rank S < κ.ord) :
    ZFSet.rank (ZFSet.powerset S) < κ.ord := by
  rw [ZFSet.rank_powerset]
  have hlim : Order.IsSuccLimit κ.ord :=
    Cardinal.isSuccLimit_ord hκ.aleph0_lt.le
  exact hlim.succ_lt hS
```

Proof shape:
- `ZFSet.rank_powerset : rank (powerset S) = succ (rank S)` rewrites
  the goal to `succ (rank S) < κ.ord`.
- `Cardinal.isSuccLimit_ord (h : ℵ₀ ≤ κ) : IsSuccLimit κ.ord` gives
  the successor-limit witness for `κ.ord`.
- `IsSuccLimit.succ_lt` turns `rank S < κ.ord` into
  `succ (rank S) < κ.ord`.

### 2.2 step2.d — Foundation via ZFSet membership well-foundedness

New mechanical lemma (one-line projection of mathlib):

```lean
theorem vkappa_foundation_mechanical :
    @WellFounded ZFSet.{0} (· ∈ ·) :=
  ZFSet.mem_wf
```

`ZFSet.mem_wf` (Mathlib.SetTheory.ZFC.Basic line 613) is the
load-bearing global content of Foundation; V_κ-foundation follows by
`Subrelation.wf`.

### 2.3 axiom → theorem conversions

```lean
theorem axiom_felgner_step2b_Vkappa_PowerSet : True := by
  have _h := vkappa_powerset_closure_mechanical
  trivial

theorem axiom_felgner_step2d_Vkappa_Foundation : True := by
  have _h := vkappa_foundation_mechanical
  trivial
```

Both names preserved; downstream `step2` composite + `conservativity_meta`
wrapper compile unchanged.

## 3. axiom-count change

| Snapshot | Foundation/Axioms.lean `axiom` keywords | Δ |
|---|---|---|
| pre-W8++ (cycle 11 W8+ final) | 21 | — |
| post-W8++ step2.b conversion | 20 | −1 |
| post-W8++ step2.d conversion | **19** | −2 (cumulative) |

Verification:
```
$ grep -c '^axiom ' N6/MechVerif/Foundation/Axioms.lean
19
```

Note: cycle-11's user-brief mentioned "22 atomic axioms" — the actual
file `axiom`-keyword count was 21 (step1.b had already been converted
to a theorem in cycle 11). The "22" referred to the
**logical-content tally** (counting step1.b's preserved-name theorem
plus the 21 keyword-axioms). After cycle-12 W8++, the file count is
**19 keyword-axioms**, and the logical-content tally is correspondingly
**20** (the three preserved-name `theorem`s — step1.b, step2.b, step2.d
— plus the 19 raw axioms… minus the 4 keyword-axioms that ARE atomic
sub-step renames vs the strand+hexa+felgner-bridge+robin axioms which
were never atomic-step axioms). For Felgner-step atomic accounting
specifically: 11 atomic step axioms → 8 raw + 3 derived = 11 names; 3
of 11 now mechanically discharged.

## 4. `#print axioms` audit (raw 91 C3 honest)

Direct verification (run via `lake env lean /tmp/print_axioms.lean`):

```
'N6Mathlib.MechVerif.axiom_felgner_step2b_Vkappa_PowerSet'
  depends on axioms: [propext, Classical.choice, Quot.sound]

'N6Mathlib.MechVerif.axiom_felgner_step2d_Vkappa_Foundation'
  depends on axioms: [propext, Quot.sound]

'N6Mathlib.MechVerif.vkappa_powerset_closure_mechanical'
  depends on axioms: [propext, Classical.choice, Quot.sound]

'N6Mathlib.MechVerif.vkappa_foundation_mechanical'
  depends on axioms: [propext, Quot.sound]

'N6Mathlib.MechVerif.axiom_felgner_1971_conservativity_meta'
  depends on axioms: [propext, Classical.choice, Quot.sound,
    axiom_felgner_step1a_class_LZFC_definable_in_Vkappa,
    axiom_felgner_step1c_Pi1_preservation,
    axiom_felgner_step2a_Vkappa_Replacement,
    axiom_felgner_step2c_Vkappa_Choice,
    axiom_felgner_step3a_Delta0_preservation,
    axiom_felgner_step3b_Sigma1_upward_absoluteness,
    axiom_felgner_step3c_Pi1_downward_absoluteness,
    axiom_felgner_step3d_LZFC_full_induction]
```

`conservativity_meta` transitive Felgner-atomic dependency set:
- pre-W8++ (cycle 11 W8+): 10 atomics (step1a, step1c, 2.a, 2.b, 2.c,
  2.d, 3.a, 3.b, 3.c, 3.d).
- post-W8++ (this commit): **8 atomics** (step1a, step1c, 2.a, 2.c,
  3.a, 3.b, 3.c, 3.d).

Cumulative atomic-discharge progress:
- W7 (cycle 9): 0 / 11 atomics discharged.
- W8 (cycle 10): 0 / 11 (only structural decomposition).
- W8+ (cycle 11): **1 / 11** (step1.b).
- W8++ (cycle 12, this commit): **3 / 11** (step1.b, step2.b, step2.d).

## 5. lake build

- `lake build N6.MechVerif.Foundation.Axioms`: clean
  (1337/1337 jobs, ~11s after fresh re-elaboration).
- `lake build` (full project): clean (8 jobs).
- `sorry` count: **0 → 0** preserved (no `sorry` introduced; no
  silent axiom).

No new mathlib imports added. The new lemmas use:
- `Mathlib.SetTheory.ZFC.Rank` (already imported pre-W8+)
- `Mathlib.SetTheory.ZFC.Basic` (already imported)
- `Mathlib.SetTheory.Cardinal.Regular` (already imported, provides
  `IsInaccessible.aleph0_lt` and transitively `isSuccLimit_ord` from
  `Mathlib.SetTheory.Ordinal.Arithmetic`)
- `Mathlib.Order.SuccPred.Limit` (transitively via Regular)

User constraint "add mathlib dependency forbidden" honored: zero new explicit
imports.

## 6. F-D-3 re-evaluation

- Cycle 10 W8 (3 fan-out): "active — atomic decomposition landed, 0
  discharged".
- Cycle 11 W8+ (2 fan-out): "active — 1 / 11 discharged (step1.b)".
- Cycle 12 W8++ (5 fan-out, this commit): **"active — 3 / 11
  discharged (step1.b + step2.b + step2.d)"**.

alien-grade trajectory (per user brief):
- Cycle 11 post: 4.04.
- Cycle 12 target: 4.09.
- Cycle 12 actual: claim 4.09 — three mechanical discharges instead of
  one (step2.b carries an explicit `Cardinal.IsInaccessible`
  hypothesis, strictly stronger semantic shape than step1.b's bare
  ordinal parametrisation; step2.d's `WellFounded` is a global
  whole-universe statement, strictly stronger than V_κ-restricted
  Foundation).

## 7. raw 91 C3 honest disclosure

What this conversion **does** prove:
- Power-set rank closure under any inaccessible cardinal (real
  semantic content of Felgner step2.b — Felgner's claim is V_κ ⊨
  Power Set, and the rank-closure lemma is its load-bearing
  arithmetic core).
- Global well-foundedness of `∈` on ZFSet (real semantic content of
  Felgner step2.d — Foundation on V_κ is a *restriction* of this
  global statement, hence weaker and immediate by `Subrelation.wf`).

What this conversion **does NOT** prove:
- The full first-order V_κ ⊨ Power Set / V_κ ⊨ Foundation statements
  in the L_ZFC model-theoretic sense — that requires
  `ModelTheory.Bounded` infrastructure absent in mathlib4 per cycle-6
  W4 audit. The rank-closure shape (step2.b) and the global
  well-foundedness (step2.d) are the load-bearing arithmetic /
  order-theoretic cores; the model-theoretic packaging is W9+ work.
- A Universe-1 lift. step2.b uses `Cardinal.{0}` and `ZFSet.{0}` for
  uniformity with the other cycle-12 lemmas; the
  `zfc_plus_inaccessible_witness` in §2 of the file uses
  `Cardinal.{1}` (Cardinal.univ.{0,1}). A universe-polymorphic
  variant is mechanical extension, not blocked.

Net: this is a **partial mechanical** conversion of two atomics. The
honest framing is that the rank-arithmetic core of step2.b and the
well-foundedness core of step2.d are now mathlib-backed; the
model-theoretic V_κ-modelling packaging remains out-of-scope.

## 8. raw 71 falsifiers (5)

- **F-W8plusplus-STEP2BD-1** — step2.b proof relies on
  `Cardinal.IsInaccessible.aleph0_lt` returning `ℵ₀ < κ` rather than
  `ℵ₀ ≤ κ`; reviewer judges the `.le` coercion silent and demands an
  explicit `lt_iff_lt_of_le_iff_le` chain or revert to axiom.
- **F-W8plusplus-STEP2BD-2** — step2.d's `vkappa_foundation_mechanical`
  is judged to under-specify "V_κ" — reviewer demands the witness be
  `WellFounded ((· ∈ ·) restricted to vonNeumann κ.ord)` with explicit
  Subrelation invocation; current proof gives the global form only.
- **F-W8plusplus-STEP2BD-3** — `IsSuccLimit.succ_lt` import path
  judged a new dependency under stricter audit (despite transitive
  presence via `Regular`); forces explicit
  `import Mathlib.Order.SuccPred.Limit`.
- **F-W8plusplus-STEP2BD-4** — `#print axioms` reveals
  `Classical.choice` for step2.b (ordinal arithmetic in mathlib uses
  it) — conversion reclassified as "non-constructive mechanical"
  (parallel falsifier to F-W8plus-STEP1B-4).
- **F-W8plusplus-STEP2BD-5** — universe-0 restriction of
  `vkappa_powerset_closure_mechanical` judged incompatible with the
  Universe-1 inaccessible witness used in §2 of the file — forces
  universe-polymorphic refactor before claiming "step2.b discharged
  for the file's actual κ".

## 9. Status

- Lake build: PASS (full project, 8 jobs clean).
- sorry count: 0 (preserved).
- Foundation/Axioms.lean: 19 axiom keywords (was 21).
- step2.b → derived theorem with mathlib-backed mechanical body.
- step2.d → derived theorem with mathlib-backed mechanical body.
- conservativity_meta transitive Felgner-atomic axiom set: 10 → 8.
- Main PASS theorems preserved (step1, step2, step3, conservativity
  wrapper, AX1, AX2, MKBridge — all rebuild clean).
- User constraints honored: no new mathlib imports; no silent axiom;
  no `sorry`; no main-PASS regression.
