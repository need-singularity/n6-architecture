-- N6.TheoremB_PrimeCase : Theorem B 의 prime case 실제 증명
-- v4 M3_v4 (2026-04-16 loop 22): Mathlib 기반 sorry-없는 증명

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- σ 1 p = p + 1 for prime p (from Mathlib's sigma_one_apply_prime_pow) -/
theorem sigma_one_prime {p : ℕ} (hp : p.Prime) : σ 1 p = p + 1 := by
  have h : σ 1 (p ^ 1) = ∑ k ∈ Finset.range 2, p ^ k :=
    sigma_one_apply_prime_pow hp
  simp at h
  linarith

/-- τ(p) = 2 for prime p, via σ 0 -/
theorem tau_prime {p : ℕ} (hp : p.Prime) : (Nat.divisors p).card = 2 := by
  have : σ 0 p = (Nat.divisors p).card := by simp [sigma_zero_apply]
  rw [← this]
  have h : σ 0 (p ^ 1) = 1 + 1 := sigma_zero_apply_prime_pow hp
  simp at h
  exact h

/-- (p+1)(p-1) = p² - 1 for p ≥ 1 in Nat -/
theorem sq_minus_one_factor {p : ℕ} (hp : 1 ≤ p) : (p + 1) * (p - 1) = p * p - 1 := by
  cases p with
  | zero => omega
  | succ n =>
    simp only [Nat.succ_sub_one]
    -- (n + 1 + 1) * n = (n + 1) * (n + 1) - 1
    -- LHS = (n+2)*n = n² + 2n
    -- RHS = (n+1)² - 1 = n² + 2n + 1 - 1 = n² + 2n
    have expand_lhs : (n + 1 + 1) * n = n * n + 2 * n := by ring
    have expand_rhs : (n + 1) * (n + 1) = n * n + 2 * n + 1 := by ring
    rw [expand_lhs, expand_rhs]
    omega

/-- Theorem B prime case: ∀ prime p, σ(p)·φ(p) ≠ p·τ(p) -/
theorem theorem_B_prime_case {p : ℕ} (hp : p.Prime) :
    σ 1 p * Nat.totient p ≠ p * (Nat.divisors p).card := by
  rw [sigma_one_prime hp, Nat.totient_prime hp, tau_prime hp]
  -- Goal: (p + 1) * (p - 1) ≠ p * 2
  have hp2 : 2 ≤ p := hp.two_le
  rcases (show p = 2 ∨ p ≥ 3 by omega) with hp_eq | hp_ge
  · subst hp_eq; decide
  · -- p ≥ 3: (p+1)(p-1) > 2p
    intro h
    -- (p + 1) * (p - 1) = p * p - 1
    have key : (p + 1) * (p - 1) = p * p - 1 := sq_minus_one_factor (by omega)
    rw [key] at h
    -- Now h : p * p - 1 = p * 2
    -- For p ≥ 3: p * p ≥ 9, so p * p - 1 ≥ 8, but p * 2 ≥ 6
    -- Specifically: p * p - 1 = 2p ⟺ p * p = 2p + 1
    -- p ≥ 3 ⟹ p * p ≥ p * 3 = 3p > 2p + 1 (since p ≥ 3 ⟹ 3p ≥ 9 > 2p + 1 when p ≥ 2)
    have h1 : p * p ≥ 3 * p := by nlinarith
    have h2 : 3 * p > 2 * p + 1 := by omega
    -- p * p ≥ 3p > 2p + 1 = p * 2 + 1
    -- So p * p - 1 ≥ 3p - 1 > 2p - 1 + 1 = 2p = p * 2
    -- Contradicts h : p * p - 1 = p * 2
    have : p * p ≥ 1 := by nlinarith
    omega

/-- 확인: 작은 prime 에서 실제 적용 -/
example : σ 1 2 * Nat.totient 2 ≠ 2 * (Nat.divisors 2).card :=
  theorem_B_prime_case Nat.prime_two

example : σ 1 3 * Nat.totient 3 ≠ 3 * (Nat.divisors 3).card :=
  theorem_B_prime_case Nat.prime_three

example : σ 1 5 * Nat.totient 5 ≠ 5 * (Nat.divisors 5).card :=
  theorem_B_prime_case (by decide : (5 : ℕ).Prime)

example : σ 1 7 * Nat.totient 7 ≠ 7 * (Nat.divisors 7).card :=
  theorem_B_prime_case (by decide : (7 : ℕ).Prime)

example : σ 1 97 * Nat.totient 97 ≠ 97 * (Nat.divisors 97).card :=
  theorem_B_prime_case (by decide : (97 : ℕ).Prime)

-- Theorem B 진전:
-- Case 1 (prime p): ✓ 증명 완료 (sorry 없음)
-- Case 2 (p·q, p ≠ q): 후속
-- Case 3 (p^a, a ≥ 2): 후속

end N6Mathlib
