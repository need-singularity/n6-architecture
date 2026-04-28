-- N6.MechVerif.AX1 : thm.AX1_n6_uniqueness — first mechanical attempt
-- W2 deliverable for proposals/hexa-weave-formal-mechanical-verification-prep.md §4 unit 1.
-- Date: 2026-04-28 (cycle 4 fan-out 4/5).
--
-- Mission-text alias path: lean4-n6/HexaWeave/AX1NUniqueness.lean
-- Canonical Spec §6 path : lean4-n6/N6/MechVerif/AX1.lean  ← THIS FILE
-- Choice rationale: keep `lean_lib N6` lakefile unchanged (no cross-cutting refactor
-- without user approval). Mission and Spec §6 both agree on theorem semantics; only
-- the file path differs. See report W2_2026_04_28 for disclosure.
--
-- Theorem statement (Spec §4 unit 1, with W1 audit corrigendum #3 applied —
-- there is NO `Nat.ArithmeticFunction.tau` symbol in mathlib4 master rev
-- `19c497800a418208f973be74c9f5c5901aac2f54`; we use `(Nat.divisors n).card`
-- which equals `σ 0 n` per `sigma_zero_apply`):
--
--   ∀ n : ℕ, n ≥ 1 →
--     ( σ 1 n * Nat.totient n = n * (Nat.divisors n).card  ↔  n = 6 )
--
-- Proof strategy per Spec §4 unit 1:
--   ⟸  (n = 6 → equality):   `decide` on Mathlib σ/φ/divisors definitions.
--   ⟹  (equality → n = 6):   bounded `decide` for n ≤ 30 (TheoremB_Capstone
--                              `theorem_B_bounded_30` already covers this);
--                              for n > 30, asymptotic tail bound:
--                                σ(n)·φ(n) > n·τ(n) for "most" n (Robin-style),
--                                and equality forces specific divisor structure.
--                              W2 RESULT: tail bound proof carries `sorry`.

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Tactic.IntervalCases

namespace N6Mathlib.MechVerif

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- Predicate: the AX-1 equality `σ(n)·φ(n) = n·τ(n)`. -/
def AX1Eq (n : ℕ) : Prop :=
  σ 1 n * Nat.totient n = n * (Nat.divisors n).card

/-- AX-1 equality is `Decidable` for every concrete `n` (used by `decide`). -/
instance (n : ℕ) : Decidable (AX1Eq n) := by
  unfold AX1Eq; exact inferInstance

/-! ## Reverse direction (n = 6 → equality)

    Direct `decide` on Mathlib σ/φ/divisors definitions for the concrete
    instance n = 6. -/

theorem AX1_reverse_n6 : AX1Eq 6 := by
  unfold AX1Eq; decide

/-- σ(6) = 12, φ(6) = 2, τ(6) = 4 — concrete witness. -/
theorem AX1_n6_witness :
    σ 1 6 = 12 ∧ Nat.totient 6 = 2 ∧ (Nat.divisors 6).card = 4 := by
  refine ⟨?_, ?_, ?_⟩ <;> decide

/-! ## Forward direction (equality → n = 6)

    Bounded portion (n ≤ 30) is dispatched via the existing capstone
    `theorem_B_bounded_30`. The unbounded portion (n > 30) is the W2
    `sorry` placeholder per raw 91 C3. -/

/-- Bounded forward: for `n ∈ [2, 30]` with `AX1Eq n`, we have `n = 6`.
    Proof: `interval_cases` enumerates n ∈ [2,30] and `decide` rules out
    every n ≠ 6 by direct computation on Mathlib σ/φ/divisors. -/
theorem AX1_forward_bounded_30 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 30)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> first | rfl | (exfalso; revert h_eq; decide)

/-- Extended bounded forward: for `n ∈ [2, 50]` with `AX1Eq n`, we have `n = 6`.
    W3 cycle-6: bounded threshold pushed from 30 → 50, reducing tail residual.
    Proof identical to `AX1_forward_bounded_30` with widened `interval_cases`. -/
theorem AX1_forward_bounded_50 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 50)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> first | rfl | (exfalso; revert h_eq; decide)

/-- Unbounded tail (n > 50): asymptotic argument PLACEHOLDER.
    Spec §4 unit 1 calls for a Robin-style tail bound. W3 cycle-6 carries `sorry`.

    W3 cycle-6 update: tail threshold raised 30 → 50 by widened `decide`
    (`AX1_forward_bounded_50`). Residual gap is n > 50 only.

    mathlib4 master rev `19c4978` does NOT contain Robin's theorem
    (verified by grep cycle-6 2026-04-28); composition path uses the
    existing TheoremB_Case3 / TheoremB_Case4{a,b,c}_* shards. -/
theorem AX1_forward_tail (n : ℕ) (h_big : 50 < n) (h_eq : AX1Eq n) : n = 6 := by
  -- raw 91 C3: tail asymptotic NOT mechanically proved in W3 cycle-6.
  -- Outline (deferred to W3 / W4 capstone composition):
  --   1. For n with ω(n) ≥ 5 prime factors, σ(n)·φ(n) > n·τ(n) by
  --      Robin's bound σ(n)/n ≥ Π_p (1 + 1/p) and corresponding φ bound.
  --   2. For n = p^a, p prime, a ≥ 1: case-analysis (TheoremB_Case3
  --      already handles a ≥ 2; primes covered by TheoremB_PrimeCase).
  --   3. For n = p·q (two distinct primes): TheoremB_Case2_OddOdd +
  --      TheoremB_Case2_P2 already give n=6 uniquely.
  --   4. For n = p^a·q^b: TheoremB_Case4b family covers (a,b) ≠ (1,1).
  --   5. For n = p·q·r (three distinct primes): TheoremB_Case4_ThreePrimes.
  --   6. Higher ω(n) ≥ 4..9 cases: TheoremB_Case4c_* shards already exist.
  --   ⇒ Capstone composition is the W3 task; tail bound for ω(n) ≥ 10
  --     is the residual.
  sorry

/-- **`thm.AX1_n6_uniqueness`** — main W2 statement.

    W3 UPDATE (cycle 6, 2026-04-28): premise hardened to `n ≥ 2` per Spec §4
    unit 1 corrigendum (W2 falsifier F-W2-AX1-1). The original `n ≥ 1` form
    is unprovable: at n=1, σ(1)·φ(1) = 1·1 = 1 = 1·τ(1), so LHS holds but
    RHS (n = 6) fails — the iff is FALSE at n=1.

    Per Spec §4 unit 1 (corrected): forward direction is partial (bounded
    ≤ 30 PASS, unbounded tail `sorry`); reverse direction PASS via `decide`.
    See `AX1_n6_uniqueness_n1_counterexample` below for the retired n=1 case. -/
theorem AX1_n6_uniqueness :
    ∀ n : ℕ, 2 ≤ n →
      (σ 1 n * Nat.totient n = n * (Nat.divisors n).card ↔ n = 6) := by
  intro n h_lo
  constructor
  · -- forward: equality → n = 6
    intro h_eq
    by_cases h_hi : n ≤ 50
    · exact AX1_forward_bounded_50 n h_lo h_hi h_eq
    · exact AX1_forward_tail n (Nat.lt_of_not_le h_hi) h_eq
  · -- reverse: n = 6 → equality
    intro h_n6
    subst h_n6
    exact AX1_reverse_n6

/-- **n=1 counter-example to the un-corrected `n ≥ 1` form** —
    explicit witness that the original Spec §4 unit 1 quantifier was wrong.
    σ(1)·φ(1) = 1 = 1·τ(1), but 1 ≠ 6. -/
theorem AX1_n6_uniqueness_n1_counterexample :
    AX1Eq 1 ∧ (1 : ℕ) ≠ 6 := by
  refine ⟨?_, ?_⟩
  · unfold AX1Eq; decide
  · decide

/-! ## Spec §4 unit 1 corrigendum surfaced in W2

    The stated quantifier `∀ n : ℕ, n ≥ 1` admits n = 1 as a counter-example
    to the iff (LHS holds trivially, RHS false). Recommended Spec amendment:
        `∀ n : ℕ, n ≥ 2 →  AX1Eq n  ↔  n = 6`
    With `n ≥ 2`, the bounded `decide` for n ∈ [2, 30] cleanly rules out
    n = 1, and the iff holds. This is reported as W2 falsifier F-W2-AX1-1
    (low severity, statement-only correction). -/

/-- Backwards-compat alias: prior W2 cycles named the n ≥ 2 form
    `AX1_n6_uniqueness_corrected`. Now identical to `AX1_n6_uniqueness`
    after the W3 cycle-6 hardening of the main theorem's premise. -/
theorem AX1_n6_uniqueness_corrected :
    ∀ n : ℕ, 2 ≤ n →
      (σ 1 n * Nat.totient n = n * (Nat.divisors n).card ↔ n = 6) :=
  AX1_n6_uniqueness

end N6Mathlib.MechVerif
