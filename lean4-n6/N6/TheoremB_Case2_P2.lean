-- N6.TheoremB_Case2_P2 : Theorem B case 2 for n = 2q (q odd prime)
-- v4 M3_v4 case 2 partial (2026-04-16 loop 23)
--
-- 목표: q 홀수 소수에 대해 σ(2q)·φ(2q) = 2q·τ(2q) ↔ q = 3
-- 전략:
--   2, q 서로소 → σ/φ/τ multiplicative 적용
--   σ(2q) = 3(q+1), φ(2q) = q-1, τ(2q) = 4
--   σ·φ = 3(q+1)(q-1) = 3(q²-1)
--   n·τ = 8q
--   등식 ⟺ 3q² - 3 = 8q ⟺ 3q² - 8q - 3 = 0 ⟺ (3q+1)(q-3) = 0 ⟺ q = 3

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- 2 와 odd prime q 는 서로소 -/
theorem coprime_2_odd_prime {q : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    Nat.Coprime 2 q := by
  apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hq).mpr
  omega

/-- σ 1 (2q) = 3 * (q + 1) for odd prime q -/
theorem sigma_one_2q {q : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    σ 1 (2 * q) = 3 * (q + 1) := by
  have hcop : Nat.Coprime 2 q := coprime_2_odd_prime hq hq_odd
  have h_mul : σ 1 (2 * q) = σ 1 2 * σ 1 q := by
    rw [(isMultiplicative_sigma (k := 1)).right hcop]
  rw [h_mul, sigma_one_prime Nat.prime_two, sigma_one_prime hq]

/-- φ (2q) = q - 1 for odd prime q -/
theorem totient_2q {q : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    Nat.totient (2 * q) = q - 1 := by
  have hcop : Nat.Coprime 2 q := coprime_2_odd_prime hq hq_odd
  rw [Nat.totient_mul hcop]
  rw [Nat.totient_prime Nat.prime_two, Nat.totient_prime hq]
  ring

/-- τ(2q) = 4 for odd prime q -/
theorem tau_2q {q : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    (Nat.divisors (2 * q)).card = 4 := by
  have hcop : Nat.Coprime 2 q := coprime_2_odd_prime hq hq_odd
  have : σ 0 (2 * q) = σ 0 2 * σ 0 q := by
    rw [(isMultiplicative_sigma (k := 0)).right hcop]
  have h_left : σ 0 (2 * q) = (Nat.divisors (2 * q)).card := by simp [sigma_zero_apply]
  have h_2 : σ 0 2 = 2 := by
    have := sigma_zero_apply_prime_pow (p := 2) (i := 1) Nat.prime_two
    simp at this; exact this
  have h_q : σ 0 q = 2 := by
    have := sigma_zero_apply_prime_pow (p := q) (i := 1) hq
    simp at this; exact this
  rw [← h_left, this, h_2, h_q]

/-- Theorem B case 2 (p=2): σ(2q)·φ(2q) = 2q·τ(2q) ⟺ q = 3 -/
theorem theorem_B_2q {q : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    σ 1 (2 * q) * Nat.totient (2 * q) = (2 * q) * (Nat.divisors (2 * q)).card ↔
    q = 3 := by
  rw [sigma_one_2q hq hq_odd, totient_2q hq hq_odd, tau_2q hq hq_odd]
  -- 3 * (q + 1) * (q - 1) = (2 * q) * 4
  -- LHS: 3(q+1)(q-1) = 3(q² - 1)
  -- RHS: 8q
  -- 등식: 3q² - 3 = 8q ⟺ 3q² - 8q - 3 = 0 ⟺ (3q+1)(q-3) = 0 ⟺ q = 3
  constructor
  · -- → 방향
    intro h
    -- h : 3 * (q + 1) * (q - 1) = 2 * q * 4
    have hq1 : q ≥ 1 := by omega
    have key : (q + 1) * (q - 1) = q * q - 1 := sq_minus_one_factor hq1
    have h2 : 3 * ((q + 1) * (q - 1)) = 2 * q * 4 := by linarith
    rw [key] at h2
    -- h2 : 3 * (q * q - 1) = 8q
    -- Distribute: 3 * (q² - 1) = 3q² - 3 in Nat (since q² ≥ 1)
    have h3 : q * q ≥ 1 := by nlinarith
    have h_expand : 3 * (q * q - 1) = 3 * (q * q) - 3 * 1 := Nat.mul_sub 3 _ _
    rw [h_expand] at h2
    simp at h2
    -- h2: 3 * (q * q) - 3 = 2 * q * 4 = 8q
    -- So 3q² = 8q + 3
    have h4 : 3 * (q * q) = 8 * q + 3 := by omega
    -- For q ≥ 3, 3q² = 8q + 3 forces q = 3
    by_contra hne
    have hq_ge_5 : q ≥ 5 := by
      rcases (show q = 3 ∨ q = 4 ∨ q ≥ 5 by omega) with h' | h' | h'
      · exact absurd h' hne
      · exfalso; subst h'; exact absurd hq (by decide)
      · exact h'
    -- q ≥ 5: 3q² ≥ 75, 8q + 3 ≤ 8q + 3. Need 3q² > 8q + 3.
    -- q = 5: 75 vs 43. Diff = 32
    -- q ≥ 5: 3q² - 8q - 3 = q(3q - 8) - 3 ≥ 5·7 - 3 = 32 > 0
    nlinarith
  · intro h; subst h; decide

/-- 확인: q = 3 에서 등식 -/
example : σ 1 (2 * 3) * Nat.totient (2 * 3) = (2 * 3) * (Nat.divisors (2 * 3)).card := by
  decide

/-- 확인: q = 5 에서 등식 불성립 -/
example : σ 1 (2 * 5) * Nat.totient (2 * 5) ≠ (2 * 5) * (Nat.divisors (2 * 5)).card := by
  decide

-- Theorem B 진전:
--  Case 1 (prime p): ✓ theorem_B_prime_case
--  Case 2a (n = 2q, q odd prime): ✓ theorem_B_2q (본 파일)
--  Case 2b (n = pq, p, q odd distinct primes): 후속
--  Case 3 (n = p^a, a ≥ 2): 후속

end N6Mathlib
