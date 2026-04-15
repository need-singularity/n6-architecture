-- N6.TheoremB_Case4b_TwoPowOddPow : Theorem B case 4b(iii) (n = 2^a·q^b, a ≥ 2, q odd)
-- v4 M3_v4 case 4b(iii) (2026-04-16 loop 29)
--
-- 목표: a ≥ 2, q ≥ 3 odd prime, b ≥ 1 인 모든 n = 2^a · q^b 에 대하여
--       σ(n)·φ(n) ≠ n·τ(n)
--
-- 전략 (weak bounds 곱):
--   σφ(2^a) = 2^(a-1)·(2^(a+1) - 1)
--   σφ(q^b) = q^(b-1)·(q^(b+1) - 1)
--   weak bound 1: 3·2^(a+1) ≥ 7(a+1) + 3 for a ≥ 2  (equality at a=2)
--   weak bound 2: 3·q^(b+1) ≥ 4q(b+1) + 3 for q≥3 odd, b≥1  (equality at (q,b)=(3,1))
--   이를 통해 9·(2^(a+1)-1)·(q^(b+1)-1) ≥ 7(a+1)·4q(b+1) = 28·q·(a+1)(b+1)
--   등식 가정: (2^(a+1)-1)(q^(b+1)-1) = 2q(a+1)(b+1) (cancel 2^(a-1)·q^(b-1))
--           → 18q(a+1)(b+1) ≥ 28q(a+1)(b+1) → 18 ≥ 28 모순 (q,a,b 양수)

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case3_PrimePow
import N6.TheoremB_Case4b_TwoPrimePow
import N6.TheoremB_Case4b_OddPrimePowers

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- Weak bound 2: 3·q^(b+1) ≥ 4q(b+1) + 3 for q ≥ 3 odd prime, b ≥ 1.
    (equality at (q,b)=(3,1)) — case 4b(i) 의 strict 버전 ≥ 완화 -/
theorem key_ineq_odd_weak {q b : ℕ} (hq : q ≥ 3) (hb : b ≥ 1) :
    3 * q ^ (b + 1) ≥ 4 * q * (b + 1) + 3 := by
  rcases Nat.lt_or_ge b 2 with hlt | hge
  · -- b = 1
    have hb1 : b = 1 := by omega
    subst hb1
    -- Goal: 3·q² ≥ 4q·2 + 3 = 8q + 3, i.e., 3q² - 8q - 3 ≥ 0, i.e., (3q+1)(q-3) ≥ 0
    have h_pow : q ^ (1 + 1) = q * q := by ring
    rw [h_pow]
    nlinarith
  · -- b ≥ 2: case 4b(i) 의 strict 버전
    have := key_ineq_4bi hq hge
    omega

/-- Weak bound 1: 3·2^(a+1) ≥ 7(a+1) + 3 for a ≥ 2. (equality at a=2) -/
theorem key_ineq_2pow_weak {a : ℕ} (ha : a ≥ 2) :
    3 * 2 ^ (a + 1) ≥ 7 * (a + 1) + 3 := by
  induction a, ha using Nat.le_induction with
  | base =>
    -- a = 2: 3·2³ = 24, 7·3+3 = 24
    decide
  | succ n hn ih =>
    -- ih: 3·2^(n+1) ≥ 7(n+1) + 3
    -- Goal: 3·2^(n+2) ≥ 7(n+2) + 3
    -- 3·2^(n+2) = 2·(3·2^(n+1)) ≥ 2·(7(n+1)+3) = 14(n+1)+6 = 14n+20
    -- Need: 14n+20 ≥ 7(n+2)+3 = 7n+17 → 7n+3 ≥ 0 ✓
    have h_exp : 3 * 2 ^ (n + 1 + 1) = 2 * (3 * 2 ^ (n + 1)) := by
      have : 2 ^ (n + 1 + 1) = 2 * 2 ^ (n + 1) := by ring
      rw [this]; ring
    rw [h_exp]
    omega

/-- Coprime 2^a and q^b for q odd prime -/
theorem coprime_2pow_odd_pow {q a b : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    Nat.Coprime (2 ^ a) (q ^ b) := by
  have h2q : Nat.Coprime 2 q := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hq).mpr
    omega
  exact (h2q.pow_left a).pow_right b

/-- σ₁(2^a · q^b) = σ₁(2^a) · σ₁(q^b) for q odd prime -/
theorem sigma_one_2pow_odd_pow {q a b : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    σ 1 (2 ^ a * q ^ b) = σ 1 (2 ^ a) * σ 1 (q ^ b) :=
  (isMultiplicative_sigma (k := 1)).right (coprime_2pow_odd_pow hq hq_odd)

/-- φ(2^a · q^b) = φ(2^a) · φ(q^b) -/
theorem totient_2pow_odd_pow {q a b : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    Nat.totient (2 ^ a * q ^ b) = Nat.totient (2 ^ a) * Nat.totient (q ^ b) :=
  Nat.totient_mul (coprime_2pow_odd_pow hq hq_odd)

/-- τ(2^a · q^b) = (a+1)(b+1) -/
theorem tau_2pow_odd_pow {q a b : ℕ} (hq : q.Prime) (hq_odd : q ≥ 3) :
    (Nat.divisors (2 ^ a * q ^ b)).card = (a + 1) * (b + 1) := by
  have hcop : Nat.Coprime (2 ^ a) (q ^ b) := coprime_2pow_odd_pow hq hq_odd
  have hσ0 : σ 0 (2 ^ a * q ^ b) = σ 0 (2 ^ a) * σ 0 (q ^ b) :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have h_left : σ 0 (2 ^ a * q ^ b) = (Nat.divisors (2 ^ a * q ^ b)).card := by
    simp [sigma_zero_apply]
  rw [← h_left, hσ0, sigma_zero_apply_prime_pow Nat.prime_two,
      sigma_zero_apply_prime_pow hq]

/-- Theorem B case 4b(iii): a ≥ 2, q odd prime ≥ 3, b ≥ 1 → σφ ≠ nτ for n = 2^a·q^b -/
theorem theorem_B_2pow_odd_pow {q a b : ℕ}
    (hq : q.Prime) (hq_odd : q ≥ 3) (ha : a ≥ 2) (hb : b ≥ 1) :
    σ 1 (2 ^ a * q ^ b) * Nat.totient (2 ^ a * q ^ b) ≠
      (2 ^ a * q ^ b) * (Nat.divisors (2 ^ a * q ^ b)).card := by
  rw [sigma_one_2pow_odd_pow hq hq_odd,
      totient_2pow_odd_pow hq hq_odd,
      tau_2pow_odd_pow hq hq_odd]
  rw [sigma_one_prime_pow_sum Nat.prime_two, sigma_one_prime_pow_sum hq]
  rw [Nat.totient_prime_pow Nat.prime_two (by omega : a > 0),
      Nat.totient_prime_pow hq (by omega : b > 0)]
  intro h
  -- h: (Σ 2^i)·(Σ q^j)·(2^(a-1)·1·(q^(b-1)·(q-1))) = 2^a·q^b·((a+1)(b+1))
  -- Reorganize
  have reorg_lhs :
      ((∑ i ∈ Finset.range (a + 1), 2 ^ i) * ∑ j ∈ Finset.range (b + 1), q ^ j) *
        (2 ^ (a - 1) * (2 - 1) * (q ^ (b - 1) * (q - 1)))
      = (2 ^ (a - 1) * q ^ (b - 1)) *
        (((2 - 1) * ∑ i ∈ Finset.range (a + 1), 2 ^ i) *
         ((q - 1) * ∑ j ∈ Finset.range (b + 1), q ^ j)) := by
    ring
  rw [reorg_lhs,
      geom_sum_identity (by norm_num : (2 : ℕ) ≥ 2),
      geom_sum_identity (by omega : q ≥ 2)] at h
  -- h: 2^(a-1)·q^(b-1) · ((2^(a+1)-1)(q^(b+1)-1)) = 2^a·q^b·((a+1)(b+1))
  have h_2a : 2 ^ a = 2 * 2 ^ (a - 1) := by
    have h1 : 2 ^ a = 2 ^ ((a - 1) + 1) := by
      congr 1; omega
    rw [h1, pow_succ]; ring
  have h_qb : q ^ b = q * q ^ (b - 1) := by
    have h1 : q ^ b = q ^ ((b - 1) + 1) := by
      congr 1; omega
    rw [h1, pow_succ]; ring
  have h_rhs :
      2 ^ a * q ^ b * ((a + 1) * (b + 1))
      = (2 ^ (a - 1) * q ^ (b - 1)) * (2 * q * ((a + 1) * (b + 1))) := by
    rw [h_2a, h_qb]; ring
  rw [h_rhs] at h
  -- h: 2^(a-1)·q^(b-1) · ((2^(a+1)-1)(q^(b+1)-1)) = 2^(a-1)·q^(b-1) · (2q(a+1)(b+1))
  have h_pow_pos : 2 ^ (a - 1) * q ^ (b - 1) > 0 :=
    Nat.mul_pos (Nat.pow_pos (by norm_num : (2 : ℕ) > 0))
                (Nat.pow_pos (by omega : q > 0))
  have h_cancel :
      (2 ^ (a + 1) - 1) * (q ^ (b + 1) - 1) = 2 * q * ((a + 1) * (b + 1)) :=
    Nat.eq_of_mul_eq_mul_left h_pow_pos h
  -- Weak bounds
  have h_2w : 3 * 2 ^ (a + 1) ≥ 7 * (a + 1) + 3 := key_ineq_2pow_weak ha
  have h_qw : 3 * q ^ (b + 1) ≥ 4 * q * (b + 1) + 3 := key_ineq_odd_weak hq_odd hb
  -- 3·(2^(a+1) - 1) ≥ 7(a+1), 3·(q^(b+1) - 1) ≥ 4q(b+1)
  have h_2_pos : 2 ^ (a + 1) ≥ 1 := Nat.one_le_pow _ _ (by norm_num)
  have h_q_pos : q ^ (b + 1) ≥ 1 := Nat.one_le_pow _ _ (by omega)
  have h_2sub : 3 * (2 ^ (a + 1) - 1) ≥ 7 * (a + 1) := by
    have : 3 * (2 ^ (a + 1) - 1) = 3 * 2 ^ (a + 1) - 3 := by
      rw [Nat.mul_sub, Nat.mul_one]
    omega
  have h_qsub : 3 * (q ^ (b + 1) - 1) ≥ 4 * q * (b + 1) := by
    have : 3 * (q ^ (b + 1) - 1) = 3 * q ^ (b + 1) - 3 := by
      rw [Nat.mul_sub, Nat.mul_one]
    omega
  -- Product: 9·(2^(a+1)-1)(q^(b+1)-1) ≥ 7(a+1)·4q(b+1) = 28q(a+1)(b+1)
  have h_product :
      9 * ((2 ^ (a + 1) - 1) * (q ^ (b + 1) - 1)) ≥ 28 * q * ((a + 1) * (b + 1)) := by
    have h_prod_mul : 9 * ((2 ^ (a + 1) - 1) * (q ^ (b + 1) - 1)) =
                     (3 * (2 ^ (a + 1) - 1)) * (3 * (q ^ (b + 1) - 1)) := by ring
    rw [h_prod_mul]
    have h28 : 28 * q * ((a + 1) * (b + 1)) = 7 * (a + 1) * (4 * q * (b + 1)) := by ring
    rw [h28]
    exact Nat.mul_le_mul h_2sub h_qsub
  -- Combined: 9·h_cancel: 9·(2q(a+1)(b+1)) ≥ 28q(a+1)(b+1), i.e., 18 ≥ 28
  have h_from_cancel : 9 * ((2 ^ (a + 1) - 1) * (q ^ (b + 1) - 1)) =
                       9 * (2 * q * ((a + 1) * (b + 1))) := by
    rw [h_cancel]
  -- 9·(2q(a+1)(b+1)) = 18q(a+1)(b+1)
  have h_eq_18 : 9 * (2 * q * ((a + 1) * (b + 1))) = 18 * q * ((a + 1) * (b + 1)) := by ring
  rw [h_eq_18] at h_from_cancel
  -- h_from_cancel: 9·(...) = 18q(a+1)(b+1)
  -- h_product: 9·(...) ≥ 28q(a+1)(b+1)
  -- → 18q(a+1)(b+1) ≥ 28q(a+1)(b+1)
  have h_combine : 18 * q * ((a + 1) * (b + 1)) ≥ 28 * q * ((a + 1) * (b + 1)) := by
    rw [← h_from_cancel]; exact h_product
  -- q ≥ 3, (a+1)(b+1) ≥ 6 → q(a+1)(b+1) ≥ 18 > 0
  have h_denom_pos : q * ((a + 1) * (b + 1)) > 0 :=
    Nat.mul_pos (by omega) (Nat.mul_pos (by omega) (by omega))
  -- 18·X ≥ 28·X with X > 0 → 18 ≥ 28 → False
  have : 18 ≥ 28 := Nat.le_of_mul_le_mul_right (by
    have : 18 * (q * ((a + 1) * (b + 1))) ≥ 28 * (q * ((a + 1) * (b + 1))) := by
      have eq1 : 18 * (q * ((a + 1) * (b + 1))) = 18 * q * ((a + 1) * (b + 1)) := by ring
      have eq2 : 28 * (q * ((a + 1) * (b + 1))) = 28 * q * ((a + 1) * (b + 1)) := by ring
      rw [eq1, eq2]; exact h_combine
    exact this) h_denom_pos
  omega

/-- 확인 예시 -/
-- n = 12 = 2²·3: σφ=112, nτ=72. 18·112 = 2016, 28·72 = 2016 (바운드 tight)
-- n = 20 = 2²·5: σφ=42·8=... 실제 σ(20)=42, φ(20)=8, σφ=336. τ=6. nτ=120. 18·336=6048, 28·120=3360 ✓
example : 3 * 2 ^ (2 + 1) ≥ 7 * (2 + 1) + 3 := by decide
example : 3 * 3 ^ (1 + 1) ≥ 4 * 3 * (1 + 1) + 3 := by decide

end N6Mathlib
