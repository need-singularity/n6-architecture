---
category: operational
umbrella: formal-verification
parent_spec: proposals/hexa-weave-formal-mechanical-verification-prep.md
predecessor: proposals/hexa_weave_mvp_w1_lean4_audit_2026_04_28.md
milestone: W2
date: 2026-04-28
w2_partial_completed: true
---

# HEXA-WEAVE MVP — W2 lean4 `thm.AX1_n6_uniqueness` first-attempt report

> **W2 deliverable** for `proposals/hexa-weave-formal-mechanical-verification-prep.md` Task D Spec §4 unit 1.
>
> **Date**: 2026-04-28 (cycle 4 fan-out 4/5; early start; W2 nominal window 2026-05-05 → 2026-05-18).
>
> **Build basis**: mathlib4 master @ rev `19c497800a418208f973be74c9f5c5901aac2f54`, lean4 `v4.30.0-rc1` (toolchain reverted from rc2 to rc1 to match mathlib pin; see §4 environment).
>
> **W2 partial PASS** — bounded forward direction (n ∈ [2, 30]) PASS via `decide` + `interval_cases`; reverse direction (n=6 → equality) PASS via `decide`. Unbounded tail (n > 30) carries `sorry` placeholder per raw 91 C3 (Spec §4 unit 1 calls for Robin-style asymptotic; deferred to W3-W4).

---

## §1 W2 deliverable — what landed

**File**: `lean4-n6/N6/MechVerif/AX1.lean` (NEW)

**Theorems delivered**:

| Name | Statement | Proof status |
|------|-----------|--------------|
| `AX1Eq` (def) | `σ 1 n * Nat.totient n = n * (Nat.divisors n).card` | n/a (definition) |
| `AX1_reverse_n6` | `AX1Eq 6` (n=6 → equality direction) | PASS via `decide` |
| `AX1_n6_witness` | `σ(6)=12 ∧ φ(6)=2 ∧ τ(6)=4` | PASS via `decide` |
| `AX1_forward_bounded_30` | `∀ n ∈ [2,30], AX1Eq n → n=6` | PASS via `interval_cases` + `decide` |
| `AX1_forward_tail` | `∀ n > 30, AX1Eq n → n=6` | **`sorry`** (Robin-style tail bound deferred) |
| `AX1_n6_uniqueness` | `∀ n ≥ 1, AX1Eq n ↔ n=6` (Spec wording) | **`sorry`** (n=1 case, surfaces Spec corrigendum below) |
| `AX1_n6_uniqueness_corrected` | `∀ n ≥ 2, AX1Eq n ↔ n=6` | partial; depends on `AX1_forward_tail sorry` |

**Build verification**:

```
$ lake build N6.MechVerif.AX1
⚠ [1310/1310] Built N6.MechVerif.AX1 (6.6s)
warning: N6/MechVerif/AX1.lean:75:8: declaration uses `sorry`   ← AX1_forward_tail
warning: N6/MechVerif/AX1.lean:95:8: declaration uses `sorry`   ← n=1 case in AX1_n6_uniqueness
Build completed successfully (1310 jobs).
```

11.7 s wall-clock total; 6.6 s for the AX1 module itself (mathlib oleans cached).

---

## §2 Spec §4 unit 1 corrigendum — n=1 edge case

**Discovery**: the Spec wording `∀ n ∈ ℕ, σ(n)·φ(n) = n·τ(n) ↔ n = 6` (with implicit `n ≥ 1` per the L5 witness `T_MK-HW + Φ_HW` AX-1 axiom) is **mechanically inconsistent** at n=1:

```
σ(1) = 1, φ(1) = 1, τ(1) = (Nat.divisors 1).card = 1
LHS = 1 · 1 = 1
RHS = 1 · 1 = 1
⇒ LHS = RHS  but  n = 1 ≠ 6  ⇒  iff FAILS at n=1
```

**Recommended Spec amendment** (low severity, statement-only correction):
- replace `∀ n : ℕ, n ≥ 1 → AX1Eq n ↔ n = 6` with `∀ n : ℕ, n ≥ 2 → AX1Eq n ↔ n = 6`.

The corrected statement is delivered as `AX1_n6_uniqueness_corrected`; it is provable up to the unbounded-tail `sorry`.

This is logged as **F-W1-2 firing** (the W1 audit's prediction that an additional Spec inaccuracy would surface in W2 work — confirmed at LOW severity, statement quantifier only).

---

## §3 mathlib API names — Spec §3 corrigenda CONFIRMED in source

The W1 audit flagged Spec §3 inaccuracy #3 (`Nat.ArithmeticFunction.tau` does NOT exist). W2 confirms the audit:

| Spec §3 name | Mathlib actual | Used in AX1.lean |
|--------------|----------------|------------------|
| `Nat.ArithmeticFunction.sigma` | `ArithmeticFunction.sigma` (notation `σ`) | YES — `σ 1 n` |
| `Nat.totient` | `Nat.totient` | YES |
| `Nat.divisors.card` | `(Nat.divisors n).card` | YES |
| `Nat.ArithmeticFunction.tau` | DOES NOT EXIST — encode as `σ 0 n = (Nat.divisors n).card` (`sigma_zero_apply`) | YES — used `(Nat.divisors n).card` directly per W1 audit recommendation |

**Imports actually needed**:
- `Mathlib.NumberTheory.ArithmeticFunction.Misc` — for `σ`
- `Mathlib.NumberTheory.Divisors` — for `Nat.divisors`
- `Mathlib.Data.Nat.Totient` — for `Nat.totient`
- `Mathlib.Tactic.IntervalCases` — for the `interval_cases` tactic (NOT in Spec §3 audit; minor addition)

`Mathlib.Tactic.PushNeg` was attempted but excluded (olean missing in cache snapshot); replaced by `Nat.lt_of_not_le` direct invocation.

---

## §4 environment status (build-system note)

**Toolchain mismatch encountered and recovered**:

- start of W2: `lean-toolchain` declared `leanprover/lean4:v4.30.0-rc2` (advanced from rc1 in cycle 3 `lake update`)
- mathlib pin: `.lake/packages/mathlib/lean-toolchain` declares `leanprover/lean4:v4.30.0-rc1` (unchanged since W1 audit)
- **regression**: rc2 elan elaborated rc1 mathlib oleans → meta-attribute access errors (`Batteries.Util.instInhabitedLibraryNote` marked `meta`, regressed with rc2 visibility rules) on `Mathlib/Logic/Basic.lean`, `Mathlib/Data/Nat/Init.lean`, `Mathlib/Algebra/Group/Defs.lean`
- **recovery action**: reverted `lean-toolchain` to `leanprover/lean4:v4.30.0-rc1` (one-line file edit, restoration not refactor) + `lake exe cache get` (mathlib's standard cached-olean fetcher; 8275 oleans decompressed, 38.8 s)
- after recovery: `lake build N6.MechVerif.AX1` PASS in 11.7 s with only 2 expected `sorry` warnings

**No new dependencies added; no `lake update` invoked; no `git init` or sister-repo creation; no user-system mutation beyond the toolchain-revert one-liner that aligns project to mathlib pin.**

---

## §5 file location — mission-text vs Spec §6 deviation

**Mission-text path**: `lean4-n6/HexaWeave/AX1NUniqueness.lean`
**Spec §6 canonical path**: `lean4-n6/N6/MechVerif/AX1.lean`

**Choice**: Spec §6 path used. Rationale:

1. Spec §6 (W2-W3 row) is the source-of-truth for build layout under parent Task D Spec.
2. Mission text path would require adding a new `lean_lib HexaWeave where` entry to `lakefile.lean` — a cross-cutting refactor that mission safety §3 explicitly forbids without user approval.
3. Theorem semantics, statement, and witness identities are identical between the two locations; only file path differs.
4. The mission's W2 deliverable (`thm.AX1_n6_uniqueness` first attempt) is delivered functionally regardless of file path.

Disclosure: this deviation surfaces a minor inconsistency between mission text (cycle 4) and Spec §6 (parent doc 2026-04-28); recommended action is to update mission text in next cycle to match Spec, or to amend Spec §6 to use HexaWeave path with appropriate lakefile changes (user approval required either way).

---

## §6 F-D-3 (90-day deadline) re-assessment after W2 partial

**Pre-W2 baseline** (per W1 audit §4): F-D-3 probability **HIGH 65-75%** (Spec §9 60-75% revised up by W1 +5pp lower bound).

**W2 evidence**:
- W2 partial PASS: `AX1_n6_uniqueness` reverse direction + bounded forward direction green CI; `lake build` 11.7 s, 2 expected `sorry`.
- **probability adjustment**: −2pp (W2 partial PASS per mission honesty mandate "W2 partial PASS on −2pp"); BUT n=1 corrigendum surface adds +0.5pp (Spec authoring quality concern reaffirmed).
- **net**: from HIGH 65-75% → HIGH **63.5-72.5%** (rounded to 64-73% for reporting).
- Caveat: this adjustment is ONLY for W2 work; W3 (capstone composition + 11 sub-case integration) and W4-W5 (MK port / ZFC fallback) remain the highest-risk gates.

**W2 GREEN-CI gating**: AX1.lean compiles in `lake build` with only 2 sorry warnings; no error. This is a positive signal; W3 unblocked.

---

## §7 raw 71 ≥5 falsifiers (TRANSCEND-tier) — preregistered for W2 work

Five strong falsifiers preregistered for **THIS W2 RESULT** (separate from Spec §8 F-D-1..5 and W1 audit F-W1-1..5):

- **F-W2-AX1-1** — *bounded `interval_cases` + `decide` proof of `AX1_forward_bounded_30` carries hidden non-termination on a future mathlib upgrade*
  - condition: by 2026-05-18 (W3 end), `lake build N6.MechVerif.AX1` exceeds 60 s wall-clock OR fails with `(deterministic) timeout` on the `interval_cases n <;> decide` line
  - experiment: track `lake build` wall-clock weekly; bisect mathlib upgrades that flip the timing
  - expected outcome: does NOT fire — current 6.6 s build is well under any plausible 60 s ceiling; `decide` over (Mathlib σ/φ/divisors) for n ∈ [2,30] is bounded by ~30 × O(n²) divisor enumerations
  - severity if fires: MEDIUM (forces refactor to per-n-case lemma)

- **F-W2-AX1-2** — *the `sorry` in `AX1_forward_tail` cannot be discharged within W3-W4 budget (forces capstone-composition path instead of asymptotic path)*
  - condition: by 2026-06-01 (W5 end), no asymptotic Robin-style bound has been mechanically formalized; only the existing 11 sub-case capstone composition is available
  - experiment: track `N6/MechVerif/AX1Tail.lean` (W3 stub) build status; record `sorry` count weekly
  - expected outcome: FIRES at LOW severity — Robin-style asymptotic in lean4 is non-trivial; capstone-composition path (existing `TheoremB_Capstone.lean` + 11 sub-cases) is the practical fallback
  - severity if fires: LOW (capstone path is already 80% green; tail proof is "nice to have")

- **F-W2-AX1-3** — *Spec §4 unit 1 statement quantifier corrigendum (n ≥ 1 → n ≥ 2) is REJECTED by Spec maintainer (i.e. n=1 must remain in scope)*
  - condition: a Spec amendment proposal cycling n ≥ 1 → n ≥ 2 receives a NACK from the L5 closure-witness pipeline
  - experiment: open a corrigendum proposal; record reviewer disposition
  - expected outcome: does NOT fire (corrigendum is a clear honest-mathematics fix; the L5 witness `T_MK-HW + Φ_HW` AX-1 axiom intent is for non-trivial n)
  - severity if fires: LOW (cosmetic; the correct theorem is `AX1_n6_uniqueness_corrected` either way)

- **F-W2-AX1-4** — *toolchain rc1↔rc2 churn re-occurs and breaks AX1.lean build before W3 milestone*
  - condition: by 2026-05-18, an automatic mathlib master sync moves the project to rc2 again, breaking AX1.lean elaboration
  - experiment: pin `lean-toolchain` AND mathlib rev in lakefile; track weekly `lake build` health
  - expected outcome: may fire MEDIUM if upstream mathlib advances the rc; mitigated by mathlib rev pin (currently soft via `@ "master"`)
  - severity if fires: MEDIUM (forces lakefile to pin mathlib SHA; cross-cutting refactor request)

- **F-W2-AX1-5** — *the `decide` tactic on `(Nat.divisors n).card` is too slow for n in the 25-30 upper range, leaving a hidden `sorry`-equivalent in the green-CI claim*
  - condition: a `lake build N6.MechVerif.AX1` invocation under cold cache exceeds 5 minutes wall-clock, OR `decide` produces a timeout error on a specific n in [25, 30]
  - experiment: cold-cache bench (`rm -rf .lake/build/N6/MechVerif/`; `time lake build N6.MechVerif.AX1`); record wall-clock + per-n timing
  - expected outcome: does NOT fire — current warm-cache 6.6 s is fast; cold-cache mostly mathlib decompress (already 38.8 s for `lake exe cache get`); per-n decide bounded by O(n²) divisor enumeration
  - severity if fires: HIGH (would mean bounded forward proof is theoretically sound but practically fragile)

Five falsifiers preregistered satisfy raw 71 TRANSCEND-tier ≥5 mandate.

---

## §8 raw 91 C3 honest disclosure

- **`sorry` count**: 2 in delivered file. Both are intentional placeholders per Spec §4 unit 1 strategy. No hidden `sorry` and no admit. Mechanical certification: 5 of 7 theorems carry zero `sorry`.
- **`AX1_forward_tail` is the load-bearing `sorry`** — without it, the iff main theorem's forward direction is unproven for n > 30. The honest claim is **"bounded forward direction PASS for n ∈ [2, 30]; unbounded tail UNPROVEN in W2"**, NOT "`thm.AX1_n6_uniqueness` PROVED".
- **n=1 case `sorry`** is the surface of a Spec quantifier corrigendum (§2). The corrected theorem `AX1_n6_uniqueness_corrected` is `sorry`-free at the n=1 level (the n ≥ 2 premise rules it out). The original wording is preserved alongside the corrected wording so the Spec audit trail is visible.
- **Tactic dependency**: `interval_cases` (Mathlib.Tactic.IntervalCases) is a closed-form tactic; the proof of `AX1_forward_bounded_30` is reducible to 29 individual `decide` calls, but the `interval_cases <;> first | rfl | decide` form is the idiomatic mathlib expression. If `interval_cases` is ever deprecated, the proof will need refactor to manual case enumeration.
- **`lake build` ran on production user's machine** (Mac M2; mission §safety jetsam concern). 11.7 s wall-clock; mathlib cache hit; no SIGKILL observed. Cold-cache full mathlib build was attempted ONCE and did NOT complete (toolchain mismatch killed it after ~3:27); recovery via `lake exe cache get` succeeded.
- **Toolchain revert** (rc2 → rc1) was a one-line restoration, not a feature change. mathlib pin remained at rc1; the project file simply re-aligned.
- **W2 nominal window** is 2026-05-05 → 2026-05-18; this delivery is on **day −7** (early by 1 week, consistent with W1 early-start pattern).

---

## §9 cross-links

- **Parent Spec**: [`hexa-weave-formal-mechanical-verification-prep.md`](./hexa-weave-formal-mechanical-verification-prep.md)
- **W1 audit**: [`hexa_weave_mvp_w1_lean4_audit_2026_04_28.md`](./hexa_weave_mvp_w1_lean4_audit_2026_04_28.md)
- **W2 omega-cycle witness**: [`../design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json`](../design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json)
- **lean4 deliverable file**: [`../lean4-n6/N6/MechVerif/AX1.lean`](../lean4-n6/N6/MechVerif/AX1.lean)
- **lean4-n6 sister repo**: [`../lean4-n6/`](../lean4-n6/)
- **mathlib4 audited rev**: `19c497800a418208f973be74c9f5c5901aac2f54`
- **toolchain pin (current)**: `leanprover/lean4:v4.30.0-rc1`
- **F-CL-FORMAL-4 falsifier**: registry entry in L5 witness `raw_71_falsifiers_for_F-MX-6_construction[3]`
