-- N6.MechVerif.AX1 : thm.AX1_n6_uniqueness — first mechanical attempt
-- W2 deliverable for proposals/hexa-weave-formal-mechanical-verification-prep.md §4 unit 1.
-- Date: 2026-04-28 (cycle 4 fan-out 4/5).
--
-- W6 cycle 8 (2026-04-28) UPDATE: `AX1Eq` definition + the named axiom
-- `axiom_robin_hardy_wright_ax1_tail` are RELOCATED to
-- `N6/MechVerif/Foundation/Axioms.lean` (single source of truth for all 7
-- W4-W6 named axioms). This file imports them; no semantic change.
-- F-W5-AX2-1 partial resolution.
--
-- Mission-text alias path: lean4-n6/HexaWeave/AX1NUniqueness.lean
-- Canonical Spec §6 path : lean4-n6/N6/MechVerif/AX1.lean  ← THIS FILE
-- Choice rationale: keep `lean_lib N6` lakefile unchanged.
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
--   ⟹  (equality → n = 6):   bounded `decide` for n ≤ 100  (W13 cycle 20);
--                              for n > 100, asymptotic tail bound named axiom.
--
-- W13 cycle 20 (2026-04-28): bounded threshold 50 → 100 via interval_cases
-- (axiom surface n>50 → n>100; axiom count UNCHANGED at 7 — PARTIAL outcome).
-- mathlib4 has NO Robin/Hardy-Wright/Wigert asymptotic; full mechanical
-- conversion deferred. raw 91 C3 honest disclosure in Foundation/Axioms.lean
-- §3 axiom-block.

import N6.MechVerif.Foundation.Axioms
import Mathlib.Tactic.IntervalCases

namespace N6Mathlib.MechVerif

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-! ## Reverse direction (n = 6 → equality)

    Direct `decide` on Mathlib σ/φ/divisors definitions for n = 6.
    `AX1Eq` is now defined in Foundation/Axioms.lean. -/

theorem AX1_reverse_n6 : AX1Eq 6 := by
  unfold AX1Eq; decide

/-- σ(6) = 12, φ(6) = 2, τ(6) = 4 — concrete witness. -/
theorem AX1_n6_witness :
    σ 1 6 = 12 ∧ Nat.totient 6 = 2 ∧ (Nat.divisors 6).card = 4 := by
  refine ⟨?_, ?_, ?_⟩ <;> decide

/-! ## Forward direction (equality → n = 6) -/

/-- Bounded forward: for `n ∈ [2, 30]` with `AX1Eq n`, we have `n = 6`. -/
theorem AX1_forward_bounded_30 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 30)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> first | rfl | (exfalso; revert h_eq; decide)

/-- Extended bounded forward: for `n ∈ [2, 50]` with `AX1Eq n`, we have `n = 6`. -/
theorem AX1_forward_bounded_50 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 50)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> first | rfl | (exfalso; revert h_eq; decide)

/-- W13 cycle-20 bounded forward: for `n ∈ [2, 100]` with `AX1Eq n`, we have
    `n = 6`. Extends bounded threshold 50 → 100; the named axiom's quantifier
    is correspondingly hardened to `100 < n`. raw 91 C3 honest: mathlib4 has
    no Robin/Hardy-Wright/Wigert asymptotic results, so full mechanical
    conversion is infeasible at cycle 20 — this is a PARTIAL surface-area
    reduction (axiom count UNCHANGED at 7). -/
theorem AX1_forward_bounded_100 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 100)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> first | rfl | (exfalso; revert h_eq; decide)

/- Helpers for cycle-21 stage-1 chunked dispatch: bounded forward over a single
   100-wide window. We isolate the interval_cases call to a 100-element
   window. The `set_option maxRecDepth` bump is required because
   `interval_cases n` with hypothesis bounds on higher absolute values
   (n > 100) elaborates a deeper goal stack than the n ≥ 2 case used in
   `AX1_forward_bounded_100`. -/

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_200 (n : ℕ) (h_lo : 100 < n) (h_hi : n ≤ 200)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_300 (n : ℕ) (h_lo : 200 < n) (h_hi : n ≤ 300)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_400 (n : ℕ) (h_lo : 300 < n) (h_hi : n ≤ 400)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_500 (n : ℕ) (h_lo : 400 < n) (h_hi : n ≤ 500)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_600 (n : ℕ) (h_lo : 500 < n) (h_hi : n ≤ 600)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_700 (n : ℕ) (h_lo : 600 < n) (h_hi : n ≤ 700)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_800 (n : ℕ) (h_lo : 700 < n) (h_hi : n ≤ 800)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_900 (n : ℕ) (h_lo : 800 < n) (h_hi : n ≤ 900)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1000 (n : ℕ) (h_lo : 900 < n) (h_hi : n ≤ 1000)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1100 (n : ℕ) (h_lo : 1000 < n) (h_hi : n ≤ 1100)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1200 (n : ℕ) (h_lo : 1100 < n) (h_hi : n ≤ 1200)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1300 (n : ℕ) (h_lo : 1200 < n) (h_hi : n ≤ 1300)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1400 (n : ℕ) (h_lo : 1300 < n) (h_hi : n ≤ 1400)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1500 (n : ℕ) (h_lo : 1400 < n) (h_hi : n ≤ 1500)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1600 (n : ℕ) (h_lo : 1500 < n) (h_hi : n ≤ 1600)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1700 (n : ℕ) (h_lo : 1600 < n) (h_hi : n ≤ 1700)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1800 (n : ℕ) (h_lo : 1700 < n) (h_hi : n ≤ 1800)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_1900 (n : ℕ) (h_lo : 1800 < n) (h_hi : n ≤ 1900)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2000 (n : ℕ) (h_lo : 1900 < n) (h_hi : n ≤ 2000)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2100 (n : ℕ) (h_lo : 2000 < n) (h_hi : n ≤ 2100)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2200 (n : ℕ) (h_lo : 2100 < n) (h_hi : n ≤ 2200)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2300 (n : ℕ) (h_lo : 2200 < n) (h_hi : n ≤ 2300)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2400 (n : ℕ) (h_lo : 2300 < n) (h_hi : n ≤ 2400)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2500 (n : ℕ) (h_lo : 2400 < n) (h_hi : n ≤ 2500)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2600 (n : ℕ) (h_lo : 2500 < n) (h_hi : n ≤ 2600)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2700 (n : ℕ) (h_lo : 2600 < n) (h_hi : n ≤ 2700)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2800 (n : ℕ) (h_lo : 2700 < n) (h_hi : n ≤ 2800)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_2900 (n : ℕ) (h_lo : 2800 < n) (h_hi : n ≤ 2900)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

set_option maxRecDepth 4000 in
set_option maxHeartbeats 4000000 in
private theorem AX1_forward_window_3000 (n : ℕ) (h_lo : 2900 < n) (h_hi : n ≤ 3000)
    (h_eq : AX1Eq n) : n = 6 := by
  unfold AX1Eq at h_eq
  interval_cases n <;> (exfalso; revert h_eq; decide)

/-- W13 cycle-22 stage-2a bounded forward: for `n ∈ [2, 2000]` with `AX1Eq n`,
    we have `n = 6`. Extends bounded threshold 1000 → 2000; chains 10
    additional 100-wide windows on top of the cycle-21 1000 dispatch. raw 91
    C3 honest: still RH-conditional regime (5040 unconditional threshold not
    yet reached); axiom count UNCHANGED at 1. -/
theorem AX1_forward_bounded_2000 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 2000)
    (h_eq : AX1Eq n) : n = 6 := by
  by_cases h1000 : n ≤ 1000
  · exact AX1_forward_bounded_1000 n h_lo h1000 h_eq
  · have h1000' : 1000 < n := Nat.lt_of_not_le h1000
    by_cases h1100 : n ≤ 1100
    · exact AX1_forward_window_1100 n h1000' h1100 h_eq
    · have h1100' : 1100 < n := Nat.lt_of_not_le h1100
      by_cases h1200 : n ≤ 1200
      · exact AX1_forward_window_1200 n h1100' h1200 h_eq
      · have h1200' : 1200 < n := Nat.lt_of_not_le h1200
        by_cases h1300 : n ≤ 1300
        · exact AX1_forward_window_1300 n h1200' h1300 h_eq
        · have h1300' : 1300 < n := Nat.lt_of_not_le h1300
          by_cases h1400 : n ≤ 1400
          · exact AX1_forward_window_1400 n h1300' h1400 h_eq
          · have h1400' : 1400 < n := Nat.lt_of_not_le h1400
            by_cases h1500 : n ≤ 1500
            · exact AX1_forward_window_1500 n h1400' h1500 h_eq
            · have h1500' : 1500 < n := Nat.lt_of_not_le h1500
              by_cases h1600 : n ≤ 1600
              · exact AX1_forward_window_1600 n h1500' h1600 h_eq
              · have h1600' : 1600 < n := Nat.lt_of_not_le h1600
                by_cases h1700 : n ≤ 1700
                · exact AX1_forward_window_1700 n h1600' h1700 h_eq
                · have h1700' : 1700 < n := Nat.lt_of_not_le h1700
                  by_cases h1800 : n ≤ 1800
                  · exact AX1_forward_window_1800 n h1700' h1800 h_eq
                  · have h1800' : 1800 < n := Nat.lt_of_not_le h1800
                    by_cases h1900 : n ≤ 1900
                    · exact AX1_forward_window_1900 n h1800' h1900 h_eq
                    · have h1900' : 1900 < n := Nat.lt_of_not_le h1900
                      exact AX1_forward_window_2000 n h1900' h_hi h_eq

/-- W13 cycle-26 stage-3a bounded forward: for `n ∈ [2, 3000]` with `AX1Eq n`,
    we have `n = 6`. Extends bounded threshold 2000 → 3000 by chaining 10
    additional 100-wide windows (2100..3000) on top of `AX1_forward_bounded_2000`.
    raw 91 C3 honest: still RH-conditional regime (5040 unconditional threshold
    not yet reached); 5040 stretch goal deferred (cycle 26 partial: 2000→3000
    only, build time + heartbeat budget); axiom count UNCHANGED at 1. -/
theorem AX1_forward_bounded_3000 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 3000)
    (h_eq : AX1Eq n) : n = 6 := by
  by_cases h2000 : n ≤ 2000
  · exact AX1_forward_bounded_2000 n h_lo h2000 h_eq
  · have h2000' : 2000 < n := Nat.lt_of_not_le h2000
    by_cases h2100 : n ≤ 2100
    · exact AX1_forward_window_2100 n h2000' h2100 h_eq
    · have h2100' : 2100 < n := Nat.lt_of_not_le h2100
      by_cases h2200 : n ≤ 2200
      · exact AX1_forward_window_2200 n h2100' h2200 h_eq
      · have h2200' : 2200 < n := Nat.lt_of_not_le h2200
        by_cases h2300 : n ≤ 2300
        · exact AX1_forward_window_2300 n h2200' h2300 h_eq
        · have h2300' : 2300 < n := Nat.lt_of_not_le h2300
          by_cases h2400 : n ≤ 2400
          · exact AX1_forward_window_2400 n h2300' h2400 h_eq
          · have h2400' : 2400 < n := Nat.lt_of_not_le h2400
            by_cases h2500 : n ≤ 2500
            · exact AX1_forward_window_2500 n h2400' h2500 h_eq
            · have h2500' : 2500 < n := Nat.lt_of_not_le h2500
              by_cases h2600 : n ≤ 2600
              · exact AX1_forward_window_2600 n h2500' h2600 h_eq
              · have h2600' : 2600 < n := Nat.lt_of_not_le h2600
                by_cases h2700 : n ≤ 2700
                · exact AX1_forward_window_2700 n h2600' h2700 h_eq
                · have h2700' : 2700 < n := Nat.lt_of_not_le h2700
                  by_cases h2800 : n ≤ 2800
                  · exact AX1_forward_window_2800 n h2700' h2800 h_eq
                  · have h2800' : 2800 < n := Nat.lt_of_not_le h2800
                    by_cases h2900 : n ≤ 2900
                    · exact AX1_forward_window_2900 n h2800' h2900 h_eq
                    · have h2900' : 2900 < n := Nat.lt_of_not_le h2900
                      exact AX1_forward_window_3000 n h2900' h_hi h_eq

/-- W13 cycle-21 stage-1 bounded forward: for `n ∈ [2, 1000]` with `AX1Eq n`,
    we have `n = 6`. Extends bounded threshold 100 → 1000; the named axiom's
    quantifier is correspondingly hardened to `1000 < n`. The proof uses a
    9-window chunked dispatch (each window is 100-wide, isolating
    `interval_cases` to its known-good [n, n+100] regime) to avoid
    `maxRecDepth` blow-up of a single 900-wide interval_cases call. raw 91 C3
    honest: mathlib4 still has no Robin/Hardy-Wright/Wigert asymptotic; this
    is a PARTIAL surface-area reduction (axiom count UNCHANGED at 7). -/
theorem AX1_forward_bounded_1000 (n : ℕ) (h_lo : 2 ≤ n) (h_hi : n ≤ 1000)
    (h_eq : AX1Eq n) : n = 6 := by
  by_cases h100 : n ≤ 100
  · exact AX1_forward_bounded_100 n h_lo h100 h_eq
  · have h100' : 100 < n := Nat.lt_of_not_le h100
    by_cases h200 : n ≤ 200
    · exact AX1_forward_window_200 n h100' h200 h_eq
    · have h200' : 200 < n := Nat.lt_of_not_le h200
      by_cases h300 : n ≤ 300
      · exact AX1_forward_window_300 n h200' h300 h_eq
      · have h300' : 300 < n := Nat.lt_of_not_le h300
        by_cases h400 : n ≤ 400
        · exact AX1_forward_window_400 n h300' h400 h_eq
        · have h400' : 400 < n := Nat.lt_of_not_le h400
          by_cases h500 : n ≤ 500
          · exact AX1_forward_window_500 n h400' h500 h_eq
          · have h500' : 500 < n := Nat.lt_of_not_le h500
            by_cases h600 : n ≤ 600
            · exact AX1_forward_window_600 n h500' h600 h_eq
            · have h600' : 600 < n := Nat.lt_of_not_le h600
              by_cases h700 : n ≤ 700
              · exact AX1_forward_window_700 n h600' h700 h_eq
              · have h700' : 700 < n := Nat.lt_of_not_le h700
                by_cases h800 : n ≤ 800
                · exact AX1_forward_window_800 n h700' h800 h_eq
                · have h800' : 800 < n := Nat.lt_of_not_le h800
                  by_cases h900 : n ≤ 900
                  · exact AX1_forward_window_900 n h800' h900 h_eq
                  · have h900' : 900 < n := Nat.lt_of_not_le h900
                    exact AX1_forward_window_1000 n h900' h_hi h_eq

/-! ## W3 cycle-7 axiomatic Robin/Hardy-Wright tail (raw 91 C3 honest disclosure)

    W13 cycle-20 UPDATE: bounded threshold extended 50 → 100; axiom hardened
    to `100 < n`. The named axiom `axiom_robin_hardy_wright_ax1_tail` is
    declared in `N6/MechVerif/Foundation/Axioms.lean` (W6 cycle 8 relocation).
    Cited literature unchanged:
      * Robin (1984), J. Math. Pures Appl. 63, 187-213.
      * Hardy & Wright, Theorems 322 (σ asymptotic), 328 (φ asymptotic).
      * Wigert (1907), Arkiv för Mat. 3, 1-9 (τ(n) = n^o(1)). -/

/-- Unbounded tail (n > 3000, W13 cycle 26 stage 3a): discharged by
    `axiom_robin_hardy_wright_ax1_tail`. raw 91 C3: tail surface narrowed
    cycle 21 (n>1000) → cycle 22 (n>2000) → cycle 26 (n>3000). The 5040
    Robin unconditional threshold remains the long-horizon target; cycle 26
    achieved 60% of that distance with reasonable build budget. -/
theorem AX1_forward_tail (n : ℕ) (h_big : 3000 < n) (h_eq : AX1Eq n) : n = 6 :=
  absurd h_eq (axiom_robin_hardy_wright_ax1_tail n h_big)

/-- **`thm.AX1_n6_uniqueness`** — main W2 statement.

    W3 UPDATE (cycle 6, 2026-04-28): premise hardened to `n ≥ 2`.
    W3 UPDATE (cycle 7, 2026-04-28): forward tail discharged by named axiom.
    W6 UPDATE (cycle 8, 2026-04-28): axiom relocated to Foundation/Axioms.lean
    (no semantic change).
    W13 UPDATE (cycle 20, 2026-04-28): bounded threshold 50 → 100; axiom
    quantifier hardened to `100 < n`. -/
theorem AX1_n6_uniqueness :
    ∀ n : ℕ, 2 ≤ n →
      (σ 1 n * Nat.totient n = n * (Nat.divisors n).card ↔ n = 6) := by
  intro n h_lo
  constructor
  · -- forward: equality → n = 6
    intro h_eq
    by_cases h_hi : n ≤ 3000
    · exact AX1_forward_bounded_3000 n h_lo h_hi h_eq
    · exact AX1_forward_tail n (Nat.lt_of_not_le h_hi) h_eq
  · -- reverse: n = 6 → equality
    intro h_n6
    subst h_n6
    exact AX1_reverse_n6

/-- **n=1 counter-example to the un-corrected `n ≥ 1` form**. -/
theorem AX1_n6_uniqueness_n1_counterexample :
    AX1Eq 1 ∧ (1 : ℕ) ≠ 6 := by
  refine ⟨?_, ?_⟩
  · unfold AX1Eq; decide
  · decide

/-! ## Spec §4 unit 1 corrigendum surfaced in W2 (W2 falsifier F-W2-AX1-1). -/

/-- Backwards-compat alias: `AX1_n6_uniqueness_corrected` ≡ `AX1_n6_uniqueness`. -/
theorem AX1_n6_uniqueness_corrected :
    ∀ n : ℕ, 2 ≤ n →
      (σ 1 n * Nat.totient n = n * (Nat.divisors n).card ↔ n = 6) :=
  AX1_n6_uniqueness

end N6Mathlib.MechVerif
